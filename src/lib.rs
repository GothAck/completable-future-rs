//! A [`Future`] value that resolves once it's explicitly completed, potentially
//! from a different thread or task, similar to Java's `CompletableFuture`.
//!
//! Currently, this is implemented using [`Mutex`][parking_lot::Mutex] from the [`parking_lot`] crate.
//!
//! # Examples
//!
//! Create an incomplete [`ControlledFuture`] and explicitly complete it with the
//! completer:
//! ```
//! # use future_clicker::ControlledFuture;
//! # use futures::executor::block_on;
//! let (future, completer) = ControlledFuture::<i32>::new();
//! completer.complete(5).unwrap();
//! assert_eq!(block_on(future), Ok(5));
//! ```
//!
//! Create an initially complete [`ControlledFuture`] that can be immediately
//! resolved:
//! ```
//! # use future_clicker::ControlledFuture;
//! # use futures::executor::block_on;
//! assert_eq!(block_on(ControlledFuture::new_completed(10)), Ok(10));
//! ```

#![warn(clippy::pedantic, missing_docs)]

mod completer;
mod error;
mod state;

use std::{
    future::Future,
    marker::Unpin,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use parking_lot::Mutex;
use tracing::{instrument, trace};

use self::state::State;
pub use self::{
    completer::FutureClicker,
    error::{Error, Result},
};

/// A [`Future`] that will resolve either immediately, or in the future.
///
/// Will not resolve unless it has been explicitly completed, either
/// by constructing it with [`ControlledFuture::new_completed`], or using [`FutureClicker::complete`].
#[derive(Debug)]
pub struct ControlledFuture<T: Unpin> {
    state: Arc<Mutex<State<T>>>,
}

impl<T: Unpin + Send + 'static> ControlledFuture<T> {
    /// Construct a `ControlledFuture` that will resolve once the returned
    /// `FutureClicker` is used to set a value.
    #[must_use]
    pub fn new() -> (Self, FutureClicker<T>) {
        let s = State::new();
        (Self { state: s.0 }, FutureClicker { state: s.1 })
    }

    /// Construct a [`ControlledFuture`] that will resolve immediately to the
    /// given value.
    ///
    /// No [`FutureClicker`] is returned as the [`ControlledFuture`] is already complete.
    #[must_use]
    pub fn new_completed(value: T) -> Self {
        Self {
            state: State::new_completed(value),
        }
    }
}

impl<T: Unpin + 'static + Send> Future for ControlledFuture<T> {
    type Output = Result<T>;

    #[instrument(skip_all)]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        use Poll::{Pending, Ready};
        use State::{Complete, Dropped, Incomplete, Waiting};

        trace!("poll");
        let mut state = self.state.lock_arc();
        trace!("locked");

        match &mut *state {
            Waiting(w) if w.will_wake(cx.waker()) => {
                trace!("state Waiting will_wake");
                Pending
            }
            state @ (Waiting(_) | Incomplete) => {
                trace!("state {state:?}");
                *state = Waiting(cx.waker().clone());
                Pending
            }
            Complete(value) => {
                if let Some(value) = value.take() {
                    trace!("state Complete Some");
                    Ready(Ok(value))
                } else {
                    trace!("state Complete None");
                    Ready(Err(Error::AlreadyCompleted))
                }
            }
            Dropped => {
                trace!("state Dropped");
                Ready(Err(Error::CompleterDropped))
            }
        }
    }
}

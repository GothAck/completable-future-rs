use std::{mem, sync::Arc};

use parking_lot::Mutex;
use tracing::{instrument, trace};

use crate::{state::State, Error, Result};

/// Used to complete a `CompletableFuture` so it may resolve to the given value.
///
/// Dropping a `FutureCompleter` without calling `FutureCompleter::complete` will
/// cause the `CompletableFuture` to panic.
#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct FutureCompleter<T: Unpin + Send + 'static> {
    pub(crate) state: Arc<Mutex<State<T>>>,
}

impl<T: Unpin + Send + 'static> FutureCompleter<T> {
    /// Complete the associated `CompletableFuture`.
    ///
    /// # Errors
    /// `Error::AlreadyCompleted` - The `CompletableFuture` future is already resolved.
    /// `Error::CompleterDropped` - The `FutureCompleter` has already been dropped.
    #[instrument(skip_all)]
    pub fn complete(self, value: T) -> Result<()> {
        use State::{Complete, Dropped, Incomplete, Waiting};

        trace!("complete");

        let mut state = self.state.lock_arc();

        trace!("have lock");

        match mem::replace(&mut *state, State::Complete(Some(value))) {
            Incomplete => Ok(()),
            Waiting(waker) => {
                waker.wake();
                Ok(())
            }
            old @ Complete(_) => {
                *state = old;
                Err(Error::AlreadyCompleted)
            }
            old @ Dropped => {
                *state = old;
                Err(Error::CompleterDropped)
            }
        }
    }
}

impl<T: Unpin + Send + 'static> Drop for FutureCompleter<T> {
    #[instrument(skip_all)]
    fn drop(&mut self) {
        use State::{Complete, Dropped, Incomplete, Waiting};
        trace!("Drop");
        let mut state = self.state.lock_arc();
        trace!("Locked");

        match mem::replace(&mut *state, Dropped) {
            Incomplete | Dropped => {}
            Waiting(waker) => waker.wake(),
            old @ Complete(_) => *state = old,
        }
    }
}

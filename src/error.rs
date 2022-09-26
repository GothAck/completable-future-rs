use thiserror::Error;

/// Error returned by [`FutureClicker::complete`][crate::FutureClicker::complete] and [`ControlledFuture`][crate::ControlledFuture] future.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Error)]
pub enum Error {
    /// The [`ControlledFuture`][crate::ControlledFuture] future was already resolved to completion.
    #[error("future already polled to completion")]
    AlreadyCompleted,
    /// The [`FutureClicker`][crate::FutureClicker] was dropped before sending completion.
    #[error("Completer dropped before sending completion")]
    CompleterDropped,
}

/// Result of [`FutureClicker::complete`][crate::FutureClicker::complete] and the [`ControlledFuture`][crate::ControlledFuture] future.
pub type Result<T> = std::result::Result<T, Error>;

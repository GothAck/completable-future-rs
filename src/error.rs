use thiserror::Error;

/// Error returned by `Completer::complete` and `Completable` future.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Error)]
pub enum Error {
    /// The `Completable` future was already resolved to completion.
    #[error("future already polled to completion")]
    AlreadyCompleted,
    /// The `Completer` was dropped before sending completion.
    #[error("Completer dropped before sending completion")]
    CompleterDropped,
}

/// Result of `Completer::complete` and the `Completable` future.
pub type Result<T> = std::result::Result<T, Error>;

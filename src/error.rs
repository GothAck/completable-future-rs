use thiserror::Error;

/// Error returned by [`FutureCompleter::complete`][crate::FutureCompleter::complete] and [`CompletableFuture`][crate::CompletableFuture] future.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Error)]
pub enum Error {
    /// The [`CompletableFuture`][crate::CompletableFuture] future was already resolved to completion.
    #[error("future already polled to completion")]
    AlreadyCompleted,
    /// The [`FutureCompleter`][crate::FutureCompleter] was dropped before sending completion.
    #[error("Completer dropped before sending completion")]
    CompleterDropped,
}

/// Result of [`FutureCompleter::complete`][crate::FutureCompleter::complete] and the [`CompletableFuture`][crate::CompletableFuture] future.
pub type Result<T> = std::result::Result<T, Error>;

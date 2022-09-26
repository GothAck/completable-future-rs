# completable

<!-- cargo-sync-readme start -->

A [`Future`] value that resolves once it's explicitly completed, potentially
from a different thread or task, similar to Java's `CompletableFuture`.

Currently, this is implemented using [`Mutex`][parking_lot::Mutex] from the [`parking_lot`] crate.

# Examples

Create an incomplete [`ControlledFuture`] and explicitly complete it with the
completer:
```rust
let (future, completer) = ControlledFuture::<i32>::new();
completer.complete(5).unwrap();
assert_eq!(block_on(future), Ok(5));
```

Create an initially complete [`ControlledFuture`] that can be immediately
resolved:
```rust
assert_eq!(block_on(ControlledFuture::new_completed(10)), Ok(10));
```

<!-- cargo-sync-readme end -->

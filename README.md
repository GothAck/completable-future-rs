# completable

<!-- cargo-sync-readme start -->

A `Future` value that resolves once it's explicitly completed, potentially
from a different thread or task, similar to Java's `CompletableFuture`.

Currently, this is implemented using the `Mutex` from the `std` crate.

# Examples

Create an incomplete `Completable` and explicitly complete it with the
completer:
```rust
let (future, completer) = Completable::<i32>::new();
completer.complete(5).unwrap();
assert_eq!(block_on(future), Ok(5));
```

Create an initially complete `Completable` that can be immediately
resolved:
```rust
assert_eq!(block_on(Completable::new_completed(10)), Ok(10));
```

<!-- cargo-sync-readme end -->

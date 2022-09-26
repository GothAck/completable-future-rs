use std::{
    thread::{sleep, spawn},
    time::Duration,
};

use tokio::runtime::Handle;
use tracing::{instrument, trace};
use tracing_test::traced_test;

use future_clicker::{ControlledFuture, FutureClicker};

#[tokio::test]
#[traced_test]
async fn test_completed() {
    let (future, completer) = ControlledFuture::<usize>::new();

    assert_eq!(completer.complete(9876), Ok(()));

    assert_eq!(future.await, Ok(9876));
}

#[tokio::test]
#[traced_test]
async fn test_pre_completed() {
    assert_eq!(ControlledFuture::new_completed(1234usize).await, Ok(1234));
}

#[tokio::test(flavor = "multi_thread")]
#[traced_test]
async fn test_threaded() {
    let (future, completer) = ControlledFuture::<usize>::new();

    #[instrument(skip_all)]
    fn create_t1(handle: Handle, future: ControlledFuture<usize>) -> impl FnOnce() {
        self::trace!("create t1");
        move || {
            self::trace!("run t1");
            futures::executor::block_on(async {
                self::trace!("exec t1");
                assert_eq!(
                    handle.spawn(future).await.expect("Task t1 panicked"),
                    Ok(6353)
                );
                self::trace!("done t1");
            });
        }
    }

    #[instrument(skip_all)]
    fn create_t2(completer: FutureClicker<usize>) -> impl FnOnce() {
        self::trace!("create t2");
        move || {
            self::trace!("run t2");
            sleep(Duration::from_millis(100));
            self::trace!("slept t2");
            completer.complete(6353).unwrap();
        }
    }

    let t1 = spawn(create_t1(Handle::current(), future));
    let t2 = spawn(create_t2(completer));

    self::trace!("spawned");

    t1.join().unwrap();
    self::trace!("t1 joined");
    t2.join().unwrap();
    self::trace!("t2 joined");
}

use std::{fmt, sync::Arc, task::Waker};

use parking_lot::Mutex;
use strum::Display;

#[derive(Display)]
pub(crate) enum State<T> {
    Incomplete,
    Waiting(Waker),
    Complete(Option<T>),
    Dropped,
}

impl<T> State<T> {
    pub(crate) fn new() -> (Arc<Mutex<Self>>, Arc<Mutex<Self>>) {
        let this = Self::Incomplete.into_arc_mutex();
        (this.clone(), this)
    }

    pub(crate) fn new_completed(value: T) -> Arc<Mutex<Self>> {
        Self::Complete(Some(value)).into_arc_mutex()
    }

    fn into_arc_mutex(self) -> Arc<Mutex<Self>> {
        Arc::new(self.into())
    }
}

impl<T> fmt::Debug for State<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl<T> From<Option<T>> for State<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            value @ Some(_) => Self::Complete(value),
            None => Self::Incomplete,
        }
    }
}

use std::{pin::Pin, task::{Context, Poll}};

use futures::Stream;

pub mod chat;
pub mod responses;

pub struct BoxStreamUnpin<T>(Pin<Box<dyn Stream<Item = T> + Send>>);

impl<T> BoxStreamUnpin<T> {
    pub fn new<S>(stream: S) -> Self
    where
        S: Stream<Item = T> + Send + 'static,
    {
        BoxStreamUnpin(Box::pin(stream))
    }
}

impl<T> Stream for BoxStreamUnpin<T> {
    type Item = T;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.get_mut().0.as_mut().poll_next(cx)
    }
}

impl<T> Unpin for BoxStreamUnpin<T> {}

pub trait BoxUnpinExt: Stream + Sized + Send + 'static {
    fn boxed_unpin(self) -> BoxStreamUnpin<Self::Item> {
        BoxStreamUnpin::new(self)
    }
}

impl<T: Stream + Sized + Send + 'static> BoxUnpinExt for T {}

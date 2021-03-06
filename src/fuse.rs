use {Future, Poll, Async};

/// A future which "fuse"s a future once it's been resolved.
///
/// Normally futures can behave unpredictable once they're used after a future
/// has been resolved, but `Fuse` is always defined to return `Async::NotReady`
/// from `poll` after it has succeeded, and after it has succeeded all future
/// calls to `schedule` will be ignored.
pub struct Fuse<A: Future> {
    future: Option<A>,
}

pub fn new<A: Future>(f: A) -> Fuse<A> {
    Fuse {
        future: Some(f),
    }
}

impl<A: Future> Future for Fuse<A> {
    type Item = A::Item;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<A::Item, A::Error> {
        let res = self.future.as_mut().map(|f| f.poll());
        match res.unwrap_or(Ok(Async::NotReady)) {
            res @ Ok(Async::Ready(_)) |
            res @ Err(_) => {
                self.future = None;
                return res
            }
            Ok(Async::NotReady) => Ok(Async::NotReady)
        }
    }
}

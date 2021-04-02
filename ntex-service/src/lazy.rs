use std::{future::Future, pin::Pin, task::Context, task::Poll};

/// Future for the [`lazy`] function.
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Lazy<F> {
    f: Option<F>,
}

// safe because we never generate `Pin<&mut F>`
impl<F> Unpin for Lazy<F> {}

/// Creates a new future that allows delayed execution of a closure.
///
/// The provided closure is only run once the future is polled.
pub fn lazy<F, R>(f: F) -> Lazy<F>
where
    F: FnOnce(&mut Context<'_>) -> R,
{
    Lazy { f: Some(f) }
}

impl<F, R> Future for Lazy<F>
where
    F: FnOnce(&mut Context<'_>) -> R,
{
    type Output = R;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<R> {
        Poll::Ready((self.f.take().expect("Lazy polled after completion"))(cx))
    }
}

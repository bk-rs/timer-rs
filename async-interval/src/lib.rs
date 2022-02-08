use core::{future::Future, pin::Pin, time::Duration};
use std::time::Instant;

use futures_util::{stream, Stream};

#[cfg(feature = "impl_async_io")]
mod impl_async_io;
#[cfg(feature = "impl_async_timer")]
mod impl_async_timer;
#[cfg(feature = "impl_tokio")]
mod impl_tokio;

//
pub trait Intervalable {
    fn interval(dur: Duration) -> Self;

    fn xx<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Option<Instant>> + Send + 'a>>;
}

/// [Ref](https://docs.rs/futures-util/0.3.21/futures_util/stream/fn.iter.html)
pub fn intervalable_iter_stream<I, INTVL>(iter: I, interval: INTVL) -> impl Stream<Item = I::Item>
where
    I: IntoIterator,
    INTVL: Intervalable,
{
    stream::unfold(
        (iter.into_iter(), interval),
        |(mut iter, mut interval)| async move {
            if let Some(item) = iter.next() {
                interval.xx().await;
                Some((item, (iter, interval)))
            } else {
                None
            }
        },
    )
}

/// [Ref](https://docs.rs/futures-util/0.3.21/futures_util/stream/fn.repeat.html)
pub fn intervalable_repeat_stream<T, INTVL>(item: T, interval: INTVL) -> impl Stream<Item = T>
where
    T: Clone,
    INTVL: Intervalable,
{
    stream::unfold((item, interval), |(item, mut interval)| async move {
        interval.xx().await;
        Some((item.clone(), (item, interval)))
    })
}

/// [Ref](https://docs.rs/futures-util/0.3.21/futures_util/stream/fn.repeat_with.html)
pub fn intervalable_repeat_with_stream<A, F, INTVL>(
    repeater: F,
    interval: INTVL,
) -> impl Stream<Item = A>
where
    F: FnMut() -> A,
    INTVL: Intervalable,
{
    stream::unfold(
        (repeater, interval),
        |(mut repeater, mut interval)| async move {
            interval.xx().await;
            Some((repeater(), (repeater, interval)))
        },
    )
}

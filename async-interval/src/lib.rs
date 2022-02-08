use core::{future::Future, pin::Pin, time::Duration};

#[cfg(feature = "impl_async_timer")]
mod impl_async_timer;
#[cfg(feature = "impl_tokio")]
mod impl_tokio;

//
pub trait Interval {
    fn new(dur: Duration) -> Self;

    fn xx<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>;
}

#[cfg(test)]
#[allow(dead_code)]
pub(crate) fn interval_iter_stream<T, I>(
    interval: T,
    iter: I,
) -> impl futures_util::Stream<Item = I::Item>
where
    T: Interval,
    I: IntoIterator,
{
    futures_util::stream::unfold(
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

use alloc::boxed::Box;
use core::{future::Future, pin::Pin, time::Duration};

use futures_util::FutureExt as _;
pub use tokio::time::{Interval, Interval as TokioTimeInterval};

use crate::Intervalable;

//
impl Intervalable for Interval {
    fn interval(dur: Duration) -> Self {
        tokio::time::interval(tokio::time::Duration::from_micros(dur.as_micros() as u64))
    }

    fn wait<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        self.reset();
        Box::pin(self.tick().map(|_| ()))
    }

    #[cfg(feature = "std")]
    fn wait_for_std<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Option<std::time::Instant>> + Send + 'a>> {
        self.reset();
        Box::pin(self.tick().map(|x| Some(x.into_std())))
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[cfg(feature = "std")]
    #[tokio::test]
    async fn test_impl() {
        #[cfg(feature = "std")]
        let now = std::time::Instant::now();

        let mut interval = <Interval as Intervalable>::interval(Duration::from_millis(100));

        //
        interval.wait().await;

        #[cfg(feature = "std")]
        {
            let elapsed_dur = now.elapsed();
            assert!(elapsed_dur.as_millis() >= 100 && elapsed_dur.as_millis() <= 105);
        }

        //
        interval.wait().await;

        #[cfg(feature = "std")]
        {
            let elapsed_dur = now.elapsed();
            assert!(elapsed_dur.as_millis() >= 200 && elapsed_dur.as_millis() <= 210);
        }

        #[cfg(feature = "std")]
        {
            assert!(interval.wait_for_std().await.is_some());

            let elapsed_dur = now.elapsed();
            assert!(elapsed_dur.as_millis() >= 300 && elapsed_dur.as_millis() <= 315);
        }
    }

    #[cfg(all(feature = "std", feature = "stream"))]
    #[tokio::test]
    async fn test_intervalable_iter_stream() {
        use alloc::{vec, vec::Vec};

        use futures_util::StreamExt as _;

        //
        let st = crate::intervalable_iter_stream(
            0..=2,
            <Interval as Intervalable>::interval(Duration::from_millis(100)),
        );

        #[cfg(feature = "std")]
        let now = std::time::Instant::now();

        assert_eq!(st.collect::<Vec<_>>().await, vec![0, 1, 2]);

        #[cfg(feature = "std")]
        {
            let elapsed_dur = now.elapsed();
            assert!(elapsed_dur.as_millis() >= 300 && elapsed_dur.as_millis() <= 310);
        }
    }

    #[cfg(all(feature = "std", feature = "stream"))]
    #[tokio::test]
    async fn test_intervalable_repeat_stream() {
        use alloc::{vec, vec::Vec};

        use futures_util::StreamExt as _;

        //
        let st = crate::intervalable_repeat_stream(
            0,
            <Interval as Intervalable>::interval(Duration::from_millis(100)),
        );

        #[cfg(feature = "std")]
        let now = std::time::Instant::now();

        assert_eq!(st.take(3).collect::<Vec<_>>().await, vec![0, 0, 0]);

        #[cfg(feature = "std")]
        {
            let elapsed_dur = now.elapsed();
            assert!(elapsed_dur.as_millis() >= 300 && elapsed_dur.as_millis() <= 310);
        }
    }

    #[cfg(all(feature = "std", feature = "stream"))]
    #[tokio::test]
    async fn test_intervalable_repeat_with_stream() {
        use alloc::{vec, vec::Vec};

        use futures_util::StreamExt as _;

        //
        let st = crate::intervalable_repeat_with_stream(
            || 0,
            <Interval as Intervalable>::interval(Duration::from_millis(100)),
        );

        #[cfg(feature = "std")]
        let now = std::time::Instant::now();

        assert_eq!(st.take(3).collect::<Vec<_>>().await, vec![0, 0, 0]);

        #[cfg(feature = "std")]
        {
            let elapsed_dur = now.elapsed();
            assert!(elapsed_dur.as_millis() >= 300 && elapsed_dur.as_millis() <= 310);
        }
    }
}

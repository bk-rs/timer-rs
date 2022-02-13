use alloc::boxed::Box;
use core::{future::Future, pin::Pin, time::Duration};

pub use async_timer::Interval;

use crate::Intervalable;

//
impl Intervalable for Interval {
    fn interval(dur: Duration) -> Self {
        Self::new(dur)
    }

    #[cfg(feature = "std")]
    fn wait<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Option<std::time::Instant>> + Send + 'a>> {
        use futures_util::FutureExt as _;

        Box::pin(self.wait().map(|_| None))
    }
    #[cfg(not(feature = "std"))]
    fn wait<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        Box::pin(self.wait())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::{vec, vec::Vec};

    use futures_util::StreamExt as _;

    use crate::intervalable_iter_stream;

    #[tokio::test]
    async fn test_intervalable_iter_stream() {
        let st = intervalable_iter_stream(
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
}

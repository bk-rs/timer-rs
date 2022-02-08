use core::{future::Future, pin::Pin, time::Duration};
use std::time::Instant;

use async_timer::Interval;
use futures_util::FutureExt as _;

use crate::Intervalable;

//
impl Intervalable for Interval {
    fn interval(dur: Duration) -> Self {
        Self::new(dur)
    }

    fn xx<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Option<Instant>> + Send + 'a>> {
        Box::pin(self.wait().map(|_| None))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use futures_util::StreamExt as _;

    use crate::intervalable_iter_stream;

    #[tokio::test]
    async fn test_intervalable_iter_stream() {
        let st = intervalable_iter_stream(
            0..=2,
            <Interval as Intervalable>::interval(Duration::from_millis(100)),
        );

        let now = std::time::Instant::now();

        assert_eq!(st.collect::<Vec<_>>().await, vec![0, 1, 2]);

        let elapsed_dur = now.elapsed();
        assert!(elapsed_dur.as_millis() >= 300 && elapsed_dur.as_millis() <= 310);
    }
}

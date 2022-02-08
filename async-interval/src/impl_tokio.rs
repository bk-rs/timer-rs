use core::{future::Future, pin::Pin, time::Duration};
use std::time::Instant;

use futures_util::FutureExt as _;
use tokio::time::Interval;

use crate::Intervalable;

//
impl Intervalable for Interval {
    fn interval(dur: Duration) -> Self {
        tokio::time::interval(tokio::time::Duration::from_micros(dur.as_micros() as u64))
    }

    fn wait<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Option<Instant>> + Send + 'a>> {
        self.reset();
        Box::pin(self.tick().map(|x| Some(x.into_std())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use futures_util::StreamExt as _;

    use crate::{
        intervalable_iter_stream, intervalable_repeat_stream, intervalable_repeat_with_stream,
    };

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

    #[tokio::test]
    async fn test_intervalable_repeat_stream() {
        let st = intervalable_repeat_stream(
            0,
            <Interval as Intervalable>::interval(Duration::from_millis(100)),
        );

        let now = std::time::Instant::now();

        assert_eq!(st.take(3).collect::<Vec<_>>().await, vec![0, 0, 0]);

        let elapsed_dur = now.elapsed();
        assert!(elapsed_dur.as_millis() >= 300 && elapsed_dur.as_millis() <= 310);
    }

    #[tokio::test]
    async fn test_intervalable_repeat_with_stream() {
        let st = intervalable_repeat_with_stream(
            || 0,
            <Interval as Intervalable>::interval(Duration::from_millis(100)),
        );

        let now = std::time::Instant::now();

        assert_eq!(st.take(3).collect::<Vec<_>>().await, vec![0, 0, 0]);

        let elapsed_dur = now.elapsed();
        assert!(elapsed_dur.as_millis() >= 300 && elapsed_dur.as_millis() <= 310);
    }
}

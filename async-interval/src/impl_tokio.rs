use core::{future::Future, pin::Pin, time::Duration};
use std::time::Instant;

use futures_util::FutureExt as _;
use tokio::time::Interval;

use crate::Intervalable;

//
impl Intervalable for Interval {
    fn interval(dur: Duration) -> Self {
        tokio::time::interval(tokio::time::Duration::from_millis(dur.as_millis() as u64))
    }

    fn xx<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Option<Instant>> + Send + 'a>> {
        self.reset();
        Box::pin(self.tick().map(|x| Some(x.into_std())))
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

        let ret = st.collect::<Vec<_>>().await;

        assert_eq!(ret, vec![0, 1, 2]);

        let elapsed_dur = now.elapsed();
        assert!(elapsed_dur.as_millis() >= 300 && elapsed_dur.as_millis() <= 310);
    }
}

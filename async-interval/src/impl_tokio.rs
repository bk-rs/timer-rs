use core::{future::Future, pin::Pin, time::Duration};

use futures_util::FutureExt as _;
use tokio::time::Interval;

impl crate::Interval for Interval {
    fn new(dur: Duration) -> Self {
        tokio::time::interval(tokio::time::Duration::from_millis(dur.as_millis() as u64))
    }

    fn xx<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        self.reset();
        Box::pin(self.tick().map(|_| ()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use futures_util::StreamExt as _;

    use crate::interval_iter_stream;

    #[tokio::test]
    async fn test() {
        let st = interval_iter_stream(
            <Interval as crate::Interval>::new(Duration::from_millis(100)),
            0..=2,
        );

        let now = std::time::Instant::now();

        let ret = st.collect::<Vec<_>>().await;

        assert_eq!(ret, vec![0, 1, 2]);

        let elapsed_dur = now.elapsed();
        println!("elapsed_dur {:?}", elapsed_dur);
        assert!(
            elapsed_dur > Duration::from_millis(300) && elapsed_dur < Duration::from_millis(310)
        );
    }
}

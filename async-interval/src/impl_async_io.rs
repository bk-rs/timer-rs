use core::{future::Future, pin::Pin, time::Duration};
use std::time::Instant;

use async_io::Timer;
use futures_util::StreamExt as _;

use crate::Intervalable;

//
impl Intervalable for Timer {
    fn interval(dur: Duration) -> Self {
        Self::interval(dur)
    }

    fn xx<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Option<Instant>> + Send + 'a>> {
        Box::pin(self.next())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::intervalable_iter_stream;

    #[tokio::test]
    async fn test_intervalable_iter_stream() {
        let st = intervalable_iter_stream(
            0..=2,
            <Timer as Intervalable>::interval(Duration::from_millis(100)),
        );

        let now = std::time::Instant::now();

        let ret = st.collect::<Vec<_>>().await;

        assert_eq!(ret, vec![0, 1, 2]);

        let elapsed_dur = now.elapsed();
        assert!(elapsed_dur.as_millis() >= 300 && elapsed_dur.as_millis() <= 310);
    }
}

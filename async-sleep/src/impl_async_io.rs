use core::{future::Future, pin::Pin, time::Duration};

pub use async_io::Timer;
use futures_util::FutureExt as _;

use crate::Sleepble;

//
impl Sleepble for Timer {
    fn sleep(dur: Duration) -> Self {
        Self::after(dur)
    }

    fn wait(self) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> {
        Box::pin(self.map(|_| ()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::sleep;

    #[tokio::test]
    async fn test_sleep() {
        let now = std::time::Instant::now();

        sleep::<Timer>(Duration::from_millis(100)).await;

        let elapsed_dur = now.elapsed();
        assert!(elapsed_dur.as_millis() >= 100 && elapsed_dur.as_millis() <= 105);
    }
}

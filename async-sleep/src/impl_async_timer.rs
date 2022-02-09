use core::{future::Future, pin::Pin, time::Duration};

pub use async_timer::timer::Platform as PlatformTimer;

use crate::Sleepble;

//
impl Sleepble for PlatformTimer {
    fn sleep(dur: Duration) -> Self {
        Self::new(dur)
    }

    fn wait(self) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> {
        Box::pin(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::sleep;

    #[tokio::test]
    async fn test_sleep() {
        let now = std::time::Instant::now();

        sleep::<PlatformTimer>(Duration::from_millis(100)).await;

        let elapsed_dur = now.elapsed();
        assert!(elapsed_dur.as_millis() >= 100 && elapsed_dur.as_millis() <= 105);
    }
}

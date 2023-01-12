use alloc::boxed::Box;
use core::{future::Future, pin::Pin, time::Duration};

pub use tokio::time::Sleep;

use crate::Sleepble;

//
impl Sleepble for Sleep {
    fn sleep(dur: Duration) -> Self {
        tokio::time::sleep(tokio::time::Duration::from_micros(dur.as_micros() as u64))
    }

    fn wait(self) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> {
        Box::pin(self)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[cfg(feature = "std")]
    #[tokio::test]
    async fn test_sleep() {
        #[cfg(feature = "std")]
        let now = std::time::Instant::now();

        crate::sleep::sleep::<Sleep>(Duration::from_millis(100)).await;

        #[cfg(feature = "std")]
        {
            let elapsed_dur = now.elapsed();
            assert!(elapsed_dur.as_millis() >= 100 && elapsed_dur.as_millis() <= 105);
        }
    }

    #[cfg(feature = "std")]
    #[tokio::test]
    async fn test_sleep_until() {
        let now = std::time::Instant::now();

        crate::sleep::sleep_until::<Sleep>(std::time::Instant::now() + Duration::from_millis(100))
            .await;

        let elapsed_dur = now.elapsed();
        assert!(elapsed_dur.as_millis() >= 100 && elapsed_dur.as_millis() <= 105);
    }
}

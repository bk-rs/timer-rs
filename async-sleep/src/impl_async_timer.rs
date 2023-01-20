use alloc::boxed::Box;
use core::time::Duration;

pub use async_timer::timer::Platform as PlatformTimer;

use crate::{Sleepble, SleepbleWaitBoxFuture};

//
impl Sleepble for PlatformTimer {
    fn sleep(dur: Duration) -> Self {
        Self::new(dur)
    }

    fn wait(self) -> SleepbleWaitBoxFuture {
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

        crate::sleep::sleep::<PlatformTimer>(Duration::from_millis(100)).await;

        #[cfg(feature = "std")]
        {
            let elapsed_dur = now.elapsed();
            assert!(elapsed_dur.as_millis() >= 100 && elapsed_dur.as_millis() <= 105);
        }
    }
}

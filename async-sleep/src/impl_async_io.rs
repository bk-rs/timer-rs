use alloc::boxed::Box;
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
    #[allow(unused_imports)]
    use super::*;

    #[cfg(feature = "std")]
    #[tokio::test]
    async fn test_sleep() {
        #[cfg(feature = "std")]
        let now = std::time::Instant::now();

        crate::sleep::sleep::<Timer>(Duration::from_millis(100)).await;

        #[cfg(feature = "std")]
        {
            let elapsed_dur = now.elapsed();
            assert!(elapsed_dur.as_millis() >= 100 && elapsed_dur.as_millis() <= 105);
        }
    }
}

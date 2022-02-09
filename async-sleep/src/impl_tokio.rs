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
    use super::*;

    use crate::sleep;

    #[tokio::test]
    async fn test_sleep() {
        let now = std::time::Instant::now();

        sleep::<Sleep>(Duration::from_millis(100)).await;

        let elapsed_dur = now.elapsed();
        assert!(elapsed_dur.as_millis() >= 100 && elapsed_dur.as_millis() <= 105);
    }
}

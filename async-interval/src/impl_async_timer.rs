use core::{future::Future, pin::Pin, time::Duration};

use async_timer::Interval;

impl crate::Interval for Interval {
    fn new(dur: Duration) -> Self {
        Self::new(dur)
    }

    fn xx<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        Box::pin(self.wait())
    }
}

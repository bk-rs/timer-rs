use core::{future::Future, time::Duration};

use crate::Sleepble;

//
pub fn sleep<SLEEP>(dur: Duration) -> impl Future<Output = ()>
where
    SLEEP: Sleepble,
{
    SLEEP::sleep(dur).wait()
}

#[cfg(feature = "std")]
pub fn sleep_until<SLEEP>(deadline: std::time::Instant) -> impl Future<Output = ()>
where
    SLEEP: Sleepble,
{
    let dur = deadline
        .checked_duration_since(std::time::Instant::now())
        .unwrap_or_default();

    sleep::<SLEEP>(dur)
}

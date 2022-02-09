use core::{future::Future, pin::Pin, time::Duration};

#[cfg(feature = "impl_async_io")]
pub mod impl_async_io;
#[cfg(feature = "impl_async_timer")]
pub mod impl_async_timer;
#[cfg(feature = "impl_tokio")]
pub mod impl_tokio;

//
pub trait Sleepble {
    fn sleep(dur: Duration) -> Self;

    fn wait(self) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>;
}

pub async fn sleep<SLEEP>(dur: Duration)
where
    SLEEP: Sleepble,
{
    SLEEP::sleep(dur).wait().await
}

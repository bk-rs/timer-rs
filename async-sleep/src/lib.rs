#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

//
use core::time::Duration;

#[cfg(feature = "futures-util")]
pub type SleepbleWaitBoxFuture = futures_util::future::BoxFuture<'static, ()>;
#[cfg(not(feature = "futures-util"))]
pub type SleepbleWaitBoxFuture =
    core::pin::Pin<alloc::boxed::Box<dyn core::future::Future<Output = ()> + Send + 'static>>;

pub trait Sleepble {
    fn sleep(dur: Duration) -> Self;

    fn wait(self) -> SleepbleWaitBoxFuture;
}

//
#[cfg(feature = "impl_async_io")]
pub mod impl_async_io;
#[cfg(feature = "impl_async_io")]
pub use impl_async_io::AsyncIoTimer;
#[cfg(feature = "impl_async_timer")]
pub mod impl_async_timer;
#[cfg(feature = "impl_async_timer")]
pub use impl_async_timer::AsyncTimerPlatform;
#[cfg(feature = "impl_tokio")]
pub mod impl_tokio;
#[cfg(feature = "impl_tokio")]
pub use impl_tokio::TokioTimeSleep;

#[cfg(feature = "rw")]
pub mod rw;
#[cfg(feature = "rw")]
pub use self::rw::{AsyncReadWithTimeoutExt, AsyncWriteWithTimeoutExt};

pub mod sleep;
pub use sleep::sleep;
#[cfg(feature = "std")]
pub use sleep::sleep_until;

#[cfg(feature = "timeout")]
pub mod timeout;
#[cfg(feature = "timeout")]
pub use self::timeout::timeout;
#[cfg(all(feature = "timeout", feature = "std"))]
pub use self::timeout::timeout_at;

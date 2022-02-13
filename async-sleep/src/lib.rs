#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

//
use alloc::boxed::Box;
use core::{future::Future, pin::Pin, time::Duration};

pub trait Sleepble {
    fn sleep(dur: Duration) -> Self;

    fn wait(self) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>;
}

//
#[cfg(feature = "impl_async_io")]
pub mod impl_async_io;
#[cfg(feature = "impl_async_timer")]
pub mod impl_async_timer;
#[cfg(feature = "impl_tokio")]
pub mod impl_tokio;

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

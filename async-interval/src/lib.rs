#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

//
use alloc::boxed::Box;
use core::{future::Future, pin::Pin, time::Duration};

pub trait Intervalable {
    fn interval(dur: Duration) -> Self;

    fn wait<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>;

    #[cfg(feature = "std")]
    fn wait_for_std<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Option<std::time::Instant>> + Send + 'a>>;
}

//
#[cfg(feature = "impl_async_io")]
pub mod impl_async_io;
#[cfg(feature = "impl_async_timer")]
pub mod impl_async_timer;
#[cfg(feature = "impl_tokio")]
pub mod impl_tokio;

//
#[cfg(feature = "stream")]
pub mod stream;
#[cfg(feature = "stream")]
pub use stream::{
    intervalable_iter_stream, intervalable_repeat_stream, intervalable_repeat_with_stream,
};

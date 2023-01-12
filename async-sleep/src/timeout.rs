use alloc::boxed::Box;
use core::{future::Future, time::Duration};

use futures_util::{
    future::{self, Either},
    FutureExt as _, TryFutureExt as _,
};

#[cfg(feature = "std")]
use crate::sleep::sleep_until;
use crate::{sleep::sleep, Sleepble};

//
pub fn internal_timeout<SLEEP, T>(
    dur: Duration,
    future: T,
) -> impl Future<Output = Result<T::Output, (Duration, T)>>
where
    SLEEP: Sleepble,
    T: Future + Unpin,
{
    future::select(future, Box::pin(sleep::<SLEEP>(dur))).map(move |either| match either {
        Either::Left((output, _)) => Ok(output),
        Either::Right((_, future)) => Err((dur, future)),
    })
}

pub fn timeout<SLEEP, T>(dur: Duration, future: T) -> impl Future<Output = Result<T::Output, Error>>
where
    SLEEP: Sleepble,
    T: Future + Unpin,
{
    internal_timeout::<SLEEP, _>(dur, future).map_err(|(dur, _)| Error::Timeout(dur))
}

#[cfg(feature = "std")]
pub fn internal_timeout_at<SLEEP, T>(
    deadline: std::time::Instant,
    future: T,
) -> impl Future<Output = Result<T::Output, (std::time::Instant, T)>>
where
    SLEEP: Sleepble,
    T: Future + Unpin,
{
    future::select(future, Box::pin(sleep_until::<SLEEP>(deadline))).map(move |either| match either
    {
        Either::Left((output, _)) => Ok(output),
        Either::Right((_, future)) => Err((deadline, future)),
    })
}

#[cfg(feature = "std")]
pub fn timeout_at<SLEEP, T>(
    deadline: std::time::Instant,
    future: T,
) -> impl Future<Output = Result<T::Output, Error>>
where
    SLEEP: Sleepble,
    T: Future + Unpin,
{
    internal_timeout_at::<SLEEP, _>(deadline, future)
        .map_err(|(instant, _)| Error::TimeoutAt(instant))
}

//
#[derive(Debug, PartialEq)]
pub enum Error {
    Timeout(Duration),
    #[cfg(feature = "std")]
    TimeoutAt(std::time::Instant),
}
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}
#[cfg(feature = "std")]
impl std::error::Error for Error {}

#[cfg(feature = "std")]
impl From<Error> for std::io::Error {
    fn from(_err: Error) -> std::io::Error {
        std::io::ErrorKind::TimedOut.into()
    }
}

#[cfg(feature = "impl_tokio")]
#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[allow(dead_code)]
    async fn foo() -> usize {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        0
    }

    #[cfg(feature = "std")]
    #[tokio::test]
    async fn test_timeout() {
        //
        #[cfg(feature = "std")]
        let now = std::time::Instant::now();

        let (_tx, rx) = tokio::sync::oneshot::channel::<()>();
        match timeout::<crate::impl_tokio::Sleep, _>(Duration::from_millis(50), rx).await {
            Ok(v) => panic!("{v:?}"),
            Err(err) => assert_eq!(err, Error::Timeout(Duration::from_millis(50))),
        }

        #[cfg(feature = "std")]
        {
            let elapsed_dur = now.elapsed();
            assert!(elapsed_dur.as_millis() >= 50 && elapsed_dur.as_millis() <= 55);
        }

        //
        #[cfg(feature = "std")]
        let now = std::time::Instant::now();

        match timeout::<crate::impl_tokio::Sleep, _>(Duration::from_millis(50), Box::pin(foo()))
            .await
        {
            Ok(v) => panic!("{v:?}"),
            Err(err) => assert_eq!(err, Error::Timeout(Duration::from_millis(50))),
        }

        #[cfg(feature = "std")]
        {
            let elapsed_dur = now.elapsed();
            assert!(elapsed_dur.as_millis() >= 50 && elapsed_dur.as_millis() <= 55);
        }

        //
        #[cfg(feature = "std")]
        let now = std::time::Instant::now();

        match timeout::<crate::impl_tokio::Sleep, _>(Duration::from_millis(150), Box::pin(foo()))
            .await
        {
            Ok(v) => assert_eq!(v, 0),
            Err(err) => panic!("{err:?}"),
        }

        #[cfg(feature = "std")]
        {
            let elapsed_dur = now.elapsed();
            assert!(elapsed_dur.as_millis() >= 100 && elapsed_dur.as_millis() <= 105);
        }
    }

    #[cfg(feature = "std")]
    #[tokio::test]
    async fn test_timeout_at() {
        //
        let now = std::time::Instant::now();

        match timeout_at::<crate::impl_tokio::Sleep, _>(
            std::time::Instant::now() + Duration::from_millis(50),
            Box::pin(foo()),
        )
        .await
        {
            Ok(v) => panic!("{v:?}"),
            Err(Error::Timeout(dur)) => panic!("{dur:?}"),
            Err(Error::TimeoutAt(instant)) => {
                let elapsed_dur = instant.elapsed();
                assert!(elapsed_dur.as_millis() <= 5);
            }
        }

        let elapsed_dur = now.elapsed();
        assert!(elapsed_dur.as_millis() >= 50 && elapsed_dur.as_millis() <= 55);
    }
}

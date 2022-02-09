use core::{fmt, future::Future, time::Duration};

use futures_util::future::{self, Either};

use crate::{sleep, Sleepble};

//
pub async fn timeout<SLEEP, T>(dur: Duration, future: T) -> Result<T::Output, Error>
where
    SLEEP: Sleepble,
    T: Future,
{
    match future::select(Box::pin(future), Box::pin(sleep::<SLEEP>(dur))).await {
        Either::Left((x, _)) => Ok(x),
        Either::Right((_, _)) => Err(Error::Timeout(dur)),
    }
}

//
#[derive(Debug, PartialEq)]
pub enum Error {
    Timeout(Duration),
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for Error {}

impl From<Error> for std::io::Error {
    fn from(_err: Error) -> std::io::Error {
        std::io::ErrorKind::TimedOut.into()
    }
}

#[cfg(feature = "impl_tokio")]
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_timeout() {
        async fn foo() -> usize {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            0
        }

        //
        let now = std::time::Instant::now();

        match timeout::<crate::impl_tokio::Sleep, _>(Duration::from_millis(50), foo()).await {
            Err(err) => assert_eq!(err, Error::Timeout(Duration::from_millis(50))),
            ret => panic!("{:?}", ret),
        }

        let elapsed_dur = now.elapsed();
        assert!(elapsed_dur.as_millis() >= 50 && elapsed_dur.as_millis() <= 55);

        //
        let now = std::time::Instant::now();

        match timeout::<crate::impl_tokio::Sleep, _>(Duration::from_millis(150), foo()).await {
            Ok(x) => assert_eq!(x, 0),
            ret => panic!("{:?}", ret),
        }

        let elapsed_dur = now.elapsed();
        assert!(elapsed_dur.as_millis() >= 100 && elapsed_dur.as_millis() <= 105);
    }
}

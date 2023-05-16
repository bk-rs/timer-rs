use alloc::boxed::Box;
use core::time::Duration;

pub use async_io::{Timer, Timer as AsyncIoTimer};
use futures_util::FutureExt as _;

use crate::{Sleepble, SleepbleWaitBoxFuture};

//
impl Sleepble for Timer {
    fn sleep(dur: Duration) -> Self {
        Self::after(dur)
    }

    fn wait(self) -> SleepbleWaitBoxFuture {
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

    #[cfg(feature = "rw")]
    #[cfg(test)]
    mod rw_tests {
        use core::time::Duration;
        use std::{
            io::ErrorKind as IoErrorKind,
            net::{TcpListener, TcpStream},
            time::Instant,
        };

        use async_io::Async;
        use futures_lite::future::block_on;

        use crate::{
            impl_async_io::Timer,
            rw::{AsyncReadWithTimeoutExt as _, AsyncWriteWithTimeoutExt as _},
        };

        #[test]
        fn simple() -> Result<(), Box<dyn std::error::Error>> {
            block_on(async {
                let listener = TcpListener::bind("127.0.0.1:0")?;

                let addr = listener.local_addr()?;

                let tcp_stream_c = TcpStream::connect(addr)?;
                let tcp_stream_s = listener
                    .incoming()
                    .next()
                    .expect("Get next incoming failed")?;

                let mut tcp_stream_c = Async::<TcpStream>::new(tcp_stream_c)?;
                let mut tcp_stream_s = Async::<TcpStream>::new(tcp_stream_s)?;

                tcp_stream_s
                    .write_with_timeout::<Timer>(b"foo", Duration::from_secs(1))
                    .await?;

                let mut buf = vec![0u8; 5];
                let n = tcp_stream_c
                    .read_with_timeout::<Timer>(&mut buf, Duration::from_secs(1))
                    .await?;
                assert_eq!(n, 3);
                assert_eq!(buf, b"foo\0\0");

                let instant = Instant::now();
                let two_secs = Duration::from_secs(2);
                let three_secs = Duration::from_secs(3);
                let err = tcp_stream_c
                    .read_with_timeout::<Timer>(&mut buf, Duration::from_secs(2))
                    .await
                    .err()
                    .unwrap();
                assert!(instant.elapsed() >= two_secs);
                assert!(instant.elapsed() < three_secs);
                assert_eq!(err.kind(), IoErrorKind::TimedOut);
                assert_eq!(err.to_string(), "read timeout");

                Ok(())
            })
        }
    }
}

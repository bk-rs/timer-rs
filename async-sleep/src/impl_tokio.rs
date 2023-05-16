use alloc::boxed::Box;
use core::time::Duration;

pub use tokio::time::{Sleep, Sleep as TokioTimeSleep};

use crate::{Sleepble, SleepbleWaitBoxFuture};

//
impl Sleepble for Sleep {
    fn sleep(dur: Duration) -> Self {
        tokio::time::sleep(tokio::time::Duration::from_micros(dur.as_micros() as u64))
    }

    fn wait(self) -> SleepbleWaitBoxFuture {
        Box::pin(self)
    }
}

//
#[derive(Debug)]
pub struct UnpinSleep(pub Sleep);
impl Unpin for UnpinSleep {}

impl Sleepble for UnpinSleep {
    fn sleep(dur: Duration) -> Self {
        UnpinSleep(tokio::time::sleep(tokio::time::Duration::from_micros(
            dur.as_micros() as u64,
        )))
    }

    fn wait(self) -> SleepbleWaitBoxFuture {
        Box::pin(self.0)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[cfg(feature = "std")]
    #[tokio::test]
    async fn test_sleep() {
        {
            #[cfg(feature = "std")]
            let now = std::time::Instant::now();

            crate::sleep::sleep::<Sleep>(Duration::from_millis(100)).await;

            #[cfg(feature = "std")]
            {
                let elapsed_dur = now.elapsed();
                assert!(elapsed_dur.as_millis() >= 100 && elapsed_dur.as_millis() <= 105);
            }
        }

        {
            #[cfg(feature = "std")]
            let now = std::time::Instant::now();

            crate::sleep::sleep::<UnpinSleep>(Duration::from_millis(100)).await;

            #[cfg(feature = "std")]
            {
                let elapsed_dur = now.elapsed();
                assert!(elapsed_dur.as_millis() >= 100 && elapsed_dur.as_millis() <= 105);
            }
        }
    }

    #[cfg(feature = "std")]
    #[tokio::test]
    async fn test_sleep_until() {
        {
            let now = std::time::Instant::now();

            crate::sleep::sleep_until::<Sleep>(
                std::time::Instant::now() + Duration::from_millis(100),
            )
            .await;

            let elapsed_dur = now.elapsed();
            assert!(elapsed_dur.as_millis() >= 100 && elapsed_dur.as_millis() <= 105);
        }

        {
            let now = std::time::Instant::now();

            crate::sleep::sleep_until::<UnpinSleep>(
                std::time::Instant::now() + Duration::from_millis(100),
            )
            .await;

            let elapsed_dur = now.elapsed();
            assert!(elapsed_dur.as_millis() >= 100 && elapsed_dur.as_millis() <= 105);
        }
    }

    #[cfg(feature = "rw")]
    #[cfg(test)]
    mod rw_tests {
        use core::time::Duration;
        use std::{io::ErrorKind as IoErrorKind, time::Instant};

        use async_compat::Compat;
        use tokio::{
            net::{TcpListener, TcpStream},
            runtime::Runtime,
        };

        use crate::{
            impl_tokio::Sleep,
            rw::{AsyncReadWithTimeoutExt as _, AsyncWriteWithTimeoutExt as _},
        };

        #[test]
        fn simple() -> Result<(), Box<dyn std::error::Error>> {
            let rt = Runtime::new().unwrap();

            let ret = rt.block_on(async {
                let listener = TcpListener::bind("127.0.0.1:0").await?;

                let addr = listener.local_addr()?;

                let tcp_stream_c = TcpStream::connect(addr).await?;
                let mut tcp_stream_c = Compat::new(tcp_stream_c);
                let (tcp_stream_s, _) = listener.accept().await.expect("Accept failed");
                let mut tcp_stream_s = Compat::new(tcp_stream_s);

                tcp_stream_s
                    .write_with_timeout::<Sleep>(b"foo", Duration::from_secs(1))
                    .await?;

                let mut buf = vec![0u8; 5];
                let n = tcp_stream_c
                    .read_with_timeout::<Sleep>(&mut buf, Duration::from_secs(1))
                    .await?;
                assert_eq!(n, 3);
                assert_eq!(buf, b"foo\0\0");

                let instant = Instant::now();
                let two_secs = Duration::from_secs(2);
                let three_secs = Duration::from_secs(3);
                let err = tcp_stream_c
                    .read_with_timeout::<Sleep>(&mut buf, Duration::from_secs(2))
                    .await
                    .err()
                    .unwrap();
                assert!(instant.elapsed() >= two_secs);
                assert!(instant.elapsed() < three_secs);
                assert_eq!(err.kind(), IoErrorKind::TimedOut);
                assert_eq!(err.to_string(), "read timeout");

                Result::<(), Box<dyn std::error::Error>>::Ok(())
            });

            match ret {
                Ok(_) => {}
                Err(err) => panic!("{err}"),
            }

            Ok(())
        }
    }
}

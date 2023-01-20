use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};
use std::io::{Error as IoError, ErrorKind as IoErrorKind};

use futures_util::io::{AsyncRead, AsyncWrite};

use crate::{Sleepble, SleepbleWaitBoxFuture};

//
//
//
pub trait AsyncReadWithTimeoutExt: AsyncRead {
    // ref https://github.com/rust-lang/futures-rs/blob/0.3.25/futures-util/src/io/mod.rs#L204
    fn read_with_timeout<'a, SLEEP: Sleepble>(
        &'a mut self,
        buf: &'a mut [u8],
        dur: Duration,
    ) -> ReadWithTimeout<'a, Self>
    where
        Self: Unpin,
    {
        ReadWithTimeout::new(self, buf, SLEEP::sleep(dur).wait())
    }
}

// ref https://github.com/rust-lang/futures-rs/blob/0.3.25/futures-util/src/io/mod.rs#L398
impl<R: AsyncRead + ?Sized> AsyncReadWithTimeoutExt for R {}

// ref https://github.com/rust-lang/futures-rs/blob/0.3.25/futures-util/src/io/read.rs
pub struct ReadWithTimeout<'a, R: ?Sized> {
    reader: &'a mut R,
    buf: &'a mut [u8],
    sleepble_wait_box_future: SleepbleWaitBoxFuture,
}

impl<R: ?Sized + Unpin> Unpin for ReadWithTimeout<'_, R> {}

impl<'a, R: AsyncRead + ?Sized + Unpin> ReadWithTimeout<'a, R> {
    fn new(
        reader: &'a mut R,
        buf: &'a mut [u8],
        sleepble_wait_box_future: SleepbleWaitBoxFuture,
    ) -> Self {
        Self {
            reader,
            buf,
            sleepble_wait_box_future,
        }
    }
}

impl<R: AsyncRead + ?Sized + Unpin> Future for ReadWithTimeout<'_, R> {
    type Output = Result<usize, IoError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = &mut *self;

        async_read_poll(
            &mut this.reader,
            this.buf,
            &mut this.sleepble_wait_box_future,
            cx,
        )
    }
}

//
//
//
pub trait AsyncWriteWithTimeoutExt: AsyncWrite {
    // ref https://github.com/rust-lang/futures-rs/blob/0.3.25/futures-util/src/io/mod.rs#L443
    fn write_with_timeout<'a, SLEEP: Sleepble>(
        &'a mut self,
        buf: &'a [u8],
        dur: Duration,
    ) -> WriteWithTimeout<'a, Self>
    where
        Self: Unpin,
    {
        WriteWithTimeout::new(self, buf, SLEEP::sleep(dur).wait())
    }
}

// ref https://github.com/rust-lang/futures-rs/blob/0.3.25/futures-util/src/io/mod.rs#L592
impl<W: AsyncWrite + ?Sized> AsyncWriteWithTimeoutExt for W {}

// ref https://github.com/rust-lang/futures-rs/blob/0.3.25/futures-util/src/io/write.rs
pub struct WriteWithTimeout<'a, W: ?Sized> {
    writer: &'a mut W,
    buf: &'a [u8],
    sleepble_wait_box_future: SleepbleWaitBoxFuture,
}

impl<W: ?Sized + Unpin> Unpin for WriteWithTimeout<'_, W> {}

impl<'a, W: AsyncWrite + ?Sized + Unpin> WriteWithTimeout<'a, W> {
    fn new(
        writer: &'a mut W,
        buf: &'a [u8],
        sleepble_wait_box_future: SleepbleWaitBoxFuture,
    ) -> Self {
        Self {
            writer,
            buf,
            sleepble_wait_box_future,
        }
    }
}

impl<W: AsyncWrite + ?Sized + Unpin> Future for WriteWithTimeout<'_, W> {
    type Output = Result<usize, IoError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = &mut *self;

        async_write_poll(
            &mut this.writer,
            this.buf,
            &mut this.sleepble_wait_box_future,
            cx,
        )
    }
}

//
//
//
pub fn async_read_poll<R: AsyncRead + ?Sized + Unpin>(
    reader: &mut R,
    buf: &mut [u8],
    sleepble_wait_box_future: &mut SleepbleWaitBoxFuture,
    cx: &mut Context<'_>,
) -> Poll<Result<usize, IoError>> {
    let poll_ret = Pin::new(reader).poll_read(cx, buf);

    match poll_ret {
        Poll::Ready(ret) => Poll::Ready(ret),
        Poll::Pending => match sleepble_wait_box_future.as_mut().poll(cx) {
            Poll::Ready(_) => Poll::Ready(Err(IoError::new(IoErrorKind::TimedOut, "read timeout"))),
            Poll::Pending => Poll::Pending,
        },
    }
}

pub fn async_write_poll<W: AsyncWrite + ?Sized + Unpin>(
    writer: &mut W,
    buf: &[u8],
    sleepble_wait_box_future: &mut SleepbleWaitBoxFuture,
    cx: &mut Context<'_>,
) -> Poll<Result<usize, IoError>> {
    let poll_ret = Pin::new(writer).poll_write(cx, buf);

    match poll_ret {
        Poll::Ready(ret) => Poll::Ready(ret),
        Poll::Pending => match sleepble_wait_box_future.as_mut().poll(cx) {
            Poll::Ready(_) => {
                Poll::Ready(Err(IoError::new(IoErrorKind::TimedOut, "write timeout")))
            }
            Poll::Pending => Poll::Pending,
        },
    }
}

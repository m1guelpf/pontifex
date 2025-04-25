use std::{
    fmt::Display,
    io,
    net::Shutdown,
    ops::{Deref, DerefMut},
};
use tokio::io::AsyncReadExt;
use tokio_vsock::{VsockAddr, VsockStream};

/// The piece of data that was being read/written when an error occurred.
#[derive(Debug)]
pub enum CodingKey {
    /// The length of the data.
    Length,
    /// The data itself.
    Payload,
}

impl Display for CodingKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Length => write!(f, "length"),
            Self::Payload => write!(f, "payload"),
        }
    }
}

pub struct Stream {
    stream: Option<VsockStream>,
}

impl Stream {
    pub const fn new(stream: VsockStream) -> Self {
        Self {
            stream: Some(stream),
        }
    }

    pub async fn connect(cid: u32, port: u32) -> io::Result<Self> {
        let stream = VsockStream::connect(VsockAddr::new(cid, port)).await?;

        Ok(Self {
            stream: Some(stream),
        })
    }

    pub async fn read_to_end(&mut self, buf: &mut Vec<u8>, len: u64) -> io::Result<usize> {
        let stream = self.stream.take().expect("stream should always be filled");
        let mut take = stream.take(len);
        let read_len = take.read_to_end(buf).await?;
        self.stream = Some(take.into_inner());
        Ok(read_len)
    }
}

impl Deref for Stream {
    type Target = VsockStream;

    fn deref(&self) -> &Self::Target {
        self.stream
            .as_ref()
            .expect("stream should always be filled")
    }
}

impl DerefMut for Stream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.stream
            .as_mut()
            .expect("stream should always be filled")
    }
}

impl Drop for Stream {
    fn drop(&mut self) {
        _ = self
            .stream
            .as_ref()
            .expect("stream should always be filled")
            .shutdown(Shutdown::Both);
    }
}

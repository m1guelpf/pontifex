#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

#[cfg(feature = "client")]
pub mod client;
#[cfg(feature = "client")]
pub use client::{ConnectionDetails, send};

#[cfg(feature = "server")]
pub mod server;
#[cfg(feature = "server")]
pub use server::listen;

mod utils;
pub use utils::CodingKey;

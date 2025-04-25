#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![doc = include_str!("../README.md")]

/// Client-side functionality.
#[cfg(feature = "client")]
pub mod client;
#[cfg(feature = "client")]
pub use client::{ConnectionDetails, send};

/// Server-side functionality.
#[cfg(feature = "server")]
pub mod server;
#[cfg(feature = "server")]
pub use server::listen;

mod utils;

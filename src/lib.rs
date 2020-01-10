//! # Async Server
//!
//! The crate contains various helpers for writing production-ready servers in
//! rust using [async-std](https://async.rs/).
//!
//! [Docs](https://docs.rs/async-server/) |
//! [Github](https://github.com/tailhook/async-server/) |
//! [Crate](https://crates.io/crates/async-server)
//!
//! # Utilities
//! * [ListenExt](trait.ListenExt.html) -- extension trait for stream of
//!   accepted sockets, provides useful conbinators for a stream
//!
//! # Low-Level Utilities
//!
//! * [is_transient_error](fn.is_transient_error.html) -- determines if the
//!   error returned from `accept()` can be ignored
//!
//! # Example
//!
//! Here is a quite elaborate example that demonstrates:
//! * Backpressure (limit on the number of simultaneous connections)
//! * Error handling
//! * Unification of Tcp and Unix sockets
//!
//! ```no_run
//! use std::error::Error;
//! use std::time::Duration;
//! use std::env::args;
//!
//! use async_std::task;
//! use async_std::net::TcpListener;
//! use async_std::os::unix::net::UnixListener;
//! use async_std::prelude::*;
//!
//! use async_server::{ListenExt, ByteStream, backpressure};
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let (_, bp) = backpressure::new(10);
//!     if args().any(|x| x == "--unix") {
//!         task::block_on(async {
//!             let listener = UnixListener::bind("./example.sock").await?;
//!             eprintln!("Accepting connections on ./example.sock");
//!             let mut incoming = listener.incoming()
//!                 .log_warnings(|e| eprintln!("Error: {}. Sleeping 0.5s", e))
//!                 .handle_errors(Duration::from_millis(500))
//!                 .backpressure_wrapper(bp);
//!             while let Some(stream) = incoming.next().await {
//!                 task::spawn(connection_loop(stream));
//!             }
//!             Ok(())
//!         })
//!     } else {
//!         task::block_on(async {
//!             let listener = TcpListener::bind("localhost:8080").await?;
//!             eprintln!("Accepting connections on localhost:8080");
//!             let mut incoming = listener.incoming()
//!                 .log_warnings(|e| eprintln!("Error: {}. Sleeping 0.5s", e))
//!                 .handle_errors(Duration::from_millis(500))
//!                 .backpressure_wrapper(bp);
//!             while let Some(stream) = incoming.next().await {
//!                 task::spawn(connection_loop(stream));
//!             }
//!             Ok(())
//!         })
//!     }
//! }
//!
//! async fn connection_loop(mut stream: ByteStream) {
//!     task::sleep(Duration::from_secs(5)).await;
//!     stream.write_all("hello".as_bytes()).await
//!         .map_err(|e| eprintln!("Write error: {}", e)).ok();
//! }
//! ```
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

mod error;
mod listen_ext;
mod log;
mod sleep;
mod byte_stream;
pub mod backpressure;
pub mod wrapper_types;

pub use byte_stream::ByteStream;
pub use error::is_transient_error;
pub use listen_ext::ListenExt;

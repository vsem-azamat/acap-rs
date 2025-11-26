//! Safe Rust bindings for the Axis VDO (Video Capture) API.
//!
//! This crate provides idiomatic Rust wrappers around the VDO library,
//! which is used for capturing video frames on Axis network cameras.
//!
//! # Features
//!
//! - **RAII memory management**: Buffers and streams are automatically cleaned up
//! - **Type-safe configuration**: Use Rust enums and types instead of raw C constants
//! - **Error handling**: All errors are converted to Rust's `Result` type
//!
//! # Example
//!
//! ```ignore
//! use vdo::{Stream, StreamSettings, Format};
//!
//! // Configure a video stream
//! let settings = StreamSettings::new()
//!     .format(Format::Yuv)
//!     .width(640)
//!     .height(480)
//!     .framerate(30.0);
//!
//! // Create and start the stream
//! let mut stream = Stream::new(&settings)?;
//! stream.start()?;
//!
//! // Capture frames
//! loop {
//!     let mut buffer = stream.get_buffer()?;
//!     if let Some(data) = buffer.data() {
//!         // Process frame data
//!         println!("Captured frame with {} bytes", data.len());
//!     }
//!     // Buffer is automatically released when dropped
//! }
//! ```
//!
//! # Snapshots
//!
//! For capturing single frames, use the [`snapshot`] function:
//!
//! ```ignore
//! use vdo::{snapshot, StreamSettings, Format};
//!
//! let settings = StreamSettings::new()
//!     .format(Format::Jpeg)
//!     .width(1920)
//!     .height(1080);
//!
//! let mut buffer = snapshot(&settings)?;
//! if let Some(data) = buffer.data() {
//!     // Save JPEG data to file
//!     std::fs::write("snapshot.jpg", data)?;
//! }
//! ```

mod buffer;
mod error;
mod format;
mod map;
mod stream;

pub use buffer::{Buffer, StandaloneBuffer};
pub use error::{Error, ErrorCode, Result};
pub use format::{Format, FrameType};
pub use map::Map;
pub use stream::{snapshot, Stream, StreamSettings};

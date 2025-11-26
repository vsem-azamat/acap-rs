//! Safe Rust bindings for the Axis VDO (Video Capture) API.

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

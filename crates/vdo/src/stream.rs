//! VDO Stream - video stream management with RAII.
//!
//! Provides a safe interface for creating and managing video streams.

use std::ptr;

use glib_sys::GError;
use vdo_sys::VdoStream as RawVdoStream;

use crate::buffer::{Buffer, StandaloneBuffer};
use crate::error::{check_gerror, Error, ErrorCode, Result};
use crate::format::Format;
use crate::map::Map;

/// A video stream for capturing frames from a camera.
///
/// The stream manages the lifecycle of video capture, including starting,
/// stopping, and retrieving frames. Resources are automatically cleaned up
/// when the stream is dropped.
///
/// # Example
///
/// ```ignore
/// use vdo::{Stream, StreamSettings, Format};
///
/// // Create stream settings
/// let settings = StreamSettings::new()
///     .format(Format::Yuv)
///     .width(640)
///     .height(480)
///     .framerate(30.0);
///
/// // Create and start the stream
/// let mut stream = Stream::new(&settings)?;
/// stream.start()?;
///
/// // Capture frames
/// for _ in 0..10 {
///     let buffer = stream.get_buffer()?;
///     if let Some(data) = buffer.data() {
///         println!("Got frame with {} bytes", data.len());
///     }
/// }
///
/// // Stream automatically stopped and cleaned up when dropped
/// ```
pub struct Stream {
    ptr: *mut RawVdoStream,
}

impl Stream {
    /// Creates a new video stream with the given settings.
    pub fn new(settings: &StreamSettings) -> Result<Self> {
        let mut error: *mut GError = ptr::null_mut();
        let ptr = unsafe { vdo_sys::vdo_stream_new(settings.map.as_ptr(), None, &mut error) };

        check_gerror!(error);

        if ptr.is_null() {
            return Err(Error::new(ErrorCode::Failed, "Failed to create stream"));
        }

        Ok(Self { ptr })
    }

    /// Returns the stream ID.
    pub fn id(&self) -> u32 {
        unsafe { vdo_sys::vdo_stream_get_id(self.ptr) }
    }

    /// Returns the file descriptor for the stream.
    pub fn fd(&self) -> Result<i32> {
        let mut error: *mut GError = ptr::null_mut();
        let fd = unsafe { vdo_sys::vdo_stream_get_fd(self.ptr, &mut error) };
        check_gerror!(error);
        Ok(fd)
    }

    /// Returns stream information.
    pub fn info(&self) -> Result<Map> {
        let mut error: *mut GError = ptr::null_mut();
        let ptr = unsafe { vdo_sys::vdo_stream_get_info(self.ptr, &mut error) };
        check_gerror!(error);

        unsafe { Map::from_raw(ptr) }
            .ok_or_else(|| Error::new(ErrorCode::Failed, "Failed to get stream info"))
    }

    /// Returns the current stream settings.
    pub fn settings(&self) -> Result<Map> {
        let mut error: *mut GError = ptr::null_mut();
        let ptr = unsafe { vdo_sys::vdo_stream_get_settings(self.ptr, &mut error) };
        check_gerror!(error);

        unsafe { Map::from_raw(ptr) }
            .ok_or_else(|| Error::new(ErrorCode::Failed, "Failed to get stream settings"))
    }

    /// Updates stream settings.
    pub fn set_settings(&mut self, settings: &Map) -> Result<()> {
        let mut error: *mut GError = ptr::null_mut();
        let success =
            unsafe { vdo_sys::vdo_stream_set_settings(self.ptr, settings.as_ptr(), &mut error) };
        check_gerror!(error);

        if success == 0 {
            return Err(Error::new(
                ErrorCode::Failed,
                "Failed to set stream settings",
            ));
        }
        Ok(())
    }

    /// Sets the framerate for the stream.
    pub fn set_framerate(&mut self, framerate: f64) -> Result<()> {
        let mut error: *mut GError = ptr::null_mut();
        let success = unsafe { vdo_sys::vdo_stream_set_framerate(self.ptr, framerate, &mut error) };
        check_gerror!(error);

        if success == 0 {
            return Err(Error::new(ErrorCode::Failed, "Failed to set framerate"));
        }
        Ok(())
    }

    /// Starts the video stream.
    pub fn start(&mut self) -> Result<()> {
        let mut error: *mut GError = ptr::null_mut();
        let success = unsafe { vdo_sys::vdo_stream_start(self.ptr, &mut error) };
        check_gerror!(error);

        if success == 0 {
            return Err(Error::new(ErrorCode::Failed, "Failed to start stream"));
        }
        Ok(())
    }

    /// Stops the video stream.
    pub fn stop(&mut self) {
        unsafe { vdo_sys::vdo_stream_stop(self.ptr) };
    }

    /// Forces the next frame to be a keyframe.
    pub fn force_keyframe(&mut self) -> Result<()> {
        let mut error: *mut GError = ptr::null_mut();
        let success = unsafe { vdo_sys::vdo_stream_force_key_frame(self.ptr, &mut error) };
        check_gerror!(error);

        if success == 0 {
            return Err(Error::new(ErrorCode::Failed, "Failed to force keyframe"));
        }
        Ok(())
    }

    /// Gets the next buffer from the stream.
    ///
    /// This function blocks until a buffer is available.
    pub fn get_buffer(&mut self) -> Result<Buffer> {
        let mut error: *mut GError = ptr::null_mut();
        let buffer_ptr = unsafe { vdo_sys::vdo_stream_get_buffer(self.ptr, &mut error) };
        check_gerror!(error);

        unsafe { Buffer::from_raw(buffer_ptr, self.ptr) }
            .ok_or_else(|| Error::new(ErrorCode::NoData, "No buffer available"))
    }

    /// Allocates a new buffer for the stream.
    pub fn allocate_buffer(&mut self) -> Result<Buffer> {
        let mut error: *mut GError = ptr::null_mut();
        let buffer_ptr =
            unsafe { vdo_sys::vdo_stream_buffer_alloc(self.ptr, ptr::null_mut(), &mut error) };
        check_gerror!(error);

        unsafe { Buffer::from_raw(buffer_ptr, self.ptr) }
            .ok_or_else(|| Error::new(ErrorCode::Oom, "Failed to allocate buffer"))
    }
}

impl Drop for Stream {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            // Stop the stream first
            unsafe { vdo_sys::vdo_stream_stop(self.ptr) };
            // Unref the stream
            unsafe { gobject_sys::g_object_unref(self.ptr as *mut _) };
        }
    }
}

unsafe impl Send for Stream {}

impl std::fmt::Debug for Stream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Stream")
            .field("id", &self.id())
            .field("ptr", &self.ptr)
            .finish()
    }
}

/// Builder for stream settings.
///
/// Provides a fluent API for configuring video stream parameters.
///
/// # Example
///
/// ```ignore
/// use vdo::{StreamSettings, Format};
///
/// let settings = StreamSettings::new()
///     .format(Format::Yuv)
///     .width(1920)
///     .height(1080)
///     .framerate(30.0);
/// ```
pub struct StreamSettings {
    pub(crate) map: Map,
}

impl StreamSettings {
    /// Creates new stream settings with default values.
    pub fn new() -> Self {
        Self {
            map: Map::new().expect("Failed to create settings map"),
        }
    }

    /// Sets the video format.
    pub fn format(mut self, format: Format) -> Self {
        let _ = self.map.set_uint32("format", format.to_raw().0 as u32);
        self
    }

    /// Sets the video width in pixels.
    pub fn width(mut self, width: u32) -> Self {
        let _ = self.map.set_uint32("width", width);
        self
    }

    /// Sets the video height in pixels.
    pub fn height(mut self, height: u32) -> Self {
        let _ = self.map.set_uint32("height", height);
        self
    }

    /// Sets the framerate in frames per second.
    pub fn framerate(mut self, fps: f64) -> Self {
        let _ = self.map.set_double("framerate", fps);
        self
    }

    /// Sets the buffer count for the stream.
    pub fn buffer_count(mut self, count: u32) -> Self {
        let _ = self.map.set_uint32("buffer.count", count);
        self
    }

    /// Sets a custom setting.
    pub fn set(mut self, key: &str, value: u32) -> Self {
        let _ = self.map.set_uint32(key, value);
        self
    }

    /// Sets a custom string setting.
    pub fn set_string(mut self, key: &str, value: &str) -> Self {
        let _ = self.map.set_string(key, value);
        self
    }

    /// Returns a reference to the underlying map.
    pub fn as_map(&self) -> &Map {
        &self.map
    }
}

impl Default for StreamSettings {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for StreamSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StreamSettings")
            .field("map", &self.map)
            .finish()
    }
}

/// Captures a single snapshot frame.
///
/// This is a convenience function that captures a single frame without
/// creating a persistent stream.
///
/// # Example
///
/// ```ignore
/// use vdo::{snapshot, StreamSettings, Format};
///
/// let settings = StreamSettings::new()
///     .format(Format::Jpeg)
///     .width(1920)
///     .height(1080);
///
/// let buffer = snapshot(&settings)?;
/// if let Some(data) = buffer.data() {
///     // Save JPEG data to file
/// }
/// ```
pub fn snapshot(settings: &StreamSettings) -> Result<StandaloneBuffer> {
    let mut error: *mut GError = ptr::null_mut();
    let buffer_ptr = unsafe { vdo_sys::vdo_stream_snapshot(settings.map.as_ptr(), &mut error) };
    check_gerror!(error);

    unsafe { StandaloneBuffer::from_raw(buffer_ptr) }
        .ok_or_else(|| Error::new(ErrorCode::Failed, "Failed to capture snapshot"))
}

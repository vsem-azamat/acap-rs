//! VDO Stream - video stream management with RAII.

use std::ptr;

use glib_sys::GError;
use log::warn;
use vdo_sys::VdoStream as RawVdoStream;

use crate::buffer::{Buffer, StandaloneBuffer};
use crate::error::{check_gerror, Error, ErrorCode, Result};
use crate::format::Format;
use crate::map::Map;

/// A video stream for capturing frames from a camera.
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

    pub fn id(&self) -> u32 {
        unsafe { vdo_sys::vdo_stream_get_id(self.ptr) }
    }

    pub fn fd(&self) -> Result<i32> {
        let mut error: *mut GError = ptr::null_mut();
        let fd = unsafe { vdo_sys::vdo_stream_get_fd(self.ptr, &mut error) };
        check_gerror!(error);
        Ok(fd)
    }

    pub fn info(&self) -> Result<Map> {
        let mut error: *mut GError = ptr::null_mut();
        let ptr = unsafe { vdo_sys::vdo_stream_get_info(self.ptr, &mut error) };
        check_gerror!(error);

        unsafe { Map::from_raw(ptr) }
            .ok_or_else(|| Error::new(ErrorCode::Failed, "Failed to get stream info"))
    }

    pub fn settings(&self) -> Result<Map> {
        let mut error: *mut GError = ptr::null_mut();
        let ptr = unsafe { vdo_sys::vdo_stream_get_settings(self.ptr, &mut error) };
        check_gerror!(error);

        unsafe { Map::from_raw(ptr) }
            .ok_or_else(|| Error::new(ErrorCode::Failed, "Failed to get stream settings"))
    }

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

    pub fn set_framerate(&mut self, framerate: f64) -> Result<()> {
        let mut error: *mut GError = ptr::null_mut();
        let success = unsafe { vdo_sys::vdo_stream_set_framerate(self.ptr, framerate, &mut error) };
        check_gerror!(error);

        if success == 0 {
            return Err(Error::new(ErrorCode::Failed, "Failed to set framerate"));
        }
        Ok(())
    }

    pub fn start(&mut self) -> Result<()> {
        let mut error: *mut GError = ptr::null_mut();
        let success = unsafe { vdo_sys::vdo_stream_start(self.ptr, &mut error) };
        check_gerror!(error);

        if success == 0 {
            return Err(Error::new(ErrorCode::Failed, "Failed to start stream"));
        }
        Ok(())
    }

    pub fn stop(&mut self) {
        unsafe { vdo_sys::vdo_stream_stop(self.ptr) };
    }

    pub fn force_keyframe(&mut self) -> Result<()> {
        let mut error: *mut GError = ptr::null_mut();
        let success = unsafe { vdo_sys::vdo_stream_force_key_frame(self.ptr, &mut error) };
        check_gerror!(error);

        if success == 0 {
            return Err(Error::new(ErrorCode::Failed, "Failed to force keyframe"));
        }
        Ok(())
    }

    /// Gets the next buffer from the stream. Blocks until a buffer is available.
    pub fn get_buffer(&mut self) -> Result<Buffer> {
        let mut error: *mut GError = ptr::null_mut();
        let buffer_ptr = unsafe { vdo_sys::vdo_stream_get_buffer(self.ptr, &mut error) };
        check_gerror!(error);

        unsafe { Buffer::from_raw(buffer_ptr, self.ptr) }
            .ok_or_else(|| Error::new(ErrorCode::NoData, "No buffer available"))
    }

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
            unsafe { vdo_sys::vdo_stream_stop(self.ptr) };
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
pub struct StreamSettings {
    pub(crate) map: Map,
}

impl StreamSettings {
    pub fn new() -> Self {
        Self {
            map: Map::new().expect("Failed to create settings map"),
        }
    }

    pub fn format(mut self, format: Format) -> Self {
        if let Err(e) = self.map.set_int32("format", format.as_i32()) {
            warn!("Failed to set format: {}", e);
        }
        self
    }

    /// Sets both width and height in one call to prevent ill-formed configurations.
    pub fn resolution(mut self, width: u32, height: u32) -> Self {
        if let Err(e) = self.map.set_uint32("width", width) {
            warn!("Failed to set width: {}", e);
        }
        if let Err(e) = self.map.set_uint32("height", height) {
            warn!("Failed to set height: {}", e);
        }
        self
    }

    pub fn framerate(mut self, fps: f64) -> Self {
        if let Err(e) = self.map.set_double("framerate", fps) {
            warn!("Failed to set framerate: {}", e);
        }
        self
    }

    pub fn buffer_count(mut self, count: u32) -> Self {
        if let Err(e) = self.map.set_uint32("buffer.count", count) {
            warn!("Failed to set buffer count: {}", e);
        }
        self
    }

    pub fn channel(mut self, channel: u32) -> Self {
        if let Err(e) = self.map.set_uint32("channel", channel) {
            warn!("Failed to set channel: {}", e);
        }
        self
    }

    pub fn set(mut self, key: &str, value: u32) -> Self {
        if let Err(e) = self.map.set_uint32(key, value) {
            warn!("Failed to set {}: {}", key, e);
        }
        self
    }

    pub fn set_string(mut self, key: &str, value: &str) -> Self {
        if let Err(e) = self.map.set_string(key, value) {
            warn!("Failed to set {}: {}", key, e);
        }
        self
    }

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
pub fn snapshot(settings: &StreamSettings) -> Result<StandaloneBuffer> {
    let mut error: *mut GError = ptr::null_mut();
    let buffer_ptr = unsafe { vdo_sys::vdo_stream_snapshot(settings.map.as_ptr(), &mut error) };
    check_gerror!(error);

    unsafe { StandaloneBuffer::from_raw(buffer_ptr) }
        .ok_or_else(|| Error::new(ErrorCode::Failed, "Failed to capture snapshot"))
}

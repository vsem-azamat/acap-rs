//! VDO Buffer - video frame buffer with RAII memory management.

use std::ptr;
use std::slice;

use vdo_sys::{VdoBuffer as RawVdoBuffer, VdoStream as RawVdoStream};

use crate::format::FrameType;
use crate::map::Map;

/// A video buffer containing frame data.
pub struct Buffer {
    ptr: *mut RawVdoBuffer,
    stream_ptr: *mut RawVdoStream,
}

impl Buffer {
    pub(crate) unsafe fn from_raw(
        buffer_ptr: *mut RawVdoBuffer,
        stream_ptr: *mut RawVdoStream,
    ) -> Option<Self> {
        if buffer_ptr.is_null() {
            None
        } else {
            Some(Self {
                ptr: buffer_ptr,
                stream_ptr,
            })
        }
    }

    pub fn id(&self) -> u32 {
        unsafe { vdo_sys::vdo_buffer_get_id(self.ptr) }
    }

    pub fn fd(&self) -> i32 {
        unsafe { vdo_sys::vdo_buffer_get_fd(self.ptr) }
    }

    pub fn capacity(&self) -> usize {
        unsafe { vdo_sys::vdo_buffer_get_capacity(self.ptr) }
    }

    pub fn size(&self) -> usize {
        unsafe { vdo_sys::vdo_frame_get_size(self.ptr) }
    }

    pub fn offset(&self) -> i64 {
        unsafe { vdo_sys::vdo_buffer_get_offset(self.ptr) }
    }

    pub fn is_complete(&self) -> bool {
        unsafe { vdo_sys::vdo_buffer_is_complete(self.ptr) != 0 }
    }

    pub fn frame_type(&self) -> Option<FrameType> {
        let raw = unsafe { vdo_sys::vdo_frame_get_frame_type(self.ptr) };
        FrameType::from_raw(raw)
    }

    pub fn is_keyframe(&self) -> bool {
        unsafe { vdo_sys::vdo_frame_is_key(self.ptr) != 0 }
    }

    pub fn sequence_number(&self) -> u32 {
        unsafe { vdo_sys::vdo_frame_get_sequence_nbr(self.ptr) }
    }

    pub fn timestamp(&self) -> u64 {
        unsafe { vdo_sys::vdo_frame_get_timestamp(self.ptr) }
    }

    pub fn custom_timestamp(&self) -> i64 {
        unsafe { vdo_sys::vdo_frame_get_custom_timestamp(self.ptr) }
    }

    pub fn is_last(&self) -> bool {
        unsafe { vdo_sys::vdo_frame_get_is_last_buffer(self.ptr) != 0 }
    }

    pub fn extra_info(&self) -> Option<Map> {
        let ptr = unsafe { vdo_sys::vdo_frame_get_extra_info(self.ptr) };
        unsafe { Map::from_raw(ptr) }
    }

    /// Returns a slice to the buffer data. Uses cached mmap internally.
    pub fn data(&self) -> Option<&[u8]> {
        let data_ptr = unsafe { vdo_sys::vdo_buffer_get_data(self.ptr) };
        if data_ptr.is_null() {
            return None;
        }

        let size = self.size();
        if size == 0 {
            return Some(&[]);
        }

        Some(unsafe { slice::from_raw_parts(data_ptr as *const u8, size) })
    }

    /// Returns a mutable slice to the buffer data.
    pub fn data_mut(&mut self) -> Option<&mut [u8]> {
        let data_ptr = unsafe { vdo_sys::vdo_buffer_get_data(self.ptr) };
        if data_ptr.is_null() {
            return None;
        }

        let size = self.size();
        if size == 0 {
            return Some(&mut []);
        }

        Some(unsafe { slice::from_raw_parts_mut(data_ptr as *mut u8, size) })
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        if !self.ptr.is_null() && !self.stream_ptr.is_null() {
            let mut ptr = self.ptr;
            let mut error: *mut glib_sys::GError = ptr::null_mut();
            unsafe {
                let _ = vdo_sys::vdo_stream_buffer_unref(self.stream_ptr, &mut ptr, &mut error);
                if !error.is_null() {
                    glib_sys::g_error_free(error);
                }
            }
        }
    }
}

unsafe impl Send for Buffer {}

impl std::fmt::Debug for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Buffer")
            .field("id", &self.id())
            .field("size", &self.size())
            .field("capacity", &self.capacity())
            .field("sequence_number", &self.sequence_number())
            .field("is_keyframe", &self.is_keyframe())
            .finish()
    }
}

/// A standalone buffer not associated with a stream, used for snapshots.
pub struct StandaloneBuffer {
    ptr: *mut RawVdoBuffer,
}

impl StandaloneBuffer {
    pub(crate) unsafe fn from_raw(ptr: *mut RawVdoBuffer) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr })
        }
    }

    pub fn id(&self) -> u32 {
        unsafe { vdo_sys::vdo_buffer_get_id(self.ptr) }
    }

    pub fn fd(&self) -> i32 {
        unsafe { vdo_sys::vdo_buffer_get_fd(self.ptr) }
    }

    pub fn capacity(&self) -> usize {
        unsafe { vdo_sys::vdo_buffer_get_capacity(self.ptr) }
    }

    pub fn size(&self) -> usize {
        unsafe { vdo_sys::vdo_frame_get_size(self.ptr) }
    }

    pub fn frame_type(&self) -> Option<FrameType> {
        let raw = unsafe { vdo_sys::vdo_frame_get_frame_type(self.ptr) };
        FrameType::from_raw(raw)
    }

    pub fn is_keyframe(&self) -> bool {
        unsafe { vdo_sys::vdo_frame_is_key(self.ptr) != 0 }
    }

    pub fn sequence_number(&self) -> u32 {
        unsafe { vdo_sys::vdo_frame_get_sequence_nbr(self.ptr) }
    }

    pub fn timestamp(&self) -> u64 {
        unsafe { vdo_sys::vdo_frame_get_timestamp(self.ptr) }
    }

    pub fn data(&self) -> Option<&[u8]> {
        let data_ptr = unsafe { vdo_sys::vdo_buffer_get_data(self.ptr) };
        if data_ptr.is_null() {
            return None;
        }

        let size = self.size();
        if size == 0 {
            return Some(&[]);
        }

        Some(unsafe { slice::from_raw_parts(data_ptr as *const u8, size) })
    }
}

impl Drop for StandaloneBuffer {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                gobject_sys::g_object_unref(self.ptr as *mut _);
            }
        }
    }
}

unsafe impl Send for StandaloneBuffer {}

impl std::fmt::Debug for StandaloneBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StandaloneBuffer")
            .field("id", &self.id())
            .field("size", &self.size())
            .field("capacity", &self.capacity())
            .finish()
    }
}

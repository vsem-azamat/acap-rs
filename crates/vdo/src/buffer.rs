//! VDO Buffer - represents a video frame buffer with RAII memory management.
//!
//! Buffers are used to hold video frame data from VDO streams.
//! They implement RAII-style memory management, automatically releasing
//! resources when dropped.

use std::ptr;
use std::slice;

use vdo_sys::{VdoBuffer as RawVdoBuffer, VdoStream as RawVdoStream};

use crate::format::FrameType;
use crate::map::Map;

/// A video buffer containing frame data.
///
/// This struct provides safe access to video frame data and metadata.
/// When dropped, the buffer is automatically unreferenced, preventing memory leaks.
///
/// # Example
///
/// ```ignore
/// use vdo::Stream;
///
/// let mut stream = Stream::new(&settings)?;
/// stream.start()?;
///
/// // Get a buffer from the stream
/// let buffer = stream.get_buffer()?;
///
/// // Access frame data
/// if let Some(data) = buffer.data() {
///     println!("Got {} bytes of frame data", data.len());
/// }
///
/// // Buffer is automatically unreferenced when dropped
/// ```
pub struct Buffer {
    ptr: *mut RawVdoBuffer,
    stream_ptr: *mut RawVdoStream,
    mapped_data: *mut std::ffi::c_void,
}

impl Buffer {
    /// Creates a Buffer from raw pointers.
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    /// - `buffer_ptr` is a valid pointer to a VdoBuffer
    /// - `stream_ptr` is a valid pointer to the VdoStream that owns this buffer
    /// - Ownership of the buffer is transferred to this struct
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
                mapped_data: ptr::null_mut(),
            })
        }
    }

    /// Returns the buffer ID.
    pub fn id(&self) -> u32 {
        unsafe { vdo_sys::vdo_buffer_get_id(self.ptr) }
    }

    /// Returns the file descriptor associated with this buffer.
    pub fn fd(&self) -> i32 {
        unsafe { vdo_sys::vdo_buffer_get_fd(self.ptr) }
    }

    /// Returns the buffer capacity in bytes.
    pub fn capacity(&self) -> usize {
        unsafe { vdo_sys::vdo_buffer_get_capacity(self.ptr) }
    }

    /// Returns the actual data size in bytes.
    pub fn size(&self) -> usize {
        unsafe { vdo_sys::vdo_frame_get_size(self.ptr) }
    }

    /// Returns the buffer offset.
    pub fn offset(&self) -> i64 {
        unsafe { vdo_sys::vdo_buffer_get_offset(self.ptr) }
    }

    /// Returns `true` if this buffer contains a complete frame.
    pub fn is_complete(&self) -> bool {
        unsafe { vdo_sys::vdo_buffer_is_complete(self.ptr) != 0 }
    }

    /// Returns the frame type.
    pub fn frame_type(&self) -> Option<FrameType> {
        let raw = unsafe { vdo_sys::vdo_frame_get_frame_type(self.ptr) };
        FrameType::from_raw(raw)
    }

    /// Returns `true` if this is a keyframe.
    pub fn is_keyframe(&self) -> bool {
        unsafe { vdo_sys::vdo_frame_is_key(self.ptr) != 0 }
    }

    /// Returns the sequence number of this frame.
    pub fn sequence_number(&self) -> u32 {
        unsafe { vdo_sys::vdo_frame_get_sequence_nbr(self.ptr) }
    }

    /// Returns the timestamp of this frame.
    pub fn timestamp(&self) -> u64 {
        unsafe { vdo_sys::vdo_frame_get_timestamp(self.ptr) }
    }

    /// Returns the custom timestamp of this frame.
    pub fn custom_timestamp(&self) -> i64 {
        unsafe { vdo_sys::vdo_frame_get_custom_timestamp(self.ptr) }
    }

    /// Returns `true` if this is the last buffer in a sequence.
    pub fn is_last(&self) -> bool {
        unsafe { vdo_sys::vdo_frame_get_is_last_buffer(self.ptr) != 0 }
    }

    /// Returns extra metadata about this frame.
    pub fn extra_info(&self) -> Option<Map> {
        let ptr = unsafe { vdo_sys::vdo_frame_get_extra_info(self.ptr) };
        unsafe { Map::from_raw(ptr) }
    }

    /// Maps the buffer memory for CPU access and returns a slice to the data.
    ///
    /// The data remains accessible until the buffer is dropped.
    ///
    /// # Returns
    ///
    /// Returns `None` if the buffer cannot be mapped.
    pub fn data(&mut self) -> Option<&[u8]> {
        if self.mapped_data.is_null() {
            self.mapped_data = unsafe { vdo_sys::vdo_frame_memmap(self.ptr) };
        }

        if self.mapped_data.is_null() {
            return None;
        }

        let size = self.size();
        if size == 0 {
            return Some(&[]);
        }

        Some(unsafe { slice::from_raw_parts(self.mapped_data as *const u8, size) })
    }

    /// Returns a mutable reference to the mapped buffer data.
    ///
    /// # Returns
    ///
    /// Returns `None` if the buffer cannot be mapped.
    pub fn data_mut(&mut self) -> Option<&mut [u8]> {
        if self.mapped_data.is_null() {
            self.mapped_data = unsafe { vdo_sys::vdo_frame_memmap(self.ptr) };
        }

        if self.mapped_data.is_null() {
            return None;
        }

        let size = self.size();
        if size == 0 {
            return Some(&mut []);
        }

        Some(unsafe { slice::from_raw_parts_mut(self.mapped_data as *mut u8, size) })
    }

    /// Unmaps the buffer memory.
    ///
    /// This is called automatically when the buffer is dropped.
    pub fn unmap(&mut self) {
        if !self.mapped_data.is_null() {
            unsafe { vdo_sys::vdo_frame_unmap(self.ptr) };
            self.mapped_data = ptr::null_mut();
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        // Unmap if mapped
        self.unmap();

        // Unref the buffer through the stream
        if !self.ptr.is_null() && !self.stream_ptr.is_null() {
            let mut ptr = self.ptr;
            let mut error: *mut glib_sys::GError = ptr::null_mut();
            unsafe {
                // Ignore errors during drop - we can't do much about them
                let _ = vdo_sys::vdo_stream_buffer_unref(self.stream_ptr, &mut ptr, &mut error);
                if !error.is_null() {
                    glib_sys::g_error_free(error);
                }
            }
        }
    }
}

// SAFETY: Buffer can be sent between threads
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

/// A standalone buffer not associated with a stream.
///
/// Used for snapshots and other operations that don't require a stream.
pub struct StandaloneBuffer {
    ptr: *mut RawVdoBuffer,
    mapped_data: *mut std::ffi::c_void,
}

impl StandaloneBuffer {
    /// Creates a StandaloneBuffer from a raw pointer.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the pointer is valid and ownership is transferred.
    pub(crate) unsafe fn from_raw(ptr: *mut RawVdoBuffer) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self {
                ptr,
                mapped_data: ptr::null_mut(),
            })
        }
    }

    /// Returns the buffer ID.
    pub fn id(&self) -> u32 {
        unsafe { vdo_sys::vdo_buffer_get_id(self.ptr) }
    }

    /// Returns the file descriptor associated with this buffer.
    pub fn fd(&self) -> i32 {
        unsafe { vdo_sys::vdo_buffer_get_fd(self.ptr) }
    }

    /// Returns the buffer capacity in bytes.
    pub fn capacity(&self) -> usize {
        unsafe { vdo_sys::vdo_buffer_get_capacity(self.ptr) }
    }

    /// Returns the actual data size in bytes.
    pub fn size(&self) -> usize {
        unsafe { vdo_sys::vdo_frame_get_size(self.ptr) }
    }

    /// Returns the frame type.
    pub fn frame_type(&self) -> Option<FrameType> {
        let raw = unsafe { vdo_sys::vdo_frame_get_frame_type(self.ptr) };
        FrameType::from_raw(raw)
    }

    /// Returns `true` if this is a keyframe.
    pub fn is_keyframe(&self) -> bool {
        unsafe { vdo_sys::vdo_frame_is_key(self.ptr) != 0 }
    }

    /// Returns the sequence number of this frame.
    pub fn sequence_number(&self) -> u32 {
        unsafe { vdo_sys::vdo_frame_get_sequence_nbr(self.ptr) }
    }

    /// Returns the timestamp of this frame.
    pub fn timestamp(&self) -> u64 {
        unsafe { vdo_sys::vdo_frame_get_timestamp(self.ptr) }
    }

    /// Maps the buffer memory and returns a slice to the data.
    pub fn data(&mut self) -> Option<&[u8]> {
        if self.mapped_data.is_null() {
            self.mapped_data = unsafe { vdo_sys::vdo_frame_memmap(self.ptr) };
        }

        if self.mapped_data.is_null() {
            return None;
        }

        let size = self.size();
        if size == 0 {
            return Some(&[]);
        }

        Some(unsafe { slice::from_raw_parts(self.mapped_data as *const u8, size) })
    }

    /// Unmaps the buffer memory.
    pub fn unmap(&mut self) {
        if !self.mapped_data.is_null() {
            unsafe { vdo_sys::vdo_frame_unmap(self.ptr) };
            self.mapped_data = ptr::null_mut();
        }
    }
}

impl Drop for StandaloneBuffer {
    fn drop(&mut self) {
        self.unmap();
        if !self.ptr.is_null() {
            unsafe {
                // Standalone buffers need to be unreffed via GObject
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

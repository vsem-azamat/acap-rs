//! VDO Map - a key-value configuration container.
//!
//! VdoMap is used throughout the VDO API for passing settings and configuration.

use std::ffi::{CStr, CString};
use std::ptr;

use vdo_sys::VdoMap as RawVdoMap;

use crate::error::{Error, ErrorCode, Result};

/// A key-value map for VDO configuration.
///
/// This is a safe wrapper around `VdoMap` that provides RAII-based memory management
/// and a type-safe interface for setting and getting values.
///
/// # Example
///
/// ```ignore
/// use vdo::Map;
///
/// let mut map = Map::new()?;
/// map.set_uint32("width", 1920)?;
/// map.set_uint32("height", 1080)?;
/// map.set_string("format", "h264")?;
/// ```
pub struct Map {
    ptr: *mut RawVdoMap,
}

impl Map {
    /// Creates a new empty VDO map.
    pub fn new() -> Result<Self> {
        let ptr = unsafe { vdo_sys::vdo_map_new() };
        if ptr.is_null() {
            return Err(Error::new(ErrorCode::Oom, "Failed to create VdoMap"));
        }
        Ok(Self { ptr })
    }

    /// Returns the raw pointer to the underlying VdoMap.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the pointer is not used after the Map is dropped.
    pub(crate) fn as_ptr(&self) -> *mut RawVdoMap {
        self.ptr
    }

    /// Creates a Map from a raw pointer.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the pointer is valid and that ownership is transferred.
    pub(crate) unsafe fn from_raw(ptr: *mut RawVdoMap) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr })
        }
    }

    /// Returns `true` if the map is empty.
    pub fn is_empty(&self) -> bool {
        unsafe { vdo_sys::vdo_map_empty(self.ptr) != 0 }
    }

    /// Returns the number of entries in the map.
    pub fn len(&self) -> usize {
        unsafe { vdo_sys::vdo_map_size(self.ptr) }
    }

    /// Returns `true` if the map contains the specified key.
    pub fn contains(&self, key: &str) -> bool {
        let c_key = match CString::new(key) {
            Ok(s) => s,
            Err(_) => return false,
        };
        unsafe { vdo_sys::vdo_map_contains(self.ptr, c_key.as_ptr()) != 0 }
    }

    /// Removes an entry from the map.
    pub fn remove(&mut self, key: &str) {
        if let Ok(c_key) = CString::new(key) {
            unsafe { vdo_sys::vdo_map_remove(self.ptr, c_key.as_ptr()) }
        }
    }

    /// Clears all entries from the map.
    pub fn clear(&mut self) {
        unsafe { vdo_sys::vdo_map_clear(self.ptr) }
    }

    /// Sets a boolean value.
    pub fn set_boolean(&mut self, key: &str, value: bool) -> Result<()> {
        let c_key = CString::new(key)
            .map_err(|_| Error::new(ErrorCode::InvalidArgument, "Invalid key string"))?;
        unsafe {
            vdo_sys::vdo_map_set_boolean(self.ptr, c_key.as_ptr(), if value { 1 } else { 0 });
        }
        Ok(())
    }

    /// Gets a boolean value.
    pub fn get_boolean(&self, key: &str, default: bool) -> bool {
        let c_key = match CString::new(key) {
            Ok(s) => s,
            Err(_) => return default,
        };
        let raw_default = if default { 1 } else { 0 };
        unsafe { vdo_sys::vdo_map_get_boolean(self.ptr, c_key.as_ptr(), raw_default) != 0 }
    }

    /// Sets a 32-bit signed integer value.
    pub fn set_int32(&mut self, key: &str, value: i32) -> Result<()> {
        let c_key = CString::new(key)
            .map_err(|_| Error::new(ErrorCode::InvalidArgument, "Invalid key string"))?;
        unsafe {
            vdo_sys::vdo_map_set_int32(self.ptr, c_key.as_ptr(), value);
        }
        Ok(())
    }

    /// Gets a 32-bit signed integer value.
    pub fn get_int32(&self, key: &str, default: i32) -> i32 {
        let c_key = match CString::new(key) {
            Ok(s) => s,
            Err(_) => return default,
        };
        unsafe { vdo_sys::vdo_map_get_int32(self.ptr, c_key.as_ptr(), default) }
    }

    /// Sets a 32-bit unsigned integer value.
    pub fn set_uint32(&mut self, key: &str, value: u32) -> Result<()> {
        let c_key = CString::new(key)
            .map_err(|_| Error::new(ErrorCode::InvalidArgument, "Invalid key string"))?;
        unsafe {
            vdo_sys::vdo_map_set_uint32(self.ptr, c_key.as_ptr(), value);
        }
        Ok(())
    }

    /// Gets a 32-bit unsigned integer value.
    pub fn get_uint32(&self, key: &str, default: u32) -> u32 {
        let c_key = match CString::new(key) {
            Ok(s) => s,
            Err(_) => return default,
        };
        unsafe { vdo_sys::vdo_map_get_uint32(self.ptr, c_key.as_ptr(), default) }
    }

    /// Sets a 64-bit signed integer value.
    pub fn set_int64(&mut self, key: &str, value: i64) -> Result<()> {
        let c_key = CString::new(key)
            .map_err(|_| Error::new(ErrorCode::InvalidArgument, "Invalid key string"))?;
        unsafe {
            vdo_sys::vdo_map_set_int64(self.ptr, c_key.as_ptr(), value);
        }
        Ok(())
    }

    /// Gets a 64-bit signed integer value.
    pub fn get_int64(&self, key: &str, default: i64) -> i64 {
        let c_key = match CString::new(key) {
            Ok(s) => s,
            Err(_) => return default,
        };
        unsafe { vdo_sys::vdo_map_get_int64(self.ptr, c_key.as_ptr(), default) }
    }

    /// Sets a 64-bit unsigned integer value.
    pub fn set_uint64(&mut self, key: &str, value: u64) -> Result<()> {
        let c_key = CString::new(key)
            .map_err(|_| Error::new(ErrorCode::InvalidArgument, "Invalid key string"))?;
        unsafe {
            vdo_sys::vdo_map_set_uint64(self.ptr, c_key.as_ptr(), value);
        }
        Ok(())
    }

    /// Gets a 64-bit unsigned integer value.
    pub fn get_uint64(&self, key: &str, default: u64) -> u64 {
        let c_key = match CString::new(key) {
            Ok(s) => s,
            Err(_) => return default,
        };
        unsafe { vdo_sys::vdo_map_get_uint64(self.ptr, c_key.as_ptr(), default) }
    }

    /// Sets a double value.
    pub fn set_double(&mut self, key: &str, value: f64) -> Result<()> {
        let c_key = CString::new(key)
            .map_err(|_| Error::new(ErrorCode::InvalidArgument, "Invalid key string"))?;
        unsafe {
            vdo_sys::vdo_map_set_double(self.ptr, c_key.as_ptr(), value);
        }
        Ok(())
    }

    /// Gets a double value.
    pub fn get_double(&self, key: &str, default: f64) -> f64 {
        let c_key = match CString::new(key) {
            Ok(s) => s,
            Err(_) => return default,
        };
        unsafe { vdo_sys::vdo_map_get_double(self.ptr, c_key.as_ptr(), default) }
    }

    /// Sets a string value.
    pub fn set_string(&mut self, key: &str, value: &str) -> Result<()> {
        let c_key = CString::new(key)
            .map_err(|_| Error::new(ErrorCode::InvalidArgument, "Invalid key string"))?;
        let c_value = CString::new(value)
            .map_err(|_| Error::new(ErrorCode::InvalidArgument, "Invalid value string"))?;
        unsafe {
            vdo_sys::vdo_map_set_string(self.ptr, c_key.as_ptr(), c_value.as_ptr());
        }
        Ok(())
    }

    /// Gets a string value.
    ///
    /// Returns `None` if the key is not found or if the string cannot be converted to UTF-8.
    pub fn get_string(&self, key: &str) -> Option<String> {
        let c_key = CString::new(key).ok()?;
        let ptr = unsafe {
            vdo_sys::vdo_map_get_string(self.ptr, c_key.as_ptr(), ptr::null_mut(), ptr::null())
        };
        if ptr.is_null() {
            return None;
        }
        unsafe { CStr::from_ptr(ptr).to_str().ok().map(|s| s.to_owned()) }
    }

    /// Merges another map into this one.
    ///
    /// Values from the other map will overwrite values in this map for matching keys.
    pub fn merge(&mut self, other: &Map) {
        unsafe {
            vdo_sys::vdo_map_merge(self.ptr, other.ptr);
        }
    }
}

impl Drop for Map {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                // VdoMap is a GObject, so we need to unref it
                gobject_sys::g_object_unref(self.ptr as *mut _);
            }
        }
    }
}

// SAFETY: Map is safe to send between threads as long as it's not accessed concurrently.
// The underlying VdoMap doesn't have thread-local state.
unsafe impl Send for Map {}

impl Clone for Map {
    fn clone(&self) -> Self {
        unsafe {
            // Increment reference count
            gobject_sys::g_object_ref(self.ptr as *mut _);
            Self { ptr: self.ptr }
        }
    }
}

impl Default for Map {
    fn default() -> Self {
        Self::new().expect("Failed to create default VdoMap")
    }
}

impl std::fmt::Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Map")
            .field("len", &self.len())
            .field("ptr", &self.ptr)
            .finish()
    }
}

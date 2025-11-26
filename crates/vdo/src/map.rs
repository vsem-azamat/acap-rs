//! VDO Map - key-value configuration container.

use std::ffi::{CStr, CString};
use std::ptr;

use vdo_sys::VdoMap as RawVdoMap;

use crate::error::{Error, ErrorCode, Result};

/// A key-value map for VDO configuration.
pub struct Map {
    ptr: *mut RawVdoMap,
}

impl Map {
    pub fn new() -> Result<Self> {
        let ptr = unsafe { vdo_sys::vdo_map_new() };
        if ptr.is_null() {
            return Err(Error::new(ErrorCode::Oom, "Failed to create VdoMap"));
        }
        Ok(Self { ptr })
    }

    pub(crate) fn as_ptr(&self) -> *mut RawVdoMap {
        self.ptr
    }

    pub(crate) unsafe fn from_raw(ptr: *mut RawVdoMap) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr })
        }
    }

    pub fn is_empty(&self) -> bool {
        unsafe { vdo_sys::vdo_map_empty(self.ptr) != 0 }
    }

    pub fn len(&self) -> usize {
        unsafe { vdo_sys::vdo_map_size(self.ptr) }
    }

    pub fn contains(&self, key: &str) -> bool {
        let c_key = match CString::new(key) {
            Ok(s) => s,
            Err(_) => return false,
        };
        unsafe { vdo_sys::vdo_map_contains(self.ptr, c_key.as_ptr()) != 0 }
    }

    pub fn remove(&mut self, key: &str) {
        if let Ok(c_key) = CString::new(key) {
            unsafe { vdo_sys::vdo_map_remove(self.ptr, c_key.as_ptr()) }
        }
    }

    pub fn clear(&mut self) {
        unsafe { vdo_sys::vdo_map_clear(self.ptr) }
    }

    pub fn set_boolean(&mut self, key: &str, value: bool) -> Result<()> {
        let c_key = CString::new(key)
            .map_err(|_| Error::new(ErrorCode::InvalidArgument, "Invalid key string"))?;
        unsafe {
            vdo_sys::vdo_map_set_boolean(self.ptr, c_key.as_ptr(), if value { 1 } else { 0 });
        }
        Ok(())
    }

    pub fn get_boolean(&self, key: &str, default: bool) -> bool {
        let c_key = match CString::new(key) {
            Ok(s) => s,
            Err(_) => return default,
        };
        let raw_default = if default { 1 } else { 0 };
        unsafe { vdo_sys::vdo_map_get_boolean(self.ptr, c_key.as_ptr(), raw_default) != 0 }
    }

    pub fn set_int32(&mut self, key: &str, value: i32) -> Result<()> {
        let c_key = CString::new(key)
            .map_err(|_| Error::new(ErrorCode::InvalidArgument, "Invalid key string"))?;
        unsafe {
            vdo_sys::vdo_map_set_int32(self.ptr, c_key.as_ptr(), value);
        }
        Ok(())
    }

    pub fn get_int32(&self, key: &str, default: i32) -> i32 {
        let c_key = match CString::new(key) {
            Ok(s) => s,
            Err(_) => return default,
        };
        unsafe { vdo_sys::vdo_map_get_int32(self.ptr, c_key.as_ptr(), default) }
    }

    pub fn set_uint32(&mut self, key: &str, value: u32) -> Result<()> {
        let c_key = CString::new(key)
            .map_err(|_| Error::new(ErrorCode::InvalidArgument, "Invalid key string"))?;
        unsafe {
            vdo_sys::vdo_map_set_uint32(self.ptr, c_key.as_ptr(), value);
        }
        Ok(())
    }

    pub fn get_uint32(&self, key: &str, default: u32) -> u32 {
        let c_key = match CString::new(key) {
            Ok(s) => s,
            Err(_) => return default,
        };
        unsafe { vdo_sys::vdo_map_get_uint32(self.ptr, c_key.as_ptr(), default) }
    }

    pub fn set_int64(&mut self, key: &str, value: i64) -> Result<()> {
        let c_key = CString::new(key)
            .map_err(|_| Error::new(ErrorCode::InvalidArgument, "Invalid key string"))?;
        unsafe {
            vdo_sys::vdo_map_set_int64(self.ptr, c_key.as_ptr(), value);
        }
        Ok(())
    }

    pub fn get_int64(&self, key: &str, default: i64) -> i64 {
        let c_key = match CString::new(key) {
            Ok(s) => s,
            Err(_) => return default,
        };
        unsafe { vdo_sys::vdo_map_get_int64(self.ptr, c_key.as_ptr(), default) }
    }

    pub fn set_uint64(&mut self, key: &str, value: u64) -> Result<()> {
        let c_key = CString::new(key)
            .map_err(|_| Error::new(ErrorCode::InvalidArgument, "Invalid key string"))?;
        unsafe {
            vdo_sys::vdo_map_set_uint64(self.ptr, c_key.as_ptr(), value);
        }
        Ok(())
    }

    pub fn get_uint64(&self, key: &str, default: u64) -> u64 {
        let c_key = match CString::new(key) {
            Ok(s) => s,
            Err(_) => return default,
        };
        unsafe { vdo_sys::vdo_map_get_uint64(self.ptr, c_key.as_ptr(), default) }
    }

    pub fn set_double(&mut self, key: &str, value: f64) -> Result<()> {
        let c_key = CString::new(key)
            .map_err(|_| Error::new(ErrorCode::InvalidArgument, "Invalid key string"))?;
        unsafe {
            vdo_sys::vdo_map_set_double(self.ptr, c_key.as_ptr(), value);
        }
        Ok(())
    }

    pub fn get_double(&self, key: &str, default: f64) -> f64 {
        let c_key = match CString::new(key) {
            Ok(s) => s,
            Err(_) => return default,
        };
        unsafe { vdo_sys::vdo_map_get_double(self.ptr, c_key.as_ptr(), default) }
    }

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
                gobject_sys::g_object_unref(self.ptr as *mut _);
            }
        }
    }
}

unsafe impl Send for Map {}

impl Clone for Map {
    fn clone(&self) -> Self {
        unsafe {
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

//! Error types for the VDO API.

use std::ffi::CStr;
use std::fmt;

use glib_sys::GError;

/// Error codes returned by the VDO API.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorCode {
    /// Resource not found.
    NotFound,
    /// Resource already exists.
    Exists,
    /// Invalid argument provided.
    InvalidArgument,
    /// Permission denied.
    PermissionDenied,
    /// Operation not supported.
    NotSupported,
    /// Resource is closed.
    Closed,
    /// Resource is busy.
    Busy,
    /// I/O error.
    Io,
    /// HAL error.
    Hal,
    /// D-Bus error.
    Dbus,
    /// Out of memory.
    Oom,
    /// Resource is idle.
    Idle,
    /// No data available.
    NoData,
    /// No buffer space available.
    NoBufferSpace,
    /// Buffer failure.
    BufferFailure,
    /// Interface is down.
    InterfaceDown,
    /// General failure.
    Failed,
    /// Fatal error.
    Fatal,
    /// Not controlled.
    NotControlled,
    /// No event.
    NoEvent,
    /// No video.
    NoVideo,
    /// Unknown error code.
    Unknown(u32),
}

impl ErrorCode {
    fn from_raw(code: u32) -> Self {
        match code {
            1 => Self::NotFound,
            2 => Self::Exists,
            3 => Self::InvalidArgument,
            4 => Self::PermissionDenied,
            5 => Self::NotSupported,
            6 => Self::Closed,
            7 => Self::Busy,
            8 => Self::Io,
            9 => Self::Hal,
            10 => Self::Dbus,
            11 => Self::Oom,
            12 => Self::Idle,
            13 => Self::NoData,
            14 => Self::NoBufferSpace,
            15 => Self::BufferFailure,
            16 => Self::InterfaceDown,
            17 => Self::Failed,
            18 => Self::Fatal,
            19 => Self::NotControlled,
            20 => Self::NoEvent,
            21 => Self::NoVideo,
            _ => Self::Unknown(code),
        }
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "not found"),
            Self::Exists => write!(f, "already exists"),
            Self::InvalidArgument => write!(f, "invalid argument"),
            Self::PermissionDenied => write!(f, "permission denied"),
            Self::NotSupported => write!(f, "not supported"),
            Self::Closed => write!(f, "closed"),
            Self::Busy => write!(f, "busy"),
            Self::Io => write!(f, "I/O error"),
            Self::Hal => write!(f, "HAL error"),
            Self::Dbus => write!(f, "D-Bus error"),
            Self::Oom => write!(f, "out of memory"),
            Self::Idle => write!(f, "idle"),
            Self::NoData => write!(f, "no data"),
            Self::NoBufferSpace => write!(f, "no buffer space"),
            Self::BufferFailure => write!(f, "buffer failure"),
            Self::InterfaceDown => write!(f, "interface down"),
            Self::Failed => write!(f, "failed"),
            Self::Fatal => write!(f, "fatal error"),
            Self::NotControlled => write!(f, "not controlled"),
            Self::NoEvent => write!(f, "no event"),
            Self::NoVideo => write!(f, "no video"),
            Self::Unknown(code) => write!(f, "unknown error ({})", code),
        }
    }
}

/// Error type for VDO operations.
#[derive(Debug)]
pub struct Error {
    code: ErrorCode,
    message: Option<String>,
}

impl Error {
    /// Creates a new error from a GError pointer.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `error` is either null or a valid pointer to a GError.
    /// If not null, this function takes ownership of the GError and will free it.
    pub(crate) unsafe fn from_gerror(error: *mut GError) -> Option<Self> {
        if error.is_null() {
            return None;
        }

        let gerror = &*error;
        let code = ErrorCode::from_raw(gerror.code as u32);
        let message = if gerror.message.is_null() {
            None
        } else {
            Some(
                CStr::from_ptr(gerror.message)
                    .to_string_lossy()
                    .into_owned(),
            )
        };

        // Free the GError
        glib_sys::g_error_free(error);

        Some(Self { code, message })
    }

    /// Creates a new error with a custom message.
    pub(crate) fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: Some(message.into()),
        }
    }

    /// Returns the error code.
    pub fn code(&self) -> ErrorCode {
        self.code
    }

    /// Returns the error message, if any.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.message {
            Some(msg) => write!(f, "{}: {}", self.code, msg),
            None => write!(f, "{}", self.code),
        }
    }
}

impl std::error::Error for Error {}

/// Result type for VDO operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Helper macro for checking GError and converting to Result.
macro_rules! check_gerror {
    ($error:expr) => {{
        let err = unsafe { $crate::error::Error::from_gerror($error) };
        if let Some(e) = err {
            return Err(e);
        }
    }};
}

pub(crate) use check_gerror;

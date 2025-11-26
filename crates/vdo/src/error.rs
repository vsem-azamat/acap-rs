//! Error types for VDO operations.

use std::ffi::CStr;
use std::fmt;

use glib_sys::GError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorCode {
    NotFound,
    Exists,
    InvalidArgument,
    PermissionDenied,
    NotSupported,
    Closed,
    Busy,
    Io,
    Hal,
    Dbus,
    Oom,
    Idle,
    NoData,
    NoBufferSpace,
    BufferFailure,
    InterfaceDown,
    Failed,
    Fatal,
    NotControlled,
    NoEvent,
    NoVideo,
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

#[derive(Debug)]
pub struct Error {
    code: ErrorCode,
    message: Option<String>,
}

impl Error {
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

        glib_sys::g_error_free(error);

        Some(Self { code, message })
    }

    pub(crate) fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: Some(message.into()),
        }
    }

    pub fn code(&self) -> ErrorCode {
        self.code
    }

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

pub type Result<T> = std::result::Result<T, Error>;

macro_rules! check_gerror {
    ($error:expr) => {{
        let err = unsafe { $crate::error::Error::from_gerror($error) };
        if let Some(e) = err {
            return Err(e);
        }
    }};
}

pub(crate) use check_gerror;

use std::ffi::CStr;
use std::fmt;

use daic_sys::daic;

#[derive(Debug, Clone)]
pub struct DaicError(pub(crate) String);

impl DaicError {
    pub(crate) fn new(msg: impl Into<String>) -> Self {
        Self(msg.into())
    }
}

impl fmt::Display for DaicError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for DaicError {}

pub type Result<T> = std::result::Result<T, DaicError>;

pub(crate) fn clear_error_flag() {
    daic::dai_clear_last_error();
}

pub(crate) fn last_error(context: &str) -> DaicError {
    match take_error_message() {
        Some(msg) if !msg.is_empty() => DaicError::new(msg),
        _ => DaicError::new(context),
    }
}

pub(crate) fn take_error_if_any(context: &str) -> Option<DaicError> {
    take_error_message().map(|msg| {
        if msg.is_empty() {
            DaicError::new(context)
        } else {
            DaicError::new(msg)
        }
    })
}

fn take_error_message() -> Option<String> {
    unsafe {
        let err_ptr = daic::dai_get_last_error();
        if err_ptr.is_null() {
            return None;
        }
        let msg = CStr::from_ptr(err_ptr).to_string_lossy().into_owned();
        daic::dai_clear_last_error();
        Some(msg)
    }
}

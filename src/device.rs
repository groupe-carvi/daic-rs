use autocxx::c_int;
use daic_sys::{daic, DaiDevice};
use std::os::raw::c_int as RawInt;

use crate::common::CameraBoardSocket;
use crate::error::{Result, clear_error_flag, last_error, take_error_if_any};

const MAX_SOCKETS: usize = 16;

pub struct Device {
    handle: DaiDevice,
}

impl Device {
    pub(crate) fn from_handle(handle: DaiDevice) -> Self {
        Self { handle }
    }

    pub fn new() -> Result<Self> {
        clear_error_flag();
        let handle = daic::dai_device_new();
        if handle.is_null() {
            Err(last_error("failed to create DepthAI device"))
        } else {
            Ok(Self { handle })
        }
    }

    /// Create another handle to the same underlying device connection.
    ///
    /// This mirrors DepthAI's C++ usage where the device is commonly shared via `std::shared_ptr`.
    pub fn try_clone(&self) -> Result<Self> {
        clear_error_flag();
        let handle = unsafe { daic::dai_device_clone(self.handle) };
        if handle.is_null() {
            Err(last_error("failed to clone DepthAI device"))
        } else {
            Ok(Self { handle })
        }
    }

    pub fn is_connected(&self) -> bool {
        unsafe { !daic::dai_device_is_closed(self.handle) }
    }

    /// Explicitly close the device connection.
    ///
    /// Note: other cloned `Device` handles to the same underlying connection will observe the
    /// closed state as well.
    pub fn close(&self) -> Result<()> {
        clear_error_flag();
        unsafe { daic::dai_device_close(self.handle) };
        if let Some(err) = take_error_if_any("failed to close DepthAI device") {
            Err(err)
        } else {
            Ok(())
        }
    }

    pub fn connected_cameras(&self) -> Result<Vec<CameraBoardSocket>> {
        clear_error_flag();
        let mut sockets = vec![c_int(0); MAX_SOCKETS];
        let count = unsafe {
            daic::dai_device_get_connected_camera_sockets(
                self.handle,
                sockets.as_mut_ptr(),
                c_int(MAX_SOCKETS as i32),
            )
        };
        let count_raw: RawInt = count.into();
        if count_raw <= 0 {
            if let Some(err) = take_error_if_any("failed to query connected cameras") {
                return Err(err);
            }
            return Ok(Vec::new());
        }
        sockets.truncate(count_raw as usize);
        Ok(sockets
            .into_iter()
            .map(|raw| CameraBoardSocket::from_raw(RawInt::from(raw)))
            .collect())
    }

    pub(crate) fn handle(&self) -> DaiDevice {
        self.handle
    }
}

impl Clone for Device {
    fn clone(&self) -> Self {
        // Clone is expected to be infallible. If cloning fails, we surface it as a panic,
        // since continuing with an invalid handle would be unsound.
        self.try_clone().expect("failed to clone DepthAI device")
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe { daic::dai_device_delete(self.handle) };
            self.handle = std::ptr::null_mut();
        }
    }
}

unsafe impl Send for Device {}
unsafe impl Sync for Device {}

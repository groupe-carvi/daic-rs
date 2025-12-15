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
    pub fn new() -> Result<Self> {
        clear_error_flag();
        let handle = daic::dai_device_new();
        if handle.is_null() {
            Err(last_error("failed to create DepthAI device"))
        } else {
            Ok(Self { handle })
        }
    }

    pub fn is_connected(&self) -> bool {
        unsafe { !daic::dai_device_is_closed(self.handle) }
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

impl Drop for Device {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe { daic::dai_device_delete(self.handle) };
        }
    }
}

unsafe impl Send for Device {}
unsafe impl Sync for Device {}

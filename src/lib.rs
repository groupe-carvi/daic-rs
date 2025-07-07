#[allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
pub mod bindings {
    include!("../generated/bindings.rs");
}

pub trait DeviceInterface {
    fn create() -> Self;
    fn destroy(self);
    fn get_name(&self) -> String;
    fn get_serial_number(&self) -> String;
    fn get_firmware_version(&self) -> String;
    fn get_hardware_version(&self) -> String;
}

pub enum ReconnectionStatus {
Reconnected, 
Reconnecting, 
ReconnectFailed
}






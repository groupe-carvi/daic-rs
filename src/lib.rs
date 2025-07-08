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






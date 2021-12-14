
pub struct Uuid(pub [u8; 16]);

pub struct Device {
    uuid: Uuid,
    state: DeviceState,
}

pub enum DeviceState {
    Unprovisioned,
    Node,
}

impl Default for DeviceState {
    fn default() -> Self {
        DeviceState::Unprovisioned
    }
}
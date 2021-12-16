#![no_std]

pub const PB_ADV: u8 = 0x29;
pub const MESH_MESSAGE: u8 = 0x2A;
pub const MESH_BEACON: u8 = 0x2B;

pub mod address;
pub mod pdu;
pub mod app;
pub mod control;
pub mod status;
pub mod device;
pub mod bearer;
pub mod beacon;
pub mod provisioning;
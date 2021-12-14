#![no_std]

pub const PB_ADV: u8 = 0x29;
pub const MESH_MESSAGE: u8 = 0x2A;
pub const MESH_BEACON: u8 = 0x2B;

mod address;
mod pdu;
mod app;
mod control;
mod status;
mod device;
mod bearer;
mod beacon;
use defmt::Format;
use heapless::Vec;
use crate::device::Uuid;

#[derive(Format)]
pub enum Gpcf {
    TransactionStart(u8),
    TransactionAck,
    TransactionContinuation(u8),
    ProvisioningBearerControl(BearerOpcode),
}

impl From<u8> for Gpcf {
    fn from(val: u8) -> Self {
        match (val & 0b11) {
            0b00 => Self::TransactionStart((val & 0b11111100) >> 2),
            0b01 => Self::TransactionAck,
            0b10 => Self::TransactionContinuation((val & 0b11111100) >> 2),
            0b11 => Self::ProvisioningBearerControl( ((val & 0b11111100) >> 2).into()),
            _ => defmt::panic!("unreachable"),
        }
    }
}

#[derive(Format)]
pub enum BearerOpcode {
    LinkOpen,
    LinkAck,
    LinkClose,
    RFU = 0x3F,
}

impl From<u8> for BearerOpcode {
    fn from(val: u8) -> Self {
        defmt::info!("opcode from {}", val);
        match val {
            0x00 => Self::LinkOpen,
            0x01 => Self::LinkAck,
            0x02 => Self::LinkClose,
            _ => Self::RFU
        }
    }
}

pub enum Reason {
    Success = 0x00,
    Timeout = 0x01,
    Fail = 0x02
}

pub struct TransactionStart {
    seg_n: u8 /* 6 bits */,
    total_len: u16,
    fcs: u8,
}

pub struct TransactionAck;

pub struct TransactionContinuation {
    segment_index: u8 /* 6 bits */,
}

pub struct LinkOpen {
    uuid: Uuid
}

pub struct LinkAck;

pub struct LinkClose {
    reason: Reason,
}
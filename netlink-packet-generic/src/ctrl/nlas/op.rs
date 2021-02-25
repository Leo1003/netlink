use crate::constants::*;
use byteorder::{ByteOrder, NativeEndian};
use netlink_packet_utils::nla::{DefaultNla, Nla};
use std::mem::size_of_val;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OpAttrs {
    Unspec(Vec<u8>),
    Id(u32),
    Flags(u32),
    Other(DefaultNla),
}

impl Nla for OpAttrs {
    fn value_len(&self) -> usize {
        use OpAttrs::*;
        match self {
            Unspec(bytes) => bytes.len(),
            Id(v) => size_of_val(v),
            Flags(v) => size_of_val(v),
            Other(nla) => nla.value_len(),
        }
    }

    fn kind(&self) -> u16 {
        use OpAttrs::*;
        match self {
            Unspec(_) => CTRL_ATTR_OP_UNSPEC,
            Id(_) => CTRL_ATTR_OP_ID,
            Flags(_) => CTRL_ATTR_OP_FLAGS,
            Other(nla) => nla.kind(),
        }
    }

    fn emit_value(&self, buffer: &mut [u8]) {
        use OpAttrs::*;
        match self {
            Unspec(bytes) => buffer.copy_from_slice(bytes),
            Id(v) => NativeEndian::write_u32(buffer, *v),
            Flags(v) => NativeEndian::write_u32(buffer, *v),
            Other(nla) => nla.emit_value(buffer),
        }
    }
}

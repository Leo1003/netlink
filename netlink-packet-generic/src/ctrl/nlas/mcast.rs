use crate::constants::*;
use byteorder::{ByteOrder, NativeEndian};
use netlink_packet_utils::nla::{DefaultNla, Nla};
use std::mem::size_of_val;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum McastGrpAttrs {
    Unspec(Vec<u8>),
    Name(String),
    Id(u32),
    Other(DefaultNla),
}

impl Nla for McastGrpAttrs {
    fn value_len(&self) -> usize {
        use McastGrpAttrs::*;
        match self {
            Unspec(bytes) => bytes.len(),
            Name(s) => s.as_bytes().len() + 1,
            Id(v) => size_of_val(v),
            Other(nla) => nla.value_len(),
        }
    }

    fn kind(&self) -> u16 {
        use McastGrpAttrs::*;
        match self {
            Unspec(_) => CTRL_ATTR_MCAST_GRP_UNSPEC,
            Name(_) => CTRL_ATTR_MCAST_GRP_NAME,
            Id(_) => CTRL_ATTR_MCAST_GRP_ID,
            Other(nla) => nla.kind(),
        }
    }

    fn emit_value(&self, buffer: &mut [u8]) {
        use McastGrpAttrs::*;
        match self {
            Unspec(bytes) => buffer.copy_from_slice(bytes),
            Name(s) => {
                buffer[..s.len()].copy_from_slice(s.as_bytes());
                buffer[s.len()] = 0;
            }
            Id(v) => NativeEndian::write_u32(buffer, *v),
            Other(nla) => nla.emit_value(buffer),
        }
    }
}

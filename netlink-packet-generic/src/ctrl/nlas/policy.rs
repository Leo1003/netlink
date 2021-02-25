use crate::constants::*;
use byteorder::{ByteOrder, NativeEndian};
use netlink_packet_utils::nla::{DefaultNla, Nla};
use std::mem::size_of_val;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PolicyAttrs {
    Unspec(Vec<u8>),
    Do(u32),
    Dump(u32),
    Other(DefaultNla),
}

impl Nla for PolicyAttrs {
    fn value_len(&self) -> usize {
        use PolicyAttrs::*;
        match self {
            Unspec(bytes) => bytes.len(),
            Do(v) => size_of_val(v),
            Dump(v) => size_of_val(v),
            Other(nla) => nla.value_len(),
        }
    }

    fn kind(&self) -> u16 {
        use PolicyAttrs::*;
        match self {
            Unspec(_) => CTRL_ATTR_POLICY_UNSPEC,
            Do(_) => CTRL_ATTR_POLICY_DO,
            Dump(_) => CTRL_ATTR_POLICY_DUMP,
            Other(nla) => nla.kind(),
        }
    }

    fn emit_value(&self, buffer: &mut [u8]) {
        use PolicyAttrs::*;
        match self {
            Unspec(bytes) => buffer.copy_from_slice(bytes),
            Do(v) => NativeEndian::write_u32(buffer, *v),
            Dump(v) => NativeEndian::write_u32(buffer, *v),
            Other(nla) => nla.emit_value(buffer),
        }
    }
}

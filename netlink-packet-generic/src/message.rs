use crate::traits::*;
use crate::header::GenlHeader;
use netlink_packet_core::{NetlinkDeserializable, NetlinkSerializable};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GenlMessage<F: GenlFamily> {
    header: GenlHeader,
    user_header: F::Header,
    payload: F::Payload,
}

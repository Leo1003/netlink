use netlink_packet_utils::nla::Nla;

mod mcast;
mod op;
mod policy;

pub use mcast::McastGrpAttrs;
pub use op::OpAttrs;
pub use policy::PolicyAttrs;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CtrlAttrs {
    Unspec,
    FamilyId(u16),
    FamilyName(String),
    Version(u32),
    HdrSize(u32),
    MaxAttr(u32),
    Ops(Vec<OpAttrs>),
    McastGroups(Vec<McastGrpAttrs>),
    Policy(Vec<u8>),
    OpPolicy(Vec<u8>),
    Op(u32),
}

impl Nla for CtrlAttrs {
    fn value_len(&self) -> usize {
        todo!()
    }

    fn kind(&self) -> u16 {
        todo!()
    }

    fn emit_value(&self, buffer: &mut [u8]) {
        todo!()
    }
}

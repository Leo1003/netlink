use crate::constants::*;
use crate::traits::*;

pub mod nlas;

#[derive(Copy, Clone, Debug)]
pub struct CtrlFamily;

impl GenlFamily for CtrlFamily {
    type Header = ();
    type Payload = CtrlCmdPayload;

    fn family_name(&self) -> &'static str {
        "nlctrl"
    }

    fn family_id(&self) -> u16 {
        GENL_ID_CTRL
    }

    fn version(&self) -> u8 {
        2
    }
}

#[derive(Debug)]
pub enum CtrlCmdPayload {
    Unspec,
    /// Notify from event
    NewFamily,
    /// Notify from event
    DelFamily,
    /// Request to get family info
    GetFamily,
    /// Currently unused
    NewOps,
    /// Currently unused
    DelOps,
    /// Currently unused
    GetOps,
    /// Notify from event
    NewMcastGrp,
    /// Notify from event
    DelMcastGrp,
    /// Currently unused
    GetMcastGrp,
    /// Request to get family policy
    GetPolicy,
}

impl GenericPayload for CtrlCmdPayload {
    fn command(&self) -> u8 {
        use CtrlCmdPayload::*;
        match self {
            Unspec => CTRL_CMD_UNSPEC,
            NewFamily => CTRL_CMD_NEWFAMILY,
            DelFamily => CTRL_CMD_DELFAMILY,
            GetFamily => CTRL_CMD_GETFAMILY,
            NewOps => CTRL_CMD_NEWOPS,
            DelOps => CTRL_CMD_DELOPS,
            GetOps => CTRL_CMD_GETOPS,
            NewMcastGrp => CTRL_CMD_NEWMCAST_GRP,
            DelMcastGrp => CTRL_CMD_DELMCAST_GRP,
            GetMcastGrp => CTRL_CMD_GETMCAST_GRP,
            GetPolicy => CTRL_CMD_GETPOLICY,
        }
    }
}

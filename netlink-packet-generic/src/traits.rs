pub trait GenlFamily {
    type Header;

    type CmdAttrs;

    fn family_name(&self) -> &'static str;

    fn family_id(&self) -> u16;

    fn version(&self) -> u8;
}

pub trait Commands {
    fn command(&self) -> u8;
}

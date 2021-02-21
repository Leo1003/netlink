
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OpAttrs {
    Unspec,
    Id(u32),
    Flags(u32),
}

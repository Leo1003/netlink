
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PolicyAttrs {
    Unspec,
    Do(u32),
    Dump(u32),
}

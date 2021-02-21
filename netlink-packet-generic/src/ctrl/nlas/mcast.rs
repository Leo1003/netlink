
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum McastGrpAttrs {
    Unspec,
    Name(String),
    Id(u32),
}

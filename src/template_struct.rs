#[derive(Debug, Copy, Clone)]
pub struct NameTemplate<'a> {
    pub name: &'a str,
    pub contents: &'a str,
    pub example: &'a str,
    pub info: NameInfo,
}

#[derive(Debug, Copy, Clone)]
pub struct NameInfo {
    pub len: u64,
    pub name_type: NameType,
}

#[derive(Debug, Copy, Clone)]
pub enum NameType {
    Macro,
    Var,
    FunctionVar,
    Function,
}

use core::fmt;
use std::fmt::{Display, Formatter};

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct VarId(String);

impl From<&String> for VarId {
    fn from(item: &String) -> Self {
        VarId(item.clone())
    }
}

impl From<String> for VarId {
    fn from(item: String) -> Self {
        VarId(item.clone())
    }
}

impl Display for VarId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}

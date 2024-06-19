use core::fmt;
use std::{
    fmt::{Display, Formatter},
    rc::Rc,
};

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct VarId(Rc<str>);

impl From<&String> for VarId {
    fn from(item: &String) -> Self {
        VarId(item.clone().into())
    }
}

impl From<String> for VarId {
    fn from(item: String) -> Self {
        VarId(item.clone().into())
    }
}

impl Display for VarId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}

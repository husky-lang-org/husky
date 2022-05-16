use entity_route::GenericArgument;

use super::*;

impl From<ListOpr> for Opr {
    fn from(list: ListOpr) -> Self {
        Self::List(list)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ListOpr {
    TupleInit,
    NewVec,
    NewDict,
    Call,
    Index,
    ModuloIndex,
    StructInit,
    MethodCall {
        ranged_ident: RangedCustomIdentifier,
        generic_arguments: Vec<GenericArgument>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListStartAttr {
    None,
    Attach,
    MethodAttach {
        ranged_ident: RangedCustomIdentifier,
        generic_arguments: Vec<GenericArgument>,
    },
}

impl ListStartAttr {
    pub fn attached(&self) -> bool {
        match self {
            ListStartAttr::None => false,
            ListStartAttr::Attach => true,
            ListStartAttr::MethodAttach { .. } => true,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ListEndAttr {
    None,
    Attach,
    Modulo,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Bracket {
    Par,
    Box,
    Angle,
    Curl,
}

impl Bracket {
    pub fn bra_code(&self) -> &'static str {
        match self {
            Bracket::Par => "(",
            Bracket::Box => "[",
            Bracket::Angle => "<",
            Bracket::Curl => "{",
        }
    }

    pub fn ket_code(&self) -> &'static str {
        match self {
            Bracket::Par => ")",
            Bracket::Box => "]",
            Bracket::Angle => ">",
            Bracket::Curl => "}",
        }
    }
}

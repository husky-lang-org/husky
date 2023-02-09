use super::*;

pub enum CDeclHir {
    Type(CTypeDeclHir),
    Form(CFormDeclHir),
}

pub enum CTypeDeclHir {
    Struct(CStructDeclHir),
    Enum(CEnumDeclHir),
    Union(CUnionDeclHir),
}

#[salsa::tracked(db = CHirDb, jar = CHirJar)]
pub struct CStructDeclHir {}

#[salsa::tracked(db = CHirDb, jar = CHirJar)]
pub struct CEnumDeclHir {}

#[salsa::tracked(db = CHirDb, jar = CHirJar)]
pub struct CUnionDeclHir {}

pub enum CFormDeclHir {
    Function(CFunctionDeclHir),
    Value(CValueDeclHir),
}

#[salsa::tracked(db = CHirDb, jar = CHirJar)]
pub struct CFunctionDeclHir {}

#[salsa::tracked(db = CHirDb, jar = CHirJar)]
pub struct CValueDeclHir {}

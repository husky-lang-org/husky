pub mod compterm;
pub mod function_ritchie;
pub mod r#static;
pub mod ty_alias;
pub mod ty_var;
pub mod val;

use self::{
    compterm::{MajorComptermSynDecl, MajorComptermSynNodeDecl},
    function_ritchie::*,
    r#static::MajorStaticSynDecl,
    ty_alias::*,
    ty_var::{TypeVarSynDecl, TypeVarSynNodeDecl},
    val::*,
};
use super::*;
use crate::decl::major_item::form::r#static::MajorStaticSynNodeDecl;
use husky_entity_kind::MajorFormKind;
use husky_entity_path::path::major_item::form::MajorFormPath;
use husky_entity_tree::node::major_item::form::FormSynNodePath;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum FormSynNodeDecl {
    FunctionRitchie(MajorFunctionRitchieSynNodeDecl),
    Val(MajorValSynNodeDecl),
    Compterm(MajorComptermSynNodeDecl),
    Static(MajorStaticSynNodeDecl),
    TypeAlias(TypeAliasSynNodeDecl),
    TypeVar(TypeVarSynNodeDecl),
}

impl FormSynNodeDecl {
    pub fn syn_expr_region(self, db: &::salsa::Db) -> SynExprRegion {
        match self {
            FormSynNodeDecl::FunctionRitchie(slf) => slf.syn_expr_region(db),
            FormSynNodeDecl::Val(slf) => slf.syn_expr_region(db),
            FormSynNodeDecl::Compterm(slf) => slf.syn_expr_region(db),
            FormSynNodeDecl::Static(slf) => slf.syn_expr_region(db),
            FormSynNodeDecl::TypeAlias(slf) => slf.syn_expr_region(db),
            FormSynNodeDecl::TypeVar(slf) => slf.syn_expr_region(db),
        }
    }

    pub fn errors(self, db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        match self {
            FormSynNodeDecl::FunctionRitchie(slf) => slf.errors(db),
            FormSynNodeDecl::Val(slf) => slf.errors(db),
            FormSynNodeDecl::TypeAlias(slf) => slf.errors(db),
            FormSynNodeDecl::TypeVar(slf) => slf.errors(db),
            FormSynNodeDecl::Compterm(slf) => slf.errors(db),
            FormSynNodeDecl::Static(slf) => slf.errors(db),
        }
    }
}

impl HasSynNodeDecl for FormSynNodePath {
    type NodeDecl = FormSynNodeDecl;

    fn syn_node_decl<'a>(self, db: &'a ::salsa::Db) -> Self::NodeDecl {
        form_syn_node_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn form_syn_node_decl(
    db: &::salsa::Db,
    syn_node_path: FormSynNodePath,
) -> FormSynNodeDecl {
    ItemSynNodeDeclParser::new(db, syn_node_path.into()).parse_form_syn_node_decl(syn_node_path)
}

impl<'a> ItemSynNodeDeclParser<'a> {
    fn parse_form_syn_node_decl(&self, syn_node_path: FormSynNodePath) -> FormSynNodeDecl {
        match syn_node_path.form_kind(self.db()) {
            MajorFormKind::Val => self.parse_val_syn_node_decl(syn_node_path).into(),
            MajorFormKind::Ritchie(ritchie_item_kind) => self
                .parse_ritchie_syn_node_decl(syn_node_path, ritchie_item_kind)
                .into(),
            MajorFormKind::TypeAlias => self.parse_ty_alias_syn_node_decl(syn_node_path).into(),
            MajorFormKind::TypeVar => self.parse_ty_var_syn_node_decl(syn_node_path).into(),
            MajorFormKind::Conceptual => todo!(),
            // self.parse_conceptual_syn_node_decl(syn_node_path).into(),
            MajorFormKind::Static => self.parse_static_syn_node_decl(syn_node_path).into(),
            MajorFormKind::Compterm => self.parse_termic_syn_node_decl(syn_node_path).into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum FormSynDecl {
    Ritchie(MajorFunctionRitchieSynDecl),
    Val(MajorValSynDecl),
    Compterm(MajorComptermSynDecl),
    Static(MajorStaticSynDecl),
    TypeAlias(TypeAliasSynDecl),
    TypeVar(TypeVarSynDecl),
}

impl FormSynDecl {
    fn from_node(
        db: &::salsa::Db,
        path: MajorFormPath,
        syn_node_decl: FormSynNodeDecl,
    ) -> SynDeclResult<Self> {
        Ok(match syn_node_decl {
            FormSynNodeDecl::FunctionRitchie(syn_node_decl) => {
                MajorFunctionRitchieSynDecl::from_node(db, path, syn_node_decl)?.into()
            }
            FormSynNodeDecl::Val(syn_node_decl) => {
                MajorValSynDecl::from_node(db, path, syn_node_decl)?.into()
            }
            FormSynNodeDecl::TypeAlias(syn_node_decl) => {
                TypeAliasSynDecl::from_node(db, path, syn_node_decl)?.into()
            }
            FormSynNodeDecl::TypeVar(syn_node_decl) => {
                TypeVarSynDecl::from_node(db, path, syn_node_decl)?.into()
            }
            FormSynNodeDecl::Compterm(syn_node_decl) => {
                MajorComptermSynDecl::from_node(db, path, syn_node_decl)?.into()
            }
            FormSynNodeDecl::Static(syn_node_decl) => {
                MajorStaticSynDecl::from_node(db, path, syn_node_decl)?.into()
            }
        })
    }

    pub fn template_parameters<'a>(self, db: &'a ::salsa::Db) -> &'a [TemplateSynParameterData] {
        match self {
            FormSynDecl::Ritchie(decl) => decl.template_parameters(db),
            FormSynDecl::Val(_decl) => &[],
            FormSynDecl::TypeAlias(_) => todo!(),
            FormSynDecl::TypeVar(_) => &[],
            FormSynDecl::Compterm(_) => todo!(),
            FormSynDecl::Static(_) => todo!(),
        }
    }

    pub fn syn_expr_region(self, db: &::salsa::Db) -> SynExprRegion {
        match self {
            FormSynDecl::Ritchie(slf) => slf.syn_expr_region(db),
            FormSynDecl::Val(slf) => slf.syn_expr_region(db),
            FormSynDecl::TypeAlias(slf) => slf.syn_expr_region(db),
            FormSynDecl::TypeVar(slf) => slf.syn_expr_region(db),
            FormSynDecl::Compterm(slf) => slf.syn_expr_region(db),
            FormSynDecl::Static(slf) => slf.syn_expr_region(db),
        }
    }

    pub fn path(self, db: &::salsa::Db) -> MajorFormPath {
        match self {
            FormSynDecl::Ritchie(slf) => slf.path(db),
            FormSynDecl::Val(slf) => slf.path(db),
            FormSynDecl::TypeAlias(slf) => slf.path(db),
            FormSynDecl::TypeVar(slf) => slf.path(db),
            FormSynDecl::Compterm(slf) => slf.path(db),
            FormSynDecl::Static(slf) => slf.path(db),
        }
    }
}

impl HasSynDecl for MajorFormPath {
    type Decl = FormSynDecl;

    fn syn_decl(self, db: &::salsa::Db) -> SynDeclResult<Self::Decl> {
        form_syn_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn form_syn_decl(db: &::salsa::Db, path: MajorFormPath) -> SynDeclResult<FormSynDecl> {
    let syn_node_decl = path.syn_node_path(db).syn_node_decl(db);
    FormSynDecl::from_node(db, path, syn_node_decl)
}

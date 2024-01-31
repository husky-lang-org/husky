mod fugitive;
mod ty_instance_constructor;
pub mod ty_variant;
mod utils;

pub use self::fugitive::*;

pub use self::ty_instance_constructor::*;

use self::utils::*;
use crate::*;

#[inline(always)]
pub fn declarative_term_item_path_declarative_ty(
    _db: &::salsa::Db,
    path: EntityPathDeclarativeTerm,
) -> DeclarativeTypeResult<DeclarativeTerm> {
    match path {
        EntityPathDeclarativeTerm::Fugitive(_) => todo!(),
        EntityPathDeclarativeTerm::Trait(_) => todo!(),
        EntityPathDeclarativeTerm::Type(_) => todo!(),
        EntityPathDeclarativeTerm::TypeVariant(_) => todo!(),
    }
}

#[salsa::tracked(jar = DeclarativeTypeJar)]
pub fn ty_ontology_path_declarative_ty(
    db: &::salsa::Db,
    path: TypePath,
) -> DeclarativeTypeResult<DeclarativeTerm> {
    let declarative_term_menu = db.declarative_term_menu(path.toolchain(db)).unwrap();
    let signature = match path.declarative_signature_template(db) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedDeclarativeTypeError::SignatureError.into()),
    };
    let Ok(variances) = ty_path_variances(db, path) else {
        todo!()
    };
    curry_from_template_parameters(
        db,
        path.toolchain(db),
        CurryKind::Explicit,
        variances,
        signature.template_parameters(db),
        declarative_term_menu.ty0(),
    )
}

#[test]
fn ty_ontology_path_declarative_ty_works() {
    DB::ast_expect_test_debug_with_db(
        |db, module_path: husky_vfs::ModulePath| {
            husky_entity_tree::helpers::paths::module_item_paths(db, module_path)
                .iter()
                .filter_map(|&module_item_path| match module_item_path {
                    ItemPath::MajorItem(MajorItemPath::Type(ty_path)) => {
                        Some((ty_path, ty_ontology_path_declarative_ty(db, ty_path)))
                    }
                    _ => None,
                })
                .collect::<Vec<_>>()
        },
        &AstTestConfig::new(
            "ty_ontology_path_declarative_ty",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::KERNEL,
        ),
    );
}

#[salsa::tracked(jar = DeclarativeTypeJar)]
pub fn trai_path_declarative_ty(
    db: &::salsa::Db,
    path: TraitPath,
) -> DeclarativeTypeResult<DeclarativeTerm> {
    let declarative_term_menu = db.declarative_term_menu(path.toolchain(db)).unwrap();
    let Ok(variances) = trai_item_variances(db, path) else {
        todo!()
    };
    let signature = match path.declarative_signature_template(db) {
        Ok(signature) => signature,
        Err(_) => todo!(),
    };
    curry_from_template_parameters(
        db,
        path.toolchain(db),
        CurryKind::Explicit,
        variances,
        signature.template_parameters_without_self_ty(db),
        declarative_term_menu.trai_ty(),
    )
}

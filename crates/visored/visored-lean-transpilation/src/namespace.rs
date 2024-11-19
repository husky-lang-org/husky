use crate::*;
use lean_item_path::namespace::LnNamespace;
use visored_item_path::module::{VdModulePath, VdModulePathData};

#[salsa::tracked]
pub fn vd_module_path_to_ln_namespace(
    db: &::salsa::Db,
    module_path: VdModulePath,
) -> Option<LnNamespace> {
    match module_path.data(db) {
        VdModulePathData::Root(_) => Some(LnNamespace::new_root(db)),
        VdModulePathData::Division {
            parent,
            division_kind,
            disambiguator,
        } => {
            let parent_namespace = vd_module_path_to_ln_namespace(db, parent)?;
            Some(parent_namespace.child(
                format!(
                    "{}{}",
                    division_kind.uppercase_code_name(),
                    disambiguator + 1
                ),
                db,
            ))
        }
        VdModulePathData::Paragraph {
            parent,
            disambiguator,
        } => None,
        VdModulePathData::Environment {
            parent,
            environment_path,
            disambiguator,
        } => {
            let parent_namespace = vd_module_path_to_ln_namespace(db, parent)?;
            Some(parent_namespace.child(
                format!(
                    "{}{}",
                    environment_path.name().coword().data(db),
                    disambiguator
                ),
                db,
            ))
        }
    }
}

#[salsa::tracked]
pub fn vd_module_path_to_ln_namespace_or_inherited(
    db: &::salsa::Db,
    module_path: VdModulePath,
) -> LnNamespace {
    match vd_module_path_to_ln_namespace(db, module_path) {
        Some(namespace) => namespace,
        None => vd_module_path_to_ln_namespace_or_inherited(db, module_path.parent(db).unwrap()),
    }
}

use super::*;

#[salsa::tracked(jar = DeclarativeTypeJar)]
pub fn form_path_declarative_ty(
    db: &dyn DeclarativeTypeDb,
    path: FugitivePath,
) -> DeclarativeTypeResult<DeclarativeTerm> {
    let signature = match path.declarative_signature_template(db) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedDeclarativeTypeError::SignatureError.into()),
    };
    let Ok(variances) = form_entity_variances(db, path) else {
        todo!()
    };
    let declarative_term_menu = db.declarative_term_menu(path.toolchain(db)).unwrap();
    match signature {
        FugitiveDeclarativeSignatureTemplate::Fn(signature) => {
            fn_path_declarative_ty(db, variances, signature)
        }
        FugitiveDeclarativeSignatureTemplate::Gn(signature) => {
            gn_path_declarative_ty(db, variances, signature)
        }
        FugitiveDeclarativeSignatureTemplate::Val(signature) => {
            val_path_declarative_ty(db, signature, declarative_term_menu)
        }
        FugitiveDeclarativeSignatureTemplate::TypeAlias(_) => todo!(),
    }
}

pub(crate) fn fn_path_declarative_ty(
    db: &dyn DeclarativeTypeDb,
    variances: &[Variance],
    signature: FnDeclarativeSignatureTemplate,
) -> DeclarativeTypeResult<DeclarativeTerm> {
    let param_declarative_tys = signature
        .parameters(db)
        .iter()
        .copied()
        .map(ExplicitParameterDeclarativeSignatureTemplate::into_ritchie_parameter_contracted_ty)
        .collect();
    let return_declarative_ty = signature.return_ty(db);
    Ok(curry_from_implicit_parameters(
        db,
        CurryKind::Implicit,
        variances,
        signature.implicit_parameters(db),
        DeclarativeTermRitchie::new(
            db,
            RitchieKind::FnType,
            param_declarative_tys,
            return_declarative_ty,
        ),
    ))
}

pub(crate) fn gn_path_declarative_ty(
    db: &dyn DeclarativeTypeDb,
    variances: &[Variance],
    signature: GnDeclarativeSignatureTemplate,
) -> DeclarativeTypeResult<DeclarativeTerm> {
    let param_declarative_tys = signature
        .parameters(db)
        .iter()
        .copied()
        .map(ExplicitParameterDeclarativeSignatureTemplate::into_ritchie_parameter_contracted_ty)
        .collect();
    let return_declarative_ty = signature.return_ty(db);
    Ok(curry_from_implicit_parameters(
        db,
        CurryKind::Implicit,
        variances,
        signature.implicit_parameters(db),
        DeclarativeTermRitchie::new(
            db,
            RitchieKind::FnType,
            param_declarative_tys,
            return_declarative_ty,
        ),
    ))
}

pub(crate) fn val_path_declarative_ty(
    db: &dyn DeclarativeTypeDb,
    signature: ValDeclarativeSignatureTemplate,
    _declarative_term_menu: &DeclarativeTermMenu,
) -> DeclarativeTypeResult<DeclarativeTerm> {
    Ok(signature.initialization_ty(db).leashed_ty(db))
}

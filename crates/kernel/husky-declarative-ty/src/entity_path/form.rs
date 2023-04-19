use super::*;

#[salsa::tracked(jar = DeclarativeTypeJar)]
pub fn form_path_raw_ty(
    db: &dyn DeclarativeTypeDb,
    path: FormPath,
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
        FormDeclarativeSignatureTemplate::Fn(signature) => {
            form_fn_entity_raw_ty(db, variances, signature)
        }
        FormDeclarativeSignatureTemplate::Val(signature) => {
            feature_entity_raw_ty(db, signature, declarative_term_menu)
        }
        FormDeclarativeSignatureTemplate::Gn(_) => todo!(),
    }
}

pub(crate) fn form_fn_entity_raw_ty(
    db: &dyn DeclarativeTypeDb,
    variances: &[Variance],
    signature: FnDeclarativeSignatureTemplate,
) -> DeclarativeTypeResult<DeclarativeTerm> {
    let param_raw_tys = signature
        .parameters(db)
        .iter()
        .copied()
        .map(ExplicitParameterSignature::into_ritchie_parameter_contracted_ty)
        .collect();
    let return_raw_ty = signature.return_ty(db);
    Ok(curry_from_implicit_parameters(
        db,
        CurryKind::Implicit,
        variances,
        signature.implicit_parameters(db),
        DeclarativeTermRitchie::new(db, TermRitchieKind::FnType, param_raw_tys, return_raw_ty),
    ))
}

pub(crate) fn feature_entity_raw_ty(
    db: &dyn DeclarativeTypeDb,
    signature: ValDeclarativeSignatureTemplate,
    _declarative_term_menu: &DeclarativeTermMenu,
) -> DeclarativeTypeResult<DeclarativeTerm> {
    Ok(signature.return_ty(db))
}

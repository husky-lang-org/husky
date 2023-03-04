use crate::*;

pub trait TypeDb: salsa::DbWithJar<TypeJar> + TermDb {
    fn ty_method_ty(&self, ty: ReducedTerm, ident: Identifier) -> TypeResult<Option<ReducedTerm>>;
    fn field_ty(&self, ty: ReducedTerm, ident: Identifier) -> TypeResult<Option<ReducedTerm>>;
    fn reduced_term(&self, term: Term) -> ReducedTerm;
    fn intrinsic_ty(&self, ty: ReducedTerm) -> IntrinsicType;
    fn reduced_term_menu<'a>(
        &'a self,
        toolchain: Toolchain,
    ) -> Result<ReducedTermMenu<'a>, &'a TermError>;
    fn term_application_expansion(&self, reduced_term: ReducedTerm) -> ApplicationExpansion;
    fn ty_call_ty(
        &self,
        term: ReducedTerm,
        toolchain: Toolchain,
        reduced_term_menu: ReducedTermMenu,
    ) -> TypeResult<ReducedTerm>;
    fn term_contains_symbol(&self, term: Term, symbol: TermSymbol) -> bool;
    fn ty_path_ty(
        &self,
        path: TypePath,
        disambiguation: TypePathDisambiguation,
    ) -> TypeResult<ReducedTerm>;
    fn trai_path_ty(&self, trai_path: TraitPath) -> TypeResult<ReducedTerm>;
    fn form_path_ty(&self, form_path: FormPath) -> TypeResult<ReducedTerm>;
}

impl<Db> TypeDb for Db
where
    Db: salsa::DbWithJar<TypeJar> + TermDb,
{
    fn ty_method_ty(&self, ty: ReducedTerm, ident: Identifier) -> TypeResult<Option<ReducedTerm>> {
        ty_method_ty(self, ty, ident)
    }

    fn reduced_term(&self, term: Term) -> ReducedTerm {
        calc_reduced_term(self, term)
    }

    fn reduced_term_menu<'a>(
        &'a self,
        toolchain: Toolchain,
    ) -> Result<ReducedTermMenu<'a>, &'a TermError> {
        let term_menu = self.term_menu(toolchain).as_ref()?;
        Ok(ReducedTermMenu::new(term_menu))
    }

    fn field_ty(&self, ty: ReducedTerm, ident: Identifier) -> TypeResult<Option<ReducedTerm>> {
        field_ty(self, ty, ident)
    }

    fn intrinsic_ty(&self, ty: ReducedTerm) -> IntrinsicType {
        intrinsic_ty(self, ty)
    }

    fn term_application_expansion(&self, reduced_term: ReducedTerm) -> ApplicationExpansion {
        application_expansion(self, reduced_term)
    }

    fn ty_call_ty(
        &self,
        ty_term: ReducedTerm,
        toolchain: Toolchain,
        reduced_term_menu: ReducedTermMenu,
    ) -> TypeResult<ReducedTerm> {
        ty_call_ty(self, ty_term, toolchain, reduced_term_menu)
    }

    fn term_contains_symbol(&self, term: Term, symbol: TermSymbol) -> bool {
        calc_term_symbols(self, term)
            .map(|term_symbols| term_symbols.contains(self, symbol))
            .unwrap_or_default()
    }
    fn ty_path_ty(
        &self,
        path: TypePath,
        disambiguation: TypePathDisambiguation,
    ) -> TypeResult<ReducedTerm> {
        ty_path_ty(self, path, disambiguation)
    }

    fn trai_path_ty(&self, trai_path: TraitPath) -> TypeResult<ReducedTerm> {
        trai_path_ty(self, trai_path)
    }

    fn form_path_ty(&self, form_path: FormPath) -> TypeResult<ReducedTerm> {
        form_path_ty(self, form_path)
    }
}

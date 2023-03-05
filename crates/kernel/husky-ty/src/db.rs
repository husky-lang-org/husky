use crate::*;
use husky_raw_term::RawTerm;

pub trait TypeDb: salsa::DbWithJar<TypeJar> + TermDb {
    fn ty_method_ty(&self, ty: Term, ident: Identifier) -> TypeResult<Option<Term>>;
    fn field_ty(&self, ty: Term, ident: Identifier) -> TypeResult<Option<Term>>;
    fn reduced_term(&self, term: RawTerm) -> Term;
    fn intrinsic_ty(&self, ty: Term) -> IntrinsicType;
    fn term_application_expansion(&self, reduced_term: Term) -> ApplicationExpansion;
    fn ty_call_ty(
        &self,
        term: Term,
        toolchain: Toolchain,
        reduced_term_menu: TermMenu,
    ) -> TypeResult<Term>;
    fn term_contains_symbol(&self, term: Term, symbol: TermSymbol) -> bool;
    fn ty_path_ty(
        &self,
        path: TypePath,
        disambiguation: TypePathDisambiguation,
    ) -> TypeResult<Term>;
    fn trai_path_ty(&self, trai_path: TraitPath) -> TypeResult<Term>;
    fn form_path_ty(&self, form_path: FormPath) -> TypeResult<Term>;
}

impl<Db> TypeDb for Db
where
    Db: salsa::DbWithJar<TypeJar> + TermDb,
{
    fn ty_method_ty(&self, ty: Term, ident: Identifier) -> TypeResult<Option<Term>> {
        ty_method_ty(self, ty, ident)
    }

    fn reduced_term(&self, term: RawTerm) -> Term {
        todo!()
        // calc_reduced_term(self, term)
    }

    fn field_ty(&self, ty: Term, ident: Identifier) -> TypeResult<Option<Term>> {
        field_ty(self, ty, ident)
    }

    fn intrinsic_ty(&self, ty: Term) -> IntrinsicType {
        intrinsic_ty(self, ty)
    }

    fn term_application_expansion(&self, reduced_term: Term) -> ApplicationExpansion {
        application_expansion(self, reduced_term)
    }

    fn ty_call_ty(
        &self,
        ty_term: Term,
        toolchain: Toolchain,
        reduced_term_menu: TermMenu,
    ) -> TypeResult<Term> {
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
    ) -> TypeResult<Term> {
        ty_path_ty(self, path, disambiguation)
    }

    fn trai_path_ty(&self, trai_path: TraitPath) -> TypeResult<Term> {
        trai_path_ty(self, trai_path)
    }

    fn form_path_ty(&self, form_path: FormPath) -> TypeResult<Term> {
        form_path_ty(self, form_path)
    }
}

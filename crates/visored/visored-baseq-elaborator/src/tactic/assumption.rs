use super::*;

impl<'db, 'sess> VdBsqElaboratorInner<'db, 'sess> {
    pub(crate) fn assumption(&mut self, prop: VdBsqExprFld<'sess>) -> Mhr<'sess> {
        AltJustOk(Ok(self.hypothesis_constructor.assumption(prop)?))
    }
}

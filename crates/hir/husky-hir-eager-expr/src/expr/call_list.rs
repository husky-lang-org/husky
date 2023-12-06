use super::*;
use crate::coersion::HirEagerCoersion;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum HirEagerCallListItemGroup {
    Regular(HirEagerExprIdx, HirEagerCoersion),
    Variadic,
    Keyed,
}

impl<'a> HirEagerExprBuilder<'a> {
    pub(super) fn new_call_list_item_groups(
        &mut self,
        pams: &[SemaRitchieParameterArgumentMatch],
    ) -> SmallVec<[HirEagerCallListItemGroup; 4]> {
        pams.iter()
            .map(|pam| self.new_call_list_item_group(pam))
            .collect()
    }

    fn new_call_list_item_group(
        &mut self,
        pam: &SemaRitchieParameterArgumentMatch,
    ) -> HirEagerCallListItemGroup {
        match pam {
            SemaRitchieParameterArgumentMatch::Regular(_, item) => {
                HirEagerCallListItemGroup::Regular(
                    item.argument_sema_expr_idx().to_hir_eager(self),
                    item.coersion.unwrap().to_hir_eager(self),
                )
            }
            SemaRitchieParameterArgumentMatch::Variadic(_, _) => todo!(),
            SemaRitchieParameterArgumentMatch::Keyed(_, _) => todo!(),
        }
    }
}

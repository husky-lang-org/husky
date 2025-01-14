use super::{stack::VdBsqHypothesisStack, *};
use crate::{session::VdBsqSession, term::VdBsqTerm};
use floated_sequential::db::FloaterDb;

pub struct VdBsqHypothesisConstructor<'db, 'sess> {
    session: &'sess VdBsqSession<'db>,
    stack: VdBsqHypothesisStack<'sess>,
    arena: VdBsqHypothesisArena<'sess>,
}

impl<'db, 'sess> std::fmt::Debug for VdBsqHypothesisConstructor<'db, 'sess> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let active_hypotheses = self
            .stack
            .active_hypotheses()
            .data()
            .iter()
            .map(|h| &self.arena[h])
            .collect::<Vec<_>>();
        f.debug_struct("VdBsqHypothesisConstructor")
            .field("active_hypotheses", &active_hypotheses)
            .finish()
    }
}

impl<'db, 'sess> VdBsqHypothesisConstructor<'db, 'sess> {
    pub(crate) fn new(session: &'sess VdBsqSession<'db>) -> Self {
        Self {
            session,
            stack: VdBsqHypothesisStack::new(),
            arena: VdBsqHypothesisArena::default(),
        }
    }
}

// # getters
impl<'db, 'sess> VdBsqHypothesisConstructor<'db, 'sess> {
    pub fn floater_db(&self) -> &'sess FloaterDb {
        self.session.floater_db()
    }

    pub fn stack(&self) -> &VdBsqHypothesisStack<'sess> {
        &self.stack
    }

    pub fn arena(&self) -> &VdBsqHypothesisArena<'sess> {
        &self.arena
    }
}

impl<'db, 'sess> VdBsqHypothesisConstructor<'db, 'sess> {
    /// Attempts to find an existing hypothesis that matches the given expression.
    ///
    /// This method implements functionality similar to the `assumption` tactic in proof
    /// assistants like Lean and Coq. It searches for a matching hypothesis in the current
    /// context that could prove the given expression.
    ///
    /// If an existing hypothesis is found with the same expression, return it directly.
    ///
    /// Otherwise, if an existing hypothesis is found with the same term, return a new hypothesis derived from it.
    pub(crate) fn assumption(
        &mut self,
        expr: VdBsqExprFld<'sess>,
    ) -> Option<VdBsqHypothesisIdx<'sess>> {
        if let Some(hypothesis) = self.stack.get_active_hypothesis_with_expr(expr) {
            Some(hypothesis)
        } else if let Some(hypothesis) = self.stack.get_active_hypothesis_with_term(expr.term()) {
            let hypothesis = self.construct_new_hypothesis(
                expr,
                VdBsqHypothesisConstruction::TermEquivalent { hypothesis },
            );
            Some(hypothesis)
        } else {
            None
        }
    }

    pub(crate) fn construct_new_hypothesis(
        &mut self,
        prop: VdBsqExprFld<'sess>,
        construction: VdBsqHypothesisConstruction<'sess>,
    ) -> VdBsqHypothesisIdx<'sess> {
        let hypothesis_idx = self
            .arena
            .alloc_one(VdBsqHypothesisEntry { prop, construction });
        self.stack.push(
            hypothesis_idx,
            &self.arena[hypothesis_idx],
            self.floater_db(),
        );
        hypothesis_idx
    }
}

impl<'db, 'sess> VdBsqHypothesisConstructor<'db, 'sess> {
    pub fn enter_block(&mut self) {
        self.stack.enter_block();
    }

    pub fn exit_block(&mut self) {
        self.stack.exit_block();
    }
}

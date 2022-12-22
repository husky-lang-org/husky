use super::*;

pub(crate) enum PresheetAction {
    ResolveEntityUse {
        module_path: ModulePath,
        entity_use_tracker_idx: EntityUseExprTrackerIdx,
        node: EntityNode,
    },
    EvaluateUseAll {
        module_path: ModulePath,
        use_all_tracker_idx: UseAllTrackerIdx,
    },
}

impl PresheetAction {
    pub(crate) fn module_path(&self) -> ModulePath {
        match self {
            PresheetAction::ResolveEntityUse { module_path, .. }
            | PresheetAction::EvaluateUseAll { module_path, .. } => *module_path,
        }
    }
}

impl EntityTreePresheet {
    pub(crate) fn collect_possible_actions(
        &self,
        ctx: EntitySymbolContext,
        actions: &mut Vec<PresheetAction>,
    ) {
        for (entity_use_tracker_idx, entity_use_tracker) in self.entity_use_trackers.indexed_iter()
        {
            if !entity_use_tracker.resolved() {
                let ident = entity_use_tracker.ident();
                if let Some(node) = ctx.get(ident) {
                    actions.push(PresheetAction::ResolveEntityUse {
                        module_path: self.module_path,
                        entity_use_tracker_idx,
                        node: node.clone(),
                    })
                }
            }
        }
        for (use_all_tracker_idx, use_all_tracker) in self.use_all_trackers.indexed_iter() {
            todo!()
        }
    }

    pub(crate) fn exec(&mut self, db: &dyn EntityTreeDb, action: PresheetAction) {
        match action {
            PresheetAction::ResolveEntityUse {
                entity_use_tracker_idx,
                node,
                ..
            } => self.resolve_entity_use(db, entity_use_tracker_idx, node),
            PresheetAction::EvaluateUseAll {
                module_path,
                use_all_tracker_idx,
            } => todo!(),
        }
    }
    fn resolve_entity_use(
        &mut self,
        db: &dyn EntityTreeDb,
        entity_use_tracker_idx: EntityUseExprTrackerIdx,
        node: EntityNode,
    ) {
        let entity_use_tracker = &mut self.entity_use_trackers[entity_use_tracker_idx];
        assert!(!entity_use_tracker.resolved());
        match node {
            EntityNode::Module {
                ident,
                accessibility,
                module_path,
            } => match self.can_access(db, accessibility) {
                true => todo!(),
                false => todo!(),
            },
            EntityNode::ModuleItem {
                ident,
                accessibility,
                ast_idx,
                path,
            } => todo!(),
            EntityNode::EntityUse {
                ident,
                accessibility,
                path,
            } => todo!(),
        }
        entity_use_tracker.mark_as_resolved()
    }
}

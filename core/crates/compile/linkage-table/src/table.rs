use smallvec::SmallVec;
use vm::Linkage;

use crate::*;

#[derive(Debug, Default, Clone)]
pub struct LinkageSourceTable {
    linkage_sources: ARwLock<HashMap<LinkageKey, Linkage>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum LinkageKey {
    VecConstructor {
        element_ty_uid: EntityUid,
    },
    TypeCall {
        ty_uid: EntityUid,
    },
    Routine {
        routine_uid: EntityUid,
    },
    ElementAccess {
        opd_uids: SmallVec<[EntityUid; 2]>,
    },
    StructFieldAccess {
        this_ty_uid: EntityUid,
        field_ident: CustomIdentifier,
    },
}

impl LinkageSourceTable {
    pub(crate) fn type_call_linkage(&self, ty_uid: EntityUid) -> Option<Linkage> {
        self.get_linkage(LinkageKey::TypeCall { ty_uid })
    }

    pub(crate) fn routine_linkage(&self, routine_uid: EntityUid) -> Option<Linkage> {
        self.get_linkage(LinkageKey::Routine { routine_uid })
    }

    pub(crate) fn struct_field_access_linkage_source(
        &self,
        this_ty_uid: EntityUid,
        field_ident: CustomIdentifier,
        field_binding: Binding,
    ) -> Option<Linkage> {
        self.get_linkage(LinkageKey::StructFieldAccess {
            this_ty_uid,
            field_ident,
        })
    }

    pub(crate) fn element_access(&self, opd_uids: SmallVec<[EntityUid; 2]>) -> Option<Linkage> {
        self.get_linkage(LinkageKey::ElementAccess { opd_uids })
    }

    fn get_linkage(&self, key: LinkageKey) -> Option<Linkage> {
        self.linkage_sources
            .read(|entries| entries.get(&key).map(|linkage_source| *linkage_source))
    }
}

mod alias;
mod builtin;
mod error;
mod intern;
mod kind;
mod module;
mod query;
mod subscope;

pub use alias::ScopeAliasTable;
pub use builtin::BuiltinIdentifier3;
pub use error::{def::ScopeDefError, ScopeError, ScopeResult, ScopeResultArc};
use file::FileId;
pub use intern::{new_scope_interner, InternScope, ScopeId, ScopeInterner};
pub use kind::ScopeKind;
pub use module::Module;
pub use query::{ModuleFromFileError, ScopeQuery, ScopeQueryStorage, ScopeSalsaQuery};
pub use subscope::SubscopeTable;

use word::{BuiltinIdentifier, CustomIdentifier, Identifier};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Scope {
    pub route: ScopeRoute,
    pub lifetimes: Vec<LifetimeParameter>,
    pub generics: Vec<GenericArgument>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LifetimeParameter {
    Elided,
    Explicit(CustomIdentifier),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GenericArgument {
    Const(usize),
    Scope(ScopeId),
}

impl From<usize> for GenericArgument {
    fn from(size: usize) -> Self {
        Self::Const(size)
    }
}

impl From<ScopeId> for GenericArgument {
    fn from(scope: ScopeId) -> Self {
        GenericArgument::Scope(scope)
    }
}

impl From<BuiltinIdentifier> for ScopeRoute {
    fn from(ident: BuiltinIdentifier) -> Self {
        Self::Builtin(ident)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScopeRoute {
    Builtin(BuiltinIdentifier),
    Package(FileId, CustomIdentifier),
    ChildScope(ScopeId, CustomIdentifier),
}

impl Scope {
    pub fn package(main_file: FileId, ident: CustomIdentifier) -> Self {
        Scope {
            route: ScopeRoute::Package(main_file, ident),
            lifetimes: Vec::new(),
            generics: Vec::new(),
        }
    }
    pub fn child_scope(
        parent_scope: ScopeId,
        ident: CustomIdentifier,
        lifetimes: Vec<LifetimeParameter>,
        generics: Vec<GenericArgument>,
    ) -> Scope {
        Scope {
            route: ScopeRoute::ChildScope(parent_scope, ident),
            lifetimes,
            generics,
        }
    }

    pub fn builtin(
        scope: BuiltinIdentifier,
        lifetimes: Vec<LifetimeParameter>,
        generic_arguments: Vec<GenericArgument>,
    ) -> Scope {
        Scope {
            route: ScopeRoute::Builtin(scope),
            lifetimes,
            generics: generic_arguments,
        }
    }

    pub fn vec(element: GenericArgument) -> Self {
        Self::builtin(BuiltinIdentifier::Vector, vec![], vec![element])
    }

    pub fn array(element: GenericArgument, size: usize) -> Self {
        Self::builtin(BuiltinIdentifier::Array, vec![], vec![element, size.into()])
    }

    pub fn tuple_or_void(args: Vec<GenericArgument>) -> Self {
        Scope::builtin(
            if args.len() > 0 {
                BuiltinIdentifier::Tuple
            } else {
                BuiltinIdentifier::Void
            },
            vec![],
            args,
        )
    }

    pub fn default_func_type(args: Vec<GenericArgument>) -> Self {
        Scope::builtin(word::default_func_type(), vec![], args)
    }
}

impl From<BuiltinIdentifier> for Scope {
    fn from(ident: BuiltinIdentifier) -> Self {
        Self::builtin(ident, vec![], Vec::new())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScopeSource {
    Builtin(&'static [(Identifier, ScopeKind)]),
    WithinModule {
        file_id: FileId,
        token_group_index: usize, // None means the whole file
    },
    Module {
        file_id: FileId,
    },
}

impl ScopeSource {
    pub fn from_file(file_id: FileId, token_group_index: usize) -> ScopeSource {
        ScopeSource::WithinModule {
            file_id,
            token_group_index: token_group_index,
        }
    }
}

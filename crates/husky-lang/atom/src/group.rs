mod convexity;

use common::*;

use crate::{error::atom_error, *};

use convexity::Convexity;
use scope::{GenericArgument, Scope, ScopeKind};
use word::BuiltinIdentifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtomGroup {
    attr: GroupAttr,
    atoms: Vec<Atom>,
}

#[derive(Clone, PartialEq, Eq, Copy)]
pub struct GroupAttr {
    pub keyword: Option<Keyword>,
    pub is_head: bool,
}

impl std::fmt::Debug for GroupAttr {
    fn fmt(&self, f: &mut common::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{{keyword: {:?}, is_head: {:?}}}",
            &self.keyword, &self.is_head
        ))
    }
}

// new
impl AtomGroup {
    pub(crate) fn new(keyword: Option<Keyword>, is_head: bool) -> Self {
        Self {
            attr: GroupAttr { keyword, is_head },
            atoms: Vec::new(),
        }
    }
}

// get
impl AtomGroup {
    pub(crate) fn convexity(&self) -> Convexity {
        if let Some(atom) = self.atoms.last() {
            convexity::right_side_convexity(&atom.kind)
        } else {
            Convexity::Concave
        }
    }

    pub(crate) fn is_convex(&self) -> bool {
        self.convexity() == Convexity::Convex
    }

    pub(crate) fn is_concave(&self) -> bool {
        self.convexity() == Convexity::Concave
    }

    pub fn attr(&self) -> GroupAttr {
        self.attr
    }
    pub fn atoms(&self) -> &[Atom] {
        &self.atoms
    }
}

// push
impl AtomGroup {
    pub(crate) fn push(&mut self, atom: Atom) -> Result<(), AtomError> {
        if convexity::compatible(self.convexity(), convexity::left_side_convexity(&atom.kind)) {
            self.atoms.push(atom);
            Ok(())
        } else {
            atom_err!(atom.text_range(), AtomRule::CompatibleConvexity,)
        }
    }

    pub(crate) fn end_list(&mut self, ket: Bracket, attr: ListEndAttr, ket_range: TextRange) {
        if self.is_convex() {
            self.push(Atom::new(ket_range.clone(), AtomKind::ListItem))
                .unwrap();
        }
        self.push(Atom::new(ket_range, AtomKind::ListEnd(ket, attr)))
            .unwrap();
    }

    pub(crate) fn end_list_or_make_type(
        &mut self,
        ket: Bracket,
        attr: ListEndAttr,
        mut tail: TextRange,
        db: &dyn AtomQuery,
    ) -> Result<(), AtomError> {
        match (ket, self.atoms.last()) {
            (
                Bracket::Par,
                Some(Atom {
                    kind: AtomKind::Scope(_, ScopeKind::Type),
                    ..
                }),
            ) => {
                let (attr, mut args) = self.pop_par_list_of_types(&mut tail)?;
                let ident = match attr {
                    ListStartAttr::None => BuiltinIdentifier::Tuple,
                    ListStartAttr::Attach => {
                        args.push(ScopeId::Builtin(BuiltinIdentifier::Void).into());
                        self.func_generic(attr)?
                    }
                };
                self.push(db.builtin_type_atom(ident, args, tail))
            }
            _ => Ok(self.end_list(ket, attr, tail)),
        }
    }

    pub(crate) fn start_list(&mut self, bra: Bracket, text_range: TextRange) {
        self.push(Atom::new(
            text_range,
            AtomKind::ListStart(
                bra,
                if self.is_convex() {
                    ListStartAttr::Attach
                } else {
                    ListStartAttr::None
                }
                .into(),
            ),
        ))
        .unwrap();
    }

    pub(crate) fn start_lambda(&mut self, text_range: TextRange) -> Result<(), AtomError> {
        self.push(Atom::new(
            text_range,
            AtomKind::ListStart(Bracket::Vert, ListStartAttr::None),
        ))
    }

    pub(crate) fn end_lambda(&mut self, text_range: TextRange) {
        self.end_list(Bracket::Vert, ListEndAttr::Attach, text_range)
    }

    fn func_generic(&mut self, attr: ListStartAttr) -> AtomResult<BuiltinIdentifier> {
        let expectation = "expect Fp, Fn, FnMut, FnOnce";

        match attr {
            ListStartAttr::None => Ok(word::default_func_type()),
            ListStartAttr::Attach => {
                let last_atom = self.atoms.pop().unwrap();
                match last_atom.kind {
                    AtomKind::Scope(ScopeId::Builtin(ident), _) => match ident {
                        BuiltinIdentifier::Fp
                        | BuiltinIdentifier::Fn
                        | BuiltinIdentifier::FnMut
                        | BuiltinIdentifier::FnOnce => Ok(ident),
                        _ => atom_err!(last_atom.text_range(), expectation),
                    },
                    _ => atom_err!(last_atom.text_range(), expectation),
                }
            }
        }
    }

    fn pop(&mut self, follower: &mut TextRange) -> AtomResult<Atom> {
        let atom = self
            .atoms
            .pop()
            .ok_or(atom_error!(follower.clone(), "something before it"))?;
        *follower = atom.to(follower);
        Ok(atom)
    }

    fn pop_par_list_of_types(
        &mut self,
        tail: &mut TextRange,
    ) -> AtomResult<(ListStartAttr, Vec<GenericArgument>)> {
        let mut types = Vec::new();
        match self.pop(tail)?.kind {
            AtomKind::ListStart(Bracket::Par, attr) => return Ok((attr, Vec::new())),
            AtomKind::Scope(scope, ScopeKind::Type) => types.push(scope.into()),
            _ => atom_err!(tail, "left parenthesis or type")?,
        };
        loop {
            match self.pop(tail)?.kind {
                AtomKind::ListStart(Bracket::Par, attr) => {
                    types.reverse();
                    return Ok((attr, types));
                }
                AtomKind::ListItem => (),
                _ => atom_err!(tail, "left parenthesis or comma")?,
            }
            match self.pop(tail)?.kind {
                AtomKind::Scope(scope, ScopeKind::Type) => types.push(scope.into()),
                _ => atom_err!(tail, "type")?,
            }
        }
    }

    pub(crate) fn make_func_type(
        &mut self,
        db: &dyn AtomQuery,
        output: ScopeId,
        mut tail: TextRange,
    ) -> Result<(), AtomError> {
        let (attr, mut args) = self.pop_par_list_of_types(&mut tail)?;
        args.push(output.into());
        let func_type = self.func_generic(attr)?;
        self.push(db.builtin_type_atom(func_type, args, tail))
    }
}

impl<'a> dyn AtomQuery + 'a {
    pub fn builtin_type_atom(
        &self,
        ident: BuiltinIdentifier,
        args: Vec<GenericArgument>,
        tail: TextRange,
    ) -> Atom {
        let scope = Scope::builtin(ident.into(), args);
        let kind = AtomKind::Scope(self.intern_scope(scope), ScopeKind::Type);
        Atom::new(tail, kind)
    }
}

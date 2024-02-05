//! Declarative instantiation uses runes as opposed to ethereal term or fly term using symbols.
//!
//! Declarative instantiation happens during the construction of ethereal terms without a regional rule system. More precisely, a declarative term is directly converted to an ethereal term without any delay. It can happen that not all implicit argument can be infered, resulting in partial instantiation or no instantiation at all.
//!
//! ```husky
//! struct A<T>(T)
//! ```
//!
//! Consider the term `A(1i32)`, it is a type construtor call of the type template `A` over a value of type `i32`.
//! But `A` must be converted from declarative term to ethereal term first, without the knowledge of what the arguments are.
//! So the type `A` is interpreted as a dependent type, with the dependent variable being called a `rune`.
//! Then we need to determine the implicit argument for constructing the ethereal term corresponding to `A(1i32)`.
//! This is when declarative instantiation kicks in.
//!
//! It's actually only useful for calculating the type of application ethereal term.

use crate::*;
use vec_like::SmallVecPairMap;

pub struct DeclarativeInstantiation {
    /// some rune might not have an instantiation
    rune_map: SmallVecPairMap<DecRune, Option<DecTerm>, 4>,
    /// indicates the separation for associated item template instantiation
    separator: Option<u8>,
}

impl DeclarativeInstantiation {
    pub fn rune_map(&self) -> &[(DecRune, Option<DecTerm>)] {
        self.rune_map.as_ref()
    }

    pub fn separator(&self) -> Option<u8> {
        self.separator
    }

    pub fn rune_instantiation(&self, rune: DecRune) -> Option<DecTerm> {
        *self
            .rune_map
            .get_value(rune)
            .expect("symbol should be in symbol_map")
    }
}

pub trait DeclarativeInstantiate: Copy {
    type Output;

    fn instantiate(
        self,
        db: &::salsa::Db,
        instantiation: &DeclarativeInstantiation,
    ) -> Self::Output;
}

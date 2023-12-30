[
    (
        TypePath(`core::mem::Ref`, `Extern`),
        Ok(
            DeclarativeTerm(`covariant core::basic::Lifetime -> covariant Type -> Type`),
        ),
    ),
    (
        TypePath(`core::mem::RefMut`, `Extern`),
        Ok(
            DeclarativeTerm(`covariant core::basic::Lifetime -> invariant Type -> Type`),
        ),
    ),
    (
        TypePath(`core::mem::Leash`, `Extern`),
        Ok(
            DeclarativeTerm(`covariant Type -> Type`),
        ),
    ),
    (
        TypePath(`core::mem::At`, `Extern`),
        Ok(
            DeclarativeTerm(`independent core::basic::Place -> independent Type -> Type`),
        ),
    ),
]
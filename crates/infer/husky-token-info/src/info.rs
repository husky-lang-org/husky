use husky_entity_taxonomy::EntityKind;

#[derive(Debug, PartialEq, Eq)]
pub enum TokenInfo {
    None,
    Entity(EntityKind),
    ImplicitParameter,
    Parameter,
    Variable,
}

impl Default for TokenInfo {
    fn default() -> Self {
        TokenInfo::None
    }
}

use base_coword::BaseCoword;
use ordered_float::NotNan;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LxSpecLiteral {
    Int(i64),
    Float(NotNan<f64>),
    String(BaseCoword),
}

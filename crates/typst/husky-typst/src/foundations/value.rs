use std::any::{Any, TypeId};
use std::cmp::Ordering;
use std::fmt::{self, Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::sync::Arc;

use ecow::{eco_format, EcoString};
use serde::de::value::{MapAccessDeserializer, SeqAccessDeserializer};
use serde::de::{Error, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::diag::StrResult;
use crate::eval::ops;
use crate::foundations::{
    fields, repr, Array, AutoTypstValue, Bytes, CastInfo, Datetime, Duration, FromTypstValue, Func,
    IntoTypstValue, IsTypstElem, Label, NativeType, NoneTypstValue, Plugin, Reflect, Str, Type,
    TypstArgs, TypstContent, TypstDict, TypstModuleEvaluation, TypstStyles,
    TypstValueAssignmentGroup, TypstValueRepr, Version,
};
use crate::layout::{Angle, Ratio, Rel, TypstAbsLength, TypstEmLength, TypstFraction, TypstLength};
use crate::symbols::Symbol;
use crate::syntax::{ast, TypstSynSpan};
use crate::text::{RawElem, TextElem};
use crate::visualize::{Gradient, Pattern, TypstColor};

/// A computational value.
#[derive(Default, Clone)]
pub enum TypstValue {
    /// The value that indicates the absence of a meaningful value.
    #[default]
    None,
    /// A value that indicates some smart default behaviour.
    Auto,
    /// A boolean: `true, false`.
    Bool(bool),
    /// An integer: `120`.
    Int(i64),
    /// A floating-point number: `1.2`, `10e-4`.
    Float(f64),
    /// A length: `12pt`, `3cm`, `1.5em`, `1em - 2pt`.
    Length(TypstLength),
    /// An angle: `1.5rad`, `90deg`.
    Angle(Angle),
    /// A ratio: `50%`.
    Ratio(Ratio),
    /// A relative length, combination of a ratio and a length: `20% + 5cm`.
    Relative(Rel<TypstLength>),
    /// A fraction: `1fr`.
    Fraction(TypstFraction),
    /// A color value: `#f79143ff`.
    Color(TypstColor),
    /// A gradient value: `gradient.linear(...)`.
    Gradient(Gradient),
    /// A pattern fill: `pattern(...)`.
    Pattern(Pattern),
    /// A symbol: `arrow.l`.
    Symbol(Symbol),
    /// A version.
    Version(Version),
    /// A string: `"string"`.
    Str(Str),
    /// Raw bytes.
    Bytes(Bytes),
    /// A label: `<intro>`.
    Label(Label),
    /// A datetime
    Datetime(Datetime),
    /// A duration
    Duration(Duration),
    /// A content value: `[*Hi* there]`.
    Content(TypstContent),
    // Content styles.
    Styles(TypstStyles),
    /// An array of values: `(1, "hi", 12cm)`.
    Array(Array),
    /// A dictionary value: `(a: 1, b: "hi")`.
    Dict(TypstDict),
    /// An executable function.
    Func(Func),
    /// Captured arguments to a function.
    Args(TypstArgs),
    /// A type.
    Type(Type),
    /// A module.
    Module(TypstModuleEvaluation),
    /// A WebAssembly plugin.
    Plugin(Plugin),
    /// A dynamic value.
    Dyn(CustomTypstValue),
}

impl TypstValue {
    /// Create a new dynamic value.
    pub fn dynamic<T>(any: T) -> Self
    where
        T: Debug + TypstValueRepr + NativeType + PartialEq + Hash + Sync + Send + 'static,
    {
        Self::Dyn(CustomTypstValue::new(any))
    }

    /// Create a numeric value from a number with a unit.
    pub fn numeric(pair: (f64, ast::Unit)) -> Self {
        let (v, unit) = pair;
        match unit {
            ast::Unit::Pt => TypstAbsLength::pt(v).into_value(),
            ast::Unit::Mm => TypstAbsLength::mm(v).into_value(),
            ast::Unit::Cm => TypstAbsLength::cm(v).into_value(),
            ast::Unit::In => TypstAbsLength::inches(v).into_value(),
            ast::Unit::Rad => Angle::rad(v).into_value(),
            ast::Unit::Deg => Angle::deg(v).into_value(),
            ast::Unit::Em => TypstEmLength::new(v).into_value(),
            ast::Unit::Fr => TypstFraction::new(v).into_value(),
            ast::Unit::Percent => Ratio::new(v / 100.0).into_value(),
        }
    }

    /// The type of this value.
    pub fn ty(&self) -> Type {
        match self {
            Self::None => Type::of::<NoneTypstValue>(),
            Self::Auto => Type::of::<AutoTypstValue>(),
            Self::Bool(_) => Type::of::<bool>(),
            Self::Int(_) => Type::of::<i64>(),
            Self::Float(_) => Type::of::<f64>(),
            Self::Length(_) => Type::of::<TypstLength>(),
            Self::Angle(_) => Type::of::<Angle>(),
            Self::Ratio(_) => Type::of::<Ratio>(),
            Self::Relative(_) => Type::of::<Rel<TypstLength>>(),
            Self::Fraction(_) => Type::of::<TypstFraction>(),
            Self::Color(_) => Type::of::<TypstColor>(),
            Self::Gradient(_) => Type::of::<Gradient>(),
            Self::Pattern(_) => Type::of::<Pattern>(),
            Self::Symbol(_) => Type::of::<Symbol>(),
            Self::Version(_) => Type::of::<Version>(),
            Self::Str(_) => Type::of::<Str>(),
            Self::Bytes(_) => Type::of::<Bytes>(),
            Self::Label(_) => Type::of::<Label>(),
            Self::Datetime(_) => Type::of::<Datetime>(),
            Self::Duration(_) => Type::of::<Duration>(),
            Self::Content(_) => Type::of::<TypstContent>(),
            Self::Styles(_) => Type::of::<TypstStyles>(),
            Self::Array(_) => Type::of::<Array>(),
            Self::Dict(_) => Type::of::<TypstDict>(),
            Self::Func(_) => Type::of::<Func>(),
            Self::Args(_) => Type::of::<TypstArgs>(),
            Self::Type(_) => Type::of::<Type>(),
            Self::Module(_) => Type::of::<TypstModuleEvaluation>(),
            Self::Plugin(_) => Type::of::<TypstModuleEvaluation>(),
            Self::Dyn(v) => v.ty(),
        }
    }

    /// Try to cast the value into a specific type.
    pub fn cast<T: FromTypstValue>(self) -> StrResult<T> {
        T::from_value(self)
    }

    /// Try to access a field on the value.
    pub fn field(&self, field: &str) -> StrResult<TypstValue> {
        match self {
            Self::Symbol(symbol) => symbol.clone().modified(field).map(Self::Symbol),
            Self::Version(version) => version.component(field).map(Self::Int),
            Self::Dict(dict) => dict.get(field).cloned(),
            Self::Content(content) => content.field_by_name(field),
            Self::Type(ty) => ty.field(field).cloned(),
            Self::Func(func) => func.field(field).cloned(),
            Self::Module(module) => module.field(field).cloned(),
            _ => fields::field(self, field),
        }
    }

    /// The associated scope, if this is a function, type, or module.
    pub fn scope(&self) -> Option<&TypstValueAssignmentGroup> {
        match self {
            Self::Func(func) => func.scope(),
            Self::Type(ty) => Some(ty.scope()),
            Self::Module(module) => Some(module.scope()),
            _ => None,
        }
    }

    /// The name, if this is a function, type, or module.
    pub fn name(&self) -> Option<&str> {
        match self {
            Self::Func(func) => func.name(),
            Self::Type(ty) => Some(ty.short_name()),
            Self::Module(module) => Some(module.name()),
            _ => None,
        }
    }

    /// Try to extract documentation for the value.
    pub fn docs(&self) -> Option<&'static str> {
        match self {
            Self::Func(func) => func.docs(),
            Self::Type(ty) => Some(ty.docs()),
            _ => None,
        }
    }

    /// Return the display representation of the value.
    pub fn display(self) -> TypstContent {
        match self {
            Self::None => TypstContent::empty(),
            Self::Int(v) => TextElem::packed(repr::format_int_with_base(v, 10)),
            Self::Float(v) => TextElem::packed(repr::display_float(v)),
            Self::Str(v) => TextElem::packed(v),
            Self::Version(v) => TextElem::packed(eco_format!("{v}")),
            Self::Symbol(v) => TextElem::packed(v.get()),
            Self::Content(v) => v,
            Self::Module(module) => module.content(),
            _ => RawElem::new(self.repr())
                .with_lang(Some("typc".into()))
                .with_block(false)
                .pack(),
        }
    }

    /// Attach a span to the value, if possible.
    pub fn spanned(self, span: TypstSynSpan) -> Self {
        match self {
            TypstValue::Content(v) => TypstValue::Content(v.spanned(span)),
            TypstValue::Func(v) => TypstValue::Func(v.spanned(span)),
            v => v,
        }
    }
}

impl Debug for TypstValue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::None => Debug::fmt(&NoneTypstValue, f),
            Self::Auto => Debug::fmt(&AutoTypstValue, f),
            Self::Bool(v) => Debug::fmt(v, f),
            Self::Int(v) => Debug::fmt(v, f),
            Self::Float(v) => Debug::fmt(v, f),
            Self::Length(v) => Debug::fmt(v, f),
            Self::Angle(v) => Debug::fmt(v, f),
            Self::Ratio(v) => Debug::fmt(v, f),
            Self::Relative(v) => Debug::fmt(v, f),
            Self::Fraction(v) => Debug::fmt(v, f),
            Self::Color(v) => Debug::fmt(v, f),
            Self::Gradient(v) => Debug::fmt(v, f),
            Self::Pattern(v) => Debug::fmt(v, f),
            Self::Symbol(v) => Debug::fmt(v, f),
            Self::Version(v) => Debug::fmt(v, f),
            Self::Str(v) => Debug::fmt(v, f),
            Self::Bytes(v) => Debug::fmt(v, f),
            Self::Label(v) => Debug::fmt(v, f),
            Self::Datetime(v) => Debug::fmt(v, f),
            Self::Duration(v) => Debug::fmt(v, f),
            Self::Content(v) => Debug::fmt(v, f),
            Self::Styles(v) => Debug::fmt(v, f),
            Self::Array(v) => Debug::fmt(v, f),
            Self::Dict(v) => Debug::fmt(v, f),
            Self::Func(v) => Debug::fmt(v, f),
            Self::Args(v) => Debug::fmt(v, f),
            Self::Type(v) => Debug::fmt(v, f),
            Self::Module(v) => Debug::fmt(v, f),
            Self::Plugin(v) => Debug::fmt(v, f),
            Self::Dyn(v) => Debug::fmt(v, f),
        }
    }
}

impl TypstValueRepr for TypstValue {
    fn repr(&self) -> EcoString {
        match self {
            Self::None => NoneTypstValue.repr(),
            Self::Auto => AutoTypstValue.repr(),
            Self::Bool(v) => v.repr(),
            Self::Int(v) => v.repr(),
            Self::Float(v) => v.repr(),
            Self::Length(v) => v.repr(),
            Self::Angle(v) => v.repr(),
            Self::Ratio(v) => v.repr(),
            Self::Relative(v) => v.repr(),
            Self::Fraction(v) => v.repr(),
            Self::Color(v) => v.repr(),
            Self::Gradient(v) => v.repr(),
            Self::Pattern(v) => v.repr(),
            Self::Symbol(v) => v.repr(),
            Self::Version(v) => v.repr(),
            Self::Str(v) => v.repr(),
            Self::Bytes(v) => v.repr(),
            Self::Label(v) => v.repr(),
            Self::Datetime(v) => v.repr(),
            Self::Duration(v) => v.repr(),
            Self::Content(v) => v.repr(),
            Self::Styles(v) => v.repr(),
            Self::Array(v) => v.repr(),
            Self::Dict(v) => v.repr(),
            Self::Func(v) => v.repr(),
            Self::Args(v) => v.repr(),
            Self::Type(v) => v.repr(),
            Self::Module(v) => v.repr(),
            Self::Plugin(v) => v.repr(),
            Self::Dyn(v) => v.repr(),
        }
    }
}

impl PartialEq for TypstValue {
    fn eq(&self, other: &Self) -> bool {
        ops::equal(self, other)
    }
}

impl PartialOrd for TypstValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        ops::compare(self, other).ok()
    }
}

impl Hash for TypstValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            Self::None => {}
            Self::Auto => {}
            Self::Bool(v) => v.hash(state),
            Self::Int(v) => v.hash(state),
            Self::Float(v) => v.to_bits().hash(state),
            Self::Length(v) => v.hash(state),
            Self::Angle(v) => v.hash(state),
            Self::Ratio(v) => v.hash(state),
            Self::Relative(v) => v.hash(state),
            Self::Fraction(v) => v.hash(state),
            Self::Color(v) => v.hash(state),
            Self::Gradient(v) => v.hash(state),
            Self::Pattern(v) => v.hash(state),
            Self::Symbol(v) => v.hash(state),
            Self::Version(v) => v.hash(state),
            Self::Str(v) => v.hash(state),
            Self::Bytes(v) => v.hash(state),
            Self::Label(v) => v.hash(state),
            Self::Content(v) => v.hash(state),
            Self::Styles(v) => v.hash(state),
            Self::Datetime(v) => v.hash(state),
            Self::Duration(v) => v.hash(state),
            Self::Array(v) => v.hash(state),
            Self::Dict(v) => v.hash(state),
            Self::Func(v) => v.hash(state),
            Self::Args(v) => v.hash(state),
            Self::Type(v) => v.hash(state),
            Self::Module(v) => v.hash(state),
            Self::Plugin(v) => v.hash(state),
            Self::Dyn(v) => v.hash(state),
        }
    }
}

impl Serialize for TypstValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => NoneTypstValue.serialize(serializer),
            Self::Bool(v) => v.serialize(serializer),
            Self::Int(v) => v.serialize(serializer),
            Self::Float(v) => v.serialize(serializer),
            Self::Str(v) => v.serialize(serializer),
            Self::Bytes(v) => v.serialize(serializer),
            Self::Symbol(v) => v.serialize(serializer),
            Self::Content(v) => v.serialize(serializer),
            Self::Array(v) => v.serialize(serializer),
            Self::Dict(v) => v.serialize(serializer),

            // Fall back to repr() for other things.
            other => serializer.serialize_str(&other.repr()),
        }
    }
}

impl<'de> Deserialize<'de> for TypstValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ValueVisitor)
    }
}

/// Visitor for value deserialization.
struct ValueVisitor;

impl<'de> Visitor<'de> for ValueVisitor {
    type Value = TypstValue;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a typst value")
    }

    fn visit_bool<E: Error>(self, v: bool) -> Result<Self::Value, E> {
        Ok(v.into_value())
    }

    fn visit_i8<E: Error>(self, v: i8) -> Result<Self::Value, E> {
        Ok(v.into_value())
    }

    fn visit_i16<E: Error>(self, v: i16) -> Result<Self::Value, E> {
        Ok(v.into_value())
    }

    fn visit_i32<E: Error>(self, v: i32) -> Result<Self::Value, E> {
        Ok(v.into_value())
    }

    fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> {
        Ok(v.into_value())
    }

    fn visit_u8<E: Error>(self, v: u8) -> Result<Self::Value, E> {
        Ok(v.into_value())
    }

    fn visit_u16<E: Error>(self, v: u16) -> Result<Self::Value, E> {
        Ok(v.into_value())
    }

    fn visit_u32<E: Error>(self, v: u32) -> Result<Self::Value, E> {
        Ok(v.into_value())
    }

    fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
        Ok(v.into_value())
    }

    fn visit_f32<E: Error>(self, v: f32) -> Result<Self::Value, E> {
        Ok((v as f64).into_value())
    }

    fn visit_f64<E: Error>(self, v: f64) -> Result<Self::Value, E> {
        Ok(v.into_value())
    }

    fn visit_char<E: Error>(self, v: char) -> Result<Self::Value, E> {
        Ok(v.into_value())
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        Ok(v.into_value())
    }

    fn visit_borrowed_str<E: Error>(self, v: &'de str) -> Result<Self::Value, E> {
        Ok(v.into_value())
    }

    fn visit_string<E: Error>(self, v: String) -> Result<Self::Value, E> {
        Ok(v.into_value())
    }

    fn visit_bytes<E: Error>(self, v: &[u8]) -> Result<Self::Value, E> {
        Ok(Bytes::from(v).into_value())
    }

    fn visit_borrowed_bytes<E: Error>(self, v: &'de [u8]) -> Result<Self::Value, E> {
        Ok(Bytes::from(v).into_value())
    }

    fn visit_byte_buf<E: Error>(self, v: Vec<u8>) -> Result<Self::Value, E> {
        Ok(Bytes::from(v).into_value())
    }

    fn visit_none<E: Error>(self) -> Result<Self::Value, E> {
        Ok(TypstValue::None)
    }

    fn visit_some<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        TypstValue::deserialize(deserializer)
    }

    fn visit_unit<E: Error>(self) -> Result<Self::Value, E> {
        Ok(TypstValue::None)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, seq: A) -> Result<Self::Value, A::Error> {
        Ok(Array::deserialize(SeqAccessDeserializer::new(seq))?.into_value())
    }

    fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
        let dict = TypstDict::deserialize(MapAccessDeserializer::new(map))?;
        Ok(match Datetime::from_toml_dict(&dict) {
            None => dict.into_value(),
            Some(datetime) => datetime.into_value(),
        })
    }
}

/// A value that is not part of the built-in enum.
#[derive(Clone, Hash)]
#[allow(clippy::derived_hash_with_manual_eq)]
pub struct CustomTypstValue(Arc<dyn TypstValueDyn>);

impl CustomTypstValue {
    /// Create a new instance from any value that satisfies the required bounds.
    pub fn new<T>(any: T) -> Self
    where
        T: Debug + TypstValueRepr + NativeType + PartialEq + Hash + Sync + Send + 'static,
    {
        Self(Arc::new(any))
    }

    /// Whether the wrapped type is `T`.
    pub fn is<T: 'static>(&self) -> bool {
        (*self.0).as_any().is::<T>()
    }

    /// Try to downcast to a reference to a specific type.
    pub fn downcast<T: 'static>(&self) -> Option<&T> {
        (*self.0).as_any().downcast_ref()
    }

    /// The name of the stored value's type.
    pub fn ty(&self) -> Type {
        self.0.dyn_ty()
    }
}

impl Debug for CustomTypstValue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl TypstValueRepr for CustomTypstValue {
    fn repr(&self) -> EcoString {
        self.0.repr()
    }
}

impl PartialEq for CustomTypstValue {
    fn eq(&self, other: &Self) -> bool {
        self.0.dyn_eq(other)
    }
}

trait TypstValueDyn: Debug + TypstValueRepr + Sync + Send + 'static {
    fn as_any(&self) -> &dyn Any;
    fn dyn_eq(&self, other: &CustomTypstValue) -> bool;
    fn dyn_ty(&self) -> Type;
    fn dyn_hash(&self, state: &mut dyn Hasher);
}

impl<T> TypstValueDyn for T
where
    T: Debug + TypstValueRepr + NativeType + PartialEq + Hash + Sync + Send + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn dyn_eq(&self, other: &CustomTypstValue) -> bool {
        let Some(other) = other.downcast::<Self>() else {
            return false;
        };
        self == other
    }

    fn dyn_ty(&self) -> Type {
        Type::of::<T>()
    }

    fn dyn_hash(&self, mut state: &mut dyn Hasher) {
        // Also hash the TypeId since values with different types but
        // equal data should be different.
        TypeId::of::<Self>().hash(&mut state);
        self.hash(&mut state);
    }
}

impl Hash for dyn TypstValueDyn {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dyn_hash(state);
    }
}

/// Implements traits for primitives (TypstValue enum variants).
macro_rules! primitive {
    (
        $ty:ty: $name:literal, $variant:ident
        $(, $other:ident$(($binding:ident))? => $out:expr)*
    ) => {
        impl Reflect for $ty {
            fn input() -> CastInfo {
                CastInfo::Type(Type::of::<Self>())
            }

            fn output() -> CastInfo {
                CastInfo::Type(Type::of::<Self>())
            }

            fn castable(value: &TypstValue) -> bool {
                matches!(value, TypstValue::$variant(_)
                    $(|  primitive!(@$other $(($binding))?))*)
            }
        }

        impl IntoTypstValue for $ty {
            fn into_value(self) -> TypstValue {
                TypstValue::$variant(self)
            }
        }

        impl FromTypstValue for $ty {
            fn from_value(value: TypstValue) -> StrResult<Self> {
                match value {
                    TypstValue::$variant(v) => Ok(v),
                    $(TypstValue::$other$(($binding))? => Ok($out),)*
                    v => Err(eco_format!(
                        "expected {}, found {}",
                        Type::of::<Self>(),
                        v.ty(),
                    )),
                }
            }
        }
    };

    (@$other:ident($binding:ident)) => { TypstValue::$other(_) };
    (@$other:ident) => { TypstValue::$other };
}

primitive! { bool: "boolean", Bool }
primitive! { i64: "integer", Int }
primitive! { f64: "float", Float, Int(v) => v as f64 }
primitive! { TypstLength: "length", Length }
primitive! { Angle: "angle", Angle }
primitive! { Ratio: "ratio", Ratio }
primitive! { Rel<TypstLength>:  "relative length",
    Relative,
    Length(v) => v.into(),
    Ratio(v) => v.into()
}
primitive! { TypstFraction: "fraction", Fraction }
primitive! { TypstColor: "color", Color }
primitive! { Gradient: "gradient", Gradient }
primitive! { Pattern: "pattern", Pattern }
primitive! { Symbol: "symbol", Symbol }
primitive! { Version: "version", Version }
primitive! {
    Str: "string",
    Str,
    Symbol(symbol) => symbol.get().into()
}
primitive! { Bytes: "bytes", Bytes }
primitive! { Label: "label", Label }
primitive! { Datetime: "datetime", Datetime }
primitive! { Duration: "duration", Duration }
primitive! { TypstContent: "content",
    Content,
    None => TypstContent::empty(),
    Symbol(v) => TextElem::packed(v.get()),
    Str(v) => TextElem::packed(v)
}
primitive! { TypstStyles: "styles", Styles }
primitive! { Array: "array", Array }
primitive! { TypstDict: "dictionary", Dict }
primitive! {
    Func: "function",
    Func,
    Type(ty) => ty.constructor()?.clone()
}
primitive! { TypstArgs: "arguments", Args }
primitive! { Type: "type", Type }
primitive! { TypstModuleEvaluation: "module", Module }
primitive! { Plugin: "plugin", Plugin }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::foundations::{array, dict};

    #[track_caller]
    fn test(value: impl IntoTypstValue, exp: &str) {
        assert_eq!(value.into_value().repr(), exp);
    }

    #[test]
    fn test_value_debug() {
        // Primitives.
        test(TypstValue::None, "none");
        test(false, "false");
        test(12i64, "12");
        test(3.24, "3.24");
        test(TypstAbsLength::pt(5.5), "5.5pt");
        test(Angle::deg(90.0), "90deg");
        test(Ratio::one() / 2.0, "50%");
        test(
            Ratio::new(0.3) + TypstLength::from(TypstAbsLength::cm(2.0)),
            "30% + 56.69pt",
        );
        test(TypstFraction::one() * 7.55, "7.55fr");

        // Collections.
        test("hello", r#""hello""#);
        test("\n", r#""\n""#);
        test("\\", r#""\\""#);
        test("\"", r#""\"""#);
        test(array![], "()");
        test(array![TypstValue::None], "(none,)");
        test(array![1, 2], "(1, 2)");
        test(dict![], "(:)");
        test(dict!["one" => 1], "(one: 1)");
        test(dict!["two" => false, "one" => 1], "(two: false, one: 1)");
    }
}
use crate::*;

// ts: { type: string; value: string; spaces_before?: number }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenProps<'eval> {
    pub kind: TraceTokenKind,
    pub value: Cow<'static, str>,
    pub associated_trace: Option<Arc<Trace<'eval>>>,
}

impl<'eval> Serialize for TokenProps<'eval> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("TokenProps", 3)?;
        state.serialize_field("kind", &self.kind)?;
        state.serialize_field("value", &self.value)?;
        state.serialize_field(
            "associated_trace",
            &self.associated_trace.as_ref().map(|trace| trace.id),
        )?;
        state.end()
    }
}

impl<'eval> From<EvalResult<'eval>> for TokenProps<'eval> {
    fn from(result: EvalResult<'eval>) -> Self {
        match result {
            Ok(value) => value.into(),
            Err(e) => Self {
                value: e.message.into(),
                associated_trace: None,
                kind: TraceTokenKind::Error,
            },
        }
    }
}

impl<'eval> From<EvalValue<'eval>> for TokenProps<'eval> {
    fn from(value: EvalValue<'eval>) -> Self {
        match value {
            EvalValue::Copyable(value) => fade!(value),
            EvalValue::Owned(value) => fade!(value.any_ref().print_short()),
            EvalValue::GlobalPure(value) => fade!(value.print_short()),
            EvalValue::EvalRef(value) => fade!(value.print_short()),
            EvalValue::Undefined => fade!("undefined"),
        }
    }
}

impl<'eval> From<VMRuntimeResult<StackValueSnapshot<'eval>>> for TokenProps<'eval> {
    fn from(_: VMRuntimeResult<StackValueSnapshot>) -> Self {
        todo!()
    }
}

impl<'eval> From<VMRuntimeResult<CopyableValue>> for TokenProps<'eval> {
    fn from(result: VMRuntimeResult<CopyableValue>) -> Self {
        match result {
            Ok(value) => value.into(),
            Err(e) => Self {
                value: e.message.into(),
                associated_trace: None,
                kind: TraceTokenKind::Error,
            },
        }
    }
}

impl<'eval> From<CopyableValue> for TokenProps<'eval> {
    fn from(value: CopyableValue) -> Self {
        fade!(value)
    }
}

impl<'eval> From<InitKind> for TokenProps<'eval> {
    fn from(init_kind: InitKind) -> Self {
        match init_kind {
            InitKind::Let => keyword!("let "),
            InitKind::Var => keyword!("var "),
            InitKind::Decl => panic!(),
        }
    }
}

// ts: string
#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TraceTokenKind {
    Keyword,
    Label,
    Ident,
    Literal,
    Special,
    Scope,
    Fade,
    Error,
}

#[macro_export]
macro_rules! keyword {
    ($value:expr) => {{
        TokenProps {
            kind: TraceTokenKind::Keyword,
            value: $value.into(),
            associated_trace: None,
        }
    }};
}

#[macro_export]
macro_rules! label {
    ($value:expr, $associated:expr) => {{
        TokenProps {
            kind: TraceTokenKind::Label,
            value: $value.into(),
            spaces_before: None,
            associated: $associated,
            associated: vec![],
        }
    }};
}

#[macro_export]
macro_rules! ident {
    ($value:expr) => {{
        TokenProps {
            kind: TraceTokenKind::Ident,
            value: $value.into(),
            associated_trace: None,
        }
    }};
    ($value:expr, $associated_trace: expr) => {{
        TokenProps {
            kind: TraceTokenKind::Ident,
            value: $value.into(),
            associated_trace: $associated_trace,
        }
    }};
}

#[macro_export]
macro_rules! literal {
    ($value:expr) => {{
        TokenProps {
            kind: TraceTokenKind::Literal,
            value: $value.into(),
            associated_trace: None,
        }
    }};
}

#[macro_export]
macro_rules! special {
    ($value: expr) => {{
        TokenProps {
            kind: TraceTokenKind::Special,
            value: $value.into(),
            associated_trace: None,
        }
    }};

    ($value: expr, $associated_trace: expr) => {{
        TokenProps {
            kind: TraceTokenKind::Special,
            value: $value.into(),
            associated_trace: $associated_trace,
        }
    }};
}

#[macro_export]
macro_rules! scope {
    ($value:expr) => {{
        TokenProps {
            kind: TraceTokenKind::Scope,
            value: $value.into(),
            associated_trace: None,
        }
    }};

    ($value:expr, $associated_trace: expr) => {{
        TokenProps {
            kind: TraceTokenKind::Scope,
            value: $value.into(),
            associated_trace: $associated_trace,
        }
    }};
}

#[macro_export]
macro_rules! fade {
    ($value:expr) => {{
        TokenProps {
            kind: TraceTokenKind::Fade,
            value: $value.into(),
            associated_trace: None,
        }
    }};
    ($value:expr, $associated:expr) => {{
        TokenProps {
            kind: TraceTokenKind::Fade,
            value: $value.into(),
            associated_trace: $associated,
        }
    }};
}

use vm::{CopyableValue, EvalResult, EvalValue, InitKind, StackValueSnapshot, VMRuntimeResult};

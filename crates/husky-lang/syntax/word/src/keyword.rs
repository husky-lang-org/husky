#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Keyword {
    Config(ConfigKeyword),
    Func(FuncKeyword),
    Type(TypeKeyword),
    Stmt(StmtKeyword),
    Use,
    Mod,
}

impl From<ConfigKeyword> for Keyword {
    fn from(kw: ConfigKeyword) -> Self {
        Self::Config(kw)
    }
}

impl From<FuncKeyword> for Keyword {
    fn from(kw: FuncKeyword) -> Self {
        Keyword::Func(kw)
    }
}

impl From<TypeKeyword> for Keyword {
    fn from(kw: TypeKeyword) -> Self {
        Keyword::Type(kw)
    }
}

impl From<StmtKeyword> for Keyword {
    fn from(stmt: StmtKeyword) -> Self {
        Keyword::Stmt(stmt)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ConfigKeyword {
    Dataset,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum FuncKeyword {
    Main,
    Test,
    Proc,
    Func,
    Def,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TypeKeyword {
    Struct,
    Rename,
    Enum,
    Props,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum StmtKeyword {
    Let,
    Var,
    If,
    Elif,
    Else,
    Switch,
    Match,
    Case,
    DeFault,
    For,
    ForExt,
    While,
    Do,
    Break,
    Return,
    Assert,
}

impl StmtKeyword {
    pub fn code(&self) -> &'static str {
        match self {
            StmtKeyword::Let => "let",
            StmtKeyword::Var => "var",
            StmtKeyword::If => "if",
            StmtKeyword::Elif => "elif",
            StmtKeyword::Else => "else",
            StmtKeyword::Switch => "switch",
            StmtKeyword::Match => "match",
            StmtKeyword::Case => "case",
            StmtKeyword::DeFault => "default",
            StmtKeyword::For => "for",
            StmtKeyword::ForExt => "forext",
            StmtKeyword::While => "while",
            StmtKeyword::Do => "do",
            StmtKeyword::Break => "break",
            StmtKeyword::Return => "return",
            StmtKeyword::Assert => "assert",
        }
    }
}

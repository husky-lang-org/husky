use husky_entity_tree::error::EntityTreeResult;
use husky_sem_expr::{helpers::region::sem_expr_region_from_region_path, SemExprRegionData};
use husky_text_protocol::range::TextRange;

use husky_token::{verse::idx::TokenVerseIdx, TokenDb};
use husky_token_data::{Keyword, TokenData};
use husky_token_info::{TokenInfo, TokenInfoData, TokenInfoDb, TokenInfoSource};

use crate::*;

pub(crate) fn calc_hover_result(
    db: &::salsa::Db,
    module_path: ModulePath,
    token_idx: TokenIdx,
) -> Option<HoverResult> {
    HoverResultCalculator::new(db, module_path, token_idx)
        .ok()?
        .gen_content()
}

struct HoverResultCalculator<'db> {
    db: &'db ::salsa::Db,
    module_path: ModulePath,
    token_idx: TokenIdx,
    token: &'db TokenData,
    token_range: TextRange,
    token_info: Option<&'db TokenInfo>,
    markdown_content: String,
    actions: Vec<CommandLinkGroup>,
    config: &'db HoverConfig,
    token_verse_idx: TokenVerseIdx,
    sem_expr_region_data: Option<&'db SemExprRegionData>,
}

impl<'a> HoverResultCalculator<'a> {
    fn new(
        db: &'a ::salsa::Db,
        module_path: ModulePath,
        token_idx: TokenIdx,
    ) -> EntityTreeResult<Self> {
        let ranged_token_sheet = db.ranged_token_sheet(module_path);
        let token_sheet_data = db.token_sheet_data(module_path);
        let token_info_sheet = db.token_info_sheet(module_path)?;
        let token_verse_idx = token_sheet_data.token_verse_idx(token_idx);
        let token_info = token_info_sheet[token_idx].as_ref();
        let sem_expr_region_data = match token_info {
            Some(token_info) => match token_info.src() {
                TokenInfoSource::SemExpr(region_path, _) => {
                    sem_expr_region_from_region_path(region_path, db).map(|region| region.data(db))
                }
                _ => None,
            },
            None => None,
        };
        Ok(Self {
            db,
            module_path,
            token_idx,
            token: &token_sheet_data[token_idx],
            token_range: ranged_token_sheet.token_text_range(token_idx),
            token_info,
            markdown_content: String::new(),
            actions: vec![],
            config: hover_config(db, module_path),
            token_verse_idx,
            sem_expr_region_data,
        })
    }

    fn gen_content(mut self) -> Option<HoverResult> {
        self.markdown_content += &self.content();
        if self.config.debug {
            self.markdown_content += &self.debug_content()
        }
        Some(self.finish())
    }

    fn finish(self) -> HoverResult {
        HoverResult {
            hover: lsp_types::Hover {
                contents: lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
                    kind: lsp_types::MarkupKind::Markdown,
                    value: self.markdown_content,
                }),
                range: Some(self.token_range.into()),
            },
            actions: self.actions,
        }
    }

    fn gen_keyword_content(&self, kw: Keyword) -> &'static str {
        match kw {
            Keyword::Form(_keyword) => "This is a paradigm",
            _ => "Other",
        }
    }

    fn content(&self) -> std::borrow::Cow<'static, str> {
        match self.token {
            TokenData::Keyword(kw) => self.gen_keyword_content(*kw).into(),
            _ => "".into(),
        }
    }

    fn debug_content(&self) -> String {
        use std::fmt::Write;

        let db = self.db;

        let mut debug_content = String::new();
        if self.config.description {
            self.add_description(&mut debug_content);
        }

        // lex

        if self.config.token_idx {
            write!(debug_content, "\ntoken_idx = {};\n", self.token_idx.index()).unwrap();
        }

        if self.config.token_line_group_idx {
            write!(
                debug_content,
                "token_line_group_idx = {}\n",
                self.token_verse_idx
            )
            .unwrap();
        }

        if self.config.token {
            write!(debug_content, "token = {:#?};\n", self.token.debug(self.db)).unwrap();
        }

        if self.config.token_info {
            write!(
                debug_content,
                "token_info = {:#?};\n",
                self.token_info.debug(self.db)
            )
            .unwrap();
        }

        // syntax

        // semantics

        if self.config.coersion {
            if let Some(token_info) = self.token_info
                && let TokenInfoSource::SemExpr(region_path, expr) = token_info.src()
            {
                let sem_expr_region_data = self.sem_expr_region_data.unwrap();
                write!(
                    debug_content,
                    "\n\ncoercion = {:#?}",
                    expr.expectation_outcome(sem_expr_region_data)
                        .map(|outcome| outcome.coercion())
                        .flatten()
                )
                .unwrap();
            }
        }

        debug_content
    }

    fn add_description(&self, debug_content: &mut String) {
        use salsa::DisplayWithDb;
        use std::fmt::Write;

        let db = self.db;
        match self.token_info {
            Some(ref info) => match info.data() {
                TokenInfoData::Entity(path) => {
                    write!(debug_content, "entity `{}`", path.display_with(db))
                }
                TokenInfoData::EntityNode(path, _) => {
                    write!(debug_content, "entity node")
                }
                TokenInfoData::CurrentVariable {
                    current_variable_idx,
                    syn_expr_region,
                    ..
                } => write!(
                    debug_content,
                    "variable",
                    // syn_expr_region.data(self.db).variable_region()[*current_variable_idx]
                    //     .debug(self.db)
                ),
                TokenInfoData::InheritedVariable {
                    inherited_variable_idx,
                    syn_expr_region,
                    ..
                } => write!(
                    debug_content,
                    "variable",
                    // syn_expr_region.data(self.db).variable_region()[*inherited_variable_idx]
                    //     .debug(self.db)
                ),
                TokenInfoData::Field => write!(debug_content, "field"),
                TokenInfoData::Method => write!(debug_content, "method"),
                TokenInfoData::BoxColon => write!(debug_content, "box colon"),
                TokenInfoData::VecFunctorBoxPrefix => {
                    write!(debug_content, "vec functor box prefix")
                }
                TokenInfoData::ArrayFunctorBoxPrefix => {
                    write!(debug_content, "array functor box prefix")
                }
                TokenInfoData::UseExpr { .. } => write!(debug_content, "use"),
                TokenInfoData::UseExprStar => write!(debug_content, "use expr star"),
                TokenInfoData::SelfType => write!(debug_content, "self type"),
                TokenInfoData::SelfValue => write!(debug_content, "self value"),
                TokenInfoData::HtmlFunctionIdent => write!(debug_content, "html function ident"),
                TokenInfoData::HtmlPropertyIdent => write!(debug_content, "html property ident"),
                TokenInfoData::UnitLeftParenthesis => {
                    write!(debug_content, "unit left parenthesis")
                }
                TokenInfoData::UnitRightParenthesis => {
                    write!(debug_content, "unit right parenthesis")
                }
                TokenInfoData::Todo => write!(debug_content, "todo"),
                TokenInfoData::Unreachable => write!(debug_content, "unreachable"),
                TokenInfoData::PrefixTypeOpr => write!(debug_content, "prefix type operator"),
                TokenInfoData::CallPar => write!(debug_content, "call par"),
                TokenInfoData::NestedBlockCurl => write!(debug_content, "nested block curl"),
                TokenInfoData::ClosureVert => write!(debug_content, "closure vert"),
                TokenInfoData::ClosureLightArrow => write!(debug_content, "closure light arrow"),
                TokenInfoData::ClosureEq => write!(debug_content, "closure eq"),
            },
            None => write!(debug_content, ""),
        }
        .unwrap();
    }
}

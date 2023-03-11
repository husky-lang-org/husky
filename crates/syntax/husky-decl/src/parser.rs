use crate::*;
use husky_ast::*;

use husky_entity_taxonomy::*;
use husky_entity_tree::*;

use husky_token::*;

use parsec::*;

pub(crate) fn module_item_decl(db: &dyn DeclDb, path: ModuleItemPath) -> DeclResultRef<Decl> {
    match path {
        ModuleItemPath::Type(path) => ty_decl(db, path).as_ref().map(|decl| (*decl).into()),
        ModuleItemPath::Trait(path) => trai_decl(db, path).as_ref().map(|decl| (*decl).into()),
        ModuleItemPath::Form(path) => form_decl(db, path).as_ref().map(|decl| (*decl).into()),
    }
}

#[salsa::tracked(jar = DeclJar, return_ref)]
pub(crate) fn ty_decl(db: &dyn DeclDb, path: TypePath) -> DeclResult<TypeDecl> {
    let parser = DeclParser::new(db, path.module_path(db))?;
    parser.parse_ty_decl(path)
}

#[test]
fn ty_decl_works() {
    let db = DB::default();
    let toolchain = db.dev_toolchain().unwrap();
    let menu = db.entity_path_menu(toolchain).unwrap();
    assert!(db.ty_decl(menu.never_ty_path()).is_ok());
}

#[salsa::tracked(jar = DeclJar, return_ref)]
pub(crate) fn form_decl(db: &dyn DeclDb, path: FormPath) -> DeclResult<FormDecl> {
    let parser = DeclParser::new(db, path.module_path(db))?;
    parser.parse_form_decl(path)
}

#[salsa::tracked(jar = DeclJar,return_ref)]
pub(crate) fn trai_decl(db: &dyn DeclDb, path: TraitPath) -> DeclResult<TraitDecl> {
    let parser = DeclParser::new(db, path.module_path(db))?;
    parser.parse_trai_decl(path)
}

pub(crate) fn impl_decl(db: &dyn DeclDb, impl_block: ImplBlock) -> DeclResultRef<ImplDecl> {
    match impl_block {
        ImplBlock::Type(impl_block) => ty_impl_decl(db, impl_block)
            .as_ref()
            .copied()
            .map(Into::into),
        ImplBlock::TypeAsTrait(impl_block) => ty_as_trai_impl_decl(db, impl_block)
            .as_ref()
            .copied()
            .map(Into::into),
        ImplBlock::IllFormed(_) => Err(&DeclError::Derived(DerivedDeclError::ImplErr)),
    }
}

#[salsa::tracked(jar = DeclJar,return_ref)]
pub(crate) fn associated_item_decl(
    db: &dyn DeclDb,
    associated_item: AssociatedItem,
) -> DeclResult<AssociatedItemDecl> {
    let parser = DeclParser::new(db, associated_item.module_path(db))?;
    parser.parse_associated_item_decl(associated_item)
}

pub(crate) struct DeclParser<'a> {
    db: &'a dyn DeclDb,
    module_symbol_context: ModuleSymbolContext<'a>,
    token_sheet_data: &'a TokenSheetData,
    ast_sheet: &'a AstSheet,
    module_entity_tree: &'a EntityTreeSheet,
    entity_tree_crate_bundle: &'a EntityTreeCrateBundle,
}

impl<'a> DeclParser<'a> {
    pub(crate) fn new(db: &'a dyn DeclDb, path: ModulePath) -> EntityTreeResult<Self> {
        let module_symbol_context = db.module_symbol_context(path)?;
        Ok(Self {
            db,
            module_symbol_context,
            token_sheet_data: db.token_sheet_data(path)?,
            ast_sheet: db.ast_sheet(path)?,
            module_entity_tree: db.entity_tree_sheet(path)?,
            entity_tree_crate_bundle: db.entity_tree_crate_bundle(path.crate_path(db))?,
        })
    }

    // MOM
    fn parse_ty_decl(&self, path: TypePath) -> DeclResult<TypeDecl> {
        let ident = path.ident(self.db);
        let Some(entity_symbol) = self
            .module_entity_tree
            .module_symbols()
            .resolve_ident(ident)
            else {
                use salsa::DisplayWithDb;
                panic!(r#"
    Path `{}` is invalid!
    This is very likely caused by expect item in standard library.
"#, path.display(self.db))
            };
        let module_item_symbol = entity_symbol.module_item_symbol().unwrap();

        let ast_idx: AstIdx = module_item_symbol.ast_idx(self.db);
        match self.ast_sheet[ast_idx] {
            Ast::Defn {
                token_group_idx,
                ref body,

                entity_kind,

                saved_stream_state,
                ..
            } => self.parse_ty_decl_aux(
                ast_idx,
                path.ty_kind(self.db),
                path,
                entity_kind,
                token_group_idx,
                body,
                saved_stream_state,
            ),
            _ => unreachable!(),
        }
    }
    fn parse_ty_decl_aux(
        &self,
        ast_idx: AstIdx,
        type_kind: TypeKind,
        path: TypePath,
        _entity_kind: EntityKind,
        token_group_idx: TokenGroupIdx,
        body: &AstIdxRange,
        saved_stream_state: TokenIdx,
    ) -> DeclResult<TypeDecl> {
        match type_kind {
            TypeKind::Enum => {
                self.parse_enum_ty_decl(ast_idx, path, token_group_idx, body, saved_stream_state)
            }
            TypeKind::Inductive => self.parse_inductive_ty_decl(
                ast_idx,
                path,
                token_group_idx,
                body,
                saved_stream_state,
            ),
            TypeKind::Record => todo!(),
            TypeKind::Struct => {
                self.parse_struct_ty_decl(ast_idx, path, token_group_idx, body, saved_stream_state)
            }
            TypeKind::Structure => self.parse_structure_ty_decl(
                ast_idx,
                path,
                token_group_idx,
                body,
                saved_stream_state,
            ),
            TypeKind::Extern => {
                self.parse_foreign_ty_decl(ast_idx, path, token_group_idx, body, saved_stream_state)
            }
        }
    }

    fn parse_enum_ty_decl(
        &self,
        ast_idx: AstIdx,
        path: TypePath,
        token_group_idx: TokenGroupIdx,
        _body: &AstIdxRange,
        saved_stream_state: TokenIdx,
    ) -> DeclResult<TypeDecl> {
        let mut parser = self.expr_parser(
            DeclRegionPath::Entity(path.into()),
            None,
            AllowSelfType::True,
            AllowSelfValue::False,
        );
        let mut ctx = parser.ctx(
            None,
            self.token_sheet_data
                .token_group_token_stream(token_group_idx, Some(saved_stream_state)),
        );
        let implicit_parameters = ctx.parse();
        Ok(EnumTypeDecl::new(self.db, path, ast_idx, parser.finish(), implicit_parameters).into())
    }

    fn parse_trai_decl(&self, path: TraitPath) -> DeclResult<TraitDecl> {
        let ident = path.ident(self.db);
        let Some(entity_symbol) = self
            .module_entity_tree
            .module_symbols()
            .resolve_ident(ident)
            else {
                use salsa::DisplayWithDb;
                panic!(r#"
    Path `{}` is invalid!
    This is very likely caused by expect item in standard library.
"#, path.display(self.db))
            };
        let module_item = entity_symbol.module_item_symbol().unwrap();
        let ast_idx: AstIdx = module_item.ast_idx(self.db);
        match self.ast_sheet[ast_idx] {
            Ast::Defn {
                token_group_idx,
                ref body,

                saved_stream_state,
                ..
            } => self.parse_trai_decl_aux(ast_idx, path, token_group_idx, body, saved_stream_state),
            _ => unreachable!(),
        }
    }

    fn parse_trai_decl_aux(
        &self,
        ast_idx: AstIdx,
        path: TraitPath,
        token_group_idx: TokenGroupIdx,
        _body: &AstIdxRange,
        saved_stream_state: TokenIdx,
    ) -> DeclResult<TraitDecl> {
        let mut parser = self.expr_parser(
            DeclRegionPath::Entity(path.into()),
            None,
            AllowSelfType::True,
            AllowSelfValue::False,
        );
        let mut ctx = parser.ctx(
            None,
            self.token_sheet_data
                .token_group_token_stream(token_group_idx, Some(saved_stream_state)),
        );
        let implicit_parameters = ctx.parse();
        Ok(TraitDecl::new(
            self.db,
            path,
            ast_idx,
            parser.finish(),
            implicit_parameters,
        ))
    }

    fn parse_inductive_ty_decl(
        &self,
        ast_idx: AstIdx,
        path: TypePath,
        token_group_idx: TokenGroupIdx,
        _body: &AstIdxRange,
        saved_stream_state: TokenIdx,
    ) -> DeclResult<TypeDecl> {
        let mut parser = self.expr_parser(
            DeclRegionPath::Entity(path.into()),
            None,
            AllowSelfType::True,
            AllowSelfValue::False,
        );
        let mut ctx = parser.ctx(
            None,
            self.token_sheet_data
                .token_group_token_stream(token_group_idx, Some(saved_stream_state)),
        );
        let implicit_parameters = ctx.parse();
        Ok(
            InductiveTypeDecl::new(self.db, path, ast_idx, parser.finish(), implicit_parameters)
                .into(),
        )
    }

    fn parse_struct_ty_decl(
        &self,
        ast_idx: AstIdx,
        path: TypePath,
        token_group_idx: TokenGroupIdx,
        _body: &AstIdxRange,
        saved_stream_state: TokenIdx,
    ) -> DeclResult<TypeDecl> {
        let mut parser = self.expr_parser(
            DeclRegionPath::Entity(path.into()),
            None,
            AllowSelfType::True,
            AllowSelfValue::False,
        );
        let mut ctx = parser.ctx(
            None,
            self.token_sheet_data
                .token_group_token_stream(token_group_idx, Some(saved_stream_state)),
        );
        let implicit_parameters = ctx.parse();
        if let Some(lcurl) = ctx.parse::<LeftCurlyBraceToken>()? {
            let field_comma_list = parse_separated_list(&mut ctx);
            let rcurl = ctx.parse_expected(OriginalDeclExprError::ExpectRightCurlyBrace);
            Ok(RegularStructTypeDecl::new(
                self.db,
                path,
                ast_idx,
                parser.finish(),
                implicit_parameters,
                lcurl,
                field_comma_list,
                rcurl,
            )
            .into())
        } else if let Some(_lbox) = ctx.parse::<LeftBoxBracketToken>()? {
            todo!()
        } else {
            Err(OriginalDeclError::ExpectLCurlOrLParOrSemicolon(ctx.save_state()).into())
        }
    }

    pub(crate) fn expr_parser(
        &self,
        expr_path: DeclRegionPath,
        parent_expr_region: Option<ExprRegion>,
        allow_self_type: AllowSelfType,
        allow_self_value: AllowSelfValue,
    ) -> ExprParser<'a> {
        ExprParser::new(
            self.db,
            expr_path.into(),
            self.token_sheet_data,
            self.module_symbol_context,
            parent_expr_region,
            allow_self_type,
            allow_self_value,
        )
    }

    fn parse_structure_ty_decl(
        &self,
        ast_idx: AstIdx,
        path: TypePath,
        token_group_idx: TokenGroupIdx,
        _body: &AstIdxRange,
        saved_stream_state: TokenIdx,
    ) -> DeclResult<TypeDecl> {
        let _token_iter = self
            .token_sheet_data
            .token_group_token_stream(token_group_idx, Some(saved_stream_state));

        let mut parser = self.expr_parser(
            DeclRegionPath::Entity(path.into()),
            None,
            AllowSelfType::True,
            AllowSelfValue::False,
        );
        let mut ctx = parser.ctx(
            None,
            self.token_sheet_data
                .token_group_token_stream(token_group_idx, Some(saved_stream_state)),
        );
        let implicit_parameters = ctx.parse();
        Ok(
            StructureTypeDecl::new(self.db, path, ast_idx, parser.finish(), implicit_parameters)
                .into(),
        )
    }

    // get declaration from tokens
    fn parse_foreign_ty_decl(
        &self,
        ast_idx: AstIdx,
        path: TypePath,
        token_group_idx: TokenGroupIdx,
        _body: &AstIdxRange,
        saved_stream_state: TokenIdx,
    ) -> DeclResult<TypeDecl> {
        let _token_iter = self
            .token_sheet_data
            .token_group_token_stream(token_group_idx, Some(saved_stream_state));

        let mut parser = self.expr_parser(
            DeclRegionPath::Entity(path.into()),
            None,
            AllowSelfType::True,
            AllowSelfValue::False,
        );
        let mut ctx = parser.ctx(
            None,
            self.token_sheet_data
                .token_group_token_stream(token_group_idx, Some(saved_stream_state)),
        );
        let implicit_parameters = ctx.parse();
        Ok(
            ExternTypeDecl::new(self.db, path, ast_idx, parser.finish(), implicit_parameters)
                .into(),
        )
    }

    fn parse_form_decl(&self, path: FormPath) -> DeclResult<FormDecl> {
        let ident = path.ident(self.db);
        let module_item = self
            .module_entity_tree
            .module_symbols()
            .resolve_ident(ident)
            .unwrap()
            .module_item_symbol()
            .unwrap();
        let ast_idx: AstIdx = module_item.ast_idx(self.db);
        match self.ast_sheet[ast_idx] {
            Ast::Defn {
                token_group_idx,
                ref body,

                entity_kind,

                saved_stream_state,
                ..
            } => self.parse_form_decl_aux(
                ast_idx,
                path,
                entity_kind,
                token_group_idx,
                body,
                saved_stream_state,
            ),
            _ => unreachable!(),
        }
    }

    fn parse_form_decl_aux(
        &self,
        ast_idx: AstIdx,
        path: FormPath,
        _entity_kind: EntityKind,
        token_group_idx: TokenGroupIdx,
        _body: &AstIdxRange,
        saved_stream_state: TokenIdx,
    ) -> Result<FormDecl, DeclError> {
        match path.form_kind(self.db) {
            FormKind::Feature => {
                self.parse_feature_decl(ast_idx, token_group_idx, saved_stream_state, path)
            }
            FormKind::Function => {
                self.parse_function_decl(ast_idx, token_group_idx, saved_stream_state, path)
            }
            FormKind::Value => todo!(),
            FormKind::TypeAlias => todo!(),
        }
    }

    fn parse_feature_decl(
        &self,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: TokenIdx,
        path: FormPath,
    ) -> Result<FormDecl, DeclError> {
        let mut parser = self.expr_parser(
            DeclRegionPath::Entity(path.into()),
            None,
            AllowSelfType::False,
            AllowSelfValue::False,
        );
        let mut ctx = parser.ctx(
            None,
            self.token_sheet_data
                .token_group_token_stream(token_group_idx, Some(saved_stream_state)),
        );
        let curry_token = ctx.parse_expected(OriginalDeclExprError::ExpectCurry);
        let return_ty = ctx.parse_expected(OriginalDeclExprError::ExpectOutputType);
        let eol_colon = ctx.parse_expected(OriginalDeclExprError::ExpectEolColon);
        Ok(FeatureDecl::new(
            self.db,
            path,
            ast_idx,
            curry_token,
            return_ty,
            eol_colon,
            parser.finish(),
        )
        .into())
    }

    fn parse_function_decl(
        &self,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: TokenIdx,
        path: FormPath,
    ) -> Result<FormDecl, DeclError> {
        let mut parser = self.expr_parser(
            DeclRegionPath::Entity(path.into()),
            None,
            AllowSelfType::False,
            AllowSelfValue::False,
        );
        let mut ctx = parser.ctx(
            None,
            self.token_sheet_data
                .token_group_token_stream(token_group_idx, Some(saved_stream_state)),
        );
        let implicit_parameter_decl_list = ctx.parse();
        let parameter_decl_list =
            ctx.parse_expected(OriginalDeclExprError::ExpectParameterDeclList);
        let curry_token = ctx.parse_expected(OriginalDeclExprError::ExpectCurry);
        let return_ty = ctx.parse_expected(OriginalDeclExprError::ExpectOutputType);
        let eol_colon = ctx.parse_expected(OriginalDeclExprError::ExpectEolColon);
        Ok(FunctionDecl::new(
            self.db,
            path,
            ast_idx,
            parser.finish(),
            implicit_parameter_decl_list,
            parameter_decl_list,
            curry_token,
            return_ty,
            eol_colon,
        )
        .into())
    }

    fn parse_associated_item_decl(
        &self,
        associated_item: AssociatedItem,
    ) -> DeclResult<AssociatedItemDecl> {
        let ast_idx = associated_item.ast_idx(self.db);
        Ok(match self.ast_sheet[ast_idx] {
            Ast::Defn {
                token_group_idx,
                body: _,
                accessibility: _,
                entity_kind:
                    EntityKind::AssociatedItem {
                        associated_item_kind,
                    },
                entity_path: _,
                ident_token: _,
                is_generic: _,
                body_kind: _,
                saved_stream_state,
            } => match associated_item_kind {
                AssociatedItemKind::TraitItem(_) => todo!(),
                AssociatedItemKind::TypeItem(ty_item_kind) => {
                    AssociatedItemDecl::TypeItem(match ty_item_kind {
                        TypeItemKind::Method => self
                            .parse_ty_method_decl(
                                ast_idx,
                                token_group_idx,
                                associated_item,
                                saved_stream_state,
                            )?
                            .into(),
                        TypeItemKind::AssociatedFunction => todo!(),
                        TypeItemKind::Memo => self
                            .parse_ty_memo_decl(
                                ast_idx,
                                token_group_idx,
                                associated_item,
                                saved_stream_state,
                            )?
                            .into(),
                    })
                }
                AssociatedItemKind::TypeAsTraitItem(ty_as_trai_item_kind) => {
                    AssociatedItemDecl::TypeAsTraitItem(match ty_as_trai_item_kind {
                        TraitItemKind::Method => self
                            .parse_ty_as_trai_method_decl(
                                ast_idx,
                                token_group_idx,
                                associated_item,
                                saved_stream_state,
                            )?
                            .into(),
                        TraitItemKind::AssociatedType => todo!(),
                    })
                }
            },
            _ => unreachable!(),
        })
    }

    fn parse_ty_method_decl(
        &self,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        associated_item: AssociatedItem,
        saved_stream_state: TokenIdx,
    ) -> DeclResult<TypeMethodDecl> {
        let Ok(impl_decl) = self.db.impl_decl(associated_item.impl_block(self.db))
            else { return Err(DerivedDeclError::UnableToParseImplDeclForTyMethodDecl.into()) };
        let mut parser = self.expr_parser(
            DeclRegionPath::AssociatedItem(associated_item.id(self.db)),
            Some(impl_decl.expr_region(self.db)),
            AllowSelfType::True,
            AllowSelfValue::True,
        );
        let mut ctx = parser.ctx(
            None,
            self.token_sheet_data
                .token_group_token_stream(token_group_idx, saved_stream_state),
        );
        let implicit_parameter_decl_list = ctx.parse();
        let path = match associated_item.path(self.db) {
            Some(AssociatedItemPath::TypeItem(path)) => Some(path),
            None => None,
            _ => unreachable!(),
        };
        let parameter_decl_list =
            ctx.parse_expected(OriginalDeclExprError::ExpectParameterDeclList);
        let curry_token = ctx.parse_expected(OriginalDeclExprError::ExpectCurry);
        let return_ty = ctx.parse_expected(OriginalDeclExprError::ExpectOutputType);
        let eol_colon = ctx.parse_expected(OriginalDeclExprError::ExpectEolColon);
        Ok(TypeMethodDecl::new(
            self.db,
            associated_item,
            path,
            ast_idx,
            parser.finish(),
            implicit_parameter_decl_list,
            parameter_decl_list,
            curry_token,
            return_ty,
            eol_colon,
        )
        .into())
    }

    fn parse_ty_memo_decl(
        &self,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        associated_item: AssociatedItem,
        saved_stream_state: TokenIdx,
    ) -> DeclResult<TypeMemoDecl> {
        let Ok(impl_decl) = self.db.impl_decl(associated_item.impl_block(self.db))
            else { todo!() };
        let mut parser = self.expr_parser(
            DeclRegionPath::AssociatedItem(associated_item.id(self.db)),
            Some(impl_decl.expr_region(self.db)),
            AllowSelfType::True,
            AllowSelfValue::True,
        );
        let mut ctx = parser.ctx(
            None,
            self.token_sheet_data
                .token_group_token_stream(token_group_idx, saved_stream_state),
        );
        let path = match associated_item.path(self.db) {
            Some(AssociatedItemPath::TypeItem(path)) => Some(path),
            None => None,
            _ => unreachable!(),
        };
        let curry_token = ctx.parse_expected(OriginalDeclExprError::ExpectCurry);
        let return_ty = ctx.parse_expected(OriginalDeclExprError::ExpectOutputType);
        let eol_colon = ctx.parse_expected(OriginalDeclExprError::ExpectEolColon);
        Ok(TypeMemoDecl::new(
            self.db,
            path,
            associated_item,
            ast_idx,
            parser.finish(),
            curry_token,
            return_ty,
            eol_colon,
        )
        .into())
    }

    fn parse_ty_as_trai_method_decl(
        &self,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        associated_item: AssociatedItem,
        saved_stream_state: TokenIdx,
    ) -> DeclResult<TypeAsTraitMethodDecl> {
        let Ok(impl_decl) = self.db.impl_decl(associated_item.impl_block(self.db))
            else {
                return Err(
                    DerivedDeclError::UnableToParseImplDeclForTyAsTraitMethodDecl.into()
                )
            };
        let mut parser = self.expr_parser(
            DeclRegionPath::AssociatedItem(associated_item.id(self.db)),
            Some(impl_decl.expr_region(self.db)),
            AllowSelfType::True,
            AllowSelfValue::True,
        );
        let mut ctx = parser.ctx(
            None,
            self.token_sheet_data
                .token_group_token_stream(token_group_idx, saved_stream_state),
        );
        let implicit_parameter_decl_list = ctx.parse();
        let path = match associated_item.path(self.db) {
            Some(AssociatedItemPath::TypeAsTraitItem(path)) => Some(path),
            None => None,
            _ => unreachable!(),
        };
        let parameter_decl_list =
            ctx.parse_expected(OriginalDeclExprError::ExpectParameterDeclList);
        let curry_token = ctx.parse_expected(OriginalDeclExprError::ExpectCurry);
        let return_ty = ctx.parse_expected(OriginalDeclExprError::ExpectOutputType);
        let eol_colon = ctx.parse_expected(OriginalDeclExprError::ExpectEolColon);
        Ok(TypeAsTraitMethodDecl::new(
            self.db,
            path,
            associated_item,
            ast_idx,
            parser.finish(),
            implicit_parameter_decl_list,
            parameter_decl_list,
            curry_token,
            return_ty,
            eol_colon,
        )
        .into())
    }

    pub(crate) fn db(&self) -> &'a dyn DeclDb {
        self.db
    }

    pub(crate) fn token_sheet_data(&self) -> &'a TokenSheetData {
        self.token_sheet_data
    }

    pub(crate) fn ast_sheet(&self) -> &'a AstSheet {
        self.ast_sheet
    }
}

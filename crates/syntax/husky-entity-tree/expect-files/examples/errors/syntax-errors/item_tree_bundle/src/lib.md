```rust
EntityTreeCrateBundle {
    sheets: [
        EntityTreeSheet {
            module_path: ModulePath(`syntax_errors`),
            major_item_node_table: MajorEntityNodeTable {
                entries: [
                    ItemNodeEntry {
                        node: ItemSynNode::Submodule(
                            SubmoduleSynNode {
                                syn_node_path: SubmoduleSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::Submodule(
                                            SubmoduleSynNodePathData {
                                                disambiguated_item_path: DisambiguatedItemPath {
                                                    maybe_ambiguous_item_path: SubmoduleItemPath(`syntax_errors::ast),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    },
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`syntax_errors`),
                                ),
                                ast_idx: 0,
                                ident_token: IdentToken {
                                    ident: `ast`,
                                    token_idx: TokenIdx(
                                        2,
                                    ),
                                },
                            },
                        ),
                        syn_node_path: ItemSynNodePath::Submodule(
                            Room32,
                            SubmoduleSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::Submodule(
                                        SubmoduleSynNodePathData {
                                            disambiguated_item_path: DisambiguatedItemPath {
                                                maybe_ambiguous_item_path: SubmoduleItemPath(`syntax_errors::ast),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                },
                            ),
                        ),
                        ident: `ast`,
                        visibility: Scope::PubUnder(
                            ModulePath(`syntax_errors`),
                        ),
                    },
                    ItemNodeEntry {
                        node: ItemSynNode::Submodule(
                            SubmoduleSynNode {
                                syn_node_path: SubmoduleSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::Submodule(
                                            SubmoduleSynNodePathData {
                                                disambiguated_item_path: DisambiguatedItemPath {
                                                    maybe_ambiguous_item_path: SubmoduleItemPath(`syntax_errors::uses),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    },
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`syntax_errors`),
                                ),
                                ast_idx: 1,
                                ident_token: IdentToken {
                                    ident: `uses`,
                                    token_idx: TokenIdx(
                                        4,
                                    ),
                                },
                            },
                        ),
                        syn_node_path: ItemSynNodePath::Submodule(
                            Room32,
                            SubmoduleSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::Submodule(
                                        SubmoduleSynNodePathData {
                                            disambiguated_item_path: DisambiguatedItemPath {
                                                maybe_ambiguous_item_path: SubmoduleItemPath(`syntax_errors::uses),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                },
                            ),
                        ),
                        ident: `uses`,
                        visibility: Scope::PubUnder(
                            ModulePath(`syntax_errors`),
                        ),
                    },
                ],
            },
            item_symbol_table: EntitySymbolTable(
                [
                    EntitySymbolEntry {
                        ident: `ast`,
                        visible_scope: Scope::PubUnder(
                            ModulePath(`syntax_errors`),
                        ),
                        symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`syntax_errors::ast),
                        },
                    },
                    EntitySymbolEntry {
                        ident: `uses`,
                        visible_scope: Scope::PubUnder(
                            ModulePath(`syntax_errors`),
                        ),
                        symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`syntax_errors::uses),
                        },
                    },
                ],
            ),
            impl_block_syn_node_table: [],
            once_use_rules: OnceUseRules(
                [],
            ),
            use_all_rules: UseAllRules(
                [],
            ),
            errors: [],
        },
        EntityTreeSheet {
            module_path: ModulePath(`syntax_errors::ast`),
            major_item_node_table: MajorEntityNodeTable {
                entries: [
                    ItemNodeEntry {
                        node: ItemSynNode::MajorItem(
                            MajorItemSynNode {
                                syn_node_path: MajorItemSynNodePath::Type(
                                    TypeSynNodePath(
                                        ItemSynNodePathId {
                                            data: ItemSynNodePathData::MajorItem(
                                                MajorItemSynNodePathData::Type(
                                                    TypeSynNodePathData {
                                                        disambiguated_item_path: DisambiguatedItemPath {
                                                            maybe_ambiguous_item_path: TypePath(`syntax_errors::ast::A`, `Struct`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`syntax_errors::ast`),
                                ),
                                ast_idx: 2,
                                ident_token: IdentToken {
                                    ident: `A`,
                                    token_idx: TokenIdx(
                                        4,
                                    ),
                                },
                                block: DefnBlock::Type {
                                    path: TypePath(`syntax_errors::ast::A`, `Struct`),
                                    variants: None,
                                },
                            },
                        ),
                        syn_node_path: ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Type(
                                TypeSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::MajorItem(
                                            MajorItemSynNodePathData::Type(
                                                TypeSynNodePathData {
                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                        maybe_ambiguous_item_path: TypePath(`syntax_errors::ast::A`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        ),
                        ident: `A`,
                        visibility: Scope::PubUnder(
                            ModulePath(`syntax_errors::ast`),
                        ),
                    },
                ],
            },
            item_symbol_table: EntitySymbolTable(
                [
                    EntitySymbolEntry {
                        ident: `A`,
                        visible_scope: Scope::PubUnder(
                            ModulePath(`syntax_errors::ast`),
                        ),
                        symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Type(
                                TypePath(`syntax_errors::ast::A`, `Struct`),
                            ),
                        },
                    },
                ],
            ),
            impl_block_syn_node_table: [
                (
                    ImplBlockSynNodePath::TypeImplBlock(
                        TypeImplBlockSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::ImplBlock(
                                    ImplBlockSynNodePathData::TypeImplBlock(
                                        TypeImplBlockSynNodePathData {
                                            path: TypeImplBlockPath(`syntax_errors::ast::A(0)`),
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                    ImplBlockSynNode::TypeImplBlock(
                        TypeImplBlockSynNode {
                            syn_node_path: TypeImplBlockSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::ImplBlock(
                                        ImplBlockSynNodePathData::TypeImplBlock(
                                            TypeImplBlockSynNodePathData {
                                                path: TypeImplBlockPath(`syntax_errors::ast::A(0)`),
                                            },
                                        ),
                                    ),
                                },
                            ),
                            ast_idx: 3,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    7,
                                ),
                            },
                            ty_expr: 0,
                            items: TypeItems {
                                ast_idx_range: ArenaIdxRange(
                                    0..1,
                                ),
                            },
                        },
                    ),
                ),
            ],
            once_use_rules: OnceUseRules(
                [],
            ),
            use_all_rules: UseAllRules(
                [],
            ),
            errors: [],
        },
        EntityTreeSheet {
            module_path: ModulePath(`syntax_errors::uses`),
            major_item_node_table: MajorEntityNodeTable {
                entries: [],
            },
            item_symbol_table: EntitySymbolTable(
                [],
            ),
            impl_block_syn_node_table: [],
            once_use_rules: OnceUseRules(
                [],
            ),
            use_all_rules: UseAllRules(
                [],
            ),
            errors: [],
        },
    ],
    principal_item_path_expr_arena: Arena {
        data: [
            MajorItemPathExpr::Root {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `A`,
                        token_idx: TokenIdx(
                            8,
                        ),
                    },
                ),
                major_path: MajorEntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`syntax_errors::ast::A`, `Struct`),
                    ),
                ),
            },
        ],
    },
}
```
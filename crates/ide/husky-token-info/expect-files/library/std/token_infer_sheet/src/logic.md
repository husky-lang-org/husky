```rust
Ok(
    TokenInfoSheet {
        token_infos: [
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::UseExpr(
                        2,
                    ),
                    data: TokenInfoData::UseExpr {
                        use_expr_idx: 2,
                        rule_idx: OnceUseRuleIdx(
                            0,
                        ),
                        state: UseOneRuleState::Resolved {
                            original_symbol: Some(
                                EntitySymbol::UniversalPrelude {
                                    item_path: PrincipalEntityPath::Module(
                                        ModulePath(`core`),
                                    ),
                                },
                            ),
                        },
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::UseExpr(
                        1,
                    ),
                    data: TokenInfoData::UseExpr {
                        use_expr_idx: 1,
                        rule_idx: OnceUseRuleIdx(
                            1,
                        ),
                        state: UseOneRuleState::Resolved {
                            original_symbol: Some(
                                EntitySymbol::PackageDependencyOrSelfLib {
                                    item_path: PrincipalEntityPath::Module(
                                        ModulePath(`core::logic`),
                                    ),
                                },
                            ),
                        },
                    },
                },
            ),
            None,
            None,
        ],
    },
)
```
```rust
SynNodeDeclSheet {
    decls: [
        (
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Trait(
                    TraitSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Trait(
                                    TraitSynNodePathData {
                                        disambiguated_item_path: DisambiguatedItemPath {
                                            maybe_ambiguous_item_path: TraitPath(`core::ops::Add`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Trait(
                    TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::MajorItem(
                                    MajorItemSynNodePathData::Trait(
                                        TraitSynNodePathData {
                                            disambiguated_item_path: DisambiguatedItemPath {
                                                maybe_ambiguous_item_path: TraitPath(`core::ops::Add`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                        template_parameter_decl_list: Ok(
                            Some(
                                SynTemplateParameterSyndicateList {
                                    langle: LaOrLtRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateSynParameterData {
                                            annotated_variance_token: None,
                                            symbol: 0,
                                            variant: TemplateParameterSyndicateVariant::Type {
                                                ident_token: IdentRegionalToken {
                                                    ident: `Rhs`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                },
                                                traits: None,
                                            },
                                        },
                                    ],
                                    commas: [],
                                    decl_list_result: Ok(
                                        (),
                                    ),
                                    rangle: RaOrGtRegionalToken(
                                        RegionalTokenIdx(
                                            6,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: SynNodeRegionPath::ItemDecl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Trait(
                                            TraitSynNodePath(
                                                ItemSynNodePathId {
                                                    data: ItemSynNodePathData::MajorItem(
                                                        MajorItemSynNodePathData::Trait(
                                                            TraitSynNodePathData {
                                                                disambiguated_item_path: DisambiguatedItemPath {
                                                                    maybe_ambiguous_item_path: TraitPath(`core::ops::Add`),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [],
                                },
                                pattern_region: SynPatternRegion {
                                    pattern_arena: Arena {
                                        data: [],
                                    },
                                    pattern_contracts: [],
                                    pattern_variable_arena: Arena {
                                        data: [],
                                    },
                                    pattern_variable_maps: [],
                                    pattern_variable_modifiers: ArenaMap {
                                        data: [],
                                    },
                                },
                                variable_region: VariableRegionData {
                                    inherited_variable_arena: Arena {
                                        data: [],
                                    },
                                    current_variable_arena: Arena {
                                        data: [
                                            CurrentVariableEntry {
                                                modifier: Compterm,
                                                access_start: RegionalTokenIdx(
                                                    6,
                                                ),
                                                access_end: None,
                                                data: CurrentVariableData::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    data: CurrentTemplateVariableData::Type {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `Rhs`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                5,
                                                            ),
                                                        },
                                                        trai_syn_expr_idxs: [],
                                                    },
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [
                                        (
                                            TemplateTypeParameter,
                                            ArenaIdxRange(
                                                0..1,
                                            ),
                                        ),
                                    ],
                                },
                                pattern_roots: [],
                                expr_roots: [],
                                has_self_lifetime: false,
                                has_self_place: false,
                                pattern_to_current_variable_map: [],
                            },
                        },
                    },
                ),
            ),
        ),
        (
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Trait(
                    TraitSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Trait(
                                    TraitSynNodePathData {
                                        disambiguated_item_path: DisambiguatedItemPath {
                                            maybe_ambiguous_item_path: TraitPath(`core::ops::AddAssign`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Trait(
                    TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::MajorItem(
                                    MajorItemSynNodePathData::Trait(
                                        TraitSynNodePathData {
                                            disambiguated_item_path: DisambiguatedItemPath {
                                                maybe_ambiguous_item_path: TraitPath(`core::ops::AddAssign`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                        template_parameter_decl_list: Ok(
                            Some(
                                SynTemplateParameterSyndicateList {
                                    langle: LaOrLtRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateSynParameterData {
                                            annotated_variance_token: None,
                                            symbol: 0,
                                            variant: TemplateParameterSyndicateVariant::Type {
                                                ident_token: IdentRegionalToken {
                                                    ident: `Rhs`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                },
                                                traits: None,
                                            },
                                        },
                                    ],
                                    commas: [],
                                    decl_list_result: Ok(
                                        (),
                                    ),
                                    rangle: RaOrGtRegionalToken(
                                        RegionalTokenIdx(
                                            6,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: SynNodeRegionPath::ItemDecl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Trait(
                                            TraitSynNodePath(
                                                ItemSynNodePathId {
                                                    data: ItemSynNodePathData::MajorItem(
                                                        MajorItemSynNodePathData::Trait(
                                                            TraitSynNodePathData {
                                                                disambiguated_item_path: DisambiguatedItemPath {
                                                                    maybe_ambiguous_item_path: TraitPath(`core::ops::AddAssign`),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [],
                                },
                                pattern_region: SynPatternRegion {
                                    pattern_arena: Arena {
                                        data: [],
                                    },
                                    pattern_contracts: [],
                                    pattern_variable_arena: Arena {
                                        data: [],
                                    },
                                    pattern_variable_maps: [],
                                    pattern_variable_modifiers: ArenaMap {
                                        data: [],
                                    },
                                },
                                variable_region: VariableRegionData {
                                    inherited_variable_arena: Arena {
                                        data: [],
                                    },
                                    current_variable_arena: Arena {
                                        data: [
                                            CurrentVariableEntry {
                                                modifier: Compterm,
                                                access_start: RegionalTokenIdx(
                                                    6,
                                                ),
                                                access_end: None,
                                                data: CurrentVariableData::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    data: CurrentTemplateVariableData::Type {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `Rhs`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                5,
                                                            ),
                                                        },
                                                        trai_syn_expr_idxs: [],
                                                    },
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [
                                        (
                                            TemplateTypeParameter,
                                            ArenaIdxRange(
                                                0..1,
                                            ),
                                        ),
                                    ],
                                },
                                pattern_roots: [],
                                expr_roots: [],
                                has_self_lifetime: false,
                                has_self_place: false,
                                pattern_to_current_variable_map: [],
                            },
                        },
                    },
                ),
            ),
        ),
        (
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Trait(
                    TraitSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Trait(
                                    TraitSynNodePathData {
                                        disambiguated_item_path: DisambiguatedItemPath {
                                            maybe_ambiguous_item_path: TraitPath(`core::ops::BitAnd`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Trait(
                    TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::MajorItem(
                                    MajorItemSynNodePathData::Trait(
                                        TraitSynNodePathData {
                                            disambiguated_item_path: DisambiguatedItemPath {
                                                maybe_ambiguous_item_path: TraitPath(`core::ops::BitAnd`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                        template_parameter_decl_list: Ok(
                            Some(
                                SynTemplateParameterSyndicateList {
                                    langle: LaOrLtRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateSynParameterData {
                                            annotated_variance_token: None,
                                            symbol: 0,
                                            variant: TemplateParameterSyndicateVariant::Type {
                                                ident_token: IdentRegionalToken {
                                                    ident: `Rhs`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                },
                                                traits: None,
                                            },
                                        },
                                    ],
                                    commas: [],
                                    decl_list_result: Ok(
                                        (),
                                    ),
                                    rangle: RaOrGtRegionalToken(
                                        RegionalTokenIdx(
                                            6,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: SynNodeRegionPath::ItemDecl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Trait(
                                            TraitSynNodePath(
                                                ItemSynNodePathId {
                                                    data: ItemSynNodePathData::MajorItem(
                                                        MajorItemSynNodePathData::Trait(
                                                            TraitSynNodePathData {
                                                                disambiguated_item_path: DisambiguatedItemPath {
                                                                    maybe_ambiguous_item_path: TraitPath(`core::ops::BitAnd`),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [],
                                },
                                pattern_region: SynPatternRegion {
                                    pattern_arena: Arena {
                                        data: [],
                                    },
                                    pattern_contracts: [],
                                    pattern_variable_arena: Arena {
                                        data: [],
                                    },
                                    pattern_variable_maps: [],
                                    pattern_variable_modifiers: ArenaMap {
                                        data: [],
                                    },
                                },
                                variable_region: VariableRegionData {
                                    inherited_variable_arena: Arena {
                                        data: [],
                                    },
                                    current_variable_arena: Arena {
                                        data: [
                                            CurrentVariableEntry {
                                                modifier: Compterm,
                                                access_start: RegionalTokenIdx(
                                                    6,
                                                ),
                                                access_end: None,
                                                data: CurrentVariableData::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    data: CurrentTemplateVariableData::Type {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `Rhs`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                5,
                                                            ),
                                                        },
                                                        trai_syn_expr_idxs: [],
                                                    },
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [
                                        (
                                            TemplateTypeParameter,
                                            ArenaIdxRange(
                                                0..1,
                                            ),
                                        ),
                                    ],
                                },
                                pattern_roots: [],
                                expr_roots: [],
                                has_self_lifetime: false,
                                has_self_place: false,
                                pattern_to_current_variable_map: [],
                            },
                        },
                    },
                ),
            ),
        ),
        (
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Trait(
                    TraitSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Trait(
                                    TraitSynNodePathData {
                                        disambiguated_item_path: DisambiguatedItemPath {
                                            maybe_ambiguous_item_path: TraitPath(`core::ops::BitAndAssign`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Trait(
                    TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::MajorItem(
                                    MajorItemSynNodePathData::Trait(
                                        TraitSynNodePathData {
                                            disambiguated_item_path: DisambiguatedItemPath {
                                                maybe_ambiguous_item_path: TraitPath(`core::ops::BitAndAssign`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                        template_parameter_decl_list: Ok(
                            Some(
                                SynTemplateParameterSyndicateList {
                                    langle: LaOrLtRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateSynParameterData {
                                            annotated_variance_token: None,
                                            symbol: 0,
                                            variant: TemplateParameterSyndicateVariant::Type {
                                                ident_token: IdentRegionalToken {
                                                    ident: `Rhs`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                },
                                                traits: None,
                                            },
                                        },
                                    ],
                                    commas: [],
                                    decl_list_result: Ok(
                                        (),
                                    ),
                                    rangle: RaOrGtRegionalToken(
                                        RegionalTokenIdx(
                                            6,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: SynNodeRegionPath::ItemDecl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Trait(
                                            TraitSynNodePath(
                                                ItemSynNodePathId {
                                                    data: ItemSynNodePathData::MajorItem(
                                                        MajorItemSynNodePathData::Trait(
                                                            TraitSynNodePathData {
                                                                disambiguated_item_path: DisambiguatedItemPath {
                                                                    maybe_ambiguous_item_path: TraitPath(`core::ops::BitAndAssign`),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [],
                                },
                                pattern_region: SynPatternRegion {
                                    pattern_arena: Arena {
                                        data: [],
                                    },
                                    pattern_contracts: [],
                                    pattern_variable_arena: Arena {
                                        data: [],
                                    },
                                    pattern_variable_maps: [],
                                    pattern_variable_modifiers: ArenaMap {
                                        data: [],
                                    },
                                },
                                variable_region: VariableRegionData {
                                    inherited_variable_arena: Arena {
                                        data: [],
                                    },
                                    current_variable_arena: Arena {
                                        data: [
                                            CurrentVariableEntry {
                                                modifier: Compterm,
                                                access_start: RegionalTokenIdx(
                                                    6,
                                                ),
                                                access_end: None,
                                                data: CurrentVariableData::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    data: CurrentTemplateVariableData::Type {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `Rhs`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                5,
                                                            ),
                                                        },
                                                        trai_syn_expr_idxs: [],
                                                    },
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [
                                        (
                                            TemplateTypeParameter,
                                            ArenaIdxRange(
                                                0..1,
                                            ),
                                        ),
                                    ],
                                },
                                pattern_roots: [],
                                expr_roots: [],
                                has_self_lifetime: false,
                                has_self_place: false,
                                pattern_to_current_variable_map: [],
                            },
                        },
                    },
                ),
            ),
        ),
        (
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Trait(
                    TraitSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Trait(
                                    TraitSynNodePathData {
                                        disambiguated_item_path: DisambiguatedItemPath {
                                            maybe_ambiguous_item_path: TraitPath(`core::ops::BitOr`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Trait(
                    TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::MajorItem(
                                    MajorItemSynNodePathData::Trait(
                                        TraitSynNodePathData {
                                            disambiguated_item_path: DisambiguatedItemPath {
                                                maybe_ambiguous_item_path: TraitPath(`core::ops::BitOr`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                        template_parameter_decl_list: Ok(
                            Some(
                                SynTemplateParameterSyndicateList {
                                    langle: LaOrLtRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateSynParameterData {
                                            annotated_variance_token: None,
                                            symbol: 0,
                                            variant: TemplateParameterSyndicateVariant::Type {
                                                ident_token: IdentRegionalToken {
                                                    ident: `Rhs`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                },
                                                traits: None,
                                            },
                                        },
                                    ],
                                    commas: [],
                                    decl_list_result: Ok(
                                        (),
                                    ),
                                    rangle: RaOrGtRegionalToken(
                                        RegionalTokenIdx(
                                            6,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: SynNodeRegionPath::ItemDecl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Trait(
                                            TraitSynNodePath(
                                                ItemSynNodePathId {
                                                    data: ItemSynNodePathData::MajorItem(
                                                        MajorItemSynNodePathData::Trait(
                                                            TraitSynNodePathData {
                                                                disambiguated_item_path: DisambiguatedItemPath {
                                                                    maybe_ambiguous_item_path: TraitPath(`core::ops::BitOr`),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [],
                                },
                                pattern_region: SynPatternRegion {
                                    pattern_arena: Arena {
                                        data: [],
                                    },
                                    pattern_contracts: [],
                                    pattern_variable_arena: Arena {
                                        data: [],
                                    },
                                    pattern_variable_maps: [],
                                    pattern_variable_modifiers: ArenaMap {
                                        data: [],
                                    },
                                },
                                variable_region: VariableRegionData {
                                    inherited_variable_arena: Arena {
                                        data: [],
                                    },
                                    current_variable_arena: Arena {
                                        data: [
                                            CurrentVariableEntry {
                                                modifier: Compterm,
                                                access_start: RegionalTokenIdx(
                                                    6,
                                                ),
                                                access_end: None,
                                                data: CurrentVariableData::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    data: CurrentTemplateVariableData::Type {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `Rhs`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                5,
                                                            ),
                                                        },
                                                        trai_syn_expr_idxs: [],
                                                    },
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [
                                        (
                                            TemplateTypeParameter,
                                            ArenaIdxRange(
                                                0..1,
                                            ),
                                        ),
                                    ],
                                },
                                pattern_roots: [],
                                expr_roots: [],
                                has_self_lifetime: false,
                                has_self_place: false,
                                pattern_to_current_variable_map: [],
                            },
                        },
                    },
                ),
            ),
        ),
        (
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Trait(
                    TraitSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Trait(
                                    TraitSynNodePathData {
                                        disambiguated_item_path: DisambiguatedItemPath {
                                            maybe_ambiguous_item_path: TraitPath(`core::ops::BitOrAssign`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Trait(
                    TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::MajorItem(
                                    MajorItemSynNodePathData::Trait(
                                        TraitSynNodePathData {
                                            disambiguated_item_path: DisambiguatedItemPath {
                                                maybe_ambiguous_item_path: TraitPath(`core::ops::BitOrAssign`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                        template_parameter_decl_list: Ok(
                            Some(
                                SynTemplateParameterSyndicateList {
                                    langle: LaOrLtRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateSynParameterData {
                                            annotated_variance_token: None,
                                            symbol: 0,
                                            variant: TemplateParameterSyndicateVariant::Type {
                                                ident_token: IdentRegionalToken {
                                                    ident: `Rhs`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                },
                                                traits: None,
                                            },
                                        },
                                    ],
                                    commas: [],
                                    decl_list_result: Ok(
                                        (),
                                    ),
                                    rangle: RaOrGtRegionalToken(
                                        RegionalTokenIdx(
                                            6,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: SynNodeRegionPath::ItemDecl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Trait(
                                            TraitSynNodePath(
                                                ItemSynNodePathId {
                                                    data: ItemSynNodePathData::MajorItem(
                                                        MajorItemSynNodePathData::Trait(
                                                            TraitSynNodePathData {
                                                                disambiguated_item_path: DisambiguatedItemPath {
                                                                    maybe_ambiguous_item_path: TraitPath(`core::ops::BitOrAssign`),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [],
                                },
                                pattern_region: SynPatternRegion {
                                    pattern_arena: Arena {
                                        data: [],
                                    },
                                    pattern_contracts: [],
                                    pattern_variable_arena: Arena {
                                        data: [],
                                    },
                                    pattern_variable_maps: [],
                                    pattern_variable_modifiers: ArenaMap {
                                        data: [],
                                    },
                                },
                                variable_region: VariableRegionData {
                                    inherited_variable_arena: Arena {
                                        data: [],
                                    },
                                    current_variable_arena: Arena {
                                        data: [
                                            CurrentVariableEntry {
                                                modifier: Compterm,
                                                access_start: RegionalTokenIdx(
                                                    6,
                                                ),
                                                access_end: None,
                                                data: CurrentVariableData::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    data: CurrentTemplateVariableData::Type {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `Rhs`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                5,
                                                            ),
                                                        },
                                                        trai_syn_expr_idxs: [],
                                                    },
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [
                                        (
                                            TemplateTypeParameter,
                                            ArenaIdxRange(
                                                0..1,
                                            ),
                                        ),
                                    ],
                                },
                                pattern_roots: [],
                                expr_roots: [],
                                has_self_lifetime: false,
                                has_self_place: false,
                                pattern_to_current_variable_map: [],
                            },
                        },
                    },
                ),
            ),
        ),
        (
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Trait(
                    TraitSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Trait(
                                    TraitSynNodePathData {
                                        disambiguated_item_path: DisambiguatedItemPath {
                                            maybe_ambiguous_item_path: TraitPath(`core::ops::BitXor`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Trait(
                    TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::MajorItem(
                                    MajorItemSynNodePathData::Trait(
                                        TraitSynNodePathData {
                                            disambiguated_item_path: DisambiguatedItemPath {
                                                maybe_ambiguous_item_path: TraitPath(`core::ops::BitXor`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                        template_parameter_decl_list: Ok(
                            Some(
                                SynTemplateParameterSyndicateList {
                                    langle: LaOrLtRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateSynParameterData {
                                            annotated_variance_token: None,
                                            symbol: 0,
                                            variant: TemplateParameterSyndicateVariant::Type {
                                                ident_token: IdentRegionalToken {
                                                    ident: `Rhs`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                },
                                                traits: None,
                                            },
                                        },
                                    ],
                                    commas: [],
                                    decl_list_result: Ok(
                                        (),
                                    ),
                                    rangle: RaOrGtRegionalToken(
                                        RegionalTokenIdx(
                                            6,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: SynNodeRegionPath::ItemDecl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Trait(
                                            TraitSynNodePath(
                                                ItemSynNodePathId {
                                                    data: ItemSynNodePathData::MajorItem(
                                                        MajorItemSynNodePathData::Trait(
                                                            TraitSynNodePathData {
                                                                disambiguated_item_path: DisambiguatedItemPath {
                                                                    maybe_ambiguous_item_path: TraitPath(`core::ops::BitXor`),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [],
                                },
                                pattern_region: SynPatternRegion {
                                    pattern_arena: Arena {
                                        data: [],
                                    },
                                    pattern_contracts: [],
                                    pattern_variable_arena: Arena {
                                        data: [],
                                    },
                                    pattern_variable_maps: [],
                                    pattern_variable_modifiers: ArenaMap {
                                        data: [],
                                    },
                                },
                                variable_region: VariableRegionData {
                                    inherited_variable_arena: Arena {
                                        data: [],
                                    },
                                    current_variable_arena: Arena {
                                        data: [
                                            CurrentVariableEntry {
                                                modifier: Compterm,
                                                access_start: RegionalTokenIdx(
                                                    6,
                                                ),
                                                access_end: None,
                                                data: CurrentVariableData::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    data: CurrentTemplateVariableData::Type {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `Rhs`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                5,
                                                            ),
                                                        },
                                                        trai_syn_expr_idxs: [],
                                                    },
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [
                                        (
                                            TemplateTypeParameter,
                                            ArenaIdxRange(
                                                0..1,
                                            ),
                                        ),
                                    ],
                                },
                                pattern_roots: [],
                                expr_roots: [],
                                has_self_lifetime: false,
                                has_self_place: false,
                                pattern_to_current_variable_map: [],
                            },
                        },
                    },
                ),
            ),
        ),
        (
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Trait(
                    TraitSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Trait(
                                    TraitSynNodePathData {
                                        disambiguated_item_path: DisambiguatedItemPath {
                                            maybe_ambiguous_item_path: TraitPath(`core::ops::BitXorAssign`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Trait(
                    TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::MajorItem(
                                    MajorItemSynNodePathData::Trait(
                                        TraitSynNodePathData {
                                            disambiguated_item_path: DisambiguatedItemPath {
                                                maybe_ambiguous_item_path: TraitPath(`core::ops::BitXorAssign`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                        template_parameter_decl_list: Ok(
                            Some(
                                SynTemplateParameterSyndicateList {
                                    langle: LaOrLtRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateSynParameterData {
                                            annotated_variance_token: None,
                                            symbol: 0,
                                            variant: TemplateParameterSyndicateVariant::Type {
                                                ident_token: IdentRegionalToken {
                                                    ident: `Rhs`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                },
                                                traits: None,
                                            },
                                        },
                                    ],
                                    commas: [],
                                    decl_list_result: Ok(
                                        (),
                                    ),
                                    rangle: RaOrGtRegionalToken(
                                        RegionalTokenIdx(
                                            6,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: SynNodeRegionPath::ItemDecl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Trait(
                                            TraitSynNodePath(
                                                ItemSynNodePathId {
                                                    data: ItemSynNodePathData::MajorItem(
                                                        MajorItemSynNodePathData::Trait(
                                                            TraitSynNodePathData {
                                                                disambiguated_item_path: DisambiguatedItemPath {
                                                                    maybe_ambiguous_item_path: TraitPath(`core::ops::BitXorAssign`),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [],
                                },
                                pattern_region: SynPatternRegion {
                                    pattern_arena: Arena {
                                        data: [],
                                    },
                                    pattern_contracts: [],
                                    pattern_variable_arena: Arena {
                                        data: [],
                                    },
                                    pattern_variable_maps: [],
                                    pattern_variable_modifiers: ArenaMap {
                                        data: [],
                                    },
                                },
                                variable_region: VariableRegionData {
                                    inherited_variable_arena: Arena {
                                        data: [],
                                    },
                                    current_variable_arena: Arena {
                                        data: [
                                            CurrentVariableEntry {
                                                modifier: Compterm,
                                                access_start: RegionalTokenIdx(
                                                    6,
                                                ),
                                                access_end: None,
                                                data: CurrentVariableData::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    data: CurrentTemplateVariableData::Type {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `Rhs`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                5,
                                                            ),
                                                        },
                                                        trai_syn_expr_idxs: [],
                                                    },
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [
                                        (
                                            TemplateTypeParameter,
                                            ArenaIdxRange(
                                                0..1,
                                            ),
                                        ),
                                    ],
                                },
                                pattern_roots: [],
                                expr_roots: [],
                                has_self_lifetime: false,
                                has_self_place: false,
                                pattern_to_current_variable_map: [],
                            },
                        },
                    },
                ),
            ),
        ),
        (
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Trait(
                    TraitSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Trait(
                                    TraitSynNodePathData {
                                        disambiguated_item_path: DisambiguatedItemPath {
                                            maybe_ambiguous_item_path: TraitPath(`core::ops::Div`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Trait(
                    TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::MajorItem(
                                    MajorItemSynNodePathData::Trait(
                                        TraitSynNodePathData {
                                            disambiguated_item_path: DisambiguatedItemPath {
                                                maybe_ambiguous_item_path: TraitPath(`core::ops::Div`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                        template_parameter_decl_list: Ok(
                            Some(
                                SynTemplateParameterSyndicateList {
                                    langle: LaOrLtRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateSynParameterData {
                                            annotated_variance_token: None,
                                            symbol: 0,
                                            variant: TemplateParameterSyndicateVariant::Type {
                                                ident_token: IdentRegionalToken {
                                                    ident: `Rhs`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                },
                                                traits: None,
                                            },
                                        },
                                    ],
                                    commas: [],
                                    decl_list_result: Ok(
                                        (),
                                    ),
                                    rangle: RaOrGtRegionalToken(
                                        RegionalTokenIdx(
                                            6,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: SynNodeRegionPath::ItemDecl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Trait(
                                            TraitSynNodePath(
                                                ItemSynNodePathId {
                                                    data: ItemSynNodePathData::MajorItem(
                                                        MajorItemSynNodePathData::Trait(
                                                            TraitSynNodePathData {
                                                                disambiguated_item_path: DisambiguatedItemPath {
                                                                    maybe_ambiguous_item_path: TraitPath(`core::ops::Div`),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [],
                                },
                                pattern_region: SynPatternRegion {
                                    pattern_arena: Arena {
                                        data: [],
                                    },
                                    pattern_contracts: [],
                                    pattern_variable_arena: Arena {
                                        data: [],
                                    },
                                    pattern_variable_maps: [],
                                    pattern_variable_modifiers: ArenaMap {
                                        data: [],
                                    },
                                },
                                variable_region: VariableRegionData {
                                    inherited_variable_arena: Arena {
                                        data: [],
                                    },
                                    current_variable_arena: Arena {
                                        data: [
                                            CurrentVariableEntry {
                                                modifier: Compterm,
                                                access_start: RegionalTokenIdx(
                                                    6,
                                                ),
                                                access_end: None,
                                                data: CurrentVariableData::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    data: CurrentTemplateVariableData::Type {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `Rhs`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                5,
                                                            ),
                                                        },
                                                        trai_syn_expr_idxs: [],
                                                    },
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [
                                        (
                                            TemplateTypeParameter,
                                            ArenaIdxRange(
                                                0..1,
                                            ),
                                        ),
                                    ],
                                },
                                pattern_roots: [],
                                expr_roots: [],
                                has_self_lifetime: false,
                                has_self_place: false,
                                pattern_to_current_variable_map: [],
                            },
                        },
                    },
                ),
            ),
        ),
        (
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Trait(
                    TraitSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Trait(
                                    TraitSynNodePathData {
                                        disambiguated_item_path: DisambiguatedItemPath {
                                            maybe_ambiguous_item_path: TraitPath(`core::ops::DivAssign`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Trait(
                    TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::MajorItem(
                                    MajorItemSynNodePathData::Trait(
                                        TraitSynNodePathData {
                                            disambiguated_item_path: DisambiguatedItemPath {
                                                maybe_ambiguous_item_path: TraitPath(`core::ops::DivAssign`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                        template_parameter_decl_list: Ok(
                            Some(
                                SynTemplateParameterSyndicateList {
                                    langle: LaOrLtRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateSynParameterData {
                                            annotated_variance_token: None,
                                            symbol: 0,
                                            variant: TemplateParameterSyndicateVariant::Type {
                                                ident_token: IdentRegionalToken {
                                                    ident: `Rhs`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                },
                                                traits: None,
                                            },
                                        },
                                    ],
                                    commas: [],
                                    decl_list_result: Ok(
                                        (),
                                    ),
                                    rangle: RaOrGtRegionalToken(
                                        RegionalTokenIdx(
                                            6,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: SynNodeRegionPath::ItemDecl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Trait(
                                            TraitSynNodePath(
                                                ItemSynNodePathId {
                                                    data: ItemSynNodePathData::MajorItem(
                                                        MajorItemSynNodePathData::Trait(
                                                            TraitSynNodePathData {
                                                                disambiguated_item_path: DisambiguatedItemPath {
                                                                    maybe_ambiguous_item_path: TraitPath(`core::ops::DivAssign`),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [],
                                },
                                pattern_region: SynPatternRegion {
                                    pattern_arena: Arena {
                                        data: [],
                                    },
                                    pattern_contracts: [],
                                    pattern_variable_arena: Arena {
                                        data: [],
                                    },
                                    pattern_variable_maps: [],
                                    pattern_variable_modifiers: ArenaMap {
                                        data: [],
                                    },
                                },
                                variable_region: VariableRegionData {
                                    inherited_variable_arena: Arena {
                                        data: [],
                                    },
                                    current_variable_arena: Arena {
                                        data: [
                                            CurrentVariableEntry {
                                                modifier: Compterm,
                                                access_start: RegionalTokenIdx(
                                                    6,
                                                ),
                                                access_end: None,
                                                data: CurrentVariableData::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    data: CurrentTemplateVariableData::Type {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `Rhs`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                5,
                                                            ),
                                                        },
                                                        trai_syn_expr_idxs: [],
                                                    },
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [
                                        (
                                            TemplateTypeParameter,
                                            ArenaIdxRange(
                                                0..1,
                                            ),
                                        ),
                                    ],
                                },
                                pattern_roots: [],
                                expr_roots: [],
                                has_self_lifetime: false,
                                has_self_place: false,
                                pattern_to_current_variable_map: [],
                            },
                        },
                    },
                ),
            ),
        ),
        (
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Trait(
                    TraitSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Trait(
                                    TraitSynNodePathData {
                                        disambiguated_item_path: DisambiguatedItemPath {
                                            maybe_ambiguous_item_path: TraitPath(`core::ops::IntIndex`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Trait(
                    TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::MajorItem(
                                    MajorItemSynNodePathData::Trait(
                                        TraitSynNodePathData {
                                            disambiguated_item_path: DisambiguatedItemPath {
                                                maybe_ambiguous_item_path: TraitPath(`core::ops::IntIndex`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                        template_parameter_decl_list: Ok(
                            None,
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: SynNodeRegionPath::ItemDecl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Trait(
                                            TraitSynNodePath(
                                                ItemSynNodePathId {
                                                    data: ItemSynNodePathData::MajorItem(
                                                        MajorItemSynNodePathData::Trait(
                                                            TraitSynNodePathData {
                                                                disambiguated_item_path: DisambiguatedItemPath {
                                                                    maybe_ambiguous_item_path: TraitPath(`core::ops::IntIndex`),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [],
                                },
                                pattern_region: SynPatternRegion {
                                    pattern_arena: Arena {
                                        data: [],
                                    },
                                    pattern_contracts: [],
                                    pattern_variable_arena: Arena {
                                        data: [],
                                    },
                                    pattern_variable_maps: [],
                                    pattern_variable_modifiers: ArenaMap {
                                        data: [],
                                    },
                                },
                                variable_region: VariableRegionData {
                                    inherited_variable_arena: Arena {
                                        data: [],
                                    },
                                    current_variable_arena: Arena {
                                        data: [],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [],
                                },
                                pattern_roots: [],
                                expr_roots: [],
                                has_self_lifetime: false,
                                has_self_place: false,
                                pattern_to_current_variable_map: [],
                            },
                        },
                    },
                ),
            ),
        ),
        (
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Trait(
                    TraitSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Trait(
                                    TraitSynNodePathData {
                                        disambiguated_item_path: DisambiguatedItemPath {
                                            maybe_ambiguous_item_path: TraitPath(`core::ops::Mul`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Trait(
                    TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::MajorItem(
                                    MajorItemSynNodePathData::Trait(
                                        TraitSynNodePathData {
                                            disambiguated_item_path: DisambiguatedItemPath {
                                                maybe_ambiguous_item_path: TraitPath(`core::ops::Mul`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                        template_parameter_decl_list: Ok(
                            Some(
                                SynTemplateParameterSyndicateList {
                                    langle: LaOrLtRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateSynParameterData {
                                            annotated_variance_token: None,
                                            symbol: 0,
                                            variant: TemplateParameterSyndicateVariant::Type {
                                                ident_token: IdentRegionalToken {
                                                    ident: `Rhs`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                },
                                                traits: None,
                                            },
                                        },
                                    ],
                                    commas: [],
                                    decl_list_result: Ok(
                                        (),
                                    ),
                                    rangle: RaOrGtRegionalToken(
                                        RegionalTokenIdx(
                                            6,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: SynNodeRegionPath::ItemDecl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Trait(
                                            TraitSynNodePath(
                                                ItemSynNodePathId {
                                                    data: ItemSynNodePathData::MajorItem(
                                                        MajorItemSynNodePathData::Trait(
                                                            TraitSynNodePathData {
                                                                disambiguated_item_path: DisambiguatedItemPath {
                                                                    maybe_ambiguous_item_path: TraitPath(`core::ops::Mul`),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [],
                                },
                                pattern_region: SynPatternRegion {
                                    pattern_arena: Arena {
                                        data: [],
                                    },
                                    pattern_contracts: [],
                                    pattern_variable_arena: Arena {
                                        data: [],
                                    },
                                    pattern_variable_maps: [],
                                    pattern_variable_modifiers: ArenaMap {
                                        data: [],
                                    },
                                },
                                variable_region: VariableRegionData {
                                    inherited_variable_arena: Arena {
                                        data: [],
                                    },
                                    current_variable_arena: Arena {
                                        data: [
                                            CurrentVariableEntry {
                                                modifier: Compterm,
                                                access_start: RegionalTokenIdx(
                                                    6,
                                                ),
                                                access_end: None,
                                                data: CurrentVariableData::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    data: CurrentTemplateVariableData::Type {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `Rhs`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                5,
                                                            ),
                                                        },
                                                        trai_syn_expr_idxs: [],
                                                    },
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [
                                        (
                                            TemplateTypeParameter,
                                            ArenaIdxRange(
                                                0..1,
                                            ),
                                        ),
                                    ],
                                },
                                pattern_roots: [],
                                expr_roots: [],
                                has_self_lifetime: false,
                                has_self_place: false,
                                pattern_to_current_variable_map: [],
                            },
                        },
                    },
                ),
            ),
        ),
        (
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Trait(
                    TraitSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Trait(
                                    TraitSynNodePathData {
                                        disambiguated_item_path: DisambiguatedItemPath {
                                            maybe_ambiguous_item_path: TraitPath(`core::ops::MulAssign`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Trait(
                    TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::MajorItem(
                                    MajorItemSynNodePathData::Trait(
                                        TraitSynNodePathData {
                                            disambiguated_item_path: DisambiguatedItemPath {
                                                maybe_ambiguous_item_path: TraitPath(`core::ops::MulAssign`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                        template_parameter_decl_list: Ok(
                            Some(
                                SynTemplateParameterSyndicateList {
                                    langle: LaOrLtRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateSynParameterData {
                                            annotated_variance_token: None,
                                            symbol: 0,
                                            variant: TemplateParameterSyndicateVariant::Type {
                                                ident_token: IdentRegionalToken {
                                                    ident: `Rhs`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                },
                                                traits: None,
                                            },
                                        },
                                    ],
                                    commas: [],
                                    decl_list_result: Ok(
                                        (),
                                    ),
                                    rangle: RaOrGtRegionalToken(
                                        RegionalTokenIdx(
                                            6,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: SynNodeRegionPath::ItemDecl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Trait(
                                            TraitSynNodePath(
                                                ItemSynNodePathId {
                                                    data: ItemSynNodePathData::MajorItem(
                                                        MajorItemSynNodePathData::Trait(
                                                            TraitSynNodePathData {
                                                                disambiguated_item_path: DisambiguatedItemPath {
                                                                    maybe_ambiguous_item_path: TraitPath(`core::ops::MulAssign`),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [],
                                },
                                pattern_region: SynPatternRegion {
                                    pattern_arena: Arena {
                                        data: [],
                                    },
                                    pattern_contracts: [],
                                    pattern_variable_arena: Arena {
                                        data: [],
                                    },
                                    pattern_variable_maps: [],
                                    pattern_variable_modifiers: ArenaMap {
                                        data: [],
                                    },
                                },
                                variable_region: VariableRegionData {
                                    inherited_variable_arena: Arena {
                                        data: [],
                                    },
                                    current_variable_arena: Arena {
                                        data: [
                                            CurrentVariableEntry {
                                                modifier: Compterm,
                                                access_start: RegionalTokenIdx(
                                                    6,
                                                ),
                                                access_end: None,
                                                data: CurrentVariableData::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    data: CurrentTemplateVariableData::Type {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `Rhs`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                5,
                                                            ),
                                                        },
                                                        trai_syn_expr_idxs: [],
                                                    },
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [
                                        (
                                            TemplateTypeParameter,
                                            ArenaIdxRange(
                                                0..1,
                                            ),
                                        ),
                                    ],
                                },
                                pattern_roots: [],
                                expr_roots: [],
                                has_self_lifetime: false,
                                has_self_place: false,
                                pattern_to_current_variable_map: [],
                            },
                        },
                    },
                ),
            ),
        ),
        (
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Trait(
                    TraitSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Trait(
                                    TraitSynNodePathData {
                                        disambiguated_item_path: DisambiguatedItemPath {
                                            maybe_ambiguous_item_path: TraitPath(`core::ops::Neg`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Trait(
                    TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::MajorItem(
                                    MajorItemSynNodePathData::Trait(
                                        TraitSynNodePathData {
                                            disambiguated_item_path: DisambiguatedItemPath {
                                                maybe_ambiguous_item_path: TraitPath(`core::ops::Neg`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                        template_parameter_decl_list: Ok(
                            None,
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: SynNodeRegionPath::ItemDecl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Trait(
                                            TraitSynNodePath(
                                                ItemSynNodePathId {
                                                    data: ItemSynNodePathData::MajorItem(
                                                        MajorItemSynNodePathData::Trait(
                                                            TraitSynNodePathData {
                                                                disambiguated_item_path: DisambiguatedItemPath {
                                                                    maybe_ambiguous_item_path: TraitPath(`core::ops::Neg`),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [],
                                },
                                pattern_region: SynPatternRegion {
                                    pattern_arena: Arena {
                                        data: [],
                                    },
                                    pattern_contracts: [],
                                    pattern_variable_arena: Arena {
                                        data: [],
                                    },
                                    pattern_variable_maps: [],
                                    pattern_variable_modifiers: ArenaMap {
                                        data: [],
                                    },
                                },
                                variable_region: VariableRegionData {
                                    inherited_variable_arena: Arena {
                                        data: [],
                                    },
                                    current_variable_arena: Arena {
                                        data: [],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [],
                                },
                                pattern_roots: [],
                                expr_roots: [],
                                has_self_lifetime: false,
                                has_self_place: false,
                                pattern_to_current_variable_map: [],
                            },
                        },
                    },
                ),
            ),
        ),
        (
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Trait(
                    TraitSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Trait(
                                    TraitSynNodePathData {
                                        disambiguated_item_path: DisambiguatedItemPath {
                                            maybe_ambiguous_item_path: TraitPath(`core::ops::Not`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Trait(
                    TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::MajorItem(
                                    MajorItemSynNodePathData::Trait(
                                        TraitSynNodePathData {
                                            disambiguated_item_path: DisambiguatedItemPath {
                                                maybe_ambiguous_item_path: TraitPath(`core::ops::Not`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                        template_parameter_decl_list: Ok(
                            None,
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: SynNodeRegionPath::ItemDecl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Trait(
                                            TraitSynNodePath(
                                                ItemSynNodePathId {
                                                    data: ItemSynNodePathData::MajorItem(
                                                        MajorItemSynNodePathData::Trait(
                                                            TraitSynNodePathData {
                                                                disambiguated_item_path: DisambiguatedItemPath {
                                                                    maybe_ambiguous_item_path: TraitPath(`core::ops::Not`),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [],
                                },
                                pattern_region: SynPatternRegion {
                                    pattern_arena: Arena {
                                        data: [],
                                    },
                                    pattern_contracts: [],
                                    pattern_variable_arena: Arena {
                                        data: [],
                                    },
                                    pattern_variable_maps: [],
                                    pattern_variable_modifiers: ArenaMap {
                                        data: [],
                                    },
                                },
                                variable_region: VariableRegionData {
                                    inherited_variable_arena: Arena {
                                        data: [],
                                    },
                                    current_variable_arena: Arena {
                                        data: [],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [],
                                },
                                pattern_roots: [],
                                expr_roots: [],
                                has_self_lifetime: false,
                                has_self_place: false,
                                pattern_to_current_variable_map: [],
                            },
                        },
                    },
                ),
            ),
        ),
        (
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Trait(
                    TraitSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Trait(
                                    TraitSynNodePathData {
                                        disambiguated_item_path: DisambiguatedItemPath {
                                            maybe_ambiguous_item_path: TraitPath(`core::ops::Sub`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Trait(
                    TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::MajorItem(
                                    MajorItemSynNodePathData::Trait(
                                        TraitSynNodePathData {
                                            disambiguated_item_path: DisambiguatedItemPath {
                                                maybe_ambiguous_item_path: TraitPath(`core::ops::Sub`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                        template_parameter_decl_list: Ok(
                            Some(
                                SynTemplateParameterSyndicateList {
                                    langle: LaOrLtRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateSynParameterData {
                                            annotated_variance_token: None,
                                            symbol: 0,
                                            variant: TemplateParameterSyndicateVariant::Type {
                                                ident_token: IdentRegionalToken {
                                                    ident: `Rhs`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                },
                                                traits: None,
                                            },
                                        },
                                    ],
                                    commas: [],
                                    decl_list_result: Ok(
                                        (),
                                    ),
                                    rangle: RaOrGtRegionalToken(
                                        RegionalTokenIdx(
                                            6,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: SynNodeRegionPath::ItemDecl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Trait(
                                            TraitSynNodePath(
                                                ItemSynNodePathId {
                                                    data: ItemSynNodePathData::MajorItem(
                                                        MajorItemSynNodePathData::Trait(
                                                            TraitSynNodePathData {
                                                                disambiguated_item_path: DisambiguatedItemPath {
                                                                    maybe_ambiguous_item_path: TraitPath(`core::ops::Sub`),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [],
                                },
                                pattern_region: SynPatternRegion {
                                    pattern_arena: Arena {
                                        data: [],
                                    },
                                    pattern_contracts: [],
                                    pattern_variable_arena: Arena {
                                        data: [],
                                    },
                                    pattern_variable_maps: [],
                                    pattern_variable_modifiers: ArenaMap {
                                        data: [],
                                    },
                                },
                                variable_region: VariableRegionData {
                                    inherited_variable_arena: Arena {
                                        data: [],
                                    },
                                    current_variable_arena: Arena {
                                        data: [
                                            CurrentVariableEntry {
                                                modifier: Compterm,
                                                access_start: RegionalTokenIdx(
                                                    6,
                                                ),
                                                access_end: None,
                                                data: CurrentVariableData::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    data: CurrentTemplateVariableData::Type {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `Rhs`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                5,
                                                            ),
                                                        },
                                                        trai_syn_expr_idxs: [],
                                                    },
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [
                                        (
                                            TemplateTypeParameter,
                                            ArenaIdxRange(
                                                0..1,
                                            ),
                                        ),
                                    ],
                                },
                                pattern_roots: [],
                                expr_roots: [],
                                has_self_lifetime: false,
                                has_self_place: false,
                                pattern_to_current_variable_map: [],
                            },
                        },
                    },
                ),
            ),
        ),
        (
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Trait(
                    TraitSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Trait(
                                    TraitSynNodePathData {
                                        disambiguated_item_path: DisambiguatedItemPath {
                                            maybe_ambiguous_item_path: TraitPath(`core::ops::Unveil`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Trait(
                    TraitSynNodeDecl {
                        syn_node_path: TraitSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::MajorItem(
                                    MajorItemSynNodePathData::Trait(
                                        TraitSynNodePathData {
                                            disambiguated_item_path: DisambiguatedItemPath {
                                                maybe_ambiguous_item_path: TraitPath(`core::ops::Unveil`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                        template_parameter_decl_list: Ok(
                            Some(
                                SynTemplateParameterSyndicateList {
                                    langle: LaOrLtRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateSynParameterData {
                                            annotated_variance_token: None,
                                            symbol: 0,
                                            variant: TemplateParameterSyndicateVariant::Type {
                                                ident_token: IdentRegionalToken {
                                                    ident: `T`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                },
                                                traits: None,
                                            },
                                        },
                                    ],
                                    commas: [],
                                    decl_list_result: Ok(
                                        (),
                                    ),
                                    rangle: RaOrGtRegionalToken(
                                        RegionalTokenIdx(
                                            6,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: SynNodeRegionPath::ItemDecl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Trait(
                                            TraitSynNodePath(
                                                ItemSynNodePathId {
                                                    data: ItemSynNodePathData::MajorItem(
                                                        MajorItemSynNodePathData::Trait(
                                                            TraitSynNodePathData {
                                                                disambiguated_item_path: DisambiguatedItemPath {
                                                                    maybe_ambiguous_item_path: TraitPath(`core::ops::Unveil`),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [],
                                },
                                pattern_region: SynPatternRegion {
                                    pattern_arena: Arena {
                                        data: [],
                                    },
                                    pattern_contracts: [],
                                    pattern_variable_arena: Arena {
                                        data: [],
                                    },
                                    pattern_variable_maps: [],
                                    pattern_variable_modifiers: ArenaMap {
                                        data: [],
                                    },
                                },
                                variable_region: VariableRegionData {
                                    inherited_variable_arena: Arena {
                                        data: [],
                                    },
                                    current_variable_arena: Arena {
                                        data: [
                                            CurrentVariableEntry {
                                                modifier: Compterm,
                                                access_start: RegionalTokenIdx(
                                                    6,
                                                ),
                                                access_end: None,
                                                data: CurrentVariableData::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    data: CurrentTemplateVariableData::Type {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `T`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                5,
                                                            ),
                                                        },
                                                        trai_syn_expr_idxs: [],
                                                    },
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [
                                        (
                                            TemplateTypeParameter,
                                            ArenaIdxRange(
                                                0..1,
                                            ),
                                        ),
                                    ],
                                },
                                pattern_roots: [],
                                expr_roots: [],
                                has_self_lifetime: false,
                                has_self_place: false,
                                pattern_to_current_variable_map: [],
                            },
                        },
                    },
                ),
            ),
        ),
        (
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Type(
                    TypeSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Type(
                                    TypeSynNodePathData {
                                        disambiguated_item_path: DisambiguatedItemPath {
                                            maybe_ambiguous_item_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Type(
                    TypeSynNodeDecl::Enum(
                        EnumSynNodeDecl {
                            syn_node_path: TypeSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Type(
                                            TypeSynNodePathData {
                                                disambiguated_item_path: DisambiguatedItemPath {
                                                    maybe_ambiguous_item_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                            template_parameter_decl_list: Ok(
                                Some(
                                    SynTemplateParameterSyndicateList {
                                        langle: LaOrLtRegionalToken(
                                            RegionalTokenIdx(
                                                4,
                                            ),
                                        ),
                                        template_parameters: [
                                            TemplateSynParameterData {
                                                annotated_variance_token: None,
                                                symbol: 0,
                                                variant: TemplateParameterSyndicateVariant::Type {
                                                    ident_token: IdentRegionalToken {
                                                        ident: `B`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                    traits: None,
                                                },
                                            },
                                            TemplateSynParameterData {
                                                annotated_variance_token: None,
                                                symbol: 1,
                                                variant: TemplateParameterSyndicateVariant::Type {
                                                    ident_token: IdentRegionalToken {
                                                        ident: `C`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            7,
                                                        ),
                                                    },
                                                    traits: None,
                                                },
                                            },
                                        ],
                                        commas: [
                                            CommaRegionalToken(
                                                RegionalTokenIdx(
                                                    6,
                                                ),
                                            ),
                                        ],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rangle: RaOrGtRegionalToken(
                                            RegionalTokenIdx(
                                                8,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::ItemDecl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Type(
                                                TypeSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::MajorItem(
                                                            MajorItemSynNodePathData::Type(
                                                                TypeSynNodePathData {
                                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                                        maybe_ambiguous_item_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_region: SynPatternRegion {
                                        pattern_arena: Arena {
                                            data: [],
                                        },
                                        pattern_contracts: [],
                                        pattern_variable_arena: Arena {
                                            data: [],
                                        },
                                        pattern_variable_maps: [],
                                        pattern_variable_modifiers: ArenaMap {
                                            data: [],
                                        },
                                    },
                                    variable_region: VariableRegionData {
                                        inherited_variable_arena: Arena {
                                            data: [],
                                        },
                                        current_variable_arena: Arena {
                                            data: [
                                                CurrentVariableEntry {
                                                    modifier: Compterm,
                                                    access_start: RegionalTokenIdx(
                                                        6,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [],
                                                        },
                                                        annotated_variance_token: None,
                                                        data: CurrentTemplateVariableData::Type {
                                                            ident_token: IdentRegionalToken {
                                                                ident: `B`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    5,
                                                                ),
                                                            },
                                                            trai_syn_expr_idxs: [],
                                                        },
                                                    },
                                                },
                                                CurrentVariableEntry {
                                                    modifier: Compterm,
                                                    access_start: RegionalTokenIdx(
                                                        8,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [],
                                                        },
                                                        annotated_variance_token: None,
                                                        data: CurrentTemplateVariableData::Type {
                                                            ident_token: IdentRegionalToken {
                                                                ident: `C`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    7,
                                                                ),
                                                            },
                                                            trai_syn_expr_idxs: [],
                                                        },
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [
                                            (
                                                TemplateTypeParameter,
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                            (
                                                TemplateTypeParameter,
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                        ],
                                    },
                                    pattern_roots: [],
                                    expr_roots: [],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    pattern_to_current_variable_map: [],
                                },
                            },
                        },
                    ),
                ),
            ),
        ),
    ],
}
```
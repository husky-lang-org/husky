[
    Some(
        ValkyrieRides {
            hir_template_parameters: Some(
                HirTemplateParameters(
                    [
                        HirTemplateParameter {
                            symbol: Const(
                                HirConstSymbol(
                                    Id {
                                        value: 1,
                                    },
                                ),
                            ),
                            data: Constant {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 23,
                                        },
                                    ),
                                ),
                                ty: PathLeading(
                                    HirTypePathLeading(
                                        Id {
                                            value: 2,
                                        },
                                    ),
                                ),
                            },
                        },
                        HirTemplateParameter {
                            symbol: Type(
                                Type {
                                    attrs: HirSymbolAttrs,
                                    variance: Some(
                                        Covariant,
                                    ),
                                    disambiguator: 0,
                                },
                            ),
                            data: Type {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 25,
                                        },
                                    ),
                                ),
                                traits: [],
                            },
                        },
                    ],
                ),
            ),
            rides: VecSet {
                data: [],
            },
        },
    ),
]
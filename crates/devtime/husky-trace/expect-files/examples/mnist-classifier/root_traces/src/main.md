```rust
[
    Trace {
        path: TracePath {
            data: TracePathData::Submodule(
                SubmoduleTracePathData {
                    submodule_item_path: SubmoduleItemPath(`mnist_classifier::digits),
                },
            ),
        },
        data: Submodule(
            SubmoduleTraceData {
                submodule_item_path: SubmoduleItemPath(
                    ItemPathId(
                        Id {
                            value: 6,
                        },
                    ),
                ),
            },
        ),
    },
    Trace {
        path: TracePath {
            data: TracePathData::Submodule(
                SubmoduleTracePathData {
                    submodule_item_path: SubmoduleItemPath(`mnist_classifier::major),
                },
            ),
        },
        data: Submodule(
            SubmoduleTraceData {
                submodule_item_path: SubmoduleItemPath(
                    ItemPathId(
                        Id {
                            value: 7,
                        },
                    ),
                ),
            },
        ),
    },
    Trace {
        path: TracePath {
            data: TracePathData::ValItem(
                ValTracePathData {
                    val_path: MajorFormPath(`mnist_classifier::main`, `Val`),
                },
            ),
        },
        data: Val(
            ValTraceData {
                path: TracePath(
                    Id {
                        value: 3,
                    },
                ),
                val_path: MajorFormPath(
                    ItemPathId(
                        Id {
                            value: 8,
                        },
                    ),
                ),
            },
        ),
    },
]
```
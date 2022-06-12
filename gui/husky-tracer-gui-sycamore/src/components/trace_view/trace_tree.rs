use super::*;

#[derive(Prop)]
pub struct TraceTreeProps {
    trace_id: TraceId,
}

#[component]
pub fn TraceTree<'a, G: Html>(scope: Scope<'a>, props: TraceTreeProps) -> View<G> {
    let tracer_context = use_context::<TracerContext>(scope);
    let tree_context = &tracer_context.tree_context;
    let shown = tree_context.shown_signal(props.trace_id);
    let expansion = tree_context.expanded_signal(props.trace_id);
    let expansion = create_memo(scope, move || expansion.get_cloned());
    let focus = tracer_context.focus_context.focus_signal.clone();
    let associated_trace_trees = View::new_fragment(
        tree_context
            .trace(props.trace_id)
            .associated_trace_ids()
            .into_iter()
            .map(|trace_id| {
                view! { scope, TraceTree {
                    trace_id
                } }
            })
            .collect(),
    );
    let subtrace_ids = create_memo(scope, {
        move || {
            if expansion.get_cloned() {
                tree_context
                    .subtrace_ids(&focus.get(), props.trace_id)
                    .to_vec()
            } else {
                vec![]
            }
        }
    });
    let has_subtraces = create_memo(scope, { move || subtrace_ids.get().len() > 0 });
    if shown.get_cloned() {
        view! {
            scope,
            div(class="TraceTree") {
                TraceNode {
                    trace_id: props.trace_id,
                    has_subtraces,
                    expansion,
                }
                (associated_trace_trees)
                div {
                    Indexed {
                        iterable: subtrace_ids,
                        view: |scope, trace_id| view! {
                            scope,
                            TraceTree {
                                trace_id
                            }
                        },
                    }
                }
            }
        }
    } else {
        view! {scope, }
    }
}

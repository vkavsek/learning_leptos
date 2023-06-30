use leptos::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let name = "Alice".to_string();
    view! { cx,
        <Outer>
            <Inner clone:name>
                <Inmost name=name.clone()/>
            </Inner>
        </Outer>
    }
}

#[component]
pub fn Outer(cx: Scope, children: ChildrenFn) -> impl IntoView {
    children(cx)
}

#[component]
pub fn Inner(cx: Scope, children: ChildrenFn) -> impl IntoView {
    children(cx)
}

#[component]
pub fn Inmost(cx: Scope, name: String) -> impl IntoView {
    view! { cx,
        <p>{name}</p>
    }
}

fn main() {
    mount_to_body(|cx| view!{cx, 
        <App/>
    });
}

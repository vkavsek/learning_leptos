use leptos::*;

fn main() {
    mount_to_body(|cx| {
        view! { cx, <App/> }
    });
}

#[component]
fn App(cx: Scope) -> impl IntoView {}

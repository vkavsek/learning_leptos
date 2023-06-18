use leptos::*;

fn main() {
    mount_to_body(|cx| {
        view! { cx, <App/> }
    })
}
 
/// An example of a simple static list.
/// Plus a demonstration of using various control flow constructs.
#[component]
fn App(cx: Scope) -> impl IntoView {
    let vals = (0..10).map(|idx| create_signal(cx, idx));

    let paragraphs = vals
        .map(|(val, _)| {
            let is_odd = move || val() & 1 == 1;

            view! { cx, <p>{move || if is_odd() { "Odd" } else { "Even" }}</p> }
        })
        .collect_view(cx);

    view! { cx, <div>{paragraphs}</div> }
}

#[component]
fn IfControl(cx: Scope, value: ReadSignal<u32>) -> impl IntoView {
}

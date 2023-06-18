use leptos::*;

fn main() {
    mount_to_body(|cx| {
        view! { cx, <App/> }
    })
}

/// An example of a simple static list, plus a demonstration of using various control flow constructs.
#[component]
fn App(cx: Scope) -> impl IntoView {
    let vals = (0..10).map(|idx| create_signal(cx, idx));

    let paragraphs = vals
        .map(|(value, _)| {
            view! { cx, <IfControl value/> }
        })
        .collect_view(cx);

    view! { cx, <div>{paragraphs}</div> }
}

/// This example shows that you can use an if block inside a Html Element. To control the flow of the program.
/// An 'if' expression returns its value, and a &str implements IntoView, so a Fn() -> &str implements IntoView, so this... just works!
/// Always remember: to be reactive, values must be functions.
/// Note the syntax for 'is_odd' closure, and the syntax inside the paragraph in the view:
/// `{ move || if is_odd() { "Odd" } else { "Even" }}`
#[component]
fn IfControl(cx: Scope, value: ReadSignal<i32>) -> impl IntoView {
    let is_odd = move || value() & 1 == 1;

    view! { cx, <p>{move || if is_odd() { "Odd" } else { "Even" }}</p> }
}

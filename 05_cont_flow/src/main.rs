use leptos::*;

fn main() {
    mount_to_body(|cx| {
        view! { cx, <App/> }
    })
}

/// An example of a simple static list, plus a demonstration of using various control flow constructs.
#[component]
fn App(cx: Scope) -> impl IntoView {
    let vals = (0..100).map(|idx| create_signal(cx, idx));

    let paragraphs = vals
        .map(|(value, _)| {
            view! { cx,
                <IfControl value/>
                <OptionControl value/>
                <MatchControl value/>
            }
        })
        .collect_view(cx);

    view! { cx,
        <div class="demo-div">
            <DemoShow/>
        </div>
        <div>{paragraphs}</div>
    }
}

/// Sometimes using the control flow statements as shown bellow can be inefficient,
/// because all of the elements get rerendered every time the value changes. 
/// If that's the case the '<Show/>' component is the answer.
/// You pass it a 'when' condition function, a 'fallback' to be shown if the 'when' function returns false,
/// and children to be rendered if 'when' is true.
#[component]
fn DemoShow(cx: Scope) -> impl IntoView {
    let (val, set_val) = create_signal(cx, 0);

    let increment = move |_| set_val.update(|val| *val += 1);

    view! { cx,
        <button on:click=increment>"Click for +1"</button>
        <p>{val}</p>
        <div class="showtext-div">
            <Show
                when=move || val() < 5
                fallback=|_| {
                    view! { cx, "Even more EXPENSIVE! X > 5!" }
                }
            >
                "X < 5. Imagine this is a really expensive paragraph!"
            </Show>
        </div>
    }
}

/// This example shows how you can use an 'if block' to control the program flow. 
/// An 'if' expression returns its value, and a &str implements IntoView, so a Fn() -> &str implements IntoView,
/// so this... just works!
/// Always remember: to be reactive, values must be functions.
/// Note the syntax for 'is_odd' closure, and the message syntax:
/// `move || if is_odd() { "Odd" } else { "Even" }`
#[component]
fn IfControl(cx: Scope, value: ReadSignal<i32>) -> impl IntoView {
    let is_odd = move || value() & 1 == 1;

    let msg = move || if is_odd() { "Odd" } else { "Even" };
    view! { cx, <p class="if-par">{msg}</p> }
}

/// This example shows how you can use an `Option<impl IntoView>` to control the program flow.
#[component]
fn OptionControl(cx: Scope, value: ReadSignal<i32>) -> impl IntoView {
    let is_odd = move || value() & 1 == 1;

    let msg = is_odd().then(|| "Ding ding ding!");
    view! { cx, <p class="option-par">{msg}</p> }
}

/// This example shows how you can use 'match statements' to control the program flow.
#[component]
fn MatchControl(cx: Scope, value: ReadSignal<i32>) -> impl IntoView {
    let is_odd = move || value() & 1 == 1;

    let msg = match value() {
        0 => "Nič",
        1 => "Ena", 
        _n if is_odd() => "Liho", 
        _ => "Sodo"
    };
    view! { cx, <p class="match-par">{msg}</p> }
}

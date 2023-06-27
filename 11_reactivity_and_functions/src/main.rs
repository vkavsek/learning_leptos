use leptos::*;

fn main() {
    let a = vec!{1, 2, 3};
    mount_to_body(|cx| view! { cx, <App/> })
}

// --- REMEMBER ---
// 1. Your component function is a setup function, not a render function: it only runs once.
// 2. For values in your view template to be reactive, they must be functions: either signals (which implement the Fn traits) or closures.

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1>"Demo"</h1>
        <ReactivityNFunctions/>
        <SimpleCounter/>
    }
}

/// The SimpleCounter function itself runs once. The value signal is created once.
/// The framework hands off the increment function to the browser as an event listener.
/// When you click the button, the browser calls increment, which updates value via set_value.
/// And that updates the single text node represented in our view by {value}.
#[component]
pub fn SimpleCounter(cx: Scope) -> impl IntoView {
    // Created once
    let (value, set_value) = create_signal(cx, 0);

    // Handed off to browser as an event listener. On each click the browser calls the function.
    let increment = move |_| set_value.update(|value| *value += 1);

    view! { cx,
        <h2>"Simple Counter"</h2>
        <button on:click=increment>
            {value}
        </button>
    }
}

#[component]
fn ReactivityNFunctions(cx: Scope) -> impl IntoView {
    // a signal holds a value, and can be updated
    let (count, set_count) = create_signal(cx, 0);

    // a derived signal is a function that accesses other signals
    let _double_count = move || count() * 2;
    let count_is_odd = move || count() & 1 == 1;
    let text = move || if count_is_odd() { "odd" } else { "even" };

    // an effect automatically tracks the signals it depends on
    // and reruns when they change
    create_effect(cx, move |_| {
        log!("text = {}", text());
    });

    view! { cx,
        <h3>"Reactivity and functions demo"</h3>
        <input
            type="text"
            on:input=move |ev| set_count( 
                if let Ok(i) = event_target_value(&ev).parse() {
                    i 
                } else { 
                    0 
                })
            prop:value=count
        />
        <p>"Value is: "{count}" and "{text}</p>
        <hr/>
    }
}

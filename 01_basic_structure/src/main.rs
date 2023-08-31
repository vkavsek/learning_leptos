use leptos::*;

fn main() {
    mount_to_body(|cx| view! { cx,
            <Cd initial_value=50/>
            <App/>
            <SimpleCounter initial_value=300/>
        })
}
#[component]
pub fn Cd(cx: Scope, initial_value: i32) -> impl IntoView {
    let (value, set_value) = create_signal(cx, initial_value);

    let clear = move |_| set_value(0);
    let decrement = move |_| set_value.update(|value| *value -= 1);
    let increment = move |_| set_value.update(|value| *value += 1);

    view! { cx,
        <div class:red=move || value() % 2 == 1>
            <button on:click=clear>"Clear"</button>
            <button on:click=decrement>"-1"</button>
            <span>"Value: " {move || value().to_string()}</span>
            <button on:click=increment>"+1"</button>
        </div>
    }
}

/// Quick basic Leptos structure explanation
#[component]
pub fn SimpleCounter(cx: Scope, initial_value: i32) -> impl IntoView {
    // create a reactive signal with the initial value
    let (value, set_value) = create_signal(cx, initial_value);

    // create event handlers for our buttons
    // note that `value` and `set_value` are `Copy`, so it's super easy to move them into closures
    let clear = move |_| set_value(0);
    let decrement = move |_| set_value.update(|value| *value -= 1);
    let increment = move |_| set_value.update(|value| *value += 1);

    // this JSX is compiled to an HTML template string for performance
    view! { cx,
        <div>
            <button on:click=clear>"Clear"</button>
            <button on:click=decrement>"-1"</button>
            <span>"Value: " {move || value().to_string()}</span>
            <button on:click=increment>"+1"</button>
        </div>
    }
}

// The #[component] macro marks a function as a reusable component
// Components are the building blocks of your user interface
// They define a reusable unit of behavior
#[component]
/// More detailed basic Leptos structure explanation
fn App(cx: Scope) -> impl IntoView {
    // here we create a reactive signal
    // and get a (getter, setter) pair
    // signals are the basic unit of change in the framework
    let (count, set_count) = create_signal(cx, 0);
    // the `view` macro is how we define the user interface
    // it uses an HTML-like format that can accept certain Rust values
    view! { cx,
        <button
            // on:click will run whenever the `click` event fires
            // every event handler is defined as `on:{eventname}`
            // we're able to move `set_count` into the closure
            // because signals are Copy and 'static
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
            // text nodes in RSX should be wrapped in quotes,
            // like a normal Rust string
            "Click me"
        </button>
        <p>
            <strong>"Reactive: "</strong>
            // you can insert Rust expressions as values in the DOM
            // by wrapping them in curly braces
            // if you pass in a function, it will reactively update
            {move || count.get()}
        </p>
        <p>
            <strong>"Reactive shorthand: "</strong>
            // signals are functions, so we can remove the wrapping closure
            {count}
        </p>
        <p>
            <strong>"Not reactive: "</strong>
            // NOTE: if you write {count()}, this will *not* be reactive
            // it simply gets the value of count once
            {count()}
        </p>
    }
}

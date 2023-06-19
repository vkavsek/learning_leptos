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
                <Conversion value/>
            }
        })
        .collect_view(cx);

    view! { cx,
            <main>
                <div>
                    <h1>"My Examples"</h1>
                    <div class="demo-div">
                        <DemoShow/>
                    </div>
                    <div>{paragraphs}</div>
                </div>
                <div>
                    <LeptosBookExample/>
                </div>
            </main>
    }
}

/// Sometimes using the control flow statements as shown bellow can be inefficient,
/// because all of the elements get rerendered every time the value changes. 
/// If that's the case the '<Show/>' component is the answer.
/// You pass it a 'when' condition function, a 'fallback' to be shown if the 'when' function returns false,
/// and children to be rendered if 'when' is true.
/// There is some overhead though, so for a very simple node a 'move || if ...' will be more efficient.
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

/// This example shows how you can return multiple different element types from different branches
/// of a condition.
/// 1. If you have multiple HtmlElement types, convert them to HtmlElement<AnyElement> with '.into_any()' 
/// 2. If you have a variety of view types that are not all HtmlElement, convert them to Views
///    with '.into_view(cx)'.
#[component]
fn Conversion(cx: Scope, value: ReadSignal<i32>) -> impl IntoView {
    let is_odd = move || value() & 1 == 1;

    view! { cx,
        {move || match is_odd() {
            true if value() == 1 => {
                view! { cx, <pre>"One"</pre> }
                    .into_any()
            }
            false if value() == 2 => {
                view! { cx, <p>"Two"</p> }
                    .into_any()
            }
            _ => {
                view! { cx, <textarea>{value()}</textarea> }
                    .into_any()
            }
        }}
    }
}

/// A book example
#[component]
fn LeptosBookExample(cx: Scope) -> impl IntoView {
    let (value, set_value) = create_signal(cx, 0);
    let is_odd = move || value() & 1 == 1;
    let odd_text = move || if is_odd() { Some("How odd!") } else { None };

    view! { cx,
        <h1>"Control Flow"</h1>

        // Simple UI to update and show a value
        <button on:click=move |_| set_value.update(|n| *n += 1)>
            "+1"
        </button>
        <p>"Value is: " {value}</p>

        <hr/>

        <h2><code>"Option<T>"</code></h2>
        // For any `T` that implements `IntoView`,
        // so does `Option<T>`

        <p>{odd_text}</p>
        // This means you can use `Option` methods on it
        <p>{move || odd_text().map(|text| text.len())}</p>

        <h2>"Conditional Logic"</h2>
        // You can do dynamic conditional if-then-else
        // logic in several ways
        //
        // a. An "if" expression in a function
        //    This will simply re-render every time the value
        //    changes, which makes it good for lightweight UI
        <p>
            {move || if is_odd() {
                "Odd"
            } else {
                "Even"
            }}
        </p>

        // b. Toggling some kind of class
        //    This is smart for an element that's going to
        //    toggled often, because it doesn't destroy
        //    it in between states
        //    (you can find the `hidden` class in `index.html`)
        <p class:hidden=is_odd>"Appears if even."</p>

        // c. The <Show/> component
        //    This only renders the fallback and the child
        //    once, lazily, and toggles between them when
        //    needed. This makes it more efficient in many cases
        //    than a {move || if ...} block
        <Show when=is_odd
            fallback=|cx| view! { cx, <p>"Even steven"</p> }
        >
            <p>"Oddment"</p>
        </Show>

        // d. Because `bool::then()` converts a `bool` to
        //    `Option`, you can use it to create a show/hide toggled
        {move || is_odd().then(|| view! { cx, <p>"Oddity!"</p> })}

        <h2>"Converting between Types"</h2>
        // e. Note: if branches return different types,
        //    you can convert between them with
        //    `.into_any()` (for different HTML element types)
        //    or `.into_view(cx)` (for all view types)
        {move || match is_odd() {
            true if value() == 1 => {
                // <pre> returns HtmlElement<Pre>
                view! { cx, <pre>"One"</pre> }.into_any()
            },
            false if value() == 2 => {
                // <p> returns HtmlElement<P>
                // so we convert into a more generic type
                view! { cx, <p>"Two"</p> }.into_any()
            }
            _ => view! { cx, <textarea>{value()}</textarea> }.into_any()
        }}
    }
}

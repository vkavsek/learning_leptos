use leptos::*;

fn main() {
    mount_to_body(|cx| {
        view! { cx, <App/> }
    });
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <NumericInput/>
        <DemoErrBoundary/>
    }
}

/// You can use the Result type similarly as Option type.
/// This example shows how you can try to parse from input to integer.
/// Note that in this component we dont handle the error, instead if error gets outputted
/// we just don't render the user input.
#[component]
fn NumericInput(cx: Scope) -> impl IntoView {
    let (val, set_val) = create_signal(cx, Ok(0));

    // when input changes, try to parse a number from the input
    let on_input = move |ev| set_val(event_target_value(&ev).parse::<i32>());

    view! { cx,
        <label>
            "Type a number (or not!)" <input type="number" on:input=on_input/>
            <p>"You entered " <strong>{val}</strong></p>
        </label>
    }
}

/// To actually handle the errors you can use the <ErrorBoundary/> component.
#[component]
fn DemoErrBoundary(cx: Scope) -> impl IntoView {
    let (val, set_val) = create_signal(cx, Ok(0));

    let on_input = move |ev| set_val(event_target_value(&ev).parse::<i32>());

    view! { cx,
        <h1>"ErrorBoundary"</h1>
        <label>
            "Type a number (or something that's not a number!)"
            <input type="number" on:input=on_input/>
            // the fallback receives a signal containing current errors
            <ErrorBoundary fallback=|cx, errors| {
                view! { cx,
                    <div class="error">
                        // we can render a list of errors as strings, if we'd like
                        <p>"Not a number! Errors: "</p>
                        <ul>
                            {move || {
                                errors
                                    .get()
                                    .into_iter()
                                    .map(|(_, e)| {
                                        view! { cx, <li>{e.to_string()}</li> }
                                    })
                                    .collect_view(cx)
                            }}
                        </ul>
                    </div>
                }
            }>
                <p>"You entered " <strong>{val}</strong></p>
            </ErrorBoundary>
        </label>
    }
}

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
        <BookExample/>
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
        <h2>"No error handling"</h2>
        <label>
            "Type a number (or not!)" <input type="number" on:input=on_input/>
            <p>"You entered " <strong>{val}</strong></p>
        </label>
    }
}

/// To actually handle the errors you can use the <ErrorBoundary/> component.
#[component]
fn BookExample(cx: Scope) -> impl IntoView {
    let (value, set_value) = create_signal(cx, Ok(0));

    // when input changes, try to parse a number from the input
    let on_input = move |ev| set_value(event_target_value(&ev).parse::<i32>());

    view! { cx,
        <h2>"<ErrorBoundary/> handling"</h2>
        <label>
            "Type a number (or something that's not a number!)"

            <input type="number" on:input=on_input/>
            // If an `Err(_) had been rendered inside the <ErrorBoundary/>,
            // the fallback will be displayed. Otherwise, the children of the
            // <ErrorBoundary/> will be displayed.
            <ErrorBoundary
                // the fallback receives a signal containing current errors
                fallback=|cx, errors| view! { cx,
                    <div class="error">
                        <p>"Not a number! Errors: "</p>
                        // we can render a list of errors
                        // as strings, if we'd like
                        <ul>
                            {move || errors.get()
                                .into_iter()
                                .map(|(_, e)| view! { cx, <li>{e.to_string()}</li>})
                                .collect::<Vec<_>>()
                            }
                        </ul>
                    </div>
                }
            >
                <p>
                    "You entered "
                    // because `value` is `Result<i32, _>`,
                    // it will render the `i32` if it is `Ok`,
                    // and render nothing and trigger the error boundary
                    // if it is `Err`. It's a signal, so this will dynamically
                    // update when `value` changes
                    <strong>{value}</strong>
                </p>
            </ErrorBoundary>
        </label>
    }
}

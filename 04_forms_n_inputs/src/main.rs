use leptos::{ev::SubmitEvent, html::Input, *};

fn main() {
    mount_to_body(|cx| {
        view! { cx,
            <h2>"Controlled Input"</h2>
            <ContInput/>
            <h2>"Uncontrolled Input"</h2>
            <UncontInput/>
        }
    })
}

/// In a "controlled input" the framework controls the state of the input element.
/// On every `input` event, it updates a local signal that holds the current state, which in turn
/// updates the `value` prop of the input.
#[component]
fn ContInput(cx: Scope) -> impl IntoView {
    let (name, set_name) = create_signal(cx, "Controlled".to_string());

    // event_target_value is a Leptos helper function it functions the same way as event.target.value
    // in JavaScript, but smooths out some of the typecasting necessary to make this work in Rust
    //
    // the `prop:` syntax lets you update a DOM property,
    // rather than an attribute.
    //
    // The `value` attribute only sets the initial value of the input, it
    // only updates the input up to the point that you begin typing.
    // The `value` property continues updating the input after that.
    // That's why we use `prop:value=` not `value=`
    view! { cx,
        <input
            type="text"
            on:input=move |ev| { set_name(event_target_value(&ev)) }
            prop:value=name
        />
        <p>"Name is: " {name}</p>
    }
}

/// In an "uncontrolled input" the browser controls the state of the input element. 
/// Rather than continuously updating a signal to hold its value, we use a NodeRef to access the
/// input once when we want to get its value.
#[component]
fn UncontInput(cx: Scope) -> impl IntoView {
    let (name, set_name) = create_signal(cx, "Uncontrolled".to_string());
    let input_elem: NodeRef<Input> = create_node_ref(cx);

    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading
        ev.prevent_default();

        // to access the DOM node stored in the NodeRef, we can simply call it as a function
        // (or using `.get()` if on stable)
        let val = input_elem()
            // event handlers can only fire after the view is mounted to the DOM,
            // so we know the `NodeRef` will be `Some`
            .expect("<input> to exist")
            // `NodeRef` implements `Deref` for the DOM element type,
            // we can call `HtmlInputElement::value()` to get the current value of the input.
            .value();
        set_name(val);
    };

    view! { cx,
        <form on:submit=on_submit>
            <input type="text" value=name node_ref=input_elem/>
            <input type="submit" value="Submit"/>
        </form>
        <p>"Name is: " {name}</p>
    }
}

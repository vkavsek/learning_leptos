use leptos::{*, html::Input};

fn main() { 
    mount_to_body(|cx| {
       view! { cx, <ContInput/> }
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
    view! { cx,
        <input
            type="text"
            on:input=move |ev| { set_name(event_target_value(&ev)) }
            prop:value=name
        />
        <p>"Name is: " {name}</p>
    }
}

#[component]
fn UncontInput (cx: Scope) -> impl IntoView {
    let (name, set_name) = create_signal(cx, "Uncontrolled".to_string());
    let input_elem: NodeRef<Input> = create_node_ref(cx);
}

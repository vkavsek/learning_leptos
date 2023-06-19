use leptos::{ev::MouseEvent, *};

fn main() {
    mount_to_body(|cx| {
        view! { cx, <App/> }
    });
}

/// How can a child send notifications about events or state changes back up to the parent?
/// There are four basic patterns of parent-child communication in Leptos.
/// 1. Pass a 'WriteSignal'
/// 2. Use a Callback
/// 3. Use an Event Listener
/// 4. Providing a Context
#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Create different signals for each button for demonstration purposes
    let (toggled_a, set_toggled_a) = create_signal(cx, false);
    let (toggled_b, set_toggled_b) = create_signal(cx, false);
    let (toggled_c, set_toggled_c) = create_signal(cx, false);
    let (toggled_d, set_toggled_d) = create_signal(cx, false);

    let click_callback = move |_| set_toggled_b.update(|val| *val = !*val);

    view! { cx,
        <p>"PassWriteSignal toggled? " {toggled_a}</p>
        <PassWriteSignal setter=set_toggled_a/>

        <p>"UseCallback toggled? " {toggled_b}</p>
        <UseCallback on_click=click_callback/>
    }
}

/// 1. Passing a 'WriteSignal'
/// This pattern is simple, but you should be careful with it: passing around a WriteSignal can
/// make it hard to reason about your code, ie. it makes it easy to write spaghetti code.
#[component]
pub fn PassWriteSignal(cx: Scope, setter: WriteSignal<bool>) -> impl IntoView {
    let click = move |_| setter.update(|value| *value = !*value);

    view! { cx, <button on:click=click>"Toggle"</button> }
}

/// 2. Using a Callback
/// In this pattern the component simply fires an event: the mutation happens back in <App/>.
/// This has the advantage of keeping local state local. But it also means the logic to mutate that
/// signal needs to exist up in <App/> not inside this component. 
#[component]
pub fn UseCallback<F>(cx: Scope, on_click: F) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    view! { cx, <button on:click=on_click>"Toggle"</button> }
}

/// 3. Using an Event Listener
#[component]
pub fn UseEventListener(cx: Scope) -> impl IntoView {}

use context::ContextApp;
use book_app::BookApp;
use leptos::{ev::MouseEvent, *};

fn main() {
    mount_to_body(|cx| {
        view! { cx,
            <App/>
            <ContextApp/>
            <BookApp/>
        }
    });
}

/// How can a child send notifications about events or state changes back up to the parent?
/// There are four basic patterns of parent-child communication in Leptos.
/// 1. Pass a 'WriteSignal'
/// 2. Use a Callback
/// 3. Use an Event Listener
/// 4. Providing a Context -> Described in `<ContextApp/>` documentation comments.
#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Create different signals for each button for demonstration purposes
    let (toggled_a, set_toggled_a) = create_signal(cx, false);
    let (toggled_b, set_toggled_b) = create_signal(cx, false);
    let (toggled_c, set_toggled_c) = create_signal(cx, false);

    let click_callback = move |_| set_toggled_b.update(|val| *val = !*val);
    let click_event_listener = move |_| set_toggled_c.update(|val| *val = !*val);

    view! { cx,
        <p>"PassWriteSignal toggled? " {toggled_a}</p>
        <PassWriteSignal setter=set_toggled_a/>

        <p>"UseCallback toggled? " {toggled_b}</p>
        <UseCallback on_click=click_callback/>

        // Note the on:click instead of on_click that was transfered here from the individual component
        // this is the same syntax as an HTML element event listener
        <p>"UseEventListener toggled? " {toggled_c}</p>
        <UseEventListener on:click=click_event_listener/>
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
/// This pattern is similar to the callback pattern.
/// If the callback maps directly onto a native DOM event, you can add an 'on:' listener directly
/// to the place you use the component in your 'view' macro in App.
/// In this pattern everything about state control is transfered to the App component.
/// This works by addin an 'on:' event listener to each element that this component returns: in
/// this case, just the one button.
///
/// Of course, this only works for actual DOM events that you're passing directly through to the
/// elements you're rendering in the components. For more complex logic that doesn't map directly
/// onto an element you should use the callback pattern.
#[component]
pub fn UseEventListener(cx: Scope) -> impl IntoView {
    view! { cx, <button>"Toggle"</button> }
}

mod context {
    use leptos::*;

    /// 4. Providing a contex
    /// If you have a deeply nested context tree you can't simply pass your 'WriteSignal' to
    /// `<ButtonD/>` because it is no longer a direct child of this component.
    ///
    /// You can provide data that skips levels by using 'provide_context' and 'use_context'.
    /// Contexts are indentified by the type of the data you provide, and they exist in a top down
    /// tree that follows the contours of your UI tree. In this, example we can use context to skip
    /// unnecessary 'prop drilling'.
    ///
    /// Here the same caveats apply as in the 1st pattern: passing a 'WriteSignal' should be done
    /// with caution, as it allows you to mutate state from arbitrary parts of your code.
    /// 
    /// There are no performance downsides to this approach. Because what is passed is a
    /// fine-grained reactive signal, *nothing happens* in the intervening components (`<Content/>`
    /// and `<Layout/>`) when you update it.
    #[component]
    pub fn ContextApp(cx: Scope) -> impl IntoView {
        let (toggled, set_toggled) = create_signal(cx, false);

        provide_context(cx, set_toggled);

        view! { cx,
            <Layout/>
            <p>"Toggled? " {toggled}</p>
        }
    }

    #[component]
    pub fn Layout(cx: Scope) -> impl IntoView {
        view! { cx,
            <header>
                <h2>"Context"</h2>
            </header>
            <main>
                <Content/>
            </main>
        }
    }

    #[component]
    pub fn Content(cx: Scope) -> impl IntoView {
        view! { cx,
            <div class="content">
                <ButtonD/>
            </div>
        }
    }

    #[component]
    pub fn ButtonD(cx: Scope) -> impl IntoView {
        let setter: WriteSignal<bool> = use_context(cx).expect("to have found the setter provided");

        let click_on_button = move |_| setter.update(|val| *val = !*val);
        view! {cx,
            <button on:click=click_on_button>"Toggle"</button>
        }
    }
}

mod book_app {
    use leptos::{ev::MouseEvent, *};

    // This highlights four different ways that child components can communicate
    // with their parent:
    // 1) <ButtonA/>: passing a WriteSignal as one of the child component props,
    //    for the child component to write into and the parent to read
    // 2) <ButtonB/>: passing a closure as one of the child component props, for
    //    the child component to call
    // 3) <ButtonC/>: adding an `on:` event listener to a component
    // 4) <ButtonD/>: providing a context that is used in the component (rather than prop drilling)

    #[derive(Copy, Clone)]
    struct SmallcapsContext(WriteSignal<bool>);

    #[component]
    pub fn BookApp(cx: Scope) -> impl IntoView {
        // just some signals to toggle three classes on our <p>
        let (red, set_red) = create_signal(cx, false);
        let (right, set_right) = create_signal(cx, false);
        let (italics, set_italics) = create_signal(cx, false);
        let (smallcaps, set_smallcaps) = create_signal(cx, false);

        // the newtype pattern isn't *necessary* here but is a good practice
        // it avoids confusion with other possible future `WriteSignal<bool>` contexts
        // and makes it easier to refer to it in ButtonC
        provide_context(cx, SmallcapsContext(set_smallcaps));

        view! {
            cx,
            <hr/>
            <div>
                <h2>"Book example"</h2>
                <p
                    // class: attributes take F: Fn() => bool, and these signals all implement Fn()
                    class:red=red
                    class:right=right
                    class:italics=italics
                    class:smallcaps=smallcaps
                >
                    "Lorem ipsum sit dolor amet."
                </p>

                // Button A: pass the signal setter
                <ButtonA setter=set_red/>

                // Button B: pass a closure
                <ButtonB on_click=move |_| set_right.update(|value| *value = !*value)/>

                // Button B: use a regular event listener
                // setting an event listener on a component like this applies it
                // to each of the top-level elements the component returns
                <ButtonC on:click=move |_| set_italics.update(|value| *value = !*value)/>

                // Button D gets its setter from context rather than props
                <ButtonD/>
            </div>
        }
    }

    /// Button A receives a signal setter and updates the signal itself
    #[component]
    pub fn ButtonA(
        cx: Scope,
        /// Signal that will be toggled when the button is clicked.
        setter: WriteSignal<bool>,
    ) -> impl IntoView {
        view! {
            cx,
            <button
                on:click=move |_| setter.update(|value| *value = !*value)
            >
                "Toggle Red"
            </button>
        }
    }

    /// Button B receives a closure
    #[component]
    pub fn ButtonB<F>(
        cx: Scope,
        /// Callback that will be invoked when the button is clicked.
        on_click: F,
    ) -> impl IntoView
    where
        F: Fn(MouseEvent) + 'static,
    {
        view! {
            cx,
            <button
                on:click=on_click
            >
                "Toggle Right"
            </button>
        }

        // just a note: in an ordinary function ButtonB could take on_click: impl Fn(MouseEvent) + 'static
        // and save you from typing out the generic
        // the component macro actually expands to define a
        //
        // struct ButtonBProps<F> where F: Fn(MouseEvent) + 'static {
        //   on_click: F
        // }
        //
        // this is what allows us to have named props in our component invocation,
        // instead of an ordered list of function arguments
        // if Rust ever had named function arguments we could drop this requirement
    }

    /// Button C is a dummy: it renders a button but doesn't handle
    /// its click. Instead, the parent component adds an event listener.
    #[component]
    pub fn ButtonC(cx: Scope) -> impl IntoView {
        view! {
            cx,
            <button>
                "Toggle Italics"
            </button>
        }
    }

    /// Button D is very similar to Button A, but instead of passing the setter as a prop
    /// we get it from the context
    #[component]
    pub fn ButtonD(cx: Scope) -> impl IntoView {
        let setter = use_context::<SmallcapsContext>(cx).unwrap().0;

        view! {
            cx,
            <button
                on:click=move |_| setter.update(|value| *value = !*value)
            >
                "Toggle Small Caps"
            </button>
        }
    }
}

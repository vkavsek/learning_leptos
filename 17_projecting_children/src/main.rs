use leptos::*;

// As you build components you may occasionally find yourself wanting to "project" children through
// multiple layers of components.

/// This is pretty straightforward: when the user is logged in, we want to show children.
/// If the user is not logged in, we want to show fallback.
#[component] 
pub fn FirstDemo(cx: Scope) -> impl IntoView {
    view! { cx,
        <h2>"LoggedIn"</h2>
        <LoggedIn fallback=|cx| {
            view! { cx, <p>"NOT LOGGED IN!!"</p> }
        }>"Logged IN!"</LoggedIn>
    }
}

/// In other words, we want to pass the children of `<LoggedIn/>` through the `<Suspense/>` component to become the children of the `<Show/>`. 
/// However, both `<Suspense/>` and `<Show/>` take ChildrenFn, i.e.,
/// their children should implement the Fn type so they can be called multiple times with only an immutable reference.
/// We can solve this problem by using the `store_value` primitive.
/// This works because <Show/> and <Suspense/> only need an immutable refernce to their children (which `.with_value()` can give it), not ownership.
/// In other cases, you may need to project owned props, this is demonstrated below this component.
#[component]
pub fn LoggedIn<F, IV>(cx: Scope, fallback: F, children: ChildrenFn) -> impl IntoView
where
    F: Fn(Scope) -> IV + 'static,
    IV: IntoView,
{
    // Get a random number wrapped inside a signal just for demonstration purposes
    let mut seed = get_rand(); 
    let (rand, set_rand) = create_signal(cx, get_rand_wseed(seed));
    let check = move || rand() < 50;
    let click = move |_| {
        seed += 1;
        set_rand(get_rand_wseed(seed))
    };

    
    // This essentially stores a value in the reactive system, handing ownership off to the framework in exchange 
    // for a reference that is, like signals: `Copy` and `'static`, which we can access or modify through certain methods.
    let fallback = store_value(cx, fallback);
    let children = store_value(cx, children);

    // Note the ˙.with_value()` syntax to get the value out of StoredValue.
    view! { cx,
        <button on:click=click>"Change random NUMBER"</button>
        <p>"Number is: " {rand} "! Numbers under 50 get logged in!"</p>
        <Suspense fallback=|| ()>
            <Show when=check fallback=move |cx| fallback.with_value(|fallback| fallback(cx))>
                {children.with_value(|children| children(cx))}
            </Show>
        </Suspense>
    }
}

/// You may need to project owned props through a function that takes ChildrenFn and therefore needs to be called more than once.
/// In this case, you may find the clone: helper in the view macro helpful.
/// It's captured through multiple leveles of chzildren that need to run more than once, and there's no obvious way to clone it into the children.
/// In this case, the `clone:` syntax comes in handy. Calling `clone:name` will clone name before moving it into `<Inner/>`'s  children,
/// which solves our ownership issue.
#[component]
pub fn SecondDemo(cx: Scope) -> impl IntoView {
    let name = "Alice".to_string();
    view! { cx,
        <h2>"OuterInnerInmost"</h2>
        <Outer>
            <Inner clone:name>
                <Inmost name=name.clone()/>
            </Inner>
        </Outer>
    }
}

#[component]
pub fn Outer(cx: Scope, children: ChildrenFn) -> impl IntoView {
    children(cx)
}

#[component]
pub fn Inner(cx: Scope, children: ChildrenFn) -> impl IntoView {
    children(cx)
}

#[component]
pub fn Inmost(cx: Scope, name: String) -> impl IntoView {
    view! { cx, <p>{name}</p> }
}


// ---- Regular RUST ---- //

use tinyrand::{Rand, Seeded, StdRand};

fn get_rand_wseed(seed: u64) -> usize {
    let mut rand = StdRand::seed(seed);
    rand.next_lim_usize(100)
}

fn get_rand() -> u64 {
    let mut rand = StdRand::default();
    rand.next_u64()
}

fn main() {
    mount_to_body(|cx| {
        view! { cx, <SecondDemo/> }
    });
}

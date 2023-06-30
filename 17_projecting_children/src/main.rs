use leptos::*;

// As you build components you may occasionally find yourself wanting to "project" children through
// multiple layers of components.

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let name = "Alice".to_string();
    view! { cx,
        <h2>"LoggedIn"</h2>
        <LoggedIn fallback=|cx| {
            view! { cx, <p>"NOT LOGGED IN!!"</p> }
        }>"Logged IN!"</LoggedIn>
        <h2>"OuterInnerInmost"</h2>
        <Outer>
            <Inner clone:name>
                <Inmost name=name.clone()/>
            </Inner>
        </Outer>
    }
}

#[component]
pub fn LoggedIn<F, IV>(cx: Scope, fallback: F, children: ChildrenFn) -> impl IntoView
where
    F: Fn(Scope) -> IV + 'static,
    IV: IntoView,
{
    // Get a random number in a signal just for demonstration purposes
    let mut seed = get_rand(); 
    let (rand, set_rand) = create_signal(cx, get_rand_wseed(seed));
    let check = move || rand() < 50;
    let click = move |_| {
        seed += 1;
        set_rand(get_rand_wseed(seed))
    };

    let fallback = store_value(cx, fallback);
    let children = store_value(cx, children);

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
        view! { cx, <App/> }
    });
}

use leptos::*;
use leptos_router::*;

fn main() {
    mount_to_body(|cx| {
        view! { cx, <RouterDemo/> }
    })
}

/// Routing behavior is provided by the `<Router/>` component.
/// This should usually be somewhere near the root of your application.
/// You shouldn't try to use multiple `<Router/>`'s in your app!
///
/// The `<Routers/>` component is where you define all the routes to which a user can navigate in your application.
/// Each possible route is defined by a `<Route/>` component. You can see the syntax below.
///
/// You should place the `<Routes/>` component at the location within your app where you want routes to be rendered.
/// Everything outside `<Routes/>` will be present on every page, so you can leave things like a navigation bar
/// or menu outside the `<Routes/>`.
///
/// Individual routes are defined by providing children to <Routes/> with the <Route/> component.
/// <Route/> takes a path and a view. When the current location matches path, the view will be created and displayed.
/// The path can include:
///     - a static path ( /about_me ),
///     - dynamic, named parameters beginning with a colon ( /about_me/:id ),
///     - and/or a wildcard beginning with an asterisk ( /*any ),
#[component]
fn RouterDemo(cx: Scope) -> impl IntoView {
    view! { cx,
        <Router>
            <nav>
                <ul class="navigation_list">
                    <li>
                        <a href="/">"HOME"</a>
                    </li>
                    <li>
                        <a href="/about_me">"ABOUT ME"</a>
                    </li>
                    <li>
                        <a href="/about_me/special">"ABOUT with :id"</a>
                    </li>
                    <li>
                        <a href="/jbg_dec">"UNDEFINED"</a>
                    </li>
                </ul>
            </nav>
            <main>
                <Routes>
                    <Route
                        path="/"
                        view=|cx| {
                            view! { cx, <Home/> }
                        }
                    />
                    <Route
                        path="/about_me"
                        view=|cx| {
                            view! { cx, <About/> }
                        }
                    />
                    <Route
                        path="/about_me/:id"
                        view=|cx| {
                            view! { cx, <AboutSpecial/> }
                        }
                    />
                    <Route
                        path="/*any"
                        view=|cx| {
                            view! { cx, <NotFound/> }
                        }
                    />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Home(cx: Scope) -> impl IntoView {
    view! { cx, <h1 id="home_page">"THIS IS MY HOME PAGE"</h1> }
}

#[component]
fn About(cx: Scope) -> impl IntoView {
    view! { cx, <h1 id="about_page">"ABOUT ME GENERIC"</h1> }
}

#[component]
fn AboutSpecial(cx: Scope) -> impl IntoView {
    view! { cx, <h1 id="about_special">"I AM SPECIAL, LOOK AT ME!"</h1> }
}

#[component]
fn NotFound(cx: Scope) -> impl IntoView {
    view! { cx, <h1>"404 - PAGE NOT FOUND"</h1> }
}

use leptos::*;
use leptos_meta::*;

// ALL of this can obviously be made to be reactive!
// There is also a <Script/>, <Html/> and <Body/> components, the details are in the Leptos Book.

/// You can also use this to import some custom CSS from another file at compile time with: 
/// `<Style> { include_str!("my_route.css") } </Style>`
#[component]
fn Styling(cx: Scope) -> impl IntoView {
    view! { cx,  
        <Style>
            "body {
                color: white;
                background-color: #572c7f;
            }"
        </Style>
    }
}

/// You can import links with a Link component, there is also a Stylesheet component that is just shortened <link rel="stylesheet">.
#[component]
fn AddLinks(cx: Scope) -> impl IntoView {
    view! { cx,
        <Link rel="preconnect" href="https://fonts.googleapis.com"/>
        <Link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous"/>
        <Stylesheet
            href="https://fonts.googleapis.com/css2?family=Rubik+Iso&display=swap"
        />
        <h2 class="font-demo">"We just imported a new font!"</h2>
    }
}

#[component]
fn ChMeta(cx: Scope,) -> impl IntoView {
    view! {cx, 
        <Meta name="description" content="Meta injection demo"/>
        <Meta name="author" content="Vid Kavšek"/>
    }
}

/// The title of the webpage changes as you type and is formatted according to the 'formatter' in the App component.
#[component]
fn ChTitle(cx: Scope) -> impl IntoView {
    let (title, set_title) = create_signal(cx, "Webpage".to_string());
    let inp = move |ev| set_title(event_target_value(&ev));

    view! { cx,
        <Title text=title/>
        <h2>"Change the title of this webpage"</h2>
        <input type="text" on:input=inp/>
    }
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let formatter = |text| format!("{} - Spletka", text);

    view! { cx,
        <Styling/>
        <AddLinks/>
        <ChMeta/>
        <hr/>
        <Title formatter/>
        <main>
            <ChTitle/>
        </main>
    }
}

fn main() {
    mount_to_body(|cx| view! { cx, <App/> });
}

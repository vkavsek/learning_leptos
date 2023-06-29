use leptos::{ev::SubmitEvent, html::Input, *};

#[component]
fn App(cx: Scope) -> impl IntoView {
    let name_list = vec![
        "igor", "marko", "jani", "rudi", "toni", "franci", "anton", "luka", "matija", "steven",
        "majkolin",
    ];

    let names: Vec<_> = uppercase_conversion(&name_list);

    view! { cx,
        <CollectViewDemo names/>
        <IntoViewDemo/>
    }
}

#[component]
fn CollectViewDemo<IV>(cx: Scope, names: Vec<IV>) -> impl IntoView
where
    IV: IntoView,
{
    view! { cx,
        <h2>
            <code>"collect_view()"</code>
            " Demo"
        </h2>
        <ul>
            {names
                .into_iter()
                .map(|name| {
                    view! { cx, <li>{name}</li> }
                })
                .collect_view(cx)}
        </ul>
    }
}

#[component]
fn IntoViewDemo(cx: Scope) -> impl IntoView {
    let (toggle, set_toggle) = create_signal(cx, false);

    let click = move |_| set_toggle.update(|boolean| *boolean = !*boolean);
    view! { cx,
        <h2>
            <code>"into_view()"</code>
            " Demo"
        </h2>
        <button on:click=click>"Click to toggle browser input practice"</button>
        <p>"Button is toggled: " {move || if toggle() { "On" } else { "Off" }}</p>
        {
            move || {
                if toggle() {
                    view! {cx, 
                        <BrowserInputPractice/>
                    }.into_view(cx)
                } else {
                    ().into_view(cx)
                }
        }
        }
    }
}

#[component]
fn BrowserInputPractice(cx: Scope) -> impl IntoView {
    let (text, set_text) = create_signal(cx, "text".to_string());
    let input_elem = create_node_ref::<Input>(cx);

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let val = input_elem().expect("<input> to be present").value();
        set_text(val);
    };

    let num_chars = move || text().chars().count();
    let is_odd = move || num_chars() & 1 == 1;

    view! { cx,
        <h2>"Browser Controlled Input practice"</h2>
        <form on:submit=on_submit>
            <input type="text" value=text node_ref=input_elem/>
            <input type="submit" value="Submit"/>
        </form>
        <p>"Submited text is: " {text}</p>
        <p>"The number of chars is: " {num_chars}</p>
        <p>"Which is: " {move || if is_odd() { "Odd" } else { "Even" }}</p>
    }
}

fn main() {
    mount_to_body(|cx| view! { cx, <App/> });
}

fn uppercase_conversion(name_list: &Vec<&str>) -> Vec<String> {
    name_list
        .iter()
        .map(|&name| {
            let mut chars_iter = name.chars();
            let first_char = chars_iter
                .next()
                .expect("not to have empty Strings present")
                .to_ascii_uppercase();

            first_char.to_string() + &chars_iter.collect::<String>()
        })
        .collect()
}


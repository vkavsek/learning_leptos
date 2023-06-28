use leptos::*;

fn main() {
    mount_to_body(|cx| view! { cx, <App/> });
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let names = vec!["igor", "marko", "jani", "rudi", "toni", "franci", "anton"];

    let u_names: Vec<_> = names
        .iter()
        .map(|name| {
            let mut c = name.chars();
            if let Some(ch) = c.next() {
                let collected: String = c.collect();
                format!("{}{}", ch, collected);
            }
            name
        })
        .collect();

    view! {cx,
        <IntoViewDemo names/>
    }
}

#[component]
fn IntoViewDemo<IV>(cx: Scope, names: Vec<IV>) -> impl IntoView
where
    IV: IntoView,
{
    view! { cx,
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

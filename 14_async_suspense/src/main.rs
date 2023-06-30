use leptos::*;
use gloo_timers::future::TimeoutFuture;

async fn important_api_call(name: String) -> String {
    TimeoutFuture::new(1_000).await;
    name.to_ascii_uppercase()
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (name, set_name) = create_signal(cx, "Bill".to_string());

    // this will reload every time `name` changes
    let async_data = create_resource(
        cx,
        name,
        |name| async move { important_api_call(name).await },
    );

    view! { cx,
        <input
            on:input=move |ev| {
                set_name(event_target_value(&ev));
            }
            prop:value=name
        />
        <p><code>"name:"</code> {name}</p>
        <Suspense
            // the fallback will show whenever a resource
            // read "under" the suspense is loading
            fallback=move || view! { cx, <p>"Loading..."</p> }
        >
            // the children will be rendered once initially,
            // and then whenever any resources has been resolved
            <p>
                "Your shouting name is "
                {move || async_data.read(cx)}
            </p>
        </Suspense>
    }
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}

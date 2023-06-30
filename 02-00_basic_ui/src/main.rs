use leptos::*;

fn main() {
    mount_to_body(|cx| {
        view! { cx,
            <ProgrA initial_value=0/>
            <ProgrB/>
            <Dyn/>
        }
    })
}

/// One way to use variables as props
/// F: Fn() -> i32 + 'static
#[component]
fn ProgressBarA<F>(
    cx: Scope,
    /// Max value of the progress bar.
    #[prop(default = 100)]
    max: u16,
    /// How much of the progress bar should be displayed.
    progress: F,
) -> impl IntoView
where
    F: Fn() -> i32 + 'static,
{
    view! { cx, <progress max=max value=progress></progress> }
}
#[component]
pub fn ProgrA(cx: Scope, initial_value: i32) -> impl IntoView {
    let (value, set_value) = create_signal(cx, initial_value);
    let double_val = move || value() * 2;

    let clear = move |_| set_value(0);
    let decrement = move |_| set_value.update(|value| *value -= 1);
    let increment = move |_| set_value.update(|value| *value += 1);

    view! { cx,
        <div style="display: grid">
            <div class:red=move || value() % 2 == 1>
                <button on:click=clear>"Clear"</button>
                <button on:click=decrement>"-1"</button>
                <span>"Value: " {move || value().to_string()}</span>
                <button on:click=increment>"+1"</button>
            </div>
            <ProgressBarA progress=value/>
            <ProgressBarA progress=double_val/>
        </div>
    }
}

/// Another way to use variables as props
/// `#[prop(into)] name: Signal<i32>`
/// Note how you need to use Signal::derive() to wrap a derived signal
#[component]
fn ProgressBarB(
    cx: Scope,
    #[prop(default = 100)] max: u16,
    #[prop(into)] progress: Signal<i32>,
) -> impl IntoView {
    view! { cx, <progress max=max value=progress></progress> }
}
#[component]
fn ProgrB(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let double_count = move || count() * 2;

    // .into() converts ReadSignal to Signal
    // or use Signal::derive() to wrap a derived signal
    view! { cx,
        <div style="display: grid">
            <button on:click=move |_| {
                set_count.update(|n| *n += 1);
            }>"Click me"</button>
            <ProgressBarB progress=count/>
            <ProgressBarB progress=Signal::derive(cx, double_count)/>
        </div>
    }
}

#[component]
pub fn Dyn(cx: Scope) -> impl IntoView {
    let (x, set_x) = create_signal(cx, 0);
    let (y, set_y) = create_signal(cx, 0);

    let increment_x = move |_| set_x.update(|xval| *xval += 50);
    let decr_x = move |_| set_x.update(|xval| *xval -= 50);
    let increment_y = move |_| set_y.update(|yval| *yval += 50);
    let decr_y = move |_| set_y.update(|yval| *yval -= 50);

    view! { cx,
        <button on:click=increment_x>"+x"</button>
        <button on:click=decr_x>"-x"</button>
        <button on:click=increment_y>"+y"</button>
        <button on:click=decr_y>"-y"</button>
        <div
            style="position: absolute"
            style:left=move || format!("{}px", x() + 200)
            style:top=move || format!("{}px", y() + 200)
            style:background-color=move || format!("rgb({}, {}, 100)", x(), y())
            // is this needed? what does it do?
            style=("--columns", x)
        >
            "Moves when coords. change"
        </div>
    }
}

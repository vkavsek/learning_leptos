use leptos::*;

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}

/// Iteration is a very common task in most applications.
/// So how do you take a list of data and render it in the DOM?
/// This example will show you the two ways:
/// 1) for mostly-static lists, using Rust iterators
/// 2) for lists that grow, shrink, or move items, using ```<For/>```
#[component]
fn App(cx: Scope) -> impl IntoView {
    let val = vec![1, 2, 3, 4, 5];
    view! { cx,
        <h1>"Iteration"</h1>
        <h2>"Static Items and Static List"</h2>
        <p>"Everything is static"</p>
        <StItems val/>
        <h2>"Static List"</h2>
        <p>"Use this pattern if the list itself is static."</p>
        <StaticList length=5/>
        <h2>"Dynamic List"</h2>
        <p>"Use this pattern if the rows in your list will change."</p>
        <DynamicList initial_length=5/>
    }
}

/// Shows how to use vectors to render static lists, also demonstrates the use of collect_view() helper method
/// that allows you to collect any iterator of ```T: IntoView``` into ```Vec<View>``` otherwise you would have to use 
/// turbofish syntax with collect() method, this is demonstrated in the StaticList component.
#[component]
fn StItems<IV>(cx: Scope, val: Vec<IV>) -> impl IntoView
where
    IV: IntoView + Clone,
{
    view! { cx,
        <p>{val.clone()}</p>
        <ul>
            {val
                .into_iter()
                .map(|n| {
                    view! { cx, <li>{n}</li> }
                })
                .collect_view(cx)}
        </ul>
    }
}

/// A list of counters, without the ability to add or remove any.
/// You can still render the contents of the list dynamically.
/// This however is very inefficient as it rerenders everything everytime anything changes
#[component]
fn StaticList(
    cx: Scope,
    /// How many counters to include in this list.
    length: usize,
) -> impl IntoView {
    // create counter signals that start at incrementing numbers
    let counters = (1..=length).map(|idx| create_signal(cx, idx));

    // when you have a list that doesn't change, you can
    // manipulate it using ordinary Rust iterators
    // and collect it into a Vec<_> to insert it into the DOM
    let counter_buttons = counters
        .map(|(count, set_count)| {
            view! { cx,
                <li>
                    <button on:click=move |_| set_count.update(|n| *n += 1)>{count}</button>
                </li>
            }
        })
        .collect::<Vec<_>>();

    // Note that if `counter_buttons` were a reactive list
    // and its value changed, this would be very inefficient:
    // it would rerender every row every time the list changed.
    view! { cx, <ul>{counter_buttons}</ul> }
}

/// A list of counters that allows you to add or
/// remove counters.
#[component]
fn DynamicList(
    cx: Scope,
    /// The number of counters to begin with.
    initial_length: usize,
) -> impl IntoView {
    // This dynamic list will use the <For/> component.
    // <For/> is a keyed list. This means that each row
    // has a defined key. If the key does not change, the row
    // will not be re-rendered. When the list changes, only
    // the minimum number of changes will be made to the DOM.

    // `next_counter_id` will let us generate unique IDs
    // we do this by simply incrementing the ID by one
    // each time we create a counter
    let mut next_counter_id = initial_length;

    // we generate an initial list as in <StaticList/>
    // but this time we include the ID along with the signal
    let initial_counters = (0..initial_length)
        .map(|id| (id, create_signal(cx, id + 1)))
        .collect::<Vec<_>>();

    // now we store that initial list in a signal
    // this way, we'll be able to modify the list over time,
    // adding and removing counters, and it will change reactively
    let (counters, set_counters) = create_signal(cx, initial_counters);

    let add_counter = move |_| {
        // create a signal for the new counter
        let sig = create_signal(cx, next_counter_id + 1);
        // add this counter to the list of counters
        set_counters.update(move |counters| {
            // since `.update()` gives us `&mut T`
            // we can just use normal Vec methods like `push`
            counters.push((next_counter_id, sig))
        });
        // increment the ID so it's always unique
        next_counter_id += 1;
    };

    // The <For/> component is central here
    // This allows for efficient, key list rendering
    //
    // `each` takes any function that returns an iterator
    // this should usually be a signal or derived signal
    // if it's not reactive, just render a Vec<_> instead of <For/>
    //
    // the key should be unique and stable for each row
    // using an index is usually a bad idea, unless your list
    // can only grow, because moving items around inside the list
    // means their indices will change and they will all rerender
    // the view function receives each item from your `each` iterator
    // and returns a view
    view! { cx,
        <div>
            <button on:click=add_counter>"Add Counter"</button>
            <ul>
                <For
                    each=counters
                    key=|counter| counter.0
                    view=move |cx, (id, (count, set_count))| {
                        view! { cx,
                            <li>
                                <button on:click=move |_| set_count.update(|n| *n += 1)>{count}</button>
                                <button on:click=move |_| {
                                    set_counters
                                        .update(|counters| { counters.retain(|(counter_id, _)| counter_id != &id) });
                                }>"Remove"</button>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

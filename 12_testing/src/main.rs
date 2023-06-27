use leptos::*;

// The easiest way to do testing is to create a wrapping type and test like normally with Rust.
// Another way is to use the wasm-bindgen-test.
// Refer to the book to find out more.
//
// In many cases, it makes sense to pull the logic out of your components and test it separately.
// For some simple components, there’s no particular logic to test, but for many it’s worth using
// a testable wrapping type and implementing the logic in ordinary Rust impl blocks.
// This example demonstrates how to do this.

fn main() {
    mount_to_body(|cx| view! {cx, <TodoApp/>})
}

#[component]
pub fn TodoApp(cx: Scope) -> impl IntoView {
    let (todos, set_todos) = create_signal(cx, Todos(vec![Todo { completed: false }]));
    // ✅ this has a test associated with it.
    let num_remaining = move || todos.with(Todos::num_remaining);

    let add_true = move |_| set_todos.update(|f| f.0.push(Todo { completed: true }));
    let add_false = move |_| set_todos.update(|f| f.0.push(Todo { completed: false }));

    create_effect(cx, move |_| {
        log!("Remaining to complete in Todos is: {:#?}", num_remaining());
        log!("Tasks completed: {:#?}", todos().0);
        }
    );


    view! {cx,
        <p>"Is the task finished?"</p>
        <button on:click=add_true>"True"</button>
        <button on:click=add_false>"False"</button>
        <p>"Tasks remaining: "{num_remaining}</p>
    }
}

#[derive(Clone)]
pub struct Todos(Vec<Todo>);

// We can test the functions implemented in here!
impl Todos {
    pub fn new(v: Vec<bool>) -> Self {
        Self {
            0: v.into_iter()
                .map(|boolean| Todo { completed: boolean })
                .collect(),
        }
    }

    pub fn num_remaining(&self) -> usize {
        self.0.iter().filter(|todo| !todo.completed).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_remaining() {
        let td = Todos::new(vec![true, false, false, true, true, false]);

        assert_eq!(td.num_remaining(), 3);
    }
}

#[derive(Debug, Clone)]
pub struct Todo {
    pub completed: bool,
}

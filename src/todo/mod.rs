use std::any;

use anyhow::Result;
use leptos::*;
use libsql_client::Row;


#[derive(Debug, Clone)]
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub extras: String,
    pub completed: bool,
}

#[component]
pub fn TodoForm(cx: Scope, todos: Vec<Todo>, route: &'static str) -> impl IntoView {
    // create user interfaces with the declarative `view!` macro
    return view! { cx,
        <form hx-post="{route}"
            hx-target="#todos"
            hx-swap="afterbegin"
            hx-trigger="submit">
            <input type="text" placeholder="More like Todone, amirite?" />
            <button type="submit">"add me daddy"</button>
            <Todos todos=todos />
        </form>
    };
}

#[component]
pub fn Todos(cx: Scope, todos: Vec<Todo>) -> impl IntoView {
    let (todos, _) = create_signal::<Vec<Todo>>(cx, todos);

    // create user interfaces with the declarative `view!` macro
    return view! { cx,
        <ul id="todos">
            <For

                // a function that returns the items we're iterating over; a signal is fine
                each=move || todos.get()

                // a unique key for each item

                key=|todo| todo.id

                // renders each item to a view
                view=move |cx, todo: Todo| {
                    view! {
                        cx,
                        <Todo todo=todo />
                    }
                }
            />
        </ul>
    };
}

#[component]
pub fn Todo(cx: Scope, todo: Todo) -> impl IntoView {
    return view! {cx,
        <div>
            <div>id: {todo.id}</div>
            <div>title: {todo.title}</div>
            <div>completed: {todo.completed}</div>
        </div>
    };
}

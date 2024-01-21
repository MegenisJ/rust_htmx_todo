use leptos::*;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub extras: String,
    pub completed: bool,
}

#[derive(Deserialize)]
pub struct NewTodo {
    pub title: String,
    pub extras: String
}

#[component]
pub fn TodoForm(cx: Scope, todos: Vec<Todo>, route: &'static str) -> impl IntoView {
    // create user interfaces with the declarative `view!` macro
    return view! { cx,
            <form hx-post="{route}"
                hx-target="#todos"
                hx-swap="afterbegin"
                hx-trigger="submit"
                class = "mx-auto mt-10 grid max-w-2xl gap-x-4 gap-y-4 border-t border-gray-200 pt-10 sm:mt-16 sm:pt-16 lg:mx-0 lg:max-w-none lg:grid-cols-4" >
                <h3>Create a new todo item</h3>
                <input name = "title" type="text" placeholder="Title" />
                <input name = "extras" type="text" placeholder="Detail" />
                <button type="submit">"Add new todo"</button>
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
    let id = todo.id;
    return view! {cx,
        <li class = "mx-auto mt-10 grid max-w-2xl grid-cols-1 gap-x-8 gap-y-16 border-t border-gray-200 pt-10 sm:mt-16 sm:pt-16 lg:mx-0 lg:max-w-none lg:grid-cols-3">
            <div>title: {todo.title}</div>
            <div>extra detail: {todo.extras}</div>
            <div>completed: {todo.completed}</div>

            <button type="submit"              
            hx-patch="todo/status/{id}"
            hx-target="closest li"
            hx-swap="innerHTML"
            class = "border rounded bg-violet-500 hover:bg-violet-600"
            >"Mark as complete"</button>
            
            <button type="submit"              
            hx-delete="todo/{id}"
            hx-target="closest li"
            hx-swap="delete outerHTML"
            class = "border rounded bg-red-500 hover:bg-red-600"
            >"Remove"</button>
        </li>
    };
}

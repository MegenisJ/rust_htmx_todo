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
                class = "p-8 w-auto mx-auto bg-white rounded-xl shadow-lg flex items-center space-x-8" >
                <h3>Create a new todo item</h3>
                <input name = "title" type="text" placeholder="Title" class = "border rounded"/>
                <input name = "extras" type="text" placeholder="Detail" class = "border rounded"/>
                <button type="submit" class = "border rounded bg-gray-100 hover:bg-gray-200 w-auto">"Add new todo"</button>

                <button 
                    hx-post = "/removeall"
                    hx-target="#todos"
                    hx-swap="innerHTML"
                    class = "bg-red-500 hover:bg-red-600 border rounded w-auto">
                        Remove all 
                </button>
            
            </form>
            <Todos todos=todos />
    };
}

#[component]
pub fn Todos(cx: Scope, todos: Vec<Todo>) -> impl IntoView {
    let (todos, _) = create_signal::<Vec<Todo>>(cx, todos);

    // create user interfaces with the declarative `view!` macro
    return view! { cx,
        <ul id="todos" class = "flex justify-items-center mx-auto">
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
    <li class = "p-6 max-w-sm mx-auto bg-white rounded-xl shadow-lg flex items-center space-x-4">


        <CompletedCheckbox todo=todo.clone()/>
        
        <p class="text-xl font-medium text-black">{todo.title.clone()}</p>
        <p class="text-slate-500">{todo.extras.clone()}</p>
        


        <button type="submit"              
            hx-delete="todo/{id}"
            hx-target="closest li"
            hx-swap="delete outerHTML"
            class = "border rounded bg-gray-100 hover:bg-gray-200 w-auto"
            >"Remove"</button>
        
        </li>
    };
}

#[component]
pub fn CompletedCheckbox(cx: Scope, todo: Todo) -> impl IntoView {

    let id = todo.id;
    if todo.completed {

        return view! {cx,
            <input 
                value="completed" 
                type="checkbox" 
                hx-patch="/todo/status/{id}" 
                hx-trigger="click"
                checked 
            /> 
        };

    }

    return view! {cx,
            <input 
                value="completed" 
                type="checkbox" 
                hx-patch="/todo/status/{id}" 
                hx-trigger="click"
            /> 
    };
}


mod todo;
mod app_state;

use std::fs;
use actix_web::web::Data;
use actix_web::{get, post, web, App,HttpRequest, HttpResponse, HttpServer, Responder, web::Form};
use todo::{Todo, TodoForm, Todos};
use leptos::*;
use app_state::*;
use serde::Deserialize;


#[get("/")]
async fn hello(_req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    let todos = data.get_all_todos().await;
     
    let html = leptos::ssr::render_to_string(move |cx| {
        view! { cx,
            <head>
                <script src="https://unpkg.com/htmx.org@1.9.2" integrity="sha384-L6OqL9pRWyyFU3+/bjdSri+iIphTN/bvYyM37tICVyOJkWZLpP2vGn6VUEXgzg6h" crossorigin="anonymous"></script>
            </head>
            <body>
                <TodoForm
                    route="/test"
                    todos = todos
                />
            </body>
        }
    });
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[derive(Deserialize)]
struct NewTodo {
    title: String,
    extras: String
}

#[post("/test")]
async fn test(Form(form): Form<NewTodo>, data: web::Data<AppState>) -> impl Responder {

    println!("{}{}", form.title, form.extras);

    data.save_todo(Todo{id:0,title: form.title,extras: form.extras,completed:false }).await;
    let todos = data.get_all_todos().await; 
    let html = leptos::ssr::render_to_string(move |cx| {
        view! { cx,
                <Todos todos = todos
                />
        }
    });
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let app_state: Data<AppState> = initialize_app_state().await; 
    app_state.get_all_todos().await;
    app_state.get_all_todos();
    app_state.save_todo(Todo{id:0,title: "new title".to_string(),extras:"new extras".to_string() ,completed:false }).await;

    app_state.get_all_todos().await;

    println!("starting HTTP server at http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(hello)
            .service(test)
    })
    .bind(("127.0.0.1", 8080))?

    .run()
    .await
}

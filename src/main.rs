mod task;
mod todo;
mod app_state;

use std::{fs, string};
use actix_web::web::Data;
use actix_web::{get, post, web, App,HttpRequest, HttpResponse, HttpServer, Responder};
use libsql_client::{Client, Config, Value,Row};
use std::sync::Arc;
use todo::{Todo, TodoForm};
use leptos::*;
use app_state::*;

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
#[post("/test")]
async fn test(_req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    
    data.save_todo(Todo{id:0,title: "testy".to_string(), extras:"extra stuff".to_string(),completed:false }).await;
    let index = fs::read_to_string("src/contents.html").unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(index)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let app_state: Data<AppState> = initialize_app_state().await; 
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

mod task;
mod todo;
mod appState;

use std::{fs, string};
use actix_web::{get, post, web, App,HttpRequest, HttpResponse, HttpServer, Responder};
use libsql_client::{Client, Config, Value,Row};
use std::sync::Arc;
use task::TodoForm;
use task::{Todo, TodoItem};
use leptos::*;


#[get("/")]
async fn hello(_req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    let todos = data
        .client
        .execute("SELECT * FROM todos").await.unwrap()
        .rows
        .iter()
        .filter_map(|x| TodoItem::try_from(x.clone()).ok())
        .collect::<Vec<_>>();
    println!("todos: {:?}", todos);
     
    let html = leptos::ssr::render_to_string(move |cx| {
        view! { cx,
            <head>
                <script src="https://unpkg.com/htmx.org@1.9.2" integrity="sha384-L6OqL9pRWyyFU3+/bjdSri+iIphTN/bvYyM37tICVyOJkWZLpP2vGn6VUEXgzg6h" crossorigin="anonymous"></script>
            </head>
            <body>
                <TodoForm
                    route="/test"
                    todos=vec![
                        TodoItem { id: 0, title: "hello".to_string(), extras: "something extra".to_string(), completed: false },
                        TodoItem { id: 1, title: "world".to_string(), extras: "something extra".to_string(),completed: true },
                    ]
                />
            </body>
        }
    });
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
#[post("/test")]
async fn test(data: web::Data<AppState>) -> impl Responder {
    let what_comes_back = data
        .client
        .execute("INSERT INTO todos (title,detail,completed) VALUES ('So something','a bit of extra detail', 0) RETURNING id")
        .await
        .unwrap();

    let index = fs::read_to_string("src/contents.html").unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(index)
}

fn get_url() -> String {
    let file = "/tmp/example.db".to_string();

    if file.starts_with("file://") {
        return file;
    }

    return format!("file://{}", file);
}
async fn get_count(client: Arc<Client>) -> std::io::Result<usize> {
    let result_set = client.execute("SELECT * FROM todos").await.unwrap();

    let count = result_set
        .rows
        .first()
        .map(|row| &row.values[0])
        .unwrap_or(&Value::Integer { value: 0 });

    let count = match count {
        Value::Integer { value: i } => *i,
        _ => 0,
    };

    if count > 0 {
        let row = &result_set.rows[0]; // ResultSet returns array of Rows
        let text : &str = row.try_column("title").unwrap();
        let detail : &str = row.try_column("detail").unwrap();
        let completed : usize = row.try_column("completed").unwrap();
        println!("{text}{detail}{completed}");
    }
    return Ok(count as usize);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let config = Config::new(get_url().as_str()).unwrap();
    let client = Arc::new(libsql_client::Client::from_config(config).await.unwrap());

    client.execute("CREATE TABLE IF NOT EXISTS todos (id INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT, detail TEXT, completed BOOLEAN)").await.unwrap();

    println!("count {}", get_count(client.clone()).await?);
    
    let app_state = web::Data::new(AppState { client });

    println!("starting HTTP server at http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(hello)
            .service(echo)
            .service(test)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

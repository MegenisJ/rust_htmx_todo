mod todo;
mod app_state;

use actix_web::web::Data;
use actix_web::{get, post,delete,patch, web, App,HttpRequest, HttpResponse, HttpServer, Responder, web::Form};
use todo::{Todo, TodoForm, Todos, NewTodo};
use leptos::*;
use leptos::ssr::render_to_string;
use app_state::*;

#[get("/")]
async fn index(_req: HttpRequest, data: web::Data<AppState>) -> impl Responder {

    let result_set = data.client.execute("SELECT * FROM todos2").await.unwrap();
    
    let mut all_todos:Vec<Todo> = vec![];

    let count = result_set.rows.len();

    if count > 0 {
        let mut x = 0;
        while x < count{
            let row = &result_set.rows[x as usize]; // ResultSet returns array of Rows
            let todo = Todo{
                id :  row.try_column("id").unwrap(),
                title :  row.try_column::<&str>("title").unwrap().to_string(),
                extras :  row.try_column::<&str>("detail").unwrap().to_string(),
                completed : row.try_column("completed").unwrap_or(0) == 1,
            };
            all_todos.push(todo);
            x+=1;
        }
    }

    let html = render_to_string(move |cx| {
        view! { cx,
            <head>
                <script src="https://unpkg.com/htmx.org@1.9.2" integrity="sha384-L6OqL9pRWyyFU3+/bjdSri+iIphTN/bvYyM37tICVyOJkWZLpP2vGn6VUEXgzg6h" crossorigin="anonymous"></script>
                <script src="https://cdn.tailwindcss.com"></script>
            </head>
            <body>
                <TodoForm
                    route="/add"
                    todos = all_todos
                />

            </body>
        }
    });

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[delete("todo/{id}")]
async fn remove(id: web::Path<usize>, data: web::Data<AppState>) -> impl Responder {
    println!("recieved remove request");
    
    let query = format!(
        "DELETE FROM todos2 WHERE id = '{}'"
        ,id.into_inner());

    let _ = data.client.execute(query).await;
   
    HttpResponse::Ok()
}

#[patch("todo/status/{id}")]
async fn update_status( id: web::Path<usize>, data: web::Data<AppState>) -> impl Responder {
    println!("recieved complete request");
    
    let id = id.into_inner();
    let query = format!(
        "update todos2 set completed = NOT completed WHERE id = '{}' RETURNING id,title,detail,completed"
        ,id);

    let result_set=  data.client.execute(query).await.unwrap();

    let row = &result_set.rows[0]; 
   
    let todo = Todo{
        id :  row.try_column("id").unwrap(),
        title :  row.try_column::<&str>("title").unwrap().to_string(),
        extras :  row.try_column::<&str>("detail").unwrap().to_string(),
        completed : row.try_column("completed").unwrap_or(0) == 1,
    };

    let html = render_to_string(move |cx| {
        view! { cx,
        <Todo todo=todo/>
        }
    });
   
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
#[post("/add")]
async fn add(Form(form): Form<NewTodo>, data: web::Data<AppState>) -> impl Responder {
    let query = format!(
        "INSERT INTO todos2 (title,detail,completed) VALUES ('{}','{}', 0) RETURNING id"
        ,form.title,form.extras);

    let response =  data.client.execute(query).await;
    let x = response.unwrap().rows[0].try_column("id").unwrap();
    let new_todo = Todo{
        id: x,
        title:form.title,
        extras:form.extras,
        completed:false
    };    

    let html = render_to_string(move |cx| {
        view! { cx,
            <Todo todo=new_todo />
        }
    });
   
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[post("/removeall")]
async fn remove_all(data: web::Data<AppState>) -> impl Responder {

    let _=  data.client.execute("delete from todos2").await;

    let empty_todos:Vec<Todo> = vec![];
    
    let html = leptos::ssr::render_to_string(move |cx| {
        view! { cx,
            <Todos todos=empty_todos />
        }
    });

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let app_state: Data<AppState> = initialize_app_state().await; 
    println!("starting HTTP server at http://127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(index)
            .service(add)
            .service(remove_all)
            .service(remove)
            .service(update_status)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


use crate::todo::*;
use std::sync::Arc;
use libsql_client::{Client, Config, Value,Row};
use actix_web::{get, post, web, App,HttpRequest, HttpResponse, HttpServer, Responder};

#[derive(Clone)]
pub struct AppState {
    client: Arc<Client>,
}

impl AppState {
    pub async fn get_all_todos(&self) -> Vec<Todo> {

        let result_set = self.client.execute("SELECT * FROM todos").await.unwrap();

        let mut all_todos:Vec<Todo> = vec![];
        
        let count = result_set
            .rows
            .first()
            .map(|row| &row.values[0])
            .unwrap_or(&Value::Integer { value: 0 });

        let count = match count {
            Value::Integer { value: i } => *i,
            _ => 0,
        };
        println!("total todos : {count}");
        if count > 0 {
            let mut x = 0;
            while x <= count{

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
        return all_todos;
    }
/*
    pub fn CompleteTodo(todo:Todo) -> bool
    {

    }

    pub fn ReinstateTodo(todo:Todo) -> bool
    {

    }
*/
    pub async fn save_todo(&self, todo:Todo) {
        println!("saving new todo {0}", todo.title);
        let title = todo.title;
        let extras = todo.extras;
        let query = format!("INSERT INTO todos (title,detail,completed) VALUES ('{title}','{extras}', 0) RETURNING id");
        let x =self.client.execute(query).await.unwrap();
        println!("{}",x.rows_affected);
    }
/*
    pub fn DeleteTodo(todo:Todo) -> bool {

    }
*/
}

pub async fn initialize_app_state() -> web::Data<AppState> {

    let config = Config::new(get_url().as_str()).unwrap();
    let client = Arc::new(libsql_client::Client::from_config(config).await.unwrap());

    client.execute("CREATE TABLE IF NOT EXISTS todos (id INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT, detail TEXT, completed BOOLEAN)").await.unwrap();
    
    let app = web::Data::new(AppState { client });
    return app;
}

fn get_url() -> String {
    let file = "/tmp/example.db".to_string();

    if file.starts_with("file://") {
        return file;
    }

    return format!("file://{}", file);
}

use crate::todo::*;
use std::sync::Arc;
use libsql_client::{Client, Config, Value,Row};

#[derive(Clone)]
pub struct AppState {
    client: Arc<Client>,
}

impl AppState {
    

    fn GetAllTodos() -> Vec<Todo> {

    }

    fn CompleteTodo(todo:Todo) -> bool
    {

    }
    
    fn ReinstateTodo(todo:Todo) -> bool
    {

    }

    fn SaveTodo(todo:Todo) -> bool {

    }

    fn DeleteTodo(todo:Todo) -> bool {

    }
}

async fn initialize_app_state() -> AppState {

    let config = Config::new(get_url().as_str()).unwrap();
    let client = Arc::new(libsql_client::Client::from_config(config).await.unwrap());

    client.execute("CREATE TABLE IF NOT EXISTS todos (id INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT, detail TEXT, completed BOOLEAN)").await.unwrap();

    let app_state = web::Data::new(AppState { client });
    return app_state;
}

fn get_url() -> String {
    let file = "/tmp/example.db".to_string();

    if file.starts_with("file://") {
        return file;
    }

    return format!("file://{}", file);
}

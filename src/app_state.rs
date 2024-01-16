use crate::todo::*;
use std::sync::Arc;
use libsql_client::{Client, Config, Value};
use actix_web::web;

#[derive(Clone)]
pub struct AppState {
    pub client: Arc<Client>,
}

impl AppState {
    pub async fn get_all_todos(&self) -> Vec<Todo> {

        let result_set = self.client.execute("SELECT * FROM todos2").await.unwrap();

        let mut all_todos:Vec<Todo> = vec![];
        
        let count = result_set.rows_affected;

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
        return all_todos;
    }
/*
    pub fn CompleteTodo(todo:Todo) INSERT INTO todos (title,detail,completed) VALUES ('{title}','{extras}', 0)-> bool

    {

    }

    pub fn ReinstateTodo(todo:Todo) -> bool
    {

    }
    */
    pub async fn save_todo(&self, todo:Todo) {
        println!("saving new todo {0}", todo.title);

        let query = format!(
            "INSERT INTO todos2 (title,detail,completed) VALUES ('{}','{}', 0)"
            ,todo.title,todo.extras);

        let x =self.client.execute(query).await.unwrap();

        let query2 = format!("select * from todos2");

        let x2 =self.client.execute(query2).await.unwrap();
        

        let new_id = x2.rows[0].try_column("title").unwrap_or("something went wrong");
        println!("{}", new_id);
        println!("rows affected{}",x.rows_affected);
    }
    /*
       pub fn DeleteTodo(todo:Todo) -> bool {

       }
       */

    pub async fn initialize_db(&self){
        self.client.execute("CREATE TABLE IF NOT EXISTS todos2 (id INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT, detail TEXT, completed BOOLEAN)").await.unwrap();
    }
}
pub async fn initialize_app_state() -> web::Data<AppState> {

    let config = Config::new(get_url().as_str()).unwrap();
    let client = Arc::new(libsql_client::Client::from_config(config).await.unwrap());


    client.execute("CREATE TABLE IF NOT EXISTS todos2 (id INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT, detail TEXT, completed BOOLEAN)").await.unwrap();

    let query = format!(
        "INSERT INTO todos2 (title,detail,completed) VALUES ('{}','{}', 0)"
        ,"test data 1","test data 2");

    let x = client.execute(query).await.unwrap();

    let query2 = format!("select * from todos2");


    let x2 = client.execute(query2).await.unwrap();

    let new_id = x2.rows[0].try_column("title").unwrap_or("something went wrong");
    println!("print new row {}", new_id);
    let app = web::Data::new(AppState { client });
    return app;
}

pub async fn get_all_todos(client: &Arc<Client>) -> Vec<Todo> {

    let result_set = client.execute("SELECT * FROM todos2").await.unwrap();

    let mut all_todos:Vec<Todo> = vec![];

    let count = result_set.rows_affected;

    println!("total todos : {count}");

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
    return all_todos;
}
fn get_url() -> String {
    let file = "/tmp/example.db".to_string();

    if file.starts_with("file://") {
        return file;
    }

    return format!("file://{}", file);
}

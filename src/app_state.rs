use crate::todo::*;
use std::sync::Arc;
use libsql_client::{Client, Config};
use actix_web::web;

#[derive(Clone)]
pub struct AppState {
    pub client: Arc<Client>,
}

pub async fn initialize_app_state() -> web::Data<AppState> {

    let config = Config::new(get_url().as_str()).unwrap();
    let client = Arc::new(libsql_client::Client::from_config(config).await.unwrap());

    client.execute("CREATE TABLE IF NOT EXISTS todos2 (id INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT, detail TEXT, completed BOOLEAN)").await.unwrap(); 

    return web::Data::new(AppState { client });

}

fn get_url() -> String {
    let file = "/tmp/example.db".to_string();

    if file.starts_with("file://") {
        return file;
    }

    return format!("file://{}", file);
}

use reqwest::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Todo {
    userId: u32,
    id: u32,
    title: String,
    completed: bool,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "https://jsonplaceholder.typicode.com/todos/1";

    let todo: Todo = reqwest::get(url)
        .await?
        .json()
        .await?;

    println!("{:#?}", todo);

    Ok(())
}

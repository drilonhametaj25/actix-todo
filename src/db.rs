use crate::models::{TodoList, TodoItem};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use std::io;


pub async fn get_todos(client: &Client ) -> Result<Vec<TodoList>, io::Error> {

    let statement = client.prepare("select * from todo_list").await.unwrap();

    let todos = client.query(&statement, &[])
        .await
        .expect("Error getting to do lists")
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>();

    Ok(todos)
}

pub async fn get_items(client: &Client, list_id: i32) -> Result<Vec<TodoItem>, io::Error> {

    let statement = client.prepare("select * from todo_items where list_id = $1").await.unwrap();

    let todos = client.query(&statement, &[&list_id])
        .await
        .expect("Error getting to do items")
        .iter()
        .map(|row| TodoItem::from_row_ref(row).unwrap())
        .collect::<Vec<TodoItem>>();

    Ok(todos)
}


pub async fn create_todo(client: &Client, title: String) -> Result<TodoList, io::Error> {

    let statement = client.prepare("insert into todo_list (titlte) values ($1) returning id, title").await.unwrap();

    let todos = client.query(&statement, &[&title])
        .await
        .expect("Error getting to do items")
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>()
        .pop()
        .ok_or(io::Error::new(io::ErrorKing::Other, "Error creating todo list"))

}

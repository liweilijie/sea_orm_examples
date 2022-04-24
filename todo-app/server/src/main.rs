mod fruits_list_table;
mod insert_values;
mod routing;
mod todo_list_table;

use axum::routing::{get, post};
use axum::Router;
pub use fruits_list_table::prelude::*;
pub use routing::*;
use std::net::SocketAddr;
pub use todo_list_table::prelude::*;

use crate::insert_values::insert_fruits;
use dotenv::dotenv;
use once_cell::sync::OnceCell;
use sea_orm::sea_query::{Alias, ColumnDef, Table};
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbBackend};

static DATABASE_CONNECTION: OnceCell<DatabaseConnection> = OnceCell::new();

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Define the database backend
    let db_postgres = DbBackend::Postgres;

    dotenv().ok();

    // Read the database environment from the .env file
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(database_url).await?;
    DATABASE_CONNECTION.set(db).unwrap();

    println!("set db ok.");

    // Create the fruits table
    let fruits_table = Table::create()
        .table(Alias::new("fruits"))
        .if_not_exists()
        .col(
            ColumnDef::new("fruit_id")
                .integer()
                .auto_increment()
                .primary_key()
                .not_null(),
        )
        .col(
            ColumnDef::new("fruit_name")
                .string()
                .unique_key()
                .not_null(),
        )
        .to_owned();

    let db = DATABASE_CONNECTION.get().unwrap();

    // Executing the SQL query to create the `fruits_table` in PostgreSQL
    let create_table_op = db.execute(db_postgres.build(&fruits_table)).await;
    // Print the result in a user friendly way
    println!(
        "`CREATE TABLE fruits` {:?}",
        match create_table_op {
            Ok(_) => "Operation successful".to_owned(),
            Err(e) => format!("Unsuccessful - Error : {}", e),
        }
    );

    // Create the `todos` table
    let todos_table = Table::create()
        .table(Alias::new("todos"))
        .if_not_exists()
        .col(
            ColumnDef::new("todo_id")
                .integer()
                .auto_increment()
                .primary_key()
                .not_null(),
        )
        .col(ColumnDef::new("username").string().unique_key().not_null())
        .col(ColumnDef::new("todo_list").string())
        .to_owned();

    // Executing the SQL query to create the `fruits_table` in PostgreSQL
    let create_table_op = db.execute(db_postgres.build(&todos_table)).await;
    // Print the result in a user friendly way
    println!(
        "`CREATE TABLE fruits` {:?}",
        match create_table_op {
            Ok(_) => "Operation successful".to_owned(),
            Err(e) => format!("Unsuccessful - Error : {}", e),
        }
    );

    insert_fruits(&db).await?;

    let app = Router::new()
        .route("/", get(root))
        .route("/fruits", get(get_fruits))
        .route("/get_user", post(get_user))
        .route("/store", post(store_todo))
        .route("/update_todo", post(update_todo));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

#![forbid(unsafe_code)]
#![forbid(clippy::allow_attributes)]
#![deny(clippy::pedantic)]

use anyhow::Result;
use postgresql_embedded::PostgreSQL;
use sqlx::postgres::PgPool;
use sqlx::Row;

#[tokio::main]
async fn main() -> Result<()> {
    let mut postgresql = PostgreSQL::default();
    postgresql.setup().await?;
    postgresql.start().await?;

    let database_name = "test";
    postgresql.create_database(database_name).await?;
    let settings = postgresql.settings();
    let database_url = settings.url(database_name);

    let pool = PgPool::connect(database_url.as_str()).await?;

    println!("Creating table 'todos'");
    create_table_todo(&pool).await?;

    let description = "Implement embedded database with sqlx";
    println!("Adding new todo with description '{description}'");
    let todo_id = add_todo(&pool, description).await?;
    println!("Added new todo with id {todo_id}");

    println!("Marking todo {todo_id} as done");
    if complete_todo(&pool, todo_id).await? {
        println!("Todo {todo_id} is marked as done");
    }

    println!("Printing list of all todos");
    list_todos(&pool).await?;

    Ok(())
}

async fn create_table_todo(pool: &PgPool) -> Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS todos(id BIGSERIAL PRIMARY KEY, description TEXT NOT NULL, done BOOLEAN NOT NULL DEFAULT FALSE);"
    ).execute(pool).await?;

    Ok(())
}

async fn add_todo(pool: &PgPool, description: &str) -> Result<i64> {
    let rec = sqlx::query("INSERT INTO todos (description) VALUES ($1) RETURNING id")
        .bind(description)
        .fetch_one(pool)
        .await?;

    let id: i64 = rec.get("id");
    Ok(id)
}

async fn complete_todo(pool: &PgPool, id: i64) -> Result<bool> {
    let rows_affected = sqlx::query("UPDATE todos SET done = TRUE WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?
        .rows_affected();

    Ok(rows_affected > 0)
}

async fn list_todos(pool: &PgPool) -> Result<()> {
    let recs = sqlx::query("SELECT id, description, done FROM todos ORDER BY id")
        .fetch_all(pool)
        .await?;

    for rec in recs {
        let id: i64 = rec.get("id");
        let description: String = rec.get("description");
        let done: bool = rec.get("done");
        println!(
            "- [{}] {}: {}",
            if done { "x" } else { " " },
            id,
            &description,
        );
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_main() -> Result<()> {
        main()
    }
}

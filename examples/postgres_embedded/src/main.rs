#![forbid(unsafe_code)]
#![forbid(clippy::allow_attributes)]
#![deny(clippy::pedantic)]

use anyhow::Result;
use postgres::{Client, NoTls};
use postgresql_embedded::blocking::PostgreSQL;

fn main() -> Result<()> {
    let mut postgresql = PostgreSQL::default();
    postgresql.setup()?;
    postgresql.start()?;

    let database_name = "test";
    postgresql.create_database(database_name)?;
    let settings = postgresql.settings();
    let mut client = Client::connect(
        format!(
            "host={host} port={port} user={username} password={password}",
            host = settings.host,
            port = settings.port,
            username = settings.username,
            password = settings.password
        )
        .as_str(),
        NoTls,
    )?;

    println!("Creating table 'todos'");
    create_table_todo(&mut client)?;

    let description = "Implement embedded database with postgres";
    println!("Adding new todo with description '{description}'");
    let todo_id = add_todo(&mut client, description)?;
    println!("Added new todo with id {todo_id}");

    println!("Marking todo {todo_id} as done");
    if complete_todo(&mut client, todo_id)? {
        println!("Todo {todo_id} is marked as done");
    }

    println!("Printing list of all todos");
    list_todos(&mut client)?;

    Ok(())
}

fn create_table_todo(client: &mut Client) -> Result<()> {
    let _ = client.execute(
        "CREATE TABLE IF NOT EXISTS todos (id BIGSERIAL PRIMARY KEY, description TEXT NOT NULL, done BOOLEAN NOT NULL DEFAULT FALSE);",
        &[],
    )?;

    Ok(())
}

fn add_todo(client: &mut Client, description: &str) -> Result<i64> {
    let row = client.query_one(
        "INSERT INTO todos (description) VALUES ($1) RETURNING id",
        &[&description],
    )?;

    let id: i64 = row.get(0);
    Ok(id)
}

fn complete_todo(client: &mut Client, id: i64) -> Result<bool> {
    let rows_affected = client.execute("UPDATE todos SET done = TRUE WHERE id = $1", &[&id])?;

    Ok(rows_affected > 0)
}

fn list_todos(client: &mut Client) -> Result<()> {
    let rows = client.query("SELECT id, description, done FROM todos ORDER BY id", &[])?;

    for rec in rows {
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

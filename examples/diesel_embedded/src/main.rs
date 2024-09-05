#![forbid(unsafe_code)]
#![forbid(clippy::allow_attributes)]
#![deny(clippy::pedantic)]

use crate::models::{NewPost, Post};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{PgConnection, RunQueryDsl, SelectableHelper};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use postgresql_embedded::{PostgreSQL, Result, Settings, VersionReq};

mod models;
pub mod schema;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations/");
#[tokio::main]
async fn main() -> Result<()> {
    let settings = Settings {
        version: VersionReq::parse("=16.4.0")?,
        username: "postgres".to_string(),
        password: "postgres".to_string(),
        ..Default::default()
    };
    let mut postgresql = PostgreSQL::new(settings);
    postgresql.setup().await?;
    postgresql.start().await?;

    let database_name = "diesel_demo";
    postgresql.create_database(database_name).await?;
    postgresql.database_exists(database_name).await?;

    {
        let database_url = postgresql.settings().url(database_name);
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .test_on_check_out(true)
            .build(manager)
            .expect("Could not build connection pool");
        let mut mig_run = pool.clone().get().unwrap();
        mig_run.run_pending_migrations(MIGRATIONS).unwrap();

        let post = create_post(
            &mut pool.get().unwrap(),
            "My First Post",
            "This is my firs post",
        );
        println!("Post '{}' created", post.title);
    }

    postgresql.drop_database(database_name).await?;

    postgresql.stop().await
}

/// Create a new post
///
/// # Panics
/// if the post cannot be saved
pub fn create_post(conn: &mut PgConnection, title: &str, body: &str) -> Post {
    use crate::schema::posts;

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_main() -> Result<()> {
        main()
    }
}

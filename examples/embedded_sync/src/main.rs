#![forbid(unsafe_code)]
#![forbid(clippy::allow_attributes)]
#![deny(clippy::pedantic)]

use postgresql_embedded::blocking::PostgreSQL;
use postgresql_embedded::Result;

fn main() -> Result<()> {
    let mut postgresql = PostgreSQL::default();
    postgresql.setup()?;
    postgresql.start()?;

    let database_name = "test";
    postgresql.create_database(database_name)?;
    postgresql.database_exists(database_name)?;
    postgresql.drop_database(database_name)?;

    postgresql.stop()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_main() -> Result<()> {
        main()
    }
}

pub mod entities;
pub mod migrator;

use sea_orm::{ConnectionTrait, Database, DatabaseConnection, Statement};
use sea_orm_migration::{MigratorTrait, SchemaManager};

use crate::{database::migrator::Migrator, error::Result};

pub const DATABASE_URL: &str = "postgres://bob:users@localhost:5432";

pub async fn construct_database() -> Result<DatabaseConnection> {
    let database = Database::connect(DATABASE_URL).await?;
    let name = "finch";

    database
        .execute(Statement::from_string(
            database.get_database_backend(),
            format!("DROP DATABASE IF EXISTS \"{}\";", name),
        ))
        .await?;
    database
        .execute(Statement::from_string(
            database.get_database_backend(),
            format!("CREATE DATABASE \"{}\";", name),
        ))
        .await?;

    let url = format!("{}/{}", DATABASE_URL, name);
    let database = Database::connect(&url).await?;

    let schema_manager = SchemaManager::new(&database);

    Migrator::refresh(&database).await?;
    assert!(schema_manager.has_table("teams").await?);
    assert!(schema_manager.has_table("users").await?);

    Ok(database)
}

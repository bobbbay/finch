mod m20220721_000001_create_teams_table;
mod m20220721_000002_create_users_table;

use sea_orm_migration::{async_trait, MigrationTrait, MigratorTrait};

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220721_000001_create_teams_table::Migration),
            Box::new(m20220721_000002_create_users_table::Migration),
        ]
    }
}

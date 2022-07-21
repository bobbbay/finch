use sea_orm_migration::prelude::*;

use super::m20220721_000001_create_teams_table::Team;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20220721_000002_create_users_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Name).string().not_null())
                    .col(
                        ColumnDef::new(User::Permissions)
                            .enumeration("permissions", vec!["member", "mentor", "superuser"]),
                    )
                    .col(ColumnDef::new(User::TeamId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user-team_id")
                            .from(User::Table, User::TeamId)
                            .to(Team::Table, Team::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum User {
    Table,
    Id,
    Name,
    Permissions,
    TeamId,
}

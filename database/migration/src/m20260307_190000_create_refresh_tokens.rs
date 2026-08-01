use entity::refresh_tokens;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::Schema;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create refresh_tokens table
        let backend = manager.get_database_backend();
        let schema = Schema::new(backend);
        manager
            .create_table(schema.create_table_from_entity(refresh_tokens::Entity))
            .await?;

        // Index on token_hash for quick lookup (used when validating presented refresh tokens)
        manager
            .create_index(
                Index::create()
                    .name("idx_refresh_tokens_token_hash")
                    .table(refresh_tokens::Entity)
                    .col(refresh_tokens::Column::Token)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop index then table
        manager
            .drop_index(
                Index::drop()
                    .name("idx_refresh_tokens_token_hash")
                    .table(refresh_tokens::Entity)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(refresh_tokens::Entity).to_owned())
            .await
    }
}

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Ip::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Ip::Uuid).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Ip::Ipv4).string().not_null())
                    .col(
                        ColumnDef::new(Ip::Created)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(ColumnDef::new(Ip::Updated).date_time())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Ip::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Ip {
    Table,
    Uuid,
    Ipv4,
    Created,
    Updated,
}

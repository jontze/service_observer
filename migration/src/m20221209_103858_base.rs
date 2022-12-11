use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(ip_table::create()).await?;
        manager.create_table(geolocation_table::create()).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(ip_table::drop()).await?;
        manager.drop_table(geolocation_table::drop()).await?;
        manager
            .drop_foreign_key(geolocation_table::drop_fk())
            .await?;
        Ok(())
    }
}

mod ip_table {
    use sea_orm_migration::prelude::*;

    #[derive(Iden)]
    pub(crate) enum Ip {
        Table,
        Id,
        Ipv4,
        Created,
        Updated,
    }

    pub(crate) fn create() -> TableCreateStatement {
        Table::create()
            .table(Ip::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Ip::Id)
                    .integer()
                    .auto_increment()
                    .not_null()
                    .primary_key(),
            )
            .col(ColumnDef::new(Ip::Ipv4).string().not_null())
            .col(
                ColumnDef::new(Ip::Created)
                    .date_time()
                    .not_null()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
            )
            .col(ColumnDef::new(Ip::Updated).date_time())
            .to_owned()
    }

    pub(crate) fn drop() -> TableDropStatement {
        Table::drop().table(Ip::Table).to_owned()
    }
}

mod geolocation_table {
    use super::ip_table::Ip;
    use sea_orm_migration::prelude::*;

    const IP_FK_NAME: &'static str = "fk-ip_id";

    #[derive(Iden)]
    enum Geolocation {
        Table,
        Id,
        IpId,
        Latitude,
        Longitude,
        Created,
    }

    pub(crate) fn create() -> TableCreateStatement {
        Table::create()
            .table(Geolocation::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Geolocation::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new(Geolocation::Latitude).double().not_null())
            .col(ColumnDef::new(Geolocation::Longitude).double().not_null())
            .col(
                ColumnDef::new(Geolocation::Created)
                    .date_time()
                    .not_null()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
            )
            .col(ColumnDef::new(Geolocation::IpId).integer().not_null())
            .foreign_key(
                ForeignKey::create()
                    .name(IP_FK_NAME)
                    .from(Geolocation::Table, Geolocation::IpId)
                    .to(Ip::Table, Ip::Id),
            )
            .to_owned()
    }

    pub(crate) fn drop() -> TableDropStatement {
        Table::drop().table(Geolocation::Table).to_owned()
    }

    pub(crate) fn drop_fk() -> ForeignKeyDropStatement {
        ForeignKey::drop()
            .table(Geolocation::Table)
            .name(IP_FK_NAME)
            .to_owned()
    }
}

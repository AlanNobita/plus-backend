use sea_orm_migration::{
	prelude::{extension::postgres::Type, *},
	sea_orm::{EnumIter, Iterable as _},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub struct PlayerStatus;

#[derive(DeriveIden, EnumIter)]
pub enum PlayerStatusVariants {
	Online,
	Offline,
}

#[derive(DeriveIden)]
enum User {
	Table,
	/// The status the player picked; `NULL` means derive it from their connection.
	StatusOverride,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.create_type(
				Type::create()
					.as_enum(PlayerStatus)
					.values(PlayerStatusVariants::iter())
					.to_owned(),
			)
			.await?;

		manager
			.alter_table(
				Table::alter()
					.table(User::Table)
					.add_column(
						ColumnDef::new(User::StatusOverride)
							.custom(PlayerStatus)
							.null(),
					)
					.to_owned(),
			)
			.await
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.alter_table(
				Table::alter()
					.table(User::Table)
					.drop_column(User::StatusOverride)
					.to_owned(),
			)
			.await?;
		manager
			.drop_type(Type::drop().name(PlayerStatus).to_owned())
			.await
	}
}

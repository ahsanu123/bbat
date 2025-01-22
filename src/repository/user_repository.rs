use crate::app_macro;
use crate::model::User;
use sea_query::{ColumnDef, Expr, Func, Iden, OnConflict, Order, Query, SqliteQueryBuilder, Table};
use sea_query_binder::SqlxBinder;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{Row, SqlitePool};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use tokio_macros;

#[derive(Iden)]
enum Character {
    Table,
    Id,
    Uuid,
    Character,
    FontSize,
    Meta,
    Created,
}

pub struct UserRepository;

impl UserRepository {
    pub async fn add_user() {
        let sql = Table::create()
            .table(Character::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Character::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new(Character::Uuid).uuid())
            .col(ColumnDef::new(Character::FontSize).integer())
            .col(ColumnDef::new(Character::Character).string())
            .col(ColumnDef::new(Character::Meta).json())
            .col(ColumnDef::new(Character::Created).date_time())
            .build(SqliteQueryBuilder);
    }
    pub async fn list_user() {}
}
#[derive(EnumIter, Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

// It's simple to iterate over the variants of an enum.

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio_macros::test]
    async fn create_table() {
        let connection_option = SqliteConnectOptions::new()
            .filename("bbat.db")
            .create_if_missing(true);

        let pool = SqlitePool::connect_with(connection_option).await.unwrap();

        // Schema
        let sql = Table::create()
            .table(Character::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Character::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new(Character::Uuid).uuid())
            .col(ColumnDef::new(Character::FontSize).integer())
            .col(ColumnDef::new(Character::Character).string())
            .col(ColumnDef::new(Character::Meta).json())
            .col(ColumnDef::new(Character::Created).date_time())
            .build(SqliteQueryBuilder);

        let result = sqlx::query(&sql).execute(&pool).await;

        println!("=============RESULT==================");
        for color in Color::iter() {
            println!("My favorite color is {:?}", color);
        }
        println!("Create table character: {result:?}");
        println!("=====================================");
    }
}

use chrono::{DateTime, Local, NaiveDate};
use diesel::prelude::{Insertable, Queryable, Selectable};
use std::string::String;

#[derive(Queryable, Selectable, Insertable, Default)]
#[diesel(table_name = crate::schema::dregs)]
pub struct Dregs {
    pub id: i32,
    pub username: String,
    pub user_id: i32,
    pub count: i32,
    pub paid: bool,
    pub taken_time: NaiveDate,
    pub production_time: NaiveDate,
    pub amount: i32,
    pub price: i32,
    pub description: String,
}

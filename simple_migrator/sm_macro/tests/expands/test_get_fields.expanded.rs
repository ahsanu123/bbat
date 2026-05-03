Option < String >
Option < String >
bool
String
#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use simple_migrator_macro::{FromRow, GetFields};
pub struct MigrationStatus {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_applied: bool,
    pub migration_id: String,
}
impl MigrationStatus {
    pub fn get_fields() -> Vec<String> {
        let fields = Vec::<
            String,
        >::from([
            "NAME".into(),
            "DESCRIPTION".into(),
            "ISAPPLIED".into(),
            "MIGRATIONID".into(),
        ]);
        fields
    }
}
use rsfbclient::{Row, SqlType};
impl From<Row> for MigrationStatus
where
    Self: Default,
{
    fn from(value: Row) -> Self {
        let mut default_value = Self::default();
        let mut key_val: HashMap<String, SqlType> = value
            .cols
            .iter()
            .map(|c| (c.name.to_uppercase(), c.value.clone()))
            .collect();
        default_value.name = key_val;
        default_value.description = key_val;
        default_value.is_applied = key_val;
        default_value.migration_id = key_val.default_value;
    }
}

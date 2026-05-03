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
use chrono::NaiveDateTime;
use std::collections::HashMap;
pub trait IntoValueTrait<T> {
    fn get_and_into(&self, key: String) -> T;
}
impl IntoValueTrait<bool> for HashMap<String, SqlType> {
    fn get_and_into(&self, key: String) -> bool {
        let sql_val = self
            .get(&key)
            .expect(
                &::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("fail to get value from key: {0}", key),
                    )
                }),
            );
        match sql_val {
            SqlType::Boolean(bool_val) => *bool_val,
            _ => {
                ::core::panicking::panic_fmt(format_args!("key is not type of String"));
            }
        }
    }
}
impl IntoValueTrait<String> for HashMap<String, SqlType> {
    fn get_and_into(&self, key: String) -> String {
        let sql_val = self
            .get(&key)
            .expect(
                &::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("fail to get value from key: {0}", key),
                    )
                }),
            );
        match sql_val {
            SqlType::Text(text) => text.clone(),
            _ => {
                ::core::panicking::panic_fmt(format_args!("key is not type of String"));
            }
        }
    }
}
impl IntoValueTrait<Option<String>> for HashMap<String, SqlType> {
    fn get_and_into(&self, key: String) -> Option<String> {
        let sql_val = self
            .get(&key)
            .expect(
                &::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("fail to get value from key: {0}", key),
                    )
                }),
            );
        match sql_val {
            SqlType::Text(text) => Some(text.clone()),
            SqlType::Null => None,
            _ => {
                ::core::panicking::panic_fmt(
                    format_args!("key is not type of Option<String>"),
                );
            }
        }
    }
}
impl IntoValueTrait<Option<i64>> for HashMap<String, SqlType> {
    fn get_and_into(&self, key: String) -> Option<i64> {
        let sql_val = self
            .get(&key)
            .expect(
                &::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("fail to get value from key: {0}", key),
                    )
                }),
            );
        match sql_val {
            SqlType::Integer(int_val) => Some(*int_val),
            SqlType::Null => None,
            _ => {
                ::core::panicking::panic_fmt(
                    format_args!("key is not type of Option<i64>"),
                );
            }
        }
    }
}
impl IntoValueTrait<i64> for HashMap<String, SqlType> {
    fn get_and_into(&self, key: String) -> i64 {
        let sql_val = self
            .get(&key)
            .expect(
                &::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("fail to get value from key: {0}", key),
                    )
                }),
            );
        match sql_val {
            SqlType::Integer(int_val) => *int_val,
            _ => {
                ::core::panicking::panic_fmt(format_args!("key is not type of i64"));
            }
        }
    }
}
impl IntoValueTrait<NaiveDateTime> for HashMap<String, SqlType> {
    fn get_and_into(&self, key: String) -> NaiveDateTime {
        let sql_val = self
            .get(&key)
            .expect(
                &::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("fail to get value from key: {0}", key),
                    )
                }),
            );
        match sql_val {
            SqlType::Timestamp(timestamp) => *timestamp,
            _ => {
                ::core::panicking::panic_fmt(
                    format_args!("key is not type of Option<i64>"),
                );
            }
        }
    }
}
impl IntoValueTrait<Option<NaiveDateTime>> for HashMap<String, SqlType> {
    fn get_and_into(&self, key: String) -> Option<NaiveDateTime> {
        let sql_val = self
            .get(&key)
            .expect(
                &::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("fail to get value from key: {0}", key),
                    )
                }),
            );
        match sql_val {
            SqlType::Timestamp(timestamp) => Some(*timestamp),
            SqlType::Null => None,
            _ => {
                ::core::panicking::panic_fmt(
                    format_args!("key is not type of Option<NaiveDateTime>"),
                );
            }
        }
    }
}
impl From<&Row> for MigrationStatus
where
    Self: Default,
{
    fn from(value: &Row) -> Self {
        let mut key_val: HashMap<String, SqlType> = value
            .cols
            .iter()
            .map(|c| (c.name.to_uppercase(), c.value.clone()))
            .collect();
        Self {
            name: key_val.get_and_into("NAME".into()),
            description: key_val.get_and_into("DESCRIPTION".into()),
            is_applied: key_val.get_and_into("ISAPPLIED".into()),
            migration_id: key_val.get_and_into("MIGRATIONID".into()),
        }
    }
}
impl From<Row> for MigrationStatus
where
    Self: Default,
{
    fn from(value: Row) -> Self {
        let mut key_val: HashMap<String, SqlType> = value
            .cols
            .iter()
            .map(|c| (c.name.to_uppercase(), c.value.clone()))
            .collect();
        Self {
            name: key_val.get_and_into("NAME".into()),
            description: key_val.get_and_into("DESCRIPTION".into()),
            is_applied: key_val.get_and_into("ISAPPLIED".into()),
            migration_id: key_val.get_and_into("MIGRATIONID".into()),
        }
    }
}

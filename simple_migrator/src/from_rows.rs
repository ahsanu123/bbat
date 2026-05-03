use std::collections::HashMap;

use crate::models::MigrationStatus;
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

        // let res: String = match key_val.get("SSMS").unwrap() {
        //     SqlType::Text(textval) => textval.clone(),
        //     SqlType::Timestamp(naive_date_time) => naive_date_time,
        //     SqlType::Null => None,
        //     _ => panic!("fail to get value from SqlType"),
        // };

        // let name_sqlvalue = value
        //     .cols
        //     .iter()
        //     .find(|c| c.name == "NAME")
        //     .unwrap()
        //     .value
        //     .clone();
        //
        // let name: String = match name_sqlvalue {
        //     SqlType::Text(sqlval) => sqlval,
        //     _ => "".into(),
        // };

        // default_value.name = Some(name);
        default_value
    }
}

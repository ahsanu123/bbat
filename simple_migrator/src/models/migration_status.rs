use rsfbclient::prelude::IntoParams;
use simple_migrator_macro::{FromRow, GetFields};

#[derive(Default, GetFields, FromRow, Debug, IntoParams)]
pub struct MigrationStatus {
    pub migration_id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub applied_date: Option<NaiveDateTime>,
    pub is_applied: bool,
}

impl MigrationStatus {
    pub fn into_positional_param(
        &self,
    ) -> (
        String,
        Option<String>,
        Option<String>,
        Option<NaiveDateTime>,
        bool,
    ) {
        (
            self.migration_id.clone(),
            self.name.clone(),
            self.description.clone(),
            self.applied_date,
            self.is_applied,
        )
    }
}

#[cfg(test)]
mod modtest_migration_status {
    use super::*;

    #[test]
    fn test_migration_status_get_fields() {
        let fields = MigrationStatus::get_fields();

        println!("fields: {:#?}", fields);
    }
}

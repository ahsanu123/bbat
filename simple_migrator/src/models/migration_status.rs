use chrono::{DateTime, Utc};
use simple_migrator_macro::GetFields;

#[derive(Default, GetFields)]
pub struct MigrationStatus {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_applied: bool,
    pub applied_date: Option<DateTime<Utc>>,
    pub migration_id: String,
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

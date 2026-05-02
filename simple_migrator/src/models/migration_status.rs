use chrono::{DateTime, Utc};

pub struct MigrationStatus {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_applied: bool,
    pub applied_date: Option<DateTime<Utc>>,
    pub migration_id: String,
}

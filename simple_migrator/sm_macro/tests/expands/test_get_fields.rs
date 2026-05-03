use simple_migrator_macro::{FromRow, GetFields};

#[derive(GetFields, FromRow)]
pub struct MigrationStatus {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_applied: bool,
    pub migration_id: String,
}

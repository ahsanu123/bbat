use anyhow::Result;

use crate::models::MigrationStatus;

pub trait ExecutorTrait {
    fn execute(&mut self, statement: String) -> Result<()>;
    fn query<T>(&mut self, statement: String) -> Result<T>;
    fn get_applied(&self) -> Result<Vec<MigrationStatus>>;

    fn ensure_migration_table(&mut self) -> Result<()>;
    fn upsert_migration_status(&self, mig_status: MigrationStatus) -> Result<()>;
    fn delete_migration_status(&self, mig_status_id: String) -> Result<()>;
}

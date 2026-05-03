use anyhow::Result;
use rsfbclient::IntoParams;

use crate::models::MigrationStatus;

pub trait ExecutorTrait {
    fn execute<P: IntoParams>(&mut self, statement: String, param: P) -> Result<()>;
    fn query<T, P: IntoParams>(&mut self, statement: String, param: P) -> Result<T>;
    fn get_applied(&self) -> Result<Vec<MigrationStatus>>;

    fn ensure_migration_table(&mut self) -> Result<()>;
    fn upsert_migration_status(&self, mig_status: MigrationStatus) -> Result<()>;
    fn delete_migration_status(&self, mig_status_id: String) -> Result<()>;
}

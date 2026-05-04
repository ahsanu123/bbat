use crate::executor::ExecutorTrait;
use crate::get_db_conn;
use crate::models::MigrationStatus;
use anyhow::anyhow;
use anyhow::{Ok, Result};
use rsfbclient::Row;
use rsfbclient::{Execute, Queryable};

pub struct FirebirdDbExecutor;

impl ExecutorTrait for FirebirdDbExecutor {
    fn execute<P: rsfbclient::IntoParams>(&mut self, statement: String, param: P) -> Result<()> {
        let mut conn = get_db_conn()
            .lock()
            .map_err(|_| anyhow!("fail to lock DATABASE_CONNECTION"))?;

        conn.execute(&statement, param)?;

        Ok(())
    }

    fn get_applied(&self) -> Result<Vec<MigrationStatus>> {
        let mut conn = get_db_conn()
            .lock()
            .map_err(|_| anyhow!("fail to lock DATABASE_CONNECTION"))?;

        let rows: Vec<Row> = conn.query("SELECT * FROM GETAPPLIEDMIGRATIONS", ())?;

        let migrations_status: Vec<MigrationStatus> = rows.iter().map(|row| row.into()).collect();

        Ok(migrations_status)
    }

    fn ensure_meta_procedures(&mut self) -> Result<()> {
        let mut conn = get_db_conn()
            .lock()
            .map_err(|_| anyhow!("fail to lock connection"))?;

        let ensure_proc: Vec<Row> = conn.query(
            "SELECT 1 FROM RDB$PROCEDURES WHERE RDB$PROCEDURE_NAME = 'ENSUREMIGRATIONTABLE';",
            (),
        )?;

        if ensure_proc.is_empty() {
            conn.execute(
                include_str!("../../sqls/meta_ensure_migration_table.sql"),
                (),
            )?;

            let result: Vec<(bool,)> =
                conn.query("SELECT CREATED FROM ENSUREMIGRATIONTABLE;", ())?;

            let (_,) = result
                .first()
                .ok_or(anyhow!("fail to run ENSUREMIGRATIONTABLE"))?;

            conn.execute(
                include_str!("../../sqls/meta_delete_migration_status.sql"),
                (),
            )?;
            conn.execute(
                include_str!("../../sqls/meta_get_applied_migration_status.sql"),
                (),
            )?;
            conn.execute(
                include_str!("../../sqls/meta_upsert_migration_status.sql"),
                (),
            )?;
        }

        Ok(())
    }

    fn upsert_migration_status(&self, mig_status: MigrationStatus) -> Result<()> {
        let mut conn = get_db_conn()
            .lock()
            .map_err(|_| anyhow!("fail to lock connection"))?;

        let positional_param = mig_status.into_positional_param();
        let _: Vec<Row> = conn.query(
            "SELECT * FROM UpsertMigrationStatus( ?, ?, ?, ?, ?);",
            positional_param,
        )?;

        Ok(())
    }

    fn delete_migration_status(&self, mig_status_id: String) -> Result<()> {
        let mut conn = get_db_conn()
            .lock()
            .map_err(|_| anyhow!("fail to lock connection"))?;

        conn.execute("SELECT * FROM DELETEMIGRATIONSTATUS(?)", (mig_status_id,))?;

        Ok(())
    }
}

use anyhow::anyhow;
use anyhow::{Ok, Result};
use chrono::DateTime;
use chrono::Utc;
use once_cell::sync::OnceCell;
use rsfbclient::Row;
use rsfbclient::SimpleConnection;
use rsfbclient::{Execute, Queryable};
use simple_migrator::models::MigrationStatus;
use simple_migrator::{
    executor::ExecutorTrait, migrations::MigrationTrait, runner_builder::RunnerBuilder,
};
use std::env;
use std::sync::Mutex;

fn main() -> Result<()> {
    let fb_runner = RunnerBuilder::create()
        .with_executor(FirebirdDbExecutor)
        .add_migrations(vec![Migration1])
        .build()?;

    Ok(())
}

pub static DATABASE_CONNECTION: OnceCell<Mutex<SimpleConnection>> = OnceCell::new();

pub fn get_db_conn() -> &'static Mutex<SimpleConnection> {
    DATABASE_CONNECTION.get_or_init(|| {
        let cwd = env::current_dir().unwrap();
        println!("cwd: {}", cwd.to_string_lossy());

        let db_path = cwd.join("firebird-test.fdb");
        println!("cwd: {}", db_path.to_string_lossy());

        let conn = rsfbclient::builder_native()
            .with_dyn_link()
            .with_embedded()
            .db_name(db_path.to_string_lossy())
            .user("sysdba")
            .connect()
            .unwrap();

        Mutex::new(conn.into())
    })
}

struct FirebirdDbExecutor;

impl ExecutorTrait for FirebirdDbExecutor {
    fn execute<P: rsfbclient::IntoParams>(&mut self, statement: String, param: P) -> Result<()> {
        let mut conn = get_db_conn()
            .lock()
            .map_err(|_| anyhow!("fail to lock DATABASE_CONNECTION"))?;

        conn.execute(&statement, param)?;

        Ok(())
    }

    fn query<T, P: rsfbclient::IntoParams>(&mut self, statement: String, param: P) -> Result<T> {
        todo!()
    }

    fn get_applied(&self) -> Result<Vec<MigrationStatus>> {
        let mut conn = get_db_conn()
            .lock()
            .map_err(|_| anyhow!("fail to lock DATABASE_CONNECTION"))?;

        let rows: Vec<Row> = conn
            .query("SELECT * FROM GETAPPLIEDMIGRATIONS", ())
            .expect("fail to query get_applied");

        let migrations_status: Vec<MigrationStatus> = rows.iter().map(|row| row.into()).collect();

        Ok(migrations_status)
    }

    fn ensure_migration_table(&mut self) -> Result<()> {
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

struct Migration1;

impl MigrationTrait for Migration1 {
    fn get_id(&self) -> String {
        todo!()
    }

    fn get_name(&self) -> String {
        todo!()
    }

    fn get_description(&self) -> String {
        todo!()
    }

    fn up(&self) -> anyhow::Result<()> {
        todo!()
    }

    fn down(&self) -> anyhow::Result<()> {
        todo!()
    }

    fn get_date(&self) -> Result<DateTime<Utc>> {
        let date_str = "2024-05-02 11:30:00 +0000";
        // Define the format matching the string exactly
        let format = "%Y-%m-%d %H:%M:%S %z";

        // Parse to FixedOffset first
        let dt = DateTime::parse_from_str(date_str, format).expect("Failed to parse");

        // Convert to Utc or Local if needed
        let utc_dt: DateTime<Utc> = DateTime::from(dt);

        Ok(utc_dt)
    }
}

#[cfg(test)]
mod test_firebird_migrator_bin {

    use chrono::Local;

    use super::*;

    #[test]
    fn string_to_date() {
        let date_str = "2024-05-02 11:30:00 +0000";
        // Define the format matching the string exactly
        let format = "%Y-%m-%d %H:%M:%S %z";

        // Parse to FixedOffset first
        let dt = DateTime::parse_from_str(date_str, format).expect("Failed to parse");

        // Convert to Utc or Local if needed
        let utc_dt: DateTime<Utc> = DateTime::from(dt);

        println!("{}", utc_dt);
    }

    #[test]
    fn test_get_prices() {
        let mut conn = get_db_conn().lock().expect("fail to lock connection");

        let rows: Vec<Row> = conn
            .query("SELECT * FROM PRICES", ())
            .expect("fail to query get_applied");

        for row in rows {
            println!("------------------------------------");
            for col in row.cols {
                println!("{}: {:?}", col.name, col.value);
            }
        }
    }

    #[test]
    fn test_ensure_migration_table() -> Result<()> {
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
        }

        Ok(())
    }

    #[test]
    fn test_from_row() -> Result<()> {
        let mut conn = get_db_conn()
            .lock()
            .map_err(|_| anyhow!("fail to lock connection"))?;

        let rows: Vec<Row> = conn.query("SELECT * FROM SIMPLEMIGRATOR", ())?;

        let migration_status: Vec<MigrationStatus> = rows.iter().map(|row| row.into()).collect();

        println!("migration_status: {:#?}", migration_status);

        Ok(())
    }

    #[test]
    fn test_create_all_meta_procedure() -> Result<()> {
        let mut conn = get_db_conn()
            .lock()
            .map_err(|_| anyhow!("fail to lock connection"))?;

        conn.execute(
            include_str!("../../sqls/meta_delete_migration_status.sql"),
            (),
        )?;

        conn.execute(
            include_str!("../../sqls/meta_ensure_migration_table.sql"),
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

        Ok(())
    }

    #[test]
    fn test_get_applied_migrations() -> Result<()> {
        let mut conn = get_db_conn()
            .lock()
            .map_err(|_| anyhow!("fail to lock DATABASE_CONNECTION"))?;

        let rows: Vec<Row> = conn
            .query("SELECT * FROM GETAPPLIEDMIGRATIONS", ())
            .expect("fail to query get_applied");

        let migrations_status: Vec<MigrationStatus> = rows.iter().map(|row| row.into()).collect();

        println!("applied migrations: {:#?}", migrations_status);

        Ok(())
    }

    #[test]
    fn test_upsert_migration_status() -> Result<()> {
        let mig_status = MigrationStatus {
            migration_id: "2".into(),
            name: Some("second migrations".into()),
            description: Some("hello world second migration".into()),
            is_applied: true,
            applied_date: Some(Local::now().naive_local()),
        };

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

    #[test]
    fn test_delete_migration_status() -> Result<()> {
        let mut conn = get_db_conn()
            .lock()
            .map_err(|_| anyhow!("fail to lock connection"))?;

        let _: Vec<Row> = conn.query("SELECT * FROM DELETEMIGRATIONSTATUS(?)", ("2",))?;

        Ok(())
    }
}

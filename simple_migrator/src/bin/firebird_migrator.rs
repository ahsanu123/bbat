use std::env;
use std::sync::Mutex;

use anyhow::{Ok, Result};
use chrono::DateTime;
use chrono::Utc;
use once_cell::sync::OnceCell;
use rsfbclient::SimpleConnection;
use simple_migrator::models::MigrationStatus;
use simple_migrator::{
    executor::ExecutorTrait, migrations::MigrationTrait, runner_builder::RunnerBuilder,
};

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
            .user("ah")
            .connect()
            .unwrap();

        Mutex::new(conn.into())
    })
}

struct FirebirdDbExecutor;

impl ExecutorTrait for FirebirdDbExecutor {
    fn execute(&mut self, statement: String) -> anyhow::Result<()> {
        todo!()
    }

    fn query<T>(&mut self, statement: String) -> anyhow::Result<T> {
        todo!()
    }

    fn get_applied(&self) -> Result<Vec<MigrationStatus>> {
        todo!()
    }

    fn ensure_migration_table(&mut self) -> Result<()> {
        todo!()
    }

    fn upsert_migration_status(&self, mig_status: MigrationStatus) -> Result<()> {
        todo!()
    }

    fn delete_migration_status(&self, mig_status_id: String) -> Result<()> {
        todo!()
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
}

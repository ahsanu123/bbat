use anyhow::anyhow;
use anyhow::{Ok, Result};
use chrono::NaiveDateTime;
use once_cell::sync::OnceCell;
use rsfbclient::Row;
use rsfbclient::SimpleConnection;
use rsfbclient::{Execute, Queryable};
use simple_migrator::get_db_conn;
use simple_migrator::models::MigrationStatus;
use simple_migrator::platforms::firebirdsql::FirebirdDbExecutor;
use simple_migrator::runner::RunnerTrait;
use simple_migrator::{migrations::MigrationTrait, runner_builder::RunnerBuilder};
use std::env;
use std::sync::Mutex;

use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// up or down
    #[arg(value_enum)]
    mode: Mode,

    /// up or down to some migration id
    migrate_to: Option<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    /// Run swiftly
    Up,
    /// Crawl slowly but steadily
    ///
    /// This paragraph is ignored because there is no long help text for possible values.
    Down,

    Getapplied,

    Printallmigrations,

    Getunapplied,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut fb_runner = RunnerBuilder::create()
        .with_executor(FirebirdDbExecutor)
        .add_migrations(vec![Box::new(Migration1), Box::new(Migration2)])
        .build()?;

    fb_runner.ensure_meta_procedures()?;

    match cli.mode {
        Mode::Up => {
            if let Some(migration_id) = cli.migrate_to {
                fb_runner.up_to(migration_id)?;
            } else {
                fb_runner.up()?;
            };
        }
        Mode::Down => {
            if let Some(migration_id) = cli.migrate_to {
                fb_runner.down_to(migration_id)?;
            } else {
                fb_runner.down()?;
            };
        }
        Mode::Getapplied => {
            let applied = fb_runner.get_applied()?;
            println!("applied migrations: {:#?}", applied);
        }
        Mode::Getunapplied => {
            let unapplied = fb_runner.get_unapplied()?;
            println!("un-applied migrations: {:#?}", unapplied);
        }
        Mode::Printallmigrations => {
            fb_runner.print_all_migrations()?;
        }
    };

    Ok(())
}

#[derive(Debug)]
struct Migration1;

impl MigrationTrait for Migration1 {
    fn get_id(&self) -> String {
        "1".into()
    }

    fn get_name(&self) -> String {
        "create price table".into()
    }

    fn get_description(&self) -> String {
        "create price table".into()
    }

    fn up(&self) -> Result<()> {
        let mut conn = get_db_conn()
            .lock()
            .map_err(|_| anyhow!("fail to lock connection"))?;

        conn.execute(include_str!("../../sqls/0_create_price_table.sql"), ())?;
        Ok(())
    }

    fn down(&self) -> Result<()> {
        let mut conn = get_db_conn()
            .lock()
            .map_err(|_| anyhow!("fail to lock connection"))?;

        conn.execute("DROP TABLE Prices;", ())?;
        Ok(())
    }

    fn get_date(&self) -> Result<NaiveDateTime> {
        let date_str = "2026-05-01_11:30:00";
        let format = "%Y-%m-%d_%H:%M:%S";
        let naive_dt_time = NaiveDateTime::parse_from_str(date_str, format)?;

        Ok(naive_dt_time)
    }
}

#[derive(Debug)]
struct Migration2;

impl MigrationTrait for Migration2 {
    fn get_id(&self) -> String {
        "2".into()
    }

    fn get_name(&self) -> String {
        "create user table".into()
    }

    fn get_description(&self) -> String {
        "create user table".into()
    }

    fn up(&self) -> Result<()> {
        let mut conn = get_db_conn()
            .lock()
            .map_err(|_| anyhow!("fail to lock connection"))?;

        conn.execute(include_str!("../../sqls/1_create_user_table.sql"), ())?;
        Ok(())
    }

    fn down(&self) -> Result<()> {
        let mut conn = get_db_conn()
            .lock()
            .map_err(|_| anyhow!("fail to lock connection"))?;

        conn.execute("DROP TABLE Users ;", ())?;
        Ok(())
    }

    fn get_date(&self) -> Result<NaiveDateTime> {
        let date_str = "2026-05-03_11:30:00";
        let format = "%Y-%m-%d_%H:%M:%S";
        let naive_dt_time = NaiveDateTime::parse_from_str(date_str, format)?;

        Ok(naive_dt_time)
    }
}

#[cfg(test)]
mod test_firebird_migrator_bin {

    use chrono::Local;

    use super::*;

    #[test]
    fn test_string_to_date() -> Result<()> {
        let date_str = "2024-05-02_11:30:00";
        let format = "%Y-%m-%d_%H:%M:%S";

        let naive_dt_time =
            NaiveDateTime::parse_from_str(date_str, format).expect("Failed to parse");

        println!("{}", naive_dt_time);

        Ok(())
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

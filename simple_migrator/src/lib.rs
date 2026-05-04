use std::{env, sync::Mutex};

use once_cell::sync::OnceCell;
use rsfbclient::SimpleConnection;

pub mod executor;
pub mod migrations;
pub mod models;
pub mod platforms;
pub mod runner;
pub mod runner_builder;

pub(crate) mod clap_args;

pub static DATABASE_CONNECTION: OnceCell<Mutex<SimpleConnection>> = OnceCell::new();

// TODO: make it generic
pub fn get_db_conn() -> &'static Mutex<SimpleConnection> {
    DATABASE_CONNECTION.get_or_init(|| {
        let cwd = env::current_dir().unwrap();
        let db_path = cwd.join("firebird-test.fdb");
        let db_path_string = db_path.to_string_lossy().to_string();
        println!("database path: {}", db_path_string);

        let mut builder = rsfbclient::builder_native().with_dyn_link().with_embedded();

        let conn = if !db_path.exists() {
            builder
                .db_name(&db_path_string)
                .user("sysdba")
                .create_database()
                .expect("database is not exists, and fail to create one!!.")
        } else {
            builder
                .db_name(&db_path_string)
                .user("sysdba")
                .connect()
                .expect("fail to create connection")
        };

        Mutex::new(conn.into())
    })
}

use std::{env, path::Path};

use firebirust::Connection;

fn main() {
    let cwd = env::current_dir().unwrap();
    println!("cwd: {}", cwd.to_string_lossy());

    let db_path = cwd.join("firebird-test.fdb");
    println!("cwd: {}", db_path.to_string_lossy());

    let mut conn =
        Connection::connect_url(&format!("firebird://ah:123@{}", db_path.to_string_lossy()))
            .unwrap();

    conn.execute_batch(
        r#"
        CREATE TABLE foo (
            a INTEGER NOT NULL,
            b VARCHAR(30) NOT NULL UNIQUE,
            c VARCHAR(1024),
            d DECIMAL(16,3) DEFAULT -0.123,
            e DATE DEFAULT '1967-08-11',
            f TIMESTAMP DEFAULT '1967-08-11 23:45:01',
            g TIME DEFAULT '23:45:01',
            h BLOB SUB_TYPE 1,
            i DOUBLE PRECISION DEFAULT 0.0,
            j FLOAT DEFAULT 0.0,
            PRIMARY KEY (a),
            CONSTRAINT CHECK_A CHECK (a <> 0)
        )
    "#,
    )
    .unwrap();
}

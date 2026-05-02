use rsfbclient::{FbError, Row, SystemInfos, prelude::*};
use std::env;

#[derive(Clone, IntoParams)]
struct ParamTest {
    colb: f32,
    colc: String,
}

#[derive(Clone, IntoParams)]
struct ParamTest2 {
    colb: f32,
    colc: String,
}

const SQL_INSERT: &str = "insert into test (col_b, col_c) values (?, ?)";
const SQL_INSERT_NAMED: &str = "insert into test (col_b, col_c) values (:colb, :colc)";
const SQL_INSERT_PURE: &str = "insert into test (col_b, col_c) values (32, 'Leite')";

fn main() -> Result<(), FbError> {
    // NOTE:
    // to create app admin
    // - login into isql-fb with SYSDBA
    // - then create user `create user ah password '123';`
    // - then add role to newly user `grant RDB$ADMIN to ah`
    // - or all `GRANT ALL ON DATABASE TO ah`

    let cwd = env::current_dir().unwrap();
    println!("cwd: {}", cwd.to_string_lossy());

    let db_path = cwd.join("firebird-test.fdb");
    println!("cwd: {}", db_path.to_string_lossy());

    let mut conn = rsfbclient::builder_native()
        .with_dyn_link()
        .with_embedded()
        .db_name(db_path.to_string_lossy())
        .user("ah")
        .connect()
        .unwrap();

    let version = conn.server_engine();
    println!("Server version: {:?}", version);

    let p1 = ParamTest {
        colb: 150.0,
        colc: "Café".to_string(),
    };
    let p2 = ParamTest {
        colb: 132.0,
        colc: "Arroz".to_string(),
    };
    let p3 = ParamTest2 {
        colb: 150.0,
        colc: "Feijão".to_string(),
    };

    let rows = conn.query_iter("select * from test;", ())?;

    println!("| col_a | col_b | col_c   |");
    println!("| ----- | ----- | ------- |");
    for row in rows {
        let (col_a, col_b, col_c): (i32, f32, String) = row?;

        println!("| {:^5} | {:^5} | {:7} |", col_a, col_b, col_c);
    }

    let rows: Vec<(i32, f32, String)> = conn.query("select * from test;", ())?;
    println!("{:?}", rows);

    let rows: Vec<Row> = conn.query("select * from test;", ())?;

    for row in rows {
        println!("------------------------------------");

        for col in row.cols {
            println!("{}: {:?}", col.name, col.value);
        }
    }

    // conn.with_transaction(|tr| {
    //     // First alternative (Recommended) (Prepares if needed and executes automatically)
    //     tr.execute(SQL_INSERT, (94, "Banana"))?; // with position params
    //     tr.execute(SQL_INSERT_NAMED, p1.clone())?; // with named params
    //     tr.execute(SQL_INSERT_NAMED, p3.clone())?; // with named params, again
    //
    //     // Second alternative
    //     tr.execute_immediate(SQL_INSERT_PURE)?;
    //
    //     // Third alternative, with position params
    //     {
    //         let mut stmt = tr.prepare(SQL_INSERT, false)?;
    //
    //         stmt.execute((-39, "test"))?;
    //         stmt.execute((12, "test 2"))?;
    //     }
    //     // Fourth alternative, with named params
    //     {
    //         let mut stmt = tr.prepare(SQL_INSERT_NAMED, true)?;
    //
    //         stmt.execute(p1.clone())?;
    //         stmt.execute(p2.clone())?;
    //         stmt.execute(p3.clone())?;
    //     }
    //
    //     Ok(())
    // })?;

    // Explicit close is optional
    conn.close()?;
    Ok(())
}

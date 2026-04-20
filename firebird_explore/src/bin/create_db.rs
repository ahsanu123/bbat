use rsfbclient::FbError;

fn main() -> Result<(), FbError> {
    let mut conn = rsfbclient::builder_native()
        .with_dyn_link()
        .with_remote()
        .db_name("examples.fdb")
        .user("SYSDBA")
        .pass("masterkey")
        .page_size(8 * 1024) // Optional
        .dialect(rsfbclient::Dialect::D1)
        .create_database()?;

    Ok(())
}

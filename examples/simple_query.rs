use chdb::Connection;

fn main() -> Result<(), chdb::Error> {
    let conn = Connection::new()?;

    let result = conn.query("SELECT number FROM numbers(10)", "TSVWithNames")?;

    println!("Elapsed: {}", result.elapsed);
    println!("Rows: {}", result.rows_read);
    println!("Bytes: {}", result.bytes_read);
    println!("Result:\n{}", result.data_utf8()?);

    Ok(())
}

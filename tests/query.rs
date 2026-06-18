//! Integration tests exercising a real in-process chdb engine.
//!
//! chdb permits only one connection per process, so these run serially within a
//! single test by reusing one [`Connection`].

use chdb::Connection;

#[test]
fn in_memory_queries() {
    let conn = Connection::new().expect("open in-memory connection");

    // Basic scalar.
    let r = conn.query("SELECT 1 + 1", "CSV").expect("scalar query");
    assert_eq!(r.data_utf8().unwrap().trim(), "2");

    // numbers() table function — the README's canonical example.
    let r = conn
        .query("SELECT number FROM numbers(10)", "CSV")
        .expect("numbers query");
    assert_eq!(r.data_utf8().unwrap(), "0\n1\n2\n3\n4\n5\n6\n7\n8\n9\n");
    assert_eq!(r.rows_read, 10);

    // Metrics are populated.
    assert!(r.elapsed >= 0.0);
}

#[test]
fn query_error_is_reported() {
    let conn = Connection::new().expect("open connection");
    let err = conn
        .query("SELECT * FROM nonexistent_table_xyz", "CSV")
        .unwrap_err();
    match err {
        chdb::Error::Query(msg) => assert!(!msg.is_empty(), "expected an error message"),
        other => panic!("expected Error::Query, got {other:?}"),
    }
}

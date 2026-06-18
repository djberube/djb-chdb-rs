//! Embedded [ClickHouse](https://clickhouse.com/) for Rust, via
//! [chdb](https://github.com/chdb-io/chdb).
//!
//! This crate links against `libchdb.so` (vendored under `vendor/chdb/`, built
//! from [chdb-core](https://github.com/chdb-io/chdb-core)) and exposes a small
//! safe wrapper over its connection-based C API.
//!
//! # Examples
//!
//! ```no_run
//! use chdb::Connection;
//!
//! // In-memory database (`:memory:`).
//! let conn = Connection::new()?;
//! let result = conn.query("SELECT number FROM numbers(3)", "CSV")?;
//! assert_eq!(result.data_utf8()?, "0\n1\n2\n");
//! # Ok::<(), chdb::Error>(())
//! ```

mod connection;
mod error;
mod ffi;
pub mod options;
mod result;

pub use connection::Connection;
pub use error::{Error, Result};
pub use options::QueryOption;
pub use result::QueryResult;

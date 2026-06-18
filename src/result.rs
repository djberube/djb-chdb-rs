//! Query result.

use crate::error::{Error, Result};

/// The materialized output of a query, with execution metrics.
///
/// The data buffer is copied out of the chdb result on construction, so a
/// `QueryResult` owns its data and is safe to keep after the
/// [`Connection`](crate::Connection) is dropped.
#[derive(Debug, Clone)]
pub struct QueryResult {
    /// Raw result bytes, rendered in the format requested at query time.
    pub buf: Vec<u8>,
    /// Wall-clock query time, in seconds.
    pub elapsed: f64,
    /// Number of rows in the result set.
    pub rows_read: u64,
    /// Bytes occupied by the result set in chdb's internal binary format.
    pub bytes_read: u64,
}

impl QueryResult {
    /// Borrow the raw result bytes.
    pub fn data(&self) -> &[u8] {
        &self.buf
    }

    /// Decode the result buffer as UTF-8.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Utf8`] if the buffer is not valid UTF-8. Binary output
    /// formats (e.g. `Native`, `Parquet`) will generally not decode; use
    /// [`data`](Self::data) for those.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use chdb::Connection;
    ///
    /// let conn = Connection::new()?;
    /// let result = conn.query("SELECT 1", "CSV")?;
    /// assert_eq!(result.data_utf8()?.trim(), "1");
    /// # Ok::<(), chdb::Error>(())
    /// ```
    pub fn data_utf8(&self) -> Result<String> {
        String::from_utf8(self.buf.clone()).map_err(Error::Utf8)
    }
}

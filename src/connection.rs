//! Embedded chdb connection.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr::NonNull;

use crate::error::{Error, Result};
use crate::ffi;
use crate::options::QueryOption;
use crate::result::QueryResult;

/// A handle to an embedded chdb (ClickHouse) engine running in this process.
///
/// "Connection" is ClickHouse vocabulary; there is no server or socket. A
/// `Connection` owns an in-process engine instance. With no `--path` argument
/// the database is purely in-memory (`:memory:`) and discarded on drop; pass
/// `--path=<dir>` to persist to disk.
///
/// chdb allows only **one** active connection per process. Constructing a
/// second `Connection` while one is alive will fail.
///
/// # Examples
///
/// ```no_run
/// use chdb::Connection;
///
/// let conn = Connection::new()?;
/// let result = conn.query("SELECT number FROM numbers(3)", "CSV")?;
/// assert_eq!(result.data_utf8()?, "0\n1\n2\n");
/// # Ok::<(), chdb::Error>(())
/// ```
#[derive(Debug)]
pub struct Connection {
    /// The pointer-to-handle returned by `chdb_connect`. We keep the outer
    /// pointer because `chdb_close_conn` takes it.
    handle: NonNull<*mut ffi::ChdbConnection>,
}

impl Connection {
    /// Open an in-memory connection (`:memory:`).
    ///
    /// # Errors
    ///
    /// Returns [`Error::ConnectFailed`] if the engine could not be started
    /// (including when a connection is already open in this process).
    pub fn new() -> Result<Self> {
        Self::with_options(std::iter::empty())
    }

    /// Open a connection backed by an on-disk database at `path`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::NulByte`] if `path` contains an interior NUL byte, or
    /// [`Error::ConnectFailed`] if the engine could not be started.
    pub fn open<P: AsRef<str>>(path: P) -> Result<Self> {
        let arg = format!("--path={}", path.as_ref());
        Self::with_options([QueryOption::Raw(arg)])
    }

    /// Open a connection with explicit engine arguments (the same flags
    /// `clickhouse local` accepts), built from [`QueryOption`] values.
    ///
    /// The leading `"clickhouse"` argv[0] is supplied automatically.
    ///
    /// # Errors
    ///
    /// Returns [`Error::NulByte`] if any argument contains an interior NUL
    /// byte, or [`Error::ConnectFailed`] if the engine could not be started.
    pub fn with_options<I>(options: I) -> Result<Self>
    where
        I: IntoIterator<Item = QueryOption>,
    {
        let mut args: Vec<String> = vec!["clickhouse".to_string()];
        args.extend(options.into_iter().map(|o| o.to_string()));

        let c_args: Vec<CString> = args
            .into_iter()
            .map(CString::new)
            .collect::<std::result::Result<_, _>>()?;
        let mut raw: Vec<*mut c_char> = c_args.iter().map(|c| c.as_ptr() as *mut c_char).collect();

        // SAFETY: `raw` holds `c_args.len()` valid pointers that outlive this
        // call (c_args is dropped after). chdb_connect copies what it needs.
        let conn = unsafe { ffi::chdb_connect(raw.len() as i32, raw.as_mut_ptr()) };

        match NonNull::new(conn) {
            Some(handle) => Ok(Connection { handle }),
            None => Err(Error::ConnectFailed),
        }
    }

    /// Run `query`, rendering output in `format` (e.g. `"CSV"`,
    /// `"TSVWithNames"`, `"JSON"`).
    ///
    /// # Errors
    ///
    /// Returns [`Error::NulByte`] if `query` or `format` contains an interior
    /// NUL byte, [`Error::NullResult`] if chdb returned no result, or
    /// [`Error::Query`] if the engine reported an error.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use chdb::Connection;
    ///
    /// let conn = Connection::new()?;
    /// let r = conn.query("SELECT 1 + 1", "CSV")?;
    /// assert_eq!(r.data_utf8()?.trim(), "2");
    /// # Ok::<(), chdb::Error>(())
    /// ```
    pub fn query(&self, query: &str, format: &str) -> Result<QueryResult> {
        let c_query = CString::new(query)?;
        let c_format = CString::new(format)?;

        // SAFETY: the connection handle is non-null and still open (we hold it).
        // chdb_query copies the query/format strings; our CStrings outlive the call.
        let result =
            unsafe { ffi::chdb_query(*self.handle.as_ptr(), c_query.as_ptr(), c_format.as_ptr()) };

        let result = match NonNull::new(result) {
            Some(r) => r,
            None => return Err(Error::NullResult),
        };

        // Ensure the C result is freed however we exit from here.
        let result = ResultGuard(result.as_ptr());

        // SAFETY: result is a valid chdb_result for the duration of this scope.
        let err = unsafe { ffi::chdb_result_error(result.0) };
        if !err.is_null() {
            // SAFETY: err is a non-null, NUL-terminated string owned by the result.
            let msg = unsafe { CStr::from_ptr(err) }
                .to_string_lossy()
                .into_owned();
            return Err(Error::Query(msg));
        }

        // SAFETY: result is valid; buffer/length describe a contiguous region
        // owned by the result. We copy it out so the Vec owns its data and the
        // C result can be freed by ResultGuard.
        let buf = unsafe {
            let ptr = ffi::chdb_result_buffer(result.0);
            let len = ffi::chdb_result_length(result.0);
            if ptr.is_null() || len == 0 {
                Vec::new()
            } else {
                std::slice::from_raw_parts(ptr as *const u8, len).to_vec()
            }
        };

        // SAFETY: result is valid for these metric reads.
        let (elapsed, rows_read, bytes_read) = unsafe {
            (
                ffi::chdb_result_elapsed(result.0),
                ffi::chdb_result_rows_read(result.0),
                ffi::chdb_result_bytes_read(result.0),
            )
        };

        Ok(QueryResult {
            buf,
            elapsed,
            rows_read,
            bytes_read,
        })
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        // SAFETY: handle came from chdb_connect and has not been closed yet;
        // chdb_close_conn takes the pointer-to-handle and frees it.
        unsafe { ffi::chdb_close_conn(self.handle.as_ptr()) }
    }
}

/// RAII guard that frees a `chdb_result` on scope exit.
struct ResultGuard(*mut ffi::ChdbResult);

impl Drop for ResultGuard {
    fn drop(&mut self) {
        // SAFETY: self.0 is a valid result handle from chdb_query that has not
        // been destroyed yet.
        unsafe { ffi::chdb_destroy_query_result(self.0) }
    }
}

//! Raw FFI declarations for the chdb (libchdb) C API.
//!
//! These match the connection-based API exported by `libchdb.so` from
//! [chdb-core](https://github.com/chdb-io/chdb-core) v26.5.0, declared in the
//! vendored `vendor/chdb/chdb.h`. The crate links against that shared object;
//! see `build.rs`.
//!
//! The older `query_stable` one-shot API is deprecated upstream and not bound
//! here — this crate uses the `chdb_connect` / `chdb_query` connection model.

use std::os::raw::{c_char, c_int};

/// Opaque connection handle. In C this is `struct chdb_connection_ *`, i.e. the
/// `chdb_connection` typedef is itself a pointer. `chdb_connect` returns a
/// pointer *to* one of these handles.
#[repr(C)]
pub struct ChdbConnection {
    _internal_data: *mut std::ffi::c_void,
}

/// Opaque query-result handle (`chdb_result`).
#[repr(C)]
pub struct ChdbResult {
    _internal_data: *mut std::ffi::c_void,
}

#[link(name = "chdb")]
extern "C" {
    /// Create a new connection. `argv` follows `clickhouse local` conventions;
    /// `--path=<dir>` selects an on-disk database, otherwise the path defaults
    /// to `:memory:`. Returns a pointer to the connection handle, or null on
    /// failure. Only one active connection is allowed per process.
    pub fn chdb_connect(argc: c_int, argv: *mut *mut c_char) -> *mut *mut ChdbConnection;

    /// Close a connection and release its resources. Takes the pointer-to-handle
    /// returned by [`chdb_connect`].
    pub fn chdb_close_conn(conn: *mut *mut ChdbConnection);

    /// Execute `query` on `conn`, rendering output in `format` (e.g. `"CSV"`).
    /// Returns a result handle; inspect it with the `chdb_result_*` accessors
    /// and free it with [`chdb_destroy_query_result`].
    pub fn chdb_query(
        conn: *mut ChdbConnection,
        query: *const c_char,
        format: *const c_char,
    ) -> *mut ChdbResult;

    /// Free a query result and all associated resources.
    pub fn chdb_destroy_query_result(result: *mut ChdbResult);

    /// Pointer to the result data buffer (not NUL-terminated; use with
    /// [`chdb_result_length`]).
    pub fn chdb_result_buffer(result: *mut ChdbResult) -> *mut c_char;

    /// Length of the result data buffer in bytes.
    pub fn chdb_result_length(result: *mut ChdbResult) -> usize;

    /// Query execution time in seconds.
    pub fn chdb_result_elapsed(result: *mut ChdbResult) -> f64;

    /// Number of rows in the result set.
    pub fn chdb_result_rows_read(result: *mut ChdbResult) -> u64;

    /// Bytes occupied by the result set in chdb's internal binary format.
    pub fn chdb_result_bytes_read(result: *mut ChdbResult) -> u64;

    /// Error message for the query, or null if it succeeded.
    pub fn chdb_result_error(result: *mut ChdbResult) -> *const c_char;
}

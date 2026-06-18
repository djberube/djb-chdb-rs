//! Engine argument builder.
//!
//! [`QueryOption`] models a single `clickhouse local` argument used when
//! opening a [`Connection`](crate::Connection). Build them directly or with the
//! [`flag!`] and [`option!`] macros.

use enum2str::EnumStr;

/// A single engine argument, rendered into the `argv` passed to chdb.
#[derive(Debug, Clone, EnumStr)]
pub enum QueryOption {
    /// A bare flag, e.g. `--readonly` → `Flag("readonly")`.
    #[enum2str("--{}")]
    Flag(&'static str),
    /// A key/value option, e.g. `--path=/tmp/db` → `Option("path", "/tmp/db")`.
    #[enum2str("--{}={}")]
    Option(&'static str, &'static str),
    /// A pre-formatted argument passed through verbatim (already includes any
    /// leading dashes). Useful for owned/dynamic strings.
    #[enum2str("{}")]
    Raw(String),
}

/// Build a [`QueryOption::Flag`].
///
/// # Examples
///
/// ```
/// let opt = chdb::flag!("readonly");
/// assert_eq!(opt.to_string(), "--readonly");
/// ```
#[macro_export]
macro_rules! flag {
    ($flag_name:expr) => {
        $crate::options::QueryOption::Flag($flag_name)
    };
}

/// Build a [`QueryOption::Option`].
///
/// # Examples
///
/// ```
/// let opt = chdb::option!("path", "/tmp/db");
/// assert_eq!(opt.to_string(), "--path=/tmp/db");
/// ```
#[macro_export]
macro_rules! option {
    ($option_name: expr, $option_value: expr) => {
        $crate::options::QueryOption::Option($option_name, $option_value)
    };
}

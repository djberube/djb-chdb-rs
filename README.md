# Chdb for rust

Use clickhouse as library, based on `clickhouse local`

### Requirements:

This crate links against `libchdb.so`, vendored under `vendor/chdb/` from
[chdb-core](https://github.com/chdb-io/chdb-core). The shared object is ~530MB
and is not committed; fetch it with:

```
./scripts/fetch-libchdb.sh
```

In the devenv shell this runs automatically on entry if the library is missing.

### Install

```
cargo add chdb
```
or add to Cargo.toml
```
chdb = "0.1"
```

Powered by:

- ClickHouse - https://clickhouse.com/
- Chdb - https://github.com/chdb-io/chdb
- libchdb - https://github.com/metrico/libchdb



## Usage

```rust
use chdb::Connection;

// In-memory database (`:memory:`). Pass a path to persist:
// `Connection::open("/tmp/mydb")`.
let conn = Connection::new().unwrap();

let result = conn
    .query("SELECT number FROM numbers(10)", "TSVWithNames")
    .unwrap();

println!("Elapsed: {}", result.elapsed);
println!("Rows: {}", result.rows_read);
println!("Bytes: {}", result.bytes_read);
println!("Result:\n{}", result.data_utf8().unwrap());
```

Outputs:
```
SELECT number FROM numbers(10)
Elapsed: 0.007413874
Rows: 10
Bytes: 80
Result:
number
0
1
2
3
4
5
6
7
8
9

```

## Options and flags

List of available options [here](OPTIONS.md)

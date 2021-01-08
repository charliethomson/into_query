
# into_query

## Usage

```rust
#[derive(IntoQuery)]
#[table_name = "whateverlol"]
pub struct Filter {
    foo: Option<String>,
    bar: Option<Vec<i32>>,
    baz: Option<Vec<u8>>,
}
```

## Features

`mysql`, `postgres`, `sqlite`: Set the backend to be used by the query builder
(default: `mysql`)

## Attributes
`table_name`: *required*
```
schema::<this>::dsl::<this>
```
`schema_prefix`: *optional* (default: `crate`)
```
<this>::schema::<table_name>::dsl
```
For example, if your project was structured as
```
src
│   main.rs
└───db
    └───schema.rs
```
and your table was "cats"
```rust
#[derive(IntoQuery)]
#[table_name = "cats"]
#[schema_prefix = "crate::db"]
struct CatOptions {
    ..
}

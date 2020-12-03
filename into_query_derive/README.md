# into_query_derive

### Disclaimer
*This crate is intended to be used only via the `derive` feature on (into_query)[https://crates.io/into_query].*

## Usage
```rust
#[derive(IntoQuery)]
pub struct Filter {
    foo: Option<String>,
    bar: Option<Vec<i32>>,
    baz: Option<Vec<u8>>,
}
```
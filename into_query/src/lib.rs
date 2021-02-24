/// Provides the `into_query` function, which converts the type into a select statement filtering by present fields.
/// T is the table the select statement will filter by.
pub trait IntoQuery<T, DB>
where
    T: diesel::Table + diesel::query_builder::AsQuery,
    T::Query: diesel::query_dsl::methods::BoxedDsl<'static, DB>,
    DB: diesel::backend::Backend,
{
    /// Convert `self` into a query
    fn into_query(self) -> diesel::helper_types::IntoBoxed<'static, T, DB>;
}

#[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
pub use into_query_derive::*;

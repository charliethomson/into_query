/// Provides the `into_query` function, which converts the type into a select statement filtering by present fields.
/// T is the table the select statement will filter by.
pub trait IntoQuery<T>
where
    T: diesel::Table + diesel::query_builder::AsQuery,
    T::Query: diesel::query_dsl::methods::BoxedDsl<'static, diesel::mysql::Mysql>,
{
    /// Convert `self` into a query
    fn into_query(self) -> diesel::helper_types::IntoBoxed<'static, T, diesel::mysql::Mysql>;
}

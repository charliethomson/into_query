
pub trait IntoQuery<T>
where
    T: diesel::Table + diesel::query_builder::AsQuery,
    T::Query: diesel::query_dsl::methods::BoxedDsl<'static, diesel::mysql::Mysql>,
{
    fn into_query(self) -> diesel::helper_types::IntoBoxed<'static, T, diesel::mysql::Mysql>;
}

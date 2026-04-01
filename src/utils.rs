pub fn pool_from(connection_str: impl ToString) -> crate::Result<deadpool_postgres::Pool> {
    let cfg = deadpool_postgres::Config {
        url: Some(connection_str.to_string()),
        manager: Some(deadpool_postgres::ManagerConfig {
            recycling_method: deadpool_postgres::RecyclingMethod::Verified,
        }),
        ..Default::default()
    };
    cfg.create_pool(
        Some(deadpool_postgres::Runtime::Tokio1),
        deadpool_postgres::tokio_postgres::NoTls,
    )
    .map_err(Into::into)
}

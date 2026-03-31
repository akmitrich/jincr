impl super::Manager {
    pub async fn connect_env() -> crate::Result<Self> {
        let pg_url = std::env::var("POSTGRES_URL")
            .unwrap_or_else(|_| "postgres://postgres:password@localhost/jsonstore".to_string());
        let pg_pool = pool_from(pg_url)?;
        tracing::debug!(?pg_pool, "start");
        Ok(Self { pg_pool })
    }

    pub async fn get_client(&self) -> crate::Result<deadpool_postgres::Object> {
        let c = self.pg_pool.get().await?;
        Ok(c)
    }
}

fn pool_from(connection_str: impl ToString) -> crate::Result<deadpool_postgres::Pool> {
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

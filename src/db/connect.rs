use crate::utils;

impl super::Manager {
    pub fn connect_env() -> crate::Result<Self> {
        let pg_url = std::env::var("POSTGRES_URL")
            .unwrap_or_else(|_| "postgres://postgres:password@localhost/jsonstore".to_string());
        Self::connect_to(pg_url)
    }

    pub fn connect_to(pg_url: impl ToString) -> crate::Result<Self> {
        let pg_pool = utils::pool_from(pg_url)?;
        tracing::debug!(?pg_pool, "start");
        Ok(Self { pg_pool })
    }

    pub async fn get_client(&self) -> crate::Result<deadpool_postgres::Object> {
        let c = self.pg_pool.get().await?;
        Ok(c)
    }
}

use deadpool_postgres::tokio_postgres;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub enum Error {
    ConnectPostgres(#[from] deadpool_postgres::CreatePoolError),
    DeadpoolPool(#[from] deadpool_postgres::PoolError),
    Io(#[from] std::io::Error),
    Postgres(#[from] tokio_postgres::Error),
}

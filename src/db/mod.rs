mod connect;
mod ensure;
mod operate;

#[derive(Debug, Clone)]
pub struct Manager {
    pg_pool: deadpool_postgres::Pool,
}

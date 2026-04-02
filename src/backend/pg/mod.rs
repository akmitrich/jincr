mod connect;
mod operate;

#[derive(Debug, Clone)]
pub struct Pg {
    pg_pool: deadpool_postgres::Pool,
}

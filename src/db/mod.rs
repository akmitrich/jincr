mod connect;
mod ensure;

#[derive(Debug)]
pub struct Manager {
    pg_pool: deadpool_postgres::Pool,
}

impl Manager {
    pub async fn save_op(&self, name: impl std::fmt::Display, op: crate::Op) -> crate::Result<()> {
        let client = self.get_client().await?;
        let res = client
            .query_opt(
                &format!("INSERT INTO {name} (path, kind, value, info, timestamp) VALUES ($1,$2,$3,$4,$5);"),
                &[&op.path, &op.kind, &op.value, &op.info, &op.timestamp],
            )
            .await?;
        dbg!(res);
        Ok(())
    }
}

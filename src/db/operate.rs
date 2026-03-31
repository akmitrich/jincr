impl super::Manager {
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

    pub async fn ops_from_last_snapshot(
        &self,
        name: impl std::fmt::Display,
    ) -> crate::Result<Box<[crate::Op]>> {
        let client = self.get_client().await?;
        let resp = client
            .query_one(
                &format!("SELECT * FROM {name} WHERE kind='snap' ORDER BY timestamp DESC LIMIT 1"),
                &[],
            )
            .await?;
        let ts = resp.get::<_, chrono::DateTime<chrono::Local>>("timestamp");
        let mut last_snap = vec![crate::Op::from_postgres(&resp)];
        tracing::info!(?last_snap);
        let rows = client
            .query(
                &format!("SELECT * FROM {name} WHERE timestamp > $1 ORDER BY TIMESTAMP ASC"),
                &[&ts],
            )
            .await?;
        last_snap.extend(rows.iter().map(crate::Op::from_postgres));
        tracing::info!(rows = rows.len());
        Ok(last_snap.into_boxed_slice())
    }
}

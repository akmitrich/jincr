use serde_json::Value;

impl super::Manager {
    pub async fn save_op(&self, name: impl std::fmt::Display, op: crate::Op) -> crate::Result<()> {
        let client = self.get_client().await?;
        let _ = client
            .query_opt(
                &format!("INSERT INTO {name} (path, kind, value, info, timestamp) VALUES ($1,$2,$3,$4,$5);"),
                &[&op.path, &op.kind, &op.value, &op.info, &op.timestamp],
            )
            .await?;
        Ok(())
    }

    pub async fn ops_from_last_snapshot(
        &self,
        name: impl std::fmt::Display,
    ) -> crate::Result<Box<[crate::Op]>> {
        let client = self.get_client().await?;
        let rows = client
            .query(
                &format!("SELECT * FROM {name} WHERE kind='snap' ORDER BY timestamp DESC LIMIT 1"),
                &[],
            )
            .await?;
        let (mut last_snap, rows) = if let Some(resp) = rows.first() {
            let ts = resp.get::<_, chrono::DateTime<chrono::Local>>("timestamp");
            (
                vec![crate::Op::from_postgres(resp)],
                client
                    .query(
                        &format!(
                            "SELECT * FROM {name} WHERE timestamp > $1 ORDER BY TIMESTAMP ASC"
                        ),
                        &[&ts],
                    )
                    .await?,
            )
        } else {
            (
                vec![],
                client
                    .query(&format!("SELECT * FROM {name} ORDER BY TIMESTAMP ASC"), &[])
                    .await?,
            )
        };
        last_snap.extend(rows.iter().map(crate::Op::from_postgres));
        tracing::info!(?last_snap);
        Ok(last_snap.into_boxed_slice())
    }

    pub async fn document(&self, name: impl std::fmt::Display) -> crate::Result<Value> {
        let ops = self.ops_from_last_snapshot(name).await?;
        let obj = crate::op::document(ops);
        Ok(obj)
    }
}

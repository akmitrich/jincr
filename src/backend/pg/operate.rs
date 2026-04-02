use crate::backend::BackendOperate;

#[async_trait::async_trait]
impl BackendOperate for super::Pg {
    async fn start_document<N: std::fmt::Display + Send + Sync>(
        &self,
        name: N,
    ) -> crate::Result<()> {
        let client = self.get_client().await?;
        let _ = client
            .simple_query("CREATE TYPE kind AS ENUM ('snap', 'replace', 'delete', 'add')")
            .await
            .inspect(|_| tracing::info!("Created type for 'kind' enum"));
        client
            .simple_query(&format!(
                "CREATE TABLE IF NOT EXISTS {name}(
                path TEXT,
                kind kind NOT NULL,
                value JSONB,
                info TEXT,
                timestamp timestamptz NOT NULL
            )"
            ))
            .await
            .inspect_err(|e| tracing::error!("creating table `{name}`. {e:?}"))?;
        client
            .simple_query(&format!(
                "CREATE INDEX IF NOT EXISTS kind_fast_lookup ON {name} USING HASH (kind)"
            ))
            .await
            .inspect_err(|e| tracing::error!("creating index on `kind`. {e:?}"))?;
        client
            .simple_query(&format!(
                "CREATE INDEX IF NOT EXISTS idx_timestamp_range ON {name} (timestamp)"
            ))
            .await
            .inspect_err(|e| tracing::error!("creating index on `timestamp`. {e:?}"))?;
        Ok(())
    }

    async fn save_op<N: std::fmt::Display + Send + Sync>(
        &self,
        name: N,
        op: crate::Op,
    ) -> crate::Result<()> {
        let client = self.get_client().await?;
        let _ = client
            .query_opt(
                &format!("INSERT INTO {name} (path, kind, value, info, timestamp) VALUES ($1,$2,$3,$4,$5)"),
                &[&op.path, &op.kind, &op.value, &op.info, &op.timestamp],
            )
            .await?;
        Ok(())
    }

    async fn last_snapshot<N: std::fmt::Display + Send + Sync>(
        &self,
        name: N,
    ) -> crate::Result<Option<crate::Op>> {
        let client = self.get_client().await?;
        let rows = client
            .query(
                &format!("SELECT * FROM {name} WHERE kind='snap' ORDER BY timestamp DESC LIMIT 1"),
                &[],
            )
            .await?;

        Ok(rows.first().map(crate::Op::from_postgres))
    }

    async fn ops_after<N: std::fmt::Display + Send + Sync>(
        &self,
        name: N,
        ts: Option<chrono::DateTime<chrono::Local>>,
    ) -> crate::Result<Vec<crate::Op>> {
        let client = self.get_client().await?;
        let start = ts.unwrap_or_default();
        let result = client
            .query(
                &format!("SELECT * FROM {name} WHERE timestamp > $1 ORDER BY timestamp ASC"),
                &[&start],
            )
            .await?
            .iter()
            .map(crate::Op::from_postgres)
            .collect();
        Ok(result)
    }
}

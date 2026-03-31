impl super::Manager {
    pub async fn create_table(&self, name: impl std::fmt::Display) -> crate::Result<()> {
        let client = self.get_client().await?;
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
}

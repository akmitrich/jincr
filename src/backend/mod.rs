mod pg;

#[async_trait::async_trait]
pub trait BackendOperate {
    async fn start_document<N: std::fmt::Display + Send + Sync>(
        &self,
        name: N,
    ) -> crate::Result<()>;

    async fn save_op<N: std::fmt::Display + Send + Sync>(
        &self,
        name: N,
        op: crate::Op,
    ) -> crate::Result<()>;

    async fn last_snapshot<N: std::fmt::Display + Send + Sync>(
        &self,
        name: N,
    ) -> crate::Result<Option<crate::Op>>;

    async fn ops_after<N: std::fmt::Display + Send + Sync>(
        &self,
        name: N,
        ts: Option<chrono::DateTime<chrono::Local>>,
    ) -> crate::Result<Vec<crate::Op>>;

    async fn ops_from_last_snapshot<N: std::fmt::Display + Send + Sync>(
        &self,
        name: N,
    ) -> crate::Result<Box<[crate::Op]>> {
        let maybe_snapshot = self.last_snapshot(&name).await?;
        let ts = maybe_snapshot.as_ref().map(|snap| snap.timestamp);
        let mut target_ops = maybe_snapshot.map(|snap| vec![snap]).unwrap_or_default();
        target_ops.extend(self.ops_after(name, ts).await?);
        Ok(target_ops.into_boxed_slice())
    }

    async fn document<N: std::fmt::Display + Send + Sync>(
        &self,
        name: N,
    ) -> crate::Result<serde_json::Value> {
        let ops = self.ops_from_last_snapshot(name).await?;
        let doc = crate::op::document(ops);
        Ok(doc)
    }
}

pub use pg::Pg;

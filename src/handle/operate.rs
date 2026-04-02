use crate::backend::BackendOperate;
use serde_json::Value;

impl<B: BackendOperate> super::Document<B> {
    pub async fn add(&self, path: impl ToString, value: impl Into<Value>) -> crate::Result<()> {
        self.db
            .save_op(
                &self.name,
                crate::op::Kind::Add
                    .builder()
                    .path(path)
                    .value(value)
                    .build(),
            )
            .await
    }

    pub async fn add_with_info(
        &self,
        path: impl ToString,
        value: impl Into<Value>,
        info: impl ToString,
    ) -> crate::Result<()> {
        self.db
            .save_op(
                &self.name,
                crate::op::Kind::Add
                    .builder()
                    .path(path)
                    .value(value)
                    .info(info)
                    .build(),
            )
            .await
    }

    pub async fn replace(&self, path: impl ToString, value: impl Into<Value>) -> crate::Result<()> {
        self.db
            .save_op(
                &self.name,
                crate::op::Kind::Replace
                    .builder()
                    .path(path)
                    .value(value)
                    .build(),
            )
            .await
    }

    pub async fn replace_with_info(
        &self,
        path: impl ToString,
        value: impl Into<Value>,
        info: impl ToString,
    ) -> crate::Result<()> {
        self.db
            .save_op(
                &self.name,
                crate::op::Kind::Replace
                    .builder()
                    .path(path)
                    .value(value)
                    .info(info)
                    .build(),
            )
            .await
    }

    pub async fn delete(&self, path: impl ToString) -> crate::Result<()> {
        self.db
            .save_op(
                &self.name,
                crate::op::Kind::Delete.builder().path(path).build(),
            )
            .await
    }

    pub async fn delete_with_info(
        &self,
        path: impl ToString,
        info: impl ToString,
    ) -> crate::Result<()> {
        self.db
            .save_op(
                &self.name,
                crate::op::Kind::Delete
                    .builder()
                    .path(path)
                    .info(info)
                    .build(),
            )
            .await
    }
}

impl<B: BackendOperate + Send + Sync> super::Document<B> {
    pub async fn snapshot(&self) -> crate::Result<()> {
        let snap = self.db.document(&self.name).await?;
        self.db
            .save_op(
                &self.name,
                crate::op::Kind::Snap.builder().value(snap).build(),
            )
            .await
    }

    pub async fn snapshot_with_info(&self, info: impl ToString) -> crate::Result<()> {
        let snap = self.db.document(&self.name).await?;
        self.db
            .save_op(
                &self.name,
                crate::op::Kind::Snap
                    .builder()
                    .value(snap)
                    .info(info)
                    .build(),
            )
            .await
    }
}

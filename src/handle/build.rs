use crate::backend::BackendOperate;
use serde_json::Value;

impl<B: BackendOperate> super::Document<B> {
    pub async fn start(db: B, name: impl ToString) -> crate::Result<Self> {
        let name = name.to_string();
        db.start_document(&name).await?;
        Ok(Self::restore(db, name))
    }

    pub fn restore(db: B, name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            db,
        }
    }
}

impl<B: BackendOperate + Send + Sync> super::Document<B> {
    pub async fn finish(self) -> crate::Result<Value> {
        self.db.document(self.name).await
    }
}

use serde_json::Value;

impl super::Document {
    pub async fn start(db: crate::db::Manager, name: impl ToString) -> crate::Result<Self> {
        let name = name.to_string();
        db.create_table(&name).await?;
        Ok(Self::restore(db, name))
    }

    pub fn restore(db: crate::db::Manager, name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            db,
        }
    }

    pub async fn finish(self) -> crate::Result<Value> {
        self.db.document(self.name).await
    }
}

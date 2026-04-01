mod build;
mod operate;

pub struct Document {
    name: String,
    db: crate::db::Manager,
}

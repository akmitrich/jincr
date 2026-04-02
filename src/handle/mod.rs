mod build;
mod operate;

pub struct Document<B> {
    name: String,
    db: B,
}

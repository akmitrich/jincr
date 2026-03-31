#[derive(Debug, Clone, Copy, postgres_types::ToSql, postgres_types::FromSql)]
#[postgres(name = "kind")]
pub enum Kind {
    #[postgres(name = "snap")]
    Snap,
    #[postgres(name = "replace")]
    Replace,
    #[postgres(name = "delete")]
    Delete,
    #[postgres(name = "add")]
    Add,
}

impl Kind {
    pub fn builder(self) -> super::Builder {
        super::Builder::new(self)
    }
}

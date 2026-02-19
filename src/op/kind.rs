#[derive(Debug, Clone, Copy)]
pub enum Kind {
    Snap,
    Replace,
    Delete,
    Add,
}

impl Kind {
    pub fn builder(self) -> super::Builder {
        super::Builder::new(self)
    }
}

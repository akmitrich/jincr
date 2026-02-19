use super::Kind;
use serde_json::Value;

#[derive(Debug)]
pub struct Builder {
    kind: Kind,
    path: Option<String>,
    value: Option<Value>,
    info: Option<String>,
}

impl Builder {
    pub fn path(mut self, path: impl ToString) -> Self {
        if !matches!(self.kind, Kind::Snap) {
            self.path = Some(path.to_string());
        }
        self
    }

    pub fn value(mut self, value: impl Into<Value>) -> Self {
        if !matches!(self.kind, Kind::Delete) {
            self.value = Some(value.into());
        }
        self
    }

    pub fn info(mut self, info: impl ToString) -> Self {
        self.info = Some(info.to_string());
        self
    }

    pub fn build(self) -> super::Op {
        super::Op::new(
            self.kind,
            self.path,
            self.value,
            chrono::Local::now(),
            self.info,
        )
    }
}

impl Builder {
    pub(super) fn new(kind: Kind) -> Self {
        Self {
            kind,
            path: None,
            value: None,
            info: None,
        }
    }
}

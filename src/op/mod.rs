mod create;
mod kind;
mod operation_builder;

pub use create::document;
pub use kind::Kind;
pub use operation_builder::Builder;

#[derive(Debug)]
pub struct Op {
    pub path: Option<String>,
    pub kind: Kind,
    pub value: Option<serde_json::Value>,
    pub timestamp: chrono::DateTime<chrono::Local>,
    pub info: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn build_operations_with_builder() {
        let ops = [
            Kind::Add.builder().path("").value(json!({"abc":true})),
            Kind::Add.builder().path("num").value(55),
            Kind::Snap.builder().value(json!({"abc":{"tag":"rust"}})),
            Kind::Delete.builder().path("abc.tag"),
            Kind::Replace.builder().path("abc").value(442),
            Kind::Replace.builder().path("abc.tag").value(true),
        ];
        let doc = document(ops.into_iter().map(Builder::build));
        assert_eq!(json!({"abc":442}), doc);
    }
}

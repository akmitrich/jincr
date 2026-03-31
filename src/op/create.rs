use super::Kind;
use serde_json::Value;

pub fn document<I>(ops: I) -> Value
where
    I: IntoIterator<Item = super::Op>,
{
    let mut result = Value::Null;
    for op in ops.into_iter() {
        tracing::trace!("apply {op:?}");
        match op.kind {
            Kind::Snap => {
                if let Some(val) = op.value {
                    result = val;
                }
            }
            Kind::Replace => {
                if let Some(path) = op.path
                    && let Some(target) = jvars::basic::get_mut(&mut result, path)
                    && let Some(new) = op.value
                {
                    *target = new;
                }
            }
            Kind::Delete => {
                if let Some(path) = op.path {
                    jvars::basic::delete(&mut result, path);
                }
            }
            Kind::Add => {
                if let Some(ref path) = op.path
                    && let Some(value) = op.value
                {
                    let _ = jvars::basic::update_or_create(&mut result, path, value)
                        .inspect_err(|e| tracing::error!("updating {path:?}. {e:?}"));
                }
            }
        }
        tracing::trace!("current: {result:?}");
    }
    result
}

impl super::Op {
    pub(super) fn new(
        kind: Kind,
        path: Option<String>,
        value: Option<Value>,
        timestamp: chrono::DateTime<chrono::Local>,
        info: Option<String>,
    ) -> Self {
        Self {
            kind,
            path,
            value,
            timestamp,
            info,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Op;
    use serde_json::json;

    #[test]
    fn it_works() {
        let ops = [
            Op {
                kind: Kind::Add,
                path: Some("".to_string()),
                value: Some(json!({"abc":true})),
                timestamp: chrono::Local::now(),
                info: Some("0th operation".to_string()),
            },
            Op {
                kind: Kind::Add,
                path: Some("num".to_string()),
                value: Some(json!(55)),
                timestamp: chrono::Local::now(),
                info: Some("assign num".to_string()),
            },
            Op {
                kind: Kind::Snap,
                path: None,
                value: Some(json!({"abc":{"tag":"rust"}})),
                timestamp: chrono::Local::now(),
                info: Some("snapshot".to_string()),
            },
            Op {
                kind: Kind::Delete,
                path: Some("abc.tag".to_string()),
                value: None,
                timestamp: chrono::Local::now(),
                info: Some("delete abc".to_string()),
            },
            Op {
                kind: Kind::Replace,
                path: Some("abc".to_string()),
                value: Some(442.into()),
                timestamp: chrono::Local::now(),
                info: Some("replace empty to 442".to_string()),
            },
            Op {
                kind: Kind::Replace,
                path: Some("abc.tag".to_string()),
                value: Some(true.into()),
                timestamp: chrono::Local::now(),
                info: Some("replace empty to 442".to_string()),
            },
        ];
        let doc = document(ops);
        assert_eq!(json!({"abc":442}), doc);
    }
}

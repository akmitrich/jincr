use serde_json::Value;

#[derive(Debug)]
pub struct Op {
    path: Option<String>,
    value: Option<Value>,
    timestamp: chrono::DateTime<chrono::Local>,
    info: Option<String>,
}

pub fn document<I>(ops: I) -> Value
where
    I: IntoIterator<Item = Op>,
{
    let mut result = Value::Null;
    for op in ops.into_iter() {
        tracing::trace!("apply {op:?}");
        match op.path {
            None => {
                if let Some(val) = op.value {
                    result = val;
                }
            }
            Some(ref path) => {
                if let Some(target) = dbg!(jvars::get_mut(&mut result, path)) {
                    match op.value {
                        Some(new) => *target = new,
                        None => {
                            jvars::delete(&mut result, path);
                        }
                    }
                } else if let Some(value) = op.value {
                    let _ = jvars::update_or_create(&mut result, path, value)
                        .inspect_err(|e| tracing::error!("updating {path:?}. {e:?}"));
                }
            }
        }
        println!("current: {result:?}");
    }
    result
}

pub fn snapshot(doc: Value) -> Op {
    dbg!(Op {
        path: None,
        value: Some(doc),
        timestamp: chrono::Local::now(),
        info: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn it_works() {
        let ops = [
            Op {
                path: Some("".to_string()),
                value: Some(json!({"abc":true})),
                timestamp: chrono::Local::now(),
                info: Some("0th operation".to_string()),
            },
            Op {
                path: Some("num".to_string()),
                value: Some(json!(55)),
                timestamp: chrono::Local::now(),
                info: Some("assign num".to_string()),
            },
            snapshot(json!({"abc":{"tag":"rust"}})),
            Op {
                path: Some("abc.tag".to_string()),
                value: None,
                timestamp: chrono::Local::now(),
                info: Some("delete abc".to_string()),
            },
        ];
        let doc = document(ops);
        println!("Final {doc:#?}");
    }
}

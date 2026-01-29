use serde_json::Value;

#[derive(Debug)]
pub struct Op {
    path: String,
    old_value: Option<Value>,
    new_value: Option<Value>,
    timestamp: chrono::DateTime<chrono::Local>,
    info: Info,
}

#[derive(Debug)]
pub enum Info {
    Snapshot,
    Remark(String),
}

pub fn document<I>(ops: I) -> Value
where
    I: IntoIterator<Item = Op>,
{
    let mut result = Value::Null;
    for op in ops.into_iter() {
        println!("apply {op:?}");
        match op.info {
            Info::Snapshot => {
                if op.new_value.is_none()
                    && let Some(val) = op.old_value
                {
                    if op.path.is_empty() {
                        result = val;
                    } else if let Some(target) = jvars::get_mut(&mut result, &op.path) {
                        *target = val
                    }
                }
            }
            _ => {
                if let Some(target) = dbg!(jvars::get_mut(&mut result, &op.path)) {
                    match (op.old_value, op.new_value) {
                        (None, None) => continue,
                        (_, Some(new)) => *target = new,
                        (Some(ref old), None) => {
                            if let Some(target) = jvars::get_mut(&mut result, &op.path)
                                && target == old
                            {
                                *target = Value::Null
                            }
                        }
                    }
                } else if op.old_value.is_none()
                    && let Some(new) = op.new_value
                {
                    let _ = jvars::update_or_create(&mut result, &op.path, new);
                }
            }
        }
        println!("current: {result:?}");
    }
    result
}

pub fn snapshot(path: Option<impl ToString>, doc: Value) -> Op {
    dbg!(Op {
        path: path.as_ref().map(ToString::to_string).unwrap_or_default(),
        old_value: Some(doc),
        new_value: None,
        timestamp: chrono::Local::now(),
        info: Info::Snapshot,
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
                path: "".to_string(),
                old_value: Some(json!({"abc":true})),
                new_value: None,
                timestamp: chrono::Local::now(),
                info: Info::Snapshot,
            },
            Op {
                path: "num".to_string(),
                old_value: None,
                new_value: Some(json!(55)),
                timestamp: chrono::Local::now(),
                info: Info::Remark(String::new()),
            },
            snapshot(Some("abc"), json!({"tag":"rust"})),
            Op {
                path: "abc.tag".to_string(),
                old_value: Some(json!("rust")),
                new_value: None,
                timestamp: chrono::Local::now(),
                info: Info::Remark(String::new()),
            },
        ];
        let doc = document(ops);
        println!("{doc:#?}");
    }
}

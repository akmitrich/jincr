use jincr::Op;
use serde_json::json;

fn main() {
    jincr::log::init_env();
    let ops = [
        Op::builder()
            .add("", json!({"abc":true}))
            .info("0th operation")
            .build(),
        Op::builder()
            .add("num", json!(55))
            .info("assign num")
            .build(),
        Op::builder()
            .snapshot(json!({"abc":{"tag":"rust"}}))
            .build(),
        Op::builder().del("abc.tag").info("delete abc").build(),
    ];
    let doc = jincr::op::document(ops);
    tracing::info!("doc={doc:#?}");
}

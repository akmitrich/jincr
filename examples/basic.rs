use jincr::Op;
use serde_json::json;

fn main() {
    jincr::log::init_env();
    let ops = [
        Op::builder()
            .path("")
            .value(json!({"abc":true}))
            .info("0th operation")
            .build(),
        Op::builder()
            .path("num")
            .value(json!(55))
            .info("assign num")
            .build(),
        // snapshot(json!({"abc":{"tag":"rust"}})),
        Op::builder().path("abc.tag").info("delete abc").build(),
    ];
    let doc = jincr::op::document(ops);
    tracing::info!("doc={doc:#?}");
}

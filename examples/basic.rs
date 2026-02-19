use jincr::op;
use serde_json::json;

fn main() {
    jincr::log::init_env();
    let ops = [
        op::Kind::Add
            .builder()
            .path("")
            .value(json!({"abc":true}))
            .info("0th operation"),
        op::Kind::Add
            .builder()
            .path("num")
            .value(json!(55))
            .info("assign num"),
        op::Kind::Snap
            .builder()
            .value(json!({"abc":{"tag":"rust"}})),
        op::Kind::Delete
            .builder()
            .path("abc.tag")
            .info("delete abc"),
        op::Kind::Replace
            .builder()
            .path("abc")
            .value(442)
            .info("real replace"),
        op::Kind::Replace
            .builder()
            .path("abc.name")
            .value(true)
            .info("ignored replace"),
    ];
    let doc = op::document(ops.into_iter().map(op::Builder::build));
    tracing::info!("doc={doc:#?}");
}

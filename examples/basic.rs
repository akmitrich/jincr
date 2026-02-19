use jincr::{
    Op,
    op::{self, OpBuilder},
};
use serde_json::json;

fn main() {
    jincr::log::init_env();
    let ops = [
        Op::builder(op::Kind::Add)
            .add("", json!({"abc":true}))
            .info("0th operation"),
        Op::builder(op::Kind::Add)
            .add("num", json!(55))
            .info("assign num"),
        Op::builder(op::Kind::Snap).snapshot(json!({"abc":{"tag":"rust"}})),
        Op::builder(op::Kind::Delete)
            .del("abc.tag")
            .info("delete abc"),
    ];
    let doc = jincr::op::document(ops.into_iter().map(OpBuilder::build));
    tracing::info!("doc={doc:#?}");
}

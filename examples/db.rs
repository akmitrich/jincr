#[tokio::main]
async fn main() {
    jincr::log::init_env();
    let store = jincr::db::Manager::connect_env().unwrap();
    store.create_table("test").await.unwrap();
    let ops = store.ops_from_last_snapshot("test").await.unwrap();
    let doc = jincr::op::document(ops);
    tracing::info!(%doc, "OK");
}

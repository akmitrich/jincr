#[tokio::main]
async fn main() {
    jincr::log::init_env();
    let store = jincr::db::Manager::connect_env().await.unwrap();
    store.create_table("test").await.unwrap();
    store
        .save_op(
            "test",
            jincr::op::Kind::Snap
                .builder()
                .value(serde_json::Value::String("мама мыла раму".to_string()))
                .build(),
        )
        .await
        .unwrap();
    tracing::info!(?store, "OK");
}

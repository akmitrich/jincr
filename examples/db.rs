#[tokio::main]
async fn main() {
    jincr::log::init_env();
    let store = jincr::backend::Pg::connect_env().unwrap();
    let handle = jincr::handle::Document::restore(store, "test");
    let doc = handle.finish().await.unwrap();
    tracing::info!(%doc, "OK");
}

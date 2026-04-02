#[tokio::main]
async fn main() {
    jincr::log::init_env();
    let addr = "0.0.0.0:8000";
    let lst = tokio::net::TcpListener::bind(addr).await.unwrap();
    jincr::server::run::with_listener(lst)
        .unwrap()
        .await
        .unwrap();
    tracing::info!("Stopped");
}

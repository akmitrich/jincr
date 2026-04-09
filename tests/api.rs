use serde_json::Value;

mod common;

#[tokio::test]
async fn server_is_up() {
    let b = common::back::TestBackend::setup().await;
    let client = common::spawn::TestClient::spawn_server(b.create_pg()).await;
    let resp = client
        .get(format!("{}/health", client.base_url()))
        .send()
        .await
        .unwrap();
    assert!(resp.status().is_success());
    println!("{:#?}", resp.json::<Value>().await.unwrap());
}

#[tokio::test]
async fn start_document_only_once() {
    let b = common::back::TestBackend::setup().await;
    let client = common::spawn::TestClient::spawn_server(b.create_pg()).await;
    let resp = client
        .post(format!("{}/api/v1/new/test", client.base_url()))
        .send()
        .await
        .unwrap();
    assert!(resp.status().is_success());
    println!("[OK] {:#?}", resp);
    let resp = client
        .post(format!("{}/api/v1/new/test", client.base_url()))
        .send()
        .await
        .unwrap();
    assert!(resp.status().is_client_error());
    println!("[Should fail] {:#?}", resp);
}

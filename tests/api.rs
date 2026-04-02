use serde_json::Value;

mod common;

#[tokio::test]
async fn server_is_up() {
    let client = common::spawn::TestClient::spawn_server().await;
    let resp = client
        .get(format!("{}/health", client.base_url()))
        .send()
        .await
        .unwrap();
    assert!(resp.status().is_success());
    println!("{:#?}", resp.json::<Value>().await.unwrap());
}

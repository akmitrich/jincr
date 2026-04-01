mod common;

#[tokio::test]
async fn it_works() {
    let app = common::setup_test_app().await;
    let db = jincr::db::Manager::connect_to(&app.database_url).unwrap();
    let name = "test";
    let value = "мама мыла раму";
    db.create_table(name).await.unwrap();
    db.save_op(name, jincr::op::Kind::Add.builder().value(value).build())
        .await
        .unwrap();
    let doc = db.document(name).await.unwrap();
    println!("value={doc}",);
    assert_eq!(value, doc);
}

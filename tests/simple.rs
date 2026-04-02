use jincr::backend::BackendOperate;

mod common;

#[tokio::test]
async fn it_works() {
    let app = common::back::TestBackend::setup().await;
    let db = jincr::backend::Pg::connect_to(&app.database_url).unwrap();
    let name = "test";
    let value = "мама мыла раму";
    db.start_document(name).await.unwrap();
    db.save_op(name, jincr::op::Kind::Add.builder().value(value).build())
        .await
        .unwrap();
    let doc = db.document(name).await.unwrap();
    println!("value={doc}",);
    assert_eq!(value, doc);
}

#[tokio::test]
async fn fairy_action() {
    let app = common::back::TestBackend::setup().await;
    let db = jincr::backend::Pg::connect_to(&app.database_url).unwrap();
    let doc = jincr::handle::Document::start(db.clone(), "test")
        .await
        .unwrap();
    doc.add_with_info("", "мама мыла раму", "never used")
        .await
        .unwrap();
    doc.add("action", "мама мыла раму").await.unwrap();
    doc.replace_with_info("nopath", 42, "hanging op")
        .await
        .unwrap();
    let incr1 = doc.finish().await.unwrap();
    let doc = jincr::handle::Document::restore(db.clone(), "test");
    doc.snapshot().await.unwrap();
    let snap = doc.finish().await.unwrap();
    assert_eq!(snap, incr1);
    let doc = jincr::handle::Document::restore(db.clone(), "test");
    doc.add("color", "зелёный").await.unwrap();
    doc.add_with_info(
        "fairy",
        serde_json::json!({"color": "зелёный", "action": "мама мыла раму"}),
        "dup inside",
    )
    .await
    .unwrap();
    doc.replace("fairy.color", "чёрный").await.unwrap();
    doc.delete("color").await.unwrap();
    doc.delete_with_info("fairy.color", "delete inner")
        .await
        .unwrap();
    let result = doc.finish().await.unwrap();
    assert_eq!(
        jvars::basic::get(&result, "action"),
        jvars::basic::get(&result, "fairy.action")
    );
}

use crate::backend::BackendOperate;
use actix_web::web;
use serde_json::Value;

#[actix_web::get("/health")]
async fn health(start_time: web::Data<chrono::DateTime<chrono::Local>>) -> actix_web::HttpResponse {
    let alive = chrono::Local::now()
        .signed_duration_since(start_time.as_ref())
        .to_std();
    actix_web::HttpResponse::Ok().json(serde_json::json!({
        "started": format!("{}", start_time.format("%d/%m/%y %T")),
        "alive": alive.map(|duration| format!("{duration:?}"))
            .unwrap_or_else(|_| String::from("NA")),
    }))
}

pub async fn start_doc<B: BackendOperate>(
    name: web::Path<String>,
    initial: Option<web::Json<Value>>,
    query: web::Query<Value>,
    backend: web::Data<B>,
) -> crate::Result<()> {
    let name = name.as_str();
    tracing::info!(name, ?initial, %query, "start document");
    backend.start_document(name).await?;
    if let Some(doc) = initial {
        let mut op = crate::op::Kind::Snap.builder().value(doc.0);
        if let Some(info) = query.get("info").and_then(Value::as_str) {
            op = op.info(info);
        }
        backend.save_op(name, op.build()).await?;
    }
    Ok(())
}

pub async fn add<B: BackendOperate>(
    name: web::Path<String>,
    value: web::Json<Value>,
    query: web::Query<Value>,
    backend: web::Data<B>,
) -> crate::Result<()> {
    tracing::info!(%name, %value, %query, "add");
    let Some(data_path) = query.get("data_path").and_then(Value::as_str) else {
        return Err(crate::Error::NeedDataPath(query.0));
    };
    let mut op = crate::op::Kind::Add
        .builder()
        .path(data_path)
        .value(value.0);
    if let Some(info) = query.get("info").and_then(Value::as_str) {
        op = op.info(info);
    }
    backend.save_op(name, op.build()).await
}

pub async fn replace<B: BackendOperate>(
    name: web::Path<String>,
    value: web::Json<Value>,
    query: web::Query<Value>,
    backend: web::Data<B>,
) -> crate::Result<()> {
    tracing::info!(%name, %value, %query, "replace");
    let Some(data_path) = query.get("data_path").and_then(Value::as_str) else {
        return Err(crate::Error::NeedDataPath(query.0));
    };
    let mut op = crate::op::Kind::Replace
        .builder()
        .path(data_path)
        .value(value.0);
    if let Some(info) = query.get("info").and_then(Value::as_str) {
        op = op.info(info);
    }
    backend.save_op(name, op.build()).await
}

pub async fn restore_document<B: BackendOperate + Send + Sync>(
    name: web::Path<String>,
    backend: web::Data<B>,
) -> crate::Result<web::Json<Value>> {
    backend.document(name).await.map(web::Json)
}

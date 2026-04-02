use actix_web::web;

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

use actix_web::web;

pub fn with_listener(listener: tokio::net::TcpListener) -> crate::Result<actix_web::dev::Server> {
    let start_time = web::Data::new(chrono::Local::now());
    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .wrap(tracing_actix_web::TracingLogger::<
                super::tracing_layer::ServerRootSpanBuilder,
            >::new())
            .app_data(web::Data::clone(&start_time))
            .app_data(web::JsonConfig::default().limit(1 << 32))
            .service(web::scope("api/v1"))
        // .service(routes::health)
    })
    .listen(listener.into_std()?)?
    .run();
    Ok(server)
}

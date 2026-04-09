use super::routes;
use crate::backend::BackendOperate;
use actix_web::web;

pub fn with_listener<B>(
    listener: tokio::net::TcpListener,
    backend: B,
) -> crate::Result<actix_web::dev::Server>
where
    B: BackendOperate + Send + Sync + 'static,
{
    let start_time = web::Data::new(chrono::Local::now());
    let backend = web::Data::new(backend);
    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .wrap(tracing_actix_web::TracingLogger::<
                super::tracing_layer::ServerRootSpanBuilder,
            >::new())
            .app_data(web::Data::clone(&start_time))
            .app_data(web::Data::clone(&backend))
            .app_data(web::JsonConfig::default().limit(1 << 32))
            .service(
                web::scope("api/v1").route("/new/{name}", web::post().to(routes::start_doc::<B>)),
            )
            .service(routes::health)
    })
    .listen(listener.into_std()?)?
    .run();
    Ok(server)
}

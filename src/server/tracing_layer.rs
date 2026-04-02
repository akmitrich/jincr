use actix_web::{body, dev};
use dashmap::DashMap;
use std::{
    sync::{LazyLock, atomic},
    time::Instant,
};
use tracing::span;

pub struct ServerRootSpanBuilder;

static REQ_NUM: atomic::AtomicU64 = atomic::AtomicU64::new(0);
static REQ_MAP: LazyLock<DashMap<span::Id, Instant>> = LazyLock::new(DashMap::new);

impl tracing_actix_web::RootSpanBuilder for ServerRootSpanBuilder {
    #[tracing::instrument(skip_all, name="chat", fields(req_num=(1 + REQ_NUM.fetch_add(1, atomic::Ordering::Relaxed))))]
    fn on_request_start(request: &dev::ServiceRequest) -> tracing::Span {
        let current_span = tracing::Span::current();
        if let Some(id) = current_span.id() {
            REQ_MAP.insert(id, Instant::now());
        }
        tracing::info!(
            method = %request.method(),
            path = %request.path(),
            active_requests = REQ_MAP.len(),
            "start processing"
        );
        current_span
    }

    fn on_request_end<B: body::MessageBody>(
        span: tracing::Span,
        outcome: &Result<dev::ServiceResponse<B>, actix_web::Error>,
    ) {
        let response = if let Ok(outcome) = outcome {
            format!("{:?}", outcome.response())
                .replace("\n", " ")
                .replace("\r", "")
        } else {
            String::new()
        };
        let start = span
            .id()
            .as_ref()
            .and_then(|id| REQ_MAP.remove(id))
            .map(|(_, start)| start)
            .unwrap_or_else(Instant::now);
        tracing::info!(
            %response,
            elapsed = ?start.elapsed(),
            active_requests = REQ_MAP.len(),
            "finished"
        );
    }
}

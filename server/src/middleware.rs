use http::{Request, Response};
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tower::{Layer, Service};
use tracing::{Span, field, info_span, record_all, trace};
use uuid::Uuid;

pub type BoxFuture<'a, T> = Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;

#[derive(Clone)]
pub struct LoggingMiddleware<S> {
    inner: S,
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for LoggingMiddleware<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut request: Request<ReqBody>) -> Self::Future {
        let mut inner = {
            // See: https://docs.rs/tower/latest/tower/trait.Service.html#be-careful-when-cloning-inner-services
            let clone = self.inner.clone();
            std::mem::replace(&mut self.inner, clone)
        };
        Box::pin(async move {
            let request_id = Uuid::now_v7();
            request.extensions_mut().insert(request_id);
            let span = info_span!("request",
                id = ?request_id,
                path = %request.uri().path(),
                client = field::Empty,
                status = field::Empty
            );
            let _enter = span.enter();
            let mut response = inner.call(request).await?;
            record_all!(&span, status = ?response.status());
            response.extensions_mut().insert(request_id);
            Ok(response)
        })
    }
}

#[derive(Clone, Default)]
pub struct LoggingLayer;
impl<S> Layer<S> for LoggingLayer {
    type Service = LoggingMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        LoggingMiddleware { inner }
    }
}

pub fn add_request_log<T: std::fmt::Debug>(request: &tonic::Request<T>) {
    let current_span = Span::current();
    if let Some(client) = request.remote_addr() {
        record_all!(current_span, client = ?client);
    }
    trace!(message = ?request.get_ref());
}

use actix_service::{Service, Transform};
use actix_web::{web, error, dev::ServiceRequest, dev::ServiceResponse, Error};
use futures::future::{ok, Ready};
use std::pin::Pin;
use futures::task::{Context, Poll};
use futures::Future;
use slog::{Fuse, Logger, Drain};
use slog::{o, PushFnValue};
use slog_json::Json;
use slog::FnValue;
use slog_term::{CompactFormat, TermDecorator, PlainDecorator};
use actix_slog::StructuredLogger;
use std::sync::Mutex;
use std::fs::OpenOptions;

pub struct RequestHandler {
    logger: slog::Logger,
}

impl RequestHandler {
    pub fn new(logger: slog::Logger) -> RequestHandler {
        RequestHandler { logger}
    }
}

impl <S, B> Transform<S> for RequestHandler
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RequestHandlerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RequestHandlerMiddleware { service, logger: self.logger.clone() })
    }
}

pub struct RequestHandlerMiddleware<S> {
    service: S,
    logger: slog::Logger,
}

impl<S, B> Service for RequestHandlerMiddleware<S>
    where
        S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let start_time = chrono::Utc::now();
        let logger = self.logger.clone();

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            let req = res.request();
            let end_time = chrono::Utc::now();
            let duration = end_time - start_time;

            info!(logger, "handled request";
                "responseTime" => duration.num_milliseconds(),
                "url" => %req.uri(),
                "route" => req.path(),
                "method" => %req.method(),
                "queryString" => req.query_string(),
                "statusCode" => res.status().as_u16(),
            );

            Ok(res)
        })
    }
}
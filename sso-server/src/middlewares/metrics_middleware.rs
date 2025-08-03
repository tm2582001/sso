use actix_web::{dev::{ServiceRequest, ServiceResponse, Transform, Service}, Error};
use opentelemetry::metrics::Meter;
use futures_util::future::{ok, Ready, LocalBoxFuture};
use std::task::{Context, Poll};
use std::rc::Rc;
use std::time::Instant;


pub struct MetricsMiddleware {
    meter: Meter,
}

impl MetricsMiddleware {
    pub fn new(meter: Meter) -> Self {
        Self { meter }
    }
}

impl<S, B> Transform<S, ServiceRequest> for MetricsMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = MetricsMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(MetricsMiddlewareService {
            service: Rc::new(service),
            meter: self.meter.clone(),
        })
    }
}

pub struct MetricsMiddlewareService<S> {
    service: Rc<S>,
    meter: Meter,
}

impl<S, B> Service<ServiceRequest> for MetricsMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let method = req.method().to_string();
        let path = req.path().to_string();
        let meter = self.meter.clone();
        let start = Instant::now();

        Box::pin(async move {
            let result = service.call(req).await;
            let elapsed = start.elapsed().as_secs_f64();

            let counter = meter
                .f64_counter("http_requests_duration_seconds")
                .with_description("HTTP request duration")
                .build();
            counter.add(elapsed, &[opentelemetry::KeyValue::new("method", method), opentelemetry::KeyValue::new("path", path)]);

            result
        })
    }
}

use tower::{Layer, Service};
use axum::http::{header, Request, HeaderValue, Method, StatusCode, Response};
use futures_util::ready;
use pin_project::pin_project;
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use std::{future::Future};
use axum::handler::Handler;

#[derive(Clone, Copy, Debug)]
pub struct  CorsLayer {
}

impl <S> Layer<S> for CorsLayer{
    type Service = CorsService<S>;
    fn layer(&self, inner: S) -> Self::Service {
        CorsService{
            service:inner
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct  CorsService<S> {
    service:S,
}

impl <S,Req,ResBody> Service<Request<Req>> for CorsService<S>
where
    S:Service<Request<Req>, Response = Response<ResBody>>  + Clone
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = ResponseFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Req>) -> Self::Future {

       let method =  req.method().clone();
        ResponseFuture {
            future: self.service.call(req),
            method,
        }
    }
}

#[pin_project]
#[derive(Debug)]
pub struct ResponseFuture<F> {
    #[pin]
    future: F,
    method: Method,
}

impl<F, ResBody, E> Future for ResponseFuture<F>
    where
        F: Future<Output = Result<Response<ResBody>, E>>,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let mut res = ready!(this.future.poll(cx)?);
        // let mut res = ready!(this.future.as_mut().poll(cx)?);
        if *this.method == Method::OPTIONS {
            *res.status_mut() = StatusCode::OK;
        }
        res.headers_mut().insert(header::ACCESS_CONTROL_ALLOW_METHODS,HeaderValue::from_static("GET,HEAD,OPTIONS,POST,PUT,PATCH,DELETE"));
        res.headers_mut().insert(header::ACCESS_CONTROL_ALLOW_ORIGIN,HeaderValue::from_static("*"));
        res.headers_mut().insert(header::ACCESS_CONTROL_ALLOW_HEADERS,HeaderValue::from_static("*"));
        res.headers_mut().insert(header::ACCESS_CONTROL_MAX_AGE,HeaderValue::from_static("3600"));
        Poll::Ready(Ok(res))
    }
}
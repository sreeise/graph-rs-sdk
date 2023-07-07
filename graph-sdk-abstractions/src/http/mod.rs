use futures::future;
use futures::future::Ready;
use http::header::RETRY_AFTER;
use pin_project::pin_project;
use reqwest::{Request, Response};
use std::{
    fmt,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};
use tokio::time::Sleep;
use tower::retry::{Policy, Retry, RetryLayer};
use tower::{BoxError, ServiceBuilder};
use tower_service::Service;

#[derive(Debug, Clone)]
pub struct RetryAfterPolicy(Duration);

impl RetryAfterPolicy {
    pub fn new(timeout: Duration) -> RetryAfterPolicy {
        RetryAfterPolicy(timeout)
    }

    pub async fn suspend(self) -> Ready<Self> {
        tokio::time::sleep(self.0).await;
        return futures::future::ready(self);
    }
}

impl Default for RetryAfterPolicy {
    fn default() -> Self {
        RetryAfterPolicy::new(Duration::from_secs(0))
    }
}

impl<E> Policy<reqwest::Request, reqwest::Response, E> for RetryAfterPolicy {
    type Future = Ready<Self>;

    fn retry(&self, req: &Request, result: Result<&Response, &E>) -> Option<Self::Future> {
        println!("INSIDE THROTTLE RETRY POLICY");
        dbg!("INSIDE THROTTLE RETRY POLICY");
        if let Ok(response) = result.as_ref() {
            let header = response.headers().get(RETRY_AFTER)?;
            let value_str = header.to_str().ok()?;
            let sec: u64 = value_str.parse().ok()?;
            return Some(tower::retry::future::ResponseFuture {
                request: req,
                retry: (),
                state: State::Retrying,
            });
        }
        None
    }

    fn clone_request(&self, req: &Request) -> Option<Request> {
        req.try_clone()
    }
}

// https://github.com/tower-rs/tower/blob/master/guides/building-a-middleware-from-scratch.md
pub struct HttpClient;

impl HttpClient {
    pub fn new(client: reqwest::Client) -> reqwest::Client {
        let service = ServiceBuilder::new()
            .retry(RetryAfterPolicy::new())
            .service(client)
            .into_inner();
        return service;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use futures::SinkExt;
    use http::StatusCode;
    use reqwest::Url;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_throttle_retry() {
        let mut service = ServiceBuilder::new()
            .layer(RetryLayer::new(RetryAfterPolicy::default()))
            .service(reqwest::Client::new());
        let res: Result<reqwest::Response, reqwest::Error> = service
            .call(Request::new(
                reqwest::Method::GET,
                Url::parse("https://graph.microsoft.com/v1.0/users/id/drive").unwrap(),
            ))
            .await;
        let response = res.unwrap();
        assert_eq!(response.status().as_u16(), 401);
    }
}

/*

pub struct ThrottleRetry<S> {
    inner: S
}

impl<S> ThrottleRetry<S> {
    fn new(inner: S) -> Self {
        ThrottleRetry { inner }
    }
}

impl<S, Request> Service<Request> for ThrottleRetry<S>
    where
        S: Service<Request>,
        S::Error: Into<BoxError>
{
    type Response = S::Response;
    type Error = BoxError;
    type Future = RetryFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, req: Request) -> Self::Future {

        let response_future = self.inner.call(req);
        let sleep = tokio::time::sleep(self.timeout);

        RetryFuture {
            response_future,
            sleep,
        }
    }
}


#[pin_project]
struct RetryFuture<F> {
    #[pin]
    response_future: F,
    #[pin]
    sleep: Sleep,
}

impl<F, Error> Future for RetryFuture<F>
    where
        F: Future<Output = Result<reqwest::Response, Error>>,
        Error: Into<BoxError>,
{
    type Output = Result<reqwest::Response, BoxError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        match this.response_future.poll(cx) {
            Poll::Ready(result) => {
                let result: Result<reqwest::Response, BoxError> = result.map_err(Into::into);

                match result {
                    Ok(response) => {
                        let status = response.status();
                        if status.is_success() {
                            return Poll::Ready(Ok(response));
                        }

                        // 429 Too Many Requests
                        // Wait on Retry-After Header
                        if status.as_u16().eq(&429) {

                        }




                        return Poll::Ready(Ok(response))
                    },
                    Err(err) => return Poll::Ready(Err(err))
                }
            }
            Poll::Pending => {}
        }

        Poll::Pending
    }
}


impl<S, Request> Service<Request> for ThrottleRetry<S>
where
    S: Service<Request>,
    S::Error: Into<BoxError>
{
    type Response = S::Response;
    type Error = BoxError;
    type Future = Response;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let response_future = self.inner.call(request);
    }
}

 */

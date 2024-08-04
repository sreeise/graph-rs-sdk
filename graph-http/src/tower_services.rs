use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;
use std::{sync::Mutex, task::Waker, thread};

use futures_util::future;
use http::StatusCode;
use reqwest::{Request, Response};

#[derive(Clone)]
pub(crate) struct Attempts(pub usize);

impl tower::retry::Policy<Request, Response, Box<(dyn std::error::Error + Send + Sync + 'static)>>
    for Attempts
{
    type Future = future::Ready<Self>;

    fn retry(
        &self,
        _req: &Request,
        result: Result<&Response, &Box<(dyn std::error::Error + Send + Sync + 'static)>>,
    ) -> Option<Self::Future> {
        match result {
            Ok(response) => {
                if response.status().is_server_error() {
                    if self.0 > 0 {
                        return Some(future::ready(Attempts(self.0 - 1)));
                    }
                }
                None
            }
            Err(_) => {
                if self.0 > 0 {
                    Some(future::ready(Attempts(self.0 - 1)))
                } else {
                    None
                }
            }
        }
    }

    fn clone_request(&self, req: &Request) -> Option<Request> {
        req.try_clone()
    }
}

#[derive(Clone)]
pub(crate) struct WaitFor();

impl tower::retry::Policy<Request, Response, Box<(dyn std::error::Error + Send + Sync + 'static)>>
    for WaitFor
{
    type Future = future::Either<future::Ready<Self>, WaitBeforeRetry<Self>>;

    fn retry(
        &self,
        _req: &Request,
        result: Result<&Response, &Box<(dyn std::error::Error + Send + Sync + 'static)>>,
    ) -> Option<Self::Future> {
        match result {
            Ok(response) => match response.status() {
                StatusCode::TOO_MANY_REQUESTS
                | StatusCode::INTERNAL_SERVER_ERROR
                | StatusCode::SERVICE_UNAVAILABLE
                | StatusCode::GATEWAY_TIMEOUT => match response.headers().get("Retry-After") {
                    Some(retry_after) => match retry_after.to_str() {
                        Ok(ra) => match ra.parse::<u64>() {
                            Ok(retry_after) => {
                                let sleep = WaitBeforeRetry::new(
                                    Some(WaitFor()),
                                    Duration::from_secs(retry_after),
                                );
                                Some(future::Either::Right(sleep))
                            }
                            Err(_) => None,
                        },
                        Err(_) => None,
                    },
                    None => None,
                },
                _ => None,
            },
            Err(_) => None,
        }
    }

    fn clone_request(&self, req: &Request) -> Option<Request> {
        req.try_clone()
    }
}

pub struct WaitBeforeRetry<T> {
    inner: Option<T>,
    shared_state: Arc<Mutex<SharedState>>,
}

struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}

impl<T> WaitBeforeRetry<T> {
    pub fn new(inner: Option<T>, duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        // Spawn the new thread
        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            // Signal that the timer has completed and wake up the last
            // task on which the future was polled, if one exists.
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                waker.wake()
            }
        });

        WaitBeforeRetry {
            inner,
            shared_state,
        }
    }
}

impl<T> Unpin for WaitBeforeRetry<T> {}

impl<T> Future for WaitBeforeRetry<T> {
    type Output = T;

    #[inline]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        {
            let mut shared_state = self.shared_state.lock().unwrap();
            if !shared_state.completed {
                shared_state.waker = Some(cx.waker().clone());
                return Poll::Pending;
            }
        }

        Poll::Ready(self.inner.take().expect("Ready polled after completion"))
    }
}

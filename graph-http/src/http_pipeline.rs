use async_trait::async_trait;
use backoff::{future::retry, ExponentialBackoff, ExponentialBackoffBuilder};
use graph_error::{GraphFailure, GraphResult};
use http::StatusCode;
use reqwest::Client;
use std::sync::Arc;
use std::time::Duration;

#[allow(dead_code)]
pub struct RequestContext {
    // todo
}

#[async_trait]
pub trait HttpPipelinePolicy: HttpPipelinePolicyClone {
    // Modify the request.
    async fn process_async(
        &self,
        client: Client,
        request: &mut reqwest::Request,
        pipeline: &[Arc<dyn HttpPipelinePolicy + Send + Sync>],
    ) -> GraphResult<reqwest::Response>;
}

pub trait HttpPipelinePolicyClone {
    fn clone_box(&self) -> Box<dyn HttpPipelinePolicy>;
}

impl<T> HttpPipelinePolicyClone for T
where
    T: 'static + HttpPipelinePolicy + Clone,
{
    fn clone_box(&self) -> Box<dyn HttpPipelinePolicy> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn HttpPipelinePolicy> {
    fn clone(&self) -> Box<dyn HttpPipelinePolicy> {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct ExponentialBackoffRetryPolicy {
    /// The initial retry interval.
    pub initial_interval: Duration,
    /// The randomization factor to use for creating a range around the retry interval.
    ///
    /// A randomization factor of 0.5 results in a random period ranging between 50% below and 50%
    /// above the retry interval.
    pub randomization_factor: f64,
    /// The value to multiply the current interval with for each retry attempt.
    pub multiplier: f64,
    /// The maximum value of the back off period. Once the retry interval reaches this
    /// value it stops increasing.
    pub max_interval: Duration,
    /// The maximum elapsed time after instantiating [`ExponentialBackoff`](struct.ExponentialBackoff.html) or calling
    /// [`reset`](trait.Backoff.html#method.reset) after which [`next_backoff`](../trait.Backoff.html#method.reset) returns `None`.
    pub max_elapsed_time: Option<Duration>,
    pub max_retries: i32,
}

impl ExponentialBackoffRetryPolicy {
    pub(crate) fn get_retries_exponential_backoff(&self) -> ExponentialBackoff {
        ExponentialBackoffBuilder::new()
            .with_initial_interval(self.initial_interval)
            .with_multiplier(self.multiplier)
            .with_randomization_factor(self.randomization_factor)
            .with_max_interval(self.max_interval)
            .with_max_elapsed_time(self.max_elapsed_time)
            .build()
    }
}

impl Default for ExponentialBackoffRetryPolicy {
    fn default() -> ExponentialBackoffRetryPolicy {
        ExponentialBackoffRetryPolicy {
            initial_interval: Duration::from_millis(500),
            randomization_factor: 0.0,
            multiplier: 1.5,
            max_interval: Duration::from_secs(450),
            max_elapsed_time: Some(Duration::from_secs(450)),
            max_retries: 10,
        }
    }
}

#[async_trait]
impl HttpPipelinePolicy for ExponentialBackoffRetryPolicy {
    async fn process_async(
        &self,
        client: Client,
        request: &mut reqwest::Request,
        pipeline: &[Arc<dyn HttpPipelinePolicy + Send + Sync>],
    ) -> GraphResult<reqwest::Response> {
        retry(self.get_retries_exponential_backoff(), || async {
            Ok(pipeline[0]
                .process_async(
                    client.clone(),
                    &mut request.try_clone().unwrap(),
                    &pipeline[1..],
                )
                .await?)
        })
        .await
    }
}

#[derive(Clone)]
pub struct ThrottleRetryPolicy {}

impl ThrottleRetryPolicy {
    pub(crate) fn get_retries_zero_backoff(&self) -> ExponentialBackoff {
        ExponentialBackoffBuilder::new()
            .with_initial_interval(Duration::ZERO)
            .with_multiplier(0.0)
            .with_randomization_factor(0.0)
            .with_max_interval(Duration::ZERO)
            .with_max_elapsed_time(Some(Duration::from_secs(450)))
            .build()
    }
}

#[async_trait]
impl HttpPipelinePolicy for ThrottleRetryPolicy {
    async fn process_async(
        &self,
        client: Client,
        request: &mut reqwest::Request,
        pipeline: &[Arc<dyn HttpPipelinePolicy + Send + Sync>],
    ) -> GraphResult<reqwest::Response> {
        retry(self.get_retries_zero_backoff(), || async {
            match pipeline[0]
                .process_async(
                    client.clone(),
                    &mut request.try_clone().unwrap(),
                    &pipeline[1..],
                )
                .await
            {
                Ok(response) => match response.status() {
                    StatusCode::TOO_MANY_REQUESTS
                    | StatusCode::INTERNAL_SERVER_ERROR
                    | StatusCode::SERVICE_UNAVAILABLE
                    | StatusCode::GATEWAY_TIMEOUT => match response.headers().get("Retry-After") {
                        Some(retry_after) => match retry_after.to_str() {
                            Ok(ra) => match ra.parse::<u64>() {
                                Ok(retry_after) => Err(backoff::Error::Transient {
                                    err: GraphFailure::TemporaryError,
                                    retry_after: Some(Duration::from_secs(retry_after)),
                                }),
                                Err(e) => Err(backoff::Error::Permanent(GraphFailure::from(e))),
                            },
                            Err(e) => Err(backoff::Error::Permanent(GraphFailure::from(e))),
                        },
                        None => Err(backoff::Error::Permanent(GraphFailure::TemporaryError)),
                    },
                    _ => Ok(response),
                },
                Err(e) => Err(backoff::Error::Permanent(e)),
            }
        })
        .await
    }
}

#[derive(Clone)]
pub struct TransportPolicy {}

#[async_trait]
impl HttpPipelinePolicy for TransportPolicy {
    async fn process_async(
        &self,
        client: Client,
        request: &mut reqwest::Request,
        _pipeline: &[Arc<dyn HttpPipelinePolicy + Send + Sync>],
    ) -> GraphResult<reqwest::Response> {
        client
            .execute(request.try_clone().unwrap())
            .await
            .map_err(GraphFailure::from)
    }
}

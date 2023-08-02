use http::Request;
use serde_json::Value;
use std::error::Error;
use std::sync::Arc;

pub struct RequestContext {
    // ... request context fields
}
// request context impl somewhere

// Just here as an example. The actual struct/impl would be different.
pub struct SomePolicyResult;

pub trait HttpPipelinePolicy {
    // Modify the request.
    fn process_async(
        &self,
        context: &RequestContext,
        request: &mut http::Request<Value>,
        pipeline: &[Arc<dyn HttpPipelinePolicy>],
    ) -> Result<SomePolicyResult, Box<dyn std::error::Error>>;

    fn process_next_async(
        &self,
        context: &RequestContext,
        request: &mut http::Request<Value>,
        pipeline: &[Arc<dyn HttpPipelinePolicy>],
    ) -> Result<SomePolicyResult, Box<dyn std::error::Error>> {
        pipeline[0].process_async(context, request, &pipeline[1..])
    }
}

// Example only. Not exact at all.
pub struct ExponentialBackoffRetryPolicy {
    // ... retry fields
    pub min_retries: u32,
}

impl HttpPipelinePolicy for ExponentialBackoffRetryPolicy {
    fn process_async(
        &self,
        _context: &RequestContext,
        _request: &mut Request<Value>,
        _pipeline: &[Arc<dyn HttpPipelinePolicy>],
    ) -> Result<SomePolicyResult, Box<dyn Error>> {
        // modify request...

        Ok(SomePolicyResult)
    }
}

pub struct ThrottleRetryPolicy {
    // ... impl
}

impl HttpPipelinePolicy for ThrottleRetryPolicy {
    fn process_async(
        &self,
        _context: &RequestContext,
        _request: &mut Request<Value>,
        _pipeline: &[Arc<dyn HttpPipelinePolicy>],
    ) -> Result<SomePolicyResult, Box<dyn Error>> {
        // modify request...

        Ok(SomePolicyResult)
    }
}

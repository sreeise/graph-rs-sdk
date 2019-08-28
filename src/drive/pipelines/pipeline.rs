use crate::drive::driveurl::MutateUrl;
use crate::drive::event::DriveEvent;
use crate::drive::item::IntoItem;
use crate::drive::pipelines::datapipeline::DataPipeline;
use crate::drive::pipelines::request::pipeline_request;
use crate::drive::pipelines::request::PipelineRequest;
use crate::drive::pipelines::request::Request;
use crate::drive::pipelines::uploadsessionpipeline::{
    upload_session_pipeline, UploadSessionPipeline,
};
use crate::drive::statusresponse::StatusResponse;
use crate::drive::ItemResult;
use graph_error::GraphFailure;

#[derive(Clone)]
pub struct Pipeline {
    pub pipeline: DataPipeline,
    pub event: DriveEvent,
}

impl Pipeline {
    pub fn new(pipeline: DataPipeline, event: DriveEvent) -> Pipeline {
        Pipeline { pipeline, event }
    }
}

impl AsRef<DriveEvent> for Pipeline {
    fn as_ref(&self) -> &DriveEvent {
        &self.event
    }
}

impl<T> From<Pipeline> for Request<T>
where
    for<'de> T: serde::Deserialize<'de>,
{
    fn from(pipeline: Pipeline) -> Self {
        Request::new(Box::new(pipeline))
    }
}

impl MutateUrl for Pipeline {}

impl<T> IntoItem<T> for Pipeline
where
    for<'de> T: serde::Deserialize<'de>,
{
    fn send(&mut self) -> ItemResult<T> {
        pipeline_request().send(self.pipeline.clone())
    }

    fn drive_event(&mut self) -> DriveEvent {
        self.event
    }
}

impl IntoItem<StatusResponse> for Pipeline {
    fn send(&mut self) -> ItemResult<StatusResponse> {
        let mut response = self.pipeline.request_builder()?.send()?;
        if let Some(err) = GraphFailure::err_from(&mut response) {
            return Err(err);
        }
        Ok(StatusResponse::new(self.event, response))
    }

    fn drive_event(&mut self) -> DriveEvent {
        self.event
    }
}

impl IntoItem<UploadSessionPipeline> for Pipeline {
    fn send(&mut self) -> ItemResult<UploadSessionPipeline> {
        upload_session_pipeline().send(self.pipeline.clone())
    }

    fn drive_event(&mut self) -> DriveEvent {
        self.event
    }
}

mod pipeline_sealed {
    use crate::drive::driveurl::DriveUrl;
    use crate::drive::pipelines::datapipeline::DataPipeline;
    use crate::drive::pipelines::pipeline::Pipeline;

    impl AsMut<DataPipeline> for Pipeline {
        fn as_mut(&mut self) -> &mut DataPipeline {
            &mut self.pipeline
        }
    }

    impl AsRef<DriveUrl> for Pipeline {
        fn as_ref(&self) -> &DriveUrl {
            &self.pipeline.as_ref()
        }
    }

    impl AsMut<DriveUrl> for Pipeline {
        fn as_mut(&mut self) -> &mut DriveUrl {
            self.pipeline.as_mut()
        }
    }
}

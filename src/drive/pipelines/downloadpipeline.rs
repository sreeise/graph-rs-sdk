use crate::drive::driveurl::DriveUrl;
use crate::drive::event::DownloadFormat;
use crate::drive::pipelines::datapipeline::DataPipeline;
use crate::drive::pipelines::request::PipelineRequest;
use crate::drive::ItemResult;
use crate::io::fetch::FetchBuilder;
use graph_error::GraphFailure;
use reqwest::RedirectPolicy;
use std::ffi::OsString;
use std::io::Write;
use std::path::PathBuf;

pub trait MutateDownload {
    fn format(&mut self, format: DownloadFormat);
    fn rename(&mut self, file_name: OsString);

    fn format_glb(&mut self) {
        self.format(DownloadFormat::GLB);
    }

    fn format_html(&mut self) {
        self.format(DownloadFormat::HTML);
    }

    fn format_jpg(&mut self) {
        self.format(DownloadFormat::JPG);
    }

    fn format_pdf(&mut self) {
        self.format(DownloadFormat::PDF);
    }
}

pub trait IntoFetch {
    fn send(&mut self) -> ItemResult<PathBuf>;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromToFile)]
pub struct FetchPipeline {
    token: String,
    pub url: DriveUrl,
    pub download_url: String,
    pub is_download: bool,
    pub directory: PathBuf,
    pub file_name: Option<OsString>,
    pub format: Option<DownloadFormat>,
}

impl FetchPipeline {
    pub fn new(directory: PathBuf, url: DriveUrl, token: &str) -> FetchPipeline {
        FetchPipeline {
            token: token.into(),
            url,
            download_url: String::new(),
            is_download: false,
            directory,
            file_name: None,
            format: None,
        }
    }
}

impl From<DataPipeline> for FetchPipeline {
    fn from(data: DataPipeline) -> Self {
        FetchPipeline::new(PathBuf::new(), data.url, data.token.as_str())
    }
}

pub fn fetch_redirect() -> impl PipelineRequest<PathBuf, FetchPipeline> {
    move |data: FetchPipeline| {
        let client = reqwest::Client::builder()
            .redirect(RedirectPolicy::custom(|attempt| {
                // There should be only 1 redirect to download a drive item.
                if attempt.previous().len() > 1 {
                    return attempt.too_many_redirects();
                }
                attempt.stop()
            }))
            .build()
            .map_err(GraphFailure::from)?;

        let mut res = client
            .get(data.url.as_str())
            .bearer_auth(data.token.as_str())
            .send()?;

        if let Some(err) = GraphFailure::err_from(&mut res) {
            return Err(err);
        }

        let mut fetch_builder =
            FetchBuilder::new(res.url().as_str(), data.directory, data.token.as_str());

        if let Some(file_name) = data.file_name {
            fetch_builder.rename(file_name);
        }

        if let Some(ext) = data.format {
            fetch_builder.format(ext);
        }

        Ok(fetch_builder.fetch()?)
    }
}

pub fn fetch_download() -> impl PipelineRequest<PathBuf, FetchPipeline> {
    move |data: FetchPipeline| {
        let mut fetch_builder = FetchBuilder::new(
            data.download_url.as_str(),
            data.directory,
            data.token.as_str(),
        );

        if let Some(file_name) = data.file_name {
            fetch_builder.rename(file_name);
        }

        if let Some(ext) = data.format {
            fetch_builder.format(ext);
        }

        Ok(fetch_builder.fetch()?)
    }
}

#[derive(Clone, PartialEq)]
pub struct DownloadPipeline {
    pub pipeline: FetchPipeline,
    pub is_direct_download: bool,
}

impl MutateDownload for DownloadPipeline {
    fn format(&mut self, format: DownloadFormat) {
        self.pipeline
            .url
            .append_query_pair("format", format.as_ref());
        self.pipeline.format = Some(format);
    }

    fn rename(&mut self, file_name: OsString) {
        self.pipeline.file_name = Some(file_name);
    }
}

impl IntoFetch for DownloadPipeline {
    fn send(&mut self) -> ItemResult<PathBuf> {
        if self.is_direct_download {
            fetch_download().send(self.pipeline.clone())
        } else {
            fetch_redirect().send(self.pipeline.clone())
        }
    }
}

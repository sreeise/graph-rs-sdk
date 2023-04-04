use crate::internal::ReqwestBlockingResult;
use crate::upload_session::RangeIter;
use graph_error::{GraphFailure, GraphResult};
use reqwest::header::HeaderMap;
use std::io::Read;
use std::thread;

pub struct UploadSessionBlocking {
    url: reqwest::Url,
    range_iter: RangeIter,
    client: reqwest::blocking::Client,
}

impl UploadSessionBlocking {
    pub fn empty(url: reqwest::Url) -> UploadSessionBlocking {
        UploadSessionBlocking {
            url,
            range_iter: Default::default(),
            client: reqwest::blocking::Client::new(),
        }
    }

    pub(crate) fn new(url: reqwest::Url, range_iter: RangeIter) -> UploadSessionBlocking {
        UploadSessionBlocking {
            url,
            range_iter,
            client: reqwest::blocking::Client::new(),
        }
    }

    pub fn url(&self) -> &reqwest::Url {
        &self.url
    }

    fn map_request_builder(
        &self,
        components: Vec<(HeaderMap, reqwest::blocking::Body)>,
    ) -> Vec<reqwest::blocking::RequestBuilder> {
        components
            .into_iter()
            .map(|(header_map, body)| {
                self.client
                    .put(self.url.clone())
                    .headers(header_map)
                    .body(body)
            })
            .collect()
    }

    fn send(&self, header_map: HeaderMap, body: reqwest::blocking::Body) -> ReqwestBlockingResult {
        self.client
            .put(self.url.clone())
            .headers(header_map)
            .body(body)
            .send()
    }

    pub fn status(&self) -> reqwest::blocking::RequestBuilder {
        self.client.get(self.url.clone())
    }

    pub fn cancel(&self) -> reqwest::blocking::RequestBuilder {
        self.client.delete(self.url.clone())
    }

    pub fn from_reader<U: AsRef<str>, R: Read>(
        upload_url: U,
        reader: R,
    ) -> GraphResult<UploadSessionBlocking> {
        Ok(UploadSessionBlocking {
            url: reqwest::Url::parse(upload_url.as_ref())?,
            range_iter: RangeIter::from_reader(reader)?,
            client: reqwest::blocking::Client::new(),
        })
    }

    pub fn channel(&mut self) -> GraphResult<std::sync::mpsc::Receiver<ReqwestBlockingResult>> {
        self.channel_buffer(self.range_iter.len() + 1)
    }

    pub fn channel_buffer(
        &mut self,
        bound: usize,
    ) -> GraphResult<std::sync::mpsc::Receiver<ReqwestBlockingResult>> {
        let components = self
            .range_iter
            .map_all_blocking()
            .ok_or(GraphFailure::invalid(
                "Invalid Headers (internal error, please report)",
            ))?;
        let request_builders = self.map_request_builder(components);
        let (sender, receiver) = std::sync::mpsc::sync_channel(bound);

        thread::spawn(move || {
            for request_builder in request_builders {
                let result = request_builder.send();
                sender.send(result).unwrap();
            }
        });

        Ok(receiver)
    }
}

impl Iterator for UploadSessionBlocking {
    type Item = ReqwestBlockingResult;

    fn next(&mut self) -> Option<Self::Item> {
        let (header_map, body) = self.range_iter.pop_front_blocking()?;
        Some(self.send(header_map, body))
    }
}

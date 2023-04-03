use crate::upload_session::RangeIter;
use graph_error::{GraphFailure, GraphResult, WithGraphError};
use reqwest::header::HeaderMap;
use std::io::Read;
use std::thread;

pub struct UploadSessionBlocking {
    url: reqwest::Url,
    range_iter: RangeIter,
    client: reqwest::blocking::Client,
    status_request_builder: reqwest::blocking::RequestBuilder,
    cancel_request_builder: reqwest::blocking::RequestBuilder,
}

impl UploadSessionBlocking {
    pub fn empty(url: reqwest::Url) -> UploadSessionBlocking {
        let client = reqwest::blocking::Client::new();
        let status_request_builder = client.get(url.clone());
        let cancel_request_builder = client.delete(url.clone());

        UploadSessionBlocking {
            url,
            range_iter: Default::default(),
            client,
            status_request_builder,
            cancel_request_builder,
        }
    }

    pub(crate) fn new(url: reqwest::Url, range_iter: RangeIter) -> UploadSessionBlocking {
        let client = reqwest::blocking::Client::new();
        let status_request_builder = client.get(url.clone());
        let cancel_request_builder = client.delete(url.clone());

        UploadSessionBlocking {
            url,
            range_iter,
            client,
            status_request_builder,
            cancel_request_builder,
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

    fn send(
        &self,
        header_map: HeaderMap,
        body: reqwest::blocking::Body,
    ) -> GraphResult<reqwest::blocking::Response> {
        self.client
            .put(self.url.clone())
            .headers(header_map)
            .body(body)
            .send()
            .map_err(GraphFailure::from)?
            .with_graph_error()
    }

    pub fn status(&self) -> GraphResult<reqwest::blocking::RequestBuilder> {
        Ok(self
            .status_request_builder
            .try_clone()
            .unwrap_or(reqwest::blocking::Client::new().get(self.url.clone())))
    }

    pub fn cancel(&self) -> GraphResult<reqwest::blocking::RequestBuilder> {
        Ok(self
            .cancel_request_builder
            .try_clone()
            .unwrap_or(reqwest::blocking::Client::new().delete(self.url.clone())))
    }

    pub fn from_reader<U: AsRef<str>, R: Read>(
        upload_url: U,
        reader: R,
    ) -> GraphResult<UploadSessionBlocking> {
        let url = reqwest::Url::parse(upload_url.as_ref())?;
        let client = reqwest::blocking::Client::new();
        let status_request_builder = client.get(url.clone());
        let cancel_request_builder = client.delete(url.clone());

        Ok(UploadSessionBlocking {
            url,
            range_iter: RangeIter::from_reader(reader)?,
            client,
            status_request_builder,
            cancel_request_builder,
        })
    }

    pub fn channel(
        &mut self,
    ) -> GraphResult<std::sync::mpsc::Receiver<GraphResult<reqwest::blocking::Response>>> {
        self.channel_buffer(self.range_iter.len() + 1)
    }

    pub fn channel_buffer(
        &mut self,
        bound: usize,
    ) -> GraphResult<std::sync::mpsc::Receiver<GraphResult<reqwest::blocking::Response>>> {
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
                match request_builder.send() {
                    Ok(response) => sender.send(response.with_graph_error()).unwrap(),
                    Err(err) => {
                        sender.send(Err(err).map_err(GraphFailure::from)).unwrap();
                        break;
                    }
                }
            }
        });

        Ok(receiver)
    }
}

impl Iterator for UploadSessionBlocking {
    type Item = GraphResult<reqwest::blocking::Response>;

    fn next(&mut self) -> Option<Self::Item> {
        let (header_map, body) = self.range_iter.pop_front_blocking()?;
        Some(self.send(header_map, body))
    }
}

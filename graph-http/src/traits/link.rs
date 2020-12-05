use crate::types::{Content, Delta};
use crate::url::GraphUrl;
use crate::GraphResponse;
use graph_error::{GraphFailure, GraphResult};
use reqwest::header::CONTENT_TYPE;
use std::convert::TryFrom;
use std::sync::mpsc::{channel, Receiver};
use std::thread;

pub trait DownloadLink<RHS = Self> {
    fn download_link(&self) -> Option<String>;
}

pub trait MetadataLink<RHS = Self> {
    fn metadata_link(&self) -> Option<String>;

    fn metadata(&self) -> Option<GraphResult<GraphResponse<Content>>> {
        let link = self.metadata_link()?;
        let client = reqwest::blocking::Client::new();
        let response = client.get(link.as_str()).send();

        if let Err(e) = response {
            Some(Err(GraphFailure::from(e)))
        } else if let Ok(res) = response {
            Some(GraphResponse::try_from(res))
        } else {
            None
        }
    }
}

pub trait NextLink<RHS = Self> {
    fn next_link(&self) -> Option<String>;
}

pub trait DeltaLink<RHS = Self> {
    fn delta_link(&self) -> Option<String>;

    #[allow(unused_assignments)]
    fn delta<T: 'static + Send + NextLink>(&self, access_token: &str) -> Option<Receiver<Delta<T>>>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let link = self.delta_link()?;
        let token = access_token.to_string();
        let (sender, receiver) = channel();
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(link.as_str())
            .bearer_auth(token.as_str())
            .send()
            .map_err(GraphFailure::from);

        if let Err(err) = response {
            sender.send(Delta::Done(Some(err))).unwrap();
            return Some(receiver);
        }

        let res = response.unwrap();
        if let Some(err) = GraphFailure::from_response(&res) {
            sender.send(Delta::Done(Some(err))).unwrap();
            return Some(receiver);
        }

        let url = GraphUrl::from(res.url());
        let headers = res.headers().clone();
        let status = res.status().as_u16();
        let next: GraphResult<T> = res.json().map_err(GraphFailure::from);
        if let Err(err) = next {
            sender.send(Delta::Done(Some(err))).unwrap();
            return Some(receiver);
        }

        let value: T = next.unwrap();
        let mut next_link = value.next_link();
        sender
            .send(Delta::Next(GraphResponse::new(url, value, status, headers)))
            .unwrap();

        thread::spawn(move || {
            while let Some(next) = next_link {
                let res = client
                    .get(next.as_str())
                    .header(CONTENT_TYPE, "application/json")
                    .bearer_auth(token.as_str())
                    .send()
                    .map_err(GraphFailure::from);

                if let Err(err) = res {
                    next_link = None;
                    sender.send(Delta::Done(Some(err))).unwrap();
                } else {
                    let response = res.unwrap();
                    let url = GraphUrl::from(response.url());
                    let headers = response.headers().clone();
                    let status = response.status().as_u16();
                    if let Some(err) = GraphFailure::from_response(&response) {
                        next_link = None;
                        sender.send(Delta::Done(Some(err))).unwrap();
                    }

                    let value_res: GraphResult<T> = response.json().map_err(GraphFailure::from);
                    if let Err(err) = value_res {
                        next_link = None;
                        sender.send(Delta::Done(Some(err))).unwrap();
                    } else {
                        let value = value_res.unwrap();
                        next_link = value.next_link();
                        sender
                            .send(Delta::Next(GraphResponse::new(url, value, status, headers)))
                            .unwrap();
                    }
                }
            }
            sender.send(Delta::Done(None)).unwrap();
        });

        Some(receiver)
    }
}

impl DownloadLink for serde_json::Value {
    fn download_link(&self) -> Option<String> {
        self["@microsoft.graph.downloadUrl"]
            .as_str()
            .map(|s| s.to_string())
    }
}

impl NextLink for serde_json::Value {
    fn next_link(&self) -> Option<String> {
        self["@odata.nextLink"].as_str().map(|s| s.to_string())
    }
}

impl DeltaLink for serde_json::Value {
    fn delta_link(&self) -> Option<String> {
        self["@odata.deltaLink"].as_str().map(|s| s.to_string())
    }
}

impl MetadataLink for serde_json::Value {
    fn metadata_link(&self) -> Option<String> {
        self["@odata.context"].as_str().map(|s| s.to_string())
    }
}

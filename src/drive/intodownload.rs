use crate::client::*;
use crate::http::{Download, FetchClient};
use crate::url::UrlOrdering;
use graph_error::GraphResult;
use std::marker::PhantomData;
use std::path::Path;

pub struct IntoDownloadClient<'a, I, T> {
    client: &'a Graph,
    item: PhantomData<T>,
    ident: PhantomData<I>,
}

impl<'a, I, T> IntoDownloadClient<'a, I, T> {
    pub fn new(client: &'a Graph) -> IntoDownloadClient<'a, I, T> {
        IntoDownloadClient {
            client,
            item: PhantomData,
            ident: PhantomData,
        }
    }

    pub fn by_id(&self, id: &str) -> GraphResult<FetchClient> {
        self.client
            .request()
            .insert(UrlOrdering::Id(id.into()))
            .format_ord()
            .download()
    }

    pub fn by_path<P: AsRef<Path>>(&'a self, path: P) -> GraphResult<FetchClient> {
        self.client
            .request()
            .remove(UrlOrdering::FileName("".into()))
            .replace(UrlOrdering::RootOrItem("root:".into()))
            .replace(UrlOrdering::Path(path.as_ref().to_path_buf()))
            .format_ord()
            .download()
    }
}

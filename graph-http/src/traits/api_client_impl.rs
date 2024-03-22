use crate::api_impl::ODataQuery;
use graph_error::GraphResult;
use url::Url;

pub trait ApiClientImpl: ODataQuery + Sized {
    fn url(&self) -> Url;

    fn render_path<S: AsRef<str>>(
        &self,
        path: S,
        path_params_map: &serde_json::Value,
    ) -> GraphResult<String>;

    fn build_url<S: AsRef<str>>(
        &self,
        path: S,
        path_params_map: &serde_json::Value,
    ) -> GraphResult<Url> {
        let path = self.render_path(path.as_ref(), path_params_map)?;
        let mut vec: Vec<&str> = path.split('/').collect();
        vec.retain(|s| !s.is_empty());
        let mut url = self.url();
        if let Ok(mut p) = url.path_segments_mut() {
            p.extend(&vec);
        }
        Ok(url)
    }
}

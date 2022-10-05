pub struct NextLink;

#[derive(Deserialize)]
pub struct NextLinkValues<V> {
    pub(crate) value: Vec<V>,
    #[serde(rename = "@odata.nextLink")]
    pub(crate) next_link: Option<String>,
}
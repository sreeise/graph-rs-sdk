use graph_error::GraphResult;

#[derive(Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct WellKnown;

impl WellKnown {
    pub fn signing_keys<T>(url: &str) -> GraphResult<T>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let client = reqwest::blocking::Client::new();
        let response = client.get(url).send()?;
        let keys: T = response.json()?;
        Ok(keys)
    }

    pub async fn async_signing_keys<T>(url: &str) -> GraphResult<T>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let client = reqwest::Client::new();
        let response = client.get(url).send().await?;
        let keys: T = response.json().await?;
        Ok(keys)
    }
}

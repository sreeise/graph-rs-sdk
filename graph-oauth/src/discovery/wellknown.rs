use graph_error::GraphFailure;

#[derive(Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct WellKnown;

impl WellKnown {
    pub fn signing_keys<T>(url: &str) -> Result<T, GraphFailure>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let client = reqwest::blocking::Client::builder().build()?;
        let response = client.get(url).send();

        match response {
            Ok(t) => {
                let keys: T = t.json()?;
                Ok(keys)
            },
            Err(e) => Err(GraphFailure::from(e)),
        }
    }

    pub async fn async_signing_keys<T>(url: &str) -> Result<T, GraphFailure>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let client = reqwest::Client::new();
        let response = client.get(url).send().await;

        match response {
            Ok(t) => {
                let keys: T = t.json().await?;
                Ok(keys)
            },
            Err(e) => Err(GraphFailure::from(e)),
        }
    }
}

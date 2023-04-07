use futures::StreamExt;
use graph_rs_sdk::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

pub async fn channel_delta() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);
    let mut receiver = client
        .users()
        .delta()
        .top("5")
        .paging()
        .channel::<serde_json::Value>()
        .await
        .unwrap();

    while let Some(result) = receiver.recv().await {
        let response = result?;
        println!("{response:#?}");

        let body = response.into_body()?;
        println!("{body:#?}");
    }
    Ok(())
}

static DELTA_TOKEN: &str = "DELTA_TOKEN";

pub async fn channel_delta_token() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);
    let mut receiver = client
        .users()
        .delta()
        .delta_token(DELTA_TOKEN)
        .paging()
        .channel::<serde_json::Value>()
        .await
        .unwrap();

    while let Some(result) = receiver.recv().await {
        let response = result?;
        println!("{response:#?}");

        let body = response.into_body()?;
        println!("{body:#?}");
    }
    Ok(())
}

pub async fn stream_delta() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);
    let mut stream = client
        .users()
        .delta()
        .paging()
        .stream::<serde_json::Value>()
        .unwrap();

    while let Some(result) = stream.next().await {
        match result {
            Ok(response) => {
                println!("{response:#?}");
                let body = response.into_body()?;
                println!("{body:#?}");
            }
            Err(err) => panic!("Error on stream users delta\n{err:#?}"),
        }
    }

    Ok(())
}

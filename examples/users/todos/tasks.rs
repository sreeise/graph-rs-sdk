use futures::StreamExt;
use graph_rs_sdk::error::GraphResult;
use graph_rs_sdk::Graph;
use std::collections::VecDeque;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ToDoListTask {
    #[serde(rename = "displayName")]
    pub display_name: String,
}

impl ToDoListTask {
    pub fn new(s: &str) -> ToDoListTask {
        ToDoListTask {
            display_name: s.to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TodoListTaskCollection {
    value: VecDeque<ToDoListTask>,
}

async fn list_tasks(user_id: &str, list_id: &str) -> GraphResult<()> {
    let client = Graph::new("ACCESS_TOKEN");

    let mut stream = client
        .user(user_id)
        .todo()
        .list(list_id)
        .tasks()
        .list_tasks()
        .paging()
        .stream::<TodoListTaskCollection>()?;

    while let Some(result) = stream.next().await {
        let response = result?;
        println!("{response:#?}");

        let body = response.into_body()?;
        println!("{body:#?}");
    }

    Ok(())
}

async fn create_task(user_id: &str, list_id: &str) -> GraphResult<()> {
    let client = Graph::new("ACCESS_TOKEN");

    let task = &serde_json::json!(
    {
       "title":"A new task",
       "categories": ["Important"],
       "linkedResources":[
          {
             "webUrl":"http://microsoft.com",
             "applicationName":"Microsoft",
             "displayName":"Microsoft task"
          }
       ]
    });

    let response = client
        .user(user_id)
        .todo()
        .list(list_id)
        .tasks()
        .create_tasks(&task)
        .send()
        .await?;

    println!("{response:#?}");

    let body: serde_json::Value = response.json().await?;
    println!("{body:#?}");

    Ok(())
}

async fn create_task_using_me(list_id: &str) -> GraphResult<()> {
    let client = Graph::new("ACCESS_TOKEN");

    let task = &serde_json::json!(
    {
       "title":"A new task",
       "categories": ["Important"],
       "linkedResources":[
          {
             "webUrl":"http://microsoft.com",
             "applicationName":"Microsoft",
             "displayName":"Microsoft task"
          }
       ]
    });

    let response = client
        .me()
        .todo()
        .list(list_id)
        .tasks()
        .create_tasks(&task)
        .send()
        .await?;

    println!("{response:#?}");

    let body: serde_json::Value = response.json().await?;
    println!("{body:#?}");

    Ok(())
}

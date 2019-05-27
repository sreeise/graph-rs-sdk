use crate::drive::event::ConflictBehavior;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewFolder {
    name: String,
    folder: HashMap<String, serde_json::Value>,
    #[serde(rename = "@microsoft.graph.conflictBehavior")]
    microsoft_graph_conflict_behavior: String,
}

impl NewFolder {
    pub fn new(name: &str, conflict_behavior: ConflictBehavior) -> NewFolder {
        NewFolder {
            name: name.into(),
            folder: Default::default(),
            microsoft_graph_conflict_behavior: conflict_behavior.to_string(),
        }
    }

    pub fn name(&mut self, name: &str) {
        self.name = name.into();
    }

    pub fn conflict_behavior(&mut self, conflict_behavior: ConflictBehavior) {
        self.microsoft_graph_conflict_behavior = conflict_behavior.to_string();
    }
}

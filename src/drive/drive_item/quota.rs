use std::io::Write;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct Quota {
    deleted: i64,
    remaining: i64,
    state: String,
    total: i64,
    used: i64,
}

impl Quota {
    pub fn new(deleted: i64, remaining: i64, state: String, total: i64, used: i64) -> Self {
        Quota {
            deleted,
            remaining,
            state,
            total,
            used,
        }
    }
}

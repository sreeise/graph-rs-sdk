#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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

    pub fn deleted(&self) -> i64 {
        self.deleted
    }

    pub fn remaining(&self) -> i64 {
        self.remaining
    }

    pub fn state(&self) -> String {
        self.state.clone()
    }

    pub fn total(&self) -> i64 {
        self.total
    }

    pub fn used(&self) -> i64 {
        self.used
    }
}

pub(crate) trait AsQuery<RHS = Self> {
    fn as_query(&self) -> String;
}

impl<T: ToString + Clone> AsQuery for std::slice::Iter<'_, T> {
    fn as_query(&self) -> String {
        self.clone()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

impl<T: ToString + Clone> AsQuery for std::collections::hash_set::Iter<'_, T> {
    fn as_query(&self) -> String {
        self.clone()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

impl<T: ToString + Clone> AsQuery for std::collections::btree_set::Iter<'_, T> {
    fn as_query(&self) -> String {
        self.clone()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

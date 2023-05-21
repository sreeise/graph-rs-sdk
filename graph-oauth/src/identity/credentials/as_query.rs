pub trait AsQuery<RHS = Self> {
    fn as_query(&self) -> String;
}

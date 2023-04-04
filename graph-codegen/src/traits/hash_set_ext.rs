use std::collections::HashSet;

pub trait HashSetExt<T, U, RHS = Self> {
    fn from_vec(vec: Vec<T>) -> HashSet<U>;
}

impl HashSetExt<String, String> for HashSet<String> {
    fn from_vec(vec: Vec<String>) -> HashSet<String> {
        HashSet::from_iter(vec)
    }
}

impl HashSetExt<&str, String> for HashSet<String> {
    fn from_vec(vec: Vec<&str>) -> HashSet<String> {
        HashSet::from_iter(vec.iter().map(|s| s.to_string()))
    }
}

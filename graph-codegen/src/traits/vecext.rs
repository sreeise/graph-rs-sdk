use crate::builder::ClientLinkSettings;
use std::mem;

pub trait VecExt<T: Default, RHS = Self> {
    fn mem_take(self) -> Vec<T>;
}

impl<T: Default> VecExt<T> for Vec<&mut T> {
    fn mem_take(self) -> Vec<T> {
        self.into_iter().map(|v| mem::take(v)).collect()
    }
}

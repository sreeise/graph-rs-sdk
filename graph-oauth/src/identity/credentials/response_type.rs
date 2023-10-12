use std::collections::BTreeSet;

use crate::identity::AsQuery;

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ResponseType {
    #[default]
    Code,
    Token,
    IdToken,
    StringSet(BTreeSet<String>),
}

impl ToString for ResponseType {
    fn to_string(&self) -> String {
        match self {
            ResponseType::Code => "code".to_owned(),
            ResponseType::Token => "token".to_owned(),
            ResponseType::IdToken => "id_token".to_owned(),
            ResponseType::StringSet(response_type_vec) => response_type_vec.iter().as_query(),
        }
    }
}

impl IntoIterator for ResponseType {
    type Item = ResponseType;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self].into_iter()
    }
}

impl<A: ToString> std::iter::FromIterator<A> for ResponseType {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let vec: BTreeSet<String> = iter.into_iter().map(|v| v.to_string()).collect();
        ResponseType::StringSet(vec)
    }
}

impl AsQuery for Vec<ResponseType> {
    fn as_query(&self) -> String {
        self.iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

impl AsQuery for BTreeSet<ResponseType> {
    fn as_query(&self) -> String {
        self.iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

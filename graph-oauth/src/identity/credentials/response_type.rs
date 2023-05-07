#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub enum ResponseType {
    #[default]
    Code,
    Token,
    IdToken,
    FromString(Vec<String>),
}

impl ToString for ResponseType {
    fn to_string(&self) -> String {
        match self {
            ResponseType::Code => "code".to_owned(),
            ResponseType::Token => "token".to_owned(),
            ResponseType::IdToken => "id_token".to_owned(),
            ResponseType::FromString(response_type_vec) => {
                let response_types: Vec<String> = response_type_vec
                    .iter()
                    .map(|s| s.trim().to_owned())
                    .collect();
                response_types.join(" ")
            }
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

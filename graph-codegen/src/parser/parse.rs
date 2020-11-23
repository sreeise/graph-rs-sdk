use crate::builder::Builder;
use crate::parser::client_resource::ClientResource;

pub trait Parse<ParseFrom> {
    type Error: std::error::Error;

    fn parse(
        parse_from: ParseFrom,
        client_resource: ClientResource<'_>,
    ) -> Result<Builder, Self::Error>;
}

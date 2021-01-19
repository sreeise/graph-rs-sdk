use crate::builder::Builder;
use crate::parser::client_resource::ClientResource;

pub trait Parse<ParseFrom> {
    type Error: std::error::Error;

    fn parse<'a>(
        parse_from: ParseFrom,
        client_resource: ClientResource<'_>,
    ) -> Result<Builder<'a>, Self::Error>;
}

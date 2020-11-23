use crate::builder::Builder;
use crate::parser::client_resource::ClientResource;
use crate::parser::error::ParseError;
use crate::parser::filter::Filter;
use crate::parser::{Parse, Parser, ParserBuilder, ParserSettings, ParserSpec, PathMap};
use from_as::FromFile;
use graph_core::resource::ResourceIdentity;
use reqwest::Url;
use std::cell::RefCell;
use std::convert::TryFrom;
use std::str::FromStr;

static API_V1_METADATA_URL_STR: &str = "https://raw.githubusercontent.com/microsoftgraph/msgraph-metadata/master/openapi/v1.0/openapi.yaml";

lazy_static! {
    static ref API_V1_METADATA_URL: reqwest::Url =
        reqwest::Url::parse(API_V1_METADATA_URL.as_ref()).unwrap();
}

#[derive(Default, Debug)]
pub struct Generator {
    builder: Builder,
}

impl Generator {
    pub fn new(resource_identity: ResourceIdentity) -> Result<Generator, ParseError> {
        let client_resource = ClientResource::try_from(resource_identity)?;
        Ok(Generator {
            builder: Generator::parse(API_V1_METADATA_URL.clone(), client_resource)?,
        })
    }

    pub fn get_client_resource<'a>(
        resource_identity: ResourceIdentity,
    ) -> Option<ClientResource<'a>> {
        ClientResource::try_from(resource_identity).ok()
    }

    pub fn generate(&self) {
        self.builder.build_clients();
    }
}

impl Parse<&std::path::Path> for Generator {
    type Error = ParseError;

    fn parse(
        parse_from: &std::path::Path,
        client_resource: ClientResource<'_>,
    ) -> Result<Builder, Self::Error> {
        match client_resource {
            ClientResource::Main { modifier } => {
                let mut path_map: PathMap = PathMap::from_file(parse_from)?;
                path_map.clean();

                let parser = Parser {
                    spec: RefCell::new(ParserSpec::parser_spec(path_map)),
                };

                parser.use_default_modifiers(&[modifier.as_ref()]);
                parser.use_default_links_override();

                let builder = Builder::new(parser);
                builder.set_build_with_modifier_filter(true);
                builder.use_defaults();

                Ok(builder)
            },
            ClientResource::Secondary {
                start_filter,
                secondary_name,
                modifier,
            } => {
                let path_map: PathMap = PathMap::from_file(parse_from)?;
                let mut path_map: PathMap = path_map.filter(start_filter).into();
                let path_map = path_map.clean_secondary(secondary_name.as_str());

                let parser = Parser {
                    spec: RefCell::new(ParserSpec::parser_spec(path_map)),
                };

                if let Ok(resource_identity) = ResourceIdentity::from_str(secondary_name.as_str()) {
                    let mut spec = parser.spec.borrow_mut();
                    for filter in ParserSettings::path_filters(resource_identity).iter() {
                        spec.paths = spec.paths.filter(filter.clone()).into();
                    }
                }

                parser.use_default_modifiers(&[modifier.as_ref()]);
                parser.use_default_links_override();

                let builder = Builder::new(parser);
                builder.set_build_with_modifier_filter(true);
                builder.use_defaults();

                Ok(builder)
            },
        }
    }
}

impl Parse<reqwest::Url> for Generator {
    type Error = ParseError;

    fn parse(
        parse_from: reqwest::Url,
        client_resource: ClientResource<'_>,
    ) -> Result<Builder, Self::Error> {
        match client_resource {
            ClientResource::Main { modifier } => {
                let parser = Parser::try_from(parse_from)?;
                parser.use_default_modifiers(&[modifier.as_ref()]);
                parser.use_default_links_override();

                let builder = Builder::new(parser);
                builder.set_build_with_modifier_filter(true);
                builder.use_defaults();

                Ok(builder)
            },
            ClientResource::Secondary {
                start_filter,
                secondary_name,
                modifier,
            } => {
                let path_map = PathMap::try_from(parse_from)?;
                let mut path_map: PathMap = path_map.filter(start_filter).into();
                let path_map = path_map.clean_secondary(secondary_name.as_str());
                let parser_spec = ParserSpec::parser_spec(path_map);

                let parser = Parser {
                    spec: RefCell::new(parser_spec),
                };

                if let Ok(resource_identity) = ResourceIdentity::from_str(secondary_name.as_str()) {
                    let mut spec = parser.spec.borrow_mut();
                    for filter in ParserSettings::path_filters(resource_identity).iter() {
                        spec.paths = spec.paths.filter(filter.clone()).into();
                    }
                }

                parser.use_default_modifiers(&[modifier.as_ref()]);
                parser.use_default_links_override();

                let builder = Builder::new(parser);
                builder.set_build_with_modifier_filter(true);
                builder.use_defaults();

                Ok(builder)
            },
        }
    }
}

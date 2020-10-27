use crate::builder::Builder;
use crate::parser::{ApiImpl, Parser, RequestSet, ResourceNames};
use from_as::*;
use inflector::Inflector;
use std::collections::HashMap;
use std::path::Path;

pub struct Generator {
    builder: Builder,
}

impl Generator {
    pub fn parse<P: AsRef<Path>>(path: P, modifiers: Option<&[&str]>) -> Generator {
        let parser = Parser::parse(path);
        let mut modifier_filter_build = false;
        if let Some(modifiers) = modifiers {
            parser.use_default_modifiers(modifiers);
            modifier_filter_build = true;
        }
        parser.use_default_links_override();
        let builder = Builder::new(parser);
        builder.set_build_with_modifier_filter(modifier_filter_build);
        builder.use_default_imports();

        Generator { builder }
    }

    pub fn parse_resource_names<P: AsRef<Path>>(
        path: P,
        resource_names: ResourceNames,
    ) -> Generator {
        let vec1: Vec<String> = resource_names
            .names
            .iter()
            .map(|s| s.to_camel_case())
            .collect();
        let vec2: Vec<&str> = vec1.iter().map(|s| s.as_ref()).collect();

        Generator::parse(path, Some(&vec2))
    }

    pub fn build(&self) {
        self.builder.build();
    }

    pub fn get(&self) -> HashMap<String, RequestSet> {
        self.builder
            .spec
            .borrow_mut()
            .parser
            .build_with_modifier_filter()
    }

    pub fn write_api_impl<P: AsRef<Path>>(&self, path: P) {
        let map: ApiImpl = self.builder.build_with_modifier_filter().into();
        map.as_file_pretty(path).unwrap();
    }
}

impl From<Parser> for Generator {
    fn from(parser: Parser) -> Self {
        let builder = Builder::new(parser);
        builder.use_default_imports();
        Generator { builder }
    }
}

impl From<Builder> for Generator {
    fn from(builder: Builder) -> Self {
        Generator { builder }
    }
}

use crate::builder::Builder;
use crate::parser::{ApiImpl, Parser, RequestSet};
use from_as::*;
use std::collections::HashMap;
use std::path::Path;

pub struct Generator {
    builder: Builder,
}

impl Generator {
    pub fn parse<P: AsRef<Path>>(path: P, modifiers: Option<&[&str]>) -> Generator {
        let parser = Parser::parse(path);
        if let Some(modifiers) = modifiers {
            parser.use_default_modifiers(modifiers);
        }
        let builder = Builder::new(parser);
        builder.use_default_imports();
        builder.use_default_links();

        Generator { builder }
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
        builder.use_default_links();
        Generator { builder }
    }
}

impl From<Builder> for Generator {
    fn from(builder: Builder) -> Self {
        Generator { builder }
    }
}

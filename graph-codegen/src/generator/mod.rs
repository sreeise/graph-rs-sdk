use crate::builder::Builder;
use crate::parser::filter::Filter;
use crate::parser::{ApiImpl, Parser, PathMap, RequestSet, ResourceNames};
use from_as::*;
use inflector::Inflector;
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

#[derive(Default, Debug)]
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
        builder.use_defaults();

        Generator { builder }
    }

    pub fn parse_secondary<P: AsRef<Path>>(
        path: P,
        start_filter: Filter,
        secondary_name: &str,
        modifiers: Option<&[&str]>,
    ) -> Generator {
        let parser = Parser::parse_secondary(path, start_filter, secondary_name);
        let mut modifier_filter_build = false;
        if let Some(modifiers) = modifiers {
            parser.use_default_modifiers(modifiers);
            modifier_filter_build = true;
        }
        parser.use_default_links_override();
        let builder = Builder::new(parser);
        builder.set_build_with_modifier_filter(modifier_filter_build);
        builder.use_defaults();
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
        self.builder.build_with_modifier_filter()
    }

    pub fn get_resource_names(&self) -> ResourceNames {
        self.builder.generate_resource_names()
    }

    pub fn filter(&self, filter: Filter<'_>) -> PathMap {
        self.builder.filter(filter)
    }

    pub fn write_api_impl<P: AsRef<Path>>(&self, path: P) {
        let map: ApiImpl = self.builder.build_with_modifier_filter().into();
        map.as_file_pretty(path).unwrap();
    }

    pub fn write_request_data<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn Error>> {
        let map = self.get();
        let p = path.as_ref().to_path_buf();
        for (name, request_set) in map.iter() {
            if !request_set.is_empty() {
                let new_path = p.join(&format!("{}.yaml", name.to_snake_case()));
                println!("{:#?}", new_path);
                request_set.as_file_pretty(new_path.as_path().as_os_str().to_str().unwrap())?;
            } else {
                println!(
                    "Client with name: {} has 0 requests in the RequestSet (0 RequestMap's)",
                    name
                )
            }
        }
        Ok(())
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

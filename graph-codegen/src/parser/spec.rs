use crate::builder::ClientLinkSettings;
use crate::parser::filter::{
    Filter, ModifierMap, ResourceIdentityModifier, ResourceUrlReplacement, SecondaryModifierMap,
};
use crate::parser::{
    DirectoryModFile, ParserSettings, PathMap, RequestMap, RequestSet, ResourceRequestMap,
};
use crate::traits::{Modify, RequestParser};
use from_as::*;
use graph_core::resource::ResourceIdentity;
use reqwest::Url;
use std::cell::{RefCell, RefMut};
use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};
use std::convert::TryFrom;
use std::fmt::Debug;
use std::path::Path;
use std::str::FromStr;

#[derive(Default, Debug, Clone)]
pub struct Modifier<'a> {
    pub(crate) name: String,
    pub(crate) resource_identity: ResourceIdentity,
    pub(crate) modifier_map: ModifierMap,
    pub(crate) resource_url_modifier: Option<ResourceIdentityModifier>,
    pub(crate) client_links: BTreeMap<String, BTreeSet<ClientLinkSettings>>,
    pub(crate) secondary_modify_target: SecondaryModifierMap,
    pub(crate) custom_methods: Option<HashMap<String, RequestSet>>,
    pub(crate) filters: Vec<Filter<'a>>,
    pub(crate) imports: BTreeSet<String>,
    pub(crate) links_override: HashMap<String, Vec<String>>,
    pub(crate) directory_mod: Option<DirectoryModFile>,
    pub(crate) is_ident_client: bool,
}

impl<'a> Modifier<'a> {
    pub fn new(modifier_name: &str) -> Modifier<'a> {
        let shorthand = &modifier_name[..modifier_name.len() - 1];
        let shorthand_name = format!("{}.{}", modifier_name, shorthand);
        let double_name = format!("{}.{}", modifier_name, modifier_name);
        let resource_identity = ResourceIdentity::from_str(modifier_name).unwrap();

        let mut filters = ParserSettings::path_filters(resource_identity);
        let default_filters = ParserSettings::default_path_filters();
        filters.extend(default_filters);

        let mut modifier = Modifier {
            name: modifier_name.to_string(),
            resource_identity,
            modifier_map: ParserSettings::target_modifiers(resource_identity),
            resource_url_modifier: ParserSettings::resource_identity_modifier(resource_identity),
            client_links: ParserSettings::client_link_settings(resource_identity),
            secondary_modify_target: ParserSettings::secondary_modifier_map(resource_identity),
            custom_methods: ParserSettings::custom_methods(resource_identity),
            filters,
            imports: ParserSettings::imports(resource_identity),
            links_override: ParserSettings::links_override(resource_identity),
            directory_mod: ParserSettings::directory_mod(resource_identity),
            is_ident_client: false,
        };

        modifier.modifier_map.operation_map("", modifier_name);
        modifier
            .modifier_map
            .operation_map(shorthand_name.as_str(), modifier_name);
        modifier
            .modifier_map
            .operation_map(double_name.as_str(), modifier_name);

        if modifier.resource_url_modifier.is_some() ||
            ParserSettings::is_ident_client(resource_identity)
        {
            modifier.is_ident_client = true;
            modifier.imports.insert("handlebars::*".into());
        }

        modifier
    }

    pub fn build_modifier_vec(names: &[&str]) -> Vec<Modifier<'a>> {
        let mut vec: Vec<Modifier> = Vec::new();
        for name in names.iter() {
            vec.push(Modifier::new(name));
        }
        vec
    }
}

#[derive(Debug, Default, Clone)]
pub struct ParserSpec<'a> {
    pub(crate) paths: PathMap,
    pub(crate) modifiers: Vec<Modifier<'a>>,
}

impl<'a> ParserSpec<'a> {
    pub(crate) fn parser_spec(path_map: PathMap, modifiers: Vec<Modifier<'a>>) -> ParserSpec<'a> {
        ParserSpec {
            paths: path_map,
            modifiers,
        }
    }

    pub fn new(mut path_map: PathMap, modifiers: Vec<Modifier<'a>>) -> ParserSpec<'a> {
        path_map.clean();
        ParserSpec::parser_spec(path_map, modifiers)
    }

    pub fn parse<P: AsRef<Path>>(file: P) -> ParserSpec<'a> {
        let mut path_map: PathMap = PathMap::from_file(file.as_ref()).unwrap();
        path_map.clean();
        ParserSpec::parser_spec(path_map, Default::default())
    }

    pub fn parse_secondary<P: AsRef<Path>>(
        file: P,
        start_filter: Filter,
        secondary_name: &str,
    ) -> ParserSpec<'a> {
        let path_map: PathMap = PathMap::from_file(file.as_ref()).unwrap();
        let mut path_map: PathMap = path_map.filter(start_filter).into();
        let path_map = path_map.clean_secondary(secondary_name);
        ParserSpec::parser_spec(path_map, Default::default())
    }

    pub fn set_modifiers(&mut self, modifiers: Vec<Modifier<'a>>) {
        self.modifiers = modifiers;
    }
}

#[derive(Default, Debug, Clone)]
pub struct Parser<'a> {
    pub(crate) spec: RefCell<ParserSpec<'a>>,
}

impl<'a> Parser<'a> {
    pub fn parse<P: AsRef<Path>>(file: P) -> Parser<'a> {
        Parser {
            spec: RefCell::new(ParserSpec::parse(file)),
        }
    }

    pub fn parse_filter<P: AsRef<str>>(file: P, filter: Filter<'_>) -> Parser<'a> {
        let mut path_map: PathMap = PathMap::from_file(file.as_ref()).unwrap();
        path_map.clean();

        Parser {
            spec: RefCell::new(ParserSpec {
                paths: path_map.filter(filter).into(),
                modifiers: Default::default(),
            }),
        }
    }

    pub fn path_map(&self) -> PathMap {
        self.spec.borrow().paths.clone()
    }

    pub fn set_path_map(&self, path_map: PathMap) {
        self.spec.borrow_mut().paths = path_map;
    }

    pub fn modifiers(&self) -> Vec<Modifier<'a>> {
        self.spec.borrow().modifiers.clone()
    }

    pub fn set_modifiers(&self, modifiers: Vec<Modifier<'a>>) {
        self.spec.borrow_mut().set_modifiers(modifiers);
    }

    pub fn filter(&self, filter: Filter<'_>) -> PathMap {
        let spec = self.spec.borrow();
        PathMap {
            paths: spec.paths.filter(filter),
        }
    }

    pub fn multi_filter(&self, filters: Vec<Filter<'_>>) -> PathMap {
        let spec = self.spec.borrow();
        let mut path_map = spec.paths.clone();

        for filter in filters.iter() {
            path_map = path_map.filter(filter.clone()).into();
        }

        path_map
    }

    fn use_filters_internal(&self, mut spec: RefMut<ParserSpec>, filters: Vec<Filter<'_>>) {
        let mut path_map = spec.paths.clone();

        for filter in filters.iter() {
            path_map = PathMap {
                paths: path_map.filter(filter.clone()),
            }
        }

        spec.paths = path_map;
    }

    pub fn use_filters(&self, filters: Vec<Filter<'_>>) {
        let spec = self.spec.borrow_mut();
        self.use_filters_internal(spec, filters);
    }

    pub fn build(&self) -> Vec<ResourceRequestMap<'a>> {
        let spec = self.spec.borrow();
        let modifiers = spec.modifiers.clone();
        let mut resource_requests: Vec<ResourceRequestMap> = Vec::new();

        for modifier in modifiers {
            let mut path_map: PathMap = spec
                .paths
                .filter(Filter::PathStartsWith(&format!("/{}", modifier.name)))
                .into();

            for filter in modifier.filters.iter() {
                path_map = path_map.filter(filter.clone()).into();
            }

            let mut requests: VecDeque<RequestMap> = VecDeque::new();
            for (path, path_spec) in path_map.paths.iter() {
                let mut req_map = RequestMap::default();
                let path = path.transform_path();
                req_map.path = path.clone();
                req_map.requests = path_spec.build_requests(&path, &modifier);

                if let Some(url_modifier) = modifier.resource_url_modifier.as_ref() {
                    if url_modifier.matches(&req_map) {
                        url_modifier.modify(&mut req_map);
                    }
                }

                if let Some(request_map) = requests
                    .iter_mut()
                    .find(|r| r.path.eq(req_map.path.as_str()))
                {
                    request_map.requests.extend(req_map.requests);
                } else {
                    requests.push_back(req_map);
                }
            }

            let mut request_set = RequestSet::default();
            while let Some(req) = requests.pop_front() {
                request_set.join_inner_insert(req);
            }

            if let Some(url_modifier) = modifier.resource_url_modifier.as_ref() {
                url_modifier.modify(&mut request_set);
            }
            resource_requests.push(ResourceRequestMap::new(modifier, request_set));
            requests.clear();
        }
        resource_requests
    }
}

impl<'a> TryFrom<reqwest::Url> for Parser<'a> {
    type Error = reqwest::Error;

    fn try_from(value: Url) -> Result<Self, Self::Error> {
        let response = reqwest::blocking::get(value)?;
        let s = response.text().unwrap();
        let path_map: PathMap = serde_yaml::from_str(s.as_str()).unwrap();
        Ok(Parser {
            spec: RefCell::new(ParserSpec::new(path_map, Default::default())),
        })
    }
}

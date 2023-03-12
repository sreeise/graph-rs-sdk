use crate::parser::error::ParseError;
use crate::parser::{HttpMethod, Request};
use crate::traits::HashMapExt;
use from_as::*;
use inflector::Inflector;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{Read, Write};
use std::str::FromStr;

lazy_static! {
    /// Matches any number. Some of the graph request data has
    /// numbers in the name of the operation id such as
    /// groups.users.get.23a. This becomes an issue when parsing
    /// the resource id. For instance, the method name for an
    /// individual request is taken from the last part of the resource id
    /// and method names really should not be named 23a.
    static ref NUM_REG: Regex = Regex::new(r"[0-9]").unwrap();

    /// matches ids attached to the resource name such as groups({id}).
    static ref PATH_ID_REG: Regex = Regex::new(r"(\(\{)(\w+)(}\))").unwrap();

    /// Matches named ids such as {group-id}.
    pub static ref PATH_ID_NAMED_REG: Regex = Regex::new(r"(\{)(\w+-\w+)(})").unwrap();

    pub static ref INTERNAL_PATH_ID: Regex = Regex::new(r"(\{\{)(\w+)(}})").unwrap();

    pub static ref KEY_VALUE_PAIR: Regex = Regex::new(r"(\w+)(=\{)(\w+)(})").unwrap();

    pub static ref KEY_VALUE_PAIR_RAW_QUOTED: Regex = Regex::new(r#"(\w+)(='\{)(\w+)(}')"#).unwrap();

    pub static ref KEY_VALUE_PAIR_SET: Regex = Regex::new(
        r#"(?P<KEY_VALUE_PAIR>\w+)(=\{)(\w+)(})|(?P<KEY_VALUE_PAIR_QUOTED>\w+)(='\{)(\w+)(}')"#
    ).unwrap();

    pub static ref PATH_REGEX_SET: Regex = Regex::new(
        r#"(?P<PATH_ID>\{)(\w+-\w+)(})|(?P<PATH_ID_NAMED>\{\{)(\w+)(}})|(?P<KEY_VALUE_PAIR>\w+)(=\{)(\w+)(})|(?P<KEY_VALUE_PAIR_QUOTED>\w+)(='\{)(\w+)(}')"#
    ).unwrap();

    pub static ref OPERATION_ID_DUPLICATE_COUNT_METHODS: Regex = Regex::new(
        r#"(?P<GET_COUNT>Get.Count.)(?P<RESOURCE_NAME>\w+)(?P<ARBITRARY_CHARS>-\w+[0-9])"#
    ).unwrap();

    pub static ref RESOURCE_NAME_WITH_ARBITRARY_CHAR_ENDING: Regex = Regex::new(r#"(?P<RESOURCE_NAME>\w+)(-\w+[0-9])"#).unwrap();

    pub static ref API_METHOD_MACRO: Regex = Regex::new(r#"(doc: "\#)"#).unwrap();
}

// (doc: \"#\w*\",) (\w+\(\{)
/*
       doc: "# Update the navigation property calendar in users",
       name: update_calendar,
       response: NoContent,
       path: "/calendar",
       params: 0,
       has_body: true
*/

// Match on first part of macro: (\w+!\(\{)
// ^(?:\w+!\(\{)((\s*)(.*))+(}\);)$
// (\w+!\(\{)(?:\s+\S+)+(:?}\);)*
// (\w+!\(\{)(\w+:+)(\}\);)
// (\w+!(\{)(\w+:[0-9])(});)
// ^(?:\w+!\(\{)*
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromFile, AsFile)]
pub enum PathMatcher {
    PathId,
    PathIdNamed,
    KeyValuePair,
    KeyValuePairQuoted,
}

impl PathMatcher {
    pub fn matches(&self, s: &str) -> bool {
        let capture_names: Vec<&str> = PATH_REGEX_SET.capture_names().flatten().collect();

        for capture in PATH_REGEX_SET.captures_iter(s) {
            let capture_s = capture[0].to_string();

            for name in capture_names.iter() {
                if capture.name(name).is_some() {
                    if let Ok(path_matcher) = PathMatcher::from_str(name) {
                        if !capture_s.contains("RID") {
                            return self.clone() == path_matcher;
                        }
                    }
                }
            }
        }
        false
    }
}

impl FromStr for PathMatcher {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PATH_ID" => Ok(PathMatcher::PathId),
            "PATH_ID_NAMED" => Ok(PathMatcher::PathIdNamed),
            "KEY_VALUE_PAIR" => Ok(PathMatcher::KeyValuePair),
            "KEY_VALUE_PAIR_QUOTED" => Ok(PathMatcher::KeyValuePairQuoted),
            _ => Err(ParseError::Path),
        }
    }
}

pub trait RequestParserBuilder<RHS: ?Sized = Self> {
    fn build(&self, path: String, http_method: HttpMethod) -> Request;
}

pub trait RequestParser<RHS = Self> {
    fn method_name(&self) -> String;
    fn operation_mapping(&self) -> String;
    fn transform_key_value_pair_query(&self) -> (String, HashSet<String>);
    fn transform_path(&self) -> String;
    fn shift_path_ids(&self) -> String;
    fn links(&self) -> HashSet<String> {
        Default::default()
    }
    fn struct_links(&self) -> HashMap<String, Vec<String>>;
}

pub trait Modify<T> {
    fn modify(&self, value: &mut T);
}

impl RequestParser for &str {
    /// Parse the method name of a request.
    fn method_name(&self) -> String {
        let mut method_name = String::new();

        let directory_object_items =
            "Get.microsoft.graph.directoryObject.Items.As.microsoft.graph.";
        let directory_object_item = "Get.microsoft.graph.directoryObject.Item.As.microsoft.graph.";
        let current_operation_id = self.to_lowercase();

        let starts_with_directory_object_items =
            current_operation_id.starts_with(directory_object_items.to_lowercase().as_str());
        let starts_with_directory_object_item =
            current_operation_id.starts_with(directory_object_item.to_lowercase().as_str());

        if starts_with_directory_object_items || starts_with_directory_object_item {
            let operation_id_clone = self
                .replace(directory_object_items, "")
                .replace(directory_object_item, "");

            for capture in
                RESOURCE_NAME_WITH_ARBITRARY_CHAR_ENDING.captures_iter(operation_id_clone.as_str())
            {
                if let Some(capture_match) = capture.name("RESOURCE_NAME") {
                    let resource_name = capture_match.as_str();
                    if !resource_name.is_empty() {
                        if starts_with_directory_object_item {
                            return format!(
                                "get_directory_object_item_as_{}_type",
                                resource_name.to_snake_case()
                            );
                        }
                    }
                }
            }
        }

        if self.starts_with("Get.Count") {
            let operation_id_clone = {
                if self.starts_with("Get.Count.microsoft.graph.") {
                    self.replace("microsoft.graph.", "")
                } else {
                    self.to_string()
                }
            };

            for capture in
                OPERATION_ID_DUPLICATE_COUNT_METHODS.captures_iter(operation_id_clone.as_str())
            {
                if let Some(capture_match) = capture.name("RESOURCE_NAME") {
                    let resource_name = capture_match.as_str();
                    if !resource_name.is_empty() {
                        return format!("get_{}_count", resource_name.to_snake_case());
                    }
                }
            }
        }

        if let Some(index) = self.rfind('.') {
            let last: &str = self[index + 1..].as_ref();
            if NUM_REG.is_match(last) && !last.contains("OAuth2") {
                if let Some(idx) = self[..index].rfind('.') {
                    method_name.push_str(self[idx + 1..index].as_ref());
                }
            } else {
                method_name.push_str(self[index + 1..].as_ref());
            }
        } else {
            method_name.push_str(self);
        }

        if method_name.contains("OAuth2") {
            return method_name.to_snake_case().replace("o_auth_2", "oauth2");
        }

        if method_name.is_empty() {
            self.to_snake_case()
        } else {
            method_name.to_snake_case()
        }
    }

    /// Create the operation mapping that will be used to create
    /// client structs, links between these structs, and the mapping
    /// for where to place request methods.
    fn operation_mapping(&self) -> String {
        let mut op_mapping = String::new();

        if self.contains('.') {
            let mut ops: Vec<&str> = self.split('.').collect();
            ops.retain(|s| !s.is_empty());

            if let Some(last) = ops.pop() {
                if NUM_REG.is_match(last) && ops.len() > 1 {
                    ops.pop();
                }

                if ops.len() > 1 {
                    op_mapping = ops.join(".");
                } else {
                    op_mapping = ops.join("");
                }
            }
        } else {
            op_mapping = self.to_string();
        }

        if op_mapping.ends_with('.') {
            op_mapping.truncate(op_mapping.len() - 1);
        }

        if op_mapping.matches('.').count() == 1 {
            let mut queue: VecDeque<&str> = op_mapping.split('.').collect();
            let first = queue.pop_front().unwrap();
            let last = queue.pop_front().unwrap();
            if first[..first.len() - 1].eq(last) {
                op_mapping = first.to_string();
            } else if last[..last.len() - 1].eq(first) {
                op_mapping = last.to_string();
            }
        }

        op_mapping
    }

    #[allow(unused_assignments)]
    fn transform_key_value_pair_query(&self) -> (String, HashSet<String>) {
        let mut path = self.to_string();
        let path_clone = path.clone();

        let replace_ids = |count: usize, s: &str, path: &mut String| {
            if count == 1 {
                path.replacen(s, "{{id}}", 1)
            } else {
                path.replacen(s, &format!("{{{{id{}}}}}", count), 1)
            }
        };

        let mut count = 1;
        let capture_names: Vec<&str> = PATH_REGEX_SET.capture_names().flatten().collect();
        let mut path_parameters = HashSet::new();

        for capture in PATH_REGEX_SET.captures_iter(path_clone.as_str()) {
            let s = capture[0].to_string();
            let mut found_match = false;

            for name in capture_names.iter() {
                if capture.name(name).is_some() {
                    if let Ok(path_matcher) = PathMatcher::from_str(name) {
                        if !s.contains("RID") {
                            match path_matcher {
                                PathMatcher::KeyValuePair => {
                                    if let Some(_i) = s.find('=') {
                                        if let Some(i) = s.find('=') {
                                            path = replace_ids(count, &s[i + 1..], &mut path);
                                            path_parameters
                                                .insert(s[i + 3..s.len() - 2].to_string());
                                            found_match = true;
                                            break;
                                        }
                                    }
                                }
                                PathMatcher::KeyValuePairQuoted => {
                                    if let Some(_i) = s.find('=') {
                                        if let Some(i) = s.find('=') {
                                            if count == 1 {
                                                path = path.replacen(&s[i + 1..], "'{{id}}'", 1);
                                                path_parameters
                                                    .insert(s[i + 3..s.len() - 2].to_string());
                                                found_match = true;
                                                break;
                                            } else {
                                                path = path.replacen(
                                                    &s[i + 1..],
                                                    &format!("'{{{{id{}}}}}'", count),
                                                    1,
                                                );
                                                path_parameters
                                                    .insert(s[i + 3..s.len() - 2].to_string());
                                                found_match = true;
                                                break;
                                            }
                                        }
                                    }
                                }
                                // If this matches return the original string.
                                _ => {
                                    return (self.to_string(), HashSet::new());
                                }
                            }
                        }
                    }
                }

                if found_match {
                    break;
                }
            }

            if found_match {
                found_match = false;
                count += 1;
            }
        }

        (path, path_parameters)
    }

    #[allow(unused_assignments)]
    fn transform_path(&self) -> String {
        let mut path = self.to_string();
        let path_clone = path.clone();

        let replace_ids = |count: usize, s: &str, path: &mut String| {
            if count == 1 {
                path.replacen(s, "{{id}}", 1)
            } else {
                path.replacen(s, &format!("{{{{id{}}}}}", count), 1)
            }
        };

        let mut count = 1;
        let capture_names: Vec<&str> = PATH_REGEX_SET.capture_names().flatten().collect();

        for capture in PATH_REGEX_SET.captures_iter(path_clone.as_str()) {
            let s = capture[0].to_string();
            let mut found_match = false;

            for name in capture_names.iter() {
                if capture.name(name).is_some() {
                    if let Ok(path_matcher) = PathMatcher::from_str(name) {
                        if !s.contains("RID") {
                            match path_matcher {
                                PathMatcher::PathId => {
                                    if count == 1 {
                                        path = path.replacen(s.as_str(), "{{id}}", 1);
                                        found_match = true;
                                        break;
                                    } else {
                                        path = path.replacen(
                                            s.as_str(),
                                            &format!("{{{{id{}}}}}", count),
                                            1,
                                        );
                                        found_match = true;
                                        break;
                                    }
                                }
                                PathMatcher::PathIdNamed => {
                                    path = replace_ids(count, s.as_str(), &mut path);
                                    found_match = true;
                                    break;
                                }
                                PathMatcher::KeyValuePair => {
                                    if let Some(_i) = s.find('=') {
                                        if let Some(i) = s.find('=') {
                                            path = replace_ids(count, &s[i + 1..], &mut path);
                                            found_match = true;
                                            break;
                                        }
                                    }
                                }
                                PathMatcher::KeyValuePairQuoted => {
                                    if let Some(_i) = s.find('=') {
                                        if let Some(i) = s.find('=') {
                                            if count == 1 {
                                                path = path.replacen(&s[i + 1..], "'{{id}}'", 1);
                                                found_match = true;
                                                break;
                                            } else {
                                                path = path.replacen(
                                                    &s[i + 1..],
                                                    &format!("'{{{{id{}}}}}'", count),
                                                    1,
                                                );
                                                found_match = true;
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                if found_match {
                    break;
                }
            }

            if found_match {
                found_match = false;
                count += 1;
            }
        }

        path
    }

    fn shift_path_ids(&self) -> String {
        self.replacen("id2", "id", 1)
            .replacen("id3", "id2", 1)
            .replacen("id4", "id3", 1)
            .replacen("id5", "id4", 1)
            .replacen("id6", "id5", 1)
    }

    fn links(&self) -> HashSet<String> {
        let mut links: HashSet<String> = HashSet::new();

        if self.contains('.') {
            let mut vec: Vec<&str> = self.split('.').collect();
            vec.retain(|s| !s.is_empty());

            let mut iter = vec.iter().peekable();

            while let Some(current) = iter.next() {
                if let Some(next) = iter.peek() {
                    links.insert(format!("{}.{}", current, next));
                }
            }
        } else {
            links.insert(self.to_string());
        }

        links
    }

    /// Creates a hash map of each struct and the client structs
    /// it links too.
    ///
    /// # Example
    ///
    /// Say we have the following operation id's or operation mappings:
    ///     groups.calendar.calendarView
    ///     groups.calendarView
    ///     groups.drive
    ///
    /// {
    ///     "groups": [
    ///         "calendar",
    ///         "calendarView",
    ///         "drive"
    ///     ],
    ///     "calendar": [
    ///         "calendarView"
    ///     ]
    /// }
    fn struct_links(&self) -> HashMap<String, Vec<String>> {
        let links = self.links();
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        let mut vec: Vec<&str> = links.iter().map(|s| s.as_str()).collect();
        vec.sort_unstable();

        for link in vec.iter() {
            if link.contains('.') {
                let mut vec: VecDeque<&str> = link.split('.').collect();
                vec.retain(|l| !l.is_empty());
                let first = vec.pop_front().unwrap();
                let last = vec.pop_front().unwrap();
                map.entry_modify_insert(first.to_pascal_case(), last.to_pascal_case());
            } else {
                map.insert(link.to_pascal_case(), vec![]);
            }
        }

        map
    }
}

impl RequestParser for String {
    fn method_name(&self) -> String {
        self.as_str().method_name()
    }

    fn operation_mapping(&self) -> String {
        self.as_str().operation_mapping()
    }

    fn transform_key_value_pair_query(&self) -> (String, HashSet<String>) {
        self.as_str().transform_key_value_pair_query()
    }

    fn transform_path(&self) -> String {
        self.as_str().transform_path()
    }

    fn shift_path_ids(&self) -> String {
        self.as_str().shift_path_ids()
    }

    fn links(&self) -> HashSet<String> {
        self.as_str().links()
    }

    fn struct_links(&self) -> HashMap<String, Vec<String>> {
        self.as_str().struct_links()
    }
}

use crate::openapi::PathItem;
use crate::parser::filter::{Filter, FilterIgnore};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use regex::Regex;
use std::collections::BTreeMap;

pub trait FilterPath {
    fn filter_path(&self, filter: Filter) -> BTreeMap<String, PathItem> {
        let iter = self.paths().into_par_iter();
        match filter {
            Filter::PathStartsWith(filter) => iter
                .filter(|(path, _path_item)| path.starts_with(filter.as_str()))
                .collect(),
            Filter::PathStartsWithMulti(vec) => iter
                .filter(|(path, _path_item)| vec.iter().any(|s| path.starts_with(s)))
                .collect(),
            Filter::None => self.paths(),
            Filter::PathEquals(filter) => iter
                .filter(|(path, _path_item)| path.eq(filter.as_str()))
                .collect(),
            Filter::PathContains(filter) => iter
                .filter(|(path, _path_item)| path.contains(filter.as_str()))
                .collect(),
            Filter::Regex(s) => {
                let regex = Regex::new(s.as_str()).unwrap();
                iter.filter(|(path, _path_spec)| regex.is_match(path.as_ref()))
                    .collect()
            }
            Filter::IgnoreIf(filter_ignore) => match filter_ignore {
                FilterIgnore::PathContains(s) => iter
                    .filter(|(path, _path_item)| !path.contains(s.as_str()))
                    .collect(),
                FilterIgnore::PathStartsWith(s) => iter
                    .filter(|(path, _path_item)| !path.starts_with(s.as_str()))
                    .collect(),
                FilterIgnore::PathContainsMulti(vec) => iter
                    .filter(|(path, _path_item)| !vec.iter().any(|s| path.contains(s)))
                    .collect(),
                FilterIgnore::PathEquals(s) => iter
                    .filter(|(path, _path_item)| path.ne(s.as_str()))
                    .collect(),
            },
            Filter::MultiFilter(_vec) => {
                // TODO: Fix MultiFilter
                Default::default()
            }
        }
    }

    fn paths(&self) -> BTreeMap<String, PathItem>;
}

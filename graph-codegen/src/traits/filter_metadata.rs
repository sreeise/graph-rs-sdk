use crate::filter::Filter;
use crate::filter::FilterIgnore;
use crate::macros::MacroQueueWriter;
use regex::Regex;
use std::collections::VecDeque;
use std::fmt::Debug;

pub trait FilterMetadata {
    type Metadata: Debug + Clone + MacroQueueWriter;

    fn metadata_iter(&self) -> std::collections::vec_deque::IntoIter<Self::Metadata>;

    fn filter_path(&self, filter: Filter) -> VecDeque<Self::Metadata>
    where
        Self: Sized,
    {
        let iter = self.metadata_iter();
        match filter {
            Filter::PathStartsWith(filter) => iter
                .filter(|metadata| metadata.path().starts_with(filter.as_str()))
                .collect(),
            Filter::PathStartsWithMulti(vec) => iter
                .filter(|metadata| vec.iter().any(|s| metadata.path().starts_with(s)))
                .collect(),
            Filter::None => iter.collect(),
            Filter::PathEquals(filter) => iter
                .filter(|metadata| metadata.path().eq(filter.as_str()))
                .collect(),
            Filter::PathContains(filter) => iter
                .filter(|metadata| metadata.path().contains(filter.as_str()))
                .collect(),
            Filter::Regex(s) => {
                let regex = Regex::new(s.as_str()).unwrap();
                iter.filter(|metadata| regex.is_match(metadata.path().as_ref()))
                    .collect()
            }
            Filter::IgnoreIf(filter_ignore) => match filter_ignore {
                FilterIgnore::PathContains(s) => iter
                    .filter(|metadata| !metadata.path().contains(s.as_str()))
                    .collect(),
                FilterIgnore::PathStartsWith(s) => iter
                    .filter(|metadata| !metadata.path().starts_with(s.as_str()))
                    .collect(),
                FilterIgnore::PathContainsMulti(vec) => iter
                    .filter(|metadata| !vec.iter().any(|s| metadata.path().contains(s)))
                    .collect(),
                FilterIgnore::PathEquals(s) => iter
                    .filter(|metadata| metadata.path().eq(s.as_str()))
                    .collect(),
            },
            Filter::MultiFilter(vec) => {
                let mut vec_dequeue: VecDeque<Self::Metadata> = VecDeque::new();
                for filter in vec {
                    let temp_queue = self.filter_path(filter);
                    vec_dequeue.extend(temp_queue);
                }
                vec_dequeue
            }
        }
    }
}

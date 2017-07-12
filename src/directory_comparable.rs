use std::collections::BTreeMap;
use std::path::PathBuf;

use file_comparable::*;

// TODO : We really want this to be
// trait FileIterator : Iterator<Item=PathBuf> + fmt::Display {}
// but the refactoring seems way hard when we make this not a concrete type...
type FileIterator = Iterator<Item = PathBuf>;

pub enum CacheExistenceQuery {
    CacheForBidirectional,
    DoNotCache,
}

pub enum WhichSet {
    Left,
    Right,
}

pub trait DirectoryComparable {
    // boxing sucks for performance, but there will be maybe 2 FileIterator's total
    fn mark_as_seen(&mut self, dir: &mut Box<FileIterator>);

    fn mark_file_as_seen(&mut self, file: &PathBuf, set: WhichSet);

    fn exists_in_directory(
        &mut self,
        file: &PathBuf,
        set: WhichSet,
        should_cache: CacheExistenceQuery,
    ) -> Option<PathBuf>;

    fn build_map(
        &mut self,
        subset_dir: &mut Box<FileIterator>,
        superset_dir: &mut Box<FileIterator>,
    ) -> BTreeMap<PathBuf, Option<PathBuf>> {

        // Populate the files we want to check against
        self.mark_as_seen(superset_dir);

        // Prepare our map
        let mut result = BTreeMap::new();

        for path in subset_dir.by_ref() {
            let corresponding_path =
                self.exists_in_directory(&path, WhichSet::Right, CacheExistenceQuery::DoNotCache);
            result.insert(path, corresponding_path);
        }

        result
    }

    fn report_missing(
        &mut self,
        subset_dir: &mut Box<FileIterator>,
        superset_dir: &mut Box<FileIterator>,
    ) -> Vec<PathBuf> {

        // Populate the files we want to check against
        self.mark_as_seen(superset_dir);

        // Prepare our list of unseen
        let mut result = Vec::new();

        for path in subset_dir.by_ref() {
            match self.exists_in_directory(
                &path,
                WhichSet::Right,
                CacheExistenceQuery::DoNotCache,
            ) {
                Some(_) => {}
                None => result.push(path),
            }
        }

        result
    }

    fn report_missing_bidirectional(
        &mut self,
        left_dir: &mut Box<FileIterator>,
        right_dir: &mut Box<FileIterator>,
    ) -> (Vec<PathBuf>, Vec<PathBuf>) {

        // Make a copy of the superset iterator
        let mut right_dir_copy = Vec::new();
        for item in right_dir.by_ref() {
            right_dir_copy.push(item);
        }

        // Populate the files we want to check against
        let right_dir_cloned_iter = right_dir_copy.into_iter();
        // clone before boxing so that we can use it one more time?
        let mut iter: Box<FileIterator> = Box::new(right_dir_cloned_iter.clone());
        self.mark_as_seen(&mut iter);

        // Prepare our list of unseen
        let mut left_missing_result = Vec::new();

        for path in left_dir.by_ref() {
            match self.exists_in_directory(
                &path,
                WhichSet::Right,
                CacheExistenceQuery::CacheForBidirectional,
            ) {
                Some(_) => {}
                None => left_missing_result.push(path),
            }
        }

        // Now iterate through the right_dir

        let mut right_missing_result = Vec::new();
        for path in right_dir_cloned_iter {
            match self.exists_in_directory(&path, WhichSet::Left, CacheExistenceQuery::DoNotCache) {
                Some(_) => {}
                None => right_missing_result.push(path),
            }
        }

        (left_missing_result, right_missing_result)
    }
}

/// Trivial mock
pub struct TrivialDirectoryComparable;

impl DirectoryComparable for TrivialDirectoryComparable {
    #[allow(unused_variables)]
    fn mark_as_seen(&mut self, dir: &mut Box<FileIterator>) {}

    #[allow(unused_variables)]
    fn mark_file_as_seen(&mut self, file: &PathBuf, set: WhichSet) {}

    #[allow(unused_variables)]
    fn exists_in_directory(
        &mut self,
        file: &PathBuf,
        set: WhichSet,
        should_cache: CacheExistenceQuery,
    ) -> Option<PathBuf> {
        Some(file.clone())
    }
}

/// A directory comparator from a file comparator
/// (This layer will hide the type information that the file comparators spit out
///  as the Ord keys since it is internal to calculating the answer to presence / mapping)
pub struct DirectoryComparableWithFileComparable<C: FileComparable> {
    comp: C,
    left_map: BTreeMap<C::Key, PathBuf>,
    right_map: BTreeMap<C::Key, PathBuf>,
}

impl<C: FileComparable> DirectoryComparableWithFileComparable<C> {
    pub fn new(comparator: C) -> Self {
        DirectoryComparableWithFileComparable {
            comp: comparator,
            left_map: BTreeMap::new(), // I guess we could only initialize this if needed lazily?
            right_map: BTreeMap::new(),
        }
    }
}

impl<C: FileComparable> DirectoryComparable for DirectoryComparableWithFileComparable<C> {
    fn mark_as_seen(&mut self, dir: &mut Box<FileIterator>) {
        for path in dir.by_ref() {
            match self.comp.get_key(&path) {
                Some(hashed) => {
                    self.right_map.insert(hashed, path);
                }
                None => {}
            };
        }
    }

    fn mark_file_as_seen(&mut self, file: &PathBuf, set: WhichSet) {
        match self.comp.get_key(&file) {
            Some(hashed) => {
                let mut which_map = match set {
                    WhichSet::Left => &mut self.left_map,
                    WhichSet::Right => &mut self.right_map,
                };
                which_map.insert(hashed, file.clone());
            }
            None => {}
        };
    }

    fn exists_in_directory(
        &mut self,
        file: &PathBuf,
        set: WhichSet,
        should_cache: CacheExistenceQuery,
    ) -> Option<PathBuf> {

        match self.comp.get_key(&file) {
            Some(hashed) => {
                match should_cache {
                    CacheExistenceQuery::CacheForBidirectional => {
                        let other_set = match &set {
                            &WhichSet::Left => WhichSet::Right,
                            &WhichSet::Right => WhichSet::Left,
                        };

                        self.mark_file_as_seen(&file, other_set)
                    }

                    CacheExistenceQuery::DoNotCache => {}
                };

                let which_map = match set {
                    WhichSet::Left => &mut self.left_map,
                    WhichSet::Right => &mut self.right_map,
                };

                match which_map.get(&hashed) {
                    Some(file) => Some(file.clone()),
                    None => None,
                }
            }
            None => None,
        }
    }
}

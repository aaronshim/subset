use std::collections::BTreeMap;
use std::fs::DirEntry;
use std::path::PathBuf;

use file_comparable::*;

// TODO : We really want this to be
// trait FileIterator : Iterator<Item=DirEntry> + fmt::Display {}
// but the refactoring seems way hard when we make this not a concrete type...
type FileIterator = Iterator<Item=DirEntry>;

pub trait DirectoryComparable {
    // boxing sucks for performance, but there will be maybe 2 FileIterator's over the course of execution
    fn mark_as_seen(&mut self, dir: &mut Box<FileIterator>);

    fn exists_in_directory(&mut self, file: &PathBuf) -> Option<PathBuf>;

    fn build_map(&mut self, subset_dir: &mut Box<FileIterator>, superset_dir: &mut Box<FileIterator>)
        -> BTreeMap<PathBuf, Option<PathBuf>> {
        
        // Populate the files we want to check against
        self.mark_as_seen(superset_dir);

        // Prepare our map
        let mut result = BTreeMap::new();

        for path in subset_dir.by_ref() {
            let path = path.path();
            let corresponding_path = self.exists_in_directory(&path);
            result.insert(path, corresponding_path);
        }

        result
    }

    fn report_missing(&mut self, subset_dir: &mut Box<FileIterator>, superset_dir: &mut Box<FileIterator>)
        -> Vec<PathBuf> {

        // Populate the files we want to check against
        self.mark_as_seen(superset_dir);

        // Prepare our list of unseen
        let mut result = Vec::new();

        for path in subset_dir.by_ref() {
            let path = path.path();
            match self.exists_in_directory(&path) {
                Some(_) => {},
                None => { result.push(path) }
            }
        }

        result
    }
}

/// Trivial mock
pub struct TrivialDirectoryComparable;

impl DirectoryComparable for TrivialDirectoryComparable {
    #[allow(unused_variables)]
    fn mark_as_seen(&mut self, dir: &mut Box<FileIterator>) {}

    #[allow(unused_variables)]
    fn exists_in_directory(&mut self, file: &PathBuf) -> Option<PathBuf> {
        Some(file.clone())
    }
}

/// A directory comparator from a file comparator
/// (This layer will hide the type information that the file comparators spit out
///  as the Ord keys since it is internal to calculating the answer to presence / mapping)
pub struct DirectoryComparableWithFileComparable<C : FileComparable> {
    comp : C,
    superset_map : BTreeMap<C::Key, PathBuf>
}

impl<C : FileComparable> DirectoryComparableWithFileComparable<C> {
    pub fn new(comparator: C) -> Self {
        DirectoryComparableWithFileComparable {
            comp : comparator,
            superset_map : BTreeMap::new()
        }
    }
}

impl<C : FileComparable> DirectoryComparable for DirectoryComparableWithFileComparable<C> {
    fn mark_as_seen(&mut self, dir: &mut Box<FileIterator>) {
        for path in dir.by_ref() {
            let path = path.path();
            match self.comp.get_key(&path) {
                Some(hashed) => { self.superset_map.insert(hashed, path); },
                None => {}
            };
        }
    }

    fn exists_in_directory(&mut self, file: &PathBuf) -> Option<PathBuf> {
        match self.comp.get_key(&file) {
            Some(hashed) => {
                match self.superset_map.get(&hashed) {
                    Some(file) => Some(file.clone()),
                    None => None
                }
            },
            None => None
        }
    }
}


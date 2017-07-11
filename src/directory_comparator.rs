use std::collections::BTreeMap;
use std::fs::DirEntry;
use std::path::PathBuf;

use file_comparable::*;

type FileIterator = Iterator<Item=DirEntry>;

pub trait DirectoryComparator {
    fn mark_as_seen(&mut self, dir: &mut Box<FileIterator>);
    fn exists_in_directory(&self, file: &PathBuf) -> Option<PathBuf>;
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
}

/// Try a trivial mock of the DirectoryComparator
pub struct TrivialDirectoryComparator;

impl TrivialDirectoryComparator {
    pub fn new() -> Self {
        TrivialDirectoryComparator {}
    }
}

impl DirectoryComparator for TrivialDirectoryComparator {
    #[allow(unused_variables)]
    fn mark_as_seen(&mut self, dir: &mut Box<FileIterator>) {}

    #[allow(unused_variables)]
    fn exists_in_directory(&self, file: &PathBuf) -> Option<PathBuf> {
        Some(file.clone())
    }
}

/// Now try to generalize the logic for ones that take a FileComparable
trait UsesFileComparator<C : FileComparable> : DirectoryComparator {
    fn new(comparator: C) -> Self;
    fn comparator(&mut self) -> &mut C;
}

// uh... reimplement Md5 Comparator?
pub struct Foo {
    comp : Md5Comparator
}

impl UsesFileComparator<Md5Comparator> for Foo {
    fn new(comparator: Md5Comparator) -> Self { Foo{comp : comparator} }
    fn comparator(&mut self) -> &mut Md5Comparator { &mut self.comp }
}

impl DirectoryComparator for Foo {
    #[allow(unused_variables)]
    fn mark_as_seen(&mut self, dir: &mut Box<FileIterator>) {}

    #[allow(unused_variables)]
    fn exists_in_directory(&self, file: &PathBuf) -> Option<PathBuf> {
        Some(file.clone())
    }
}

/*
impl<T, C> DirectoryComparator for T where T : UsesFileComparator<C>, C : FileComparable {
    #[allow(unused_variables)]
    fn mark_as_seen(&mut self, dir: &mut Box<FileIterator>) {}

    #[allow(unused_variables)]
    fn exists_in_directory(&self, file: &PathBuf) -> Option<PathBuf> {
        let foo : C = self.comparator();
        Some(file.clone())
    }
}
*/


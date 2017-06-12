use std::fs::*;
use std::fmt;
use std::path::PathBuf; // Still unclear on the difference between Path and PathBuf
use std::collections::vec_deque::VecDeque;

pub struct DirectoryFiles<'a> {
    root_dir: &'a PathBuf, // We can just borrow a root path so we don't have to make a copy for ourselves-- or is it better design for this struct to own its own copy of the path?
    queue: VecDeque<DirEntry>,
    pub num_found_items: u32
    // what do we put in here? Probably a queue of some sort for our BFS.
    // Probably a representation of the top level directory
    // and maybe a count of files found so far?
}

impl<'a> DirectoryFiles<'a> {
    pub fn new(root_dir: &PathBuf) -> DirectoryFiles {
        let mut df = DirectoryFiles { root_dir: root_dir, queue: VecDeque::new(), num_found_items: 0 };
        df.enqueue(root_dir);
        df
    }

    fn enqueue(&mut self, dir: &PathBuf) {
        match read_dir(dir) {
            Ok(dir_entries) =>
                for entry in dir_entries {
                    match entry {
                        Ok(result) => self.queue.push_back(result),
                        Err(msg) => println!("Failed to read directory entry under {}: {}", dir.display(), msg)
                    }
                },
            Err(msg) => println!("Failed to read {}: {}", dir.display(), msg)
        }
    }
}

// Breadth-first trasversal popping off files one at a time
impl<'a> Iterator for DirectoryFiles<'a> {
    // may have to be a different representation of file names?
    type Item = DirEntry;

    fn next(&mut self) -> Option<DirEntry> {
        match self.queue.pop_front() {
            None => None,
            Some(elem) => {
                self.num_found_items += 1;
                if elem.path().is_dir() {
                    self.enqueue(&elem.path());
                    self.next() // we only wanna give out files
                }
                else {
                    Some(elem)
                }
            }
        }
    }
}

// Pretty-printing state
impl<'a> fmt::Display for DirectoryFiles<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} items found under directory {}", self.num_found_items, self.root_dir.display())
    }
}
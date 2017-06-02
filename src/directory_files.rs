pub struct DirectoryFiles {
    // what do we put in here? Probably a queue of some sort for our BFS.
    // Probably a representation of the top level directory
    // and maybe a count of files found so far?
}

impl DirectoryFiles {
    pub fn new() -> DirectoryFiles {
        DirectoryFiles {}
    }
}

// Breadth-first trasversal popping off files one at a time
impl Iterator for DirectoryFiles {
    // may have to be a different representation of file names?
    type Item = String;

    fn next(&mut self) -> Option<String> {
        None
    }
}
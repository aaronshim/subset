extern crate crypto;

use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

use self::crypto::md5::Md5;
use self::crypto::digest::Digest;

pub trait FileComparable {
    type Key : Ord; // What might have to happen is that we need to concretize this into an actual type :(
    fn get_key(&mut self, file: &PathBuf) -> Option<Self::Key>;
}

// We should move this out to a submodule, maybe? But I can't figure out Rust's module system :(

pub struct Md5Comparable;

impl Md5Comparable {
    pub fn new() -> Md5Comparable {
        Md5Comparable {}
    }
}

impl FileComparable for Md5Comparable {
    type Key = String;
    fn get_key(&mut self, file_path: &PathBuf) -> Option<String> {
        // Yuck! There must be some monadic simplification here!
        match fs::File::open(&file_path) {
            Ok(file) => {
                
                let mut buf_reader = io::BufReader::new(file);
                let mut contents = Vec::new();

                match buf_reader.read_to_end(&mut contents) {
                    Ok(_) => {
                        let mut sh = Md5::new();
                        sh.input(&contents);
                        //println!("{}", sh.result_str());
                        Some(sh.result_str())
                    },
                    Err(msg) => {println!("Cannot read file {} to calculate hash: {}", file_path.display(), msg); None }
                }
            },
            Err(msg) => { println!("Cannot open file {} to calculate hash: {}", file_path.display(), msg); None }
        }
    }
}

// Another one

pub struct FileNameComparable;

impl FileNameComparable {
    pub fn new() -> FileNameComparable {
        FileNameComparable {}
    }
}

impl FileComparable for FileNameComparable {
    type Key = String;

    fn get_key(&mut self, file_path: &PathBuf) -> Option<String> {
        // Some monadic simplification would be much appreciated here ;)
        match file_path.file_name() {
            Some(s) => {
                match s.to_os_string().into_string() {
                    Ok(filename) => Some(filename),
                    Err(_) => None
                }
            },
            None => None
        }
    }
}
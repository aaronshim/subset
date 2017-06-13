extern crate crypto;

use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

use self::crypto::md5::Md5;
use self::crypto::digest::Digest;

pub trait FileComparable<T> {
    fn get_key(&mut self, file: &PathBuf) -> Option<T> where T : Ord;
}

// Generic trait to compose two comprators together (because we should be able to come up with an ordered set out of the product of two ordered sets.)
// It's a bonus you can add on top of just being a comparator.
pub trait FileComparableComposable<S, T, U, V> : FileComparable<S> {
    fn compose(&mut self, other: &mut U, file: &PathBuf) -> Option<V> where S : Ord, T : Ord, U : FileComparable<T>, V : Ord;
}

// We should move this out to a submodule, maybe? But I can't figure out Rust's module system :()

pub struct Md5Comparator;

impl Md5Comparator {
    pub fn new() -> Md5Comparator {
        Md5Comparator {}
    }
}

impl FileComparable<String> for Md5Comparator {
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

// To see if our Trait-based strategy pattern will work

pub struct TrivialComparator;

impl TrivialComparator {
    pub fn new() -> TrivialComparator {
        TrivialComparator {}
    }
}

impl FileComparable<u32> for TrivialComparator {
    fn get_key(&mut self, file_path: &PathBuf) -> Option<u32> {
        Some(1)
    }
}
subset
=====
[![Travis](https://travis-ci.org/aaronshim/subset.svg?branch=master)](https://travis-ci.org/aaronshim/subset)

**subset** is a tool to see whether all the files of the first directory are contained within the second directory. There are different strategies you can specify for what constitutes equal files (full md5 hash check, file name check, etc.) It is currently still a work in progress.

```
Usage: subset [-q | -v] [-t | -n] [-b] <dir1> <dir2>
       subset --help

subset lets you compare two directory structures.

We are going to check whether the files in dir1 are a subset of the files in dir2, regardless of directory structure.

We are going to check to see that every file under the directory structure in dir1 must be present somewhere in the dir2 directory structure, regardless of where in the directory structure or definitions of equality.

There are multiple definitions of file equality that you can specify using flags, but the default is a MD5 hash of the contents of the file. It is conceivable that you can define a custom equality strategy that relies on other parameters, such as file name, subdirectory location, metadata, EXIF data, etc. The possibilities are endless.

Common options:
    -h, --help           Show this usage message.
    -q, --quiet          Do not print all mappings.
    -v, --verbose        Print all mappings.
    -t, --trivial        Will swap out the MD5 comparison for a trivial comparison (everything is equal). (This is to test extensibility.)
    -n, --name           Will swap out the MD5 comparison for a filename comparison.
    -b, --bidirectional  Also check whether dir2 is also a subset of dir1 (essentially, set equality) and print out missing lists for both directories.
```

## Usage

    cargo run -- <flags and inputs>

## TODO

- Write mocking support for file and directory read operations so we can write tests
- Write tests
- Come up with more strategies for file equality
- Move to a more recent versio of Docopt that uses serde
- Parallelize comparison key generating operation (this should be embarassingly parallel)

## How to contribute

Feel free to send me a pull request!
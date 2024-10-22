use std::collections::{HashMap, HashSet};
use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::PathBuf;

type Folders = HashMap<OsString, usize>;
type Files = HashSet<OsString>;

const MAX_SIZE: usize = 70_000_000;
const NEEDED_SIZE: usize = 30_000_000;
fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let folders = extract(&file);

    let smallest = find_smallest_folder_to_delete(&folders);

    println!("{}", smallest);
}

/// Finds the smallest folder that will clear enough space for the
/// update of deleted
fn find_smallest_folder_to_delete(folders: &Folders) -> usize {
    let free_size = MAX_SIZE - *folders.get(OsStr::new("/")).unwrap();
    let size_to_delete = NEEDED_SIZE - free_size;

    let mut smallest_found: usize = MAX_SIZE;

    for &folder_size in folders.values() {
        if folder_size < size_to_delete { continue; }
        if folder_size < smallest_found { smallest_found = folder_size; }
    }

    smallest_found
}

/// Extracts the folders and their sizes from the file
fn extract(to_extract: &str) -> Folders {
    let mut folders: Folders = HashMap::new();
    let mut files: Files = HashSet::new();
    let mut path: PathBuf = PathBuf::new();

    for line in to_extract.lines() {
        let contents: Vec<&str> = line.split(" ").collect();

        // Ignore things we don't need
        if contents.starts_with(&["dir"]) { continue; }
        if contents.starts_with(&["$", "ls"]) { continue; }

        if contents.starts_with(&["$", "cd"]) {
            path = move_path(path, contents.get(2).unwrap());
            folders.entry(path.clone().into_os_string()).or_default();
            continue;
        }

        let size: usize = contents.first().unwrap().parse().unwrap();
        let file: &str = contents.last().unwrap();

        let mut file_path = path.clone();
        file_path.push(file);

        if files.contains(file_path.as_os_str()) { continue; }

        // Add this files size to all it's ancestors
        for ancestor in file_path.ancestors().skip(1) {
            let current_size = folders.get_mut(ancestor.as_os_str()).unwrap();
            *current_size += size;
        }

        files.insert(file_path.into_os_string());
    }

    folders
}

/// Moves the path to the specified location
fn move_path(mut path: PathBuf, to: &str) -> PathBuf {
    match to {
        "/" => {
            path.clear();
            path.push("/");
        }
        ".." => { path.pop(); }
        _ => { path.push(to); }
    }

    path
}
use std::collections::{HashMap, HashSet};
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;

type Folders = HashMap<OsString, usize>;
type Files = HashSet<OsString>;

const MAX_SIZE: usize = 100_000;
fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let folders = extract(&file);

    let total = get_sum_of_folders_below_max(&folders);

    println!("{}", total);
}

/// Returns the sum of folders which size is less or equal to MAX_SIZE
fn get_sum_of_folders_below_max(folders: &Folders) -> usize {
    let mut total: usize = 0;

    for folder_size in folders.values() {
        if *folder_size > MAX_SIZE { continue; }
        total += folder_size;
    }

    total
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
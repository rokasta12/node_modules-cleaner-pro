use dirs;
use std::collections::HashSet;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

fn main() {
    let home_dir = dirs::home_dir().expect("Could not find home directory");
    let start_path = home_dir.join("documents/github/");

    let mut visited = HashSet::new();

    for entry in WalkDir::new(&start_path)
        .min_depth(1)
        .max_depth(2)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if is_node_modules_dir(&entry) {
            let parent_dir = entry.path().parent().unwrap_or_else(|| Path::new(""));
            if visited.insert(parent_dir.to_path_buf()) {
                match calculate_dir_size(entry.path()) {
                    Ok(size) => {
                        let size_mb = size as f64 / 1_000_000.0;
                        println!("{:.2} MB - {:?}", size_mb, entry.path());
                    }
                    Err(e) => eprintln!("Error calculating size for {:?}: {}", entry.path(), e),
                }
            }
        }
    }
}

fn is_node_modules_dir(entry: &DirEntry) -> bool {
    entry.file_name() == "node_modules" && entry.metadata().map(|m| m.is_dir()).unwrap_or(false)
}

fn calculate_dir_size(path: &Path) -> std::io::Result<u64> {
    let mut size = 0;
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        size += entry.metadata()?.len();
    }
    Ok(size)
}

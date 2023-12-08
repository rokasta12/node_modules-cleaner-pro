use dirs;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::Path;
use std::time::SystemTime;
use walkdir::{DirEntry, WalkDir};

fn main() -> io::Result<()> {
    let home_dir = dirs::home_dir().expect("Could not find home directory");
    let start_path = home_dir.join("documents/github/");

    let mut visited = HashSet::new();
    let mut node_modules_folders = Vec::new();
    let mut total_size = 0;

    for entry in WalkDir::new(&start_path)
        .min_depth(1)
        .max_depth(2)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if is_node_modules_dir(&entry) {
            let parent_dir = entry.path().parent().unwrap_or_else(|| Path::new(""));
            if visited.insert(parent_dir.to_path_buf()) {
                let size = calculate_dir_size(entry.path())?;
                total_size += size;
                let last_modified = get_last_modified_time(parent_dir)?;
                let folder_name = parent_dir.file_name().unwrap_or_default().to_string_lossy();
                node_modules_folders.push((size, folder_name.to_string(), last_modified));
            }
        }
    }

    // Sort folders by size (largest to smallest)
    node_modules_folders.sort_by(|a, b| b.0.cmp(&a.0));

    // Print the table
    println!(
        "{:>15} {:<30} {:<20}",
        "Size (MB)", "Project", "Last Modified"
    );
    for (size, folder_name, last_modified) in node_modules_folders {
        let size_mb = size as f64 / 1_000_000.0;
        println!(
            "{:>15.2} {:<30} {:<20}",
            size_mb, folder_name, last_modified
        );
    }

    // Print total size
    let total_size_mb = total_size as f64 / 1_000_000.0;
    println!("\nTotal Size: {:.2} MB", total_size_mb);

    Ok(())
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

fn get_last_modified_time(path: &Path) -> io::Result<String> {
    let metadata = fs::metadata(path)?;
    let last_modified = metadata.modified()?;
    let now = SystemTime::now();
    match now.duration_since(last_modified) {
        Ok(duration) => {
            let days = duration.as_secs() / 86_400; // seconds in a day
            Ok(format!("{} days ago", days))
        }
        Err(_) => Ok("Time Error".to_string()),
    }
}

use std::ffi::OsStr;
use walkdir::WalkDir;

fn main() {
    println!("List Markdown files in content directory");
    for entry in WalkDir::new("./content")
        .into_iter()
        .filter_map(|e| e.ok()) // Ignore any errors during traversal
        .filter(|e| {
            // First, check if the entry is a file
            e.file_type().is_file() &&
            // Then, check if the file has the ".md" extension
            e.path().extension().and_then(OsStr::to_str) == Some("md")
        })
    {
        println!("{}", entry.path().display());
    }
}

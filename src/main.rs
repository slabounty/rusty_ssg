use std::ffi::OsStr;
use walkdir::WalkDir;

fn main() {
    println!("Rusty Static Site Generator");
    convert_files("./content", "./template", "./output");
}

fn convert_files(content_dir: &str, _template_dir: &str, _output_dir: &str) {
    println!("List Markdown files in {} directory", content_dir);
    for entry in WalkDir::new(content_dir)
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

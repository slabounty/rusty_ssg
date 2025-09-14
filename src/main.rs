use std::fs;
use std::ffi::OsStr;
use walkdir::WalkDir;
use pulldown_cmark::{Parser, Options, html};


fn main() {
    println!("Rusty Static Site Generator");

    // At somepoint make it optional to get from the command line.
    let content = String::from("./content");
    let template = String::from("./template");
    let output = String::from("./output");

    // Convert the files in content with the template files and put them in the output directory.
    convert_files(&content, &template, &output);
}

fn convert_files(content_dir: &String, _template_dir: &str, _output_dir: &str) {
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
        convert_file_to_html(&entry.path().display().to_string());
    }
}

fn convert_file_to_html(md_file_path: &String) {
    println!("Markdown file = {}", md_file_path);
    let markdown_input = fs::read_to_string(md_file_path);
    match markdown_input {
        Ok(markdown_text) => convert_md_text_to_html(&markdown_text),
        Err(e) => println!("Operation failed: {}", e), // std::io::Error implements Display
    }
}

fn convert_md_text_to_html(markdown_text: &String) {
    println!("Markdown text = \n{}", markdown_text);

    // Set up options (e.g., enable tables, footnotes, etc.)
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    // Add more options as needed

    let parser = Parser::new_ext(&markdown_text, options);

    // Create a buffer to store the HTML output
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // Print or save the HTML output
    println!("HTML output =\n{}", html_output);
}

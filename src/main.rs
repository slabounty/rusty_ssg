use std::fs;
use std::ffi::OsStr;
use pulldown_cmark::{Parser, Options, html};
use tera::{Tera, Context};
use walkdir::WalkDir;


fn main() {
    println!("Rusty Static Site Generator");

    // At somepoint make it optional to get from the command line.
    let content = String::from("./content");
    let template = String::from("./templates/*.html");
    let output = String::from("./output");

    // Convert the files in content with the template files and put them in the output directory.
    convert_files(&content, &template, &output);
}

fn convert_files(content_dir: &str, template_dir: &str, _output_dir: &str) {
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
        convert_file_to_html(&entry.path().display().to_string(), template_dir);
    }
}

fn convert_file_to_html(md_file_path: &str, template_dir: &str) {
    println!("Markdown file = {}", md_file_path);
    let markdown_input = fs::read_to_string(md_file_path);
    match markdown_input {
        Ok(markdown_text) => convert_md_text_to_html(&markdown_text, template_dir),
        Err(e) => println!("Operation failed: {}", e), // std::io::Error implements Display
    }
}

fn convert_md_text_to_html(markdown_text: &str, template_dir: &str) {
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

    let base_html = String::from("base.html"); // The template we'll use
    let tera = Tera::new(template_dir).unwrap(); // Let tera know where the templates are

    // Create a context and add the data into it.
    let mut context = Context::new();
    context.insert("title", "The Title"); // Insert the title (in the base.html this is {{ title }})
    context.insert("content", &html_output); // Insert the generated HTML content (in the base.html this is {{ content }})

    // Render the html from the template and the context
    let rendered_html = tera.render(&base_html, &context).unwrap();
    println!("Rendered html = \n {}", rendered_html);
}

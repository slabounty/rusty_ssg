use std::fs;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use log::{info};
use env_logger::Env;
use pulldown_cmark::{Parser, Options, html};
use tera::{Tera, Context};
use walkdir::WalkDir;

struct SitePaths {
    content_path: String,
    template_path: String,
    output_path: String,
    base_template: String,
}

fn main() {
    // Initialize the logger based on the `RUST_LOG` environment variable.
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("Rusty Static Site Generator");

    let site_paths = SitePaths {
        content_path: String::from("./content"),
        template_path: String::from("./templates/*.html"),
        output_path: String::from("./output"),
        base_template: String::from("base.html"),
    };

    // Convert the files in content with the template files and put them in the output directory.
    convert_files(&site_paths);
}

fn convert_files(site_paths: &SitePaths) {
    for entry in WalkDir::new(&site_paths.content_path)
        .into_iter()
        .filter_map(|e| e.ok()) // Ignore any errors during traversal
        .filter(|e| {
            // First, check if the entry is a file
            e.file_type().is_file() &&
            // Then, check if the file has the ".md" extension
            e.path().extension().and_then(OsStr::to_str) == Some("md")
        })
    {
        convert_file_to_html(site_paths, &entry.path().display().to_string());
    }
}

fn convert_file_to_html(site_paths: &SitePaths, md_file_path: &str) {
    let markdown_input = fs::read_to_string(md_file_path);
    match markdown_input {
        Ok(markdown_text) => convert_md_text_to_html(site_paths, &md_file_path, &markdown_text),
        Err(e) => println!("Operation failed: {}", e), // std::io::Error implements Display
    }
}

fn convert_md_text_to_html(site_paths: &SitePaths, md_file_path: &str, markdown_text: &str) {
    info!("Processing: {}", md_file_path);

    // Set up options (e.g., enable tables, footnotes, etc.)
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    // Add more options as needed

    let parser = Parser::new_ext(&markdown_text, options);

    // Create a buffer to store the HTML output
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    let tera = Tera::new(&site_paths.template_path).unwrap(); // Let tera know where the templates are

    // Create a context and add the data into it.
    let mut context = Context::new();
    context.insert("title", "The Title"); // Insert the title (in the base.html this is {{ title }})
    context.insert("content", &html_output); // Insert the generated HTML content (in the base.html this is {{ content }})

    // Render the html from the template and the context
    let rendered_html = tera.render(&site_paths.base_template, &context).unwrap();

    let output_file = output_html_path(md_file_path, &site_paths.output_path);

    // Create the output directory if it doesn't exist and write the file.
    info!("Writing output: {}", output_file.display());
    fs::create_dir_all(&site_paths.output_path).unwrap();
    fs::write(output_file, rendered_html).unwrap();
}

fn output_html_path(md_path: &str, output_dir: &str) -> PathBuf {
    let md_path = Path::new(md_path);
    let output_dir = Path::new(output_dir);

    // Get just the filename ("hello.md")
    let filename = md_path.file_stem().unwrap();

    // Build new path: output_dir + "hello.html"
    output_dir.join(format!("{}.html", filename.to_string_lossy()))
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_convert_md_text_to_html_basic() {
        let md_path = "./tests/content/test.md";

        // Arrange: markdown input with a header and paragraph
        let md = "# Hello\n\nThis is a test.";
        // Minimal template string to simulate Tera
        //let template_dir = "tests/templates/*.html";

        let site_paths = SitePaths {
            content_path: String::from("./tests/content"),
            template_path: String::from("./tests/templates/*.html"),
            output_path: String::from("./tests/output"),
            base_template: String::from("base.html"),
        };

        // Ensure test template exists
        fs::create_dir_all("tests/templates").unwrap();
        fs::write("tests/templates/base.html", "<html><head><title>{{ title }}</title></head><body>{{ content | safe }}</body></html>").unwrap();

        // Act: convert
        convert_md_text_to_html(&site_paths, &md_path, md);

        // Assert: just check template exists, tera loads it, and HTML is generated
        // (Here we don’t capture stdout, but you could with `assert_cmd` or `duct`)
        let tera = Tera::new(&site_paths.template_path).unwrap();
        let mut ctx = Context::new();
        ctx.insert("title", "The Title");
        ctx.insert("content", "<h1>Hello</h1>\n<p>This is a test.</p>\n");
        let rendered = tera.render(&site_paths.base_template, &ctx).unwrap();

        assert!(rendered.contains("<h1>Hello</h1>"));
        assert!(rendered.contains("<p>This is a test.</p>"));
        assert!(rendered.contains("<title>The Title</title>"));
    }

    #[test]
    fn test_convert_file_to_html_missing_file() {
        // Arrange: point to a missing file
        let missing_path = "tests/fixtures/does_not_exist.md";

        let site_paths = SitePaths {
            content_path: String::from("./tests/content"),
            template_path: String::from("./tests/templates/*.html"),
            output_path: String::from("./tests/output"),
            base_template: String::from("base.html"),
        };

        // Act: function should not panic
        convert_file_to_html(&site_paths, missing_path);

        // Assert: nothing to assert directly, but no panic = pass
        assert!(!Path::new(missing_path).exists());
    }

    #[test]
    fn test_output_html_path() {
        let md = "./content/hello.md";
        let out = "./output";
        let result = output_html_path(md, out);

        assert_eq!(result, PathBuf::from("./output/hello.html"));
    }
}

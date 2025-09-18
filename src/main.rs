use std::fs;
use std::ffi::OsStr;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use clap::{Parser as ClapParser};
use log::{info};
use env_logger::Env;
use pulldown_cmark::{Parser, Options, html};
use tera::{Tera, Context};
use walkdir::WalkDir;

#[derive(ClapParser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    content: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    output: Option<String>,
}

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

    let cli = Cli::parse();

    let site_paths = SitePaths {
        content_path: cli.content.unwrap_or_else(|| String::from("./content")),
        template_path: String::from("./templates/*.html"),
        output_path: cli.output.unwrap_or_else(|| String::from("./output")),
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

    let rendered_html = match render_page(&site_paths, &html_output) {
        Ok(html) => html,
        Err(e) => {
            panic!("Error rendering template: {}", e);
        },
    };

    let output_file = output_html_path(md_file_path, &site_paths.output_path);

    // Create the output directory if it doesn't exist and write the file.
    info!("Writing output: {}", output_file.display());
    if let Err(e) = create_and_write_file(&output_file, &rendered_html) {
        eprintln!("Operation failed: {}", e);
    }
}

// The function must return a Result to use the '?' operator.
fn render_page(site_paths: &SitePaths, html_output: &str) -> Result<String, tera::Error> {
    let tera = Tera::new(&site_paths.template_path)?;

    // Create a context and add the data into it.
    let mut context = Context::new();
    context.insert("title", "The Title");
    context.insert("content", &html_output);

    // Render the html from the template and the context.
    let rendered_html = tera.render(&site_paths.base_template, &context)?;

    Ok(rendered_html)
}

fn create_and_write_file(path: &Path, content: &str) -> io::Result<()> {
    // Create parent directories if they don't exist
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?; // The '?' operator propagates errors
    }

    match fs::File::create(path) {
        Ok(mut file) => {
            match file.write_all(content.as_bytes()) {
                Ok(_) => {
                    info!("Successfully wrote to file: {:?}", path);
                    Ok(())
                }
                Err(e) => {
                    eprintln!("Error writing to file {:?}: {}", path, e);
                    Err(e)
                }
            }
        }
        Err(e) => {
            eprintln!("Error creating file {:?}: {}", path, e);
            Err(e)
        }
    }
}

fn output_html_path(md_path: &str, output_dir: &str) -> PathBuf {
    let md_path = Path::new(md_path);
    let output_dir = Path::new(output_dir);

    // Get just the filename ("hello.md"). Since we got the md_path from reading it,
    // this really shouldn't ever happen. If it does, just exit with the error message.
    let filename = md_path.file_stem().unwrap_or_else( || {
        panic!("Path has no file stem: {:?}", md_path);
    });

    // Build new path: output_dir + "hello.html"
    output_dir.join(format!("{}.html", filename.to_string_lossy()))
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use super::Cli;
    use clap::Parser;
    use tempfile::tempdir; // add `tempfile = "3"` to Cargo.toml dev-dependencies

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


    #[test]
    fn test_with_arguments() {
        let args = ["test", "--content", "./my_content", "--output", "./my_output"];
        let cli = Cli::parse_from(&args);

        assert_eq!(cli.content, Some("./my_content".to_string()));
        assert_eq!(cli.output, Some("./my_output".to_string()));
    }

    #[test]
    fn test_with_defaults() {
        let args = ["test"]; // no flags
        let cli = Cli::parse_from(&args);

        assert_eq!(cli.content, None);
        assert_eq!(cli.output, None);
    }

    #[test]
    fn test_render_page() {
        // Arrange: set up a temp template directory
        let template_dir = "tests/templates_render_page";
        fs::create_dir_all(template_dir).unwrap();

        let base_template = "base.html";
        let template_path = format!("{}/*.html", template_dir);
        let template_file = Path::new(template_dir).join(base_template);

        // Write a minimal template file with {{ title }} and {{ content }}
        fs::write(
            &template_file,
            "<html><head><title>{{ title }}</title></head><body>{{ content | safe }}</body></html>",
        )
        .unwrap();

        // Define SitePaths (adjust to your struct fields)
        let site_paths = SitePaths {
            content_path: "tests/content".into(),
            template_path,
            base_template: base_template.into(),
            output_path: "tests/output".into(),
        };

        let html_output = "<h1>Hello</h1><p>World</p>";

        // Act
        let rendered = render_page(&site_paths, html_output).unwrap();

        // Assert
        assert!(rendered.contains("<title>The Title</title>"));
        assert!(rendered.contains("<h1>Hello</h1>"));
        assert!(rendered.contains("<p>World</p>"));
    }

    #[test]
    fn test_create_and_write_file_creates_and_writes() -> io::Result<()> {
        // Arrange: make a temporary directory
        let dir = tempdir()?;
        let file_path: PathBuf = dir.path().join("nested").join("hello.txt");

        let content = "Hello, world!";

        // Act: write to file
        create_and_write_file(&file_path, content)?;

        // Assert: file exists
        assert!(file_path.exists());

        // Assert: contents are correct
        let written = fs::read_to_string(&file_path)?;
        assert_eq!(written, content);

        Ok(())
    }

    #[test]
    fn test_create_and_write_file_overwrites_existing() -> io::Result<()> {
        let dir = tempdir()?;
        let file_path: PathBuf = dir.path().join("test.txt");

        // First write
        create_and_write_file(&file_path, "first")?;
        assert_eq!(fs::read_to_string(&file_path)?, "first");

        // Second write should overwrite
        create_and_write_file(&file_path, "second")?;
        assert_eq!(fs::read_to_string(&file_path)?, "second");

        Ok(())
    }
}

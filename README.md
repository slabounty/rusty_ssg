# Rust Based Static Site Generator

## Hour 1 – Project Setup

- Install/update Rust (rustup update).
- Create a new binary project:
- cargo new rusty_ssg


### Set up directory structure:

rusty_ssg/
  ├── content/   # your markdown files
  ├── templates/ # base.html
  └── output/    # generated HTML


### Add dependencies to Cargo.toml:

[dependencies]
walkdir = "2"        # For directory traversal
pulldown-cmark = "0.9"  # Markdown to HTML
tera = "1.20"        # Templating
anyhow = "1"         # Easier error handling

## Hour 2 – File Discovery

Write code to:

- Traverse content/ using walkdir.
- Filter for .md files.
- Print the list of discovered files.

Key Concepts:

- Iterators
- Result handling (? operator)
- Path manipulation (std::path::Path)

## Hour 3 – Basic Markdown Conversion

- Pick a single .md file and convert it to HTML using pulldown-cmark.
- Output raw HTML to the terminal for testing.

Key Concepts:

- Using external crates
- Reading file contents (std::fs::read_to_string)

## Hour 4 – Templating Setup

Create a simple templates/base.html:

<html>
  <head><title>{{ title }}</title></head>
  <body>{{ content | safe }}</body>
</html>


Render converted HTML into this template using tera.

Key Concepts:

- Templating with tera
- Structuring code with functions

## Hour 5 – Output Writing

Write generated HTML to output/ directory, preserving file names (example.md → example.html).

Key Concepts:

- Creating directories if they don’t exist (std::fs::create_dir_all)
- File writing (std::fs::write)

## Hour 6 – Process Multiple Files

- Loop through all .md files.
- For each: convert → template → write.
- Log each file processed.

Key Concepts:

- Iterators (for entry in ...)
- Handling multiple Results (collect::<Result<_,_>>()? or loops with ?)

## Hour 7 – CLI Interface

Add command-line arguments using clap (or argh) for:

- Input directory
- Output directory
- Optional: template path

Example:

cargo run -- --input content --output output

## Hour 8 – Error Handling & Cleanup

- Replace unwrap() calls with ? and anyhow::Result.
- Add meaningful error messages for missing directories, bad markdown, etc.
- Organize code into modules (mod cli; mod generator;).

## Hour 9 – Add Front Matter (Optional Bonus)

- Allow YAML front matter (title, date) using serde_yaml.
- Extract metadata before converting Markdown.
- Pass title into template dynamically.

## Hour 10 – Polish & Stretch

- Add logging (log + env_logger).
- Add a minimal CSS file.
- Test on multiple Markdown files.
- Optional: Watch mode with notify crate (auto-regenerate on file changes).

## End Result

A fully working static site generator:

- Converts Markdown → HTML
- Applies a template
- Writes to output directory
- Configurable via CLI
- Extendable into a real project later.

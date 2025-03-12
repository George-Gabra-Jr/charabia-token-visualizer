# Token Visualization

Token Visualization is a Rust project that reads an input text file, tokenizes its content, and generates an HTML file to visually display each token.

## Features

- **Tokenization:** Uses the charabia crate to break input into tokens.
- **Customization:** Supports extra separators and ignorable tokens via a TOML configuration file.
- **HTML Output:** Formats tokens in HTML with randomized light background colors and custom styling for separators.
- **File I/O:** Reads input from a text file and writes the token visualization output to an HTML file.

## Prerequisites

Rust and Cargo installed on your system:

<https://www.rust-lang.org/tools/install>.

Dependencies:

- rand
- charabia
- html_escape
- serde
- toml

## Building and Running

1. **Run the project using Cargo:**

     ```bash
     cargo run -- <input_file> [output_file]
     ```

     - `<input_file>`: Path to the text file to process.
     - `[output_file]`: Optional output file name (defaults to `output.html` if not provided).

2. **Build the project:**

     ```bash
     cargo build --release
     ```

3. **Running the Binary Directly:**
   After building the project, you can run the binary directly from the release folder:

     ```bash
     ./token_vis <input_file> [output_file]
     ```

## Configuration

If you wish to customize token separators, create a `config.toml` file in the project directory. Example:

```toml
extra_separators = [",", ".", ";"]
removed_separators = [" "]
```

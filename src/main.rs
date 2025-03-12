use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write, Read};
use rand::Rng;
use charabia::Tokenize;
use html_escape::encode_text;
use serde::Deserialize;
use toml;

#[derive(Deserialize)]
struct Config {
    extra_separators: Vec<String>,
    removed_separators: Vec<String>,
}

fn load_config() -> Config {
    let conf_filename = "config.toml";
    let config_file = File::open(conf_filename);
    match config_file {
        Ok(file) => {
            let mut s = String::new();
            let mut buf_reader = BufReader::new(file);
            if let Err(e) = buf_reader.read_to_string(&mut s) {
                eprintln!("Error reading config file: {}. Using defaults.", e);
                return Config {
                    extra_separators: Vec::new(),
                    removed_separators: Vec::new(),
                };
            }
            match toml::from_str(&s) {
                Ok(config) => config,
                Err(e) => {
                    eprintln!("Error parsing config file: {}. Using defaults.", e);
                    Config {
                        extra_separators: Vec::new(),
                        removed_separators: Vec::new(),
                    }
                }
            }
        }
        Err(_) => {
            // Use default empty config if the file does not exist.
            Config {
                extra_separators: Vec::new(),
                removed_separators: Vec::new(),
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    // Get command-line arguments: input file and (optionally) output file.
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file> [output_file]", args[0]);
        std::process::exit(1);
    }
    let input_filename = &args[1];
    let output_filename = if args.len() >= 3 { &args[2] } else { "output.html" };

    // Load configuration file for extra and removed separators.
    let config = load_config();

    // Open the input file.
    let input_file = File::open(input_filename)?;
    let reader = BufReader::new(input_file);

    // Create the output HTML file.
    let mut output_file = File::create(output_filename)?;

    // Write HTML header.
    writeln!(output_file, "<!DOCTYPE html>")?;
    writeln!(output_file, "<html lang=\"en\">")?;
    writeln!(output_file, "<head>")?;
    writeln!(output_file, "<meta charset=\"UTF-8\">")?;
    writeln!(output_file, "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">")?;
    writeln!(output_file, "<title>Token Visualization</title>")?;
    writeln!(output_file, "</head>")?;
    writeln!(output_file, "<body>")?;

    // Process each line in the input file.
    for line in reader.lines() {
        let line = line?;
        // Write the original line.
        writeln!(output_file, "<section style=\"border: 1px solid; margin-bottom: 10px; padding: 10px;\">")?;
        writeln!(output_file, "<p><strong>Original:</strong><br> {}</p>", encode_text(&line))?;

        // Tokenize the line using charabia.
        let str_line = line.as_str();
        let mut tokens = str_line.tokenize();
        write!(output_file, "<p><strong>Tokenized:</strong><br> ")?;
        while let Some(token) = tokens.next() {
            let lemma = token.lemma();
            // Skip tokens marked to be removed.
            if config.removed_separators.contains(&lemma.to_string()) {
                continue;
            }
            // Determine if the token is a separator.
            let is_separator = token.is_separator() || config.extra_separators.contains(&lemma.to_string());
            if is_separator {
                write!(
                    output_file,
                    "<span style=\"background-color: blue; color: white; padding: 1px; margin: 5px;\">{}</span>",
                    encode_text(lemma)
                )?;
            } else {
                // Generate a random light background color.
                let mut rng = rand::rng();
                let r: u8 = rng.random_range(180..=255);
                let g: u8 = rng.random_range(180..=255);
                let b: u8 = rng.random_range(180..=255);
                let color = format!("#{:02X}{:02X}{:02X}", r, g, b);
                write!(
                    output_file,
                    "<span style=\"background-color: {}; padding: 1px; margin: 5px;\">{}</span>",
                    color,
                    encode_text(lemma)
                )?;
            }
        }
        writeln!(output_file, "</p>")?;
        writeln!(output_file, "</section>")?;
    }

    // Write the HTML footer.
    writeln!(output_file, "</body>")?;
    writeln!(output_file, "</html>")?;

    Ok(())
}

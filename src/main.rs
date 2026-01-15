// src/main.rs
use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::path::{PathBuf};

/// Simple generator for Caddyfile.dev from a template with {{key}} placeholders
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Output directory where Caddyfile.dev will be created (default: current directory)
    #[arg(short = 'o', long = "output-dir", value_name = "DIR")]
    output_dir: Option<PathBuf>,

    /// Full path to the template file (default: <output-dir>/Caddyfile.template)
    #[arg(short = 't', long = "template", value_name = "FILE")]
    template: Option<PathBuf>,

    /// Variables in key=value format (can be repeated)
    #[arg(long = "var", value_name = "KEY=VALUE", value_parser = parse_key_val)]
    variables: Vec<(String, String)>,
}

/// Parse a single key=value pair
fn parse_key_val(s: &str) -> Result<(String, String), String> {
    let parts: Vec<&str> = s.splitn(2, '=').collect();
    if parts.len() != 2 {
        return Err(format!("Invalid variable format: '{}'. Expected key=value", s));
    }
    Ok((parts[0].to_string(), parts[1].to_string()))
}

fn main() {
    let args = Args::parse();

    // Output directory (default: current)
    let output_dir = args.output_dir.unwrap_or_else(|| PathBuf::from("."));
    if !output_dir.is_dir() {
        eprintln!("Error: Output directory '{}' does not exist or is not a directory.", output_dir.display());
        std::process::exit(1);
    }

    // Template path (default: output_dir/Caddyfile.template)
    let template_path = args.template.unwrap_or_else(|| output_dir.join("Caddyfile.template"));

    // Read template content
    let template_content = match fs::read_to_string(&template_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading template '{}': {}", template_path.display(), e);
            std::process::exit(1);
        }
    };

    // Collect variables into a HashMap
    let vars: HashMap<String, String> = args.variables.into_iter().collect();

    // Perform substitutions
    let mut result = template_content;
    for (key, value) in &vars {
        let placeholder = format!("{{{{{}}}}}", key);
        result = result.replace(&placeholder, value);
    }

    // Final output path
    let output_path = output_dir.join("Caddyfile.dev");

    // Write the result
    if let Err(e) = fs::write(&output_path, result) {
        eprintln!("Error writing '{}': {}", output_path.display(), e);
        std::process::exit(1);
    }

    println!("Caddyfile.dev successfully generated at: {}", output_path.display());
    if !vars.is_empty() {
        println!("Applied variables: {:?}", vars.keys().collect::<Vec<_>>());
    } else {
        println!("No variables provided → template copied without changes.");
    }
    println!("Reload Caddy with: caddy reload (or it will auto-reload with --watch)");
}

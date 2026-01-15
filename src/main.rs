// src/main.rs
use clap::{Parser, Subcommand};
use dialoguer::{Confirm, Input};
use dirs::config_dir;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Simple generator for Caddyfile.dev from a template with {{key}} placeholders
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, subcommand_required = true)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

/// Available commands for caddy-dev
#[derive(Subcommand, Debug)]
enum Command {
    /// Generate Caddyfile.dev from a template
    Generate {
        /// Output directory where Caddyfile.dev will be created (default: current directory)
        #[arg(short = 'o', long = "output-dir", value_name = "DIR")]
        output_dir: Option<PathBuf>,

        /// Full path to the template file (default: <output-dir>/Caddyfile.template)
        #[arg(short = 't', long = "template", value_name = "FILE")]
        template: Option<PathBuf>,

        /// Variables in key=value format (can be repeated)
        #[arg(long = "var", value_name = "KEY=VALUE", value_parser = parse_key_val)]
        variables: Vec<(String, String)>,
    },

    /// Initialize caddy-dev by setting up folders to import Caddyfile.dev from
    Init,

    /// Reload Caddy with the generated config
    Reload,
}

/// Parse a single key=value pair
fn parse_key_val(s: &str) -> Result<(String, String), String> {
    let parts: Vec<&str> = s.splitn(2, '=').collect();
    if parts.len() != 2 {
        return Err(format!(
            "Invalid variable format: '{}'. Expected key=value",
            s
        ));
    }
    Ok((parts[0].to_string(), parts[1].to_string()))
}

/// Get the caddy-dev config directory (~/.config/caddy-dev)
fn get_config_dir() -> PathBuf {
    // Use XDG-compliant ~/.config/caddy-dev for cross-platform consistency
    // This overrides the platform-specific config_dir() to ensure consistent behavior
    if let Some(home_dir) = dirs::home_dir() {
        home_dir.join(".config").join("caddy-dev")
    } else {
        // Fallback to platform-specific config directory if home not found
        config_dir()
            .unwrap_or_else(|| PathBuf::from("/home/.config"))
            .join("caddy-dev")
    }
}

/// Get the main Caddyfile path in config directory
fn get_main_caddyfile_path() -> PathBuf {
    get_config_dir().join("Caddyfile")
}

/// Generate Caddyfile.dev from template
fn generate_caddyfile_dev(
    output_dir: Option<PathBuf>,
    template: Option<PathBuf>,
    variables: Vec<(String, String)>,
) {
    // Output directory (default: current)
    let output_dir = output_dir.unwrap_or_else(|| PathBuf::from("."));
    if !output_dir.is_dir() {
        eprintln!(
            "Error: Output directory '{}' does not exist or is not a directory.",
            output_dir.display()
        );
        std::process::exit(1);
    }

    // Template path (default: output_dir/Caddyfile.template)
    let template_path = template.unwrap_or_else(|| output_dir.join("Caddyfile.template"));

    // Read template content
    let template_content = match fs::read_to_string(&template_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!(
                "Error reading template '{}': {}",
                template_path.display(),
                e
            );
            std::process::exit(1);
        }
    };

    // Collect variables into a HashMap
    let vars: HashMap<String, String> = variables.into_iter().collect();

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

    println!(
        "Caddyfile.dev successfully generated at: {}",
        output_path.display()
    );
    if !vars.is_empty() {
        println!("Applied variables: {:?}", vars.keys().collect::<Vec<_>>());
    } else {
        println!("No variables provided → template copied without changes.");
    }
    println!("Reload Caddy with: caddy-dev reload");
}

/// Interactive initialization to set up import folders
fn init_caddydev() {
    println!("=== Caddy-dev Initialization ===");
    println!("This will help you configure which folders to import Caddyfile.dev from.");
    println!();

    // Get or create config directory
    let config_dir = get_config_dir();
    if let Err(e) = fs::create_dir_all(&config_dir) {
        eprintln!(
            "Error creating config directory '{}': {}",
            config_dir.display(),
            e
        );
        std::process::exit(1);
    }

    // Check if there's an existing configuration
    let main_caddyfile_path = get_main_caddyfile_path();
    let has_existing = main_caddyfile_path.exists();

    if has_existing {
        println!(
            "Found existing configuration at: {}",
            main_caddyfile_path.display()
        );
        if !Confirm::new()
            .with_prompt("Do you want to overwrite it?")
            .default(false)
            .interact()
            .expect("Failed to read input")
        {
            println!("Keeping existing configuration.");
            return;
        }
    }

    // Interactive folder selection
    println!("Enter the folders (or glob patterns) containing Caddyfile.dev files.");
    println!("Examples:");
    println!("  - /path/to/project");
    println!("  - /path/to/**/Caddyfile.dev");
    println!("  - ~/projects/*/Caddyfile.dev");
    println!();
    println!("Press Enter after each entry. Enter an empty line when done.");

    let mut folders: Vec<String> = Vec::new();

    loop {
        let input: String = Input::new()
            .with_prompt(format!("Folder {} (or glob pattern)", folders.len() + 1))
            .allow_empty(true)
            .interact()
            .expect("Failed to read input");

        if input.trim().is_empty() {
            break;
        }

        // Expand home directory if present
        let expanded = if input.starts_with("~") {
            match dirs::home_dir() {
                Some(home) => home.join(&input[2..]).to_string_lossy().into_owned(),
                None => input,
            }
        } else {
            input
        };

        folders.push(expanded);
    }

    if folders.is_empty() {
        println!("No folders specified. Configuration not saved.");
        return;
    }

    // Generate the main Caddyfile with imports
    let mut caddyfile_content = String::new();
    caddyfile_content.push_str("# Auto-generated by caddy-dev init\n");
    caddyfile_content.push_str("# Edit this file or run 'caddy-dev init' to reconfigure\n\n");

    // Process each folder/pattern and add imports
    caddyfile_content.push_str("# Import Caddyfile.dev files from configured folders\n");

    for folder in &folders {
        caddyfile_content.push_str(&format!("# Pattern: {}\n", folder));
        let clean_folder = folder.trim_end_matches('/');

        // Check if it's a glob pattern (contains * or ?)
        if clean_folder.contains('*') || clean_folder.contains('?') {
            // It's a glob pattern - use it directly
            caddyfile_content.push_str(&format!("import {}\n", clean_folder));
        } else {
            // It's a directory path - clean trailing slashes and generate glob pattern
            // Caddy glob patterns only support single wildcards, not **
            caddyfile_content.push_str(&format!("import {}/*/Caddyfile.dev\n", clean_folder));
        }
    }

    // Write the main Caddyfile
    if let Err(e) = fs::write(&main_caddyfile_path, caddyfile_content) {
        eprintln!(
            "Error writing configuration to '{}': {}",
            main_caddyfile_path.display(),
            e
        );
        std::process::exit(1);
    }

    println!();
    println!("Configuration saved to: {}", main_caddyfile_path.display());
    println!("Imported {} folder(s).", folders.len());
    println!("Run 'caddy-dev reload' to apply the configuration.");
}

/// Reload Caddy with the generated config
fn reload_caddy() {
    let main_caddyfile_path = get_main_caddyfile_path();

    if !main_caddyfile_path.exists() {
        eprintln!(
            "Error: Configuration file not found at '{}'",
            main_caddyfile_path.display()
        );
        eprintln!("Run 'caddy-dev init' first to set up the configuration.");
        std::process::exit(1);
    }

    println!(
        "Reloading Caddy with config: {}",
        main_caddyfile_path.display()
    );

    // Execute caddy reload
    let status = std::process::Command::new("caddy")
        .args(&["reload", "--config", main_caddyfile_path.to_str().unwrap()])
        .status()
        .expect("Failed to execute 'caddy reload'");

    if status.success() {
        println!("Caddy successfully reloaded!");
    } else {
        eprintln!(
            "Error: Caddy reload failed with exit code: {:?}",
            status.code()
        );
        std::process::exit(1);
    }
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Generate {
            output_dir,
            template,
            variables,
        } => {
            generate_caddyfile_dev(output_dir, template, variables);
        }
        Command::Init => {
            init_caddydev();
        }
        Command::Reload => {
            reload_caddy();
        }
    }
}

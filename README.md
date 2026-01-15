# caddy-dev

Caddy development tool for generating and managing Caddyfile.dev from templates.

## Overview

`caddy-dev` is a Rust CLI application that simplifies local Caddy web server development by:

- **Generating** `Caddyfile.dev` from templates with variable substitution
- **Initializing** project configurations with folder import setup
- **Reloading** Caddy with the generated configuration

The tool bridges local development templates with Caddy's configuration system, enabling modular Caddyfile management across multiple projects.

## Why caddy-dev?

**Configure once, use everywhere.**

Managing local development environments is tedious:

- Manual Caddyfile configuration for each project
- Different domains and subdomains to remember
- No consistent way to generate configs for multiple worktrees
- Projects don't start when you access their domain

caddy-dev gives you a template-driven workflow:

1. **Create a template** — Define domains, subdomains, reverse proxy targets, TLS, environment variables, and any Caddy configuration exactly how you want
2. **Generate for any worktree** — Run one command to generate a `Caddyfile.dev` for any Git worktree. Variables are replaced automatically
3. **Centralize with imports** — Configure caddy-dev to import all your `Caddyfile.dev` files into a single main Caddyfile
4. **Auto-start projects** — Use Caddy's `php_server`, `spawn_backend`, or `exec` directives to automatically start your development servers when you access their domain

### Example Workflow

Suppose you have multiple projects in `~/Developer`:

```
~/Developer/
├── project-a
├── project-b
└── my-monorepo
```

Create a template for your projects:

```bash
# project-a/Caddyfile.template
{{subdomain}}.localhost {
    reverse_proxy localhost:{{port}}
    tls internal
}

# project-b/Caddyfile.template
{{subdomain}}.localhost {
    reverse_proxy localhost:{{port}}
    tls internal
}

# my-monorepo/Caddyfile.template
{{subdomain}}.localhost {
	handle /api* {
		reverse_proxy localhost:{{api_port}}
	}
	handle /ws* {
		reverse_proxy localhost:{{api_port}}
	}
	handle {
		reverse_proxy localhost:{{web_port}}
	}
}
```

Generate configs for each project:

```bash
# Generate config for project-a
cd ~/Developer/project-a
caddy-dev generate \
    --var subdomain=project-a \
    --var port=3000

# Generate config for project-b
cd ~/Developer/project-b
caddy-dev generate \
    --var subdomain=project-b \
    --var port=4000

# Generate config for my-monorepo
cd ~/Developer/my-monorepo
caddy-dev generate \
    --var subdomain=my-monorepo \
    --var web_port=5000 \
    --var api_port=6000
```

Initialize caddy-dev to import all projects:

```bash
caddy-dev init
# Add: ~/Developer

# Reload Caddy
caddy-dev reload
```

Now access all your projects via subdomains:
- `http://project-a.localhost` → project-a on port 3000
- `http://project-b.localhost` → project-b on port 4000
- `http://my-monorepo.localhost` → my-monorepo on port 5000
- `http://my-monorepo.localhost/api` → my-monorepo on port 6000

One template, multiple projects, unified subdomain management.

## Installation

### From Source

```bash
cargo build --release
```

The binary will be available at `target/release/caddy-dev`.

### From Crates (if published)

```bash
cargo install caddy-dev
```

## Usage

caddy-dev uses a subcommand structure with three available commands:

```bash
caddy-dev <command> [options]
```

### Commands

#### generate

Generate `Caddyfile.dev` from a template file with variable substitution.

```bash
caddy-dev generate [OPTIONS]
```

**Options:**

- `-o, --output-dir <DIR>` — Output directory for Caddyfile.dev (default: current directory)
- `-t, --template <FILE>` — Path to template file (default: `<output-dir>/Caddyfile.template`)
- `--var <KEY=VALUE>` — Variable for substitution (repeatable)

**Example:**

```bash
# Generate with variables
caddy-dev generate --var domain=example.com --var port=8080

# Use custom template and output directory
caddy-dev generate -t /path/to/template -o /path/to/output
```

**Template Format:**

Use `{{key}}` placeholders in your template file:

```
{{domain}}:{{port}} {
    reverse_proxy localhost:3000
    tls internal
}
```

#### init

Interactive initialization to configure folders for importing Caddyfile.dev files.

```bash
caddy-dev init
```

This command:

1. Creates the config directory (`~/.config/caddy-dev/`)
2. Prompts for folders containing Caddyfile.dev files
3. Generates a main Caddyfile with import statements

**Examples of valid folder inputs:**

- `/path/to/project`
- `/path/to/**/Caddyfile.dev` (glob patterns)
- `~/projects/*/Caddyfile.dev` (home directory expansion)

#### reload

Reload Caddy with the generated configuration.

```bash
caddy-dev reload
```

Executes `caddy reload --config ~/.config/caddy-dev/Caddyfile`.

**Prerequisite:** Run `caddy-dev init` first to set up the configuration.

## Configuration

### Config Directory

The tool uses XDG-compliant configuration:

```
~/.config/caddy-dev/
```

This directory contains:

- `Caddyfile` — Main Caddyfile with import statements to all configured Caddyfile.dev files

### Generated Files

When you run `caddy-dev generate`:

1. Reads template file (default: `Caddyfile.template` in output directory)
2. Substitutes `{{key}}` placeholders with provided values
3. Writes result to `Caddyfile.dev` in the output directory

## Examples

### Complete Workflow

```bash
# 1. Initialize caddy-dev with your project folders
caddy-dev init

# 2. Create a template in your project
echo '{{domain}} {
    reverse_proxy localhost:3000
}' > Caddyfile.template

# 3. Generate Caddyfile.dev with variables
caddy-dev generate --var domain=localhost

# 4. Reload Caddy to apply configuration
caddy-dev reload
```

### Multiple Variables

```bash
caddy-dev generate \
    --var domain=api.example.com \
    --var port=9000 \
    --var tls=internal
```

### Different Output Directory

```bash
caddy-dev generate \
    --output-dir /path/to/project \
    --template /path/to/custom.template \
    --var environment=development
```

## Dependencies

- **clap 4.5** — Command-line argument parsing
- **dialoguer 0.11** — Interactive prompts
- **glob 0.3** — File pattern matching
- **dirs 5** — Cross-platform directory handling

## Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Format code
cargo fmt

# Run clippy
cargo clippy
```

## Error Handling

The tool follows CLI conventions:

- Errors are printed to stderr with descriptive messages
- Non-zero exit codes indicate failures
- Common errors include:
  - Missing output directory
  - Missing template file
  - Invalid variable format (must be `key=value`)
  - Missing configuration (run `caddy-dev init` first)

## License

MIT License

## Author

Wlad Paiva <me@wladpaiva.xyz>

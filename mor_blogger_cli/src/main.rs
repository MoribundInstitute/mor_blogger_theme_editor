use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::*;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

use mor_blogger_core::config::ThemeConfig;
use mor_blogger_core::presets::resolve_palette_pair;
use mor_blogger_core::render::theme::{render_theme, save_bundle_to_disk, save_xml_to_disk};

#[derive(Parser)]
#[command(
    name = "mbt",
    about = "Headless compiler for Moribund Blogger Themes",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scaffolds a new workspace.toml and directory structure
    Init {
        /// Optional directory to initialize the workspace in
        #[arg(default_value = ".")]
        path: PathBuf,

        /// The starting template (e.g., minimal, lms, wiki)
        #[arg(short, long, default_value = "minimal")]
        template: String,

        /// Overwrite existing workspace files if they exist
        #[arg(short, long)]
        force: bool,
    },
    /// Validates workspace.toml and template syntax without building
    Check {
        /// Path to the input .toml project file
        #[arg(short, long, default_value = "workspace.toml")]
        input: PathBuf,

        /// Treat warnings as hard errors
        #[arg(long)]
        strict: bool,
    },
    /// Compiles a .toml workspace file into a Blogger .xml theme
    Build {
        /// Path to the input .toml project file
        #[arg(short, long, default_value = "workspace.toml")]
        input: PathBuf,

        /// Path for the output .xml file
        #[arg(short, long, default_value = "theme.xml")]
        output: PathBuf,
    },
    /// Compiles the theme and packages it with HTML stencils into a .zip archive
    Bundle {
        /// Path to the input .toml project file
        #[arg(short, long, default_value = "workspace.toml")]
        input: PathBuf,

        /// Path for the output .zip archive
        #[arg(short, long, default_value = "theme_bundle.zip")]
        output: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let start_time = Instant::now();

    match &cli.command {
        Commands::Init { path, template, force } => {
            let target_file = path.join("workspace.toml");
            
            if target_file.exists() && !force {
                anyhow::bail!(
                    "Workspace already exists at {}. Use --force to overwrite.", 
                    target_file.display().to_string().yellow()
                );
            }

            println!("{} Initializing new '{}' workspace in {:?}...", "➔".cyan(), template, path);
            // TODO: Wire up core::scaffold::create_workspace()
            println!("{} Workspace ready.", "✔".green().bold());
        }
        Commands::Check { input, strict } => {
            println!("{} Reading configuration...", "➔".cyan());
            let config = load_config(input)?;
            
            println!("{} Rendering template to XML in memory...", "➔".cyan());
            let (light, dark) = resolve_palette_pair(None, &config);
            let xml = render_theme(&config, &light, &dark);

            println!("{} Running strict XML well-formedness check...", "➔".cyan());
            
            // This is where the magic happens. roxmltree will catch unescaped &, <, >, and unclosed tags.
            match roxmltree::Document::parse(&xml) {
                Ok(doc) => {
                    // Thin semantic layer: check for required Blogger V3 nodes
                    let has_skin = doc.descendants().any(|n| n.has_tag_name("b:skin") || n.has_tag_name("b:template-skin"));
                    
                    if !has_skin {
                        if *strict {
                            anyhow::bail!("Validation failed: Missing required <b:skin> or <b:template-skin> block.");
                        } else {
                            println!("{} Warning: No <b:skin> or <b:template-skin> block detected.", "⚠".yellow());
                        }
                    }

                    println!("{} Workspace and generated XML are valid.", "✔".green().bold());
                }
                Err(e) => {
                    // roxmltree provides exact line/column positions for syntax errors
                    anyhow::bail!(
                        "Template integrity failure! The generated XML is malformed and Blogger will reject it.\n  {}\n  Check your custom HTML modules or JavaScript CDATA wrappers.",
                        e.to_string().red()
                    );
                }
            }
        }
        Commands::Build { input, output } => {
            println!("{} Reading workspace configuration...", "➔".cyan());
            let config = load_config(input)?;

            println!("{} Resolving theme components...", "➔".cyan());
            let (light, dark) = resolve_palette_pair(None, &config);
            let xml = render_theme(&config, &light, &dark);

            save_xml_to_disk(&xml, output).map_err(|e| anyhow::anyhow!(e))?;
            
            println!(
                "{} Theme successfully compiled to {} in {:?}",
                "✔".green().bold(),
                output.display().to_string().yellow(),
                start_time.elapsed()
            );
        }
        Commands::Bundle { input, output } => {
            println!("{} Reading workspace configuration...", "➔".cyan());
            let config = load_config(input)?;

            println!("{} Resolving theme components...", "➔".cyan());
            let (light, dark) = resolve_palette_pair(None, &config);
            let xml = render_theme(&config, &light, &dark);

            println!("{} Generating static HTML stencils and archiving...", "➔".cyan());
            save_bundle_to_disk(&xml, &config.site.site_title, &config.static_pages, output)
                .map_err(|e| anyhow::anyhow!(e))?;

            println!(
                "{} Bundle successfully compiled to {} in {:?}",
                "✔".green().bold(),
                output.display().to_string().yellow(),
                start_time.elapsed()
            );
        }
    }

    Ok(())
}

/// Helper function to read and deserialize the TOML workspace cleanly
fn load_config(path: &PathBuf) -> Result<ThemeConfig> {
    let toml_str = fs::read_to_string(path)
        .with_context(|| format!("Failed to read input file at '{}'", path.display()))?;

    let config: ThemeConfig = toml::from_str(&toml_str)
        .with_context(|| format!("Failed to parse valid TOML configuration from '{}'", path.display()))?;

    Ok(config)
}
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::*;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

use mor_blogger_core::config::ThemeConfig;
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
    Init {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(short, long, default_value = "minimal")]
        template: String,
        #[arg(short, long)]
        force: bool,
    },
    Check {
        #[arg(short, long, default_value = "workspace.toml")]
        input: PathBuf,
        #[arg(long)]
        strict: bool,
    },
    Build {
        #[arg(short, long, default_value = "workspace.toml")]
        input: PathBuf,
        #[arg(short, long, default_value = "theme.xml")]
        output: PathBuf,
    },
    Bundle {
        #[arg(short, long, default_value = "workspace.toml")]
        input: PathBuf,
        #[arg(short, long, default_value = "theme_bundle.zip")]
        output: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let start_time = Instant::now();

    match &cli.command {
        Commands::Init {
            path,
            template,
            force,
        } => {
            let target_file = path.join("workspace.toml");

            if target_file.exists() && !force {
                anyhow::bail!(
                    "Workspace already exists at {}. Use --force to overwrite.",
                    target_file.display().to_string().yellow()
                );
            }

            println!(
                "{} Initializing new '{}' workspace in {:?}...",
                "➔".cyan(),
                template,
                path
            );
            println!("{} Workspace ready.", "✔".green().bold());
        }
        Commands::Check { input, strict } => {
            println!("{} Reading configuration...", "➔".cyan());
            let config = load_config(input)?;

            println!("{} Rendering template to XML in memory...", "➔".cyan());
            let xml = render_theme(&config, &std::collections::HashMap::new());

            println!("{} Running strict XML well-formedness check...", "➔".cyan());

            match roxmltree::Document::parse(&xml) {
                Ok(doc) => {
                    let has_skin = doc
                        .descendants()
                        .any(|n| n.has_tag_name("b:skin") || n.has_tag_name("b:template-skin"));

                    if !has_skin {
                        if *strict {
                            anyhow::bail!("Validation failed: Missing required <b:skin> or <b:template-skin> block.");
                        } else {
                            println!(
                                "{} Warning: No <b:skin> or <b:template-skin> block detected.",
                                "⚠".yellow()
                            );
                        }
                    }

                    println!(
                        "{} Workspace and generated XML are valid.",
                        "✔".green().bold()
                    );
                }
                Err(e) => {
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
            let xml = render_theme(&config, &std::collections::HashMap::new());

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
            let xml = render_theme(&config, &std::collections::HashMap::new());

            println!(
                "{} Generating static HTML stencils and archiving...",
                "➔".cyan()
            );
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

fn load_config(path: &PathBuf) -> Result<ThemeConfig> {
    let toml_str = fs::read_to_string(path)
        .with_context(|| format!("Failed to read input file at '{}'", path.display()))?;

    let config: ThemeConfig = toml::from_str(&toml_str).with_context(|| {
        format!(
            "Failed to parse valid TOML configuration from '{}'",
            path.display()
        )
    })?;

    Ok(config)
}

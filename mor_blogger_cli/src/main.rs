use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::*;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

use mor_blogger_core::diagnostics::check_integrity;
use mor_blogger_core::presets::theme_config_from_path;
use mor_blogger_core::render::theme::{render_theme, save_bundle_to_disk, save_xml_to_disk};
use mor_blogger_core::utils::rehydration::{
    extract_workspace_state, inject_workspace_state, load_vfs_from_dir, persist_vfs_overrides,
    write_vfs_to_dir, RehydrationPayload,
};
use roxmltree::{Document, ParsingOptions};

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
        /// Output path, or "-" to write Blogger XML to stdout.
        #[arg(short, long, default_value = "theme.xml")]
        output: PathBuf,
    },
    /// Headless render: preset TOML → template_resolver → xml_generator (no UI).
    Render {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(long, conflicts_with = "output")]
        stdout: bool,
    },
    Bundle {
        #[arg(short, long, default_value = "workspace.toml")]
        input: PathBuf,
        #[arg(short, long, default_value = "theme_bundle.zip")]
        output: PathBuf,
    },
    /// Export Blogger XML with embedded reversible workspace state (config + VFS + TOML bytes).
    Export {
        #[arg(short, long, default_value = "workspace.toml")]
        input: PathBuf,
        #[arg(short, long, default_value = "theme.xml")]
        output: PathBuf,
        /// Directory containing `.css` / `.js` VFS overrides to embed in the export.
        #[arg(long)]
        vfs_dir: Option<PathBuf>,
    },
    /// Rehydrate editor state from exported Blogger XML.
    Restore {
        #[arg(short, long)]
        input: PathBuf,
        /// Write restored TOML bytes here (defaults to `workspace.toml`).
        #[arg(short, long)]
        config_out: Option<PathBuf>,
        /// Write restored VFS overrides here (defaults to `build/restored_vfs/`).
        #[arg(long)]
        vfs_dir: Option<PathBuf>,
        /// Also persist restored VFS overrides to the OS workspace directory.
        #[arg(long)]
        persist_vfs: bool,
    },
    /// Install a local MCP plugin directory (manifest.toml + entrypoint) into editor prefs and daemon registry.
    Plugin {
        #[command(subcommand)]
        command: PluginCommands,
    },
}

#[derive(Subcommand)]
enum PluginCommands {
    /// Register a local MCP plugin without recompiling the editor workspace.
    Install {
        /// Path to plugin folder containing manifest.toml and entrypoint script/binary.
        path: PathBuf,
    },
    /// List MCP binaries and config-bridge plugin registrations.
    List,
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

            // ponytail: one template ("minimal") = the default config. Add a match
            // here when a second starter template actually exists.
            let starter = mor_blogger_core::config::defaults::default_theme_config();
            let toml_bytes = toml::to_string_pretty(&starter)
                .context("Failed to serialize starter workspace config")?;
            if let Some(parent) = target_file.parent() {
                fs::create_dir_all(parent).ok();
            }
            fs::write(&target_file, toml_bytes.as_bytes())
                .with_context(|| format!("Failed to write {}", target_file.display()))?;

            println!(
                "{} Workspace ready at {}.",
                "✔".green().bold(),
                target_file.display()
            );
        }
        Commands::Check { input, strict } => {
            println!("{} Reading configuration...", "➔".cyan());
            let config = load_config(input)?;

            println!("{} Rendering template to XML in memory...", "➔".cyan());
            let xml = render_theme(&config, &std::collections::HashMap::new());

            println!("{} Running structural integrity analyzer...", "➔".cyan());
            let diag = check_integrity(&xml, &config.template_pack);
            println!("{}", diag.format_report());

            if !diag.is_valid {
                if *strict {
                    anyhow::bail!(
                        "Template integrity failure! Fix the reported module errors before deploying to Blogger."
                    );
                }
                println!(
                    "{} Integrity errors detected (non-strict mode — render may still be malformed).",
                    "⚠".yellow()
                );
            }

            println!("{} Running strict XML well-formedness check...", "➔".cyan());

            match parse_blogger_xml(&xml) {
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

                    if diag.is_valid {
                        println!(
                            "{} Workspace and generated XML are valid.",
                            "✔".green().bold()
                        );
                    }
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
            println!("{} Reading configuration...", "➔".cyan());
            let config = load_config(input)?;

            println!("{} Resolving theme components...", "➔".cyan());
            let xml = render_theme(&config, &std::collections::HashMap::new());

            if output.as_os_str() == "-" {
                use std::io::Write;
                std::io::stdout()
                    .write_all(xml.as_bytes())
                    .context("Failed to write XML to stdout")?;
            } else {
                save_xml_to_disk(&xml, output).map_err(|e| anyhow::anyhow!(e))?;
                println!(
                    "{} Theme successfully compiled to {} in {:?}",
                    "✔".green().bold(),
                    output.display().to_string().yellow(),
                    start_time.elapsed()
                );
            }
        }
        Commands::Render {
            input,
            output,
            stdout,
        } => {
            println!(
                "{} Headless pipeline: {} → template_resolver → xml_generator",
                "➔".cyan(),
                input.display().to_string().yellow()
            );
            let config = load_config(input)?;

            println!("{} Resolving embedded template parts...", "➔".cyan());
            let xml = render_theme(&config, &std::collections::HashMap::new());

            println!("{} Validating Blogger XML...", "➔".cyan());
            parse_blogger_xml(&xml).map_err(|e| {
                anyhow::anyhow!("Generated XML is malformed: {}", e.to_string().red())
            })?;

            if *stdout
                || output.as_ref().map(|p| p.as_os_str()) == Some(std::ffi::OsStr::new("-"))
            {
                use std::io::Write;
                std::io::stdout()
                    .write_all(xml.as_bytes())
                    .context("Failed to write XML to stdout")?;
            } else {
                let out = output
                    .clone()
                    .unwrap_or_else(|| PathBuf::from("build/theme.xml"));
                if let Some(parent) = out.parent() {
                    if !parent.as_os_str().is_empty() {
                        fs::create_dir_all(parent).with_context(|| {
                            format!("Failed to create output directory '{}'", parent.display())
                        })?;
                    }
                }
                save_xml_to_disk(&xml, &out).map_err(|e| anyhow::anyhow!(e))?;
                println!(
                    "{} Valid Blogger XML written to {} ({:.1} KB) in {:?}",
                    "✔".green().bold(),
                    out.display().to_string().yellow(),
                    xml.len() as f64 / 1024.0,
                    start_time.elapsed()
                );
            }
        }
        Commands::Export {
            input,
            output,
            vfs_dir,
        } => {
            println!("{} Reading configuration...", "➔".cyan());
            let config_toml = fs::read_to_string(input)
                .with_context(|| format!("Failed to read input file at '{}'", input.display()))?;
            let config = load_config(input)?;

            let vfs = if let Some(dir) = vfs_dir {
                load_vfs_from_dir(dir).map_err(anyhow::Error::msg)?
            } else {
                std::collections::HashMap::new()
            };

            println!(
                "{} Rendering + embedding workspace state ({} VFS overrides)...",
                "➔".cyan(),
                vfs.len()
            );
            let rendered = render_theme(&config, &vfs);
            let payload = RehydrationPayload::from_config(config)
                .with_vfs(vfs)
                .with_config_toml(config_toml);
            let xml = inject_workspace_state(&rendered, &payload).map_err(anyhow::Error::msg)?;
            extract_workspace_state(&xml)
                .map_err(|e| anyhow::anyhow!("Export self-check failed: {}", e))?;

            parse_blogger_xml(&xml).map_err(|e| {
                anyhow::anyhow!("Generated XML is malformed: {}", e.to_string().red())
            })?;
            if !xml.contains("MORIBUND_THEME_STATE:") {
                anyhow::bail!("Export failed: rehydration marker missing from output XML.");
            }

            if let Some(parent) = output.parent() {
                if !parent.as_os_str().is_empty() {
                    fs::create_dir_all(parent)?;
                }
            }
            save_xml_to_disk(&xml, output).map_err(|e| anyhow::anyhow!(e))?;
            println!(
                "{} Reversible theme exported to {} ({:.1} KB) in {:?}",
                "✔".green().bold(),
                output.display().to_string().yellow(),
                xml.len() as f64 / 1024.0,
                start_time.elapsed()
            );
        }
        Commands::Restore {
            input,
            config_out,
            vfs_dir,
            persist_vfs,
        } => {
            println!(
                "{} Rehydrating workspace from {}",
                "➔".cyan(),
                input.display().to_string().yellow()
            );
            let xml = fs::read_to_string(input)
                .with_context(|| format!("Failed to read XML at '{}'", input.display()))?;
            let payload = extract_workspace_state(&xml).map_err(anyhow::Error::msg)?;

            let config_path = config_out
                .clone()
                .unwrap_or_else(|| PathBuf::from("workspace.toml"));
            let vfs_path = vfs_dir
                .clone()
                .unwrap_or_else(|| PathBuf::from("build/restored_vfs"));

            let restored_toml = if payload.config_toml.is_empty() {
                toml::to_string_pretty(&payload.config)
                    .context("Failed to serialize restored config to TOML")?
            } else {
                payload.config_toml.clone()
            };

            if let Some(parent) = config_path.parent() {
                if !parent.as_os_str().is_empty() {
                    fs::create_dir_all(parent)?;
                }
            }
            fs::write(&config_path, restored_toml.as_bytes()).with_context(|| {
                format!(
                    "Failed to write restored TOML to '{}'",
                    config_path.display()
                )
            })?;

            write_vfs_to_dir(&payload.vfs, &vfs_path).map_err(anyhow::Error::msg)?;
            if *persist_vfs {
                persist_vfs_overrides(&payload.vfs).map_err(anyhow::Error::msg)?;
            }

            println!(
                "{} Restored TOML → {} ({} bytes)",
                "✔".green().bold(),
                config_path.display().to_string().yellow(),
                restored_toml.len()
            );
            println!(
                "{} Restored VFS → {} ({} files)",
                "✔".green().bold(),
                vfs_path.display().to_string().yellow(),
                payload.vfs.len()
            );
            println!(
                "{} Rehydration complete in {:?}",
                "✔".green().bold(),
                start_time.elapsed()
            );
        }
        Commands::Plugin { command } => match command {
            PluginCommands::Install { path } => {
                println!(
                    "{} Installing local MCP plugin from {}...",
                    "➔".cyan(),
                    path.display().to_string().yellow()
                );

                let report = mor_blogger_core::utils::mcp_install::install_local_mcp_plugin(&path)
                    .map_err(|e| anyhow::anyhow!(e))?;

                println!(
                    "{} Plugin '{}' registered in config bridge",
                    "✔".green().bold(),
                    report.plugin_id
                );
                println!(
                    "  binary: {}",
                    report.binary_path.display().to_string().yellow()
                );
                println!(
                    "  daemon registry: {}",
                    report.daemon_registry.display().to_string().yellow()
                );
                println!(
                    "  editor prefs: {}",
                    report.editor_prefs.display().to_string().yellow()
                );
                if let Some(claude_path) = report.claude_config {
                    println!(
                        "  claude MCP config: {}",
                        claude_path.display().to_string().yellow()
                    );
                }

                let registry =
                    mor_blogger_core::utils::mcp_install::read_daemon_registry().map_err(|e| {
                        anyhow::anyhow!(e)
                    })?;
                if let Some(server) = registry["servers"].get(&report.plugin_id) {
                    if let Some(prompt) = server.get("system_prompt").and_then(|v| v.as_str()) {
                        println!("  system_prompt: {}", prompt);
                    }
                }

                println!(
                    "{} MCP injection complete in {:?} — restart the editor to see it in Plugin Manager.",
                    "✔".green().bold(),
                    start_time.elapsed()
                );
            }
            PluginCommands::List => {
                let binaries = mor_blogger_core::utils::mcp_install::list_installed_mcp_binaries();
                let plugin_ids =
                    mor_blogger_core::utils::mcp_install::list_registered_plugin_ids()
                        .map_err(|e| anyhow::anyhow!(e))?;
                let registry =
                    mor_blogger_core::utils::mcp_install::read_daemon_registry().map_err(|e| {
                        anyhow::anyhow!(e)
                    })?;

                println!("{} MCP plugin inventory", "➔".cyan());
                println!("  binaries ({}):", binaries.len());
                for name in binaries {
                    println!("    - {}", name);
                }
                println!("  config bridge plugins ({}):", plugin_ids.len());
                for id in plugin_ids {
                    println!("    - {}", id);
                }
                if let Some(servers) = registry.get("servers").and_then(|v| v.as_object()) {
                    println!("  daemon registry ({}):", servers.len());
                    for (key, entry) in servers {
                        let display = entry
                            .get("display_name")
                            .and_then(|v| v.as_str())
                            .unwrap_or(key);
                        println!("    - {} ({})", display, key);
                    }
                }
            }
        },
        Commands::Bundle { input, output } => {
            println!("{} Reading configuration...", "➔".cyan());
            let toml_str = fs::read_to_string(input)
                .with_context(|| format!("Failed to read input file at '{}'", input.display()))?;
            let config = load_config(input)?;

            println!("{} Resolving theme components...", "➔".cyan());
            let xml = render_theme(&config, &std::collections::HashMap::new());

            println!(
                "{} Generating static HTML stencils and archiving...",
                "➔".cyan()
            );
            save_bundle_to_disk(&xml, &toml_str, output).map_err(|e| anyhow::anyhow!(e))?;

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

fn parse_blogger_xml(xml: &str) -> Result<Document<'_>, roxmltree::Error> {
    Document::parse_with_options(
        xml,
        ParsingOptions {
            allow_dtd: true,
            ..ParsingOptions::default()
        },
    )
}

fn load_config(path: &PathBuf) -> Result<mor_blogger_core::config::ThemeConfig> {
    theme_config_from_path(path).map_err(|e| {
        anyhow::anyhow!(
            "Failed to load theme configuration from '{}': {}",
            path.display(),
            e
        )
    })
}

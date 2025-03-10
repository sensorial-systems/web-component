#![doc = include_str!("../README.md")]

use clap::Parser;
use anyhow::{Result, Context};

#[derive(Parser)]
#[command(name = "cargo-web-component", bin_name = "cargo web-component")]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands
}

#[derive(Parser)]
struct NewArgs {
    name: String,
}

#[derive(Parser)]
enum Commands {
    /// Create a new project for Dioxus.
    #[clap(name = "new")]
    New(NewArgs),
    /// Serve the project.
    #[clap(name = "serve")]
    Serve,
}

async fn process_new_command(opts: NewArgs) -> Result<()> {
    let path = std::path::Path::new(&opts.name);
    std::fs::create_dir_all(path.join("assets")).unwrap();
    std::fs::create_dir_all(path.join("src/logo")).unwrap();
    std::fs::create_dir_all(path.join("src/navigation_bar")).unwrap();
    std::fs::create_dir_all(path.join("src/parameterized_route")).unwrap();
    std::fs::write(path.join("Cargo.toml"), format!(include_str!("templates/project/Cargo.toml.template"), NAME = opts.name, VERSION = env!("CARGO_PKG_VERSION"))).unwrap();
    std::fs::write(path.join("Dioxus.toml"), format!(include_str!("templates/project/Dioxus.toml"), NAME = opts.name)).unwrap();
    std::fs::write(path.join(".gitignore"), include_str!("templates/project/.gitignore")).unwrap();
    std::fs::write(path.join("src/main.rs"), include_str!("templates/project/src/main.rs")).unwrap();
    std::fs::write(path.join("src/logo/mod.rs"), include_str!("templates/project/src/logo/mod.rs")).unwrap();
    std::fs::write(path.join("src/logo/style.css"), include_str!("templates/project/src/logo/style.css")).unwrap();
    std::fs::write(path.join("src/navigation_bar/mod.rs"), include_str!("templates/project/src/navigation_bar/mod.rs")).unwrap();
    std::fs::write(path.join("src/navigation_bar/style.css"), include_str!("templates/project/src/navigation_bar/style.css")).unwrap();
    std::fs::write(path.join("src/parameterized_route/mod.rs"), include_str!("templates/project/src/parameterized_route/mod.rs")).unwrap();
    Ok(())
}

async fn process_serve_command() -> Result<()> {
    let status = tokio::process::Command::new("dx")
        .arg("serve")
        .status()
        .await
        .context("Failed to spawn dioxus command")?;
    if !status.success() {
        anyhow::bail!("Failed to serve Dioxus project.");
    }
    Ok(())
}

pub struct BinaryDependencies;

impl BinaryDependencies {
    pub fn check(dependencies: &[&str]) -> Result<()> {
        for dependency in dependencies {
            which::which(dependency).context(format!("The \"{}\" binary is not installed.", dependency))?;
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    BinaryDependencies::check(&["dx"])?;

    let args = Args::try_parse_from(std::env::args().skip(1))?;
    match args.command {
        Commands::New(opts) => process_new_command(opts).await,
        Commands::Serve => process_serve_command().await,
    }
}

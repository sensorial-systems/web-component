#![doc = include_str!("../README.md")]

use dioxus_cli::{StructuredOutput, link, Result};
use clap::Parser;

#[derive(Parser)]
#[command(name = "cargo-web-component", bin_name = "cargo web-component")]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,

    #[command(flatten)]
    pub verbosity: dioxus_cli::Verbosity,
}

#[derive(Parser)]
struct WebComponentArgs {
    #[command(subcommand)]
    command: AllCommands,
}

#[derive(Parser)]
struct NewArgs {
    name: String,
}

#[derive(Parser)]
enum AllCommands {
    /// Create a new project for Dioxus.
    #[clap(name = "new")]
    New(NewArgs),
    
    /// Build the Dioxus project and all of its assets.
    #[clap(name = "build")]
    Build(dioxus_cli::BuildArgs),

    /// Translate a source file into Dioxus code.
    #[clap(name = "translate")]
    Translate(dioxus_cli::Translate),

    /// Build, watch & serve the Dioxus project and all of its assets.
    #[clap(name = "serve")]
    Serve(dioxus_cli::ServeArgs),

    /// Init a new project for Dioxus in the current directory (by default).
    /// Will attempt to keep your project in a good state.
    #[clap(name = "init")]
    Init(dioxus_cli::Init),

    /// Clean output artifacts.
    #[clap(name = "clean")]
    Clean(dioxus_cli::Clean),

    /// Bundle the Dioxus app into a shippable object.
    #[clap(name = "bundle")]
    Bundle(dioxus_cli::Bundle),

    /// Automatically format RSX.
    #[clap(name = "fmt")]
    Autoformat(dioxus_cli::Autoformat),

    /// Check the project for any issues.
    #[clap(name = "check")]
    Check(dioxus_cli::Check),

    /// Run the project without any hotreloading
    #[clap(name = "run")]
    Run(dioxus_cli::RunArgs),

    /// Ensure all the tooling is installed and configured correctly
    #[clap(name = "doctor")]
    Doctor(dioxus_cli::Doctor),

    /// Dioxus config file controls.
    #[clap(subcommand)]
    #[clap(name = "config")]
    Config(dioxus_cli::Config),
}


#[derive(Parser)]
enum Commands {
    #[clap(hide = true)]
    WebComponent(WebComponentArgs),
    #[clap(flatten)]
    AllCommands(AllCommands),
}

async fn process_new_command(opts: NewArgs) -> Result<StructuredOutput> {
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
    Ok(StructuredOutput::Success)
}

async fn process_all_commands(opts: AllCommands) -> Result<(), Box<dyn std::error::Error>> {
    let result = match opts {
        AllCommands::New(opts) => process_new_command(opts).await,
        AllCommands::Translate(opts) => opts.translate(),
        AllCommands::Init(opts) => opts.init(),
        AllCommands::Config(opts) => opts.config(),
        AllCommands::Autoformat(opts) => opts.autoformat(),
        AllCommands::Check(opts) => opts.check().await,
        AllCommands::Clean(opts) => opts.clean().await,
        AllCommands::Build(opts) => opts.run_cmd().await,
        AllCommands::Serve(opts) => opts.serve().await,
        AllCommands::Bundle(opts) => opts.bundle().await,
        AllCommands::Run(opts) => opts.run().await,
        AllCommands::Doctor(opts) => opts.run().await,    
    };

    // Provide a structured output for third party tools that can consume the output of the CLI
    match result {
        Ok(output) => {
            tracing::debug!(json = ?output);
        }
        Err(err) => {
            tracing::error!(
                ?err,
                json = ?StructuredOutput::Error {
                    message: format!("{err:?}"),
                },
            );

            std::process::exit(1);
        }
    };
    Ok(())

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // If we're being ran as a linker (likely from ourselves), we want to act as a linker instead.
    if let Some(link_action) = link::LinkAction::from_env() {
        link_action.run();
        std::process::exit(0);
    }

    let args = Args::parse();

    dioxus_cli::VERBOSITY.set(args.verbosity).expect("Failed to set verbosity");

    match args.command {
        Commands::AllCommands(opts) => process_all_commands(opts).await,
        Commands::WebComponent(opts) => process_all_commands(opts.command).await,
    }
}

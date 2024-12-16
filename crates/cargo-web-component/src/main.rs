use dioxus_cli::{StructuredOutput, link};
use clap::Parser;

#[derive(Parser)]
#[command(name = "cargo-web-component", bin_name = "cargo web-component")]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser)]
struct WebComponentArgs {
    #[command(subcommand)]
    command: dioxus_cli::Commands,
}

#[derive(Parser)]
enum Commands {
    #[clap(hide = true)]
    WebComponent(WebComponentArgs),
    #[clap(flatten)]
    Dioxus(dioxus_cli::Commands),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // If we're being ran as a linker (likely from ourselves), we want to act as a linker instead.
    if let Some(link_action) = link::LinkAction::from_env() {
        link_action.run();
        std::process::exit(0);
    }

    let args = Args::parse();

    let commands = match args.command {
        Commands::Dioxus(dioxus) => dioxus,
        Commands::WebComponent(dioxus) => dioxus.command,
    };

    let result = match commands {
        dioxus_cli::Commands::Translate(opts) => opts.translate(),
        dioxus_cli::Commands::New(opts) => opts.create(),
        dioxus_cli::Commands::Init(opts) => opts.init(),
        dioxus_cli::Commands::Config(opts) => opts.config(),
        dioxus_cli::Commands::Autoformat(opts) => opts.autoformat(),
        dioxus_cli::Commands::Check(opts) => opts.check().await,
        dioxus_cli::Commands::Clean(opts) => opts.clean().await,
        dioxus_cli::Commands::Build(opts) => opts.run_cmd().await,
        dioxus_cli::Commands::Serve(opts) => opts.serve().await,
        dioxus_cli::Commands::Bundle(opts) => opts.bundle().await,
        dioxus_cli::Commands::Run(opts) => opts.run().await,
        dioxus_cli::Commands::Doctor(opts) => opts.run().await,    
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

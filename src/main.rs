use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser)]
#[command(name = "confy")]
#[command(version = "0.2.0", about = "A CLI to manage environment configurations")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Merge base + environment override
    Build {
        /// Name of the environment (e.g. dev, staging, prod)
        #[arg(short, long, default_value = "dev")]
        env: String,
        /// Output file (optional). If not specified, the result will be printed to stdout
        #[arg(short, long)]
        output: Option<String>,
        /// Path to the configs directory
        #[arg(short = 'p', long, default_value = "config")]
        config_path: String,
        /// Validate the final configuration against a JSON schema
        #[arg(short = 'v', long)]
        validate: Option<bool>,
    },
    /// Compare/diff two environment configurations
    Diff {
        #[arg(short, long)]
        env1: String,
        #[arg(short, long)]
        env2: String,
        #[arg(short = 'p', long, default_value = "config")]
        config_path: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Build { env, output, config_path, validate } => {
            commands::build::handle_build(env, output, config_path, validate);
        }
        Commands::Diff { env1, env2, config_path } => {
            commands::diff::handle_diff(env1, env2, config_path);
        }
    }
}


use clap::{Parser, Subcommand};
mod modules;

#[derive(Parser)]
#[command(name = "secops", version = "1.0", about = "Async Pentest Tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(subcommand)]
    Pentest(PentestCommands),
}

#[derive(Subcommand)]
enum PentestCommands {
    Scan { target: String, #[arg(long)] range: String },
    Dns { target: String, #[arg(long)] wordlist: String },
    Dirbrute { target: String, #[arg(long)] wordlist: String, #[arg(long, default_value_t = 10)] threads: usize },
    Headers { target: String, #[arg(long)] grade: bool },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Pentest(cmd) => match cmd {
            PentestCommands::Scan { target, range } => modules::scanner::run(target, range).await?,
            PentestCommands::Dns { target, wordlist } => modules::dns::run(target, wordlist).await?,
            PentestCommands::Dirbrute { target, wordlist, threads } => modules::dirbrute::run(target, wordlist, threads).await?,
            PentestCommands::Headers { target, grade } => modules::headers::run(target, grade).await?,
        },
    }
    Ok(())
}

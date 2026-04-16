/// Ana modül ve başlangıç noktası
use clap::{Parser, Subcommand};
/// Alt modüllerin tanımlanması
mod modules;

/// CLI argümanlarını temsil eden ana yapı
#[derive(Parser)]
#[command(name = "secops", version = "1.0", about = "Async Pentest Tool")]
struct Cli {
    /// Ana komutlar
    #[command(subcommand)]
    command: Commands,
}

/// Desteklenen ana komutlar ağacı
#[derive(Subcommand)]
enum Commands {
    /// Sızma testi ve güvenlik araçları
    #[command(subcommand)]
    Pentest(PentestCommands),
}

/// Sızma testi işlemleri için alt komutlar
#[derive(Subcommand)]
enum PentestCommands {
    /// Hızlı asenkron port tarama işlemi
    Scan { target: String, #[arg(long)] range: String },
    /// Sözlük tabanlı DNS alt alan adı (subdomain) keşfi
    Dns { target: String, #[arg(long)] wordlist: String },
    /// Web dizinleri için kaba kuvvet (brute-force) taraması
    Dirbrute { target: String, #[arg(long)] wordlist: String, #[arg(long, default_value_t = 10)] threads: usize },
    /// HTTP güvenlik başlıklarının analizi ve değerlendirmesi
    Headers { target: String, #[arg(long)] grade: bool },
}

/// Uygulamanın asenkron ana giriş noktası
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Komut satırı argümanlarını ayrıştır
    let cli = Cli::parse();
    
    // Kullanıcının seçtiği modüle göre ilgili fonksiyonları tetikle
    match cli.command {
        Commands::Pentest(cmd) => match cmd {
            PentestCommands::Scan { target, range } => modules::scanner::run(target, range).await?,
            PentestCommands::Dns { target, wordlist } => modules::dns::run(target, wordlist).await?,
            PentestCommands::Dirbrute { target, wordlist, threads } => modules::dirbrute::run(target, wordlist, threads).await?,
            // Güvenlik header analizi modülünü çalıştır
            PentestCommands::Headers { target, grade } => modules::header::run(target, grade).await?,
        },
    }
    
    // Herhangi bir hata oluşmazsa başarıyla sonlandır
    Ok(())
}

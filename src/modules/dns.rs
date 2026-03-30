use hickory_resolver::TokioAsyncResolver;
use std::fs::read_to_string;
use colored::*;

pub async fn run(target: String, wordlist: String) -> anyhow::Result<()> {
    let resolver = TokioAsyncResolver::tokio_from_system_conf()?;
    let content = read_to_string(wordlist)?;

    println!("🌐 {} için DNS keşfi...", target.bold().cyan());
    let mut tasks = vec![];

    for line in content.lines() {
        let subdomain = format!("{}.{}", line, target);
        let resolver = resolver.clone();
        tasks.push(tokio::spawn(async move {
            if let Ok(res) = resolver.lookup_ip(subdomain.clone()).await {
                if let Some(ip) = res.iter().next() {
                    println!("  [+] {} -> {}", subdomain.blue(), ip.to_string().yellow());
                }
            }
        }));
    }
    futures::future::join_all(tasks).await;
    Ok(())
}

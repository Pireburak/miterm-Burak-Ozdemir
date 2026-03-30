use tokio::sync::Semaphore;
use std::sync::Arc;
use reqwest::Client;
use std::fs::read_to_string;
use colored::*;

pub async fn run(target: String, wordlist: String, threads: usize) -> anyhow::Result<()> {
    let content = read_to_string(wordlist)?;
    let client = Client::new();
    let sem = Arc::new(Semaphore::new(threads));

    println!("📁 {} dizinleri taranıyor...", target.bold().cyan());
    let mut tasks = vec![];

    for line in content.lines() {
        let url = format!("{}/{}", target.trim_end_matches('/'), line);
        let client = client.clone();
        let permit = sem.clone().acquire_owned().await?;

        tasks.push(tokio::spawn(async move {
            let _permit = permit;
            if let Ok(res) = client.get(&url).send().await {
                if res.status().is_success() {
                    println!("  [{}] {}", "200".green(), url);
                }
            }
        }));
    }
    futures::future::join_all(tasks).await;
    Ok(())
}

use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use std::sync::Arc;
use tokio::sync::Semaphore;
use colored::*;

pub async fn run(target: String, range: String) -> anyhow::Result<()> {
    let parts: Vec<&str> = range.split('-').collect();
    let start: u16 = parts[0].parse()?;
    let end: u16 = parts[1].parse()?;
    let sem = Arc::new(Semaphore::new(100));
    
    println!("🚀 {} portları taranıyor...", target.bold().cyan());
    let mut tasks = vec![];

    for port in start..=end {
        let target = target.clone();
        let permit = sem.clone().acquire_owned().await?;
        tasks.push(tokio::spawn(async move {
            let _permit = permit;
            let addr = format!("{}:{}", target, port);
            if timeout(Duration::from_millis(800), TcpStream::connect(&addr)).await.is_ok() {
                println!("  [+] Port {} {}", port, "AÇIK".green());
            }
        }));
    }
    futures::future::join_all(tasks).await;
    Ok(())
}

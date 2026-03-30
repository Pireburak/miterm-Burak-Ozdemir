use reqwest::Client;
use colored::*;

pub async fn run(target: String, grade: bool) -> anyhow::Result<()> {
    let client = Client::new();
    let res = client.get(&target).send().await?;
    let headers = res.headers();
    let checks = vec!["Content-Security-Policy", "Strict-Transport-Security", "X-Frame-Options"];

    let mut score = 0;
    for c in &checks {
        if headers.contains_key(*c) {
            println!("  [{}] {}", "✔".green(), c);
            score += 33;
        } else {
            println!("  [{}] {}", "✘".red(), c);
        }
    }

    if grade {
        let g = if score > 90 { "A+" } else if score > 60 { "B" } else { "F" };
        println!("\n⭐ Not: {}", g.bold().yellow());
    }
    Ok(())
}

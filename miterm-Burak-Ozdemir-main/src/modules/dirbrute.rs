use tokio::sync::Semaphore;
use std::sync::Arc;
use reqwest::Client;
use std::fs::read_to_string;
use colored::*;

/// Dizin kaba kuvvet taraması gerçekleştirir.
///
/// Verilen bir wordlist dosyasındaki satırları okuyarak hedef URL üzerinde
/// asenkron HTTP GET istekleri yapar. Başarılı olanları ekrana yazdırır.
///
/// * `target` - Taramanın yapılacağı hedef URL (örn: http://example.com)
/// * `wordlist` - Denenecek dizin yollarının bulunduğu metin dosyasının yolu
/// * `threads` - Aynı anda çalıştırılacak maksimum paralel istek sayısı
pub async fn run(target: String, wordlist: String, threads: usize) -> anyhow::Result<()> {
    // Wordlist dosyasının içeriğini bir string olarak belleğe al
    let content = read_to_string(wordlist)?;
    // Asenkron HTTP istemcisini oluştur
    let client = Client::new();
    // Paralel işlem sayısını sınırlamak için bir semafor oluştur
    let sem = Arc::new(Semaphore::new(threads));

    println!("📁 {} dizinleri taranıyor...", target.bold().cyan());
    
    // Asenkron görevleri (task'leri) tutacağımız vektör
    let mut tasks = vec![];

    // Wordlistteki her satır için bir asenkron görev başlat
    for line in content.lines() {
        // Hedef URL'nin sonundaki fazladan slash karakterlerini düzelt ve yolu birleştir
        let url = format!("{}/{}", target.trim_end_matches('/'), line);
        let client = client.clone();
        
        // Semafor üzerinden bir permit al (izin), limit dolduysa bekler
        let permit = sem.clone().acquire_owned().await?;

        // Yeni bir asenkron görev oluştur ve havuza ekle
        tasks.push(tokio::spawn(async move {
            // Görev bitene kadar semafor iznini elimizde tutuyoruz
            let _permit = permit;
            // HTTP GET isteğini gönder ve sonucu kontrol et
            if let Ok(res) = client.get(&url).send().await {
                // Sadece başarılı yanıt veren sayfaları (HTTP 2xx) göster
                if res.status().is_success() {
                    println!("  [{}] {}", "200".green(), url);
                }
            }
        }));
    }
    
    // Tüm asenkron görevlerin tamamlanmasını bekle
    futures::future::join_all(tasks).await;
    
    Ok(())
}

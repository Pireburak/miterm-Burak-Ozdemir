use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use std::sync::Arc;
use tokio::sync::Semaphore;
use colored::*;

/// Belirtilen IP adresi ve port aralığında TCP bağlantı testleri yaparak açık portları bulur.
///
/// Port taraması son derece asenkron ve çok kanallı çalışır. Belirtilen aralıktaki tüm portlara
/// verilen timeout süresi içerisinde bağlantı açılmaya çalışılır. Başarılı olanlar AÇIK olarak işaretlenir.
///
/// * `target` - Taramanın yapılacağı hedef IP adresi
/// * `range` - Başlangıç ve bitiş portlarını temsil eden string (örn: "1-1024")
pub async fn run(target: String, range: String) -> anyhow::Result<()> {
    // Port aralığı stringini '-' karakterine göre ayır ve bir vektör haline getir
    let parts: Vec<&str> = range.split('-').collect();
    // Başlangıç port numarasını parser yardımıyla satha çevir
    let start: u16 = parts[0].parse()?;
    // Bitiş port numarasını çevir
    let end: u16 = parts[1].parse()?;
    
    // Aynı anda denenecek bağlantı limitini semafor olarak tanımla (100 concurrent bağlantı)
    let sem = Arc::new(Semaphore::new(100));
    
    println!("🚀 {} portları taranıyor...", target.bold().cyan());
    
    // Tarama hedeflerini tutacak asenkron havuz
    let mut tasks = vec![];

    // Belirtilen aralıktaki her port için bir tokio asenkron görevi başlat
    for port in start..=end {
        let target = target.clone();
        
        // Fazla yüklenmemek için permit ile asenkron izni bekle
        let permit = sem.clone().acquire_owned().await?;
        
        // Yeni asenkron işlem arka planda başlatılıyor
        tasks.push(tokio::spawn(async move {
            let _permit = permit;
            let addr = format!("{}:{}", target, port);
            
            // 800ms'lik bir timeout vererek TCP bağlantısı açmayı dene
            // Eğer başarıyla bağlanırsa port açık demektir.
            if timeout(Duration::from_millis(800), TcpStream::connect(&addr)).await.is_ok() {
                println!("  [+] Port {} {}", port, "AÇIK".green());
            }
        }));
    }
    
    // Başlatılan tüm asenkron görevlerin sonucunu bekle
    futures::future::join_all(tasks).await;
    
    Ok(())
}

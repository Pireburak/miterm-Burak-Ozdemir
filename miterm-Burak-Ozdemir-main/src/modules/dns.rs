use hickory_resolver::TokioAsyncResolver;
use std::fs::read_to_string;
use colored::*;

/// DNS alt alan adı keşfi işlemini başlatır.
///
/// Verilen sözlük dosyasındaki kelimeleri hedef alan adının önüne ekleyerek
/// DNS çözümlemesi yapar ve geçerli olan alt alan adlarını listeler.
///
/// * `target` - Keşif yapılacak ana domain adresi
/// * `wordlist` - Alt alan adları için kullanılacak kelime dosyasının yolu
pub async fn run(target: String, wordlist: String) -> anyhow::Result<()> {
    // Sistem konfigürasyonunu kullanarak asenkron DNS çözücüyü başlat
    let resolver = TokioAsyncResolver::tokio_from_system_conf()?;
    // Dosyadaki kelimeleri oku ve belleğe al
    let content = read_to_string(wordlist)?;

    println!("🌐 {} için DNS keşfi...", target.bold().cyan());
    
    // DNS çözümleme görevlerini saklayacağımız havuz
    let mut tasks = vec![];

    // Wordlist içerisindeki her kelime için bir asenkron görev başlat
    for line in content.lines() {
        // Alt alan adı formatını oluştur (örn: dev.example.com)
        let subdomain = format!("{}.{}", line, target);
        let resolver = resolver.clone();
        
        // Asenkron parçacığı başlat
        tasks.push(tokio::spawn(async move {
            // DNS sorgusunu asenkron olarak gerçekleştir
            if let Ok(res) = resolver.lookup_ip(subdomain.clone()).await {
                // Eğer bir geçerli IP dönerse hedef aktiftir, ekrana yazdırır
                if let Some(ip) = res.iter().next() {
                    println!("  [+] {} -> {}", subdomain.blue(), ip.to_string().yellow());
                }
            }
        }));
    }
    
    // Tüm çözümleme görevlerinin aynı anda bitmesini bekle
    futures::future::join_all(tasks).await;
    
    Ok(())
}

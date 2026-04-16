use reqwest::Client;
use colored::*;

/// Hedef sunucunun HTTP güvenlik başlıklarını analiz eder.
///
/// HTTP GET isteği gönderip sunucunun yolladığı HTTP response
/// header'larını okuyarak temel güvenlik politikası (CSP), HSTS ve X-Frame gibi
/// önemli başlıkların olup olmadığını kontrol eder.
///
/// * `target` - İncelenecek web sunucusunun adresi (örn: https://example.com)
/// * `grade` - Eğer true ise başlıkların durumuna göre genel bir harf notu verir
pub async fn run(target: String, grade: bool) -> anyhow::Result<()> {
    // Asenkron HTTP istemcisi oluştur
    let client = Client::new();
    
    // Hedef adrese GET isteği gönder ve yanıtı al
    let res = client.get(&target).send().await?;
    let headers = res.headers();
    
    // Kontrol edilecek kritik güvenlik başlıklarının listesi
    let checks = vec!["Content-Security-Policy", "Strict-Transport-Security", "X-Frame-Options"];

    // Puanlama sistemi için başlangıç değeri
    let mut score = 0;
    
    // Her bir başlığı tek tek döngüyle kontrol et
    for c in &checks {
        // Eğer başlığın anahtarı dönüş yapılan header listesinde varsa
        if headers.contains_key(*c) {
            println!("  [{}] {}", "✔".green(), c);
            score += 33; // Bulunan her başlık için puan ekle
        } else {
            // Başlık yoksa hata olarak göster
            println!("  [{}] {}", "✘".red(), c);
        }
    }

    // Eğer kullanıcı `--grade` bayrağı gönderdiyse sonuçlara göre puan ver
    if grade {
        // Puana göre harf notu hesaplama
        let g = if score > 90 { "A+" } else if score > 60 { "B" } else { "F" };
        println!("\n⭐ Not: {}", g.bold().yellow());
    }
    
    Ok(())
}

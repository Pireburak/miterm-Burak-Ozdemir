# miterm-Burak-Ozdemir
Paralel Tarama (Aynsc Tokio)
🛡️ SecOps Pentest Suite
SecOps, Rust ve Tokio runtime kullanılarak geliştirilmiş, yüksek performanslı ve asenkron bir siber güvenlik tarama setidir. Bu araç, modern ağ tarama ihtiyaçlarını karşılamak için minimum kaynak tüketimi ve maksimum hız prensibiyle tasarlanmıştır.

✨ Özellikler
⚡ Hızlı Port Tarama: Binlerce portu Semaphore kontrolü ile sistem kaynaklarını yormadan saniyeler içinde tarar.

🌐 DNS Keşfi (Subdomain): Wordlist desteğiyle aktif alt alan adlarını asenkron olarak tespit eder.

🔒 SSL & HTTP Başlık Analizi: Web sitelerinin güvenlik yapılandırmalarını (HSTS, CSP vb.) denetler ve güvenlik notu (A-F) atar.

📁 DirBrute: Web dizinlerini worker-pool mantığıyla kaba kuvvet (brute-force) yöntemiyle haritalandırır.

🛠️ Kurulum
Sisteminizde Rust kurulu olmalıdır:

Bash
git clone https://github.com/KULLANICI_ADIN/pentester.git
cd pentester
cargo build --release
🚀 Kullanım
Tüm komutlar secops pentest ön ekiyle çalışır:

1. Port Tarama
Bash
cargo run -- pentest scan 127.0.0.1 --range 1-1000
2. DNS Subdomain Keşfi
Bash
cargo run -- pentest dns example.com --wordlist subdomains.txt
3. HTTP Güvenlik Başlıkları
Bash
cargo run -- pentest headers https://target.com --grade
4. Dizin Tarama
Bash
cargo run -- pentest dirbrute https://target.com --wordlist common.txt --threads 20
🏗️ Mimari Yapı
Bu proje, asenkron G/Ç (I/O) işlemlerini yönetmek için Tokio kütüphanesini kullanır. Geleneksel tarayıcıların aksine, her bir istek ayrı bir işletim sistemi iş parçacığı (thread) yerine hafif bir Tokio Task olarak çalışır.

Güvenlik: Rust'ın sahiplik (ownership) modeli sayesinde bellek sızıntıları önlenir.

Performans: FuturesUnordered ve Semaphore ile ağ trafiği optimize edilir.

📄 Lisans
Bu proje MIT Lisansı altında lisanslanmıştır. Daha fazla bilgi için LICENSE dosyasına bakınız.

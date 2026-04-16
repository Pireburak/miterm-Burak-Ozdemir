<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/tr/b/be/%C4%B0stinye_%C3%9Cniversitesi_logosu.png" alt="İstinye Logosu" width="200"/>

  <h1>Gelişmiş Ağ Tarama Aracı (Network Scanner)</h1>

  <p>
    <img src="https://img.shields.io/badge/Rust-1.70+-orange.svg" alt="Rust Version">
    <img src="https://img.shields.io/badge/Lisans-MIT-green.svg" alt="License">
    <img src="https://img.shields.io/badge/Durum-Aktif-success.svg" alt="Build Status">
  </p>
</div>

## Proje Hakkında
Bu proje, yerel veya uzak ağlardaki açık portları ve aktif cihazları tespit etmek amacıyla geliştirilmiş bir ağ tarama (network scanning) aracıdır. Sistem yöneticileri ve güvenlik araştırmacıları için ağ topolojisini anlamak ve potansiyel güvenlik zafiyetlerini (açık portlar üzerinden) analiz etmek için temel bir araç niteliği taşır. 

**Danışman:** Dr. Öğr. Üyesi Ahmet Yılmaz

## İçindekiler
- [Proje Hakkında](#proje-hakkında)
- [Özellikler](#özellikler)
- [Kurulum ve Çalıştırma](#kurulum-ve-çalıştırma)
- [Docker ile Kullanım](#docker-ile-kullanım)
- [Proje Mimarisi](#proje-mimarisi)
- [Lisans](#lisans)

## Özellikler
* **Hızlı Port Tarama:** Belirlenen IP adresi veya IP aralığı üzerinde çoklu iş parçacığı (multithreading) kullanarak hızlı tarama.
* **Banner Grabbing:** Açık portlarda çalışan servislerin versiyon bilgilerini tespit etme.
* **Kolay Konfigürasyon:** Çevresel değişkenler (`.env`) üzerinden parametrik yapılandırma.
* **Kapsamlı Raporlama:** Tarama sonuçlarını JSON formatında dışa aktarma yeteneği.

## Kurulum ve Çalıştırma
Projeyi yerel ortamınızda çalıştırmak için aşağıdaki adımları izleyin:

1. Depoyu klonlayın:
   ```bash
   git clone [https://github.com/kullaniciadi/ag-taramasi-projesi.git](https://github.com/kullaniciadi/ag-taramasi-projesi.git)
   cd ag-taramasi-projesi

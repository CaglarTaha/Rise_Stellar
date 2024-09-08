
```markdown

#PROJE TEKNİK SEBEPLERDEN ÖTÜRÜ TAMAMLANAMAMIŞTIR 

#Stellar Fullstack Bootcamp - Turkey

Bu proje, Stellar ağı üzerinde XLM ödemeleri ve kısa mesajlar gönderimi yapabilen basit bir sistemdir. Kullanıcılar, Stellar ağı aracılığıyla XLM transferi yapabilir ve bu transferlere kısa mesajlar ekleyebilirler. Ayrıca, cüzdan bakiyelerini sorgulama imkanı sunar.

## Özellikler

- XLM transferi yapma
- Transfer işlemlerine kısa mesaj ekleme
- Cüzdan bakiyesini sorgulama

## Gereksinimler

- Rust (1.65.0 veya üzeri)
- `cargo` ve `rustc` komutları
-Stellar CLI

## Kurulum

1. **Proje Bağımlılıklarını Kurun:**

   Projeyi yerel sisteminizde çalıştırmadan önce, gerekli bağımlılıkları yüklemeniz gerekmektedir. Bu işlemi gerçekleştirmek için terminalde şu komutu çalıştırın:

   ```sh
   cargo build
   ```

2. **Ortam Değişkenlerini Ayarlayın:**

   `.env` dosyasını proje kök dizininde oluşturun ve gerekli ortam değişkenlerini aşağıdaki gibi ayarlayın:

   ```env
   SECRET_SEED=Your_Secret_Seed
   RECEIVER_ADDRESS=Receiver_Address
   ACCOUNT_ID=Your_Account_ID
   ```

   Burada `SECRET_SEED`, Stellar cüzdanınızın gizli anahtarıdır; `RECEIVER_ADDRESS`, XLM gönderilecek cüzdan adresidir; `ACCOUNT_ID` ise bakiyesini sorgulamak istediğiniz Stellar hesabının kimliğidir.

## Kullanım

Projenin ana fonksiyonları `main.rs` dosyasında yer almaktadır. Çalıştırmak için terminalde şu komutu kullanabilirsiniz:

```sh
cargo build
cargo run
```

Bu komut, aşağıdaki işlemleri gerçekleştirir:

1. **Cüzdan Oluşturma:** Yeni bir Stellar cüzdanı oluşturur ve public/secret anahtarlarını ekrana yazdırır.
2. **Bakiye Sorgulama:** Belirtilen hesap kimliği ile Stellar ağı üzerinden bakiye bilgilerini çeker.
3. **Ödeme Gönderme:** Belirtilen gizli anahtar ve alıcı adresi ile Stellar ağı üzerinden XLM transferi yapar ve kısa bir mesaj ekler.

## Kod Yapısı

- `src/main.rs`
- `src/services/stellar_api.rs`
- `src/services/utils.rs`
- `Cargo.toml`
- `.env`



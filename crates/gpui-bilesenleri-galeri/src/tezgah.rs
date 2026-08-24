//! Bileşen tezgâhının bileşen-bağımsız kabuğu.
//!
//! Tezgâh `BİL-010`'a ait bir ekran değildir: token, yüz, yerleşim ve profil
//! burada durur, bileşene özgü olan her şey `TezgahProfili` uygulayıcısına
//! aittir. `BİL-010` bu kabuğun ilk profilidir; yeni bir aile yeniden
//! yazıldığında kendi profilini verir ve kabuğun çizimi değişmez.
//!
//! Faz durumu (`raporlar/BILESEN_TEZGAHI_YENI_TASARIM_GOC_PLANI.md`):
//!
//! - **F0a:** `tokenlar` — dört tema kipi ve semantik renk rolleri.
//! - **F1 yapısal akış · burada:** `arayuz` (kabuk/profil sınırı), `yerlesim`
//!   (kip ve akış dağıtımı), `govde` (çizim).
//! - **F0b · burada:** `profil` — `ORT-017` tipli görünüm profili. Ölçü ve
//!   tipografi rolü profilde tek sahipli; `TextStyle` yalnız `çöz()`
//!   katmanında doğar. Köşe yarıçapı `ORT-003 KutuŞekliTercihi`nden çözülür.
//!   Kayıt kapısı (`anatomi_kaydet`) hâlâ kapalıdır: `GörünümKayıtDefteri`nin
//!   somut uygulayıcısı yok.

pub mod arayuz;
pub mod govde;
pub mod kabuk;
pub mod profil;
pub mod tokenlar;
pub mod yerlesim;
pub mod yuzler;

pub use arayuz::*;
pub use govde::*;
pub use kabuk::*;
pub use profil::*;
pub use tokenlar::*;
pub use yerlesim::*;
pub use yuzler::*;

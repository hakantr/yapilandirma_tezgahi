//! Tarayıcı köprüleri ve platform bildirimleri.
//!
//! Bütün JS köprüleri burada toplanır; başlatıcı yalnız pencereyi açar.
//! Sarmalayıcının işi bildirimi okumaktır — öncelik sırası ve düşme
//! politikası her portun kendi çekirdek çözümündedir.

#[cfg(target_family = "wasm")]
use gpui_bilesenleri_galeri::{
    GizlilikKapılıYetenek, GmtFarkı, OtomatikDoldurmaAmacı, OtomatikDoldurmaHatası,
    PlatformMetinİmleciTercihi, PlatformOtomatikDoldurmaPortu, PlatformSaatDilimiPortu,
    PlatformİmleçPortu, PlatformİzinDurumu, SaatDilimiKaynağı, UnicodeMetinMotoru,
    ÇözülmüşSaatDilimi,
};
#[cfg(target_family = "wasm")]
use std::sync::Arc;
#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
extern "C" {
    /// `Intl` kimliği; tarayıcı bildirmezse boş dizi döner.
    #[wasm_bindgen(js_namespace = globalThis, js_name = gpuiSaatDilimiKimligi)]
    fn dilim_kimliği() -> String;

    /// `Date.getTimezoneOffset()` dakikası. JS işareti terstir: UTC+3 için
    /// `-180` döner.
    #[wasm_bindgen(js_namespace = globalThis, js_name = gpuiSaatDilimiFarki)]
    fn dilim_farkı() -> f64;

    /// `matchMedia("(prefers-reduced-motion: reduce)").matches`
    #[wasm_bindgen(js_namespace = globalThis, js_name = gpuiHareketAzaltilsin)]
    fn hareket_azaltılsın() -> bool;

    /// Sayfanın açılış aşamalarını konağa bildirir.
    #[wasm_bindgen(js_namespace = globalThis, js_name = gpuiGaleriDurumu)]
    pub fn galeri_durumu(aşama: &str, ayrıntı: &str);
}

#[cfg(target_family = "wasm")]
pub struct TarayıcıSaatDilimi {
    /// `ORT-002` doğrulama kapısı: `Intl` dizesi kimliğe ancak kayıt
    /// yolundan çevrilir, port kimlik mühürlemez.
    motor: Arc<UnicodeMetinMotoru>,
}

#[cfg(target_family = "wasm")]
impl TarayıcıSaatDilimi {
    pub fn yeni(motor: Arc<UnicodeMetinMotoru>) -> Self {
        Self { motor }
    }
}

#[cfg(target_family = "wasm")]
impl PlatformSaatDilimiPortu for TarayıcıSaatDilimi {
    fn dilim(&self) -> Option<ÇözülmüşSaatDilimi> {
        // JS `getTimezoneOffset` yerelden UTC'ye farkı verir; işaret
        // çevrilmezse bütün dilimler ters çıkar.
        let dakika = -dilim_farkı();
        if !dakika.is_finite() {
            return None;
        }
        let gmt_farkı = GmtFarkı(dakika as i16);
        if !gmt_farkı.geçerli_mi() {
            return None;
        }
        let ad = dilim_kimliği();
        Some(ÇözülmüşSaatDilimi {
            // Tanınmayan dize kimlik olarak bildirilmez; çözüm GMT
            // farkıyla sürer.
            kimlik: self.motor.saat_dilimi(ad.trim()).ok(),
            gmt_farkı,
            kaynak: SaatDilimiKaynağı::Platform,
        })
    }
}

/// Tarayıcılar imleç yanıp sönme hızını bildirmez; standart bir API yok.
/// Bildirebildikleri tek ilgili tercih `prefers-reduced-motion`. Uydurulmuş
/// bir hız bildirmek, ölçülmemiş bir değeri platform tercihi gibi göstermek
/// olurdu; o yüzden yalnız bu tercih taşınıyor.
#[cfg(target_family = "wasm")]
pub struct TarayıcıİmleciTercihi;

#[cfg(target_family = "wasm")]
impl PlatformİmleçPortu for TarayıcıİmleciTercihi {
    fn metin_imleci_tercihi(&self) -> PlatformMetinİmleciTercihi {
        // `prefers-reduced-motion` açıkken imleç sabittir. Kapalıyken
        // **bildirilmedi** döner, "yanıp sönsün" değil: tarayıcı bir dönem
        // bildirmiyor ve uydurulmuş bir süre platform tercihi gibi
        // görünürdü. `Bildirilmedi`de çözüm temanın değerinde kalır.
        if hareket_azaltılsın() {
            PlatformMetinİmleciTercihi::Sabit
        } else {
            PlatformMetinİmleciTercihi::Bildirilmedi
        }
    }
}

/// Yerel derlemede bu modülün tarayıcı girişi olmadığını bildirir.
#[cfg(not(target_family = "wasm"))]
pub const fn yalnız_wasm_hedefidir() -> bool {
    true
}

/// `§25` tarayıcı otomatik doldurma yeteneği.
///
/// Tarayıcı otomatik doldurmayı gerçek bir `<input>` üzerindeki
/// `autocomplete` bildirimiyle sunar. GPUI web tuvale çiziyor ama IME için
/// gizli bir `<input>` de açıyor; niyet oraya yazılır ve tarayıcı kendi
/// önerisini o alana getirir.
///
/// Girdi bulunamazsa yetenek **kullanılamaz** bildirilir. `autocomplete`
/// için tarayıcı izin sormaz, bu yüzden izin durumu `Gerekmiyor`dur —
/// `ORT-019` kanonik kapısı bunu `Verildi` ile birlikte kabul eder.
#[cfg(target_family = "wasm")]
pub struct TarayıcıOtomatikDoldurma;

#[cfg(target_family = "wasm")]
impl TarayıcıOtomatikDoldurma {
    /// GPUI web'in IME için açtığı gizli girdi.
    ///
    /// Tuval metin almadığı için sayfadaki tek `input` odur; başka bir
    /// işaret (kimlik, sınıf) `gpui_web` tarafından verilmiyor.
    fn gizli_girdi() -> Option<web_sys::HtmlInputElement> {
        use wasm_bindgen::JsCast as _;
        web_sys::window()?
            .document()?
            .query_selector("input")
            .ok()??
            .dyn_into::<web_sys::HtmlInputElement>()
            .ok()
    }

    /// `OtomatikDoldurmaAmacı` → WHATWG `autocomplete` jetonu.
    fn jeton(amaç: OtomatikDoldurmaAmacı) -> &'static str {
        match amaç {
            OtomatikDoldurmaAmacı::Ad => "name",
            OtomatikDoldurmaAmacı::KullanıcıAdı => "username",
            OtomatikDoldurmaAmacı::YeniParola => "new-password",
            OtomatikDoldurmaAmacı::GeçerliParola => "current-password",
            OtomatikDoldurmaAmacı::TekKullanımlıkKod => "one-time-code",
            OtomatikDoldurmaAmacı::EPosta => "email",
            OtomatikDoldurmaAmacı::Telefon => "tel",
            OtomatikDoldurmaAmacı::AdresSatırı => "address-line1",
            OtomatikDoldurmaAmacı::Kuruluş => "organization",
        }
    }
}

#[cfg(target_family = "wasm")]
impl PlatformOtomatikDoldurmaPortu for TarayıcıOtomatikDoldurma {
    fn yetenek(&self, _: &gpui::App) -> GizlilikKapılıYetenek {
        GizlilikKapılıYetenek {
            kullanılabilir: Self::gizli_girdi().is_some(),
            izin: PlatformİzinDurumu::Gerekmiyor,
            geçici_oturum: false,
            sürüm: 0,
        }
    }

    fn içerik_amacını_uygula(
        &self,
        amaç: OtomatikDoldurmaAmacı,
        _: &mut gpui::Window,
        _: &mut gpui::App,
    ) -> Result<(), OtomatikDoldurmaHatası> {
        let girdi = Self::gizli_girdi().ok_or(OtomatikDoldurmaHatası::Desteklenmiyor)?;
        // `type` değiştirilmez: gizli girdi aynı zamanda IME köprüsüdür ve
        // türünü değiştirmek yazımı bozar. `autocomplete` standart kancadır
        // ve tarayıcı önerisini onunla çözer.
        girdi
            .set_attribute("autocomplete", Self::jeton(amaç))
            .map_err(|_| OtomatikDoldurmaHatası::Desteklenmiyor)?;
        girdi
            .set_attribute("name", Self::jeton(amaç))
            .map_err(|_| OtomatikDoldurmaHatası::Desteklenmiyor)?;
        Ok(())
    }
}

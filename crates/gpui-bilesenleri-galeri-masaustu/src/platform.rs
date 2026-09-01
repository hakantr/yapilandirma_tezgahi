//! Masaüstü platform bildirimleri.
//!
//! Sarmalayıcının tek işi bildirimi okumaktır; öncelik sırası ve düşme
//! politikası her portun kendi çekirdek çözümündedir.

use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use gpui_bilesenleri_galeri::{
    GizlilikKapılıYetenek, GmtFarkı, OtomatikDoldurmaAmacı, OtomatikDoldurmaHatası,
    PlatformMetinİmleciTercihi, PlatformOtomatikDoldurmaPortu, PlatformSaatDilimiPortu,
    PlatformİmleçPortu, PlatformİzinDurumu, SaatDilimiKaynağı, SaatDilimiKimliği,
    UnicodeMetinMotoru, ÇözülmüşSaatDilimi,
};

/// `§25` masaüstü otomatik doldurma yeteneği.
///
/// İşletim sistemi düzeyinde otomatik doldurma (macOS Anahtar Zinciri,
/// Windows kimlik yöneticisi) yerel metin alanının `contentType` bildirimine
/// dayanır. GPUI kendi tuvaline çiziyor ve böyle bir alan açmıyor; bu yüzden
/// yetenek **kullanılamaz** bildirilir.
///
/// Bu bir yer tutucu değil, dürüst bir bildirimdir: "denedik olmadı" diye
/// sessizce geçmek ürünün özelliği çalışıyor sanmasına yol açardı. Yetenek
/// kapalıyken alan niyet üretmez ve `Desteklenmiyor` döner.
pub struct SistemOtomatikDoldurma;

impl PlatformOtomatikDoldurmaPortu for SistemOtomatikDoldurma {
    fn yetenek(&self, _: &gpui::App) -> GizlilikKapılıYetenek {
        GizlilikKapılıYetenek {
            kullanılabilir: false,
            izin: PlatformİzinDurumu::Gerekmiyor,
            geçici_oturum: false,
            sürüm: 0,
        }
    }

    fn içerik_amacını_uygula(
        &self,
        _: OtomatikDoldurmaAmacı,
        _: &mut gpui::Window,
        _: &mut gpui::App,
    ) -> Result<(), OtomatikDoldurmaHatası> {
        Err(OtomatikDoldurmaHatası::Desteklenmiyor)
    }
}

/// Bildirim tazelik penceresi: dilim çözümü senkron alt süreç (`date`) ve
/// dosya sistemi okuduğu için her çağrıda koşamaz — bu port kanonik çözüm
/// tarafından tuş vuruşu başına bile sorgulanabiliyor. Dilim değişimi
/// (seyahat, elle ayar) saniyeler mertebesinde nadir bir olaydır; pencere
/// dolana kadar son bildirim geçerli sayılır.
const DİLİM_TAZELİK_PENCERESİ: Duration = Duration::from_secs(60);

pub struct SistemSaatDilimi {
    /// `ORT-002` doğrulama kapısı: platform dizesi kimliğe ancak kayıt
    /// yolundan çevrilir, port kimlik mühürlemez.
    motor: Arc<UnicodeMetinMotoru>,
    /// Son bildirim ve okunduğu an; tazelik penceresi içinde yeniden
    /// süreç doğurulmaz.
    önbellek: Mutex<Option<(Instant, Option<ÇözülmüşSaatDilimi>)>>,
}

impl SistemSaatDilimi {
    pub fn yeni(motor: Arc<UnicodeMetinMotoru>) -> Self {
        Self {
            motor,
            önbellek: Mutex::new(None),
        }
    }

    fn oku(&self) -> Option<ÇözülmüşSaatDilimi> {
        let kimlik = iana_kimliği(&self.motor);
        let fark = gmt_farkı()?;
        Some(ÇözülmüşSaatDilimi {
            kimlik,
            gmt_farkı: fark,
            kaynak: SaatDilimiKaynağı::Platform,
        })
    }
}

impl PlatformSaatDilimiPortu for SistemSaatDilimi {
    fn dilim(&self) -> Option<ÇözülmüşSaatDilimi> {
        let mut önbellek = self
            .önbellek
            .lock()
            .expect("dilim önbellek kilidi zehirlenmemeli");
        if let Some((an, bildirim)) = önbellek.as_ref()
            && an.elapsed() < DİLİM_TAZELİK_PENCERESİ
        {
            return bildirim.clone();
        }
        let bildirim = self.oku();
        *önbellek = Some((Instant::now(), bildirim.clone()));
        bildirim
    }
}

/// İşletim sisteminin IANA kimliği.
///
/// `TZ` ortam değişkeni **açık niyettir** ve tek başına belirleyicidir:
/// tanınmayan bir `TZ` sistem dilimine düşülmez, kimlik bildirilmez ve
/// çözüm GMT farkıyla sürer. `TZ` hiç yokken macOS ve Linux'ta
/// `/etc/localtime` bağının hedefi kimliği taşır.
fn iana_kimliği(motor: &UnicodeMetinMotoru) -> Option<SaatDilimiKimliği> {
    if let Ok(tz) = std::env::var("TZ")
        && !tz.trim().is_empty()
    {
        return motor.saat_dilimi(tz.trim()).ok();
    }
    let hedef = std::fs::read_link("/etc/localtime").ok()?;
    let metin = hedef.to_str()?;
    // `.../zoneinfo/Europe/Istanbul` → `Europe/Istanbul`
    let (_, kimlik) = metin.split_once("zoneinfo/")?;
    motor.saat_dilimi(kimlik).ok()
}

/// UTC'ye göre dakika farkı.
///
/// `date +%z` yerel saatin o anki farkını verir; yaz saati uygulanmışsa
/// uygulanmış hâlini bildirir. Ayrı bir tarih kitaplığı eklemek yerine
/// platformun kendi bildirimini okuyoruz.
fn gmt_farkı() -> Option<GmtFarkı> {
    let çıktı = std::process::Command::new("date")
        .arg("+%z")
        .output()
        .ok()?;
    if !çıktı.status.success() {
        return None;
    }
    farkı_ayrıştır(std::str::from_utf8(&çıktı.stdout).ok()?.trim())
}

/// `+0300` biçimindeki farkı dakikaya çevirir.
fn farkı_ayrıştır(metin: &str) -> Option<GmtFarkı> {
    let (işaret, rakamlar) = metin.split_at(metin.char_indices().nth(1)?.0);
    let çarpan = match işaret {
        "+" => 1,
        "-" => -1,
        _ => return None,
    };
    if rakamlar.len() != 4 || !rakamlar.chars().all(|k| k.is_ascii_digit()) {
        return None;
    }
    let saat: i16 = rakamlar[..2].parse().ok()?;
    let dakika: i16 = rakamlar[2..].parse().ok()?;
    let fark = GmtFarkı(çarpan * (saat * 60 + dakika));
    fark.geçerli_mi().then_some(fark)
}

#[cfg(test)]
mod testler {
    use super::*;

    #[test]
    fn fark_metni_dakikaya_cevrilir() {
        assert_eq!(farkı_ayrıştır("+0300"), Some(GmtFarkı(180)));
        assert_eq!(farkı_ayrıştır("-0530"), Some(GmtFarkı(-330)));
        assert_eq!(farkı_ayrıştır("+0545"), Some(GmtFarkı(345)));
        assert_eq!(farkı_ayrıştır("+0000"), Some(GmtFarkı(0)));
        // Aralık dışı ve bozuk girdi bildirilmez; çekirdek yedeğe düşer.
        assert_eq!(farkı_ayrıştır("+1500"), None);
        assert_eq!(farkı_ayrıştır("0300"), None);
        assert_eq!(farkı_ayrıştır("+03"), None);
    }
}

/// Masaüstü imleç tercihi bildirimi.
///
/// Bildirim **kuruluşta bir kez** okunur: kanonik `BİL-010` bu portu her
/// prepaint karesinde sorgular ve `defaults` alt süreci kare başına ~10 ms
/// ana-iş-parçacığı maliyetiyle bütün kare ölçümlerini kirletiyordu.
/// macOS imleç yanıp sönme tercihleri oturum içinde pratikte değişmez;
/// canlı takip bilinçli olarak süreç ömrüne feda edildi (değişiklik yeniden
/// başlatmayla alınır). Okuma pencere açılmadan, kuruluşta yapılır.
pub struct SistemİmleciTercihi {
    tercih: PlatformMetinİmleciTercihi,
}

impl SistemİmleciTercihi {
    pub fn yeni() -> Self {
        Self {
            tercih: tercihi_oku(),
        }
    }
}

fn tercihi_oku() -> PlatformMetinİmleciTercihi {
    // macOS iki ayrı süre tutar; biri bile tanımlıysa kullanıcı
    // varsayılanı değiştirmiş demektir. Hiçbiri yoksa platform sessizdir
    // ve bu "sabit" demek değildir.
    let açık = süreyi_oku("NSTextInsertionPointBlinkPeriodOn");
    let kapalı = süreyi_oku("NSTextInsertionPointBlinkPeriodOff");
    if açık.is_none() && kapalı.is_none() {
        return PlatformMetinİmleciTercihi::Bildirilmedi;
    }
    // macOS'un belgeli varsayılan fazı; eksik anahtar bu değerle
    // tamamlanır. Çözüm politikası yine `ORT-004` mühürlü kapısındadır,
    // bu yalnız yarım bildirimin diğer yarısıdır.
    const VARSAYILAN_FAZ: Duration = Duration::from_millis(530);
    let açık = açık.unwrap_or(VARSAYILAN_FAZ);
    let kapalı = kapalı.unwrap_or(VARSAYILAN_FAZ);
    // Sıfır süre "yanıp sönme" demektir: kullanıcı imleci sabitlemiş.
    if açık.is_zero() || kapalı.is_zero() {
        return PlatformMetinİmleciTercihi::Sabit;
    }
    PlatformMetinİmleciTercihi::YanıpSönen {
        dönem: açık.saturating_add(kapalı),
        görünür_süre: açık,
    }
}

impl PlatformİmleçPortu for SistemİmleciTercihi {
    fn metin_imleci_tercihi(&self) -> PlatformMetinİmleciTercihi {
        self.tercih
    }
}

/// Kullanıcı varsayılanlarından milisaniye okur.
///
/// Anahtar tanımlı değilse `defaults` sıfırdan farklı bir durumla döner ve
/// `None` üretilir; bu "kullanıcı dokunmamış" demektir, hata değil.
fn süreyi_oku(anahtar: &str) -> Option<Duration> {
    let çıktı = std::process::Command::new("defaults")
        .args(["read", "-g", anahtar])
        .output()
        .ok()?;
    if !çıktı.status.success() {
        return None;
    }
    let metin = std::str::from_utf8(&çıktı.stdout).ok()?.trim();
    let milisaniye: f64 = metin.parse().ok()?;
    // Aşırı değerler kullanılabilir bir imleç üretmez; bildirim yok sayılır.
    (0.0..=10_000.0)
        .contains(&milisaniye)
        .then(|| Duration::from_millis(milisaniye as u64))
}

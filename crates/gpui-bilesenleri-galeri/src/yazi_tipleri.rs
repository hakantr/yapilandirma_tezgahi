//! Galerinin kendi yazı tipi kaydı.
//!
//! GPUI iki hedefte farklı yazı tipi kaynağı kullanır: masaüstü işletim
//! sisteminin kurulu ailelerini okur, WASM ise tarayıcı sistem listesini
//! vermediği için yalnız `add_fonts` ile eklenmiş yüzleri görür. Bu yüzden
//! galeri kendi yüzlerini iki hedefte de kaydeder; aksi hâlde masaüstünde
//! çalışan bir aile seçimi WASM'de sessizce düşer ve "aynı zemin, aynı
//! davranış" kuralı bozulur.
//!
//! Yüzler derleme zamanında gömülür: çalışma anında ağ erişimi yoktur.
//! Hepsi SIL Open Font License altındadır; lisans metinleri
//! `varliklar/yazi_tipleri/*/OFL.txt` içindedir.

use std::borrow::Cow;

use gpui::App;

/// Kitaplığın garanti ettiği yazı tipi yüzleri.
///
/// `IBM Plex Sans` ve `Lilex` `../gpui` WASM yapısında zaten gömülüdür ama
/// masaüstünde değildir; ikisini de burada kaydetmek iki hedefi eşitler.
///
/// Hepsi statik yüzdür, değişken (`[wght]`) sürüm değil: fontdb değişken bir
/// yüzü tek ağırlıkla (öntanımlı örnek) kaydeder, eksen değerini uygulamaz.
/// Değişken sürüm kullanılırsa kalın seçimi eşleşecek yüz bulamaz ve sessizce
/// düz yüze düşer — galeride ölçtüğümüz durum tam olarak buydu.
///
/// Her aile üç ağırlık taşır: `İnce` (Light 300), düz (Regular 400) ve `Koyu`
/// (SemiBold 600; Lilex'te Bold 700). Tasarımın yazı biçimi grubu üçünü de
/// düğme olarak sunuyor; ağırlığın yüzü yoksa düğme sessizce hiçbir şey
/// yapmaz.
const YÜZLER: &[&[u8]] = &[
    include_bytes!("../../../varliklar/yazi_tipleri/ibm-plex-sans/IBMPlexSans-Regular.ttf"),
    include_bytes!("../../../varliklar/yazi_tipleri/ibm-plex-sans/IBMPlexSans-Italic.ttf"),
    include_bytes!("../../../varliklar/yazi_tipleri/ibm-plex-sans/IBMPlexSans-SemiBold.ttf"),
    include_bytes!("../../../varliklar/yazi_tipleri/ibm-plex-sans/IBMPlexSans-SemiBoldItalic.ttf"),
    include_bytes!("../../../varliklar/yazi_tipleri/ibm-plex-sans/IBMPlexSans-Light.ttf"),
    include_bytes!("../../../varliklar/yazi_tipleri/ibm-plex-sans/IBMPlexSans-LightItalic.ttf"),
    include_bytes!("../../../varliklar/yazi_tipleri/ibm-plex-mono/IBMPlexMono-Regular.ttf"),
    include_bytes!("../../../varliklar/yazi_tipleri/ibm-plex-mono/IBMPlexMono-Italic.ttf"),
    include_bytes!("../../../varliklar/yazi_tipleri/ibm-plex-mono/IBMPlexMono-SemiBold.ttf"),
    include_bytes!("../../../varliklar/yazi_tipleri/ibm-plex-mono/IBMPlexMono-SemiBoldItalic.ttf"),
    include_bytes!("../../../varliklar/yazi_tipleri/ibm-plex-mono/IBMPlexMono-Light.ttf"),
    include_bytes!("../../../varliklar/yazi_tipleri/ibm-plex-mono/IBMPlexMono-LightItalic.ttf"),
    include_bytes!("../../../varliklar/yazi_tipleri/lilex/Lilex-Regular.ttf"),
    include_bytes!("../../../varliklar/yazi_tipleri/lilex/Lilex-Italic.ttf"),
    include_bytes!("../../../varliklar/yazi_tipleri/lilex/Lilex-Bold.ttf"),
    include_bytes!("../../../varliklar/yazi_tipleri/lilex/Lilex-BoldItalic.ttf"),
    include_bytes!("../../../varliklar/yazi_tipleri/lilex/Lilex-Light.ttf"),
    include_bytes!("../../../varliklar/yazi_tipleri/lilex/Lilex-LightItalic.ttf"),
    include_bytes!("../../../varliklar/yazi_tipleri/inter/Inter-Regular.ttf"),
    include_bytes!("../../../varliklar/yazi_tipleri/inter/Inter-Italic.ttf"),
    include_bytes!("../../../varliklar/yazi_tipleri/inter/Inter-SemiBold.ttf"),
    include_bytes!("../../../varliklar/yazi_tipleri/inter/Inter-SemiBoldItalic.ttf"),
    include_bytes!("../../../varliklar/yazi_tipleri/inter/Inter-Light.ttf"),
    include_bytes!("../../../varliklar/yazi_tipleri/inter/Inter-LightItalic.ttf"),
    include_bytes!("../../../varliklar/yazi_tipleri/source-serif/SourceSerif4-Regular.otf"),
    include_bytes!("../../../varliklar/yazi_tipleri/source-serif/SourceSerif4-Italic.otf"),
    include_bytes!("../../../varliklar/yazi_tipleri/source-serif/SourceSerif4-SemiBold.otf"),
    include_bytes!("../../../varliklar/yazi_tipleri/source-serif/SourceSerif4-SemiBoldItalic.otf"),
    include_bytes!("../../../varliklar/yazi_tipleri/source-serif/SourceSerif4-Light.otf"),
    include_bytes!("../../../varliklar/yazi_tipleri/source-serif/SourceSerif4-LightItalic.otf"),
];

/// Kitaplık yüzlerini yazı tipi sistemine kaydeder.
///
/// Başlatıcılar pencere açmadan önce çağırır. Kayıt başarısız olursa hata
/// yutulmaz: eksik yüz, çalışmayan bir yazı tipi tercihi demektir ve bunun
/// sessizce geçmesi tam da kaçınmak istediğimiz durumdur.
pub fn galeri_yazı_tiplerini_kur(bağlam: &mut App) -> gpui::Result<()> {
    bağlam
        .text_system()
        .add_fonts(YÜZLER.iter().map(|yüz| Cow::Borrowed(*yüz)).collect())
}

/// Kayıtlı yüzlerin sayısı; kanıt testleri için.
pub fn kayıtlı_yüz_sayısı() -> usize {
    YÜZLER.len()
}

/// Gömülü yüz baytları; kanıt testleri için.
pub fn gömülü_yüzler() -> &'static [&'static [u8]] {
    YÜZLER
}

/// GPUI'nin `all_font_names()` sonucuna kendiliğinden kattığı yedek adlar.
///
/// `TextSystem::all_font_names` platform listesine yedek yığınını ve
/// `.SystemUIFont` adını ekler. Bu adlar kurulu olduklarını göstermez:
/// yığın sabittir ve her hedefte aynı gelir. WASM'de hiçbiri çözülmez, yani
/// listede bırakılırlarsa seçildiklerinde sessizce yedeğe düşerler — tam da
/// göstermemek istediğimiz "çalışmayan tercih" durumu.
///
/// Bedeli: macOS'ta gerçekten kurulu olan `Helvetica` ve `Arial` de listeden
/// düşer. Yüzlerce aile içinde iki ad kaybetmek, çözülmeyen ad göstermeye
/// yeğdir.
const YEDEK_ADLAR: [&str; 9] = [
    "Helvetica",
    "Segoe UI",
    "Ubuntu",
    "Adwaita Sans",
    "Cantarell",
    "Noto Sans",
    "DejaVu Sans",
    "Arial",
    ".SystemUIFont",
];

/// Bir aile adı kullanıcıya gösterilmeli mi?
///
/// Nokta ile başlayanlar iç adlardır (`.ZedSans` gibi) ve bir kullanıcı
/// seçimi değildir; yedek adlar ise kurulu olduklarını göstermez.
pub fn aile_gösterilebilir_mi(ad: &str) -> bool {
    !ad.starts_with('.') && !YEDEK_ADLAR.contains(&ad)
}

/// Kitaplığın garanti ettiği aile adları.
///
/// Bu adlar iki hedefte de çözülür. Listeye eklenen her ad için `YÜZLER`
/// içinde o aileye ait bir yüz bulunmalıdır.
pub const KİTAPLIK_AİLELERİ: [&str; 5] = [
    "IBM Plex Sans",
    "IBM Plex Mono",
    "Lilex",
    "Inter",
    "Source Serif 4",
];

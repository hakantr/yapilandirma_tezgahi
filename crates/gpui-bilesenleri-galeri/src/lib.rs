//! Masaüstü ve WASM'in paylaştığı bileşen galerisi çekirdeği.
//!
//! Galeri yalnız kanonik API'leri tüketir; davranış türü veya varsayılanı
//! tanımlamaz. Uyum kiti galeriyle aynı kanonik katmanın ayrı tüketicisidir;
//! test desteği WASM çalışma profiline transitif olarak giremez.

#![allow(non_ascii_idents)]

use gpui::{
    Context, Entity, IntoElement, Render, ScrollHandle, Window, div, point, prelude::*, px, rgb,
};
use gpui_bilesenleri::{
    BileşenKimliği, EksikGirişPolitikası, GirişKutusu, GirişMaskesi, GirişYapılandırması,
    MetinGirişMaskesi, RakamKümesi, Sabitİçerik, SayaçYapılandırması, SayımBirimi, TanımKimliği,
    TarihGirişMaskesi, UzunlukSınırı, UzunlukSınırıDavranışı, YardımcıEylemTürü,
    YardımcıEylemYuvası, medya_fallback_planı, ÖrnekKimliğiFabrikası, İçerikGörünürlüğü,
};
use std::{cell::Cell, rc::Rc, sync::Arc};

mod galeri;
mod metin_girisi_profili;
mod metin_girisi_tezgahi;
// `ORT-002`/`ORT-021` uygulama-kökü hizmet sahipliği. Kök burada bir kez
// kurulur; bileşenler yalnız verilen capability değerlerini tüketir.
mod metin_hizmetleri;
#[cfg(feature = "olcum-izleyici")]
mod minimal_giris_olcumu;
mod onboarding;
mod palet;
// Alanı gözleyen panel entity'leri: kök alanın durum değişimini dinlemez,
// bu paneller dinler (performans mimarisi turu, Ağu 2026).
mod paneller;
mod sergiler;
mod simgeler;
// Bileşen-bağımsız tezgâh kabuğu. `BİL-010` onun ilk profilidir; kabuk
// hiçbir `BİL-*` tipini tanımaz.
pub mod tezgah;
mod yazi_tipleri;
// Sarmalayıcılar kanonik katmana yalnız galeri üzerinden erişir (katman
// sınırı). `ORT-002 §5.2` saat dilimi portu sarmalayıcıda uygulanacağı için
// yüzeyi buradan geçer.
pub use gpui_bilesenleri::{
    GizlilikKapılıYetenek, GmtFarkı, MetinİmleciHareketKaynağı, MetinİmleciHareketi,
    MetinİmleciÇözümHatası, MetinİmleciÇözümleyicisi, OtomatikDoldurmaAmacı,
    OtomatikDoldurmaHatası, PlatformMetinİmleciTercihi, PlatformOtomatikDoldurmaPortu,
    PlatformSaatDilimiPortu, PlatformİmleçPortu, PlatformİzinDurumu, SaatDilimiKaynağı,
    SaatDilimiKimliği, SaatDilimiTercihi, TemaMetinİmleciAdayı, metin_imleci_çözümleyicisi,
    saat_dilimini_çöz, ÇözülmüşMetinİmleciHareketi, ÇözülmüşSaatDilimi, İmleçTokenları,
};

// Sarmalayıcı platform portları `SaatDilimiKimliği` gibi kimlikleri elle
// mühürleyemez; doğrulama kapısı `ORT-002` motorudur ve katman sınırı gereği
// galeri üzerinden dışa açılır.
pub use gpui_bilesenleri_temel::UnicodeMetinMotoru;

pub use galeri::*;
pub use metin_girisi_profili::*;
pub use metin_girisi_tezgahi::*;
pub(crate) use metin_hizmetleri::{
    MetinHizmetleriKökü, SaatDilimiSeçenekleri, TezgahÇözümKaydı, TezgahİletiÇözücüsü,
    YerelKökHatası, son_çözüm_hatası_metni, yerel_kök_hatası_metni,
};
#[cfg(feature = "olcum-izleyici")]
pub use minimal_giris_olcumu::*;
pub use onboarding::*;
pub use palet::*;
pub use paneller::*;
pub use simgeler::*;
pub use tezgah::*;
pub use yazi_tipleri::*;

/// Galerinin tükettiği kanonik bileşenlerin tuş bağlarını kaydeder.
///
/// Başlatıcılar pencere açmadan önce çağırır. Galeri kendi tuş yolu
/// tanımlamaz; yalnız kanonik bileşenin kaydını iletir.
pub fn bileşen_tuş_bağlarını_kur(bağlam: &mut gpui::App) {
    gpui_bilesenleri::tuş_bağlarını_kur(bağlam);
}

fn galeri_bileşen_kimliği(
    fabrika: &ÖrnekKimliğiFabrikası,
    ad_alanı: &'static str,
    yerel_ad: &'static str,
) -> BileşenKimliği {
    let tanım = match TanımKimliği::denetimli(Arc::from(ad_alanı), Arc::from(yerel_ad)) {
        Ok(tanım) => tanım,
        Err(_) => unreachable!("galeri sabit bileşen tanımı geçerlidir"),
    };
    let örnek = match fabrika.sonraki() {
        Ok(örnek) => örnek,
        Err(_) => panic!("galeri örnek kimliği üretim soyu tükendi"),
    };
    BileşenKimliği { tanım, örnek }
}

/// `§23.1` galerinin ürün eylemi kimliği.
///
/// Ürün eyleminin `EylemKimliği`ni ürün verir; galeri burada ürün rolünü
/// oynuyor ve kendi ad alanında tek bir kararlı kimlik kullanıyor.
pub fn ürün_eylem_kimliği() -> gpui_bilesenleri::EylemKimliği {
    let tanım = gpui_bilesenleri::TanımKimliği::denetimli(
        std::sync::Arc::from("galeri"),
        std::sync::Arc::from("tezgah-urun-eylemi"),
    )
    .expect("galeri ürün eylemi tanımı geçerlidir");
    gpui_bilesenleri::EylemKimliği(tanım)
}

#[derive(Clone)]
struct YardımcıKimlikleri {
    temizle: BileşenKimliği,
    parolayı_göster: BileşenKimliği,
    aramayı_başlat: BileşenKimliği,
    seçiciyi_aç: BileşenKimliği,
    /// `§23.1` ürünün kendi eylemi.
    ///
    /// Yerleşik dördün kimliği kanonikte sabit; ürün eyleminin kimliğini
    /// ürün verir. Galeri burada bir ürün rolü oynuyor.
    ürün: BileşenKimliği,
}

impl YardımcıKimlikleri {
    fn yeni(fabrika: &ÖrnekKimliğiFabrikası) -> Self {
        Self {
            temizle: galeri_bileşen_kimliği(fabrika, "BİL-010/YardımcıEylem", "Temizle"),
            parolayı_göster: galeri_bileşen_kimliği(
                fabrika,
                "BİL-010/YardımcıEylem",
                "ParolayıGöster",
            ),
            aramayı_başlat: galeri_bileşen_kimliği(
                fabrika,
                "BİL-010/YardımcıEylem",
                "AramayıBaşlat",
            ),
            seçiciyi_aç: galeri_bileşen_kimliği(fabrika, "BİL-010/YardımcıEylem", "SeçiciyiAç"),
            ürün: galeri_bileşen_kimliği(fabrika, "BİL-010/YardımcıEylem", "ÜrünEylemi"),
        }
    }

    fn al(&self, tür: &YardımcıEylemTürü) -> BileşenKimliği {
        match tür {
            YardımcıEylemTürü::Temizle => self.temizle.clone(),
            YardımcıEylemTürü::ParolayıGöster => self.parolayı_göster.clone(),
            YardımcıEylemTürü::AramayıBaşlat => self.aramayı_başlat.clone(),
            YardımcıEylemTürü::SeçiciyiAç => self.seçiciyi_aç.clone(),
            YardımcıEylemTürü::Ürün(_) => self.ürün.clone(),
        }
    }
}

/// Sarmalayıcının kurduğu platform bildirimleri.
///
/// Galeri bunları yalnız tüketir; hiçbirinin politikası burada değildir.
#[derive(Default)]
pub struct PlatformPortları {
    /// `ORT-002 §5.2` saat dilimi bildirimi.
    pub saat_dilimi: Option<Arc<dyn PlatformSaatDilimiPortu>>,
    /// `ORT-004` imleç tercihi bildirimi.
    pub imleç: Option<Arc<dyn PlatformİmleçPortu>>,
    /// `§25`/`ORT-019` otomatik doldurma yeteneği.
    pub otomatik_doldurma: Option<Arc<dyn gpui_bilesenleri::PlatformOtomatikDoldurmaPortu>>,
}

/// Tezgâhın dış doğrulama **gösterim beslemesi**: sabit bir sonuç döner.
///
/// Gerçek sunucu değildir ve kart bunu açıkça yazar. Sonucun sürümü
/// isteğin sürümüne eşitlenir ki `ORT-007` commit kapısından geçsin;
/// bayatlık gösterimin konusu değildir.
struct GösterimDoğrulamaPortu(Vec<gpui_bilesenleri::GeçerlilikSorunu>);

impl gpui_bilesenleri::EşzamansızDoğrulamaPortu for GösterimDoğrulamaPortu {
    fn doğrula(
        &self,
        _metin: String,
        değer_sürümü: u64,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Vec<gpui_bilesenleri::GeçerlilikSorunu>> + Send>,
    > {
        let mut sorunlar = self.0.clone();
        for sorun in &mut sorunlar {
            sorun.değer_sürümü = değer_sürümü;
        }
        Box::pin(async move { sorunlar })
    }
}

/// Masaüstü ve WASM'in aynı katalog ve bilgi mimarisiyle açtığı galeri.
/// `§30` tercih→kutu eşitlemesinin **kalıcı** ret kaydı.
///
/// Geçici olan yalnız `CompositionEtkin`dir; o buraya girmez, bekleyen
/// kümeyle taşınır (`BİL-010 ≥23.0` ile bu ret her zaman **yaşayan** bir
/// kompozisyonu anlatır: birleşimin her bitiş yolu — `unmark_text` de,
/// `insertText`-commit de — kompozisyon değerini düşürür, asılı eksen
/// kalmaz). Diğer her ret bu kalıcı typed kanala düşer. Buradaki kayıt
/// exact typed hatayı ve hangi tercih kutusunda üretildiğini taşır;
/// önizleme tanı satırı çizer, aynı kutunun sonraki başarılı eşitlemesi
/// kaydı düşürür.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TercihEşitlemeKaydı {
    pub(crate) kutu: gpui::EntityId,
    pub(crate) hata: gpui_bilesenleri::GirişHatası,
}

/// `§30` kalıcı eşitleme retinin exact sunum satırı; varyant adı korunur.
pub(crate) fn tercih_eşitleme_hatası_metni(kayıt: &TercihEşitlemeKaydı) -> String {
    format!(
        "Tercih kutusu eşitlemesi kalıcı retle düştü: ‹{:?}›",
        kayıt.hata
    )
}

/// Atomik yerel-bağlam inişi retinin exact sunum satırı; varyant adı
/// korunur. Alan eski (tutarlı) bağlamda kalmıştır.
pub(crate) fn yerel_uygulama_hatası_metni(hata: &gpui_bilesenleri::GirişHatası) -> String {
    format!("Yerel bağlam alana uygulanamadı, alan eski bağlamda: ‹{hata:?}›")
}

pub struct GaleriUygulaması {
    pub model: GaleriModeli,
    kimlik_fabrikası: ÖrnekKimliğiFabrikası,
    /// `ORT-002`/`ORT-021` uygulama-kökü metin hizmetleri.
    ///
    /// Unicode hizmet kökü, yaşayan yerel bağlam, katalog kütüğü ve mühürlü
    /// çözüm hizmeti uygulama kökünde **bir kez** kurulur; alanlar bileşen
    /// başına kök kurmaz.
    metin_hizmetleri: MetinHizmetleriKökü,
    /// `§29` tezgâh alanının **kur anındaki** uyarı raporu
    /// (`GirişKuruluşSonucu::rapor`). Çalışma-anı yeniden yapılandırmanın
    /// rapor otoritesi bu kopya değildir: yaşayan alan kendi
    /// `yapılandırma_raporu`nu taşır ve `§29` doğrulama kartı raporu
    /// tercihten güncel üretir.
    tezgah_kuruluş_raporu: Option<gpui_bilesenleri::GirişYapılandırmaRaporu>,
    /// `§14` kuruluş sırasındaki varsayılan **sağlayıcı** hatası; entity'yi
    /// öldürmez, exact typed olarak burada taşınır.
    tezgah_varsayılan_değer_hatası: Option<gpui_bilesenleri::VarsayılanDeğerHatası>,
    /// `§29` tezgâh alanı kuruluş başarısızlığı. Expect/panic ile yutulmaz;
    /// önizleme bu exact sonucu çizer, tercih değişince yeniden denenir.
    tezgah_kuruluş_hatası: Option<gpui_bilesenleri::GirişKuruluşHatası>,
    /// Sergi/tercih kutuları kuruluş başarısızlığı; exact typed taşınır.
    sergi_kuruluş_hatası: Option<gpui_bilesenleri::GirişKuruluşHatası>,
    /// Yerel kök yenilemesinin terminal akıbeti (sürüm ekseni tükenmesi).
    /// Sessiz doyurma yoktur; ret exact typed olarak burada durur,
    /// [`Self::yerel_kök_hatası`] ile okunur ve saat dilimi kartında çizilir.
    yerel_kök_hatası: Option<YerelKökHatası>,
    /// Canlı UI çözüm yolunun typed-akıbet yuvası.
    ///
    /// `TezgahİletiÇözücüsü::çöz` sunum dizesini üretmeden **önce** son
    /// hatayı buraya yazar; payload dizeye indirgenmeden kökte korunur ve
    /// [`Self::son_ileti_çözüm_hatası`] ile okunur.
    ileti_çözüm_hata_kaydı: Rc<std::cell::RefCell<Option<TezgahÇözümKaydı>>>,
    /// `§30` dış-yazım reddiyle ertelenen tercih→kutu eşitlemeleri.
    ///
    /// Kümedeki kimlik, tercihin hedef metnini o kutuya henüz yazamadığı
    /// bir tercih kutusudur; buraya giren tek akıbet `CompositionEtkin`dir
    /// — yaşayan IME birleşimi dış yazımla bozulamaz ve birleşimin her
    /// bitiş yolu (`unmark_text` de, `insertText`-commit de) kompozisyon
    /// eksenini kapattığı için erteleme sınırlıdır (diğer her ret kalıcı
    /// typed kanala gider). Bekleyen kutunun metin olayı tercihe **geri
    /// yazılmaz** — kullanıcının seçtiği hedefi birleşim metni ezemez;
    /// olay önce ileri eşitlemeyi yeniden dener, hedef uygulanınca kayıt
    /// düşer. Hedefin kendisi saklanmaz: ileri eşitleme hedefi her
    /// denemede yaşayan tercihten türetir, bayat kopya oluşmaz.
    bekleyen_tercih_eşitlemeleri: std::collections::HashSet<gpui::EntityId>,
    /// `§30` tercih eşitlemesinin son **kalıcı** reddi (varsa), exact typed.
    ///
    /// `CompositionEtkin` dışındaki retler — örn. terminal `SürümTükendi` —
    /// bekleyen kayda dönüşmez ve metin olaylarını bastırmaz; akıbetleri
    /// burada durur, [`Self::tercih_eşitleme_hatası`] ile okunur ve
    /// önizleme tanı satırında çizilir.
    tercih_eşitleme_hatası: Option<TercihEşitlemeKaydı>,
    /// Atomik yerel-bağlam inişinin (`yerel_bağlamı_değiştir`) son typed
    /// reddi (varsa) — ör. yeni yerelde maskenin yeniden uygulanamaması.
    ///
    /// Ret alanı **eski (tutarlı)** bağlamda bırakır; akıbet burada durur,
    /// önizleme tanı satırında çizilir ve uygulama hatası yaşarken her
    /// eşitleme turu inişi yeniden dener — başarıda kayıt düşer.
    yerel_uygulama_hatası: Option<gpui_bilesenleri::GirişHatası>,
    orta_kaydırma: ScrollHandle,
    /// Tezgâh sol alt bloğunun sıradan flex-scroll tutamacı.
    tezgah_sol_kaydırma: ScrollHandle,
    /// `BİL-010` yaşayan giriş alanları. Galeri metin düzenlemeyi kendisi
    /// uygulamaz; kanonik bileşeni tüketir.
    sergi_girişleri: Option<MetinGirişiAlanları>,
    /// `BİL-010` tezgâhının tercihleri ve yaşayan önizleme alanı.
    tezgah: TezgahTercihleri,
    tezgah_alanı: Option<Entity<GirişKutusu>>,
    tezgah_yardımcı_kimlikleri: Option<YardımcıKimlikleri>,
    /// Açık olan yüzer tercih kutusu. Aynı anda yalnız biri açıktır.
    /// Köşe yarıçapı kaydırma çubuğunun izinin ekrandaki yeri.
    ///
    /// Tıklama konumundan değer türetmek için izin sınırları gerekir. Çizim
    /// sırasında `canvas` ile yakalanır; `Cell` olması bildirim üretmeden
    /// yazılabilmesi içindir — yerleşim sırasında bildirim döngü kurar.
    köşe_izi: Rc<Cell<gpui::Bounds<gpui::Pixels>>>,
    /// Köşe çubuğunun tutamağı basılı mı?
    ///
    /// Tıklama tek adım atar; sürüklemek için basılı kalma durumu tutulur ve
    /// fare hareketi bu bayrak açıkken değere çevrilir.
    köşe_sürükleniyor: bool,
    /// İşletim sisteminde kurulu yazı tipi aileleri.
    ///
    /// Sayım pahalıdır (macOS'ta CoreText tüm aileleri tarar) ve çizim her
    /// karede koşar; bu yüzden bir kez okunup saklanır.
    sistem_yazı_aileleri: Option<Rc<Vec<String>>>,
    /// `ORT-002 §5.2` platform dilim bildirimi.
    ///
    /// Sarmalayıcı kurar; galeri politikayı değil yalnız bildirimi tüketir.
    saat_dilimi_portu: Option<Arc<dyn PlatformSaatDilimiPortu>>,
    /// `ORT-004` platform imleç bildirimi.
    imleç_portu: Option<Arc<dyn PlatformİmleçPortu>>,
    /// `§25` otomatik doldurma portu; alanlara kuruluşta verilir.
    otomatik_doldurma_portu: Option<Arc<dyn gpui_bilesenleri::PlatformOtomatikDoldurmaPortu>>,
    /// Pencerenin tamamına uygulanan tema ve kip.
    ///
    /// `ORT-004` renk değerinin sahibi temadır; galeri kendi kabuğunu da
    /// bir temadan çözer ki tema yönetiminin çalıştığı görülebilsin.
    /// Uygulama tezgâh ekranıyla mı açılıyor?
    ///
    /// Bugün her zaman `true`: sözleşmesi biten tek kamusal bileşen
    /// `BİL-010` ve tezgâh onun ekranı. Alan bir bayrak olarak duruyor
    /// çünkü galeri kataloğunun kodu hâlâ derleniyor ve testleri koşuyor;
    /// `YÖN-006 §3` revizyonu gelene kadar o yol silinmez.
    tezgah_ekranı: bool,
    /// Açık olan üst şerit seçicisi.
    ///
    /// Tasarımın `<select>` karşılığı. Bu bir yüzer panel **değil**: liste
    /// akışın içinde açılır ve `ORT-006` konağına bağlı değil. Aynı anda tek
    /// seçici açık kalır — iki liste birden açıksa hangisinin seçimi
    /// uygulanacağı belirsizleşir.
    açık_seçici: Option<gpui::SharedString>,
    /// Alanı gözleyen panel entity'leri (`C` türetilmiş durumlar, `§13/§19`
    /// değer üçlüsü, `§26` olay akışı).
    ///
    /// Kök alanın durum değişimini dinlemez: alan okuyan kartlar kendi
    /// entity'lerinde yaşar ve alan bildirdiğinde yalnız onlar kirlenir.
    /// Alanla birlikte kurulur, tür değişiminde alana yeniden bağlanır.
    tezgah_panelleri: Option<TezgahPanelleri>,
    /// Tercih her değiştiğinde artar; rapor ve kod önbelleğinin anahtarı.
    tercih_sürümü: u64,
    /// `§29` doğrulama raporu, tercih sürümüne bağlı.
    ///
    /// Rapor yalnız tercihten türer. Her çizimde yeniden kurmak doğrulamayı
    /// ve kimlik fabrikasını kare hızında çalıştırıyordu.
    rapor_önbelleği: Option<(u64, Rc<gpui_bilesenleri::GirişYapılandırmaRaporu>)>,
    /// Kod paneli metni, tercih sürümüne bağlı; yalnız `A` bölümünü taşır.
    kod_önbelleği: Option<(u64, gpui::SharedString)>,
    /// Çözülmüş tasarım görünümü, tema sürümüne bağlı.
    ///
    /// Çözüm yalnız temaya (ve temayla birlikte sürümü artan palete) göre
    /// değişir; kare dikişi onu her karede yeniden çözmez.
    görünüm_önbelleği: Option<(u64, std::sync::Arc<ÇözülmüşTezgahGörünümü>)>,
    /// Bu kökün penceresi; hedefli geçersizleme için saklanır.
    ///
    /// `refresh_windows()` bütün pencereleri yeniler — tezgâhta tek pencere
    /// olduğu için bugün sonuç aynıdır, ama niyet "kolonu tazele" iken
    /// bütün uygulamayı yenilemek hedefli değildir ve ileride açılacak
    /// başka pencerelerin önbelleklerini de kırardı. Tutamaç ilk çizimde
    /// yakalanır.
    pencere_tutamacı: Option<gpui::AnyWindowHandle>,
    galeri_teması: GaleriTeması,
    galeri_kipi: gpui_bilesenleri::TemaKipi,
    sergi_düğme_sayacı: u32,
    sergi_seçimi: u8,
    sergi_onaylı: bool,
    sergi_sekmesi: u8,
    sergi_paneli_açık: bool,
    sergi_araç_taşması_açık: bool,
    sergi_modali_açık: bool,
    sergi_seçici_sonucu: u8,
    sergi_tablo_azalan: bool,
    sergi_bildirimi_açık: bool,
    sergi_form_gönderildi: bool,
    sergi_sürekli_değer: u8,
    sergi_ilerleme: u8,
    sergi_takvim_günü: u8,
    sergi_disclosure_açık: bool,
    sergi_renk_seçimi: u8,
    sergi_aktarım: u8,
    sergi_arama_eşleşmesi: u8,
    sergi_kısayol_değiştirildi: bool,
    sergi_ayar_koyu: bool,
    sergi_bağlantı_başarılı: bool,
    sergi_kod_satırı: u8,
    sergi_yüzen_grup_açık: bool,
    sergi_gezinme_hedefi: u8,
    sergi_görsel_konumu: u8,
    sergi_kod_sembolü_qr: bool,
    sergi_medya_niyeti: bool,
    sergi_ort_durumları: u32,
    sergi_kab_durumları: u16,
    dar_aile_listesi_açık: bool,
}

impl GaleriUygulaması {
    pub fn yeni() -> Self {
        Self::hedef(GaleriHedefi::Masaüstü)
    }

    pub fn wasm() -> Self {
        Self::hedef(GaleriHedefi::Wasm)
    }

    pub fn hedef(hedef: GaleriHedefi) -> Self {
        let kimlik_fabrikası = match ÖrnekKimliğiFabrikası::yeni_süreç_kapsamı() {
            Ok(fabrika) => fabrika,
            Err(_) => panic!("galeri örnek kimliği üretim soyu kurulamıyor"),
        };
        // Uygulama kökü hizmetleri **burada bir kez** kurulur. Açılışta
        // platform portu henüz bağlı değildir; dilim tercihten portsuz
        // çözülür, port kurulunca `platform_portlarını_kur` kökü eşitler.
        let tezgah = TezgahTercihleri::default();
        let açılış_dilimi = saat_dilimini_çöz(&tezgah.saat_dilimi_tercihi, None);
        let metin_hizmetleri =
            MetinHizmetleriKökü::kur(&kimlik_fabrikası, açılış_dilimi.kimlik.as_ref());
        Self {
            model: GaleriModeli::yerleşik_hedef(hedef),
            kimlik_fabrikası,
            metin_hizmetleri,
            tezgah_kuruluş_raporu: None,
            tezgah_varsayılan_değer_hatası: None,
            tezgah_kuruluş_hatası: None,
            sergi_kuruluş_hatası: None,
            yerel_kök_hatası: None,
            ileti_çözüm_hata_kaydı: Rc::new(std::cell::RefCell::new(None)),
            bekleyen_tercih_eşitlemeleri: std::collections::HashSet::new(),
            tercih_eşitleme_hatası: None,
            yerel_uygulama_hatası: None,
            orta_kaydırma: ScrollHandle::new(),
            tezgah_sol_kaydırma: ScrollHandle::new(),
            sergi_girişleri: None,
            tezgah: TezgahTercihleri::default(),
            tezgah_alanı: None,
            tezgah_yardımcı_kimlikleri: None,
            köşe_izi: Rc::new(Cell::new(gpui::Bounds::default())),
            köşe_sürükleniyor: false,
            sistem_yazı_aileleri: None,
            saat_dilimi_portu: None,
            imleç_portu: None,
            otomatik_doldurma_portu: None,
            // İki hedef iki farklı temayla açılır: aynı çekirdeğin farklı
            // temalarla nasıl göründüğü tek bakışta karşılaştırılabilsin.
            tezgah_ekranı: true,
            açık_seçici: None,
            tezgah_panelleri: None,
            tercih_sürümü: 0,
            rapor_önbelleği: None,
            kod_önbelleği: None,
            görünüm_önbelleği: None,
            pencere_tutamacı: None,
            galeri_teması: match hedef {
                GaleriHedefi::Masaüstü => GaleriTeması::Kağıt,
                GaleriHedefi::Wasm => GaleriTeması::Mürekkep,
            },
            galeri_kipi: gpui_bilesenleri::TemaKipi::Açık,
            sergi_düğme_sayacı: 0,
            sergi_seçimi: 0,
            sergi_onaylı: false,
            sergi_sekmesi: 0,
            sergi_paneli_açık: true,
            sergi_araç_taşması_açık: false,
            sergi_modali_açık: false,
            sergi_seçici_sonucu: 0,
            sergi_tablo_azalan: false,
            sergi_bildirimi_açık: false,
            sergi_form_gönderildi: false,
            sergi_sürekli_değer: 40,
            sergi_ilerleme: 35,
            sergi_takvim_günü: 12,
            sergi_disclosure_açık: true,
            sergi_renk_seçimi: 0,
            sergi_aktarım: 25,
            sergi_arama_eşleşmesi: 0,
            sergi_kısayol_değiştirildi: false,
            sergi_ayar_koyu: false,
            sergi_bağlantı_başarılı: false,
            sergi_kod_satırı: 1,
            sergi_yüzen_grup_açık: false,
            sergi_gezinme_hedefi: 0,
            sergi_görsel_konumu: 0,
            sergi_kod_sembolü_qr: true,
            sergi_medya_niyeti: false,
            sergi_ort_durumları: 0,
            sergi_kab_durumları: 0,
            dar_aile_listesi_açık: false,
        }
    }
}

impl GaleriUygulaması {
    /// Tezgâh ekranını çizer.
    ///
    /// Galeri kabuğundan tümüyle ayrıdır: ne ağaç menü, ne kategori
    /// başlıkları, ne aile kartları. Tasarımın `§5` kökü budur.
    fn tezgah_ekranını_çiz(
        &mut self,
        pencere: &mut Window,
        bağlam: &mut Context<Self>,
    ) -> gpui::Div {
        // Gövde içeriği test erişim noktasıyla **aynı** yoldan üretilir;
        // iki kopya bir süre yan yana yaşadı ve sessizce ayrışıyordu.
        let içerik = self.tezgah_profil_içeriği(pencere, bağlam);
        let kabuk = sergiler::TezgahKabukDurumu {
            tema: self.galeri_teması,
            kip: self.galeri_kipi,
            hedef: self.model.hedef,
        };
        let sistem_aileleri = self.sistem_ailelerini_al(bağlam);
        // `YÖN-006.ACC-008`: kabuk başlıkları ham dizeden değil, `ORT-021`
        // kök-kapsamlı hizmetinden çözülür.
        let çözücü = self.tezgah_çözücüsü();
        // Tercih yalnız okunur: klonlanmaz, doğrudan ödünç verilir.
        gpui::div().size_full().child(sergiler::tezgah_ekranı(
            &self.tezgah,
            içerik,
            sistem_aileleri,
            kabuk,
            çözücü,
            bağlam,
        ))
    }

    /// Galeri temasını değiştirir ve tezgâh temasını da tazeler.
    pub fn galeri_temasını_seç(&mut self, tema: GaleriTeması, bağlam: &mut Context<Self>) {
        if self.galeri_teması != tema {
            self.galeri_teması = tema;
            self.temayı_tazele(bağlam);
        }
    }

    /// Üst şerit seçicisini açar ya da kapatır.
    ///
    /// Açık seçiciye yeniden basmak kapatır; başkasına basmak öncekini
    /// kapatıp yenisini açar.
    pub fn seçiciyi_değiştir(
        &mut self,
        kimlik: impl Into<gpui::SharedString>,
        bağlam: &mut Context<Self>,
    ) {
        let kimlik = kimlik.into();
        self.açık_seçici = if self.açık_seçici.as_ref() == Some(&kimlik) {
            None
        } else {
            Some(kimlik)
        };
        // Kolondaki seçicilerin açık/kapalı yüzü ve tembel liste içeriği
        // bu duruma bakar.
        self.kolonu_geçersizle(bağlam);
        bağlam.notify();
    }

    /// Tema kipini doğrudan seçer.
    ///
    /// Dört kip elle kayıtlıdır ve yüksek karşıtlık **otomatik üretilmez**:
    /// parlaklık kaydırarak türetilen bir palet karşıtlık oranını garanti
    /// etmez. Bu yüzden kip bir geçiş değil, dört değerli bir seçimdir.
    pub fn galeri_kipini_seç(
        &mut self,
        kip: gpui_bilesenleri::TemaKipi,
        bağlam: &mut Context<Self>,
    ) {
        if self.galeri_kipi != kip {
            self.galeri_kipi = kip;
            self.temayı_tazele(bağlam);
        }
    }

    /// Açık ve koyu kip arasında geçiş yapar.
    pub fn galeri_kipini_değiştir(&mut self, bağlam: &mut Context<Self>) {
        use gpui_bilesenleri::TemaKipi;
        self.galeri_kipi = if self.galeri_kipi == TemaKipi::Koyu {
            TemaKipi::Açık
        } else {
            TemaKipi::Koyu
        };
        self.temayı_tazele(bağlam);
    }

    /// Kare dikişlerini kurar: palet, kabuk görünümü, çözülmüş görünüm ve
    /// açık seçici.
    ///
    /// Çözülmüş görünüm tema sürümüne bağlıdır: çözüm yalnız temaya (ve
    /// temayla birlikte sürümü artan palete) göre değişir. Her karede
    /// yeniden çözmek, profil çözümünü ve tam bir tema anlık görüntüsü
    /// kuruluşunu kare hızında koşturuyordu.
    fn kare_dikişlerini_kur(&mut self) {
        paleti_kur(galeri_paleti(self.galeri_teması, self.galeri_kipi));
        // Kabuk görünümü çözümden **önce** kurulur: `tasarım_görünümünü_çöz`
        // tema anlık görüntüsünü okur ve tipografi ile yoğunluk oradan gelir.
        kabuk_görünümünü_kur(
            &self.tezgah.tema.yazı_ailesi,
            self.tezgah.tema.punto,
            self.tezgah.tema.metin_ölçeği,
            self.tezgah.tema.yoğunluk,
            self.tezgah.tema.hareket,
        );
        let görünüm = if let Some((sürüm, görünüm)) = &self.görünüm_önbelleği
            && *sürüm == self.tezgah.tema.sürüm
        {
            Arc::clone(görünüm)
        } else {
            let çözüm = Arc::new(tasarım_görünümünü_çöz());
            self.görünüm_önbelleği = Some((self.tezgah.tema.sürüm, Arc::clone(&çözüm)));
            çözüm
        };
        görünümü_paylaşımlı_kur(görünüm);
        açık_seçiciyi_kur(self.açık_seçici.clone());
    }

    /// Yeni paleti canlı alanlara da uygular.
    ///
    /// Palet kare başında kurulur ama `GirişKutusu` kendi anlık görüntüsünü
    /// saklar; tazelenmezse tezgâh kutusu eski renklerde kalırdı.
    fn temayı_tazele(&mut self, bağlam: &mut Context<Self>) {
        // Sürüm dikişten **önce** artar: görünüm önbelleğinin anahtarı odur
        // ve eski sürümle kurulan dikiş bayat çözüm döndürürdü.
        self.tezgah.tema.kip = self.galeri_kipi;
        self.tezgah.tema.sürümü_artır();
        self.kare_dikişlerini_kur();
        if let Some(alan) = self.tezgah_alanı.clone() {
            let tema = tezgah_teması(&self.tezgah.kutu_teması());
            alan.update(bağlam, |alan, bağlam| {
                alan.temayı_değiştir(tema, bağlam)
            });
        }
        // Tercih kutuları da kabuk temasını taşır; kuruluştaki temayı
        // saklıyorlar ve kip değişince kendiliğinden yenilenmiyorlar.
        if let Some(alanlar) = self.sergi_girişleri.clone() {
            alanlar.temayı_değiştir(bağlam);
        }
        // Palet ve çözülmüş görünüm kolonun bütün yüzlerini besler.
        self.kolonu_geçersizle(bağlam);
        bağlam.notify();
    }

    /// Platform bildirimlerini kurar.
    ///
    /// Tek çağrı olmasının nedeni sarmalayıcıyı sabit tutmak: her yeni port
    /// için sarmalayıcıya bir satır eklemek, davranışın oraya kaymasının ilk
    /// adımıdır. Politika buraya da girmez — öncelik sırası ve düşme kuralı
    /// her portun kendi çekirdek çözümündedir.
    pub fn platform_portlarını_kur(
        &mut self,
        portlar: PlatformPortları,
        bağlam: &mut Context<Self>,
    ) {
        self.saat_dilimi_portu = portlar.saat_dilimi;
        self.imleç_portu = portlar.imleç;
        self.otomatik_doldurma_portu = portlar.otomatik_doldurma;
        // Platform dilimi bağlandı: yaşayan yerel kök yeni çözümle
        // eşitlenir. Eşitleme koşulsuz güvenlidir — sarmalayıcı bunu
        // pencere açılmadan çağırır ama sözleşme bir çağrı sırasına
        // dayanmaz: yaşayan alan varsa yeni bağlam ona da inilir, hiçbir
        // alan eski kökü sessizce kullanmaz.
        self.yerel_kökü_eşitle(bağlam);
    }

    /// `§25` platform otomatik doldurmayı sunuyor mu?
    ///
    /// Tercih yalnız yetenek açıkken gösterilir: çalışmayan bir tercih
    /// programcıyı yanıltır. Masaüstünde `GPUI` yerel metin alanı açmadığı
    /// için yetenek kapalıdır; tarayıcıda gizli girdi varsa açıktır.
    pub fn otomatik_doldurma_kullanılabilir(&self, bağlam: &gpui::App) -> bool {
        self.otomatik_doldurma_portu
            .as_ref()
            .is_some_and(|port| port.yetenek(bağlam).kullanılabilir)
    }

    /// `B` bölümünün port kapıları.
    ///
    /// Uzak doğrulama portu açılışta kurulmaz: `EşzamansızDoğrulamaPortu`
    /// bir ürün kararıdır (benzersizlik, sunucu kuralı) ve galeri onları
    /// taklit etmez. Kullanıcı kararıyla tek istisna dış doğrulama
    /// kartının **gösterim beslemesi**dir: ilk bildirimden sonra port
    /// bağlı görünür, kartın açıklaması gerçek sunucu olmadığını yazar.
    pub fn port_durumu(&self, bağlam: &gpui::App) -> metin_girisi_profili::PortDurumu {
        metin_girisi_profili::PortDurumu {
            otomatik_doldurma: self.otomatik_doldurma_kullanılabilir(bağlam),
            saat_dilimi: self.saat_dilimi_portu.is_some(),
            imleç: self.imleç_portu.is_some(),
            uzak_doğrulama: self
                .tezgah_alanı
                .as_ref()
                .is_some_and(|alan| alan.read(bağlam).doğrulama_portu.is_some()),
        }
    }

    /// `§16` tezgâhın dış doğrulama **gösterim beslemesi**.
    ///
    /// "Galeri sahte sunucu taklit etmez" duruşu bu eksen için kullanıcı
    /// kararıyla esnetildi: dış hata temizleme politikası ancak bir dış
    /// bildirimle gözlemlenebilir. Besleme benzersizlik ya da iş kuralı
    /// taklidi yapmaz; tek işi sabit bir `Sunucu` sonucunu (ya da boş
    /// sonucu) alanın gerçek port yolundan — `ORT-007` commit kapısı
    /// dâhil — geçirmektir.
    pub fn tezgah_dış_bildirimi(
        &mut self,
        hata: bool,
        pencere: &mut Window,
        bağlam: &mut Context<Self>,
    ) {
        use gpui_bilesenleri::{
            DoğrulamaKaynağı, GeçerlilikSorunu, GeçerlilikSorunuKimliği, GeçerlilikÖnemi,
        };
        let Some(alan) = self.tezgah_alanını_al(pencere, bağlam) else {
            // Alan kurulamadıysa besleme bağlanacak yer yoktur; exact
            // kuruluş sonucu zaten galeri durumunda çizilidir.
            return;
        };
        alan.update(bağlam, |alan, bağlam| {
            let sorunlar = if hata {
                vec![GeçerlilikSorunu {
                    kimlik: GeçerlilikSorunuKimliği(9001),
                    kaynak: DoğrulamaKaynağı::Sunucu,
                    önem: GeçerlilikÖnemi::Hata,
                    ileti: "Sunucu reddetti (gösterim)".into(),
                    // Port sürümü isteğin sürümüne eşitler; buradaki değer
                    // yer tutucudur.
                    değer_sürümü: 0,
                }]
            } else {
                Vec::new()
            };
            alan.doğrulama_portu = Some(std::sync::Arc::new(GösterimDoğrulamaPortu(sorunlar)));
            alan.eşzamansız_doğrulamayı_başlat(bağlam);
        });
        // Port kapıları kartı `doğrulama_portu.is_some()` okur ve kolonun
        // içindedir. Kök artık alanı gözlemediği için buradaki değişim
        // açıkça bildirilir; sorunların kendisi panellerin işidir.
        self.kolonu_geçersizle(bağlam);
        bağlam.notify();
    }

    /// Tezgâh tercihine göre çözülmüş saat dilimi.
    pub fn çözülmüş_saat_dilimi(&self) -> ÇözülmüşSaatDilimi {
        saat_dilimini_çöz(
            &self.tezgah.saat_dilimi_tercihi,
            self.saat_dilimi_portu.as_deref(),
        )
    }

    /// Yaşayan yerel kökü çözülmüş dilimle eşitler.
    ///
    /// Kök değiştiyse `ORT-021` hizmeti tek atomda yeniden mühürlenmiştir;
    /// yeni bağlam bütün yaşayan alanlara **atomik yüzeyden**
    /// (`GirişKutusu::yerel_bağlamı_değiştir`) inilir: yerel-türevli
    /// planlar (maske) bileşenin kendi atomunda yeniden kurulur ve metin
    /// damgası sabit kalır (`ACC-158`) — eksen-ayrılığı hükmü artık
    /// bileşenin kendi sözleşmesidir. Bağlamlar fabrika üretimidir, elle
    /// kurulmaz. Not: bu yüzey
    /// `authorize_bil010_injected_locale_context_and_atomic_runtime_locale_update_surface`
    /// karar atomunun ürünüdür ve şimdilik kardeşin **commitsiz çalışma
    /// ağacından** tüketilir; kardeş yüzeyi mühürleyene kadar bu dikiş
    /// yeniden oynayabilir.
    /// Eşitlemenin dayandığı çözülmüş dilimi döndürür: dilimi okuyan her
    /// sunum yolu bu dönüşü kullanmalı ki ekrandaki dilim ile `ORT-021`
    /// kökü hiçbir karede ayrışmasın (masaüstü portu bildirimi tazelik
    /// penceresiyle kendiliğinden yenileyebilir).
    fn yerel_kökü_eşitle(&mut self, bağlam: &mut Context<Self>) -> ÇözülmüşSaatDilimi {
        let dilim = self.çözülmüş_saat_dilimi();
        let yeni_kök = match self
            .metin_hizmetleri
            .yerel_kökü_gerekirse_yenile(&self.kimlik_fabrikası, dilim.kimlik.as_ref())
        {
            Ok(Some(yeni_kök)) => yeni_kök,
            Ok(None) => {
                // Kök yerinde; ama önceki iniş typed retle düştüyse alanlar
                // eski (tutarlı) bağlamda kalmıştır — her eşitleme turu
                // inişi yeniden dener, başarıda kayıt düşer. Güncel alan
                // için bileşen erken döner; tur ucuz kalır.
                if self.yerel_uygulama_hatası.is_some() {
                    let kök = self.metin_hizmetleri.yerel_kök();
                    self.yaşayan_alanlara_uygula(&kök, bağlam);
                }
                return dilim;
            }
            Err(hata) => {
                // Terminal akıbet: kök ve alanlar eski (hâlâ tutarlı)
                // bağlamda kalır; exact sonuç durumda görünür durur.
                // Bildirim yalnız geçişte üretilir — eşitleme artık kartın
                // okuma yolundan da koşar ve terminal durumda her karede
                // notify etmek çizimi döngüye sokar.
                if self.yerel_kök_hatası != Some(hata) {
                    self.yerel_kök_hatası = Some(hata);
                    bağlam.notify();
                }
                return dilim;
            }
        };
        self.yaşayan_alanlara_uygula(&yeni_kök, bağlam);
        dilim
    }

    /// Yeni yerel bağlamı bütün yaşayan alanlara atomik yüzeyden indirir.
    ///
    /// Ret typed'dır ve yutulmaz: son ret [`Self::yerel_uygulama_hatası`]
    /// yuvasında durur ve önizleme tanı satırında çizilir; başarılı tam
    /// iniş yuvayı temizler. Ret alanı **eski (tutarlı)** bağlamda bırakır
    /// (bileşen atomu commit'i tek noktada yapar). Bildirim yalnız geçişte
    /// üretilir — bu yol çizimden de çağrılır.
    fn yaşayan_alanlara_uygula(
        &mut self,
        kök: &Arc<gpui_bilesenleri::YerelMetinBağlamı>,
        bağlam: &mut Context<Self>,
    ) {
        let mut son_ret: Option<gpui_bilesenleri::GirişHatası> = None;
        if let Some(alan) = self.tezgah_alanı.clone() {
            let yeni = (**kök).clone();
            if let Err(hata) = alan.update(bağlam, |alan, bağlam| {
                alan.yerel_bağlamı_değiştir(yeni, bağlam)
            }) {
                son_ret = Some(hata);
            }
        }
        if let Some(alanlar) = self.sergi_girişleri.clone() {
            if let Some(hata) = alanlar.yerel_bağlamı_değiştir(kök, bağlam) {
                son_ret = Some(hata);
            }
        }
        if self.yerel_uygulama_hatası != son_ret {
            self.yerel_uygulama_hatası = son_ret;
            bağlam.notify();
        }
    }

    /// Çizim ağacının `ORT-021` anahtar çözücüsü; kök-kapsamlı hizmeti sarar
    /// ve typed akıbetleri kökün yaşayan yuvasına bağlar.
    pub(crate) fn tezgah_çözücüsü(&self) -> TezgahİletiÇözücüsü {
        self.metin_hizmetleri
            .çözücü(Rc::clone(&self.ileti_çözüm_hata_kaydı))
    }

    /// Yerel kök yenilemesinin son terminal akıbeti (varsa), exact typed.
    pub(crate) fn yerel_kök_hatası(&self) -> Option<YerelKökHatası> {
        self.yerel_kök_hatası
    }

    /// Canlı UI çözüm yolunun son typed hatası (varsa).
    ///
    /// `çöz` sunum dizesini üretmeden önce buraya yazar; typed payload
    /// dizeye indirgenmeden kökte gözlenebilir kalır.
    pub(crate) fn son_ileti_çözüm_hatası(&self) -> Option<TezgahÇözümKaydı> {
        self.ileti_çözüm_hata_kaydı.borrow().clone()
    }

    /// `§30` tercih eşitlemesinin son kalıcı reddi (varsa), exact typed.
    pub(crate) fn tercih_eşitleme_hatası(&self) -> Option<TercihEşitlemeKaydı> {
        self.tercih_eşitleme_hatası.clone()
    }

    /// Atomik yerel-bağlam inişinin son typed reddi (varsa).
    pub(crate) fn yerel_uygulama_hatası(&self) -> Option<gpui_bilesenleri::GirişHatası> {
        self.yerel_uygulama_hatası.clone()
    }

    /// Uygulama kökünün `ORT-002` motoru.
    ///
    /// Sarmalayıcı platform portları kimlik doğrulaması için bunu alır;
    /// ikinci bir Unicode kökü kurmaz, kimlik de elle mühürlemez.
    pub fn metin_motoru(&self) -> Arc<UnicodeMetinMotoru> {
        self.metin_hizmetleri.motor()
    }

    /// Maske laboratuvarında hazır deseni desen alanına yazar.
    ///
    /// Desen alanı kanonik bileşendir; galeri metni doğrudan tamponuna
    /// yazmaz, bileşenin kendi giriş yolunu kullanır.
    pub fn desen_şablonunu_uygula(
        &mut self,
        desen: &str,
        pencere: &mut Window,
        bağlam: &mut Context<Self>,
    ) {
        let Some(alanlar) = self.sergi_girişleri.clone() else {
            return;
        };
        let desen = desen.to_owned();
        alanlar.desen.update(bağlam, |alan, bağlam| {
            alan.durum.tümünü_seç();
            let seçim = alan
                .durum
                .bayt_aralığını_utf16_çevir(alan.durum.seçim_baytları());
            gpui::EntityInputHandler::replace_text_in_range(
                alan,
                Some(seçim),
                &desen,
                pencere,
                bağlam,
            );
        });
        bağlam.notify();
    }
}

impl GaleriUygulaması {
    /// Tezgâh tercihini değiştirir ve önizlemeyi günceller.
    ///
    /// Yazılan değer korunur: tercih açılıp kapanırken kutu sıfırlanırsa
    /// etkiyi canlı izlemek imkânsızlaşır. Bu yüzden alan yeniden kurulmaz,
    /// yapılandırması yerinde değiştirilir; ham değeri bileşenin kendi
    /// `§9.7` kuralı taşır.
    ///
    /// Tek istisna değer türüdür: tür değişince metnin anlamı da değişir,
    /// alan o türe uygun örnek değerle baştan kurulur.
    pub fn tezgahı_değiştir(
        &mut self,
        değiştir: impl FnOnce(&mut TezgahTercihleri),
        bağlam: &mut Context<Self>,
    ) {
        let önceki_tür = self.tezgah.değer_türü;
        let önceki_tema = self.tezgah.tema.clone();
        // Seçim yapıldı: açık liste kapanır. Tercih değiştiren tek yol
        // liste öğesine ya da liste dışındaki bir düğmeye basmaktır;
        // ikisinde de açık listenin ayakta kalması için sebep yok. Dış
        // tıklamayla kapanma ayrı bir iş (`ORT-006` konağı, borç 17) —
        // burada kapanan yalnız seçimi izleyen liste.
        self.açık_seçici = None;
        değiştir(&mut self.tezgah);
        // Rapor ve kod önbelleğinin anahtarı: tercih değişmiş olabilir,
        // sürüm artar ve ikisi de bir sonraki okumada yeniden kurulur.
        self.tercih_sürümü = self.tercih_sürümü.wrapping_add(1);
        // Tür değişince o türde kurulamayan tercihler kapanır; galeride
        // görünmeyen bir tercihin kodda etkili kalması yanıltıcı olur.
        self.tezgah.türe_uyarla();
        // `ORT-004` tema tercihi değişince yeni anlık görüntü üretilir.
        if self.tezgah.tema != önceki_tema {
            self.tezgah.tema.sürümü_artır();
            // Tercih kutuları kabuk temasını kuruluşta saklıyor. Üst
            // şeritten aile, punto ya da metin ölçeği değişince onlar da
            // tazelenmeli — kip değişiminde aynı boşluk ön ek kutusunu
            // açık kipte bırakıyordu.
            if let Some(alanlar) = self.sergi_girişleri.clone() {
                alanlar.temayı_değiştir(bağlam);
            }
        }
        // `ORT-002 §5.2` dilim çözümü tercih değişince yeniden koşar. Yerel
        // bağlam elle mutasyona uğratılmaz: kök değiştiyse hizmet ve bağlam
        // tek atomda yenilenir, yeni kök bütün yaşayan alanlara inilir.
        self.yerel_kökü_eşitle(bağlam);
        // Yaşayan alan yokken **her** tercih değişimi yeni bir `kur`
        // denemesidir: gerçek bir kuruluş reddi (ör. derlenemeyen desen)
        // tercih düzeltilince yeniden denenmeli, eski exact sonuç yeni
        // yapılandırmaya ait değildir.
        if self.tezgah_alanı.is_none() {
            self.tezgah_kuruluş_raporu = None;
            self.tezgah_varsayılan_değer_hatası = None;
            self.tezgah_kuruluş_hatası = None;
        }
        if self.tezgah.değer_türü != önceki_tür {
            self.tezgah_alanı = None;
            self.tezgah_yardımcı_kimlikleri = None;
            // Kuruluş ekseni de sıfırlanır: yeni tür yeni bir `kur`
            // denemesidir, eski exact sonuç ona ait değildir.
            self.tezgah_kuruluş_raporu = None;
            self.tezgah_varsayılan_değer_hatası = None;
            self.tezgah_kuruluş_hatası = None;
        } else if let Some(alan) = self.tezgah_alanı.clone() {
            let kimlikler = self
                .tezgah_yardımcı_kimlikleri
                .as_ref()
                .expect("yaşayan tezgâh alanının yardımcı kimlikleri vardır");
            let yapılandırma = self
                .tezgah
                .yapılandırma_kimliklerle(kimlikler, &self.metin_hizmetleri.motor());
            let tema = tezgah_teması(&self.tezgah.kutu_teması());
            let önem_zemini = self.tezgah.önem_zemini;
            alan.update(bağlam, |alan, bağlam| {
                alan.yapılandırmayı_değiştir(yapılandırma, bağlam);
                alan.temayı_değiştir(tema, bağlam);
                // `§28` durumu ve önemi **kurulamaz**: ikisi de `§16` sorun
                // kümesinden türetilir ve tek yazarları `sorunları_uygula`dır
                // (`§29.0`). Tezgâh onları doğrulama kuralı üzerinden üretir;
                // kartlar sonucu okur.
                //
                // `ORT-004` erişim durumu da türetilmiştir: kaynağı
                // `salt_okunur`/`etkin` yapılandırmasıdır. Senaryo ekseni
                // aynı sonuca ikinci bir yol açıyordu (`§29.0`).
                alan.önem_zemini = önem_zemini;
            });
        }
        self.tercih_alanlarını_eşitle(bağlam);
        // Bölüm listesi, rapor ve kod tercihe bağlıdır: kolon tazelenmeli.
        self.kolonu_geçersizle(bağlam);
        bağlam.notify();
    }

    /// Tercihin kendi kurduğu metinleri ekrandaki tercih kutularına yazar.
    ///
    /// Desen, ön ek ve son ek iki yönlü: kutuya yazınca tercih değişir, ama
    /// içerik türü seçimi de tercihi değiştirir. İkinci yön ekrana
    /// yansımadığında kutuda "₺" yazarken alan "+90" bekliyordu.
    fn tercih_alanlarını_eşitle(&mut self, bağlam: &mut Context<Self>) {
        let Some(alanlar) = self.sergi_girişleri.clone() else {
            return;
        };
        let hedefler = [
            (alanlar.desen.clone(), self.tezgah.desen.clone()),
            (alanlar.ön_ek_metni.clone(), self.tezgah.ön_ek_metni.clone()),
            (
                alanlar.son_ek_metni.clone(),
                self.tezgah.son_ek_metni.clone(),
            ),
        ];
        for (kutu, hedef) in hedefler {
            let sonuç = kutu.update(bağlam, |kutu, bağlam| {
                if kutu.metin() == hedef {
                    return Ok(());
                }
                // Tampon doğrudan yazılmaz: `§30` dış değişiklik portu
                // sürümlü uygular. Ret olağan bir typed akıbettir ve panic'e
                // çevrilmez (birleşim sırasındaki bir tercih tıklaması
                // uygulamayı düşürmemeli). Anlık görüntü aynı kirada
                // alındığı için `EskiSürüm` bu yolda üretilemez.
                let anlık = gpui_bilesenleri::MetinDüzenlemePortu::anlık_görüntü(kutu);
                let sonuç = gpui_bilesenleri::MetinDüzenlemePortu::dış_değişikliği_uygula(
                    kutu,
                    gpui_bilesenleri::MetinDeğişikliği {
                        utf8_aralığı: 0..anlık.metin.len(),
                        yeni_metin: hedef.clone(),
                    },
                    anlık.değer_sürümü,
                );
                if sonuç.is_ok() {
                    bağlam.notify();
                }
                sonuç.map(|_| ())
            });
            match sonuç {
                Ok(()) => {
                    self.bekleyen_tercih_eşitlemeleri.remove(&kutu.entity_id());
                    // Kutu ve tercih yeniden uyumlu: bu kutuya ait kalıcı
                    // ret kaydı (varsa) geçmiştir, satır söner.
                    if self
                        .tercih_eşitleme_hatası
                        .as_ref()
                        .is_some_and(|kayıt| kayıt.kutu == kutu.entity_id())
                    {
                        self.tercih_eşitleme_hatası = None;
                        bağlam.notify();
                    }
                }
                // `CompositionEtkin` geçici erteleme akıbetidir: yaşayan IME
                // birleşimi dış yazımla bozulamaz ve birleşimin her bitiş
                // yolu — `unmark_text` de, `insertText`-commit de
                // (`BİL-010 ≥23.0` commit kolu kompozisyon değerini düşürür)
                // — ekseni kapattığı için erteleme sınırlıdır. Hedef
                // kaybolmaz — bekleyen kayıt, kutunun bir sonraki metin
                // olayında ileri eşitlemeyi yeniden denetir ve o olayın kutu
                // metnini tercihe geri yazmasını bastırır; birleşim biterken
                // seçilen tercih birleşim metnine ezilmez.
                Err(gpui_bilesenleri::GirişHatası::CompositionEtkin) => {
                    self.bekleyen_tercih_eşitlemeleri.insert(kutu.entity_id());
                }
                // Diğer her ret **kalıcıdır**: örn. `SürümTükendi`
                // terminaldir (eşitleme bir daha başarılı olamaz). Kalıcı
                // ret bekleyen kayda dönüşmez — dönüşseydi kutunun metin
                // olayları süresiz bastırılırdı; exact typed akıbet burada
                // gözlenir ve önizleme tanı satırında çizilir.
                Err(hata) => {
                    self.bekleyen_tercih_eşitlemeleri.remove(&kutu.entity_id());
                    let kayıt = TercihEşitlemeKaydı {
                        kutu: kutu.entity_id(),
                        hata,
                    };
                    if self.tercih_eşitleme_hatası.as_ref() != Some(&kayıt) {
                        self.tercih_eşitleme_hatası = Some(kayıt);
                        bağlam.notify();
                    }
                }
            }
        }
    }

    /// Ölçüm koşumlarının alan kapısı.
    ///
    /// Ölçüm varsayılan (bilinen-geçerli) tercihle koşar; kuruluş burada
    /// düşerse bu bir ürün durumu değil ölçüm ortamı arızasıdır ve exact
    /// typed sonuç mesajla taşınır — hata sessizce yutulup sahte sayı
    /// üretilmez.
    fn ölçüm_alanı(
        &mut self,
        pencere: &mut Window,
        bağlam: &mut Context<Self>,
    ) -> Entity<GirişKutusu> {
        match self.tezgah_alanını_al(pencere, bağlam) {
            Some(alan) => alan,
            None => panic!(
                "ölçüm tezgâh alanı kurulamadı: {:?}",
                self.tezgah_kuruluş_hatası
            ),
        }
    }

    /// `ORT-018` `bil-010.input.commit` ölçümü.
    ///
    /// Tezgâhın yaşayan alanı üzerinde gerçek kabul yolunu koşturur ve her
    /// koşumun süresini milisaniye döner. Ölçüm ayrı bir taklit alanda
    /// koşarsa sayı ürünle karşılaştırılamaz.
    pub fn ölçüm_koştur(
        &mut self,
        ısınma: u32,
        tekrar: u32,
        pencere: &mut Window,
        bağlam: &mut Context<Self>,
    ) -> Vec<f64> {
        let alan = self.ölçüm_alanı(pencere, bağlam);
        let mut süreler = Vec::with_capacity(tekrar as usize);
        for sıra in 0..(ısınma + tekrar) {
            let süre = alan.update(bağlam, |alan, bağlam| {
                // Kabul kirliliği düşürür; her koşum aynı işi yapsın diye
                // metin yeniden kirletilir, yoksa ikinci koşum kısa yoldan
                // döner ve ölçüm gerçek maliyeti göstermez.
                alan.durum.düzenleme_kirli = true;
                let başlangıç = şimdi_ms();
                alan.değeri_kabul_et_dışarıdan(pencere, bağlam);
                şimdi_ms() - başlangıç
            });
            if sıra >= ısınma {
                süreler.push(süre);
            }
        }
        süreler
    }

    /// Kare ölçümü: tezgâh alanının metnini gerçek giriş yolundan değiştirir.
    ///
    /// `tests/kare_olcumu.rs` tuş vuruşu senaryosunu bununla kurar: metin
    /// bileşenin kendi `replace_text_in_range` yolundan geçer (maske,
    /// politika, bildirim ve olay yayını dâhil), alan tutamacı dışarı
    /// verilmez. Her çağrı tüm metni değiştirir ki tekrarlar arasında
    /// metin uzunluğu kaymasın.
    pub fn ölçüm_alanına_yaz(
        &mut self,
        metin: &str,
        pencere: &mut Window,
        bağlam: &mut Context<Self>,
    ) {
        let alan = self.ölçüm_alanı(pencere, bağlam);
        let metin = metin.to_owned();
        alan.update(bağlam, |alan, bağlam| {
            alan.durum.tümünü_seç();
            let seçim = alan
                .durum
                .bayt_aralığını_utf16_çevir(alan.durum.seçim_baytları());
            gpui::EntityInputHandler::replace_text_in_range(
                alan,
                Some(seçim),
                &metin,
                pencere,
                bağlam,
            );
        });
    }

    /// `ORT-018` toplu ölçüm: `tekrar` kabulün toplam süresi (ms).
    ///
    /// Tarayıcı `performance.now()`u güvenlik gerekçesiyle `0.1 ms`e
    /// yuvarlar; tek kabul bu eşiğin altında kaldığı için tek tek ölçüm
    /// yalnız saatin çözünürlüğünü gösterir. Toplu ölçüm gerçek ortalamayı
    /// verir — yüzdebirlik veremez ama uydurma sayı da üretmez.
    pub fn ölçüm_toplu_ms(
        &mut self,
        ısınma: u32,
        tekrar: u32,
        pencere: &mut Window,
        bağlam: &mut Context<Self>,
    ) -> f64 {
        let alan = self.ölçüm_alanı(pencere, bağlam);
        // Ölçüm içeriği sabitlenir: boş alanın kabulü dolu alanınkinden
        // ucuzdur ve iki hedefin sayısı karşılaştırılamaz hâle gelir.
        // İçerik gerçek giriş yolundan yazılır; ham tampona doğrudan yazım
        // güncel yüzeyde yok.
        alan.update(bağlam, |alan, bağlam| {
            alan.durum.tümünü_seç();
            let seçim = alan
                .durum
                .bayt_aralığını_utf16_çevir(alan.durum.seçim_baytları());
            gpui::EntityInputHandler::replace_text_in_range(
                alan,
                Some(seçim),
                ÖLÇÜM_METNİ,
                pencere,
                bağlam,
            );
        });
        let mut koştur = |alan: &Entity<GirişKutusu>, sayı: u32, bağlam: &mut Context<Self>| {
            for _ in 0..sayı {
                alan.update(bağlam, |alan, bağlam| {
                    alan.durum.düzenleme_kirli = true;
                    alan.değeri_kabul_et_dışarıdan(pencere, bağlam);
                });
            }
        };
        koştur(&alan, ısınma, bağlam);
        let başlangıç = şimdi_ms();
        koştur(&alan, tekrar, bağlam);
        şimdi_ms() - başlangıç
    }

    /// Yazı sisteminin bildirdiği aileleri bir kez okur ve saklar.
    ///
    /// Kitaplık aileleri kendi bölümünde listelendiği için burada
    /// yinelenmez; iç adlar ve gpui'nin eklediği yedek adlar
    /// `aile_gösterilebilir_mi` ile ayıklanır.
    fn sistem_ailelerini_al(&mut self, bağlam: &mut Context<Self>) -> Rc<Vec<String>> {
        if let Some(aileler) = self.sistem_yazı_aileleri.clone() {
            return aileler;
        }
        let mut adlar: Vec<String> = bağlam
            .text_system()
            .all_font_names()
            .into_iter()
            .filter(|ad| aile_gösterilebilir_mi(ad) && !KİTAPLIK_AİLELERİ.contains(&ad.as_str()))
            .collect();
        adlar.sort_unstable();
        adlar.dedup();
        let aileler = Rc::new(adlar);
        self.sistem_yazı_aileleri = Some(aileler.clone());
        aileler
    }

    fn tezgah_alanını_al(
        &mut self,
        pencere: &mut Window,
        bağlam: &mut Context<Self>,
    ) -> Option<Entity<GirişKutusu>> {
        if let Some(alan) = self.tezgah_alanı.clone() {
            return Some(alan);
        }
        // Kuruluş bir kez denenir; exact typed hata durur ve önizleme onu
        // çizer. Tercih (tür) değişimi eksenleri sıfırlar ve yeniden dener.
        if self.tezgah_kuruluş_hatası.is_some() {
            return None;
        }
        let yardımcı_kimlikleri = YardımcıKimlikleri::yeni(&self.kimlik_fabrikası);
        let yapılandırma = self
            .tezgah
            .yapılandırma_kimliklerle(&yardımcı_kimlikleri, &self.metin_hizmetleri.motor());
        // Önizleme seçili tercihi açılışta göstersin: boş kutu hizalamayı,
        // ayracı, gizlemeyi ve temizleme simgesini görünür kılmaz.
        let örnek = self.tezgah.örnek_değer();
        // Tezgâh kendi kâğıt paletini taşır; önizleme kutusu da o palete
        // göre çizilir ki ekranın bütünü tek bir tasarım dili konuşsun.
        let tema = tezgah_teması(&self.tezgah.kutu_teması());
        let imleç_portu = self.imleç_portu.clone();
        let doldurma_portu = self.otomatik_doldurma_portu.clone();
        let katalog = galeri_simge_kataloğu();
        let bileşen =
            galeri_bileşen_kimliği(&self.kimlik_fabrikası, "galeri.metin_girisi", "tezgah");
        // `BİL-010 §29` fallible kuruluş: `ORT-002` kökü ve başlangıç
        // damgası uygulama kökünden verilir; kutu kök kurmaz.
        let sonuç = GirişKutusu::kur(
            bileşen,
            self.metin_hizmetleri.unicode(),
            self.metin_hizmetleri.alan_damgası(&self.kimlik_fabrikası),
            // Yerel bağlam kuruluşta enjekte edilir: `kur` hazırlığı
            // (maske şablonu, varsayılan) host kökünün bağlamıyla koşar,
            // kutu bağlam üretmez.
            (*self.metin_hizmetleri.yerel_kök()).clone(),
            yapılandırma,
            örnek,
            tema,
            pencere,
            bağlam,
        );
        let gpui_bilesenleri::GirişKuruluşSonucu {
            bileşen: alan,
            rapor,
            varsayılan_değer_hatası,
        } = match sonuç {
            Ok(sonuç) => sonuç,
            Err(hata) => {
                // Başarısız kuruluş yarım entity, abonelik ya da yaşayan
                // durum bırakmaz (`kur` entity'yi ancak bütün fallible
                // adımlardan sonra üretir); exact sonuç galeri durumuna
                // iner. Önceki bir alandan kalan gözlem panelleri de
                // düşürülür — aksi hâlde eski entity'yi tutamaç ve
                // abonelikleriyle süresiz canlı tutarlardı; invariant
                // koşulsuzdur, başarılı kuruluş panelleri baştan kurar.
                self.tezgah_panelleri = None;
                self.tezgah_kuruluş_hatası = Some(hata);
                return None;
            }
        };
        // Kuruluşun uyarı raporu ve varsayılan sağlayıcı akıbeti kayıpsız
        // saklanır; sağlayıcı hatası entity'yi öldürmez.
        self.tezgah_kuruluş_raporu = Some(rapor);
        self.tezgah_varsayılan_değer_hatası = varsayılan_değer_hatası;
        // Portlar, simge kataloğu ve yaşayan yerel kök hosttan verilir;
        // yerel bağlam fabrika üretimidir, alan üzerinde elle kurulmaz.
        //
        alan.update(bağlam, |alan, _| {
            alan.simge_kataloğu = Some(katalog);
            alan.imleç_portu = imleç_portu;
            alan.otomatik_doldurma_portu = doldurma_portu;
        });
        self.tezgah_yardımcı_kimlikleri = Some(yardımcı_kimlikleri);
        // Alanı gözleyen paneller alanla birlikte kurulur. `§16.2` gözlem
        // paneli sonucu **saklamaz**; her çizimde alandan ödünç okur ve
        // ödünç okumanın tazelenmesi için alanın bildirimini dinler. Kök bu
        // bildirimi dinlemez: GPUI'de bir view ancak kendisi (ya da bir alt
        // view'ı) bildirdiğinde kirlenir ve ilerideki önbellekli bölgeler
        // ancak okuma gözleyen panelde durursa doğru kalır.
        match self.tezgah_panelleri.clone() {
            Some(paneller) => {
                // Tür değişiminde alan yeniden kuruldu; alan gözleyen
                // paneller yaşamaya devam eder, yalnız yeni alana bağlanır.
                // Bölüm paneli alanı gözlemediği için bağlanacak bir şeyi
                // yok; kolonun tazelenmesini kökün bildirimi sağlar.
                paneller.alan_durumu.update(bağlam, |panel, bağlam| {
                    panel.alanı_bağla(alan.clone(), bağlam);
                });
                paneller.olay_akışı.update(bağlam, |panel, bağlam| {
                    panel.alanı_bağla(alan.clone(), bağlam);
                });
                paneller.yuva_notu.update(bağlam, |panel, bağlam| {
                    panel.alanı_bağla(alan.clone(), bağlam);
                });
            }
            None => {
                let kök = bağlam.weak_entity();
                let alan_durumu = {
                    let kök = kök.clone();
                    let alan = alan.clone();
                    bağlam.new(move |bağlam| AlanDurumPaneli::yeni(kök, alan, bağlam))
                };
                let olay_akışı = {
                    let alan = alan.clone();
                    bağlam.new(move |bağlam| OlayAkışıPaneli::yeni(alan, bağlam))
                };
                let yuva_notu = {
                    let kök = kök.clone();
                    let alan = alan.clone();
                    bağlam.new(move |bağlam| YuvaNotuPaneli::yeni(kök, alan, bağlam))
                };
                let bölümler = {
                    let kök = bağlam.entity();
                    bağlam.new(move |_| BölümlerPaneli::yeni(&kök))
                };
                #[cfg(feature = "olcum-izleyici")]
                let sanal_sol = {
                    let kök = kök.clone();
                    bağlam.new(move |_| SanalSolKolonPaneli::yeni(kök))
                };
                self.tezgah_panelleri = Some(TezgahPanelleri {
                    alan_durumu,
                    olay_akışı,
                    yuva_notu,
                    bölümler,
                    #[cfg(feature = "olcum-izleyici")]
                    sanal_sol,
                });
            }
        }
        self.tezgah_alanı = Some(alan.clone());
        Some(alan)
    }

    /// Önbellekli sağ kolonun geçersizleme yolu.
    ///
    /// Kolon `Entity::cached` sınırındadır ve GPUI'de **`notify` onu
    /// patlatmaz**: `App::notify` bir entity'nin bildirimini yalnız o
    /// entity pencerenin `tracked_entities` kümesindeyken `invalidate_view`e
    /// çevirir; önbellekten dönen bir view render edilmediği için o kümeye
    /// kendi kimliğiyle girmez. Ölçüldü: ne kökün bildirimi, ne panele
    /// doğrudan `notify` kolonu yeniden kurdurur (`raporlar/
    /// PERFORMANS_MIMARISI.md` §6.3).
    ///
    /// Çalışan yol `refresh`tir: `refreshing` bayrağı prepaint'teki cache
    /// koşulunu düşürür. Efekt üzerinden çağrılır çünkü `Window` erişimi
    /// gerektirmez ve gerçek akışa uyar — listener bildirir, efekt döngüsü
    /// bayrağı kurar, sıradaki kare kolonu yeniden kurar, sonraki temiz
    /// kareler yine önbellekten gelir.
    ///
    /// Kolonu ilgilendiren **her** kök değişimi bunu çağırmalıdır; kapı
    /// `tests/kolon_tazeligi.rs`.
    fn kolonu_geçersizle(&self, bağlam: &mut Context<Self>) {
        // Yalnız bu pencere yenilenir. `refresh` doğrudan çağrılamaz:
        // buraya bir listener'ın içinden gelinir ve pencere o sırada
        // kiralıdır (`App::update_window` pencereyi `take` eder, iç içe
        // çağrı boş döner). `defer` kirayı bırakıldıktan sonra koşar ve
        // gerçek akışa da uyar: listener bildirir, efekt döngüsü bayrağı
        // kurar, sıradaki kare kolonu yeniden kurar.
        match self.pencere_tutamacı {
            Some(tutamaç) => bağlam.defer(move |bağlam| {
                tutamaç
                    .update(bağlam, |_, pencere, _| pencere.refresh())
                    .ok();
            }),
            // Henüz çizilmediyse tutamaç yok; ilk çizim zaten her şeyi kurar.
            None => bağlam.refresh_windows(),
        }
    }

    /// Tezgâhın yürürlükteki tercihleri; paneller çizimde buradan okur.
    pub(crate) fn tezgah_tercihleri(&self) -> &TezgahTercihleri {
        &self.tezgah
    }

    /// `§29` doğrulama raporu; tercih sürümüne bağlı.
    ///
    /// Rapor yalnız tercihten türer. Her çizimde yeniden kurmak hem
    /// doğrulamayı hem kimlik fabrikasını kare hızında çalıştırıyordu;
    /// şimdi yalnız tercih değişince kurulur.
    fn tezgah_raporu(&mut self) -> Rc<gpui_bilesenleri::GirişYapılandırmaRaporu> {
        if let Some((sürüm, rapor)) = &self.rapor_önbelleği
            && *sürüm == self.tercih_sürümü
        {
            return Rc::clone(rapor);
        }
        let rapor = Rc::new(
            self.tezgah
                .yapılandırma(&self.kimlik_fabrikası, &self.metin_hizmetleri.motor())
                .doğrula(),
        );
        self.rapor_önbelleği = Some((self.tercih_sürümü, Rc::clone(&rapor)));
        rapor
    }

    /// `ORT-003 §2` yarıçap tavanı: kısa kenarın yarısı; tema sürümüne bağlı.
    ///
    /// Tek satırlı alanda kısıtlayan kenar kutu yüksekliğidir.
    fn en_fazla_yarıçap(&self) -> f32 {
        // Tezgâh temasının etkileşim hedefi sabittir ve tek kaynaktan
        // okunur; sınır için tam tema anlık görüntüsü kurulmaz, önbelleğe
        // de gerek kalmaz.
        f32::from(crate::galeri::tezgah_etkileşim_hedefi()) / 2.
    }

    /// Kod paneli metni; tercih sürümüne bağlı.
    fn tezgah_kodu(&mut self) -> gpui::SharedString {
        if let Some((sürüm, kod)) = &self.kod_önbelleği
            && *sürüm == self.tercih_sürümü
        {
            return kod.clone();
        }
        let kod = gpui::SharedString::from(self.tezgah.kod());
        self.kod_önbelleği = Some((self.tercih_sürümü, kod.clone()));
        kod
    }

    /// `BİL-010` profilinin ürettiği tezgâh içeriği.
    ///
    /// Kabuk ile profil arasındaki köprü: uygulama durumunu toplar,
    /// `metin_girisi_profili` onu `Tezgahİçeriği`ne çevirir. Kabuk bu yapıdan
    /// ötesini görmez.
    pub fn tezgah_profil_içeriği(
        &mut self,
        pencere: &mut Window,
        bağlam: &mut Context<Self>,
    ) -> Tezgahİçeriği {
        // Tanı **aynı karede** görünür: kanonik anahtar sunumdan önce
        // yoklanır, sistemik akıbet yuvaya şimdi düşer ve satır bu karenin
        // render girdisiyle çizilir — okuma yazmadan önce kalmaz, ikinci
        // kare planına da gerek yoktur. Kuruluş-hatası yolu da dâhil:
        // yoklama alan denetiminden önce koşar. Anahtar-yerel akıbetler
        // kendi öğelerinde işaretle zaten görünür.
        self.tezgah_çözücüsü().yokla();
        let Some(alan) = self.tezgah_alanını_al(pencere, bağlam) else {
            // Kuruluş düştü: yarım entity/panel yok. Önizleme exact typed
            // sonucu ve (varsa) sistemik çözüm tanısını birlikte çizer;
            // tercih değişimi yeni bir deneme başlatır.
            return metin_girisi_profili::kuruluş_hatası_içeriği(
                self.tezgah_kuruluş_hatası.as_ref(),
                self.son_ileti_çözüm_hatası(),
                self.tercih_eşitleme_hatası(),
                self.yerel_uygulama_hatası(),
                self.tezgah_sol_kaydırma.clone(),
            );
        };
        let paneller = self
            .tezgah_panelleri
            .clone()
            .expect("paneller alanla birlikte kurulur");
        // Kod paneli metni tercih sürümüne bağlıdır: kart sonucu okur,
        // yeniden hesaplamaz. `§29` raporu da öyledir ama artık bu yolun
        // değil, bölüm panelinin girdisidir (`tezgah_bölümleri`).
        let kod = self.tezgah_kodu();
        let en_fazla_yarıçap = self.en_fazla_yarıçap();

        // Tercih yalnız okunur: klonlanmaz, doğrudan ödünç verilir.
        metin_girisi_profili::tezgah_içeriği(
            metin_girisi_profili::MetinGirişiProfilGirdisi {
                tercih: &self.tezgah,
                alan,
                paneller: &paneller,
                kod,
                sol_kaydırma: self.tezgah_sol_kaydırma.clone(),
                en_fazla_yarıçap,
                köşe_izi: self.köşe_izi.clone(),
                son_çözüm_hatası: self.son_ileti_çözüm_hatası(),
                tercih_eşitleme_hatası: self.tercih_eşitleme_hatası(),
                yerel_uygulama_hatası: self.yerel_uygulama_hatası(),
            },
            bağlam,
        )
    }

    /// Sanal `ListState` yolunun istediği tek üst düzey sol-kolon öğesini
    /// kökün bağlamında kurar. Normal yol aynı kurucuları topluca çağırır.
    #[cfg(feature = "olcum-izleyici")]
    pub(crate) fn sanal_sol_kartı(
        &mut self,
        indis: usize,
        bağlam: &mut Context<Self>,
    ) -> gpui::AnyElement {
        let paneller = self
            .tezgah_panelleri
            .clone()
            .expect("sanal liste alan panelleri kurulduktan sonra çizilir");
        if indis < metin_girisi_profili::SOL_TOPLAM_KART_SAYISI - 1 {
            return metin_girisi_profili::sol_ek_kartı(indis, &self.tezgah, &paneller, bağlam)
                .unwrap_or_else(|| div().into_any_element());
        }
        let kod = self.tezgah_kodu();
        metin_girisi_profili::sol_kod_kartı(kod)
    }

    /// Ölçümde sıradan scroll ile sanal listenin aynı üst düzey öğesini
    /// görünür kılar.
    #[cfg(feature = "olcum-izleyici")]
    pub fn ölçüm_sol_konumunu_ayarla(
        &mut self,
        konum: SolListeÖlçümKonumu,
        bağlam: &mut Context<Self>,
    ) {
        if sol_liste_sanallaştırması_açık() {
            if let Some(paneller) = self.tezgah_panelleri.clone() {
                paneller.sanal_sol.update(bağlam, |panel, bağlam| {
                    panel.konumu_ayarla(konum);
                    bağlam.notify();
                });
            }
        } else {
            match konum {
                SolListeÖlçümKonumu::Üst => {
                    self.tezgah_sol_kaydırma.set_offset(point(px(0.), px(0.)));
                }
                SolListeÖlçümKonumu::Orta => {
                    self.tezgah_sol_kaydırma.scroll_to_top_of_item(3);
                }
                SolListeÖlçümKonumu::Son => self.tezgah_sol_kaydırma.scroll_to_bottom(),
            }
        }
        bağlam.notify();
    }

    /// Isınma karesinden sonra gerçekleşen mantıksal sol-scroll konumu.
    #[cfg(feature = "olcum-izleyici")]
    pub fn ölçüm_sol_mantıksal_konumu(&self, bağlam: &gpui::App) -> (usize, gpui::Pixels) {
        if sol_liste_sanallaştırması_açık()
            && let Some(paneller) = self.tezgah_panelleri.as_ref()
        {
            return paneller.sanal_sol.read(bağlam).mantıksal_konum();
        }
        self.tezgah_sol_kaydırma.logical_scroll_top()
    }

    /// `BİL-010` sergi/tercih kutularını bir kez kurar.
    ///
    /// Kuruluş fallible'dır: başarısızlık exact typed olarak durumda
    /// saklanır ve yarım alan kümesi yaşatılmaz; ikinci deneme yapılmaz.
    fn sergi_girişlerini_al(
        &mut self,
        pencere: &mut Window,
        bağlam: &mut Context<Self>,
    ) -> Option<MetinGirişiAlanları> {
        if let Some(alanlar) = self.sergi_girişleri.clone() {
            return Some(alanlar);
        }
        if self.sergi_kuruluş_hatası.is_some() {
            return None;
        }
        match MetinGirişiAlanları::kur(
            &self.metin_hizmetleri,
            &self.kimlik_fabrikası,
            pencere,
            bağlam,
        ) {
            Ok(alanlar) => {
                self.sergi_girişleri = Some(alanlar.clone());
                Some(alanlar)
            }
            Err(hata) => {
                self.sergi_kuruluş_hatası = Some(hata);
                None
            }
        }
    }

    /// Sağ kolonun bölüm listesi; profilin `§9` tür süzgecinden geçmiş.
    ///
    /// Çizimde bölüm paneli, testlerde tür süzgeci kanıtları buradan okur —
    /// iki tüketici de aynı kurulumu görür. Bölümlerin girdisi kökün
    /// durumudur; alan entity'si **okunmaz** (port kapısı yalnız portun
    /// varlığına bakar), bu yüzden panelin çizimi alanı kirletmez.
    pub fn tezgah_bölümleri(
        &mut self,
        pencere: &mut Window,
        bağlam: &mut Context<Self>,
    ) -> Vec<TezgahBölümü> {
        let Some(alanlar) = self.sergi_girişlerini_al(pencere, bağlam) else {
            // Sergi alanları kurulamadı: exact typed sonuç durumda durur,
            // bölüm listesi boş döner — yarım alan kümesiyle çizim yapılmaz.
            return Vec::new();
        };
        // Kartın dilim okuması eşitlemeden geçer: masaüstü portu bildirimi
        // tazelik penceresi dolunca kendiliğinden yeniler; kök yalnız
        // tercih/port olaylarında eşitlenseydi kart yeni platform dilimini
        // gösterirken `ORT-021` kökü eskide kalabilirdi. Değişim yoksa bu
        // çağrı tek port okumasıdır.
        let saat_dilimi = self.yerel_kökü_eşitle(bağlam);
        let dilim_seçenekleri = self.metin_hizmetleri.dilim_seçenekleri();
        let doldurma_var = self.otomatik_doldurma_kullanılabilir(bağlam);
        let portlar = self.port_durumu(bağlam);
        // `§29` raporu tercih sürümüne bağlıdır: kart sonucu okur.
        let rapor = self.tezgah_raporu();
        // Tercih yalnız okunur: klonlanmaz, doğrudan ödünç verilir.
        metin_girisi_profili::bölümler(
            metin_girisi_profili::BölümGirdisi {
                tercih: &self.tezgah,
                alanlar: &alanlar,
                saat_dilimi: &saat_dilimi,
                dilim_seçenekleri: &dilim_seçenekleri,
                yerel_kök_hatası: self.yerel_kök_hatası(),
                doldurma_var,
                portlar,
                sayısal: self.tezgah.sayısal_mı(),
                rapor: &rapor,
            },
            bağlam,
        )
    }
}

impl GaleriUygulaması {
    pub fn köşe_izi(&self) -> Rc<Cell<gpui::Bounds<gpui::Pixels>>> {
        self.köşe_izi.clone()
    }

    pub fn köşe_sürükleniyor_mu(&self) -> bool {
        self.köşe_sürükleniyor
    }

    /// Tutamağı basılı duruma alır ya da bırakır.
    pub fn köşe_sürüklemesini_ayarla(&mut self, basılı: bool) {
        self.köşe_sürükleniyor = basılı;
    }

    /// `ORT-003` çubuk konumunu piksele çevirip tercihe yazar.
    pub fn köşe_yarıçapını_konumdan_ayarla(
        &mut self,
        x: gpui::Pixels,
        en_fazla: f32,
        bağlam: &mut Context<Self>,
    ) {
        let sınırlar = self.köşe_izi.get();
        let genişlik = f32::from(sınırlar.size.width);
        if genişlik <= 0. {
            return;
        }
        let oran = ((f32::from(x) - f32::from(sınırlar.origin.x)) / genişlik).clamp(0., 1.);
        let piksel = (oran * en_fazla).round();
        if self.tezgah.köşe_pikseli != Some(piksel) {
            self.tezgahı_değiştir(|t| t.köşe_pikseli = Some(piksel), bağlam);
        }
    }
}

impl Default for GaleriUygulaması {
    fn default() -> Self {
        Self::yeni()
    }
}

impl Render for GaleriUygulaması {
    fn render(&mut self, pencere: &mut Window, bağlam: &mut Context<Self>) -> impl IntoElement {
        crate::render_ölç(|| self.kök_gövdesi(pencere, bağlam))
    }
}

impl GaleriUygulaması {
    /// Kökün çizim gövdesi; `render` yalnız onu ölçerek sarar.
    fn kök_gövdesi(
        &mut self,
        pencere: &mut Window,
        bağlam: &mut Context<Self>,
    ) -> gpui::AnyElement {
        // Kare dikişleri **her iki yolda da** kurulur. Erken dönüşten sonra
        // kurmak, tezgâh ekranının onları hiç görmemesi demekti: palet ve
        // görünüm kendi fallback'leriyle ayakta kaldı ama açık seçici
        // fallback'siz olduğu için hiçbir liste açılmıyordu.
        self.kare_dikişlerini_kur();
        // Hedefli geçersizlemenin adresi; ilk çizimde yakalanır.
        self.pencere_tutamacı = Some(pencere.window_handle());

        // Uygulama tezgâh ekranıyla açılır. Tezgâh, galeri bilgi mimarisinin
        // içine gömülü bir aile sayfası **değildir**: tasarımın `§5`/`§6`
        // kabuğu kendi başlığını, bileşen seçicisini ve tema otoritesini
        // taşır. Galeri kataloğu kodda duruyor ama bu ekrandan açılmaz —
        // `YÖN-006 §3` bilgi mimarisinin akıbeti ayrı bir sözleşme
        // revizyonudur ve o karar verilene kadar kod silinmez.
        if self.tezgah_ekranı {
            return self.tezgah_ekranını_çiz(pencere, bağlam).into_any_element();
        }

        let p = palet();
        let kenarlık = rgb(p.kabuk_kenarlık);
        let ana_metin = rgb(p.kabuk_ana_metin);
        let ikincil_metin = rgb(p.kabuk_ikincil_metin);
        let vurgu = rgb(p.kabuk_vurgu);
        let genişlik = u32::from(pencere.viewport_size().width);
        let yerleşim = yerleşimi_çöz(genişlik, self.model.eksenler.metin_ölçeği);
        self.model.yerleşim = yerleşim;
        let Some(sergi_girişi) = self.sergi_girişlerini_al(pencere, bağlam) else {
            // Sergi alanları kurulamadı: katalog yerine exact typed sonuç
            // çizilir; yarım alan kümesiyle sergi kurulmaz.
            return div()
                .size_full()
                .p_5()
                .child(format!(
                    "Sergi giriş alanları kurulamadı: {:?}",
                    self.sergi_kuruluş_hatası
                ))
                .into_any_element();
        };
        let tezgah_tercihi = self.tezgah.clone();
        let dar = yerleşim == GaleriYerleşimKipi::Dar;
        let koyu_mu = self.galeri_kipi == gpui_bilesenleri::TemaKipi::Koyu;
        let hedef_etiketi = match self.model.hedef {
            GaleriHedefi::Masaüstü => "Masaüstü",
            GaleriHedefi::Wasm => "WASM",
        };
        let üst_araç_çubuğu = if dar {
            div().flex().flex_col().items_start().gap_3()
        } else {
            div().flex().flex_row().items_center().justify_between()
        }
        .id("üst-araç-çubuğu")
        .border_b_1()
        .border_color(kenarlık)
        .bg(rgb(p.kabuk_kart))
        .flex_shrink_0()
        .px_5()
        .py_3()
        .child(
            div()
                .flex()
                .flex_col()
                .when(dar, |başlık| başlık.w_full())
                .when(!dar, |başlık| başlık.w(px(360.)).flex_shrink_0())
                .child(div().text_lg().child("GPUI Bileşen Galerisi"))
                .child(
                    div()
                        .text_xs()
                        .text_color(ikincil_metin)
                        .child("Türkçe, erişilebilir ve platformdan bağımsız bileşen kitaplığı"),
                ),
        )
        .child(
            div()
                .flex()
                .flex_wrap()
                .items_center()
                .gap_2()
                .child(
                    div()
                        .border_1()
                        .border_color(kenarlık)
                        .rounded_md()
                        .px_3()
                        .py_2()
                        .text_sm()
                        .text_color(ikincil_metin)
                        .child("Bileşen ara"),
                )
                .child(
                    div()
                        .rounded_md()
                        .bg(rgb(p.kabuk_seçili_zemin))
                        .px_3()
                        .py_2()
                        .text_sm()
                        .text_color(vurgu)
                        .child(format!("{} bileşen", self.model.katalog.len())),
                )
                .children(GaleriTeması::TÜMÜ.map(|tema| {
                    let seçili = self.galeri_teması == tema;
                    div()
                        .id(format!("tema-{}", tema.adı()))
                        .cursor_pointer()
                        .rounded_md()
                        .border_1()
                        .border_color(if seçili { vurgu } else { kenarlık })
                        .bg(if seçili {
                            rgb(p.kabuk_seçili_zemin)
                        } else {
                            rgb(p.kabuk_kart)
                        })
                        .px_3()
                        .py_2()
                        .text_sm()
                        .text_color(if seçili { vurgu } else { ana_metin })
                        .child(tema.adı())
                        .on_click(bağlam.listener(move |bu, _, _, bağlam| {
                            bu.galeri_temasını_seç(tema, bağlam);
                        }))
                }))
                .child(
                    div()
                        .id("tema-kipi")
                        .cursor_pointer()
                        .rounded_md()
                        .border_1()
                        .border_color(kenarlık)
                        .bg(rgb(p.kabuk_kart))
                        .px_3()
                        .py_2()
                        .text_sm()
                        .text_color(ana_metin)
                        .child(if koyu_mu { "Koyu" } else { "Açık" })
                        .on_click(bağlam.listener(|bu, _, _, bağlam| {
                            bu.galeri_kipini_değiştir(bağlam);
                        })),
                )
                .child(
                    div()
                        .rounded_md()
                        .bg(rgb(p.kabuk_zemin))
                        .px_3()
                        .py_2()
                        .text_sm()
                        .text_color(ikincil_metin)
                        .child(format!("TR · {hedef_etiketi}")),
                ),
        );

        let aileler: Vec<(GaleriKategorisi, _)> = self
            .model
            .katalog
            .iter()
            .map(|kayıt| {
                let kategori = kayıt.kategori;
                (kategori, {
                    let sözleşme = Arc::clone(&kayıt.sözleşme);
                    div()
                        .id(format!("aile-{}", kayıt.sözleşme))
                        .border_1()
                        .border_color(kenarlık)
                        .rounded_lg()
                        .bg(rgb(p.kabuk_kart))
                        .w(px(210.))
                        .p_4()
                        .cursor_pointer()
                        .on_click(bağlam.listener(move |bu, _, _, bağlam| {
                            if bu.model.aileyi_aç(Arc::clone(&sözleşme)) {
                                bu.orta_kaydırma.set_offset(point(px(0.), px(0.)));
                                bağlam.notify();
                            }
                        }))
                        .child(
                            div()
                                .text_sm()
                                .child(aile_görünen_adı(kayıt.sözleşme.as_ref())),
                        )
                        .child(
                            div()
                                .mt_2()
                                .text_xs()
                                .text_color(ikincil_metin)
                                .child(aile_açıklaması(kayıt.sözleşme.as_ref())),
                        )
                })
            })
            .collect();
        let seçili_gezinti = self.model.seçili_aile.clone();
        let aile_gezintisi: Vec<_> = self
            .model
            .katalog
            .iter()
            .map(|kayıt| {
                let sözleşme = Arc::clone(&kayıt.sözleşme);
                let etkin = seçili_gezinti.as_ref() == Some(&kayıt.sözleşme);
                div()
                    .id(format!("gezinti-{}", kayıt.sözleşme))
                    .cursor_pointer()
                    .rounded_md()
                    .bg(if etkin {
                        rgb(p.kabuk_seçili_zemin)
                    } else {
                        rgb(p.kabuk_kart)
                    })
                    .px_2()
                    .py_2()
                    .on_click(bağlam.listener(move |bu, _, _, bağlam| {
                        if bu.model.aileyi_aç(Arc::clone(&sözleşme)) {
                            bu.orta_kaydırma.set_offset(point(px(0.), px(0.)));
                            bağlam.notify();
                        }
                    }))
                    .child(
                        div()
                            .text_sm()
                            .text_color(if etkin { vurgu } else { ana_metin })
                            .child(aile_görünen_adı(kayıt.sözleşme.as_ref())),
                    )
            })
            .collect();
        let dar_aile_gezintisi: Vec<_> = self
            .model
            .katalog
            .iter()
            .map(|kayıt| {
                let sözleşme = Arc::clone(&kayıt.sözleşme);
                let etkin = seçili_gezinti.as_ref() == Some(&kayıt.sözleşme);
                div()
                    .id(format!("dar-gezinti-{}", kayıt.sözleşme))
                    .cursor_pointer()
                    .rounded_md()
                    .bg(if etkin {
                        rgb(p.kabuk_seçili_zemin)
                    } else {
                        rgb(p.kabuk_kart)
                    })
                    .px_2()
                    .py_2()
                    .on_click(bağlam.listener(move |bu, _, _, bağlam| {
                        if bu.model.aileyi_aç(Arc::clone(&sözleşme)) {
                            bu.dar_aile_listesi_açık = false;
                            bu.orta_kaydırma.set_offset(point(px(0.), px(0.)));
                            bağlam.notify();
                        }
                    }))
                    .child(
                        div()
                            .text_sm()
                            .text_color(if etkin { vurgu } else { ana_metin })
                            .child(aile_görünen_adı(kayıt.sözleşme.as_ref())),
                    )
            })
            .collect();
        let dar_aile_listesi_açık = self.dar_aile_listesi_açık;
        let orta_içerik = if let Some(seçili) = self.model.seçili_aile.clone() {
            let medya = medya_fallback_planı(true);
            // Tezgâh içeriği yalnız `BİL-010` sergisi için gerekir; onu da
            // ekran yolundaki profil üretir, katalog ikinci kez kurmaz.
            let tezgah_içeriği = if seçili.as_ref() == "BİL-010" {
                Some(self.tezgah_profil_içeriği(pencere, bağlam))
            } else {
                None
            };
            let tezgah_çözücüsü = tezgah_içeriği.is_some().then(|| self.tezgah_çözücüsü());
            let canlı_sergi = sergiler::aile_sergisi(
                seçili.as_ref(),
                sergiler::SergiDurumu {
                    girişi: sergi_girişi.clone(),
                    tezgah: tezgah_tercihi,
                    tezgah_içeriği,
                    tezgah_çözücüsü,
                    düğme_sayacı: self.sergi_düğme_sayacı,
                    seçili: self.sergi_seçimi,
                    onaylı: self.sergi_onaylı,
                    sekme: self.sergi_sekmesi,
                    panel_açık: self.sergi_paneli_açık,
                    araç_taşması_açık: self.sergi_araç_taşması_açık,
                    modal_açık: self.sergi_modali_açık,
                    seçici_sonucu: self.sergi_seçici_sonucu,
                    tablo_azalan: self.sergi_tablo_azalan,
                    bildirim_açık: self.sergi_bildirimi_açık,
                    form_gönderildi: self.sergi_form_gönderildi,
                    sürekli_değer: self.sergi_sürekli_değer,
                    ilerleme: self.sergi_ilerleme,
                    takvim_günü: self.sergi_takvim_günü,
                    disclosure_açık: self.sergi_disclosure_açık,
                    renk_seçimi: self.sergi_renk_seçimi,
                    aktarım: self.sergi_aktarım,
                    arama_eşleşmesi: self.sergi_arama_eşleşmesi,
                    kısayol_değiştirildi: self.sergi_kısayol_değiştirildi,
                    ayar_koyu: self.sergi_ayar_koyu,
                    bağlantı_başarılı: self.sergi_bağlantı_başarılı,
                    kod_satırı: self.sergi_kod_satırı,
                    yüzen_grup_açık: self.sergi_yüzen_grup_açık,
                    gezinme_hedefi: self.sergi_gezinme_hedefi,
                    görsel_konumu: self.sergi_görsel_konumu,
                    kod_sembolü_qr: self.sergi_kod_sembolü_qr,
                    medya_niyeti: self.sergi_medya_niyeti,
                    ort_durumları: self.sergi_ort_durumları,
                    kab_durumları: self.sergi_kab_durumları,
                },
                bağlam,
            );
            // `BİL-010` tezgâhı kendi başlığını ve çerçevesini taşıyor;
            // üstüne bir de galeri başlığı, açıklama satırı ve sergi kartı
            // koymak aynı bilgiyi iki kez yazmak olur. Bu ailede tezgâh üst
            // barın hemen altından başlar; geri dönüş soldaki aile listesi.
            let çıplak_tezgah = seçili.as_ref() == "BİL-010";
            div()
                .when(!çıplak_tezgah, |kök| {
                    kök.child(
                        div()
                            .id("genel-bakışa-dön")
                            .flex()
                            .w(px(148.))
                            .cursor_pointer()
                            .rounded_md()
                            .border_1()
                            .border_color(kenarlık)
                            .bg(rgb(p.kabuk_kart))
                            .px_3()
                            .py_2()
                            .text_sm()
                            .text_color(vurgu)
                            .child("← Tüm bileşenler")
                            .on_click(bağlam.listener(|bu, _, _, bağlam| {
                                bu.model.genel_bakışa_dön();
                                bu.orta_kaydırma.set_offset(point(px(0.), px(0.)));
                                bağlam.notify();
                            })),
                    )
                    .child(
                        div()
                            .mt_4()
                            .text_xl()
                            .child(aile_görünen_adı(seçili.as_ref())),
                    )
                    .child(
                        div()
                            .mt_1()
                            .text_sm()
                            .text_color(ikincil_metin)
                            .child(aile_açıklaması(seçili.as_ref())),
                    )
                    .child(div().mt_5().text_lg().child("Sergiler"))
                })
                .child(
                    div()
                        .when(!çıplak_tezgah, |kart| {
                            kart.mt_2()
                                .border_1()
                                .border_color(kenarlık)
                                .rounded_lg()
                                .bg(rgb(p.kabuk_kart))
                                .p_4()
                                .child(div().text_sm().child("Varsayılan · Varyantlar · Durumlar"))
                                .child(div().mt_2().text_xs().text_color(ikincil_metin).child(
                                    "Tema, profil ve locale eksenleri aynı kanonik modelden çözülür.",
                                ))
                        })
                        .child(canlı_sergi)
                        .when(seçili.as_ref() == "BİL-290", |sergi| {
                            sergi.child(
                                div()
                                    .id("bil-290-guvenli-fallback")
                                    .mt_4()
                                    .rounded_lg()
                                    .bg(rgb(p.kod_zemin))
                                    .p_5()
                                    .text_color(rgb(p.kabuk_kart))
                                    .child(div().text_lg().child("Medya önizlemesi"))
                                    .child(div().mt_2().text_sm().child(
                                        "Bu galeri hedefinde doğrulanmış oynatma portu yok.",
                                    ))
                                    .child(div().mt_3().text_xs().child(format!(
                                        "Poster: {} · Güvenli açıklama: {} · Harici açma: {}",
                                        medya.poster, medya.güvenli_açıklama, medya.harici_açma,
                                    )))
                                    .child(div().mt_2().text_xs().child(format!(
                                        "Çalışan denetimler: {}",
                                        medya.çalışan_denetimler,
                                    ))),
                            )
                        }),
                )
                .child(div().mt_5().text_lg().child("Model ve API"))
                .child(div().mt_2().text_sm().text_color(ikincil_metin).child(
                    "Davranış kanonik sandıktadır; galeri yalnız yayımlanmış API'yi tüketir.",
                ))
                .child(div().mt_5().text_lg().child("Erişilebilirlik"))
                .child(
                    div()
                        .mt_2()
                        .text_sm()
                        .text_color(ikincil_metin)
                        .child("Odak, klavye ve adlandırma kabul ölçütlerine bağlıdır."),
                )
                .child(div().mt_5().text_lg().child("Platform yetenekleri"))
                .child(
                    div()
                        .mt_2()
                        .text_sm()
                        .text_color(ikincil_metin)
                        .child(format!("Etkin galeri hedefi: {hedef_etiketi}")),
                )
                .child(div().mt_5().text_lg().child("Profil eksenleri"))
                .child(
                    div()
                        .mt_2()
                        .text_sm()
                        .text_color(ikincil_metin)
                        .child("Açık tema · Temel profil · Rahat yoğunluk · tr locale"),
                )
                .child(div().mt_5().text_lg().child("Kanıt"))
                .child(
                    div()
                        .mt_2()
                        .text_sm()
                        .text_color(vurgu)
                        .child("Davranış kabul ölçütüne bağlıdır"),
                )
        } else {
            let öne_çıkan_sergiler = sergiler::öne_çıkan_sergiler(
                sergiler::SergiDurumu {
                    girişi: sergi_girişi.clone(),
                    tezgah: tezgah_tercihi,
                    // Genel bakış tezgâh gövdesini çizmez.
                    tezgah_içeriği: None,
                    tezgah_çözücüsü: None,
                    düğme_sayacı: self.sergi_düğme_sayacı,
                    seçili: self.sergi_seçimi,
                    onaylı: self.sergi_onaylı,
                    sekme: self.sergi_sekmesi,
                    panel_açık: self.sergi_paneli_açık,
                    araç_taşması_açık: self.sergi_araç_taşması_açık,
                    modal_açık: self.sergi_modali_açık,
                    seçici_sonucu: self.sergi_seçici_sonucu,
                    tablo_azalan: self.sergi_tablo_azalan,
                    bildirim_açık: self.sergi_bildirimi_açık,
                    form_gönderildi: self.sergi_form_gönderildi,
                    sürekli_değer: self.sergi_sürekli_değer,
                    ilerleme: self.sergi_ilerleme,
                    takvim_günü: self.sergi_takvim_günü,
                    disclosure_açık: self.sergi_disclosure_açık,
                    renk_seçimi: self.sergi_renk_seçimi,
                    aktarım: self.sergi_aktarım,
                    arama_eşleşmesi: self.sergi_arama_eşleşmesi,
                    kısayol_değiştirildi: self.sergi_kısayol_değiştirildi,
                    ayar_koyu: self.sergi_ayar_koyu,
                    bağlantı_başarılı: self.sergi_bağlantı_başarılı,
                    kod_satırı: self.sergi_kod_satırı,
                    yüzen_grup_açık: self.sergi_yüzen_grup_açık,
                    gezinme_hedefi: self.sergi_gezinme_hedefi,
                    görsel_konumu: self.sergi_görsel_konumu,
                    kod_sembolü_qr: self.sergi_kod_sembolü_qr,
                    medya_niyeti: self.sergi_medya_niyeti,
                    ort_durumları: self.sergi_ort_durumları,
                    kab_durumları: self.sergi_kab_durumları,
                },
                bağlam,
            );
            // Ant Design genel bakışı gibi: kategori başlığı + kart ızgarası.
            let mut kalan = aileler;
            let kategori_bölümleri: Vec<_> = self
                .model
                .kategoriler()
                .collect::<Vec<_>>()
                .into_iter()
                .map(|kategori| {
                    let (bu_kategori, geri_kalan): (Vec<_>, Vec<_>) = std::mem::take(&mut kalan)
                        .into_iter()
                        .partition(|(k, _)| *k == kategori);
                    kalan = geri_kalan;
                    let sayı = bu_kategori.len();
                    div()
                        .id(format!("kategori-{}", kategori.görünen_adı()))
                        .mt_6()
                        .child(
                            div()
                                .flex()
                                .items_baseline()
                                .gap_2()
                                .child(div().text_lg().child(kategori.görünen_adı()))
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(ikincil_metin)
                                        .child(format!("{sayı} bileşen")),
                                ),
                        )
                        .child(
                            div()
                                .mt_1()
                                .text_sm()
                                .text_color(ikincil_metin)
                                .child(kategori_açıklaması(kategori)),
                        )
                        .child(
                            div()
                                .mt_3()
                                .flex()
                                .flex_row()
                                .flex_wrap()
                                .gap_3()
                                .children(bu_kategori.into_iter().map(|(_, kart)| kart)),
                        )
                })
                .collect();
            div()
                .child(div().text_xl().child("Bileşenler"))
                .child(
                    div()
                        .mt_1()
                        .mb_4()
                        .text_sm()
                        .text_color(ikincil_metin)
                        .child("Kategoriye göz atın; her kart bileşenin canlı örneğine açılır."),
                )
                .child(öne_çıkan_sergiler)
                .children(kategori_bölümleri)
        };
        div()
            .id("galeri-kök")
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(p.kabuk_zemin))
            .text_color(ana_metin)
            .child(üst_araç_çubuğu)
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_1()
                    .min_h_0()
                    .when(!dar, |gövde| {
                        gövde.child(
                            div()
                                .id("sol-bileşen-gezintisi")
                                .w(px(224.))
                                .h_full()
                                .overflow_y_scroll()
                                .border_r_1()
                                .border_color(kenarlık)
                                .bg(rgb(p.kabuk_kart))
                                .p_4()
                                .child(
                                    div()
                                        .mb_3()
                                        .text_xs()
                                        .text_color(ikincil_metin)
                                        .child("KATEGORİLER"),
                                )
                                .children(self.model.kategoriler().map(|kategori| {
                                    let sayı = self
                                        .model
                                        .katalog
                                        .iter()
                                        .filter(|kayıt| kayıt.kategori == kategori)
                                        .count();
                                    div()
                                        .flex()
                                        .justify_between()
                                        .rounded_md()
                                        .px_2()
                                        .py_2()
                                        .text_sm()
                                        .child(kategori.görünen_adı())
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(ikincil_metin)
                                                .child(sayı.to_string()),
                                        )
                                }))
                                .child(
                                    div()
                                        .mt_5()
                                        .mb_2()
                                        .text_xs()
                                        .text_color(ikincil_metin)
                                        .child("AİLELER"),
                                )
                                .children(aile_gezintisi),
                        )
                    })
                    .child(
                        div()
                            .id("orta-belge-alanı")
                            .flex_1()
                            .min_w_0()
                            .overflow_y_scroll()
                            .track_scroll(&self.orta_kaydırma)
                            .p_5()
                            .when(dar, |orta| {
                                orta.child(
                                    div()
                                        .id("dar-gezinme-özeti")
                                        .mb_4()
                                        .border_1()
                                        .border_color(kenarlık)
                                        .rounded_lg()
                                        .bg(rgb(p.kabuk_kart))
                                        .p_3()
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .justify_between()
                                                .child(
                                                    div()
                                                        .text_sm()
                                                        .text_color(vurgu)
                                                        .child("Bileşen aileleri"),
                                                )
                                                .child(
                                                    div()
                                                        .id("dar-aile-listesi-geçişi")
                                                        .cursor_pointer()
                                                        .rounded_md()
                                                        .border_1()
                                                        .border_color(kenarlık)
                                                        .px_3()
                                                        .py_2()
                                                        .text_sm()
                                                        .child(if dar_aile_listesi_açık {
                                                            "Listeyi kapat"
                                                        } else {
                                                            "Bileşenleri aç"
                                                        })
                                                        .on_click(bağlam.listener(
                                                            |bu, _, _, bağlam| {
                                                                bu.dar_aile_listesi_açık =
                                                                    !bu.dar_aile_listesi_açık;
                                                                bağlam.notify();
                                                            },
                                                        )),
                                                ),
                                        )
                                        .child(
                                            div()
                                                .mt_1()
                                                .text_xs()
                                                .text_color(ikincil_metin)
                                                .child("Bileşenleri adlarıyla açın."),
                                        )
                                        .when(dar_aile_listesi_açık, |özet| {
                                            özet.child(
                                                div()
                                                    .id("dar-aile-listesi")
                                                    .mt_3()
                                                    .max_h(px(360.))
                                                    .overflow_y_scroll()
                                                    .border_t_1()
                                                    .border_color(kenarlık)
                                                    .pt_2()
                                                    .children(dar_aile_gezintisi),
                                            )
                                        }),
                                )
                            })
                            .child(orta_içerik),
                    ),
            )
            .into_any_element()
    }
}

/// `BİL-010` yetenek sergisinin yaşayan alanları.
///
/// Her alan sözleşmenin ayrı bir yetenek ekseninde nasıl davrandığını
/// gösterir; hepsi aynı kanonik `GirişKutusu` bileşenidir.
#[derive(Clone)]
pub struct MetinGirişiAlanları {
    /// Yalın alan: yer tutucu, temizleme yuvası, sayaç ve uzunluk sınırı.
    pub yalın: Entity<GirişKutusu>,
    /// Arama alanı: arama ve temizleme yuvaları birlikte.
    pub arama: Entity<GirişKutusu>,
    /// Parola alanı: gizli içerik ve göster/gizle yuvası.
    pub parola: Entity<GirişKutusu>,
    /// Access kod kümeli metin maskesi: `\0(000) 000 00 00`.
    pub maskeli: Entity<GirişKutusu>,
    /// Tarih maskesi ve seçici yuvası.
    pub tarih: Entity<GirişKutusu>,
    /// Para alanı: `§6` ön/son ek yuvaları ve `ParaBirimi` değer türü.
    pub tutar: Entity<GirişKutusu>,
    /// `§20` salt okunur alan: odak ve seçim var, mutasyon yok.
    pub salt_okunur: Entity<GirişKutusu>,
    /// Tezgâhın maske deseni girdisi.
    pub desen: Entity<GirişKutusu>,
    /// Tezgâhın ön ek metni girdisi.
    pub ön_ek_metni: Entity<GirişKutusu>,
    /// Tezgâhın son ek metni girdisi.
    pub son_ek_metni: Entity<GirişKutusu>,
    /// Bu girdileri tezgâh tercihine bağlayan abonelikler.
    ///
    /// `Arc` yalnız `MetinGirişiAlanları`nın klonlanabilir kalması içindir;
    /// abonelik yaşam süresi galeri görünümüne bağlıdır.
    _abonelikler: Arc<[gpui::Subscription]>,
    /// `§29` kuruluş sonuçlarının kayıpsız kaydı: her alanın uyarı raporu
    /// ve varsayılan sağlayıcı akıbeti. Sergi alanları açık başlangıç
    /// metniyle kurulduğu için sağlayıcı ekseni beklenen durumda `None`dır;
    /// eksen yine de taşınır, sessizce düşürülmez.
    pub kuruluş_notları: Arc<[SergiKuruluşNotu]>,
}

/// Bir sergi alanının `GirişKuruluşSonucu` eksenleri (bileşen dışı).
#[derive(Clone, Debug)]
pub struct SergiKuruluşNotu {
    pub alan: &'static str,
    pub rapor: gpui_bilesenleri::GirişYapılandırmaRaporu,
    pub varsayılan_değer_hatası: Option<gpui_bilesenleri::VarsayılanDeğerHatası>,
}

/// Maske tanımlama alanında kullanılabilecek hazır desenler.
///
/// Excel/Access kod kümesi: `0` zorunlu rakam, `9` isteğe bağlı rakam,
/// `L` zorunlu harf, `?` isteğe bağlı harf, `A` harf veya rakam,
/// `&` herhangi grafem, `>` büyüt, `<` küçült, `\` kaçış, `"…"` blok sabit.
pub const HAZIR_DESENLER: &[(&str, &str)] = &[
    // Baştaki `0` numaranın değişmez ön eki: `\` kaçışıyla sabit çizilir,
    // kullanıcı yazmaz (kullanıcı kararı, Ağu 2026). Tezgâhın varsayılan
    // desenidir; ürün geliştiricisi kendi desenini kurar.
    ("Telefon", "\\0(000) 000 00 00"),
    ("Tarih", "00/00/0000"),
    ("Plaka", ">00 L?? 00999"),
    ("IBAN", r#""TR"00 0000 0000 0000 0000 0000 00"#),
    ("Vergi no", "0000000000"),
    ("Saat", "00:00"),
];

/// Kullanıcının yazdığı deseni `§9.1` metin maskesine çevirir.
///
/// Boş desen maskeyi kaldırır; geçersiz desen `None` döner ve alan
/// maskesiz kalır.
pub fn deseni_maskeye_çevir(desen: &str) -> Option<GirişMaskesi> {
    let desen = desen.trim();
    if desen.is_empty() {
        return None;
    }
    Some(GirişMaskesi::Metin(MetinGirişMaskesi {
        desen: desen.to_owned().into(),
        yer_tutucu_grafemi: "_".into(),
        sabitleri_göster: true,
    }))
}

impl MetinGirişiAlanları {
    /// Tema değişince **her** tercih kutusunu tazeler.
    ///
    /// Kutular kuruluşta `galeri_teması()` alıyor ve o temayı kendi
    /// içlerinde saklıyor; kip değişince kendiliğinden yenilenmiyorlar.
    /// Yalnız önizleme alanı tazeleniyordu, o yüzden koyu kipte ön ek ve
    /// son ek kutuları açık kipte kalıyordu.
    fn temayı_değiştir(&self, bağlam: &mut Context<GaleriUygulaması>) {
        let tema = galeri_teması();
        for kutu in self.kutular() {
            let tema = tema.clone();
            kutu.update(bağlam, |kutu, bağlam| {
                kutu.temayı_değiştir(tema, bağlam)
            });
        }
    }

    /// Bütün yaşayan sergi kutuları; toplu tema/yerel güncellemeleri gezer.
    fn kutular(&self) -> [&Entity<GirişKutusu>; 10] {
        [
            &self.yalın,
            &self.arama,
            &self.parola,
            &self.maskeli,
            &self.tarih,
            &self.tutar,
            &self.salt_okunur,
            &self.desen,
            &self.ön_ek_metni,
            &self.son_ek_metni,
        ]
    }

    /// Host yerel kökü değişince **bütün** kutulara yeni bağlamı indirir.
    ///
    /// Bağlam host fabrikasının ürünüdür; kutu üzerinde alan alan mutasyon
    /// yapılmaz, yaşayan bağlam bileşenin atomik yüzeyiyle bütün olarak
    /// değiştirilir (yerel-türevli planlar bileşen atomunda yenilenir,
    /// metin damgası sabit kalır). Böylece hiçbir kutu eski kökü sessizce
    /// kullanmaz.
    fn yerel_bağlamı_değiştir(
        &self,
        kök: &Arc<gpui_bilesenleri::YerelMetinBağlamı>,
        bağlam: &mut Context<GaleriUygulaması>,
    ) -> Option<gpui_bilesenleri::GirişHatası> {
        // Atomik yüzey: yerel-türevli planlar bileşenin kendi atomunda
        // yenilenir, güncel kutu için erken döner. Ret typed döndürülür ve
        // çağıran (kök) yuvasında gözlenebilir tutar; ret alan kutuyu eski
        // (tutarlı) bağlamda bırakır.
        let mut son_ret = None;
        for kutu in self.kutular() {
            let yeni = (**kök).clone();
            if let Err(hata) = kutu.update(bağlam, |kutu, bağlam| {
                kutu.yerel_bağlamı_değiştir(yeni, bağlam)
            }) {
                son_ret = Some(hata);
            }
        }
        son_ret
    }

    /// Sergi kutularını `BİL-010 §29` fallible kuruluş yolundan kurar.
    ///
    /// Herhangi bir alanın kuruluşu düşerse exact hata döner; o ana kadar
    /// kurulan entity tutamaçları ve abonelikler erken dönüşle düşer
    /// (GPUI aboneliği drop'ta çözülür) ve yarım bir alan kümesi
    /// yaşatılmaz.
    fn kur(
        hizmetler: &MetinHizmetleriKökü,
        kimlik_fabrikası: &ÖrnekKimliğiFabrikası,
        pencere: &mut Window,
        bağlam: &mut Context<GaleriUygulaması>,
    ) -> Result<Self, gpui_bilesenleri::GirişKuruluşHatası> {
        let tema = galeri_teması();
        let katalog = galeri_simge_kataloğu();
        let yerel_kök = hizmetler.yerel_kök();
        // `§29` kuruluş eksenleri kayıpsız toplanır (rapor + varsayılan
        // sağlayıcı akıbeti); `RefCell` yalnız iki kurucu kapanışın aynı
        // listeye yazabilmesi içindir.
        let notlar = std::cell::RefCell::new(Vec::new());

        let alan = |kimlik: &'static str,
                    mut yapılandırma: GirişYapılandırması,
                    metin: &'static str,
                    pencere: &mut Window,
                    bağlam: &mut Context<GaleriUygulaması>|
         -> Result<Entity<GirişKutusu>, gpui_bilesenleri::GirişKuruluşHatası> {
            // `ORT-009` adsız alan erişilebilir ağaca girmez; sergi alanları
            // da adlı kurulur.
            if yapılandırma.erişilebilir_ad.is_none() {
                yapılandırma.erişilebilir_ad = Some(hazır_ileti("Sergi giriş alanı"));
            }
            let tema = tema.clone();
            let katalog = katalog.clone();
            let yerel_kök = Arc::clone(&yerel_kök);
            let (ad_alanı, yerel_ad) = kimlik
                .rsplit_once('.')
                .expect("galeri giriş tanımı ad alanı ve yerel ad taşır");
            let bileşen = galeri_bileşen_kimliği(kimlik_fabrikası, ad_alanı, yerel_ad);
            let sonuç = GirişKutusu::kur(
                bileşen,
                hizmetler.unicode(),
                hizmetler.alan_damgası(kimlik_fabrikası),
                // Yaşayan yerel bağlam kuruluşta host kökünden enjekte
                // edilir; kutu kendi kökünü kurmaz.
                (*yerel_kök).clone(),
                yapılandırma,
                metin,
                tema,
                pencere,
                bağlam,
            )?;
            notlar.borrow_mut().push(SergiKuruluşNotu {
                alan: kimlik,
                rapor: sonuç.rapor,
                varsayılan_değer_hatası: sonuç.varsayılan_değer_hatası,
            });
            let alan = sonuç.bileşen;
            alan.update(bağlam, |alan, _| {
                alan.simge_kataloğu = Some(katalog);
            });
            Ok(alan)
        };

        let yalın = {
            let mut y = GirişYapılandırması::tek_satırlı_metin();
            let yardımcı_kimlikleri = YardımcıKimlikleri::yeni(kimlik_fabrikası);
            y.yer_tutucu = Some(hazır_ileti("Bir bileşen adı yazın…"));
            y.yardımcı_eylem = Some(YardımcıEylemYuvası::kademeli(
                yardımcı_kimlikleri.al(&YardımcıEylemTürü::Temizle),
                YardımcıEylemTürü::Temizle,
            ));
            y.uzunluk_sınırı = Some(UzunlukSınırı {
                en_fazla_grafem: 24,
                davranış: UzunlukSınırıDavranışı::Kırp,
            });
            y.sayaç = Some(SayaçYapılandırması {
                birim: SayımBirimi::Grafem,
                sınırı_göster: true,
            });
            alan("galeri.metin_girisi.yalın", y, "", pencere, bağlam)?
        };

        let arama = {
            let mut y = GirişYapılandırması::tek_satırlı_metin();
            let yardımcı_kimlikleri = YardımcıKimlikleri::yeni(kimlik_fabrikası);
            y.yer_tutucu = Some(hazır_ileti("Komut ara…"));
            y.yardımcı_eylemler = Some(Arc::from(
                [
                    YardımcıEylemYuvası::kademeli(
                        yardımcı_kimlikleri.al(&YardımcıEylemTürü::Temizle),
                        YardımcıEylemTürü::Temizle,
                    ),
                    YardımcıEylemYuvası::her_zaman(
                        yardımcı_kimlikleri.al(&YardımcıEylemTürü::AramayıBaşlat),
                        YardımcıEylemTürü::AramayıBaşlat,
                    ),
                ]
                .as_slice(),
            ));
            alan("galeri.metin_girisi.arama", y, "", pencere, bağlam)?
        };

        let parola = {
            let mut y = GirişYapılandırması::tek_satırlı_metin();
            let yardımcı_kimlikleri = YardımcıKimlikleri::yeni(kimlik_fabrikası);
            y.yer_tutucu = Some(hazır_ileti("Parola"));
            y.içerik_görünürlüğü = İçerikGörünürlüğü::Gizli {
                maske_grafemi: "•".into(),
            };
            y.yardımcı_eylemler = Some(Arc::from(
                [
                    YardımcıEylemYuvası::kademeli(
                        yardımcı_kimlikleri.al(&YardımcıEylemTürü::Temizle),
                        YardımcıEylemTürü::Temizle,
                    ),
                    YardımcıEylemYuvası::her_zaman(
                        yardımcı_kimlikleri.al(&YardımcıEylemTürü::ParolayıGöster),
                        YardımcıEylemTürü::ParolayıGöster,
                    ),
                ]
                .as_slice(),
            ));
            alan(
                "galeri.metin_girisi.parola",
                y,
                "gizli-değer",
                pencere,
                bağlam,
            )?
        };

        let maskeli = {
            let mut y = GirişYapılandırması::tek_satırlı_metin();
            let yardımcı_kimlikleri = YardımcıKimlikleri::yeni(kimlik_fabrikası);
            y.yer_tutucu = Some(hazır_ileti("0(5__) ___ __ __"));
            y.maske = Some(GirişMaskesi::Metin(MetinGirişMaskesi {
                desen: "\\0(000) 000 00 00".into(),
                yer_tutucu_grafemi: "_".into(),
                sabitleri_göster: true,
            }));
            y.yardımcı_eylem = Some(YardımcıEylemYuvası::kademeli(
                yardımcı_kimlikleri.al(&YardımcıEylemTürü::Temizle),
                YardımcıEylemTürü::Temizle,
            ));
            alan("galeri.metin_girisi.maskeli", y, "", pencere, bağlam)?
        };

        let tarih = {
            let mut y = GirişYapılandırması::tek_satırlı_metin();
            let yardımcı_kimlikleri = YardımcıKimlikleri::yeni(kimlik_fabrikası);
            y.giriş_türü =
                TezgahDeğerKipi::Tarih.kanonik_tür(gpui_bilesenleri::MetinİçerikTürü::Düz);
            y.yer_tutucu = Some(hazır_ileti("gg.aa.yyyy"));
            y.maske = Some(GirişMaskesi::Tarih(TarihGirişMaskesi {
                desen: "gg.aa.yyyy".into(),
                // Takvim kimliği elle mühürlenmez; hostun yaşayan yerel
                // kökünün motor-doğrulanmış takvimi kullanılır.
                takvim: hizmetler.yerel_kök().takvim().clone(),
                eksik_giriş: Some(EksikGirişPolitikası::İzinVer),
                rakam_kümesi: Some(RakamKümesi::Latin),
                bölüm_gezinimi: None,
            }));
            y.yardımcı_eylem = Some(YardımcıEylemYuvası::her_zaman(
                yardımcı_kimlikleri.al(&YardımcıEylemTürü::SeçiciyiAç),
                YardımcıEylemTürü::SeçiciyiAç,
            ));
            alan("galeri.metin_girisi.tarih", y, "", pencere, bağlam)?
        };

        let tutar = {
            let mut y = GirişYapılandırması::tek_satırlı_metin();
            // `§6` para tür değildir: sergi tutar alanı ondalık türde,
            // para anlamı ön ekte kalır (biçim profili tezgâhın işidir).
            y.giriş_türü =
                TezgahDeğerKipi::Ondalık.kanonik_tür(gpui_bilesenleri::MetinİçerikTürü::Düz);
            y.ön_ek = Some(Sabitİçerik::metin("₺", false));
            y.son_ek = Some(Sabitİçerik::metin("KDV dahil", false));
            alan("galeri.metin_girisi.tutar", y, "", pencere, bağlam)?
        };

        // Tezgâhın metin girdileri. Üçü de kanonik `GirişKutusu`dır: galeri
        // kendi giriş kutusunu çizmez, tercihi de bunların metninden okur.
        let mut metin_tercihi =
            |kimlik: &'static str,
             yer_tutucu: &'static str,
             başlangıç: &'static str,
             bağlam: &mut Context<GaleriUygulaması>| {
                let mut y = GirişYapılandırması::tek_satırlı_metin();
                let yardımcı_kimlikleri = YardımcıKimlikleri::yeni(kimlik_fabrikası);
                y.yer_tutucu = Some(hazır_ileti(yer_tutucu));
                y.yardımcı_eylem = Some(YardımcıEylemYuvası::kademeli(
                    yardımcı_kimlikleri.al(&YardımcıEylemTürü::Temizle),
                    YardımcıEylemTürü::Temizle,
                ));
                alan(kimlik, y, başlangıç, pencere, bağlam)
            };
        // Başlangıç metinleri tercihin varsayılanıyla aynı sabitten gelir.
        // İki yerde ayrı yazıldığında ekranda "₺" görünürken kutu "+90"
        // bekliyordu.
        let desen = metin_tercihi(
            "galeri.metin_girisi.desen",
            "Maske deseni: \\0(000) 000 00 00",
            HAZIR_DESENLER[0].1,
            bağlam,
        )?;
        let ön_ek_metni = metin_tercihi(
            "galeri.metin_girisi.ön-ek",
            "Ön ek metni",
            metin_girisi_tezgahi::VARSAYILAN_ÖN_EK,
            bağlam,
        )?;
        let son_ek_metni = metin_tercihi(
            "galeri.metin_girisi.son-ek",
            "Son ek metni",
            metin_girisi_tezgahi::VARSAYILAN_SON_EK,
            bağlam,
        )?;

        // Bu alanların metni değiştikçe tezgâh tercihi ve önizleme yenilenir.
        let abone = |hedef: &Entity<GirişKutusu>,
                     yaz: fn(&mut TezgahTercihleri, String),
                     bağlam: &mut Context<GaleriUygulaması>| {
            bağlam.subscribe(hedef, move |bu, hedef, olay, bağlam| {
                if matches!(
                    olay,
                    gpui_bilesenleri::GirişOlayı::DüzenlemeMetniDeğişti { .. }
                ) {
                    // Bekleyen ileri eşitleme varken kutunun metni tercihe
                    // geri yazılmaz: birleşim sırasında seçilen hedef,
                    // birleşim metninin olayıyla sessizce geri alınıyordu.
                    // Önce ileri yön yeniden denenir (birleşim bittiyse
                    // hedef şimdi uygulanır ve kayıt düşer); tercih yazımı
                    // ancak bekleyen düştükten sonraki olaylarda sürer.
                    if bu.bekleyen_tercih_eşitlemeleri.contains(&hedef.entity_id()) {
                        bu.tercih_alanlarını_eşitle(bağlam);
                        return;
                    }
                    let metin = hedef.read(bağlam).metin().to_owned();
                    bu.tezgahı_değiştir(move |t| yaz(t, metin), bağlam);
                }
            })
        };
        let abonelikler = vec![
            abone(&desen, |t, m| t.desen = m, bağlam),
            abone(&ön_ek_metni, |t, m| t.ön_ek_metni = m, bağlam),
            abone(&son_ek_metni, |t, m| t.son_ek_metni = m, bağlam),
        ];

        let salt_okunur = {
            let mut y = GirişYapılandırması::tek_satırlı_metin();
            y.salt_okunur = true;
            alan(
                "galeri.metin_girisi.salt-okunur",
                y,
                "Seçilebilir, yazılamaz",
                pencere,
                bağlam,
            )?
        };

        Ok(Self {
            yalın,
            arama,
            parola,
            maskeli,
            tarih,
            tutar,
            salt_okunur,
            desen,
            ön_ek_metni,
            son_ek_metni,
            _abonelikler: abonelikler.into(),
            kuruluş_notları: notlar.into_inner().into(),
        })
    }
}

/// `ORT-018` ölçümünün sabit içeriği; masaüstü koşumuyla aynıdır.
const ÖLÇÜM_METNİ: &str = "5386977934";

/// Platformun yüksek çözünürlüklü saati, milisaniye.
///
/// Tarayıcıda `performance.now()`, masaüstünde `Instant`. İkisi de aynı
/// ölçüm noktasını okur; sayı iki hedefte karşılaştırılabilir kalır.
#[cfg(target_family = "wasm")]
fn şimdi_ms() -> f64 {
    web_sys::window()
        .and_then(|pencere| pencere.performance())
        .map_or(0.0, |saat| saat.now())
}

#[cfg(not(target_family = "wasm"))]
fn şimdi_ms() -> f64 {
    use std::time::Instant;
    thread_local! {
        static BAŞLANGIÇ: Instant = Instant::now();
    }
    BAŞLANGIÇ.with(|başlangıç| başlangıç.elapsed().as_secs_f64() * 1000.0)
}

pub(crate) fn hazır_ileti(metin: &str) -> gpui_bilesenleri::Kullanıcıİletisi {
    gpui_bilesenleri::Kullanıcıİletisi::Hazır(gpui_bilesenleri::GüvenliMetin::yeni(
        metin.to_owned(),
        false,
        true,
    ))
}

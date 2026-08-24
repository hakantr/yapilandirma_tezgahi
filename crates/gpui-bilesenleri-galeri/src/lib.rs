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
    BileşenKimliği, EksikGirişPolitikası, GirişKutusu, GirişMaskesi,
    GirişYapılandırması, MetinGirişMaskesi, RakamKümesi, Sabitİçerik, SayaçYapılandırması,
    SayımBirimi, TanımKimliği, TarihGirişMaskesi, UzunlukSınırı, UzunlukSınırıDavranışı,
    YardımcıEylemTürü, YardımcıEylemYuvası, medya_fallback_planı, ÖrnekKimliğiFabrikası,
    İçerikGörünürlüğü,
};
use std::{cell::Cell, rc::Rc, sync::Arc};

mod galeri;
mod metin_girisi_profili;
mod metin_girisi_tezgahi;
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
    OtomatikDoldurmaAmacı, OtomatikDoldurmaHatası, PlatformMetinİmleciTercihi,
    PlatformOtomatikDoldurmaPortu, PlatformSaatDilimiPortu, PlatformİmleçPortu, PlatformİzinDurumu,
    SaatDilimiKaynağı, SaatDilimiKimliği, SaatDilimiTercihi, metin_imleci_hareketini_çöz,
    saat_dilimini_çöz, ÇözülmüşMetinİmleciHareketi, ÇözülmüşSaatDilimi, İmleçTokenları,
};

pub use galeri::*;
pub use metin_girisi_profili::*;
pub use metin_girisi_tezgahi::*;
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
pub struct GaleriUygulaması {
    pub model: GaleriModeli,
    kimlik_fabrikası: ÖrnekKimliğiFabrikası,
    orta_kaydırma: ScrollHandle,
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
        Self {
            model: GaleriModeli::yerleşik_hedef(hedef),
            kimlik_fabrikası,
            orta_kaydırma: ScrollHandle::new(),
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
        gpui::div().size_full().child(sergiler::tezgah_ekranı(
            self.tezgah.clone(),
            içerik,
            sistem_aileleri,
            kabuk,
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

    /// Yeni paleti canlı alanlara da uygular.
    ///
    /// Palet kare başında kurulur ama `GirişKutusu` kendi anlık görüntüsünü
    /// saklar; tazelenmezse tezgâh kutusu eski renklerde kalırdı.
    fn temayı_tazele(&mut self, bağlam: &mut Context<Self>) {
        paleti_kur(galeri_paleti(self.galeri_teması, self.galeri_kipi));
        kabuk_görünümünü_kur(
            &self.tezgah.tema.yazı_ailesi,
            self.tezgah.tema.punto,
            self.tezgah.tema.metin_ölçeği,
            self.tezgah.tema.yoğunluk,
            self.tezgah.tema.hareket,
        );
        görünümü_kur(tasarım_görünümünü_çöz());
        açık_seçiciyi_kur(self.açık_seçici.clone());
        self.tezgah.tema.kip = self.galeri_kipi;
        self.tezgah.tema.sürümü_artır();
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
        bağlam.notify();
    }

    /// Platform bildirimlerini kurar.
    ///
    /// Tek çağrı olmasının nedeni sarmalayıcıyı sabit tutmak: her yeni port
    /// için sarmalayıcıya bir satır eklemek, davranışın oraya kaymasının ilk
    /// adımıdır. Politika buraya da girmez — öncelik sırası ve düşme kuralı
    /// her portun kendi çekirdek çözümündedir.
    pub fn platform_portlarını_kur(&mut self, portlar: PlatformPortları) {
        self.saat_dilimi_portu = portlar.saat_dilimi;
        self.imleç_portu = portlar.imleç;
        self.otomatik_doldurma_portu = portlar.otomatik_doldurma;
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
        let alan = self.tezgah_alanını_al(pencere, bağlam);
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
        // Port kapıları kartı `doğrulama_portu.is_some()` okur ve kök
        // çizimindedir. Kök artık alanı gözlemediği için buradaki değişim
        // açıkça bildirilir; sorunların kendisi panellerin işidir.
        bağlam.notify();
    }

    /// Tezgâh tercihine göre çözülmüş saat dilimi.
    pub fn çözülmüş_saat_dilimi(&self) -> ÇözülmüşSaatDilimi {
        saat_dilimini_çöz(
            &self.tezgah.saat_dilimi_tercihi,
            self.saat_dilimi_portu.as_deref(),
        )
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
        if self.tezgah.değer_türü != önceki_tür {
            self.tezgah_alanı = None;
            self.tezgah_yardımcı_kimlikleri = None;
        } else if let Some(alan) = self.tezgah_alanı.clone() {
            let kimlikler = self
                .tezgah_yardımcı_kimlikleri
                .as_ref()
                .expect("yaşayan tezgâh alanının yardımcı kimlikleri vardır");
            let yapılandırma = self.tezgah.yapılandırma_kimliklerle(kimlikler);
            let tema = tezgah_teması(&self.tezgah.kutu_teması());
            // `ORT-002 §5.2` dilim çözümü tercih değişince yeniden koşar ve
            // yeni bir bağlam sürümü üretir.
            let dilim = self.çözülmüş_saat_dilimi();
            let önem_zemini = self.tezgah.önem_zemini;
            alan.update(bağlam, |alan, bağlam| {
                alan.yapılandırmayı_değiştir(yapılandırma, bağlam);
                alan.temayı_değiştir(tema, bağlam);
                alan.yerel.saat_dilimi = dilim.bağlam_metni();
                alan.yerel.sürüm = alan.yerel.sürüm.saturating_add(1);
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
        let eşitle = |kutu: &Entity<GirişKutusu>, hedef: &str, bağlam: &mut Context<Self>| {
            kutu.update(bağlam, |kutu, bağlam| {
                if kutu.metin() == hedef {
                    return;
                }
                let uzunluk = kutu.metin().len();
                kutu.durum.metni_aralıkta_değiştir(0..uzunluk, hedef);
                bağlam.notify();
            });
        };
        let (desen, ön_ek, son_ek) = (
            self.tezgah.desen.clone(),
            self.tezgah.ön_ek_metni.clone(),
            self.tezgah.son_ek_metni.clone(),
        );
        eşitle(&alanlar.desen, &desen, bağlam);
        eşitle(&alanlar.ön_ek_metni, &ön_ek, bağlam);
        eşitle(&alanlar.son_ek_metni, &son_ek, bağlam);
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
        let alan = self.tezgah_alanını_al(pencere, bağlam);
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
        let alan = self.tezgah_alanını_al(pencere, bağlam);
        // Ölçüm içeriği sabitlenir: boş alanın kabulü dolu alanınkinden
        // ucuzdur ve iki hedefin sayısı karşılaştırılamaz hâle gelir.
        alan.update(bağlam, |alan, _| {
            alan.durum.ham_girişi_uygula(ÖLÇÜM_METNİ);
            alan.maske_şablonunu_kur();
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
    ) -> Entity<GirişKutusu> {
        if let Some(alan) = self.tezgah_alanı.clone() {
            return alan;
        }
        let yardımcı_kimlikleri = YardımcıKimlikleri::yeni(&self.kimlik_fabrikası);
        let yapılandırma = self.tezgah.yapılandırma_kimliklerle(&yardımcı_kimlikleri);
        // Önizleme seçili tercihi açılışta göstersin: boş kutu hizalamayı,
        // ayracı, gizlemeyi ve temizleme simgesini görünür kılmaz.
        let örnek = self.tezgah.örnek_değer();
        // Tezgâh kendi kâğıt paletini taşır; önizleme kutusu da o palete
        // göre çizilir ki ekranın bütünü tek bir tasarım dili konuşsun.
        let tema = tezgah_teması(&self.tezgah.kutu_teması());
        let dilim = self.çözülmüş_saat_dilimi();
        let imleç_portu = self.imleç_portu.clone();
        let doldurma_portu = self.otomatik_doldurma_portu.clone();
        let katalog = galeri_simge_kataloğu();
        let bileşen =
            galeri_bileşen_kimliği(&self.kimlik_fabrikası, "galeri.metin_girisi", "tezgah");
        let alan = bağlam.new(move |bağlam| {
            let mut alan = GirişKutusu::yeni(bileşen, yapılandırma, örnek, tema, pencere, bağlam);
            alan.simge_kataloğu = Some(katalog);
            alan.yerel.saat_dilimi = dilim.bağlam_metni();
            alan.imleç_portu = imleç_portu;
            alan.otomatik_doldurma_portu = doldurma_portu;
            alan
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
                    let alan = alan.clone();
                    bağlam.new(move |bağlam| YuvaNotuPaneli::yeni(kök, alan, bağlam))
                };
                let bölümler = {
                    let kök = bağlam.entity();
                    bağlam.new(move |bağlam| BölümlerPaneli::yeni(&kök, bağlam))
                };
                self.tezgah_panelleri = Some(TezgahPanelleri {
                    alan_durumu,
                    olay_akışı,
                    yuva_notu,
                    bölümler,
                });
            }
        }
        self.tezgah_alanı = Some(alan.clone());
        alan
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
        let rapor = Rc::new(self.tezgah.yapılandırma(&self.kimlik_fabrikası).doğrula());
        self.rapor_önbelleği = Some((self.tercih_sürümü, Rc::clone(&rapor)));
        rapor
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
        let alan = self.tezgah_alanını_al(pencere, bağlam);
        let paneller = self
            .tezgah_panelleri
            .clone()
            .expect("paneller alanla birlikte kurulur");
        let tercih = self.tezgah.clone();
        // Kod paneli metni tercih sürümüne bağlıdır: kart sonucu okur,
        // yeniden hesaplamaz. `§29` raporu da öyledir ama artık bu yolun
        // değil, bölüm panelinin girdisidir (`tezgah_bölümleri`).
        let kod = self.tezgah_kodu();
        // `ORT-003 §2` yarıçap kısa kenarın yarısını aşamaz; tek satırlı
        // alanda kısıtlayan kenar kutu yüksekliğidir.
        let en_fazla_yarıçap = f32::from(tezgah_teması(&tercih.tema).ölçüler.etkileşim_hedefi) / 2.;

        metin_girisi_profili::tezgah_içeriği(
            metin_girisi_profili::MetinGirişiProfilGirdisi {
                tercih: &tercih,
                alan,
                paneller: &paneller,
                kod,
                en_fazla_yarıçap,
                köşe_izi: self.köşe_izi.clone(),
            },
            bağlam,
        )
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
        let alanlar = match self.sergi_girişleri.clone() {
            Some(alanlar) => alanlar,
            None => {
                let alanlar = MetinGirişiAlanları::kur(&self.kimlik_fabrikası, pencere, bağlam);
                self.sergi_girişleri = Some(alanlar.clone());
                alanlar
            }
        };
        let tercih = self.tezgah.clone();
        let saat_dilimi = self.çözülmüş_saat_dilimi();
        let doldurma_var = self.otomatik_doldurma_kullanılabilir(bağlam);
        let portlar = self.port_durumu(bağlam);
        // `§29` raporu tercih sürümüne bağlıdır: kart sonucu okur.
        let rapor = self.tezgah_raporu();
        metin_girisi_profili::bölümler(
            metin_girisi_profili::BölümGirdisi {
                tercih: &tercih,
                alanlar: &alanlar,
                saat_dilimi: &saat_dilimi,
                doldurma_var,
                portlar,
                sayısal: tercih.sayısal_mı(),
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
        // Kare dikişleri **her iki yolda da** kurulur. Erken dönüşten sonra
        // kurmak, tezgâh ekranının onları hiç görmemesi demekti: palet ve
        // görünüm kendi fallback'leriyle ayakta kaldı ama açık seçici
        // fallback'siz olduğu için hiçbir liste açılmıyordu.
        paleti_kur(galeri_paleti(self.galeri_teması, self.galeri_kipi));
        // Kabuk görünümü `görünümü_kur`dan **önce** kurulur:
        // `tasarım_görünümünü_çöz` tema anlık görüntüsünü okur ve tipografi
        // ile yoğunluk oradan gelir.
        kabuk_görünümünü_kur(
            &self.tezgah.tema.yazı_ailesi,
            self.tezgah.tema.punto,
            self.tezgah.tema.metin_ölçeği,
            self.tezgah.tema.yoğunluk,
            self.tezgah.tema.hareket,
        );
        görünümü_kur(tasarım_görünümünü_çöz());
        açık_seçiciyi_kur(self.açık_seçici.clone());

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
        let sergi_girişi = match self.sergi_girişleri.clone() {
            Some(alanlar) => alanlar,
            None => {
                let alanlar = MetinGirişiAlanları::kur(&self.kimlik_fabrikası, pencere, bağlam);
                self.sergi_girişleri = Some(alanlar.clone());
                alanlar
            }
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
            let canlı_sergi = sergiler::aile_sergisi(
                seçili.as_ref(),
                sergiler::SergiDurumu {
                    girişi: sergi_girişi.clone(),
                    tezgah: tezgah_tercihi,
                    tezgah_içeriği,
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
        for kutu in [
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
        ] {
            let tema = tema.clone();
            kutu.update(bağlam, |kutu, bağlam| {
                kutu.temayı_değiştir(tema, bağlam)
            });
        }
    }

    fn kur(
        kimlik_fabrikası: &ÖrnekKimliğiFabrikası,
        pencere: &mut Window,
        bağlam: &mut Context<GaleriUygulaması>,
    ) -> Self {
        let tema = galeri_teması();
        let katalog = galeri_simge_kataloğu();

        let alan = |kimlik: &'static str,
                    mut yapılandırma: GirişYapılandırması,
                    metin: &'static str,
                    pencere: &mut Window,
                    bağlam: &mut Context<GaleriUygulaması>| {
            // `ORT-009` adsız alan erişilebilir ağaca girmez; sergi alanları
            // da adlı kurulur.
            if yapılandırma.erişilebilir_ad.is_none() {
                yapılandırma.erişilebilir_ad = Some(hazır_ileti("Sergi giriş alanı"));
            }
            let tema = tema.clone();
            let katalog = katalog.clone();
            let (ad_alanı, yerel_ad) = kimlik
                .rsplit_once('.')
                .expect("galeri giriş tanımı ad alanı ve yerel ad taşır");
            let bileşen = galeri_bileşen_kimliği(kimlik_fabrikası, ad_alanı, yerel_ad);
            bağlam.new(move |bağlam| {
                let mut alan =
                    GirişKutusu::yeni(bileşen, yapılandırma, metin, tema, pencere, bağlam);
                alan.simge_kataloğu = Some(katalog);
                alan
            })
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
            alan("galeri.metin_girisi.yalın", y, "", pencere, bağlam)
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
            alan("galeri.metin_girisi.arama", y, "", pencere, bağlam)
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
            )
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
            alan("galeri.metin_girisi.maskeli", y, "", pencere, bağlam)
        };

        let tarih = {
            let mut y = GirişYapılandırması::tek_satırlı_metin();
            let yardımcı_kimlikleri = YardımcıKimlikleri::yeni(kimlik_fabrikası);
            y.giriş_türü = TezgahDeğerKipi::Tarih.kanonik_tür(gpui_bilesenleri::MetinİçerikTürü::Düz);
            y.yer_tutucu = Some(hazır_ileti("gg.aa.yyyy"));
            y.maske = Some(GirişMaskesi::Tarih(TarihGirişMaskesi {
                desen: "gg.aa.yyyy".into(),
                takvim: gpui_bilesenleri::TakvimKimliği(Arc::from("gregory")),
                eksik_giriş: Some(EksikGirişPolitikası::İzinVer),
                rakam_kümesi: Some(RakamKümesi::Latin),
                bölüm_gezinimi: None,
            }));
            y.yardımcı_eylem = Some(YardımcıEylemYuvası::her_zaman(
                yardımcı_kimlikleri.al(&YardımcıEylemTürü::SeçiciyiAç),
                YardımcıEylemTürü::SeçiciyiAç,
            ));
            alan("galeri.metin_girisi.tarih", y, "", pencere, bağlam)
        };

        let tutar = {
            let mut y = GirişYapılandırması::tek_satırlı_metin();
            // `§6` para tür değildir: sergi tutar alanı ondalık türde,
            // para anlamı ön ekte kalır (biçim profili tezgâhın işidir).
            y.giriş_türü =
                TezgahDeğerKipi::Ondalık.kanonik_tür(gpui_bilesenleri::MetinİçerikTürü::Düz);
            y.ön_ek = Some(Sabitİçerik::metin("₺", false));
            y.son_ek = Some(Sabitİçerik::metin("KDV dahil", false));
            alan("galeri.metin_girisi.tutar", y, "", pencere, bağlam)
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
        );
        let ön_ek_metni = metin_tercihi(
            "galeri.metin_girisi.ön-ek",
            "Ön ek metni",
            metin_girisi_tezgahi::VARSAYILAN_ÖN_EK,
            bağlam,
        );
        let son_ek_metni = metin_tercihi(
            "galeri.metin_girisi.son-ek",
            "Son ek metni",
            metin_girisi_tezgahi::VARSAYILAN_SON_EK,
            bağlam,
        );

        // Bu alanların metni değiştikçe tezgâh tercihi ve önizleme yenilenir.
        let abone = |hedef: &Entity<GirişKutusu>,
                     yaz: fn(&mut TezgahTercihleri, String),
                     bağlam: &mut Context<GaleriUygulaması>| {
            bağlam.subscribe(hedef, move |bu, hedef, olay, bağlam| {
                if matches!(
                    olay,
                    gpui_bilesenleri::GirişOlayı::DüzenlemeMetniDeğişti { .. }
                ) {
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
            )
        };

        Self {
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
        }
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

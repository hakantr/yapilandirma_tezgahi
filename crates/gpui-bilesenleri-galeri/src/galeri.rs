use gpui::{Hsla, TextStyle, hsla, px, rgb};
use gpui_bilesenleri_kabuk::{
    ArayüzYoğunluğu, BağlamSürümü, BoşlukTokenları, GölgeTokenları, GölgeTokenı, HareketTercihi,
    HareketTokenları, KutuRenkleri, KöşeTokenları, OrtakKutuTemaRolleri, RenkTokenları,
    SemantikRenkRolleri, TemaAnlıkGörüntüsü, TemaBağlamı, TemaKimliği, TipografiTokenları,
    YerelleştirmeAnahtarı, ÖlçüTokenları, İletiArgümanı, İletiÇözümleyicisi, İletiİsteği,
};
// `ORT-021` mühürlü çözüm hizmeti ve snapshot yalnız temel sandıktan dışa
// açılır; sergi başlığı çözümü o yüzeyden geçer.
use gpui_bilesenleri_temel::{
    YerelMetinBağlamı, İletiKataloğuSnapshot, İletiÇözümHatası, İletiÇözümHizmeti,
};
use std::{collections::BTreeSet, sync::Arc, time::Duration};

pub const ORT_AİLELERİ: &[&str] = &[
    "ORT-001", "ORT-002", "ORT-003", "ORT-004", "ORT-005", "ORT-006", "ORT-007", "ORT-008",
    "ORT-009", "ORT-010", "ORT-011", "ORT-012", "ORT-013", "ORT-014", "ORT-015", "ORT-016",
    "ORT-017", "ORT-018", "ORT-019", "ORT-020", "ORT-021", "ORT-022", "ORT-023",
];
pub const BİL_AİLELERİ: &[&str] = &[
    "BİL-010", "BİL-020", "BİL-030", "BİL-040", "BİL-050", "BİL-060", "BİL-070", "BİL-080",
    "BİL-090", "BİL-100", "BİL-110", "BİL-120", "BİL-130", "BİL-140", "BİL-150", "BİL-160",
    "BİL-170", "BİL-180", "BİL-190", "BİL-200", "BİL-210", "BİL-220", "BİL-230", "BİL-250",
    "BİL-260", "BİL-270", "BİL-280", "BİL-290",
];
pub const KAB_AİLELERİ: &[&str] = &[
    "KAB-010", "KAB-020", "KAB-030", "KAB-040", "KAB-050", "KAB-060", "KAB-070", "KAB-080",
    "KAB-090", "KAB-100",
];

/// Teknik sözleşme kimliğini kullanıcıya yönelik aile adına çevirir.
/// Kimlik kanıt ve API yüzeyinde kalır; gezinmenin birincil etiketi değildir.
pub fn aile_görünen_adı(sözleşme: &str) -> &'static str {
    match sözleşme {
        "ORT-001" => "GPUI Proje Temelleri",
        "ORT-002" => "Unicode ve Yerel Metin",
        "ORT-003" => "Kutu Şekli",
        "ORT-004" => "Etkileşim Durumu ve Tema",
        "ORT-005" => "Odak ve Klavye",
        "ORT-006" => "Yüzen Yüzey ve Bağlam Menüsü",
        "ORT-007" => "Eşzamansız İş ve Sürümleme",
        "ORT-008" => "Değer Biçimlendirme",
        "ORT-009" => "Erişilebilirlik",
        "ORT-010" => "Sürükle ve Bırak",
        "ORT-011" => "Yeniden Boyutlandırma",
        "ORT-012" => "Koleksiyon ve Sanallaştırma",
        "ORT-013" => "Geri Alma ve Yineleme",
        "ORT-014" => "Otomatik Kaydetme ve Kurtarma",
        "ORT-015" => "Zengin Pano",
        "ORT-016" => "Simge Çözümleme",
        "ORT-017" => "Görünüm Profili ve Render",
        "ORT-018" => "Performans Bütçesi",
        "ORT-019" => "Hassas İçerik ve Tanı",
        "ORT-020" => "Ayar Kapsamı ve Depo",
        "ORT-021" => "Yerelleştirme ve İleti",
        "ORT-022" => "Komut Kataloğu",
        "ORT-023" => "Rota ve Gezinme Geçmişi",
        "BİL-010" => "Metin Girişi",
        "BİL-020" => "Seçim ve Liste",
        "BİL-030" => "Onay Kutusu ve Anahtar",
        "BİL-040" => "Düğme",
        "BİL-050" => "Sekmeler",
        "BİL-060" => "Panel",
        "BİL-070" => "Araç Çubuğu",
        "BİL-080" => "Modal ve Dialog",
        "BİL-090" => "Seçici",
        "BİL-100" => "Veri Tablosu",
        "BİL-110" => "Bildirim",
        "BİL-120" => "Form",
        "BİL-130" => "Slider ve Aralık",
        "BİL-140" => "Durum ve İlerleme",
        "BİL-150" => "Takvim ve Tarih Seçimi",
        "BİL-160" => "Yapısal Sunum",
        "BİL-170" => "Renk Seçici",
        "BİL-180" => "Dosya Aktarımı",
        "BİL-190" => "Belge İçi Arama",
        "BİL-200" => "Kısayol Düzenleyici",
        "BİL-210" => "Ayar Kataloğu",
        "BİL-220" => "Veri Kaynağı Bağlantısı",
        "BİL-230" => "Kod ve Sözdizimi Görünümü",
        "BİL-250" => "Yüzen Eylem Düğmesi",
        "BİL-260" => "Gezinme",
        "BİL-270" => "Görsel Sunum",
        "BİL-280" => "Kod Sembolü",
        "BİL-290" => "Medya Oynatma",
        "KAB-010" => "Dock Konağı",
        "KAB-020" => "Alt Çalışma Alanı",
        "KAB-030" => "Pencere Kromu ve Başlık",
        "KAB-040" => "Durum Çubuğu",
        "KAB-050" => "Pencere Yaşam Döngüsü",
        "KAB-060" => "Bölünmüş Görünüm",
        "KAB-070" => "Uygulama Menüsü",
        "KAB-080" => "Çoklu Pencere ve Yerleşim",
        "KAB-090" => "Oturum Kurtarma ve Güvenli Mod",
        "KAB-100" => "Veri Konumları ve Gizli Saklama",
        _ => "Bilinmeyen Aile",
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum GaleriKategorisi {
    Temeller,
    Genel,
    Yerleşim,
    Gezinme,
    VeriGirişi,
    VeriSunumu,
    GeriBildirim,
    Kabuk,
    Diğer,
}

impl GaleriKategorisi {
    pub const TÜMÜ: [Self; 9] = [
        Self::Temeller,
        Self::Genel,
        Self::Yerleşim,
        Self::Gezinme,
        Self::VeriGirişi,
        Self::VeriSunumu,
        Self::GeriBildirim,
        Self::Kabuk,
        Self::Diğer,
    ];
    pub const fn görünen_adı(self) -> &'static str {
        match self {
            Self::Temeller => "Temeller",
            Self::Genel => "Genel",
            Self::Yerleşim => "Yerleşim",
            Self::Gezinme => "Gezinme",
            Self::VeriGirişi => "Veri Girişi",
            Self::VeriSunumu => "Veri Sunumu",
            Self::GeriBildirim => "Geri Bildirim",
            Self::Kabuk => "Kabuk",
            Self::Diğer => "Diğer",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SergiTanımı {
    pub kimlik: Arc<str>,
    pub sözleşme: Arc<str>,
    pub ölçütler: Arc<[Arc<str>]>,
    pub başlık_anahtarı: YerelleştirmeAnahtarı,
    pub masaüstü: bool,
    pub wasm: bool,
    pub görünür_tüketim: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GaleriKatalogKaydı {
    pub sözleşme: Arc<str>,
    pub başlık_anahtarı: YerelleştirmeAnahtarı,
    pub açıklama_anahtarı: YerelleştirmeAnahtarı,
    pub kategori: GaleriKategorisi,
    pub önizleme: Arc<str>,
    pub sıra: u16,
}

pub(crate) fn anahtar(değer: &str) -> YerelleştirmeAnahtarı {
    YerelleştirmeAnahtarı::yeni(değer).expect("yerleşik galeri anahtarı geçerlidir")
}

fn kategori(sözleşme: &str) -> GaleriKategorisi {
    if sözleşme.starts_with("ORT-") {
        GaleriKategorisi::Temeller
    } else if sözleşme.starts_with("KAB-") {
        GaleriKategorisi::Kabuk
    } else {
        match &sözleşme[4..] {
            "010" | "020" | "030" | "040" | "090" | "120" | "130" | "150" | "170" | "200"
            | "210" => GaleriKategorisi::VeriGirişi,
            "050" | "060" | "070" | "080" | "250" | "260" => GaleriKategorisi::Gezinme,
            "100" | "160" | "230" | "270" | "280" | "290" => GaleriKategorisi::VeriSunumu,
            "110" | "140" => GaleriKategorisi::GeriBildirim,
            _ => GaleriKategorisi::Genel,
        }
    }
}

fn aile_kaydı(sözleşme: &str, sıra: u16) -> (SergiTanımı, GaleriKatalogKaydı) {
    let küçük = sözleşme.to_ascii_lowercase();
    let başlık = anahtar(&format!("galeri.aile.{küçük}.başlık"));
    let sergi: Arc<str> = format!("{küçük}/genel/varsayılan").into();
    (
        SergiTanımı {
            kimlik: Arc::clone(&sergi),
            sözleşme: sözleşme.into(),
            ölçütler: [format!("{sözleşme}.ACC-001").into()].into(),
            başlık_anahtarı: başlık.clone(),
            masaüstü: true,
            wasm: true,
            görünür_tüketim: true,
        },
        GaleriKatalogKaydı {
            sözleşme: sözleşme.into(),
            başlık_anahtarı: başlık,
            açıklama_anahtarı: anahtar(&format!("galeri.aile.{küçük}.açıklama")),
            kategori: kategori(sözleşme),
            önizleme: sergi,
            sıra,
        },
    )
}

pub fn yerleşik_kayıtlar() -> (Vec<SergiTanımı>, Vec<GaleriKatalogKaydı>) {
    BİL_AİLELERİ
        .iter()
        .chain(ORT_AİLELERİ)
        .chain(KAB_AİLELERİ)
        .enumerate()
        .map(|(i, id)| aile_kaydı(id, i as u16))
        .unzip()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GaleriSayfası {
    GenelBakış,
    Aile,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GaleriYerleşimKipi {
    ÜçBölgeli,
    Dar,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GaleriHedefi {
    Masaüstü,
    Wasm,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GaleriBölümTanımı {
    pub kimlik: Arc<str>,
    pub başlık_anahtarı: YerelleştirmeAnahtarı,
    pub sıra: u16,
}

pub fn aile_bölümleri() -> Vec<GaleriBölümTanımı> {
    [
        "amaç",
        "kullanım",
        "sergiler",
        "model-api",
        "erişilebilirlik",
        "capability",
        "profil",
        "kanıt",
    ]
    .into_iter()
    .enumerate()
    .map(|(i, k)| GaleriBölümTanımı {
        kimlik: k.into(),
        başlık_anahtarı: anahtar(&format!("galeri.bölüm.{k}")),
        sıra: i as u16,
    })
    .collect()
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GaleriEksenleri {
    pub tema: Arc<str>,
    pub profil: Arc<str>,
    pub yoğunluk: Arc<str>,
    pub metin_ölçeği: u16,
    pub locale: Arc<str>,
    pub rtl: bool,
    pub azaltılmış_hareket: bool,
    pub sürüm: BağlamSürümü,
}

impl GaleriEksenleri {
    pub fn işlemsel_değiştir(&self, değişim: impl FnOnce(&mut Self)) -> Self {
        let mut yeni = self.clone();
        değişim(&mut yeni);
        yeni.sürüm = BağlamSürümü(self.sürüm.0 + 1);
        yeni
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GaleriModeli {
    pub hedef: GaleriHedefi,
    pub sergiler: Vec<SergiTanımı>,
    pub katalog: Vec<GaleriKatalogKaydı>,
    pub aile_bölümleri: Vec<GaleriBölümTanımı>,
    pub sayfa: GaleriSayfası,
    pub seçili_aile: Option<Arc<str>>,
    pub yerleşim: GaleriYerleşimKipi,
    pub eksenler: GaleriEksenleri,
}

impl GaleriModeli {
    pub fn yerleşik() -> Self {
        Self::yerleşik_hedef(GaleriHedefi::Masaüstü)
    }

    pub fn yerleşik_hedef(hedef: GaleriHedefi) -> Self {
        let (sergiler, katalog) = yerleşik_kayıtlar();
        Self {
            hedef,
            sergiler,
            katalog,
            aile_bölümleri: aile_bölümleri(),
            sayfa: GaleriSayfası::GenelBakış,
            seçili_aile: None,
            yerleşim: GaleriYerleşimKipi::ÜçBölgeli,
            eksenler: GaleriEksenleri {
                tema: "açık".into(),
                profil: "temel".into(),
                yoğunluk: "rahat".into(),
                metin_ölçeği: 100,
                locale: "tr".into(),
                rtl: false,
                azaltılmış_hareket: false,
                sürüm: BağlamSürümü(1),
            },
        }
    }
    pub fn kategoriler(&self) -> impl Iterator<Item = GaleriKategorisi> + '_ {
        GaleriKategorisi::TÜMÜ
            .into_iter()
            .filter(|k| self.katalog.iter().any(|x| x.kategori == *k))
    }

    pub fn aileyi_aç(&mut self, sözleşme: impl Into<Arc<str>>) -> bool {
        let sözleşme = sözleşme.into();
        if !self.katalog.iter().any(|kayıt| kayıt.sözleşme == sözleşme) {
            return false;
        }
        self.sayfa = GaleriSayfası::Aile;
        self.seçili_aile = Some(sözleşme);
        true
    }

    pub fn genel_bakışa_dön(&mut self) {
        self.sayfa = GaleriSayfası::GenelBakış;
        self.seçili_aile = None;
    }
}

pub fn yerleşimi_çöz(kullanılabilir: u32, metin_ölçeği: u16) -> GaleriYerleşimKipi {
    if kullanılabilir.saturating_mul(100) / u32::from(metin_ölçeği) >= 900 {
        GaleriYerleşimKipi::ÜçBölgeli
    } else {
        GaleriYerleşimKipi::Dar
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DarYerleşimEşdeğerliği {
    pub sol_drawer: bool,
    pub sağ_satır_içi: bool,
    pub orta_sıra_korunur: bool,
    pub kapalı_bölgeler_odaksız: bool,
}
pub const fn dar_yerleşim_eşdeğerliği() -> DarYerleşimEşdeğerliği {
    DarYerleşimEşdeğerliği {
        sol_drawer: true,
        sağ_satır_içi: true,
        orta_sıra_korunur: true,
        kapalı_bölgeler_odaksız: true,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapabilitySunumu {
    pub ad: Arc<str>,
    pub destekleniyor: bool,
    pub rozet_görünür: bool,
    pub sahte_kontrol: bool,
}
pub fn capability_sunumu(ad: impl Into<Arc<str>>, destekleniyor: bool) -> CapabilitySunumu {
    CapabilitySunumu {
        ad: ad.into(),
        destekleniyor,
        rozet_görünür: !destekleniyor,
        sahte_kontrol: false,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SergiKartıSonucu {
    Yaşıyor,
    YalıtılmışHata(Arc<str>),
}
pub fn sergiyi_yalıt(sonuç: Result<(), &str>) -> SergiKartıSonucu {
    sonuç
        .map(|_| SergiKartıSonucu::Yaşıyor)
        .unwrap_or_else(|k| SergiKartıSonucu::YalıtılmışHata(k.into()))
}

pub fn sergi_başlığı_isteği(sergi: &SergiTanımı) -> İletiİsteği {
    İletiİsteği {
        anahtar: sergi.başlık_anahtarı.clone(),
        argümanlar: Arc::from(Vec::<İletiArgümanı>::new()),
    }
}

/// `YÖN-006.ACC-008` sergi başlığı `ORT-021` mühürlü hizmetiyle, yaşayan
/// yerel bağlam ve güncel katalog snapshot'ı üzerinden çözülür.
///
/// Sahte çözücü yoktur: `İletiÇözümleyicisi` mühürlüdür ve tek üretim
/// implementor'u `İletiÇözümHizmeti`dir. Bayat bağlam/katalog exact
/// `İletiÇözümHatası` varyantlarıyla döner.
pub fn sergi_başlığını_çöz(
    çözücü: &İletiÇözümHizmeti,
    sergi: &SergiTanımı,
    yerel: &YerelMetinBağlamı,
    katalog: Arc<İletiKataloğuSnapshot>,
) -> Result<gpui::SharedString, İletiÇözümHatası> {
    çözücü
        .çöz(&sergi_başlığı_isteği(sergi), yerel, katalog)
        .map(|çözülen| gpui::SharedString::new(çözülen.metin().metin()))
}

pub fn kanıtsız_ölçütler<'a>(
    ölçütler: impl IntoIterator<Item = &'a str>,
    sergiler: &[SergiTanımı],
) -> BTreeSet<Arc<str>> {
    let bağlı: BTreeSet<&str> = sergiler
        .iter()
        .flat_map(|s| s.ölçütler.iter().map(AsRef::as_ref))
        .collect();
    ölçütler
        .into_iter()
        .filter(|m| !bağlı.contains(m))
        .map(Arc::from)
        .collect()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GaleriBilgiMimarisi {
    pub üst_araç_çubuğu: bool,
    pub sol_kategori_aile: bool,
    pub orta_belge: bool,
    pub sağ_çapa: bool,
    pub hedefe_özgü_ikinci_tasarım: bool,
}
pub const fn ortak_bilgi_mimarisi(_hedef: GaleriHedefi) -> GaleriBilgiMimarisi {
    GaleriBilgiMimarisi {
        üst_araç_çubuğu: true,
        sol_kategori_aile: true,
        orta_belge: true,
        sağ_çapa: true,
        hedefe_özgü_ikinci_tasarım: false,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TasarımProvenance {
    pub ant_bilgi_mimarisi_referansı: bool,
    pub react_css_varlık_metin_kopyası: bool,
    pub sayısal_token_breakpoint_kopyası: bool,
    pub kanonik_tema_ve_profil: bool,
}
pub const fn tasarım_provenance() -> TasarımProvenance {
    TasarımProvenance {
        ant_bilgi_mimarisi_referansı: true,
        react_css_varlık_metin_kopyası: false,
        sayısal_token_breakpoint_kopyası: false,
        kanonik_tema_ve_profil: true,
    }
}

thread_local! {
    /// Kabuk tipografisi: tezgâhın **kendi** çiziminin yazı ailesi ve
    /// puntosu.
    ///
    /// Üst şeritteki aile seçimi buraya iner ve bütün tezgâhı etkiler.
    /// Metin kutusunun kendi ailesi ayrı bir eksendir (`parça_ailesi`) ve
    /// yalnız o kutuya uygulanır — ikisini tek değere bağlamak, kabuk
    /// fontunu denemek isteyen kullanıcının önizlemeyi de değiştirmesine
    /// yol açıyordu.
    static KABUK_GÖRÜNÜMÜ: std::cell::RefCell<Option<KabukGörünümü>> =
        const { std::cell::RefCell::new(None) };
}

/// Üst şeridin kabuğa inen ekseni.
#[derive(Clone, Debug)]
struct KabukGörünümü {
    aile: Arc<str>,
    punto: f32,
    ölçek: f32,
    yoğunluk: ArayüzYoğunluğu,
    hareket: HareketTercihi,
}

/// Kare başında kabuk görünümünü kurar.
///
/// `metin_ölçeği` de buraya iner: `ORT-004` ölçeği yalnız önizleme
/// kutusuna uygulanıyordu, oysa üst şeritteki aile ve punto bütün tezgâhı
/// etkiliyor. Aynı şeritteki üçüncü eksenin farklı kapsamda olması
/// tutarsızdı — ve `YÖN-006.ACC-011` `%200` ölçekte kabuğun da okunur
/// kalmasını istiyor.
///
/// Yoğunluk ve hareket aynı gerekçeyle buraya iner: ikisi de üst şeritte
/// yaşıyor. Tezgâhın kendi yüzleri `tasarım_görünümünü_çöz` üzerinden
/// kabuk temasını okuduğu için, seam'e girmeyen bir eksen düğmede seçili
/// görünüp hiçbir şeyi değiştirmiyordu.
pub fn kabuk_görünümünü_kur(
    aile: &str,
    punto: f32,
    ölçek: f32,
    yoğunluk: ArayüzYoğunluğu,
    hareket: HareketTercihi,
) {
    KABUK_GÖRÜNÜMÜ.with(|hücre| {
        *hücre.borrow_mut() = Some(KabukGörünümü {
            aile: Arc::from(aile),
            punto,
            ölçek,
            yoğunluk,
            hareket,
        });
    });
}

/// Galerinin `ORT-004` token değerlerini üreten ürün teması.
///
/// `ORT-004` token *şeklinin* sahibidir; değerleri ürün verir. Galeri kendi
/// paletini burada tek yerde token'a çevirir ve yaşayan bileşenler rengi,
/// ölçüyü ve tipografiyi yalnız buradan çözer.
pub fn galeri_teması() -> Arc<TemaAnlıkGörüntüsü> {
    // Canlı sergiler de seçilen paletten beslenir; aksi hâlde koyu kipte
    // kabuk döner ama kartların içi açık kalır.
    let p = crate::palet();
    let renk = |onaltılık: u32| -> Hsla { rgb(onaltılık).into() };
    let gölge = |y: f32, bulanıklık: f32| GölgeTokenı {
        renk: hsla(0., 0., 0., 0.12),
        ofset_x: px(0.),
        ofset_y: px(y),
        bulanıklık: px(bulanıklık),
        yayılma: px(0.),
    };
    // Kabuk tipografisi kurulmadıysa kütüphane varsayılanı kullanılır;
    // galerinin başka ekranları bu dikişi kurmaz.
    let KabukGörünümü {
        aile,
        punto,
        ölçek,
        yoğunluk,
        hareket,
    } = KABUK_GÖRÜNÜMÜ
        .with(|hücre| hücre.borrow().clone())
        .unwrap_or_else(|| KabukGörünümü {
            aile: Arc::from("IBM Plex Sans"),
            punto: 14.,
            ölçek: 1.,
            yoğunluk: ArayüzYoğunluğu::Normal,
            hareket: HareketTercihi::Tam,
        });
    let gövde = TextStyle {
        color: renk(p.kabuk_ana_metin),
        font_family: aile.to_string().into(),
        font_size: px(punto * ölçek).into(),
        line_height: px(punto * ölçek * 1.45).into(),
        ..TextStyle::default()
    };
    let kutu = |arka: u32, ön: u32, kenar: u32| KutuRenkleri {
        arka_plan: renk(arka),
        ön_plan: renk(ön),
        kenarlık: renk(kenar),
    };

    Arc::new(TemaAnlıkGörüntüsü {
        // Aday bildirilmiyor: ORT-017 profilinin kütüphane
        // varsayılanı kullanılır (`None` sıfır padding demek değildir).
        metin_düzenleme_iç_boşluğu: None,
        metin_imleci: gpui_bilesenleri::MetinİmleciTemaRolleri {
            ince_imleç: gpui::hsla(0., 0., 0.2, 1.),
            blok_zemini: gpui::hsla(0., 0., 0.2, 1.),
            blok_üstü_metin: gpui::hsla(0., 0., 0.98, 1.),
            seçim_zemini: gpui::hsla(0.6, 0.5, 0.7, 1.),
            seçim_metni: gpui::hsla(0., 0., 0.1, 1.),
            hareket: None,
        },
        bağlam: TemaBağlamı {
            kimlik: TemaKimliği(Arc::from("galeri")),
            // Palet değişince renk sürümü de değişmeli: `ORT-004` aynı
            // sürümün iki farklı değer taşımasını yasaklar.
            sürüm: u64::from(p.kabuk_zemin),
            renk_sürümü: u64::from(p.kabuk_zemin),
            ölçü_sürümü: 1,
            // Kip paletin kendi alanından gelir; zemin parlaklığından
            // tahmin edilmez (yüksek karşıtlık kipleri tahmini yanıltır).
            kip: p.kip,
            yoğunluk,
            hareket,
            metin_ölçeği: ölçek,
        },
        renkler: RenkTokenları {
            pencere: renk(p.kabuk_zemin),
            yüzey: renk(p.kabuk_kart),
            yükseltilmiş: renk(p.kabuk_kart),
            metin: renk(p.kabuk_ana_metin),
            soluk_metin: renk(p.soluk),
            odak: renk(p.kabuk_vurgu),
            bilgi: renk(0x2563eb),
            başarı: renk(0x047857),
            uyarı: renk(0xb45309),
            hata: renk(0xb91c1c),
        },
        // `ORT-004` metin ölçeği **her** role uygulanır. Yalnız `gövde`
        // ölçeklendiğinde `%200`de kart başlıkları, rozetler ve etiketler
        // küçük kalıyor, ekran yarı ölçekli bir karma oluyordu.
        tipografi: TipografiTokenları {
            küçük_gövde: TextStyle {
                font_size: px(12. * ölçek).into(),
                line_height: px(16. * ölçek).into(),
                ..gövde.clone()
            },
            etiket: TextStyle {
                color: renk(p.kabuk_ikincil_metin),
                font_size: px(12. * ölçek).into(),
                line_height: px(16. * ölçek).into(),
                ..gövde.clone()
            },
            başlık: TextStyle {
                font_size: px(18. * ölçek).into(),
                line_height: px(24. * ölçek).into(),
                ..gövde.clone()
            },
            tek_aralıklı: TextStyle {
                font_family: "monospace".into(),
                ..gövde.clone()
            },
            gövde,
        },
        boşluklar: BoşlukTokenları {
            küçük: px(4.),
            normal: px(12.),
            büyük: px(20.),
        },
        ölçüler: ÖlçüTokenları {
            etkileşim_hedefi: px(42.),
            simge: px(16.),
            ayırıcı: px(1.),
            yeniden_boyutlandırma_alanı: px(6.),
        },
        // `ORT-003` kademeleri birbirinden ayırt edilebilir olmalı: 4/10/16
        // arasındaki fark gözle seçilir. `Hap` token değildir, yüksekliğin
        // yarısından türer.
        köşeler: KöşeTokenları {
            yok: px(0.),
            küçük: px(4.),
            normal: px(10.),
            büyük: px(16.),
        },
        gölgeler: GölgeTokenları {
            düşük: gölge(1., 2.),
            orta: gölge(2., 6.),
            yüksek: gölge(4., 12.),
        },
        hareketler: HareketTokenları {
            hızlı: Duration::from_millis(80),
            normal: Duration::from_millis(160),
            yavaş: Duration::from_millis(240),
        },
        ayrıntılı_renkler: Some(SemantikRenkRolleri {
            örtü: hsla(0., 0., 0., 0.4),
            devre_dışı_metin: renk(0x9ca3af),
            vurgu_metni: renk(0x3046b8),
            ters_metin: renk(0xffffff),
            olağan_kenarlık: renk(0xdfe5ef),
            soluk_kenarlık: renk(0xe5e7eb),
            odak_kenarlığı: renk(0x3046b8),
            seçili_kenarlık: renk(0x3046b8),
            hata_kenarlığı: renk(0xb91c1c),
            üzerinde_etkileşim: renk(0xf1f3f7),
            basılı_etkileşim: renk(0xe4e8f5),
            seçili_etkileşim: hsla(0.63, 0.6, 0.55, 0.32),
            bırakma_hedefi: renk(0xeef1ff),
        }),
        hareket_koreografileri: None,
        kutu_köşe_rolleri: None,
        ortak_kutu_rolleri: Some(OrtakKutuTemaRolleri {
            normal: kutu(p.kabuk_kart, p.kabuk_ana_metin, p.kabuk_kenarlık),
            üzerinde: kutu(p.kabuk_zemin, p.kabuk_ana_metin, p.kabuk_kenarlık),
            basılı: kutu(p.kabuk_seçili_zemin, p.kabuk_ana_metin, p.kabuk_vurgu),
            devre_dışı: kutu(p.kabuk_zemin, p.soluk, p.kabuk_kenarlık),
        }),
        imleç: None,
        gelişmiş_tema: None,
    })
}

/// `BİL-010` tezgâh ekranının renk paleti.
///
/// Sıcak kâğıt zemin ve terracotta vurgu, tezgâhı katalogdan görsel olarak
/// ayırır: burası gezinilecek bir liste değil, üzerinde çalışılan bir yüzey.
/// Değerler tek yerde durur ki hem tema hem çevre çizimi aynı paletten
/// beslensin.
///
/// Değerler artık sabit değil: kare başında kurulan `palet()` seçilen
/// temadan gelir, böylece tema değişimi bütün pencereye uygulanır.
pub fn tezgah_kağıt() -> u32 {
    crate::palet().kağıt
}
pub fn tezgah_yüzey() -> u32 {
    crate::palet().yüzey
}
pub fn tezgah_kenarlık() -> u32 {
    crate::palet().kenarlık
}
pub fn tezgah_ince() -> u32 {
    crate::palet().ince
}
pub fn tezgah_ana_metin() -> u32 {
    crate::palet().ana_metin
}
pub fn tezgah_ikincil_metin() -> u32 {
    crate::palet().ikincil_metin
}
pub fn tezgah_soluk() -> u32 {
    crate::palet().soluk
}
pub fn tezgah_vurgu() -> u32 {
    crate::palet().vurgu
}
pub fn tezgah_vurgu_zemin() -> u32 {
    crate::palet().vurgu_zemin
}
pub fn tezgah_kod_zemin() -> u32 {
    crate::palet().kod_zemin
}
pub fn tezgah_kod_metin() -> u32 {
    crate::palet().kod_metin
}

/// `BİL-010` tezgâhının kâğıt paleti.
///
/// Tezgâh ekranı kendi görsel dilini taşır: sıcak kâğıt zemin, terracotta
/// vurgu, ince kenarlıklar. Bu palet yalnız o ekranın önizleme kutusuna
/// uygulanır; galerinin geri kalanı `galeri_teması()` ile çizilmeye devam
/// eder. Böylece tek bir ailenin tasarımı bütün kataloğu değiştirmez.
pub fn tezgah_teması(tercih: &crate::TezgahTeması) -> Arc<TemaAnlıkGörüntüsü> {
    let renk = |onaltılık: u32| -> Hsla { rgb(onaltılık).into() };
    let kutu = |arka: u32, ön: u32, kenar: u32| KutuRenkleri {
        arka_plan: renk(arka),
        ön_plan: renk(ön),
        kenarlık: renk(kenar),
    };
    let taban = galeri_teması();
    // `ORT-004 §4` bileşen ham font ailesi okuyamaz; aileyi tema verir.
    // Tezgâhtaki yazı denetimleri bu yüzden alana değil temaya yazar.
    let gövde = TextStyle {
        color: renk(tezgah_ana_metin()),
        font_family: tercih.yazı_ailesi.clone().into(),
        font_size: px(tercih.punto * tercih.metin_ölçeği).into(),
        line_height: px(tercih.punto * tercih.metin_ölçeği * 1.45).into(),
        font_weight: tercih.ağırlık.gpui_ağırlığı(),
        font_style: if tercih.eğik {
            gpui::FontStyle::Italic
        } else {
            gpui::FontStyle::Normal
        },
        underline: tercih.altı_çizili.then(|| gpui::UnderlineStyle {
            thickness: px(1.),
            color: Some(renk(tezgah_ana_metin())),
            wavy: false,
        }),
        strikethrough: tercih.üstü_çizili.then(|| gpui::StrikethroughStyle {
            thickness: px(1.),
            color: Some(renk(tezgah_ana_metin())),
        }),
        ..taban.tipografi.gövde.clone()
    };

    Arc::new(TemaAnlıkGörüntüsü {
        // `ORT-004` iç boşluk farkı tercihe bağlı. `None` sıfır dolgu
        // demek değil: fark bildirilmez ve `ORT-017` kütüphane varsayılanı
        // (8/8/4/4) geçerli kalır.
        metin_düzenleme_iç_boşluğu: tercih.iç_boşluk.kanonik(),
        metin_imleci: gpui_bilesenleri::MetinİmleciTemaRolleri {
            ince_imleç: gpui::hsla(0., 0., 0.2, 1.),
            blok_zemini: gpui::hsla(0., 0., 0.2, 1.),
            blok_üstü_metin: gpui::hsla(0., 0., 0.98, 1.),
            seçim_zemini: gpui::hsla(0.6, 0.5, 0.7, 1.),
            seçim_metni: gpui::hsla(0., 0., 0.1, 1.),
            hareket: tercih.imleç_hızı.hareket(),
        },
        bağlam: TemaBağlamı {
            kimlik: TemaKimliği(Arc::from("tezgah-kağıt")),
            // `ORT-004` anlık görüntü değişmezdir: tercih değişince yeni bir
            // görüntü üretilir ve sürümü artar. Aynı sürüm hep aynı değeri
            // verir; bileşen bunu güvenle önbelleğe alabilir.
            sürüm: tercih.sürüm,
            renk_sürümü: tercih.sürüm,
            ölçü_sürümü: tercih.sürüm,
            kip: tercih.kip,
            metin_ölçeği: tercih.metin_ölçeği,
            // `ORT-004 §25` anlık görüntüsü yoğunluk ve hareket tercihini de
            // taşır. Sayısal karşılıklarını tema üretmez (`ORT-004 §43`:
            // ikinci kaynak yasak); çözülmüş metrik bileşenin `ORT-017`
            // görünüm profilinden gelir. Burada yalnız bağlam doğru kurulur.
            yoğunluk: tercih.yoğunluk,
            hareket: tercih.hareket,
        },
        renkler: RenkTokenları {
            pencere: renk(tezgah_kağıt()),
            yüzey: renk(tezgah_yüzey()),
            yükseltilmiş: renk(tezgah_yüzey()),
            metin: renk(tezgah_ana_metin()),
            soluk_metin: renk(tezgah_soluk()),
            odak: renk(tezgah_vurgu()),
            ..taban.renkler.clone()
        },
        tipografi: TipografiTokenları {
            etiket: TextStyle {
                color: renk(tezgah_ikincil_metin()),
                ..taban.tipografi.etiket.clone()
            },
            gövde,
            ..taban.tipografi.clone()
        },
        // Tasarımın önizleme kutusu 58 piksel yüksekliğindedir; `Hap`
        // yarıçapı bu ölçünün yarısından türer.
        ölçüler: ÖlçüTokenları {
            etkileşim_hedefi: px(58.),
            ..taban.ölçüler
        },
        köşeler: KöşeTokenları {
            yok: px(0.),
            küçük: px(2.),
            normal: px(8.),
            büyük: px(16.),
        },
        ayrıntılı_renkler: taban
            .ayrıntılı_renkler
            .clone()
            .map(|roller| SemantikRenkRolleri {
                vurgu_metni: renk(tezgah_vurgu()),
                olağan_kenarlık: renk(tezgah_kenarlık()),
                soluk_kenarlık: renk(tezgah_ince()),
                odak_kenarlığı: renk(tezgah_vurgu()),
                seçili_kenarlık: renk(tezgah_vurgu()),
                devre_dışı_metin: renk(tezgah_soluk()),
                üzerinde_etkileşim: renk(tezgah_vurgu_zemin()),
                basılı_etkileşim: renk(tezgah_vurgu_zemin()),
                bırakma_hedefi: renk(tezgah_vurgu_zemin()),
                ..roller
            }),
        imleç: Some(gpui_bilesenleri::İmleçTokenları {
            kalınlık: px(tercih.imleç_kalınlığı),
        }),
        ortak_kutu_rolleri: Some(OrtakKutuTemaRolleri {
            normal: kutu(tezgah_yüzey(), tezgah_ana_metin(), tezgah_kenarlık()),
            üzerinde: kutu(tezgah_yüzey(), tezgah_ana_metin(), tezgah_kenarlık()),
            basılı: kutu(tezgah_vurgu_zemin(), tezgah_ana_metin(), tezgah_vurgu()),
            devre_dışı: kutu(tezgah_kağıt(), tezgah_soluk(), tezgah_ince()),
        }),
        ..(*taban).clone()
    })
}

// Tezgâh anahtarlarının görünen karşılığı artık kodda bir sözlük değildir:
// taşınan metin-girişi yüzeyinin kayıtları `metin_hizmetleri` modülündeki
// gerçek `ORT-021` katalog paketindedir ve çözüm mühürlü hizmetten geçer.
// Bilinmeyen anahtar yine sessizce boşa düşmez; çözücü anahtarın kendisini
// gösterir. Aşağıdaki aile adı/açıklaması sözlükleri ise **taşınmamış**
// bileşen ailelerinin placeholder'larıdır ve bu atomda değişmez.

/// Bileşenin ne işe yaradığını bir satırda anlatır.
///
/// Kullanıcı yüzeyinde sözleşme numarası geçmez: sözleşmeler bizim tasarım
/// denetimimizdir, kütüphaneyi kullananın bilmesi gerekmez. Kimlik yalnız
/// katalog anahtarı ve kanıt yüzeyinde kalır.
pub fn aile_açıklaması(sözleşme: &str) -> &'static str {
    match sözleşme {
        "ORT-001" => "Uygulama yaşam döngüsü, pencere kökü ve platform hedefi",
        "ORT-002" => "Unicode metin, grafem sınırı, yazı yönü ve Türkçe harf kuralları",
        "ORT-003" => "Kutu şekli, köşe yarıçapı ve paylaşılan kenar çözümü",
        "ORT-004" => "Tema tokenları, etkileşim durumları ve anlamsal renk rolleri",
        "ORT-005" => "Odak sırası, klavye bölgeleri ve sekme turu",
        "ORT-006" => "Açılır yüzey konumlandırma, dışarı tıklama ve kapanış",
        "ORT-007" => "Sürümlü eşzamansız iş ve eski sonucun reddi",
        "ORT-008" => "Sayı, para ve tarih biçimlendirme ile ayrıştırma",
        "ORT-009" => "Rol, erişilebilir ad, durum ve canlı duyuru",
        "ORT-010" => "Sürükleme yükü, bırakma hedefi ve sonuç",
        "ORT-011" => "Yeniden boyutlandırma ekseni, sınır ve sürükleme deltası",
        "ORT-012" => "Pencerelenmiş liste, sanallaştırma ve güvenli overscan",
        "ORT-013" => "Geri alma ve yineleme işlem kaydı",
        "ORT-014" => "Otomatik kaydetme, kirli durum ve kurtarma",
        "ORT-015" => "Pano biçim müzakeresi ve güvenli yapıştırma",
        "ORT-016" => "Simge kataloğu, varyant, yön ve güvenli varlık çözümü",
        "ORT-017" => "Bileşen anatomisi, yuvalar ve görünüm profili",
        "ORT-018" => "Çizim bütçesi, yüzdebirlik ve ölçüm",
        "ORT-019" => "Hassas içerik, redaksiyon ve tanı gizliliği",
        "ORT-020" => "Ayar kapsamı, katman önceliği ve sürümlü depo",
        "ORT-021" => "Yerelleştirme kataloğu, çoğul ve locale fallback",
        "ORT-022" => "Komut kataloğu, etkinlik ve tek yürütme niyeti",
        "ORT-023" => "Tipli rota, geçmiş ve ayrılma koruması",
        "BİL-010" => "Maske, biçim, doğrulama ve IME destekli metin alanı",
        "BİL-020" => "Tek ve çoklu seçim listeleri, süzme ve jeton",
        "BİL-030" => "Onay kutusu, üç durumlu kutu ve anahtar",
        "BİL-040" => "Birincil, ikincil ve hayalet düğmeler; yükleme durumu",
        "BİL-050" => "Sekme çubuğu; önizleme ve sabitlenmiş sekmeler",
        "BİL-060" => "Yan panel; görünürlük, konum ve yaşam döngüsü",
        "BİL-070" => "Araç çubuğu; bölge, daralma ve taşma menüsü",
        "BİL-080" => "Modal, onay dialogu ve dismissal kuralları",
        "BİL-090" => "Komut seçici; sorgu, vurgu, önizleme ve kabul",
        "BİL-100" => "Veri tablosu; sütun, sıralama, seçim ve sanallaştırma",
        "BİL-110" => "Toast, banner ve bildirim merkezi",
        "BİL-120" => "Form düzeni; alan grupları, doğrulama ve gönderim",
        "BİL-130" => "Kaydırıcı ve aralık seçimi; adım ve tutamaç",
        "BİL-140" => "Durum rozeti, ilerleme çubuğu ve iskelet",
        "BİL-150" => "Takvim, tarih ve tarih aralığı seçimi",
        "BİL-160" => "Ağaç, akordiyon ve açılır kapanır bölümler",
        "BİL-170" => "Renk seçici; palet, tekerlek ve alfa",
        "BİL-180" => "Dosya yükleme ve indirme; ilerleme ve iptal",
        "BİL-190" => "Belge içi arama; eşleşme gezinme ve vurgu",
        "BİL-200" => "Kısayol düzenleyici; tuş yakalama ve çakışma çözümü",
        "BİL-210" => "Ayar kataloğu; arama, kapsam ve yönetilen değer",
        "BİL-220" => "Veri kaynağı bağlantı profili ve bağlantı sınaması",
        "BİL-230" => "Salt okunur kod görünümü ve sözdizimi vurgusu",
        "BİL-250" => "Yüzen eylem düğmesi ve eylem grubu",
        "BİL-260" => "Gezinme çubuğu, kırıntı ve yan gezinme",
        "BİL-270" => "Görsel sunum; önizleme, galeri ve dizi konumu",
        "BİL-280" => "QR ve çubuk kod üretimi; ön doğrulama",
        "BİL-290" => "Medya oynatıcı; poster, denetimler ve capability fallback",
        "KAB-010" => "Dock bölgeleri; panel tutturma ve görünürlük",
        "KAB-020" => "Alt çalışma alanı; terminal ve çıktı sekmeleri",
        "KAB-030" => "Pencere kromu, başlık çubuğu ve sürükleme bölgesi",
        "KAB-040" => "Durum çubuğu; öğe önceliği ve taşma",
        "KAB-050" => "Pencere yaşam döngüsü ve kirli kapanış onayı",
        "KAB-060" => "Bölünmüş görünüm; oran ve klavye ile boyutlandırma",
        "KAB-070" => "Uygulama menüsü; komut etkinliği ve kısayol sunumu",
        "KAB-080" => "Çoklu pencere kaydı ve ekran sınırına uyarlama",
        "KAB-090" => "Oturum kurtarma, bozuk kayıt yalıtımı ve güvenli mod",
        "KAB-100" => "Uygulama veri konumları ve gizli değer saklama",
        _ => "Bileşen ailesi",
    }
}

/// Ant Design genel bakış sayfasındaki gibi kategori özeti.
pub const fn kategori_açıklaması(kategori: GaleriKategorisi) -> &'static str {
    match kategori {
        GaleriKategorisi::Temeller => "Tema, metin, odak ve platform altyapısı",
        GaleriKategorisi::Genel => "Her üründe kullanılan temel denetimler",
        GaleriKategorisi::Yerleşim => "Sayfa ve panel yerleşimi",
        GaleriKategorisi::Gezinme => "Sekme, menü ve gezinme yüzeyleri",
        GaleriKategorisi::VeriGirişi => "Değer toplayan ve doğrulayan alanlar",
        GaleriKategorisi::VeriSunumu => "Liste, tablo ve içerik gösterimi",
        GaleriKategorisi::GeriBildirim => "Durum, ilerleme ve bildirim",
        GaleriKategorisi::Kabuk => "Pencere, dock ve uygulama kabuğu",
        GaleriKategorisi::Diğer => "Diğer bileşenler",
    }
}

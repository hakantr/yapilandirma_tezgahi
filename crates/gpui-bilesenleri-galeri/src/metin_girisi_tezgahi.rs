//! `BİL-010` yapılandırma tezgâhı: programcının kod yüzeyini denenebilir kılar.
//!
//! Galeri burada davranış tanımlamaz; yalnız `GirişYapılandırması`nın
//! alanlarını açar, canlı alana uygular ve karşılığı olan Rust kodunu
//! gösterir. Maske tanımı da bu tezgâhın bir tercihidir: ayrı bir kart
//! değil, aynı yüzeyin bir satırı.
//!
//! Tercihler değer türüne göre süzülür. Metin alanında ondalık basamak,
//! sayısal alanda parola düğmesi göstermek programcıyı yanıltır; tezgâh
//! yalnız o türde gerçekten kurulabilen tercihi açar.

use std::time::Duration;

use gpui_bilesenleri::{
    AramaGönderimYapılandırması, ArayüzYoğunluğu, AçıkSaatBiçimi, AçıkTarihBiçimi,
    AçıkTarihSaatBiçimi, AçılırYüzeyYapılandırması, BasamakGruplama, BilimselBiçim, BitişikBölüt,
    BitişikBölütKuşağı, BitişikBölütTürü, BiçimTanımı, BiçimYapılandırması, BoşMetinPolitikası,
    DurumGöstergesiAçıklamaTercihi, DurumGöstergesiYapılandırması, DurumGöstergesiYerleşimTercihi,
    DüğmeŞekli, DışHataTemizleme, EnterDavranışı, EscapeDavranışı, GeçerlilikKuralTürü,
    GeçerlilikKuralı, GeçerlilikKuralıKimliği, GeçerlilikTetikleyicisi, GeçerlilikÖnemi,
    GeçersizOdakDavranışı, GeçiciGösterimPolitikası, GirişDikeyHizalama, GirişMaskesi, GirişTürü,
    GirişYapılandırmaHatası, GirişYapılandırmaUyarısı, GirişYapılandırması, GirişYatayHizalama,
    HareketTercihi, HarfDönüşümü, KabulSeçimi, KesirBiçimi, KesirPaydası, KutuŞekliTercihi,
    KırpmaPolitikası, MetinYapıştırmaDönüşümü, MetinİçerikTürü, OdakSeçimi, OndalıkDeğer,
    OndalıkDuyarlılık, ParaBirimiGösterimi, ParaBiçimi, RakamKümesi, SaatDilimiGösterimi,
    SaatDilimiTercihi, SaatDöngüsü, Sabitİçerik, SabitİçerikSunumRolü, SayaçYapılandırması,
    SayıBiçimi, SayımBirimi, SeçiciGörünürlüğü, SeçiciUyarlaması, SüreBirimi, SüreBiçimi,
    TarihParçasıGösterimi, TemaKipi, UzunlukSınırı, UzunlukSınırıDavranışı,
    YardımcıEylemGörünürlüğü, YardımcıEylemTürü, YardımcıEylemYuvası, YardımcıEylemÇalışması,
    YüzdeBiçimi, ÇalışırkenEnterPolitikası, İçerikGörünürlüğü, İşaretKonumu,
};

/// Tezgâhın değer türü kipi — ekrandaki dokuz seçim.
///
/// Kanonik eksen dört ailedir (`GirişTürü`): para ve yüzde tür değil,
/// `Ondalık` türün `ORT-008` biçim profilleridir; tarih ailesi tek
/// `TarihZaman` türünün kipleridir. Ekran dokuz kipi korur, kanonik
/// yapılandırmaya `kanonik_tür` ve biçim çözümüyle iner.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TezgahDeğerKipi {
    Metin,
    Tamsayı,
    Ondalık,
    ParaBirimi,
    Yüzde,
    Tarih,
    Saat,
    TarihSaat,
    Süre,
}

impl TezgahDeğerKipi {
    fn tarih_zaman(kip: gpui_bilesenleri::TarihZamanKipi) -> GirişTürü {
        GirişTürü::TarihZaman(gpui_bilesenleri::TarihZamanTanımı {
            kip,
            motor: gpui_bilesenleri::TarihZamanMotorTercihi::Otomatik,
            belirsiz_zaman: gpui_bilesenleri::DstPolitikası::Reddet,
        })
    }

    /// `§6` kanonik giriş türü; Metin ailesi içerik türünü tanımında taşır.
    pub fn kanonik_tür(self, içerik_türü: MetinİçerikTürü) -> GirişTürü {
        use gpui_bilesenleri::TarihZamanKipi as K;
        match self {
            Self::Metin => GirişTürü::Metin(gpui_bilesenleri::MetinTanımı { içerik_türü }),
            Self::Tamsayı => GirişTürü::Tamsayı(gpui_bilesenleri::TamsayıTanımı::default()),
            Self::Ondalık | Self::ParaBirimi | Self::Yüzde => {
                GirişTürü::Ondalık(gpui_bilesenleri::OndalıkTanımı::default())
            }
            Self::Tarih => Self::tarih_zaman(K::Tarih),
            Self::Saat => Self::tarih_zaman(K::Saat),
            Self::TarihSaat => Self::tarih_zaman(K::YerelTarihSaat),
            Self::Süre => Self::tarih_zaman(K::Aralık),
        }
    }
}

/// Tezgâhın açtığı tercihler. Her alan `GirişYapılandırması`nın bir alanına
/// veya bir alan kümesine karşılık gelir.
#[derive(Clone, Debug, PartialEq)]
pub struct TezgahTercihleri {
    // §7 değer
    pub değer_türü: TezgahDeğerKipi,
    /// `§7` `MetinTanımı::içerik_türü`.
    ///
    /// `GirişYapılandırması` bugün `giriş_türü: GirişTürü` taşımıyor
    /// (`§8/16` borcu), bu yüzden seçim **koda yazılamaz**: yazacak alan
    /// yok. Ekranda durur çünkü eksen sözleşmede var ve tür ailesinin alt
    /// tanımı; gizlemek onu hiç yokmuş gibi gösterirdi.
    pub metin_içerik_türü: MetinİçerikTürü,
    // §9 maske
    pub maske: TezgahMaskesi,
    pub desen: String,
    /// Tasarımın biçim listesinde seçili satır; `BİÇİM_SEÇENEKLERİ` sırası.
    ///
    /// Maske ve biçim alanları bundan türetilir. Seçimi ayrıca saklıyoruz
    /// çünkü birden çok seçenek aynı alanlara iniyor: yalnız `maske` ve
    /// `desen`e bakarak hangi satırın seçildiği geri okunamaz.
    pub biçim_seçeneği: usize,
    pub ondalık_basamak: usize,
    pub binler_ayracı: bool,
    // §6 ön/son ek ve yer tutucu
    pub ön_ek: bool,
    pub ön_ek_metni: String,
    pub son_ek: bool,
    pub son_ek_metni: String,
    pub yer_tutucu: bool,
    // §23 yardımcı eylemler
    pub temizle: bool,
    pub arama: bool,
    pub parola_düğmesi: bool,
    pub seçici: bool,
    /// `§23` yuvaların sunum kademesi.
    ///
    /// Tezgâh her yuvayı `YardımcıEylemYuvası::kademeli` ile kuruyordu:
    /// dört görünürlük kipinden yalnız biri (`DeğerVarkenKademeli`)
    /// ekranda vardı, diğer üçü hiç denenemiyordu. Kanonikte kip yuva
    /// başınadır; tezgâh hepsine aynı değeri uygular ve kod çıktısı bunu
    /// olduğu gibi yazar.
    pub yuva_görünürlüğü: YardımcıEylemGörünürlüğü,
    /// `§23`/`BİL-040` yuvanın etkinleştirme kapısı.
    ///
    /// Kapalıyken alan etkin olsa bile yuva eylemi çalışmaz — ayrı bir
    /// kapıdır ve alanın `etkin` bayrağıyla karışmamalıdır.
    pub yuvalar_etkin: bool,
    /// `§23` arama yuvasının eylemi alanın gönderimine bağlı mı?
    ///
    /// `YardımcıEylemÇalışması::AlanınGönderimineBağlı` yalnız arama
    /// yuvasında anlamlıdır: temizleme ya da parola yuvası gönderim
    /// üretmez.
    pub arama_gönderime_bağlı: bool,
    /// `§23.1` ürünün kendi yardımcı eylemi.
    ///
    /// Yerleşik dört tür dışında ürün kendi eylemini yuvaya koyabilir.
    /// Simgesini **ürün sağlar**: `BİL-010` eylem kimliğinden ikinci bir
    /// simge kimliği türetmez, bu yüzden yuva tezgâhta simgesiz ama adlı
    /// ve tıklanabilir görünür — sözleşmenin dürüst karşılığı budur.
    pub ürün_eylemi: bool,
    /// `ORT-009` alanın erişilebilir adı kurulsun mu?
    ///
    /// Ad `§29`'da uyarı üretir: adsız alan erişilebilir ağaca girmez.
    /// Tezgâh adı sabit kuruyordu, yani `ErişilebilirAdYok` uyarısı hiç
    /// görülemiyordu — uyarının metni ekranda vardı ama yolu yoktu.
    pub erişilebilir_ad: bool,
    /// `ORT-009` yardımcı eylem yuvaları adlandırılsın mı?
    ///
    /// Ayrı bir eksen: alanın adı varken yuvalar adsız kalabilir ve
    /// `YardımcıEylemAdsız` ayrı bir uyarıdır. Adsız düğme ekran
    /// okuyucuda yalnız "düğme" diye okunur.
    pub yuva_adları: bool,
    /// `§23`/`ORT-003 §3.1` bitişik bölütün kendi kenarlığı olsun mu?
    ///
    /// Kuşakta yalnız dış köşeler yuvarlanır; iç kenar alanla paylaşılır.
    /// Bölütün kendi sınırı bu paylaşımı görünür kılar ya da gizler.
    pub bölüt_sınırı: bool,
    /// `ORT-008 §6` para/yüzde işaretinin yeri; yalnız o biçimlerde okunur.
    pub işaret_konumu: İşaretKonumu,
    // §22 içerik görünürlüğü
    pub görünürlük: TezgahGörünürlüğü,
    /// `§22` geçici gösterimin geri dönüş politikası; yalnız `GeçiciGöster`
    /// görünürlüğünde yapılandırmaya yazılır.
    pub geçici_gösterim: TezgahGeçiciGösterimi,
    /// `§16` dış hatanın yerel düzenlemede temizlenip temizlenmeyeceği.
    pub dış_hata_temizleme: DışHataTemizleme,
    /// `§16.2` birincil sorun göstergesinin ankrajı.
    ///
    /// `None` göstergeyi kapatır — `§16.2.4` bunu ayrı bir kademe değil,
    /// yapılandırmanın kapalı hâli sayar.
    pub gösterge_ankrajı: Option<DurumGöstergesiYerleşimTercihi>,
    /// `§16.2` göstergenin yardımcı açıklama yüzeyi tercihi.
    pub gösterge_açıklaması: DurumGöstergesiAçıklamaTercihi,
    // §6 metin işleme
    /// `ORT-002` harf dönüşümü; kabulde uygulanır.
    pub harf_dönüşümü: HarfDönüşümü,
    pub kırpma: KırpmaPolitikası,
    /// `§6` boş metnin değere çevrilme politikası.
    pub boş_metin: BoşMetinPolitikası,
    /// `§10` yapıştırılan metnin dönüşümü.
    pub yapıştırma: TezgahYapıştırması,
    // §17 kabul ve odak kaçışı
    pub escape: EscapeDavranışı,
    pub geçersiz_odak: GeçersizOdakDavranışı,
    // §23 bitişik bölüt ve arama gönderimi
    pub başlangıç_bölütü: Option<TezgahBölütü>,
    pub bitiş_bölütü: Option<TezgahBölütü>,
    /// `§23` yardımcı eylem yuvasının sunum kademesi: kademeli mi sabit mi.
    pub bölüt_kademeli: bool,
    pub çalışırken_enter: ÇalışırkenEnterPolitikası,
    /// `§23.3` arama gönderimi; yalnız `AramayıBaşlat` yuvası varken kurulur.
    pub arama_enter_gönderir: bool,
    pub arama_temizleme_gönderir: bool,
    // §24 seçici uyarlaması
    /// `§24` seçici yuvasının görünürlük politikası.
    pub seçici_görünürlüğü: SeçiciGörünürlüğü,
    // §15 doğrulama
    /// `§15` zorunluluk kuralı; kurallar listesine `Zorunlu` ekler.
    pub zorunlu: bool,
    pub doğrulama_tetikleyicisi: GeçerlilikTetikleyicisi,
    pub doğrulama_önemi: GeçerlilikÖnemi,
    /// `§29` birden çok kural varken ilk hatada durulsun mu?
    ///
    /// Kanonik alan `çekirdek.rs`'te iş yapıyor: açıkken alan ilk
    /// başarısız kuraldan sonra kalanları koşturmaz. Ekranda karşılığı
    /// yoktu, yani tezgâh yalnız "hepsini koştur" dalını gösteriyordu.
    pub ilk_hatada_dur: bool,
    // §9.7–9.8 sınır ve sayaç
    pub uzunluk_sınırı: bool,
    pub uzunluk_davranışı: UzunlukSınırıDavranışı,
    pub sayaç: bool,
    pub sayaç_birimi: SayımBirimi,
    pub sayaç_sınırı_göster: bool,
    /// `§9.6` sayısal adım açık mı?
    pub sayısal_adım: bool,
    pub adım_ölçeği: AdımÖlçeği,
    /// Sonuç tabana göre adımın katına hizalansın mı?
    pub adım_hizala: bool,
    /// `§15` `SayısalAralık` kuralı kurulsun mu?
    ///
    /// Sınır adımın kendi alanı değildir; aynı kural doğrulamayı da besler.
    pub adım_sınırı: bool,
    pub adım_sarma: bool,
    /// `§14` varsayılan değer açık mı?
    pub varsayılan_değer: bool,
    pub sıfırlama: gpui_bilesenleri::SıfırlamaDavranışı,
    /// `§9.5` bölüm gezinimi açık mı?
    pub bölüm_gezinimi: bool,
    pub bölüm_atla: bool,
    pub bölüm_dolunca_ilerle: bool,
    pub bölüm_artır: bool,
    pub bölüm_taşar: bool,
    pub bölüm_ayraç: bool,
    /// `§25` otomatik doldurma açık mı ve hangi amaçla?
    pub otomatik_doldurma: bool,
    pub doldurma_amacı: gpui_bilesenleri::OtomatikDoldurmaAmacı,
    // §21 hizalama · ORT-003 şekil
    pub hizalama: GirişYatayHizalama,
    pub dikey: GirişDikeyHizalama,
    pub ek_sunum_rolü: SabitİçerikSunumRolü,
    pub şekil: DüğmeŞekli,
    // C bölümü · önizleme senaryosu
    /// `§28` önem yüzeyi zemine de uygulansın mı?
    ///
    /// Varsayılan yalnız kenarlıktır; açıkken alan zemini de önem
    /// tokenının soluk tonuna geçer.
    pub önem_zemini: bool,
    /// `§7.1` parça tipografisi: önizleme **değerine** uygulanan yazı ailesi.
    ///
    /// `None` "rolden devral" demektir. Parça ataması yalnız değeri etkiler
    /// ve yerleşim düzeyi atamasını ezer; bu yüzden kabuk ailesinden ayrı
    /// bir eksendir. `D` bölümü olduğu için koda yazılmaz.
    pub parça_ailesi: Option<String>,
    /// `oto`: kabuk şekli görünüm profilinden gelir.
    ///
    /// Kanonik `KutuŞekliTercihi::GörünümProfilinden` karşılığı. Kademe ya
    /// da piksel seçmek bunu kapatır — üçü aynı anda geçerli olamaz.
    pub şekil_oto: bool,
    /// Açıkken köşe yarıçapı hazır kademeden değil bu pikselden çözülür.
    pub köşe_pikseli: Option<f32>,
    // §12/§18 odak ve kabul
    /// `§18` alan sekme sırasında bir durak mı?
    ///
    /// Kanonik `odak.sekme_durağı` alanı `tab_stop` olarak iş yapıyor ve
    /// `§29` uyarısı ("sekme durağı kapalı ama Enter sonrakine geçiyor")
    /// onu anlatıyordu; ekranda karşılığı olmadığı için o uyarı hiç
    /// kurulamıyordu.
    pub sekme_durağı: bool,
    pub odak_seçimi: OdakSeçimi,
    pub kabul_seçimi: KabulSeçimi,
    pub dış_tıklamada_odağı_bırak: bool,
    /// `§12.1` alanın açılış kipi; `Insert` çalışma anında değiştirir.
    pub üzerine_yazma: bool,
    pub enter: EnterDavranışı,
    // §20 erişim
    pub salt_okunur: bool,
    pub etkin: bool,
    /// `ORT-002 §5.2` saat dilimi kaynağı.
    ///
    /// Tezgâhta bulunmasının nedeni dördünü de sınayabilmek: platformun
    /// bildirdiği dilim, kullanıcının canlı seçimi ve ürünün sabitlediği
    /// dilim aynı alanda karşılaştırılabilir olmalı.
    pub saat_dilimi_tercihi: SaatDilimiTercihi,
    /// `ORT-004` tema tercihleri.
    ///
    /// Tipografi alanın değil temanın malıdır: sözleşme bileşenin ham font
    /// ailesi okumasını yasaklar. Bu yüzden yazı denetimleri
    /// `GirişYapılandırması`na değil, alana verilen anlık görüntüye yazar.
    /// Tezgâhta bulunmalarının nedeni tam da bu: tema yönetiminin gerçekten
    /// çalıştığını galeride sınamak.
    pub tema: TezgahTeması,
}

/// Tezgâhta seçilebilen adım çiftleri.
///
/// Serbest sayı kutusu yerine hazır çift sunulur: amaç adımın kendisini
/// yazmak değil, küçük/büyük ayrımının ve ondalık kesinliğin gerçekten
/// çalıştığını görmek.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum AdımÖlçeği {
    /// 1 · 10
    #[default]
    Birim,
    /// 5 · 25
    Beşlik,
    /// 0,25 · 1
    Çeyrek,
    /// 0,1 · 1
    Onda,
}

impl AdımÖlçeği {
    pub const TÜMÜ: [Self; 4] = [Self::Birim, Self::Beşlik, Self::Çeyrek, Self::Onda];

    pub const fn adı(self) -> &'static str {
        match self {
            Self::Birim => "1 · 10",
            Self::Beşlik => "5 · 25",
            Self::Çeyrek => "0,25 · 1",
            Self::Onda => "0,1 · 1",
        }
    }

    /// Adım kesirli mi? Tamsayı alanda kesirli adım `§29` hatasıdır.
    pub const fn kesirli_mi(self) -> bool {
        matches!(self, Self::Çeyrek | Self::Onda)
    }

    /// (küçük, büyük) çifti.
    pub fn çift(self) -> (OndalıkDeğer, OndalıkDeğer) {
        match self {
            Self::Birim => (ondalık(1, 0), ondalık(10, 0)),
            Self::Beşlik => (ondalık(5, 0), ondalık(25, 0)),
            Self::Çeyrek => (ondalık(25, 2), ondalık(1, 0)),
            Self::Onda => (ondalık(1, 1), ondalık(1, 0)),
        }
    }
}

fn ondalık(katsayı: i128, ölçek: u32) -> OndalıkDeğer {
    OndalıkDeğer::yeni(katsayı, ölçek).unwrap_or_else(|_| OndalıkDeğer::sıfır())
}

/// `ORT-004` anlık görüntüsünü üreten tezgâh tercihleri.
#[derive(Clone, Debug, PartialEq)]
pub struct TezgahTeması {
    /// Seçili aile adı; GPUI yazı sisteminin tanıdığı biçimde.
    ///
    /// Sabit bir liste yerine ad tutulur: kitaplık yüzlerinin yanında
    /// işletim sisteminde kurulu aileler de seçilebilir ve bu küme
    /// makineden makineye değişir.
    pub yazı_ailesi: String,
    pub punto: f32,
    /// Yazı ağırlığı; tasarımın `İnce · düz · Koyu` üçlüsü.
    pub ağırlık: YazıAğırlığı,
    pub eğik: bool,
    pub altı_çizili: bool,
    pub üstü_çizili: bool,
    /// `ORT-004 §20.1` imleç hızı.
    pub imleç_hızı: İmleçHızı,
    /// `ORT-004 §20.1` imleç kalınlığı (px).
    pub imleç_kalınlığı: f32,
    /// `TemaBağlamı::metin_ölçeği`; erişilebilirlik ölçeği.
    pub metin_ölçeği: f32,
    /// `ORT-004` arayüz yoğunluğu.
    pub yoğunluk: ArayüzYoğunluğu,
    /// `ORT-004` hareket tercihi.
    pub hareket: HareketTercihi,
    /// Sistem kipini izle.
    ///
    /// Açıkken kalıcı kip seçimleri **silinmez**: sistemin güncel kipine
    /// göre biri etkinleşir. Kapatıldığında kullanıcı en son neyi seçtiyse
    /// ona döner — izleme bir üst kademe, seçimin yerine geçen bir değer
    /// değil.
    pub sistem_kipini_izle: bool,
    /// `ORT-004` metin düzenleme iç boşluğu farkı.
    ///
    /// Tema `metin_düzenleme_iç_boşluğu` alanını taşıyor ve galeri onu
    /// `None` bırakıyordu: kütüphane varsayılanı (8/8/4/4) dışına
    /// çıkılamıyor, kutunun iç boşluğu hiç denenemiyordu. `None` sıfır
    /// dolgu demek değil, "fark bildirilmedi" demek.
    pub iç_boşluk: TezgahİçBoşluğu,
    pub kip: TemaKipi,
    /// Her değişiklikte artan anlık görüntü sürümü.
    ///
    /// Sözleşme "anlık görüntü değişmezdir, aynı sürüm aynı değerleri verir"
    /// diyor. Tercih değişince yeni bir anlık görüntü üretilir ve sürümü
    /// artar; eski sürüm asla farklı değer taşımaz.
    pub sürüm: u64,
}

/// Tezgâhta seçilebilen imleç hızları.
///
/// `Platform` temanın açık tanım vermediği durumdur: davranış işletim
/// sistemi ya da tarayıcı bildiriminden çözülür. Diğerleri temanın açık
/// tanımıdır ve platformu ezer.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum İmleçHızı {
    #[default]
    Platform,
    Sabit,
    Hızlı,
    Normal,
    Yavaş,
}

impl İmleçHızı {
    pub const TÜMÜ: [Self; 5] = [
        Self::Platform,
        Self::Sabit,
        Self::Hızlı,
        Self::Normal,
        Self::Yavaş,
    ];

    pub const fn adı(self) -> &'static str {
        match self {
            Self::Platform => "Platform",
            Self::Sabit => "Sabit",
            Self::Hızlı => "Hızlı",
            Self::Normal => "Normal",
            Self::Yavaş => "Yavaş",
        }
    }

    /// `ORT-004` temanın metin imleci **adayı**.
    ///
    /// `None`, temanın hareketi ezmediği anlamına gelir: çözüm hareket
    /// tercihine, oradan platforma düşer. Aday doğrulanmamıştır; çözüm
    /// mühürlü `metin_imleci_çözümleyicisi` kapısındadır.
    pub const fn hareket(self) -> Option<gpui_bilesenleri::TemaMetinİmleciAdayı> {
        use gpui_bilesenleri::TemaMetinİmleciAdayı;
        // Görev döngüsü eşit: farklı oran isteyen ürün tokenı doğrudan
        // kurar, tezgâh yaygın olanı sunar.
        const fn eşit(ms: u64) -> Option<TemaMetinİmleciAdayı> {
            Some(TemaMetinİmleciAdayı::YanıpSönen {
                dönem: Duration::from_millis(ms * 2),
                görünür_süre: Duration::from_millis(ms),
            })
        }
        match self {
            Self::Platform => None,
            Self::Sabit => Some(TemaMetinİmleciAdayı::Sabit),
            Self::Hızlı => eşit(250),
            Self::Normal => eşit(530),
            Self::Yavaş => eşit(900),
        }
    }
}

/// Tezgâhın sunduğu yazı ağırlıkları.
///
/// Üçü de kitaplık yüzleriyle karşılanır: `İnce` Light 300, `Düz` Regular
/// 400, `Koyu` SemiBold 600. Karşılığı olmayan bir ağırlık göstermek,
/// tıklandığında hiçbir şey yapmayan bir düğme demektir.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum YazıAğırlığı {
    İnce,
    #[default]
    Düz,
    Koyu,
}

impl YazıAğırlığı {
    /// GPUI ağırlık değeri.
    ///
    /// `Koyu` 700 değil 600'dür: gömülü kalın yüzler SemiBold. Lilex yalnız
    /// Bold 700 taşır; CSS eşleme kuralı 600 isteğini en yakın üst ağırlığa,
    /// yani 700'e bağlar.
    pub const fn gpui_ağırlığı(self) -> gpui::FontWeight {
        match self {
            Self::İnce => gpui::FontWeight::LIGHT,
            Self::Düz => gpui::FontWeight::NORMAL,
            Self::Koyu => gpui::FontWeight::SEMIBOLD,
        }
    }
}

impl Default for TezgahTeması {
    fn default() -> Self {
        Self {
            yazı_ailesi: crate::KİTAPLIK_AİLELERİ[0].to_string(),
            punto: 14.,
            ağırlık: YazıAğırlığı::Düz,
            eğik: false,
            altı_çizili: false,
            üstü_çizili: false,
            imleç_hızı: İmleçHızı::Platform,
            imleç_kalınlığı: 1.5,
            metin_ölçeği: 1.0,
            yoğunluk: ArayüzYoğunluğu::Normal,
            hareket: HareketTercihi::Tam,
            sistem_kipini_izle: false,
            iç_boşluk: TezgahİçBoşluğu::Tema,
            kip: TemaKipi::Açık,
            sürüm: 1,
        }
    }
}

impl TezgahTeması {
    /// Tercih değişti: yeni anlık görüntü sürümü.
    pub fn sürümü_artır(&mut self) {
        self.sürüm = self.sürüm.saturating_add(1);
    }
}

/// `§7.3` ondalık derinliğinin üst sınırı.
///
/// Tasarımın kabul ölçütü `0–12`. Kod uzun süre `6`da duruyordu: para ve
/// yüzde için yeterliydi ama bilimsel gösterimde alan kullanıcının
/// isteyebileceği derinliğe çıkamıyordu.
pub const EN_ÇOK_ONDALIK: usize = 12;

/// `§7` kamusal tür ailesi.
///
/// Sözleşme `§7` dört aile tanımlıyor: `Metin(MetinTanımı)`,
/// `Tamsayı(TamsayıTanımı)`, `Ondalık(OndalıkTanımı)`,
/// `TarihZaman(TarihZamanTanımı)`. Para, yüzde ve bilimsel giriş beşinci bir
/// tür **değil**, `Ondalık` ailesinin biçim profilleridir; tarih, saat ve
/// tarih/saat de `TarihZaman`ın kipleridir.
///
/// Fiziksel `TezgahDeğerKipi` hâlâ dokuz düz varyant taşıyor (`§8/16` borcu).
/// Tezgâh o alanı kurmaya devam eder ama **ekranda** aileyi gösterir:
/// dokuz düğme, sözleşmenin dört ailesini beş ayrı türmüş gibi sunuyordu.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TezgahAilesi {
    Metin,
    Tamsayı,
    Ondalık,
    TarihZaman,
}

impl TezgahAilesi {
    pub const TÜMÜ: [Self; 4] = [Self::Metin, Self::Tamsayı, Self::Ondalık, Self::TarihZaman];

    pub const fn adı(self) -> &'static str {
        match self {
            Self::Metin => "Metin",
            Self::Tamsayı => "Tamsayı",
            Self::Ondalık => "Ondalık",
            Self::TarihZaman => "TarihZaman",
        }
    }

    /// Ailenin varsayılan fiziksel türü.
    ///
    /// `Ondalık` ailesi `ParaBirimi`/`Yüzde` biçim profillerine de açılır;
    /// aile seçmek o profilleri sıfırlar ve düz `Ondalık`a döner.
    pub const fn varsayılan_tür(self) -> TezgahDeğerKipi {
        match self {
            Self::Metin => TezgahDeğerKipi::Metin,
            Self::Tamsayı => TezgahDeğerKipi::Tamsayı,
            Self::Ondalık => TezgahDeğerKipi::Ondalık,
            Self::TarihZaman => TezgahDeğerKipi::Tarih,
        }
    }
}

/// Fiziksel türün ait olduğu kamusal aile.
pub const fn tür_ailesi(tür: TezgahDeğerKipi) -> TezgahAilesi {
    match tür {
        TezgahDeğerKipi::Metin => TezgahAilesi::Metin,
        TezgahDeğerKipi::Tamsayı => TezgahAilesi::Tamsayı,
        // Para ve yüzde `Ondalık` ailesinin biçim profilleridir.
        TezgahDeğerKipi::Ondalık | TezgahDeğerKipi::ParaBirimi | TezgahDeğerKipi::Yüzde => {
            TezgahAilesi::Ondalık
        }
        // Süre de `TarihZaman` ailesinin bir kipidir.
        TezgahDeğerKipi::Tarih
        | TezgahDeğerKipi::Saat
        | TezgahDeğerKipi::TarihSaat
        | TezgahDeğerKipi::Süre => TezgahAilesi::TarihZaman,
    }
}

/// `§7` `TarihZaman` ailesinin kipleri.
/// `Ondalık` ailesinin kipleri.
///
/// Sözleşme `§7` para ve yüzdeyi ayrı bir tür değil, ondalık ailesinin
/// **biçim profili** sayıyor; fiziksel `TezgahDeğerKipi` ise ikisini ayrı
/// varyant olarak taşıyor (borç 16). Tür satırı dört aile gösterdiği ve
/// bu iki varyant hiçbir kip listesinde olmadığı için ekranda **hiç
/// seçilemiyorlardı** — para biçimi de kendi türünü istediği için birlikte
/// kilitleniyordu.
pub const ONDALIK_KİPLERİ: [(&str, TezgahDeğerKipi); 3] = [
    ("Ondalık", TezgahDeğerKipi::Ondalık),
    ("Para birimi", TezgahDeğerKipi::ParaBirimi),
    ("Yüzde", TezgahDeğerKipi::Yüzde),
];

pub const TARİH_KİPLERİ: [(&str, TezgahDeğerKipi); 4] = [
    ("Tarih", TezgahDeğerKipi::Tarih),
    ("Saat", TezgahDeğerKipi::Saat),
    ("Tarih ve saat", TezgahDeğerKipi::TarihSaat),
    // Süre bu ailenin dördüncü kipi. Yorum onu ailenin parçası sayıyordu
    // ama kip listesinde yoktu: dokuz değer türünden biri ekranda hiç
    // seçilemiyordu.
    ("Süre", TezgahDeğerKipi::Süre),
];

/// `§29` bir yapılandırma çelişkisinin okunabilir karşılığı.
///
/// Ham enum adı yazılmaz: `ÇakışanİçerikYuvası` programcıya bir şey
/// söylemez, "Ön ek ile yardımcı eylem aynı yuvayı istiyor" söyler. Tablo
/// tasarımın `§8.15` çelişki/sonuç sütunlarını izler.
/// `§13` bir `Değer`in panelde okunur karşılığı.
///
/// Hassas metin **redakte edilir**: `GüvenliMetin::tanı_metni` yalnız
/// açıkça güvenli sınıflandırılmış metni tanı sınırına verir ve bu panel
/// de bir tanı yüzeyidir. Parola kipindeki bir alanın ham değerini
/// yandaki karta yazmak, kutuda gizlediğimiz şeyi iki santim yana
/// kopyalamak olurdu.
pub fn değer_özeti(değer: &gpui_bilesenleri::Değer) -> String {
    use gpui_bilesenleri::Değer;
    let güvenli = |metin: &gpui_bilesenleri::GüvenliMetin| {
        metin
            .tanı_metni()
            .map_or_else(|| "‹gizli›".to_owned(), str::to_owned)
    };
    match değer {
        Değer::Boş => "boş".to_owned(),
        Değer::Metin(metin) => güvenli(metin),
        Değer::Kimlik(metin) => güvenli(metin),
        Değer::Mantıksal(değer) => değer.to_string(),
        Değer::Tamsayı(sayı) => format!("{sayı:?}"),
        Değer::Ondalık(sayı) => format!("{sayı:?}"),
        Değer::Para { tutar, .. } => format!("{tutar:?}"),
        Değer::TarihZaman(an) => format!("{an:?}"),
        Değer::Özel(_) => "özel".to_owned(),
    }
}

/// `ORT-004` metin düzenleme iç boşluğu seçimi.
///
/// Kanonik alan kısmi bir **fark**tır: verilmeyen kenar tabandan gelir ve
/// hiçbir kenarı olmayan fark reddedilir. Tezgâh üç hazır fark sunuyor;
/// dört kenarı ayrı ayrı açmak bu ekseni sekiz denetime çıkarırdı.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TezgahİçBoşluğu {
    /// Fark bildirilmez; `ORT-017` kütüphane varsayılanı geçerlidir.
    Tema,
    Dar,
    Geniş,
}

impl TezgahİçBoşluğu {
    pub const TÜMÜ: [Self; 3] = [Self::Tema, Self::Dar, Self::Geniş];

    pub const fn adı(self) -> &'static str {
        match self {
            Self::Tema => "Temadan",
            Self::Dar => "Dar",
            Self::Geniş => "Geniş",
        }
    }

    /// Kanonik fark; `Tema` bildirim üretmez.
    pub fn kanonik(self) -> Option<gpui_bilesenleri::MantıksalİçBoşlukFarkı> {
        let (yatay, dikey) = match self {
            Self::Tema => return None,
            Self::Dar => (4., 2.),
            Self::Geniş => (16., 8.),
        };
        gpui_bilesenleri::MantıksalİçBoşlukFarkı::kenarlar(
            Some(gpui::px(yatay)),
            Some(gpui::px(yatay)),
            Some(gpui::px(dikey)),
            Some(gpui::px(dikey)),
        )
        .ok()
    }
}

/// Tezgâhın olay akışında duran tek satır.
///
/// `sayı` art arda gelen aynı olayı toplar: metin yazarken alan her tuşta
/// `DüzenlemeMetniDeğişti` yayımlıyor ve on beş satırlık akış tek bir
/// tuş dizisiyle doluyordu.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TezgahOlayı {
    pub ad: &'static str,
    pub özet: String,
    pub sayı: u32,
}

/// `§26` alanın yayımladığı olayı akış satırına çevirir.
///
/// Ad kanonik varyantın adıdır — tezgâh olayı yeniden adlandırmaz, çünkü
/// programcı `match` yazarken bu adı görecek. Özet, varyantın taşıdığı
/// yükün okunur karşılığıdır; yük yoksa boştur.
pub fn olay_özeti(olay: &gpui_bilesenleri::GirişOlayı) -> TezgahOlayı {
    use gpui_bilesenleri::GirişOlayı;
    let (ad, özet): (&'static str, String) = match olay {
        GirişOlayı::DüzenlemeMetniDeğişti {
            değer_sürümü, ..
        } => ("DüzenlemeMetniDeğişti", format!("sürüm {değer_sürümü}")),
        GirişOlayı::GeçiciDeğerDeğişti {
            değer,
            değer_sürümü,
        } => (
            "GeçiciDeğerDeğişti",
            format!(
                "{} · sürüm {değer_sürümü}",
                if değer.is_some() {
                    "değer var"
                } else {
                    "boş"
                }
            ),
        ),
        GirişOlayı::DeğerKabulEdildi { neden, .. } => ("DeğerKabulEdildi", format!("{neden:?}")),
        GirişOlayı::KabulReddedildi { sorunlar } => {
            ("KabulReddedildi", format!("{} sorun", sorunlar.len()))
        }
        GirişOlayı::OdakGeçişiReddedildi { sorunlar } => {
            ("OdakGeçişiReddedildi", format!("{} sorun", sorunlar.len()))
        }
        GirişOlayı::EskiDeğereDönüldü => ("EskiDeğereDönüldü", String::new()),
        GirişOlayı::YapıştırmaSüzüldü {
            atılan_grafem_sayısı,
        } => (
            "YapıştırmaSüzüldü",
            format!("{atılan_grafem_sayısı} grafem atıldı"),
        ),
        GirişOlayı::YapılandırmaReddedildi { hatalar } => {
            ("YapılandırmaReddedildi", format!("{} hata", hatalar.len()))
        }
        GirişOlayı::VarsayılanDeğerReddedildi(hata) => {
            ("VarsayılanDeğerReddedildi", format!("{hata:?}"))
        }
        GirişOlayı::ÜzerineYazmaDeğişti { açık } => (
            "ÜzerineYazmaDeğişti",
            (if *açık { "açık" } else { "kapalı" }).to_owned(),
        ),
        GirişOlayı::GirişReddedildi { değer_türü } => {
            ("GirişReddedildi", format!("{değer_türü:?} kümesi dışı"))
        }
        GirişOlayı::UzunlukSınırıUygulandı {
            atılan,
            birim,
            politika,
        } => (
            "UzunlukSınırıUygulandı",
            format!("{atılan} {birim:?} · {politika:?}"),
        ),
        GirişOlayı::YardımcıEylemİstendi(tür) => ("YardımcıEylemİstendi", format!("{tür:?}")),
        GirişOlayı::AramaGönderildi { kaynak, .. } => ("AramaGönderildi", format!("{kaynak:?}")),
        GirişOlayı::Hata(hata) => ("Hata", format!("{hata:?}")),
    };
    TezgahOlayı {
        ad, özet, sayı: 1
    }
}

/// Akışta tutulan en fazla satır.
///
/// Panel geçmiş defteri değil, "az önce ne oldu" penceresidir; daha uzun
/// bir liste sol kolonu kendi başına doldururdu.
pub const OLAY_AKIŞI_SINIRI: usize = 8;

pub fn çelişki_metni(hata: &GirişYapılandırmaHatası) -> &'static str {
    use GirişYapılandırmaHatası as H;
    match hata {
        H::TürVeMaskeUyumsuz => "Maske türü ile değer türü uyumsuz",
        H::GeçersizEnterDavranışı => "Enter davranışı bu alan kipinde kurulamaz",
        H::UyumsuzSeçici => "Uyumsuz türde görünür seçici",
        H::GeçersizMaskeGrafemi => "Maske grafemi tek grafem değil",
        H::GeçersizOdakPolitikası => "Geçersiz odakta OdağıKoru + odaklanamaz alan",
        H::GeçersizBasamakSınırı => "Basamak sınırında en az, en fazladan büyük",
        H::GeçersizSunumPolitikası => "Geçici göster görünürlüğü politikasız kurulamaz",
        H::GeçersizSayısalAdım => "Sayısal adım sıfır/negatif ya da sarma için sınır eksik",
        H::ÇakışanTuşDavranışı => "İki tuş davranışı aynı tuşu istiyor",
        H::ÇakışanİçerikYuvası => "İki içerik aynı yuvayı istiyor",
        H::GeçersizYardımcıEylem => "Yardımcı yuva ekseni uyuşmuyor ya da yineleniyor",
        H::GeçersizBitişikBölüt => "Bitişik bölüt kuşağı geçersiz",
        H::UyumsuzAramaGönderimi => "AramayıBaşlat yuvası yokken arama gönderimi kuruldu",
        H::ÇakışanÇalışmaBağı => "Çalışma bağı başka bir eksenle çakışıyor",
        H::GeçersizUzunlukSınırı => "Uzunluk sınırı bu türde kurulamaz",
        H::GeçersizSayaç => "Sayaç yapılandırması geçersiz",
        H::GeçersizOdaklıBiçimPlanı => "Odaklı biçim planı geçersiz",
        H::GeçersizTarihZamanPlanı => "Tarih/zaman planı geçersiz",
        H::GerekliGenişMotorKapalı => "Gerekli geniş motor kapalı",
        H::KaynakKısıtıÇözülemedi => "Kaynak kısıtı çözülemedi",
        H::YapılandırmaEskidi => "Yapılandırma eskidi",
        H::GeçersizKaynakBütçesi => "Kaynak bütçesi geçersiz",
        H::GirişYüzeyBağıEksik => "Giriş yüzey bağı eksik",
        H::GeçersizDurumAçıklamaProfili => "Durum açıklama profili geçersiz",
        H::GeçersizSeçiciYüzeyProfili => "Seçici yüzey profili geçersiz",
        H::MaskeTokenTavanıAşıldı => "Maske token tavanı aşıldı",
    }
}

/// `§29` bir yapılandırma uyarısının okunabilir karşılığı.
///
/// Uyarı hata değildir: yapılandırma kurulur ama bir davranış sürprizi
/// taşır. Tasarımın tablosunda ayrı bir renk sınıfı olması bu yüzden.
pub fn uyarı_metni(uyarı: &GirişYapılandırmaUyarısı) -> &'static str {
    use GirişYapılandırmaUyarısı as U;
    match uyarı {
        U::SekmeDurağıYokkenSonrakineGeçiş => {
            "Sekme durağı kapalı ama Enter sonrakine geçiyor · üst odak yöneticisi çözer"
        }
        U::BoşMetinReddiZorunluKuralıOlmadan => {
            "Boş metin reddediliyor ama Zorunlu kuralı yok · bilinçli fark olabilir"
        }
        U::YardımcıEylemAdsız => {
            "Yardımcı eylem adsız · ekran okuyucuda yalnız düğme diye okunur"
        }
        U::ErişilebilirAdYok => {
            "Alanın erişilebilir adı yok · adı üst bileşen taşımıyorsa ağaçta görünmez"
        }
    }
}

/// `§23` bitişik bölütün türü.
///
/// Kanonik `BitişikBölüt` ayrıca `kendi_sınırı` ve `opaklık_kademeli`
/// taşır; tezgâhta bunlar tek bir kademe ekseninden gelir çünkü ikisini
/// ayrı ayrı açmak, tasarımda karşılığı olmayan dört kombinasyon üretirdi.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TezgahBölütü {
    /// Sabit metin; tasarımın örneği `https://` ön ekidir.
    SabitMetin,
    /// Eylem bölütü. Kuşak alan kabuğunun **dışındadır** ve mantıksal satır
    /// sırasına girmez; bu yüzden yardımcı eylem yuvasıyla çakışmaz.
    Eylem,
}

impl TezgahBölütü {
    /// Tasarımın sabit metin örneği (`https://`).
    ///
    /// Kanonik `BitişikBölütKuşağı` bugün yalnız `başlangıç`/`bitiş`
    /// taşıyor; bölütün **içeriği** ayrı bir `BitişikEylemBölütü` tipinde ve
    /// `GirişYapılandırması`'na bağlı değil. Bu yüzden metin ekranda
    /// örnek olarak yazılır, yapılandırmaya girmez.
    pub const SABİT_METİN: &'static str = "https://";

    pub const fn adı(self) -> &'static str {
        match self {
            Self::SabitMetin => "Sabit metin",
            Self::Eylem => "Eylem",
        }
    }

    fn kanonik(self, kademeli: bool, kendi_sınırı: bool) -> BitişikBölüt {
        BitişikBölüt {
            tür: match self {
                Self::SabitMetin => BitişikBölütTürü::Sabit,
                Self::Eylem => BitişikBölütTürü::Eylem,
            },
            kendi_sınırı,
            opaklık_kademeli: kademeli,
        }
    }
}

/// `§10` yapıştırma dönüşümü ekseni.
///
/// Kanonik `MetinYapıştırmaDönüşümü::TanımlıYerelAyarlarıDene` bir
/// `Vec<DilEtiketi>` taşır. Tezgâhta serbest etiket yazdırmıyoruz: geçersiz
/// bir etiket `ORT-002` doğrulamasından döner ve eksen "çalışmıyor" gibi
/// görünürdü. Bunun yerine sabit bir deneme kümesi sunulur; kod paneli
/// gerçek `motor.dil_etiketi(...)` çağrısını yazar.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TezgahYapıştırması {
    #[default]
    Katı,
    GeçerliKarakterleriSüz,
    YerelBiçimiAyıkla,
    /// `tr-TR` ve `en-US` sırasıyla denenir.
    TanımlıYerelAyarlarıDene,
}

impl TezgahYapıştırması {
    /// Tasarımın deneme kümesi; sıra anlamlıdır (ilk eşleşen kazanır).
    const YERELLER: [&'static str; 2] = ["tr-TR", "en-US"];

    pub const TÜMÜ: [Self; 4] = [
        Self::Katı,
        Self::GeçerliKarakterleriSüz,
        Self::YerelBiçimiAyıkla,
        Self::TanımlıYerelAyarlarıDene,
    ];

    pub const fn adı(self) -> &'static str {
        match self {
            Self::Katı => "Katı",
            Self::GeçerliKarakterleriSüz => "Geçerli karakterleri süz",
            Self::YerelBiçimiAyıkla => "Yerel biçimi ayıkla",
            Self::TanımlıYerelAyarlarıDene => "Tanımlı yerelleri dene",
        }
    }

    /// Dil etiketleri `ORT-002` motorunun doğrulama kapısından geçer;
    /// tezgâh etiket mühürlemez.
    pub fn kanonik(
        self,
        motor: &gpui_bilesenleri_temel::UnicodeMetinMotoru,
    ) -> MetinYapıştırmaDönüşümü {
        match self {
            Self::Katı => MetinYapıştırmaDönüşümü::Katı,
            Self::GeçerliKarakterleriSüz => {
                MetinYapıştırmaDönüşümü::GeçerliKarakterleriSüz
            }
            Self::YerelBiçimiAyıkla => MetinYapıştırmaDönüşümü::YerelBiçimiAyıkla,
            Self::TanımlıYerelAyarlarıDene => {
                MetinYapıştırmaDönüşümü::TanımlıYerelAyarlarıDene {
                    // `expect` güvenli: etiketler derleme zamanı sabitleri ve
                    // testte de doğrulanıyor. Kullanıcı girdisi değiller.
                    yerel_ayarlar: Self::YERELLER
                        .iter()
                        .map(|etiket| {
                            motor
                                .dil_etiketi(etiket)
                                .expect("sabit dil etiketi kayıtlarda tanınır")
                        })
                        .collect(),
                }
            }
        }
    }

    /// Kod panelindeki karşılığı. `Katı` varsayılandır ve yazılmaz.
    pub fn kod(self) -> Option<String> {
        let gövde = match self {
            Self::Katı => return None,
            Self::GeçerliKarakterleriSüz => "GeçerliKarakterleriSüz".to_owned(),
            Self::YerelBiçimiAyıkla => "YerelBiçimiAyıkla".to_owned(),
            Self::TanımlıYerelAyarlarıDene => format!(
                "TanımlıYerelAyarlarıDene {{\n        yerel_ayarlar: vec![\n{}\n        ],\n    }}",
                Self::YERELLER
                    .iter()
                    .map(|etiket| format!(
                        "            motor.dil_etiketi({etiket:?}).expect(\"geçerli etiket\"),"
                    ))
                    .collect::<Vec<_>>()
                    .join("\n")
            ),
        };
        Some(format!(
            "yapılandırma.yapıştırma = MetinYapıştırmaDönüşümü::{gövde};"
        ))
    }
}

/// `§22` içerik görünürlüğü ekseni.
///
/// Kanonik `İçerikGörünürlüğü` dört durumludur; tezgâh bunu uzun süre bir
/// `bool` ile sunuyordu ve `GeçiciGöster` ile `Opak` ekranda hiç yoktu.
/// `Opak` bir "daha gizli"lik kademesi değildir: `Gizli` değeri tutup
/// maskeler, `Opak` değeri **hiç almamıştır** — reveal, kopyalama ve
/// düzenleme yoktur.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TezgahGörünürlüğü {
    #[default]
    Açık,
    Gizli,
    GeçiciGöster,
    Opak,
}

impl TezgahGörünürlüğü {
    /// Tasarımın `§8.9` maske grafemi. Birden fazla grafem `§29` hatasıdır.
    const GRAFEM: &'static str = "•";
    /// Opak yer tutucunun sabit uzunluğu.
    ///
    /// Gerçek uzunluk yazılamaz: `ORT-019.ACC-006` yer tutucunun kaynağın
    /// uzunluğunu açığa vurmasını yasaklar.
    const OPAK_UZUNLUK: usize = 8;

    pub const TÜMÜ: [Self; 4] = [Self::Açık, Self::Gizli, Self::GeçiciGöster, Self::Opak];

    pub const fn adı(self) -> &'static str {
        match self {
            Self::Açık => "Açık",
            Self::Gizli => "Gizli",
            Self::GeçiciGöster => "Geçici göster",
            Self::Opak => "Opak",
        }
    }

    /// Değer maskeleniyor mu? Önizleme metni bunu okur.
    pub const fn maskeli(self) -> bool {
        !matches!(self, Self::Açık)
    }

    /// `ParolayıGöster` yuvası yalnız `Gizli` ve `GeçiciGöster`de bulunur.
    ///
    /// `Opak`ta reveal **yoktur**: elde olmayan değer gösterilemez.
    pub const fn parola_yuvası_var(self) -> bool {
        matches!(self, Self::Gizli | Self::GeçiciGöster)
    }

    pub fn kanonik(self) -> İçerikGörünürlüğü {
        match self {
            Self::Açık => İçerikGörünürlüğü::Açık,
            Self::Gizli => İçerikGörünürlüğü::Gizli {
                maske_grafemi: Self::GRAFEM.into(),
            },
            Self::GeçiciGöster => İçerikGörünürlüğü::GeçiciGöster {
                maske_grafemi: Self::GRAFEM.into(),
            },
            Self::Opak => İçerikGörünürlüğü::Opak {
                maske_grafemi: Self::GRAFEM.into(),
                yer_tutucu_uzunluğu: Self::OPAK_UZUNLUK,
            },
        }
    }

    /// Kod panelindeki karşılığı. `Açık` varsayılandır ve yazılmaz.
    pub fn kod(self) -> Option<String> {
        let gövde = match self {
            Self::Açık => return None,
            Self::Gizli => format!("Gizli {{ maske_grafemi: {:?}.into() }}", Self::GRAFEM),
            Self::GeçiciGöster => {
                format!(
                    "GeçiciGöster {{ maske_grafemi: {:?}.into() }}",
                    Self::GRAFEM
                )
            }
            Self::Opak => format!(
                "Opak {{\n        maske_grafemi: {:?}.into(),\n        \
                 yer_tutucu_uzunluğu: {},\n    }}",
                Self::GRAFEM,
                Self::OPAK_UZUNLUK
            ),
        };
        Some(format!(
            "yapılandırma.içerik_görünürlüğü =\n    İçerikGörünürlüğü::{gövde};"
        ))
    }
}

/// `§22` geçici gösterimin geri dönüş politikası.
///
/// Süre serbest yazılmaz: geçersiz bir süre girişi ekseni "çalışmıyor"
/// gibi gösterirdi. Tezgâh sabit bir deneme süresi sunar; kod paneli
/// gerçek `Duration` kuruluşunu yazar.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TezgahGeçiciGösterimi {
    BasılıTutarken,
    ZamanSınırlı,
    /// Mevcut göster/gizle davranışının adı; en az sürprizli varsayılan.
    #[default]
    TekrarEtkinleştireneKadar,
}

impl TezgahGeçiciGösterimi {
    /// Tasarımın deneme süresi: gözle izlenebilecek kadar uzun, tezgâh
    /// gezintisini bekletmeyecek kadar kısa.
    const SÜRE_SANİYE: u64 = 3;

    pub const TÜMÜ: [Self; 3] = [
        Self::BasılıTutarken,
        Self::ZamanSınırlı,
        Self::TekrarEtkinleştireneKadar,
    ];

    pub const fn adı(self) -> &'static str {
        match self {
            Self::BasılıTutarken => "Basılı tutarken",
            Self::ZamanSınırlı => "Zaman sınırlı",
            Self::TekrarEtkinleştireneKadar => "Tekrar etkinleştirene kadar",
        }
    }

    pub fn kanonik(self) -> GeçiciGösterimPolitikası {
        match self {
            Self::BasılıTutarken => GeçiciGösterimPolitikası::BasılıTutarken,
            Self::ZamanSınırlı => GeçiciGösterimPolitikası::ZamanSınırlı {
                süre: std::time::Duration::from_secs(Self::SÜRE_SANİYE),
            },
            Self::TekrarEtkinleştireneKadar => {
                GeçiciGösterimPolitikası::TekrarEtkinleştireneKadar
            }
        }
    }

    /// Kod panelindeki karşılığı; yalnız `GeçiciGöster` seçiliyken yazılır.
    pub fn kod(self) -> String {
        let gövde = match self {
            Self::BasılıTutarken => "BasılıTutarken".to_owned(),
            Self::ZamanSınırlı => format!(
                "ZamanSınırlı {{\n        süre: std::time::Duration::from_secs({}),\n    }}",
                Self::SÜRE_SANİYE
            ),
            Self::TekrarEtkinleştireneKadar => "TekrarEtkinleştireneKadar".to_owned(),
        };
        format!("yapılandırma.geçici_gösterim =\n    Some(GeçiciGösterimPolitikası::{gövde});")
    }
}

/// Tezgâhta seçilebilen maske kipleri.
///
/// `Desen` kullanıcının yazdığı `§9.1` şablonunu kullanır; hazır desenler
/// yalnız o alanı dolduran kısayollardır, ayrı bir maske türü değildir.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TezgahMaskesi {
    Yok,
    Desen,
    /// `§9.5` bölüm gezinimi yalnız maskede vardır; tarih maskesi onu
    /// denenebilir kılar.
    Tarih,
}

impl TezgahMaskesi {
    pub const fn adı(self) -> &'static str {
        match self {
            Self::Yok => "Yok",
            Self::Desen => "Desen",
            Self::Tarih => "gg.aa.yyyy",
        }
    }
}

/// Tasarımın geniş biçim listesindeki bir seçenek.
///
/// Liste `metinkutusu.cozulmus.html` 433–467. satırlardan alındı ve üç
/// öbekten oluşur: `Biçim` (`ORT-008` biçim tanımı), `Tarih ve saat` (yine
/// `ORT-008`) ve `Giriş maskesi` (`BİL-010 §9.1` desenleri).
///
/// Bir seçeneğin kanonik karşılığı yoksa `uygulanabilir_mi` `false` döner ve
/// tezgâh onu pasif çizer. Kanıtsız bir seçeneği çalışıyormuş gibi göstermek,
/// programcıya var olmayan bir yetenek satmaktır.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BiçimSeçeneği {
    pub öbek: BiçimÖbeği,
    pub etiket: &'static str,
    pub uygulama: BiçimUygulaması,
}

/// Tasarımın tarih ve saat satırlarının karşılığı.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TarihKipi {
    KısaTarih,
    UzunTarih,
    Saat,
    UzunSaat,
    TarihSaat,
}

impl TarihKipi {
    /// `ORT-008` biçim tanımı.
    pub fn biçim_tanımı(self) -> BiçimTanımı {
        let sayısal_tarih = AçıkTarihBiçimi {
            yıl: TarihParçasıGösterimi::Sayısal,
            ay: TarihParçasıGösterimi::İkiHane,
            gün: TarihParçasıGösterimi::İkiHane,
            hafta_günü: TarihParçasıGösterimi::Gizli,
        };
        let uzun_tarih = AçıkTarihBiçimi {
            yıl: TarihParçasıGösterimi::Sayısal,
            ay: TarihParçasıGösterimi::UzunAd,
            gün: TarihParçasıGösterimi::Sayısal,
            hafta_günü: TarihParçasıGösterimi::Gizli,
        };
        let saat = |saniye: bool| AçıkSaatBiçimi {
            döngü: SaatDöngüsü::YerelVarsayılan,
            saniye,
            alt_saniye_basamağı: 0,
            saat_dilimi: SaatDilimiGösterimi::Gizli,
        };
        match self {
            Self::KısaTarih => BiçimTanımı::AçıkTarih(sayısal_tarih),
            Self::UzunTarih => BiçimTanımı::AçıkTarih(uzun_tarih),
            Self::Saat => BiçimTanımı::AçıkSaat(saat(false)),
            Self::UzunSaat => BiçimTanımı::AçıkSaat(saat(true)),
            Self::TarihSaat => BiçimTanımı::AçıkTarihSaat(AçıkTarihSaatBiçimi {
                tarih: sayısal_tarih,
                saat: saat(false),
            }),
        }
    }

    /// Bu kipin çalıştığı giriş değer türü.
    pub const fn değer_türü(self) -> TezgahDeğerKipi {
        match self {
            Self::KısaTarih | Self::UzunTarih => TezgahDeğerKipi::Tarih,
            Self::Saat | Self::UzunSaat => TezgahDeğerKipi::Saat,
            Self::TarihSaat => TezgahDeğerKipi::TarihSaat,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BiçimÖbeği {
    Biçim,
    TarihSaat,
    Maske,
}

impl BiçimÖbeği {
    pub const fn başlığı(self) -> &'static str {
        match self {
            Self::Biçim => "Biçim",
            Self::TarihSaat => "Tarih ve saat",
            Self::Maske => "Giriş maskesi",
        }
    }
}

/// Bir seçeneğin tezgâh tercihlerine nasıl indiği.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BiçimUygulaması {
    /// `BiçimYapılandırması::Genel`; tür ne ise onun genel gösterimi.
    Genel,
    /// `ORT-008` sayı gösterimi; `gruplama` binler ayracını kurar.
    Sayı { gruplama: bool },
    /// Para birimi gösterimi.
    Para,
    /// Yüzde gösterimi.
    Yüzde,
    /// Ham metin; biçimlendirme yok.
    Metin,
    /// `ORT-008 §8.1` tarih/saat gösterimi; `açık` alanı parçaları seçer.
    Tarih(TarihKipi),
    /// `ORT-008 §7.1` bilimsel gösterim.
    Bilimsel,
    /// `§9.5` bölümlü tarih maskesi; bölüm gezinimi burada denenir.
    BölümlüTarih,
    /// `ORT-008 §7.2` kesir gösterimi.
    Kesir,
    /// `§9.1` desen maskesi.
    Desen(&'static str),
    /// `ORT-008` süre gösterimi; en küçük ve en büyük birim çifti.
    Süre(SüreBirimi, SüreBirimi),
    /// Gizli içerik; maske değil `§22` görünürlük tercihi.
    Gizli,
    /// Deseni kullanıcının yazması için desen alanını açar.
    ÖzelDesen,
    /// Kanonik karşılığı henüz yok; pasif çizilir.
    Eksik(&'static str),
}

/// Tasarımın biçim listesi, sırası bozulmadan.
pub const BİÇİM_SEÇENEKLERİ: &[BiçimSeçeneği] = &[
    // --- Biçim
    seçenek(BiçimÖbeği::Biçim, "Genel", BiçimUygulaması::Genel),
    seçenek(
        BiçimÖbeği::Biçim,
        "Sayı · 1234,56",
        BiçimUygulaması::Sayı { gruplama: false },
    ),
    seçenek(
        BiçimÖbeği::Biçim,
        "Binlik ayraçlı · 1.234,56",
        BiçimUygulaması::Sayı { gruplama: true },
    ),
    seçenek(
        BiçimÖbeği::Biçim,
        "Para birimi · ₺1.234,56",
        BiçimUygulaması::Para,
    ),
    seçenek(
        BiçimÖbeği::Biçim,
        "Muhasebe · ₺ 1.234,56",
        // `ParaBiçimi` birim gösterimini seçtirir ama yerleşimini değil:
        // muhasebe biçiminde simge kutunun soluna, tutar sağına yaslanır.
        BiçimUygulaması::Eksik("ORT-008 para yerleşimi tanımlamıyor"),
    ),
    seçenek(BiçimÖbeği::Biçim, "Yüzde · %12,50", BiçimUygulaması::Yüzde),
    seçenek(
        BiçimÖbeği::Biçim,
        "Bilimsel · 1,23E+04",
        BiçimUygulaması::Bilimsel,
    ),
    seçenek(BiçimÖbeği::Biçim, "Kesir · 1/4", BiçimUygulaması::Kesir),
    seçenek(BiçimÖbeği::Biçim, "Metin", BiçimUygulaması::Metin),
    // --- Tarih ve saat
    //
    // Tür ve biçim tanımı `ORT-008`'de var (`BiçimTanımı::Tarih` ve
    // kardeşleri) ama `YerelSayıMotoru::biçimlendir` bu dallar için bir
    // uygulama taşımıyor: çağrı `BiçimHatası::TürUyuşmazlığı` döner.
    seçenek(
        BiçimÖbeği::TarihSaat,
        "Kısa tarih · 04.08.2026",
        BiçimUygulaması::Tarih(TarihKipi::KısaTarih),
    ),
    seçenek(
        BiçimÖbeği::TarihSaat,
        "Uzun tarih · 4 Ağustos 2026",
        BiçimUygulaması::Tarih(TarihKipi::UzunTarih),
    ),
    seçenek(
        BiçimÖbeği::TarihSaat,
        "Saat · 14:30",
        BiçimUygulaması::Tarih(TarihKipi::Saat),
    ),
    seçenek(
        BiçimÖbeği::TarihSaat,
        "Uzun saat · 14:30:05",
        BiçimUygulaması::Tarih(TarihKipi::UzunSaat),
    ),
    seçenek(
        BiçimÖbeği::TarihSaat,
        "Tarih/saat · 04.08.2026 14:30",
        BiçimUygulaması::Tarih(TarihKipi::TarihSaat),
    ),
    // `ORT-008` süre gösterimi birim **çifti** ister: en küçük ve en
    // büyük. Tek bir "süre" seçeneği hangi birimlerin görüneceğini
    // söylemezdi.
    seçenek(
        BiçimÖbeği::TarihSaat,
        "Süre · 02:45",
        BiçimUygulaması::Süre(SüreBirimi::Dakika, SüreBirimi::Saat),
    ),
    seçenek(
        BiçimÖbeği::TarihSaat,
        "Süre · 02:45:30",
        BiçimUygulaması::Süre(SüreBirimi::Saniye, SüreBirimi::Saat),
    ),
    seçenek(
        BiçimÖbeği::TarihSaat,
        "Süre · 45:30",
        BiçimUygulaması::Süre(SüreBirimi::Saniye, SüreBirimi::Dakika),
    ),
    // --- Giriş maskesi
    seçenek(
        BiçimÖbeği::Maske,
        "Telefon · 0(500) 000 00 00",
        // Baştaki `0` yazılan bir rakam değil, numaranın değişmez ön
        // ekidir; `\` kaçışıyla sabit karakter olarak kurulur (kullanıcı
        // kararı, Ağu 2026). Kaçışsız `0` zorunlu rakam yuvası olurdu.
        // Sabit hat ve cep ayrı satırlardı; desenleri özdeşleşince tek
        // `Telefon` satırında birleştiler.
        BiçimUygulaması::Desen("\\0(000) 000 00 00"),
    ),
    seçenek(
        BiçimÖbeği::Maske,
        "Tarih · 00/00/0000",
        BiçimUygulaması::Desen("00/00/0000"),
    ),
    seçenek(
        BiçimÖbeği::Maske,
        "Tarih · gg.aa.yyyy (bölümlü)",
        BiçimUygulaması::BölümlüTarih,
    ),
    seçenek(
        BiçimÖbeği::Maske,
        "Saat · 00:00",
        BiçimUygulaması::Desen("00:00"),
    ),
    seçenek(
        BiçimÖbeği::Maske,
        "TC kimlik no · 00000000000",
        BiçimUygulaması::Desen("00000000000"),
    ),
    seçenek(
        BiçimÖbeği::Maske,
        "Vergi no · 0000000000",
        BiçimUygulaması::Desen("0000000000"),
    ),
    seçenek(
        BiçimÖbeği::Maske,
        "Posta kodu · 00000",
        BiçimUygulaması::Desen("00000"),
    ),
    seçenek(
        BiçimÖbeği::Maske,
        "Plaka · >00 L?? 0999",
        BiçimUygulaması::Desen(">00 L?? 0999"),
    ),
    seçenek(
        BiçimÖbeği::Maske,
        "IBAN · \"TR\"00 0000 0000 0000 0000 0000 00",
        BiçimUygulaması::Desen(r#""TR"00 0000 0000 0000 0000 0000 00"#),
    ),
    seçenek(
        BiçimÖbeği::Maske,
        "Kredi kartı · 0000 0000 0000 0000",
        BiçimUygulaması::Desen("0000 0000 0000 0000"),
    ),
    seçenek(
        BiçimÖbeği::Maske,
        "Parola · ••••••••",
        BiçimUygulaması::Gizli,
    ),
    seçenek(
        BiçimÖbeği::Maske,
        "Kısaltma · >LLL",
        BiçimUygulaması::Desen(">LLL"),
    ),
    seçenek(BiçimÖbeği::Maske, "Özel…", BiçimUygulaması::ÖzelDesen),
];

/// Bir desen maskesinin `BİÇİM_SEÇENEKLERİ` içindeki sırası.
///
/// Maske deseni ile biçim listesi aynı şeyi iki yerden söyler: içerik türü
/// telefon seçince desen kurulur, ama biçim listesi hâlâ `Genel`de kalırsa
/// ekranda iki farklı yanıt görünür.
fn desen_seçeneği(desen: &str) -> usize {
    BİÇİM_SEÇENEKLERİ
        .iter()
        .position(|s| matches!(s.uygulama, BiçimUygulaması::Desen(d) if d == desen))
        .unwrap_or(0)
}

/// Tercihin kendi kurduğu ön ek metinleri.
///
/// Kullanıcı ön ek kutusuna kendi metnini yazdıysa tür değişimi onu
/// silmez; yalnız tezgâhın daha önce **kendi** yazdığı bir değer yeniden
/// türetilir.
const OTOMATİK_ÖN_EKLER: &[&str] = &[TELEFON_ÖN_EKİ, VARSAYILAN_ÖN_EK, URL_ÖN_EKİ];

/// Ön ek kutusunun açılış metni.
pub const VARSAYILAN_ÖN_EK: &str = "₺";
/// Son ek kutusunun açılış metni.
pub const VARSAYILAN_SON_EK: &str = "KDV dahil";
const TELEFON_ÖN_EKİ: &str = "+90";
const URL_ÖN_EKİ: &str = "https://";

const fn seçenek(
    öbek: BiçimÖbeği,
    etiket: &'static str,
    uygulama: BiçimUygulaması,
) -> BiçimSeçeneği {
    BiçimSeçeneği {
        öbek,
        etiket,
        uygulama,
    }
}

impl BiçimSeçeneği {
    /// Kanonik karşılığı var mı?
    pub const fn uygulanabilir_mi(&self) -> bool {
        !matches!(self.uygulama, BiçimUygulaması::Eksik(_))
    }

    /// Uygulanamıyorsa nedeni.
    pub const fn eksiklik_nedeni(&self) -> Option<&'static str> {
        match self.uygulama {
            BiçimUygulaması::Eksik(neden) => Some(neden),
            _ => None,
        }
    }
}

impl Default for TezgahTercihleri {
    fn default() -> Self {
        Self {
            değer_türü: TezgahDeğerKipi::Metin,
            metin_içerik_türü: MetinİçerikTürü::Düz,
            // `Düz` içerik türüyle açılıyoruz; maske beklentiyi daraltır ve
            // ekranda "Düz metin" yazarken kutunun telefon numarası
            // beklemesine yol açardı.
            maske: TezgahMaskesi::Yok,
            desen: crate::HAZIR_DESENLER[0].1.to_owned(),
            biçim_seçeneği: 0,
            ondalık_basamak: 2,
            binler_ayracı: false,
            ön_ek: false,
            ön_ek_metni: VARSAYILAN_ÖN_EK.to_owned(),
            son_ek: false,
            son_ek_metni: VARSAYILAN_SON_EK.to_owned(),
            yer_tutucu: true,
            temizle: true,
            arama: false,
            parola_düğmesi: false,
            seçici: false,
            yuva_görünürlüğü: YardımcıEylemGörünürlüğü::DeğerVarkenKademeli,
            yuvalar_etkin: true,
            arama_gönderime_bağlı: false,
            ürün_eylemi: false,
            erişilebilir_ad: true,
            yuva_adları: true,
            bölüt_sınırı: true,
            görünürlük: TezgahGörünürlüğü::Açık,
            geçici_gösterim: TezgahGeçiciGösterimi::TekrarEtkinleştireneKadar,
            dış_hata_temizleme: DışHataTemizleme::YerelDüzenlemedeTemizle,
            işaret_konumu: İşaretKonumu::Sonda,
            gösterge_ankrajı: Some(DurumGöstergesiYerleşimTercihi::SatırSonu),
            gösterge_açıklaması: DurumGöstergesiAçıklamaTercihi::Yok,
            harf_dönüşümü: HarfDönüşümü::Yok,
            kırpma: KırpmaPolitikası::Yok,
            boş_metin: BoşMetinPolitikası::BoşDeğer,
            yapıştırma: TezgahYapıştırması::Katı,
            escape: EscapeDavranışı::EskiDeğereDön,
            geçersiz_odak: GeçersizOdakDavranışı::OdakKaybınaİzinVer,
            başlangıç_bölütü: None,
            bitiş_bölütü: None,
            bölüt_kademeli: true,
            çalışırken_enter: ÇalışırkenEnterPolitikası::Yoksay,
            arama_enter_gönderir: true,
            arama_temizleme_gönderir: false,
            seçici_görünürlüğü: SeçiciGörünürlüğü::UyumluTürdeGöster,
            zorunlu: false,
            doğrulama_tetikleyicisi: GeçerlilikTetikleyicisi::Kabulde,
            doğrulama_önemi: GeçerlilikÖnemi::Hata,
            ilk_hatada_dur: false,
            uzunluk_sınırı: false,
            sayısal_adım: false,
            adım_ölçeği: AdımÖlçeği::Birim,
            adım_hizala: false,
            adım_sınırı: false,
            adım_sarma: false,
            varsayılan_değer: false,
            sıfırlama: gpui_bilesenleri::SıfırlamaDavranışı::BoşaDön,
            bölüm_gezinimi: false,
            bölüm_atla: true,
            bölüm_dolunca_ilerle: true,
            bölüm_artır: true,
            bölüm_taşar: false,
            bölüm_ayraç: true,
            otomatik_doldurma: false,
            doldurma_amacı: gpui_bilesenleri::OtomatikDoldurmaAmacı::KullanıcıAdı,
            uzunluk_davranışı: UzunlukSınırıDavranışı::Kırp,
            sayaç: false,
            sayaç_birimi: SayımBirimi::Grafem,
            sayaç_sınırı_göster: true,
            hizalama: GirişYatayHizalama::Genel,
            dikey: GirişDikeyHizalama::Orta,
            ek_sunum_rolü: SabitİçerikSunumRolü::İkincil,
            şekil: DüğmeŞekli::Yuvarlatılmış,
            şekil_oto: false,
            parça_ailesi: None,
            önem_zemini: false,
            köşe_pikseli: None,
            sekme_durağı: true,
            odak_seçimi: OdakSeçimi::TümünüSeç,
            kabul_seçimi: KabulSeçimi::TümünüSeç,
            dış_tıklamada_odağı_bırak: true,
            üzerine_yazma: false,
            enter: EnterDavranışı::DeğeriİşleVeKal,
            salt_okunur: false,
            etkin: true,
            saat_dilimi_tercihi: SaatDilimiTercihi::Platform,
            tema: TezgahTeması::default(),
        }
    }
}

impl TezgahTercihleri {
    /// Seçili değer türü sayısal mı?
    ///
    /// `Yüzde` de sayısaldır: kanonik doğrulama onu maske yasağında
    /// diğer sayısal türlerle aynı kefeye koyar, tezgâh dışarıda
    /// bırakınca `Yüzde` + desen bileşimi geçersiz yüzey üretiyordu.
    pub const fn sayısal_mı(&self) -> bool {
        matches!(
            self.değer_türü,
            TezgahDeğerKipi::Tamsayı
                | TezgahDeğerKipi::Ondalık
                | TezgahDeğerKipi::ParaBirimi
                | TezgahDeğerKipi::Yüzde
        )
    }

    /// Ondalık basamak tercihi bu türde anlamlı mı?
    ///
    /// Tamsayıda ondalık yoktur; metin türünde sayısal biçim yoktur.
    pub const fn ondalık_anlamlı_mı(&self) -> bool {
        matches!(
            self.değer_türü,
            TezgahDeğerKipi::Ondalık | TezgahDeğerKipi::ParaBirimi | TezgahDeğerKipi::Yüzde
        )
    }

    /// Değer türü tarih, saat veya tarih/saat mi?
    ///
    /// Saat dilimi tercihi yalnız bu türlerde görünür etkiye sahiptir.
    pub const fn tarih_türü_mü(&self) -> bool {
        matches!(
            self.değer_türü,
            TezgahDeğerKipi::Tarih | TezgahDeğerKipi::Saat | TezgahDeğerKipi::TarihSaat
        )
    }

    /// Bu türde kurulabilecek maske kipleri.
    ///
    /// `§9.3` sayısal alanda maske yoktur: sayısal düzenleme yapısı `ORT-008`
    /// biçim planından gelir, ayrı bir maskeden değil.
    pub fn maske_seçenekleri(&self) -> &'static [TezgahMaskesi] {
        if self.sayısal_mı() {
            &[TezgahMaskesi::Yok]
        } else if self.tarih_türü_mü() {
            // Tarih maskesi saat ve tarih/saat türlerinde de kurulur; desen
            // maskesi orada bölüt yapısını ikinci kez tanımlardı.
            &[TezgahMaskesi::Yok, TezgahMaskesi::Tarih]
        } else {
            &[TezgahMaskesi::Yok, TezgahMaskesi::Desen]
        }
    }

    /// Bu seçenek seçili değer türünde kurulabilir mi?
    ///
    /// Excel'deki gibi: biçim veriye uyar, veri biçime değil. Sayısal bir
    /// biçim metin alanında, desen maskesi sayısal alanda kurulamaz. Uymayan
    /// seçenek listeden silinmez, pasif çizilir — böylece programcı neyin
    /// hangi türde açıldığını görür.
    pub fn seçenek_uygun_mu(&self, seçenek: &BiçimSeçeneği) -> bool {
        if !seçenek.uygulanabilir_mi() {
            return false;
        }
        match seçenek.uygulama {
            BiçimUygulaması::Genel => true,
            BiçimUygulaması::Sayı { .. } => self.sayısal_mı(),
            // İkisi de ondalık değer üzerinde çalışır; tamsayı ölçeği sıfır
            // ondalıktır, para birimi ise ayrı bir gösterim taşır.
            BiçimUygulaması::Bilimsel | BiçimUygulaması::Kesir => matches!(
                self.değer_türü,
                TezgahDeğerKipi::Tamsayı | TezgahDeğerKipi::Ondalık
            ),
            BiçimUygulaması::Para => self.değer_türü == TezgahDeğerKipi::ParaBirimi,
            // `BiçimTanımı::Yüzde` bir ondalık değeri ölçekler; tamsayı ya da
            // para değerinde karşılığı yok.
            BiçimUygulaması::Yüzde => self.değer_türü == TezgahDeğerKipi::Ondalık,
            // Tarih ve saat biçimleri kendi değer türünü kurar. Tür satırı
            // tasarımda dört düğme taşır ve tarih türü orada yok; seçimin
            // türü değiştirmesi bu satırların tek erişim yolu.
            BiçimUygulaması::Tarih(_) | BiçimUygulaması::BölümlüTarih => true,
            // Süre biçimi kendi türünü kurar; tarih biçimleriyle aynı yol.
            BiçimUygulaması::Süre(..) => true,
            BiçimUygulaması::Metin
            | BiçimUygulaması::Desen(_)
            | BiçimUygulaması::Gizli
            | BiçimUygulaması::ÖzelDesen => !self.sayısal_mı(),
            BiçimUygulaması::Eksik(_) => false,
        }
    }

    /// Biçim listesinden bir satır seçer ve karşılığını tercihlere indirir.
    pub fn biçim_seçeneğini_uygula(&mut self, sıra: usize) {
        let Some(seçenek) = BİÇİM_SEÇENEKLERİ.get(sıra) else {
            return;
        };
        if !self.seçenek_uygun_mu(seçenek) {
            return;
        }
        self.biçim_seçeneği = sıra;
        match seçenek.uygulama {
            BiçimUygulaması::Genel => {
                self.maske = TezgahMaskesi::Yok;
            }
            // `ORT-008` süre gösterimi kendi değer türünü ister; tarih
            // biçimlerinin türü kurması ile aynı kural.
            BiçimUygulaması::Süre(..) => {
                self.değer_türü = TezgahDeğerKipi::Süre;
                self.maske = TezgahMaskesi::Yok;
            }
            BiçimUygulaması::Sayı { gruplama } => {
                self.maske = TezgahMaskesi::Yok;
                self.binler_ayracı = gruplama;
            }
            BiçimUygulaması::Para | BiçimUygulaması::Yüzde => {
                self.maske = TezgahMaskesi::Yok;
            }
            BiçimUygulaması::Bilimsel | BiçimUygulaması::Kesir => {
                self.maske = TezgahMaskesi::Yok;
            }
            BiçimUygulaması::Metin => {
                self.maske = TezgahMaskesi::Yok;
            }
            BiçimUygulaması::Tarih(kip) => {
                self.değer_türü = kip.değer_türü();
                self.maske = TezgahMaskesi::Yok;
            }
            BiçimUygulaması::Desen(desen) => {
                self.maske = TezgahMaskesi::Desen;
                self.desen = desen.to_owned();
            }
            // `§9.5` bölüm gezinimi maskenin alanıdır; seçenek hem türü hem
            // maskeyi kurar ki gezinim gerçekten denenebilsin.
            BiçimUygulaması::BölümlüTarih => {
                self.değer_türü = TezgahDeğerKipi::Tarih;
                self.maske = TezgahMaskesi::Tarih;
                self.bölüm_gezinimi = true;
            }
            BiçimUygulaması::Gizli => {
                self.maske = TezgahMaskesi::Yok;
                self.görünürlük = TezgahGörünürlüğü::Gizli;
            }
            BiçimUygulaması::ÖzelDesen => {
                self.maske = TezgahMaskesi::Desen;
            }
            BiçimUygulaması::Eksik(_) => {}
        }
        self.türe_uyarla();
    }

    /// Seçili satırın `ORT-008` biçim yapılandırması.
    ///
    /// Yalnız `Biçim` öbeği açık bir tanım kurar. Maske seçenekleri değeri
    /// `§9.1` deseniyle sınırlar, gösterimini değil; onlarda biçim `Genel`
    /// kalır ve tür ne ise onun genel gösterimi uygulanır.
    pub fn biçim_çöz(&self) -> BiçimYapılandırması {
        let kesir = self.ondalık_basamak.min(u8::MAX as usize) as u8;
        let sayı = |gruplama: bool, kesir: u8| SayıBiçimi {
            duyarlılık: Some(OndalıkDuyarlılık::Sabit(kesir)),
            yuvarlama: None,
            basamak_gruplama: Some(if gruplama {
                BasamakGruplama::YerelVarsayılan
            } else {
                BasamakGruplama::Yok
            }),
            işaret: None,
            sıfır: None,
            kısaltma: None,
            en_az_tamsayı_basamağı: None,
        };
        match self.seçili_biçim().uygulama {
            BiçimUygulaması::Sayı { gruplama } => match self.değer_türü {
                TezgahDeğerKipi::Tamsayı => {
                    BiçimYapılandırması::Açık(BiçimTanımı::Tamsayı(sayı(gruplama, 0)))
                }
                _ => BiçimYapılandırması::Açık(BiçimTanımı::Ondalık(sayı(gruplama, kesir))),
            },
            BiçimUygulaması::Para => match gpui_bilesenleri::ParaBirimi::yeni("TRY") {
                Ok(birim) => BiçimYapılandırması::Açık(BiçimTanımı::Para(ParaBiçimi {
                    sayı: sayı(true, kesir),
                    birim,
                    birim_gösterimi: Some(ParaBirimiGösterimi::Simge),
                    birim_konumu: self.işaret_konumu,
                })),
                Err(_) => BiçimYapılandırması::Genel,
            },
            BiçimUygulaması::Yüzde => {
                BiçimYapılandırması::Açık(BiçimTanımı::Yüzde(YüzdeBiçimi {
                    sayı: sayı(false, kesir),
                    // Model değeri zaten yüzde birimindedir; ölçeklenmez.
                    model_ölçeği: ondalık(1, 0),
                    işaret_konumu: self.işaret_konumu,
                }))
            }
            BiçimUygulaması::Metin => BiçimYapılandırması::Açık(BiçimTanımı::Metin),
            BiçimUygulaması::Tarih(kip) => BiçimYapılandırması::Açık(kip.biçim_tanımı()),
            BiçimUygulaması::Süre(en_küçük, en_büyük) => {
                BiçimYapılandırması::Açık(BiçimTanımı::Süre(SüreBiçimi {
                    en_küçük_birim: en_küçük,
                    en_büyük_birim: en_büyük,
                }))
            }
            BiçimUygulaması::Bilimsel => {
                BiçimYapılandırması::Açık(BiçimTanımı::Bilimsel(BilimselBiçim {
                    mantis: sayı(false, kesir.max(2)),
                    en_az_üs_basamağı: 2,
                }))
            }
            BiçimUygulaması::Kesir => {
                BiçimYapılandırması::Açık(BiçimTanımı::Kesir(KesirBiçimi {
                    // Tasarımın örneği `1/4`; tek basamaklı payda o aileyi verir.
                    payda: KesirPaydası::EnFazlaBasamak(1),
                    tam_kısım_ayrı: true,
                }))
            }
            // `§6` para ve yüzde tür değildir: kip karşılığını biçim
            // profilinde bulur. Genel'e düşmek kipi sessizce düz ondalığa
            // çevirirdi; açık seçim yoksa kipin varsayılan profili kurulur.
            _ => match self.değer_türü {
                TezgahDeğerKipi::ParaBirimi => match gpui_bilesenleri::ParaBirimi::yeni("TRY") {
                    Ok(birim) => BiçimYapılandırması::Açık(BiçimTanımı::Para(ParaBiçimi {
                        sayı: sayı(true, kesir),
                        birim,
                        birim_gösterimi: Some(ParaBirimiGösterimi::Simge),
                        birim_konumu: self.işaret_konumu,
                    })),
                    Err(_) => BiçimYapılandırması::Genel,
                },
                TezgahDeğerKipi::Yüzde => {
                    BiçimYapılandırması::Açık(BiçimTanımı::Yüzde(YüzdeBiçimi {
                        sayı: sayı(false, kesir),
                        model_ölçeği: ondalık(1, 0),
                        işaret_konumu: self.işaret_konumu,
                    }))
                }
                _ => BiçimYapılandırması::Genel,
            },
        }
    }

    /// Seçili biçim satırı.
    pub fn seçili_biçim(&self) -> &'static BiçimSeçeneği {
        BİÇİM_SEÇENEKLERİ
            .get(self.biçim_seçeneği)
            .unwrap_or(&BİÇİM_SEÇENEKLERİ[0])
    }

    /// `§23` bölüt kuşağının kod panelindeki karşılığı.
    fn bitişik_bölüt_kodu(&self) -> Option<String> {
        let yaz = |bölüt: Option<TezgahBölütü>| match bölüt {
            Some(bölüt) => format!(
                "Some(BitişikBölüt {{\n        tür: BitişikBölütTürü::{},\n        \
                 kendi_sınırı: {},\n        opaklık_kademeli: {},\n    }})",
                match bölüt {
                    TezgahBölütü::SabitMetin => "Sabit",
                    TezgahBölütü::Eylem => "Eylem",
                },
                self.bölüt_sınırı,
                self.bölüt_kademeli
            ),
            None => "None".to_owned(),
        };
        (self.başlangıç_bölütü.is_some() || self.bitiş_bölütü.is_some()).then(|| {
            format!(
                "yapılandırma.bitişik_bölütler = Some(BitişikBölütKuşağı {{\n    \
                 başlangıç: {},\n    bitiş: {},\n}});",
                yaz(self.başlangıç_bölütü),
                yaz(self.bitiş_bölütü)
            )
        })
    }

    /// `§16.2.4` ankraj düğmesine basmanın karşılığı.
    ///
    /// Seçili ankraja yeniden basmak yapılandırmayı `None`'a indirir: gösterge
    /// kapalılığı ayrı bir yerleşim kademesi değil, `durum_göstergesi`
    /// alanının yokluğudur. Bu yüzden ekranda üçüncü bir "Kapalı" düğmesi
    /// yoktur — olsaydı kapalılık ankrajla eşdeğer bir seçenek gibi
    /// görünürdü.
    pub fn gösterge_ankrajına_bas(&mut self, değer: DurumGöstergesiYerleşimTercihi) {
        self.gösterge_ankrajı = if self.gösterge_ankrajı == Some(değer) {
            None
        } else {
            Some(değer)
        };
    }

    /// Seçili biçimin kod panelindeki gövdesi.
    ///
    /// `biçim_çöz` gerçek `BiçimTanımı`nı kurar; burada onun **okunabilir**
    /// karşılığı yazılır. `SayıBiçimi`nin sekiz alanını tam yazmak paneli
    /// biçim eksenine boğardı; kod panelinin işi seçimi göstermek, kanonik
    /// yapıyı yeniden basmak değil.
    fn biçim_kod_gövdesi(&self) -> String {
        let kesir = self.ondalık_basamak;
        let gruplama = |var: bool| if var { "YerelVarsayılan" } else { "Yok" };
        match self.seçili_biçim().uygulama {
            BiçimUygulaması::Genel => "/* Genel */".to_owned(),
            BiçimUygulaması::Süre(en_küçük, en_büyük) => format!(
                "BiçimTanımı::Süre(SüreBiçimi {{\n        \
                 en_küçük_birim: SüreBirimi::{en_küçük:?},\n        \
                 en_büyük_birim: SüreBirimi::{en_büyük:?},\n    }})"
            ),
            BiçimUygulaması::Sayı { gruplama: grup } => {
                if self.değer_türü == TezgahDeğerKipi::Tamsayı {
                    format!(
                        "BiçimTanımı::Tamsayı(SayıBiçimi {{\n        \
                         duyarlılık: Some(OndalıkDuyarlılık::Sabit(0)),\n        \
                         basamak_gruplama: Some(BasamakGruplama::{}),\n        \
                         ..SayıBiçimi::default()\n    }})",
                        gruplama(grup)
                    )
                } else {
                    format!(
                        "BiçimTanımı::Ondalık(SayıBiçimi {{\n        \
                         duyarlılık: Some(OndalıkDuyarlılık::Sabit({kesir})),\n        \
                         basamak_gruplama: Some(BasamakGruplama::{}),\n        \
                         ..SayıBiçimi::default()\n    }})",
                        gruplama(grup)
                    )
                }
            }
            BiçimUygulaması::Para => format!(
                "BiçimTanımı::Para(ParaBiçimi {{\n        \
                 sayı: SayıBiçimi {{ duyarlılık: Some(OndalıkDuyarlılık::Sabit({kesir})), \
                 ..SayıBiçimi::default() }},\n        \
                 birim: ParaBirimi::yeni(\"TRY\").expect(\"geçerli birim\"),\n        \
                 birim_gösterimi: Some(ParaBirimiGösterimi::Simge),\n    }})"
            ),
            BiçimUygulaması::Yüzde => format!(
                "BiçimTanımı::Yüzde(YüzdeBiçimi {{\n        \
                 sayı: SayıBiçimi {{ duyarlılık: Some(OndalıkDuyarlılık::Sabit({kesir})), \
                 ..SayıBiçimi::default() }},\n        \
                 // Model değeri zaten yüzde birimindedir; ölçeklenmez.\n        \
                 model_ölçeği: OndalıkSayı::yeni(1, 0),\n    }})"
            ),
            BiçimUygulaması::Metin => "BiçimTanımı::Metin".to_owned(),
            BiçimUygulaması::Tarih(kip) => {
                format!("/* `§8.1` tarih/saat gösterimi */ TarihKipi::{kip:?}.biçim_tanımı()")
            }
            BiçimUygulaması::Bilimsel => format!(
                "BiçimTanımı::Bilimsel(BilimselBiçim {{\n        \
                 mantis: SayıBiçimi {{ duyarlılık: Some(OndalıkDuyarlılık::Sabit({})), \
                 ..SayıBiçimi::default() }},\n        \
                 en_az_üs_basamağı: 2,\n    }})",
                kesir.max(2)
            ),
            BiçimUygulaması::Kesir => "BiçimTanımı::Kesir(KesirBiçimi::default())".to_owned(),
            BiçimUygulaması::BölümlüTarih => {
                "/* `§9.5` bölümlü tarih; biçim değil maske kurar */".to_owned()
            }
            BiçimUygulaması::Gizli
            | BiçimUygulaması::Desen(_)
            | BiçimUygulaması::ÖzelDesen
            | BiçimUygulaması::Eksik(_) => {
                // Bu satırlar biçim değil maske/görünürlük kurar ya da
                // kanonik karşılığı yoktur; biçim ekseni `Genel` kalır ve
                // `biçim_çöz` buraya `Açık` döndürmez.
                "/* biçim kurulmaz */".to_owned()
            }
        }
    }

    /// Değer türü değişince türe uymayan tercihleri kapatır.
    ///
    /// Gizlenen bir tercihin arka planda uygulanmaya devam etmesi programcıyı
    /// yanıltır: galeride görünmeyen ama kodda etkili bir alan kalmamalı.
    pub fn türe_uyarla(&mut self) {
        // Seçili biçim yeni türde kurulamıyorsa `Genel`e döner: uymayan bir
        // biçimi sessizce uygulamaya devam etmek yanıltıcı olur. Tarih
        // biçimleri türü kendileri kurduğu için ayrı ölçülür: tür satırından
        // başka bir tür seçilmişse tarih biçimi de düşer.
        let biçim = self.seçili_biçim();
        // Bazı biçimler kendi değer türünü kurar; tür satırından başka bir
        // tür seçilince o biçim de düşmeli. Kontrol tek tek eklendiği için
        // iki kez eksik kaldı: önce süre, sonra bölümlü tarih. Artık üçü
        // aynı yerden ölçülüyor — dördüncüsü eklenirse buraya girer.
        let kurduğu_tür = match biçim.uygulama {
            BiçimUygulaması::Tarih(kip) => Some(kip.değer_türü()),
            BiçimUygulaması::BölümlüTarih => Some(TezgahDeğerKipi::Tarih),
            BiçimUygulaması::Süre(..) => Some(TezgahDeğerKipi::Süre),
            _ => None,
        };
        let tür_uyumsuz = kurduğu_tür.is_some_and(|tür| tür != self.değer_türü);
        if tür_uyumsuz || !self.seçenek_uygun_mu(biçim) {
            self.biçim_seçeneği = 0;
        }
        if !self.maske_seçenekleri().contains(&self.maske) {
            self.maske = TezgahMaskesi::Yok;
        }
        if self.sayısal_mı() {
            // Metni ilgilendiren tercihler sayısal alanda kurulamaz.
            self.görünürlük = TezgahGörünürlüğü::Açık;
            self.parola_düğmesi = false;
            self.uzunluk_sınırı = false;
            self.sayaç = false;
        } else {
            // `§9.6` adım yalnız sayısal alanda iş yapar.
            self.sayısal_adım = false;
        }
        self.adım_tutarlılığını_kur();
        // `§9.3` binler ayracı gösterimin işidir; sayısal olmayan türde
        // karşılığı yoktur.
        if !self.sayısal_mı() {
            self.binler_ayracı = false;
        }
        self.içerik_türü_tutarlılığını_kur();
        self.seçici_ve_odak_tutarlılığını_kur();
    }

    /// `§24`/`§17` seçici yuvasını ve odak politikasını türe uyarlar.
    fn seçici_ve_odak_tutarlılığını_kur(&mut self) {
        // `§24` seçici yalnız takvim/saat üreten türlerde ve serbest
        // metinde anlamlıdır; sayısal ve süre alanında açacak bir yüzey yok.
        if !matches!(
            self.değer_türü,
            TezgahDeğerKipi::Tarih
                | TezgahDeğerKipi::Saat
                | TezgahDeğerKipi::TarihSaat
                | TezgahDeğerKipi::Metin
        ) {
            self.seçici = false;
        }
        // `§17` odağı korumak odaklanabilir bir alan ister. Devre dışı
        // alanda `OdağıKoru` kanonik doğrulamada hata; tezgâh iki tercihi
        // bağımsız sunduğu için bileşim kurulabiliyordu.
        if !self.etkin && self.geçersiz_odak == GeçersizOdakDavranışı::OdağıKoru {
            self.geçersiz_odak = GeçersizOdakDavranışı::OdakKaybınaİzinVer;
        }
    }

    /// `§7` içerik türü yalnız `Metin` ailesinde anlamlıdır.
    fn içerik_türü_tutarlılığını_kur(&mut self) {
        if crate::tür_ailesi(self.değer_türü) != crate::TezgahAilesi::Metin {
            self.metin_içerik_türü = MetinİçerikTürü::Düz;
        }
    }

    /// `§7` içerik türünü seçer ve maske ile ekleri ona uyarlar.
    ///
    /// İçerik türü alanın **ne beklediğini** söyler; maske ile ön ek de onu
    /// söyler. Üçü birbirinden bağımsız kalsaydı ekranda "Düz metin"
    /// yazarken kutu telefon numarası bekleyebilirdi — nitekim öyle
    /// oluyordu.
    ///
    /// Uyarlama yalnız **bu seçim** anında yapılır: kullanıcı sonra maskeyi
    /// ya da eki tek tek değiştirebilir ve seçimi geri alınmaz.
    pub fn içerik_türünü_seç(&mut self, tür: MetinİçerikTürü) {
        self.metin_içerik_türü = tür;
        match tür {
            // Düz metin serbest girdidir: desen maskesi ve ülke kodu eki
            // beklentiyi daraltır.
            MetinİçerikTürü::Düz => {
                if self.maske == TezgahMaskesi::Desen {
                    self.maske = TezgahMaskesi::Yok;
                    self.biçim_seçeneği = 0;
                }
                if OTOMATİK_ÖN_EKLER.contains(&self.ön_ek_metni.as_str()) {
                    self.ön_ek = false;
                    self.ön_ek_metni = VARSAYILAN_ÖN_EK.to_owned();
                }
            }
            MetinİçerikTürü::Telefon => {
                self.maske = TezgahMaskesi::Desen;
                self.desen = crate::HAZIR_DESENLER[0].1.to_owned();
                self.biçim_seçeneği = desen_seçeneği(&self.desen);
                self.ön_ek = true;
                if OTOMATİK_ÖN_EKLER.contains(&self.ön_ek_metni.as_str()) {
                    self.ön_ek_metni = TELEFON_ÖN_EKİ.to_owned();
                }
            }
            // E-posta ve URL desen maskesiyle yazılamaz: uzunluk ve şekil
            // serbesttir, kural doğrulamanın işidir.
            MetinİçerikTürü::EPosta | MetinİçerikTürü::Url => {
                if self.maske == TezgahMaskesi::Desen {
                    self.maske = TezgahMaskesi::Yok;
                    self.biçim_seçeneği = 0;
                }
                if OTOMATİK_ÖN_EKLER.contains(&self.ön_ek_metni.as_str()) {
                    // URL şeması kutunun içinde durur; e-postada karşılığı yok.
                    self.ön_ek = tür == MetinİçerikTürü::Url;
                    self.ön_ek_metni = if self.ön_ek {
                        URL_ÖN_EKİ.to_owned()
                    } else {
                        VARSAYILAN_ÖN_EK.to_owned()
                    };
                }
            }
        }
    }

    /// `§29` adım tercihlerini kendi içinde tutarlı tutar.
    ///
    /// Çelişkili bir adım alanı geçersiz yüzeye düşürür. Tezgâh geçersiz
    /// yapılandırmayı sergilemek için değil, çalışan davranışı göstermek
    /// için var; bu yüzden çelişki tercih düzeyinde çözülür.
    fn adım_tutarlılığını_kur(&mut self) {
        // `§14` varsayılan tarih/saat/süre türünde uygulanmıyor.
        if !self.varsayılan_uygulanabilir_mi() {
            self.varsayılan_değer = false;
            self.sıfırlama = gpui_bilesenleri::SıfırlamaDavranışı::BoşaDön;
        }
        // `§9.5` bölüm gezinimi yalnız bölümlü maskede vardır.
        if !self.bölüm_gezinimi_anlamlı_mı() {
            self.bölüm_gezinimi = false;
        }
        // Tamsayı alanda kesirli adım kurulamaz.
        if self.değer_türü == TezgahDeğerKipi::Tamsayı && self.adım_ölçeği.kesirli_mi() {
            self.adım_ölçeği = AdımÖlçeği::Birim;
        }
        // Sarma sonlu alt ve üst sınır çiftini ister.
        if !self.adım_sınırı {
            self.adım_sarma = false;
        }
    }

    /// Önizlemenin açılışta taşıdığı değer.
    ///
    /// **Boş.** Önceden türe uygun bir örnek yazıyordu; gerekçe hizalama,
    /// binler ayracı ve temizleme simgesinin ancak içerik varken
    /// görünmesiydi. Ama o örnek `§19` Escape davranışını da gizliyordu:
    /// kutu dolu açılınca "boşa dön" ile "son kabul edilene dön" aynı
    /// görünüyordu. Boş açılışta ilk Escape kutuyu temizler, bir değer
    /// kabul edildikten sonraki Escape o değere döner — iki dal ayrı
    /// ayrı denenebilir.
    ///
    /// İçerik gerektiren eksenler yazınca zaten görünür hâle gelir.
    pub fn örnek_değer(&self) -> &'static str {
        ""
    }

    /// Tercihleri kanonik `GirişYapılandırması`na çevirir.
    ///
    /// Galeri hiçbir varsayılan icat etmez: taban `tek_satırlı_metin()`
    /// yapılandırmasıdır, tercihler yalnız onun alanlarını değiştirir.
    /// Tek seferlik tüketiciler için yapılandırmayı host kimlik fabrikasıyla kurar.
    ///
    /// Yaşayan galeri, yeniden yapılandırmalarda kimlikleri korumak için
    /// [`Self::yapılandırma_kimliklerle`] yolunu kullanır.
    /// Metin kutusunun **kendi** teması.
    ///
    /// Parça ailesi seçiliyse kabuk ailesini ezer; `Rolden devral` iken kutu
    /// kabuğun ailesini kullanır. İki eksen ayrı: üst şerit bütün tezgâhı,
    /// bu liste yalnız önizleme kutusunu değiştirir.
    pub fn kutu_teması(&self) -> TezgahTeması {
        let mut tema = self.tema.clone();
        if let Some(aile) = &self.parça_ailesi {
            tema.yazı_ailesi = aile.clone();
        }
        tema
    }

    /// `motor`, `ORT-002` kökünden verilen doğrulama kapısıdır: takvim ve
    /// dil etiketi kimlikleri yalnız oradan doğar, tezgâh mühürlemez.
    pub fn yapılandırma(
        &self,
        fabrika: &gpui_bilesenleri::ÖrnekKimliğiFabrikası,
        motor: &gpui_bilesenleri_temel::UnicodeMetinMotoru,
    ) -> GirişYapılandırması {
        let yardımcı_kimlikleri = crate::YardımcıKimlikleri::yeni(fabrika);
        self.yapılandırma_kimliklerle(&yardımcı_kimlikleri, motor)
    }

    pub(crate) fn yapılandırma_kimliklerle(
        &self,
        yardımcı_kimlikleri: &crate::YardımcıKimlikleri,
        motor: &gpui_bilesenleri_temel::UnicodeMetinMotoru,
    ) -> GirişYapılandırması {
        let mut y = GirişYapılandırması::tek_satırlı_metin();
        // `§6` kip kanonik aileye iner: para/yüzde `Ondalık` türdür ve
        // karşılığını biçim profili verir; Metin ailesi içerik türünü kendi
        // tanımında taşır — "koda yazılamaz" rozeti bu satırla kapandı.
        y.giriş_türü = self.değer_türü.kanonik_tür(self.metin_içerik_türü);
        y.maske = self.maske_çöz(motor);
        y.biçim = self.biçim_çöz();
        y.hizalama.yatay = self.hizalama;
        y.hizalama.dikey = self.dikey;
        y.şekil = match (self.şekil_oto, self.köşe_pikseli) {
            (true, _) => KutuŞekliTercihi::GörünümProfilinden,
            (false, Some(piksel)) => KutuŞekliTercihi::Yarıçap(gpui::px(piksel)),
            (false, None) => KutuŞekliTercihi::Açık(self.şekil),
        };
        y.odak.sekme_durağı = self.sekme_durağı;
        y.odak_seçimi = self.odak_seçimi;
        y.kabul_seçimi = self.kabul_seçimi;
        y.dış_tıklamada_odağı_bırak = self.dış_tıklamada_odağı_bırak;
        y.üzerine_yazma = self.üzerine_yazma;
        y.enter = self.enter;
        y.salt_okunur = self.salt_okunur;
        y.etkin = self.etkin;

        if self.ön_ek && !self.ön_ek_metni.is_empty() {
            let mut ek = Sabitİçerik::metin(self.ön_ek_metni.as_str(), false);
            ek.sunum_rolü = self.ek_sunum_rolü;
            y.ön_ek = Some(ek);
        }
        if self.son_ek && !self.son_ek_metni.is_empty() {
            let mut ek = Sabitİçerik::metin(self.son_ek_metni.as_str(), false);
            ek.sunum_rolü = self.ek_sunum_rolü;
            y.son_ek = Some(ek);
        }
        // `§25` amaç açıkken zorunludur; kapalıyken amaç bildirilmez.
        y.otomatik_doldurma = gpui_bilesenleri::OtomatikDoldurmaTercihleri {
            etkin: self.otomatik_doldurma,
            amaç: self.otomatik_doldurma.then_some(self.doldurma_amacı),
        };
        y.varsayılan_değer = self.varsayılan_değer_çöz();
        y.sıfırlama = self.sıfırlama;
        // `ORT-009` alan erişilebilir ağaca adıyla girer.
        y.erişilebilir_ad = self
            .erişilebilir_ad
            .then(|| crate::hazır_ileti("Yapılandırma tezgâhı alanı"));
        if self.yer_tutucu {
            y.yer_tutucu = Some(crate::hazır_ileti("Değer girin…"));
        }
        y.içerik_görünürlüğü = self.görünürlük.kanonik();
        // `§22` `GeçiciGöster` politikasız kurulamaz; politika da yalnız o
        // görünürlükte yazılır, başka görünürlükte okunmayan bir tercih
        // tarif ederdi.
        y.geçici_gösterim = (self.görünürlük == TezgahGörünürlüğü::GeçiciGöster)
            .then(|| self.geçici_gösterim.kanonik());
        // `§23` kuşak yalnız en az bir bölüt seçiliyken kurulur; boş bir
        // kuşak alan kabuğunun dışında görünmez bir kutu bırakırdı.
        y.bitişik_bölütler =
            (self.başlangıç_bölütü.is_some() || self.bitiş_bölütü.is_some()).then(|| {
                BitişikBölütKuşağı {
                    başlangıç: self
                        .başlangıç_bölütü
                        .map(|bölüt| bölüt.kanonik(self.bölüt_kademeli, self.bölüt_sınırı)),
                    bitiş: self
                        .bitiş_bölütü
                        .map(|bölüt| bölüt.kanonik(self.bölüt_kademeli, self.bölüt_sınırı)),
                }
            });
        // `§23.3` `AramayıBaşlat` yuvası yokken `arama_gönderimi = Some`
        // olması `UyumsuzAramaGönderimi`dir: alanı arama alanı yapan şey
        // yuvadır, gönderim yapılandırması değil.
        y.arama_gönderimi = self.arama.then_some(AramaGönderimYapılandırması {
            enter_gönderir: self.arama_enter_gönderir,
            temizleme_gönderir: self.arama_temizleme_gönderir,
            çalışırken_enter: self.çalışırken_enter,
        });
        // `§24` uyarlama yalnız seçici yuvası açıkken kurulur; yuvasız bir
        // görünürlük politikası ulaşılamayan bir hattı tarif ederdi.
        y.seçici = self.seçici.then(|| SeçiciUyarlaması {
            görünürlük: self.seçici_görünürlüğü,
            // Açılma tetikleyicileri ve yüzey `ORT-006` alanıdır; tezgâh
            // yüzey geometrisi kurmaz, yalnız görünürlük politikasını açar.
            açılma_tetikleyicileri: Vec::new(),
            yüzey: AçılırYüzeyYapılandırması::default(),
        });
        // `§16` dış hatanın temizlenme politikası.
        y.doğrulama.dış_hata_temizleme = self.dış_hata_temizleme;
        // `§15` zorunluluk kuralı. Kimlik `2`: `1` sayısal adım sınırının.
        if self.zorunlu {
            y.doğrulama.kurallar.push(GeçerlilikKuralı {
                kimlik: GeçerlilikKuralıKimliği(2),
                tetikleyici: self.doğrulama_tetikleyicisi,
                önem: self.doğrulama_önemi,
                kural: GeçerlilikKuralTürü::Zorunlu,
                ileti: Some("Bu alan zorunludur".into()),
            });
        }
        // `§29` birden çok kural varken ilk hatada durulur mu?
        y.doğrulama.ilk_hatada_dur = self.ilk_hatada_dur;
        y.harf_dönüşümü = self.harf_dönüşümü;
        y.kırpma = self.kırpma;
        y.boş_metin = self.boş_metin;
        y.yapıştırma = self.yapıştırma.kanonik(motor);
        y.escape = self.escape;
        y.geçersiz_odak = self.geçersiz_odak;
        // `§16.2` ankraj yoksa gösterge yapılandırmayla kapalıdır; `None`
        // ayrı bir "kapalı" kademesi değil, alanın kendisinin yokluğudur.
        y.durum_göstergesi =
            self.gösterge_ankrajı
                .map(|yerleşim| DurumGöstergesiYapılandırması {
                    yerleşim,
                    açıklama: self.gösterge_açıklaması,
                });
        if self.uzunluk_sınırı {
            y.uzunluk_sınırı = Some(UzunlukSınırı {
                en_fazla_grafem: 12,
                davranış: self.uzunluk_davranışı,
            });
        }
        if self.sayısal_adım {
            let (küçük, büyük) = self.adım_ölçeği.çift();
            y.sayısal_adım = Some(gpui_bilesenleri::SayısalAdım {
                küçük,
                büyük,
                kata_hizala: self.adım_hizala,
                hizalama_tabanı: gpui_bilesenleri::AdımHizalamaTabanı::Sıfır,
                sarma: self.adım_sarma,
            });
            // `§9.6` sınır adımın kendi alanı değil; aynı `§15` kuralından
            // gelir. İki yerde ayrı bildirilseydi çelişebilirlerdi.
            if self.adım_sınırı {
                y.doğrulama
                    .kurallar
                    .push(gpui_bilesenleri::GeçerlilikKuralı {
                        kimlik: gpui_bilesenleri::GeçerlilikKuralıKimliği(1),
                        tetikleyici: gpui_bilesenleri::GeçerlilikTetikleyicisi::Kabulde,
                        önem: gpui_bilesenleri::GeçerlilikÖnemi::Hata,
                        kural: gpui_bilesenleri::GeçerlilikKuralTürü::SayısalAralık {
                            en_az: Some(ondalık(0, 0)),
                            en_fazla: Some(ondalık(100, 0)),
                        },
                        ileti: Some("Değer 0 ile 100 arasında olmalı".into()),
                    });
            }
        }
        if self.sayaç {
            y.sayaç = Some(SayaçYapılandırması {
                birim: self.sayaç_birimi,
                sınırı_göster: self.sayaç_sınırı_göster,
            });
        }

        let yuvalar: Vec<_> = self
            .yardımcı_eylem_türleri()
            .into_iter()
            // `ORT-009` adsız düğme erişilebilir ağaca girmez.
            .map(|tür| {
                let ad = match tür {
                    YardımcıEylemTürü::Temizle => "Alanı temizle",
                    YardımcıEylemTürü::ParolayıGöster => "Parolayı göster",
                    YardımcıEylemTürü::AramayıBaşlat => "Aramayı başlat",
                    YardımcıEylemTürü::SeçiciyiAç => "Seçiciyi aç",
                    YardımcıEylemTürü::Ürün(_) => "Ürün eylemi",
                };
                let mut yuva =
                    YardımcıEylemYuvası::kademeli(yardımcı_kimlikleri.al(&tür), tür.clone());
                if self.yuva_adları {
                    yuva = yuva.adla(crate::hazır_ileti(ad));
                }
                yuva.görünürlük = self.yuva_görünürlüğü;
                yuva.etkin = self.yuvalar_etkin;
                // `§23` gönderim bağı yalnız arama yuvasında anlamlı:
                // temizleme ya da parola yuvası gönderim üretmez.
                if tür == YardımcıEylemTürü::AramayıBaşlat && self.arama_gönderime_bağlı {
                    yuva.çalışma = YardımcıEylemÇalışması::AlanınGönderimineBağlı;
                }
                yuva
            })
            .collect();
        if !yuvalar.is_empty() {
            y.yardımcı_eylemler = Some(std::sync::Arc::from(yuvalar.as_slice()));
        }
        y
    }

    /// `§23` seçili yardımcı eylemler, en fazla üç yuva.
    /// `§23` şu an kabukta bulunan yardımcı eylem yuvası sayısı.
    pub fn açık_yuva_sayısı(&self) -> usize {
        [
            self.temizle,
            self.parola_düğmesi && self.görünürlük.parola_yuvası_var(),
            self.arama,
            self.seçici,
            self.ürün_eylemi,
        ]
        .into_iter()
        .filter(|açık| *açık)
        .count()
    }

    /// `§23` kapalı bir yuva daha açılabilir mi?
    ///
    /// Sınır dolduğunda dördüncü yuva sessizce kırpılıyordu: kullanıcı
    /// düğmeye basıyor, hiçbir şey olmuyor ve nedenini göremiyordu.
    pub fn yuva_eklenebilir_mi(&self) -> bool {
        self.açık_yuva_sayısı() < 3
    }

    fn yardımcı_eylem_türleri(&self) -> Vec<YardımcıEylemTürü> {
        [
            self.temizle.then_some(YardımcıEylemTürü::Temizle),
            self.parola_düğmesi
                .then_some(YardımcıEylemTürü::ParolayıGöster),
            self.arama.then_some(YardımcıEylemTürü::AramayıBaşlat),
            self.seçici.then_some(YardımcıEylemTürü::SeçiciyiAç),
            self.ürün_eylemi
                .then(|| YardımcıEylemTürü::Ürün(crate::ürün_eylem_kimliği())),
        ]
        .into_iter()
        .flatten()
        .take(3)
        .collect()
    }

    fn maske_çöz(
        &self,
        motor: &gpui_bilesenleri_temel::UnicodeMetinMotoru,
    ) -> Option<GirişMaskesi> {
        match self.maske {
            TezgahMaskesi::Yok => None,
            TezgahMaskesi::Desen => crate::deseni_maskeye_çevir(&self.desen),
            TezgahMaskesi::Tarih => {
                Some(GirişMaskesi::Tarih(gpui_bilesenleri::TarihGirişMaskesi {
                    desen: "gg.aa.yyyy".into(),
                    // Takvim kimliği elle mühürlenmez; `ORT-002` kayıt
                    // yolundan çözülür.
                    takvim: motor
                        .takvim("gregory")
                        .expect("`gregory` yerleşik takvim kayıtlarında bulunur"),
                    eksik_giriş: None,
                    rakam_kümesi: Some(RakamKümesi::Latin),
                    bölüm_gezinimi: self.bölüm_gezinimi_çöz(),
                }))
            }
        }
    }

    /// `§9.5` tezgâhtaki bölüm gezinimi tercihleri.
    fn bölüm_gezinimi_çöz(&self) -> Option<gpui_bilesenleri::BölümGezinimi> {
        self.bölüm_gezinimi
            .then_some(gpui_bilesenleri::BölümGezinimi {
                yön_tuşuyla_atla: self.bölüm_atla,
                dolunca_ilerle: self.bölüm_dolunca_ilerle,
                yön_tuşuyla_artır: self.bölüm_artır,
                artırma_taşar: self.bölüm_taşar,
                ayraç_yazımı_ilerletir: self.bölüm_ayraç,
            })
    }

    /// `§14` tezgâhtaki varsayılan değer, alanın türüne göre.
    ///
    /// Tarih/saat/süre burada yok: `§14` o türlerde varsayılanı
    /// uygulamıyor, uygulanmayan bir tercih gösterilmez.
    fn varsayılan_değer_çöz(&self) -> gpui_bilesenleri::VarsayılanDeğer {
        use gpui_bilesenleri::Değer;
        if !self.varsayılan_değer {
            return gpui_bilesenleri::VarsayılanDeğer::Yok;
        }
        let değer = match self.değer_türü {
            TezgahDeğerKipi::Tamsayı => Değer::Tamsayı(42_i128.into()),
            TezgahDeğerKipi::Ondalık | TezgahDeğerKipi::Yüzde => {
                Değer::Ondalık(ondalık(1250, 2))
            }
            TezgahDeğerKipi::ParaBirimi => match gpui_bilesenleri::ParaBirimi::yeni("TRY") {
                Ok(birim) => Değer::Para {
                    tutar: ondalık(9990, 2),
                    birim,
                },
                Err(_) => return gpui_bilesenleri::VarsayılanDeğer::Yok,
            },
            TezgahDeğerKipi::Metin => Değer::Metin(gpui_bilesenleri::GüvenliMetin::yeni(
                "Varsayılan".to_owned(),
                false,
                true,
            )),
            _ => return gpui_bilesenleri::VarsayılanDeğer::Yok,
        };
        gpui_bilesenleri::VarsayılanDeğer::Sabit(değer)
    }

    /// `§14` varsayılan bu türde uygulanabilir mi?
    pub const fn varsayılan_uygulanabilir_mi(&self) -> bool {
        matches!(
            self.değer_türü,
            TezgahDeğerKipi::Metin
                | TezgahDeğerKipi::Tamsayı
                | TezgahDeğerKipi::Ondalık
                | TezgahDeğerKipi::Yüzde
                | TezgahDeğerKipi::ParaBirimi
        )
    }

    /// `§9.5` bölüm gezinimi bu maskede anlamlı mı?
    pub const fn bölüm_gezinimi_anlamlı_mı(&self) -> bool {
        matches!(self.maske, TezgahMaskesi::Tarih)
    }

    /// Tercihlerin karşılığı olan Rust kodu.
    ///
    /// Programcı galeride gördüğü alanı kendi kodunda birebir kurabilsin diye
    /// Kod panelindeki `giriş_türü` kuruluş metni.
    fn giriş_türü_kodu(&self) -> String {
        let tarih_zaman = |kip: &str| {
            format!(
                "GirişTürü::TarihZaman(TarihZamanTanımı {{\n        kip: TarihZamanKipi::{kip},\n        \
                 motor: TarihZamanMotorTercihi::Otomatik,\n        \
                 belirsiz_zaman: DstPolitikası::Reddet,\n    }})"
            )
        };
        match self.değer_türü {
            TezgahDeğerKipi::Metin => format!(
                "GirişTürü::Metin(MetinTanımı {{ içerik_türü: MetinİçerikTürü::{:?} }})",
                self.metin_içerik_türü
            ),
            TezgahDeğerKipi::Tamsayı => "GirişTürü::Tamsayı(TamsayıTanımı::default())".to_owned(),
            TezgahDeğerKipi::Ondalık | TezgahDeğerKipi::ParaBirimi | TezgahDeğerKipi::Yüzde => {
                "GirişTürü::Ondalık(OndalıkTanımı::default())".to_owned()
            }
            TezgahDeğerKipi::Tarih => tarih_zaman("Tarih"),
            TezgahDeğerKipi::Saat => tarih_zaman("Saat"),
            TezgahDeğerKipi::TarihSaat => tarih_zaman("YerelTarihSaat"),
            TezgahDeğerKipi::Süre => tarih_zaman("Aralık"),
        }
    }

    /// yalnız varsayılandan **sapan** alanlar yazılır.
    pub fn kod(&self) -> String {
        let mut satırlar =
            vec!["let mut yapılandırma = GirişYapılandırması::tek_satırlı_metin();".to_owned()];
        let taban = Self::default();

        if self.değer_türü != taban.değer_türü || self.metin_içerik_türü != taban.metin_içerik_türü
        {
            satırlar.push(format!(
                "yapılandırma.giriş_türü =\n    {};",
                self.giriş_türü_kodu()
            ));
        }
        match self.maske {
            TezgahMaskesi::Yok => {}
            TezgahMaskesi::Desen => satırlar.push(format!(
                "yapılandırma.maske = Some(GirişMaskesi::Metin(MetinGirişMaskesi {{\n    \
                 desen: {:?}.into(),\n    yer_tutucu_grafemi: \"_\".into(),\n    \
                 sabitleri_göster: true,\n}}));",
                self.desen
            )),
            TezgahMaskesi::Tarih => satırlar.push(
                "yapılandırma.maske = Some(GirişMaskesi::Tarih(TarihGirişMaskesi {\n    \
                 desen: \"gg.aa.yyyy\".into(),\n    takvim: TakvimKimliği(\"gregory\".into()),\n    \
                 ..\n}));"
                    .to_owned(),
            ),
        }
        if let Some(gezinim) = self.bölüm_gezinimi_çöz() {
            satırlar.push(format!(
                "// `§9.5` bölüm gezinimi maskenin alanıdır.\n\
                 bölüm_gezinimi: Some(BölümGezinimi {{\n    \
                 yön_tuşuyla_atla: {},\n    dolunca_ilerle: {},\n    \
                 yön_tuşuyla_artır: {},\n    artırma_taşar: {},\n    \
                 ayraç_yazımı_ilerletir: {},\n}}),",
                gezinim.yön_tuşuyla_atla,
                gezinim.dolunca_ilerle,
                gezinim.yön_tuşuyla_artır,
                gezinim.artırma_taşar,
                gezinim.ayraç_yazımı_ilerletir
            ));
        }
        if self.varsayılan_değer {
            satırlar.push(
                "yapılandırma.varsayılan_değer = VarsayılanDeğer::Sabit(/* türe göre */);"
                    .to_owned(),
            );
        }
        if self.sıfırlama != gpui_bilesenleri::SıfırlamaDavranışı::BoşaDön {
            satırlar.push(format!(
                "yapılandırma.sıfırlama = SıfırlamaDavranışı::{:?};",
                self.sıfırlama
            ));
        }
        if self.ön_ek {
            satırlar.push(format!(
                "yapılandırma.ön_ek = Some(Sabitİçerik::metin({:?}, false));",
                self.ön_ek_metni
            ));
        }
        if self.son_ek {
            satırlar.push(format!(
                "yapılandırma.son_ek = Some(Sabitİçerik::metin({:?}, false));",
                self.son_ek_metni
            ));
        }
        // Taban `tek_satırlı_metin()` `yer_tutucu: None` kurar; ekrandaki
        // "Değer girin…" tezgâhın kendi kurduğu bir sapmadır. Panel yalnız
        // kapalı dalı yazdığı sürece kopyalanan kod yer tutucusuz bir alan
        // üretiyor ve ekrandakiyle aynı olmuyordu.
        if self.yer_tutucu {
            satırlar.push(
                "yapılandırma.yer_tutucu = Some(Kullanıcıİletisi::hazır(\"Değer girin…\"));"
                    .to_owned(),
            );
        }
        if let Some(kod) = self.görünürlük.kod() {
            satırlar.push(kod);
        }
        // `§22` politika yalnız `GeçiciGöster`le birlikte yazılır; panel
        // kopyalanınca geçerli (politikalı) bir yapılandırma üretmeli.
        if self.görünürlük == TezgahGörünürlüğü::GeçiciGöster {
            satırlar.push(self.geçici_gösterim.kod());
        }
        // `§16` dış hata temizleme; varsayılan (`YerelDüzenlemedeTemizle`)
        // yazılmaz.
        if self.dış_hata_temizleme == DışHataTemizleme::YenidenBildirimeKadarKoru {
            satırlar.push(
                "yapılandırma.doğrulama.dış_hata_temizleme =\n    \
                 DışHataTemizleme::YenidenBildirimeKadarKoru;"
                    .to_owned(),
            );
        }
        // Biçim ekseni uzun süre kod panelinde yoktu: kullanıcı biçim
        // listesinden seçim yapıyor, panel değişmiyordu. `Genel` varsayılan
        // olduğu için yalnız açık biçim yazılır.
        if let BiçimYapılandırması::Açık(_) = self.biçim_çöz() {
            satırlar.push(format!(
                "// `§8` biçim profili · seçili satır: {}\n\
                 yapılandırma.biçim = BiçimYapılandırması::Açık({});",
                self.seçili_biçim().etiket,
                self.biçim_kod_gövdesi()
            ));
        }
        // `§20.1` alan erişilebilir ağaca adıyla girer; ad kod panelinde de
        // görünmeli, yoksa kopyalanan yapılandırma adsız bir alan kurar.
        satırlar.push(
            if self.erişilebilir_ad {
                "// `ORT-009` alan erişilebilir ağaca adıyla girer.\n\
                 yapılandırma.erişilebilir_ad = Some(Kullanıcıİletisi::hazır(\"Alan adı\"));"
            } else {
                // Adsız alan `§29`'da uyarı üretir; kod bunu sessizce
                // geçerse kopyalanan yapılandırma neden uyardığını
                // söylemez.
                "// `ORT-009` ad yok: alan erişilebilir ağaca girmez (`§29` uyarısı).\n\
                 yapılandırma.erişilebilir_ad = None;"
            }
            .to_owned(),
        );
        if self.seçici {
            satırlar.push(format!(
                "yapılandırma.seçici = Some(SeçiciUyarlaması {{\n    \
                 görünürlük: SeçiciGörünürlüğü::{:?},\n    \
                 açılma_tetikleyicileri: Vec::new(),\n    \
                 yüzey: AçılırYüzeyYapılandırması::default(),\n}});",
                self.seçici_görünürlüğü
            ));
        }
        if self.ilk_hatada_dur {
            satırlar.push("yapılandırma.doğrulama.ilk_hatada_dur = true;".to_owned());
        }
        if self.zorunlu {
            satırlar.push(format!(
                "yapılandırma.doğrulama.kurallar.push(GeçerlilikKuralı {{\n    \
                 kimlik: GeçerlilikKuralıKimliği(2),\n    \
                 tetikleyici: GeçerlilikTetikleyicisi::{:?},\n    \
                 önem: GeçerlilikÖnemi::{:?},\n    \
                 kural: GeçerlilikKuralTürü::Zorunlu,\n    \
                 ileti: Some(\"Bu alan zorunludur\".into()),\n}});",
                self.doğrulama_tetikleyicisi, self.doğrulama_önemi
            ));
        }
        // `§23` bölüt kuşağı ve arama gönderimi.
        if let Some(kuşak) = &self.bitişik_bölüt_kodu() {
            satırlar.push(kuşak.clone());
        }
        if self.arama {
            satırlar.push(format!(
                "// `§23.3` gönderim yalnız `AramayıBaşlat` yuvası varken kurulur.\n\
                 yapılandırma.arama_gönderimi = Some(AramaGönderimYapılandırması {{\n    \
                 enter_gönderir: {},\n    temizleme_gönderir: {},\n    \
                 çalışırken_enter: ÇalışırkenEnterPolitikası::{:?},\n}});",
                self.arama_enter_gönderir, self.arama_temizleme_gönderir, self.çalışırken_enter
            ));
        }
        // `§6`/`§10`/`§17` düz eksenler: varsayılandan sapan yazılır.
        if self.harf_dönüşümü != taban.harf_dönüşümü {
            satırlar.push(format!(
                "yapılandırma.harf_dönüşümü = HarfDönüşümü::{:?};",
                self.harf_dönüşümü
            ));
        }
        if self.kırpma != taban.kırpma {
            satırlar.push(format!(
                "yapılandırma.kırpma = KırpmaPolitikası::{:?};",
                self.kırpma
            ));
        }
        if self.boş_metin != taban.boş_metin {
            satırlar.push(format!(
                "yapılandırma.boş_metin = BoşMetinPolitikası::{:?};",
                self.boş_metin
            ));
        }
        if let Some(kod) = self.yapıştırma.kod() {
            satırlar.push(kod);
        }
        if self.escape != taban.escape {
            satırlar.push(format!(
                "yapılandırma.escape = EscapeDavranışı::{:?};",
                self.escape
            ));
        }
        if self.geçersiz_odak != taban.geçersiz_odak {
            satırlar.push(format!(
                "yapılandırma.geçersiz_odak = GeçersizOdakDavranışı::{:?};",
                self.geçersiz_odak
            ));
        }
        // `§6`/`§10`/`§17` düz eksenler: varsayılandan sapan yazılır.
        if self.harf_dönüşümü != taban.harf_dönüşümü {
            satırlar.push(format!(
                "yapılandırma.harf_dönüşümü = HarfDönüşümü::{:?};",
                self.harf_dönüşümü
            ));
        }
        if self.kırpma != taban.kırpma {
            satırlar.push(format!(
                "yapılandırma.kırpma = KırpmaPolitikası::{:?};",
                self.kırpma
            ));
        }
        if self.boş_metin != taban.boş_metin {
            satırlar.push(format!(
                "yapılandırma.boş_metin = BoşMetinPolitikası::{:?};",
                self.boş_metin
            ));
        }
        if let Some(kod) = self.yapıştırma.kod() {
            satırlar.push(kod);
        }
        if self.escape != taban.escape {
            satırlar.push(format!(
                "yapılandırma.escape = EscapeDavranışı::{:?};",
                self.escape
            ));
        }
        if self.geçersiz_odak != taban.geçersiz_odak {
            satırlar.push(format!(
                "yapılandırma.geçersiz_odak = GeçersizOdakDavranışı::{:?};",
                self.geçersiz_odak
            ));
        }
        if self.gösterge_ankrajı != taban.gösterge_ankrajı
            || self.gösterge_açıklaması != taban.gösterge_açıklaması
        {
            satırlar.push(match self.gösterge_ankrajı {
                Some(yerleşim) => format!(
                    "yapılandırma.durum_göstergesi = Some(DurumGöstergesiYapılandırması {{\n    \
                     yerleşim: DurumGöstergesiYerleşimTercihi::{yerleşim:?},\n    \
                     açıklama: DurumGöstergesiAçıklamaTercihi::{:?},\n}});",
                    self.gösterge_açıklaması
                ),
                None => "// `§16.2` gösterge yapılandırmayla kapalı.\n\
                         yapılandırma.durum_göstergesi = None;"
                    .to_owned(),
            });
        }
        if self.üzerine_yazma != taban.üzerine_yazma {
            satırlar.push(format!(
                "yapılandırma.üzerine_yazma = {};",
                self.üzerine_yazma
            ));
        }
        if self.uzunluk_sınırı {
            satırlar.push(format!(
                "yapılandırma.uzunluk_sınırı = Some(UzunlukSınırı {{\n    \
                 en_fazla_grafem: 12,\n    davranış: UzunlukSınırıDavranışı::{:?},\n}});",
                self.uzunluk_davranışı
            ));
        }
        if self.sayaç {
            satırlar.push(format!(
                "yapılandırma.sayaç = Some(SayaçYapılandırması {{\n    \
                 birim: SayımBirimi::{:?}, sınırı_göster: {},\n}});",
                self.sayaç_birimi, self.sayaç_sınırı_göster
            ));
        }
        if self.sayısal_adım {
            let (küçük, büyük) = self.adım_ölçeği.çift();
            satırlar.push(format!(
                "yapılandırma.sayısal_adım = Some(SayısalAdım {{\n    \
                 küçük: OndalıkDeğer::yeni({}, {})?,\n    \
                 büyük: OndalıkDeğer::yeni({}, {})?,\n    \
                 kata_hizala: {},\n    hizalama_tabanı: AdımHizalamaTabanı::Sıfır,\n    \
                 sarma: {},\n}});",
                küçük.katsayı().unwrap_or_default(),
                küçük.ölçek().unwrap_or_default(),
                büyük.katsayı().unwrap_or_default(),
                büyük.ölçek().unwrap_or_default(),
                self.adım_hizala,
                self.adım_sarma
            ));
            if self.adım_sınırı {
                satırlar.push(
                    "// `§9.6` sınır `§15` aralık kuralından gelir.\n\
                     yapılandırma.doğrulama.kurallar.push(GeçerlilikKuralı {\n    \
                     kural: GeçerlilikKuralTürü::SayısalAralık {\n        \
                     en_az: Some(OndalıkDeğer { katsayı: 0, ölçek: 0 }),\n        \
                     en_fazla: Some(OndalıkDeğer { katsayı: 100, ölçek: 0 }),\n    \
                     },\n    ..\n});"
                        .to_owned(),
                );
            }
        }
        let yuvalar = self.yardımcı_eylem_türleri();
        if !yuvalar.is_empty() {
            satırlar.push(format!(
                "yapılandırma.yardımcı_eylemler = Some(Arc::from([\n{}\n].as_slice()));",
                yuvalar
                    .iter()
                    .map(|y| {
                        // Yuva alanları kamusal; kurucudan sapan kip
                        // açık atamayla yazılır. `.görünürlükle(..)` gibi
                        // bir zincir kanonikte **yok** — üretilen kodun
                        // kopyalanıp derlenebilmesi gerekiyor.
                        // `§23.1` ürün eylemi kimliği ürünündür; `{y:?}`
                        // ham `EylemKimliği` basar ve kopyalanan kod
                        // derlenmez.
                        let tür_kodu = match y {
                            YardımcıEylemTürü::Ürün(_) => {
                                "YardımcıEylemTürü::Ürün(ürün_eylem_kimliği)".to_owned()
                            }
                            diğer => format!("YardımcıEylemTürü::{diğer:?}"),
                        };
                        let sapma = self.yuva_görünürlüğü != taban.yuva_görünürlüğü
                            || !self.yuvalar_etkin
                            || (*y == YardımcıEylemTürü::AramayıBaşlat
                                && self.arama_gönderime_bağlı);
                        if !sapma {
                            return if self.yuva_adları {
                                format!(
                                    "    YardımcıEylemYuvası::kademeli(yardımcı_bileşen_kimliği, \
                                     {tür_kodu})\n        \
                                     .adla(Kullanıcıİletisi::hazır(\"Yuva adı\")),"
                                )
                            } else {
                                format!(
                                    "    // `ORT-009` adsız yuva erişilebilir ağaca girmez.\n    \
                                     YardımcıEylemYuvası::kademeli(yardımcı_bileşen_kimliği, \
                                     {tür_kodu}),"
                                )
                            };
                        }
                        let mut satır = format!(
                            "    {{\n        let mut yuva = YardımcıEylemYuvası::kademeli(\n            \
                             yardımcı_bileşen_kimliği,\n            \
                             {tür_kodu},\n        );"
                        );
                        if self.yuva_görünürlüğü != taban.yuva_görünürlüğü {
                            satır.push_str(&format!(
                                "\n        yuva.görünürlük = YardımcıEylemGörünürlüğü::{:?};",
                                self.yuva_görünürlüğü
                            ));
                        }
                        if !self.yuvalar_etkin {
                            satır.push_str("\n        yuva.etkin = false;");
                        }
                        if *y == YardımcıEylemTürü::AramayıBaşlat && self.arama_gönderime_bağlı {
                            satır.push_str(
                                "\n        yuva.çalışma = \
                                 YardımcıEylemÇalışması::AlanınGönderimineBağlı;",
                            );
                        }
                        if self.yuva_adları {
                            satır.push_str(
                                "\n        yuva = yuva.adla(Kullanıcıİletisi::hazır(\"Yuva adı\"));",
                            );
                        }
                        satır.push_str("\n        yuva\n    },");
                        satır
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            ));
        }
        if self.ek_sunum_rolü != taban.ek_sunum_rolü {
            satırlar.push(format!(
                "// ön/son ek: ek.sunum_rolü = SabitİçerikSunumRolü::{:?};",
                self.ek_sunum_rolü
            ));
        }
        if self.dikey != taban.dikey {
            satırlar.push(format!(
                "yapılandırma.hizalama.dikey = GirişDikeyHizalama::{:?};",
                self.dikey
            ));
        }
        if self.hizalama != taban.hizalama {
            satırlar.push(format!(
                "yapılandırma.hizalama.yatay = GirişYatayHizalama::{:?};",
                self.hizalama
            ));
        }
        match self.köşe_pikseli {
            Some(piksel) => satırlar.push(format!(
                "yapılandırma.şekil = KutuŞekliTercihi::Yarıçap(px({piksel}.));"
            )),
            None if self.şekil != taban.şekil => satırlar.push(format!(
                "yapılandırma.şekil = KutuŞekliTercihi::Açık(DüğmeŞekli::{:?});",
                self.şekil
            )),
            None => {}
        }
        if self.sekme_durağı != taban.sekme_durağı {
            satırlar.push(format!(
                "yapılandırma.odak.sekme_durağı = {};",
                self.sekme_durağı
            ));
        }
        if self.odak_seçimi != taban.odak_seçimi {
            satırlar.push(format!(
                "yapılandırma.odak_seçimi = OdakSeçimi::{:?};",
                self.odak_seçimi
            ));
        }
        if self.kabul_seçimi != taban.kabul_seçimi {
            satırlar.push(format!(
                "yapılandırma.kabul_seçimi = KabulSeçimi::{:?};",
                self.kabul_seçimi
            ));
        }
        if self.dış_tıklamada_odağı_bırak != taban.dış_tıklamada_odağı_bırak {
            satırlar.push(format!(
                "yapılandırma.dış_tıklamada_odağı_bırak = {};",
                self.dış_tıklamada_odağı_bırak
            ));
        }
        if self.enter != taban.enter {
            satırlar.push(format!(
                "yapılandırma.enter = EnterDavranışı::{:?};",
                self.enter
            ));
        }
        if self.salt_okunur {
            satırlar.push("yapılandırma.salt_okunur = true;".to_owned());
        }
        if !self.etkin {
            satırlar.push("yapılandırma.etkin = false;".to_owned());
        }
        satırlar.join("\n")
    }
}

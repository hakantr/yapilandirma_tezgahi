//! Tezgâh panelleri.
//!
//! Kök `GaleriUygulaması` alanın durum değişimlerini dinlemez: alan
//! durumunu okuyan kartlar kendi entity'lerinde yaşar ve alanın bildirimi
//! yalnız onları kirletir. Kazanç **yarıçaptır**, sıklık değil — GPUI
//! gerçekleşen her çizimde kökten render eder, yani bu paneller de her
//! çizimde yeniden kurulur; kirlenen alan tuş vuruşunda kabuğun tamamına
//! değil bu üç küçük panele değer.
//!
//! Sağ kolon (`BölümlerPaneli`) da burada yaşar ama alanı gözlemez:
//! `Entity::cached` sınırındadır ve tuş vuruşu karelerinde kurulmaz.
//! Geçersizlemesi `notify` ile değil `refresh` ile olur — gerekçe tipin
//! belgesinde.
//!
//! Panel çizimleri `sergiler.rs`'teki `pub(crate)` fonksiyonları kullanır;
//! kart gövdeleri orada kalır çünkü `§16.2` yapısal kanıt testleri
//! (`tezgah_gosterge.rs`, `tezgah_f4.rs`, `tezgah_kabul.rs`) o dosyayı
//! `include_str!` ile okur.

use gpui::{
    AnyElement, Context, Entity, IntoElement, Render, WeakEntity, Window, div, prelude::*, px,
};
#[cfg(feature = "olcum-izleyici")]
use gpui::{ListAlignment, ListOffset, ListSizingBehavior, ListState, list};
use gpui_bilesenleri::GirişKutusu;

use crate::{GaleriUygulaması, OLAY_AKIŞI_SINIRI, TezgahOlayı};

/// Tezgâhın panel entity'leri; profil girdisinde birlikte taşınır.
///
/// İlk üçü alanı gözler ve alan bildirdiğinde yalnız kendileri kirlenir;
/// `bölümler` sağ kolondur ve kök çiziminin bir parçası olarak çizilir.
#[derive(Clone)]
pub struct TezgahPanelleri {
    pub alan_durumu: Entity<AlanDurumPaneli>,
    pub olay_akışı: Entity<OlayAkışıPaneli>,
    pub yuva_notu: Entity<YuvaNotuPaneli>,
    pub bölümler: Entity<BölümlerPaneli>,
    /// Değişken yükseklikli sol kartların yalnız görünür bölümünü kuran
    /// ölçüm paneli. Normal ürün yolunda çizim ağacına girmez.
    #[cfg(feature = "olcum-izleyici")]
    pub sanal_sol: Entity<SanalSolKolonPaneli>,
}

// Sol kolon sanallaştırma deneyinin çalışma zamanı bayrağı.
#[cfg(feature = "olcum-izleyici")]
thread_local! {
    static SOL_LİSTE_SANAL: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

#[cfg(feature = "olcum-izleyici")]
pub fn sol_liste_sanallaştırmasını_ayarla(açık: bool) {
    SOL_LİSTE_SANAL.with(|değer| değer.set(açık));
}

#[cfg(feature = "olcum-izleyici")]
pub fn sol_liste_sanallaştırması_açık() -> bool {
    SOL_LİSTE_SANAL.with(std::cell::Cell::get)
}

/// A ve B yollarında aynı üst düzey kartı görünür kılan ölçüm konumu.
#[cfg(feature = "olcum-izleyici")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SolListeÖlçümKonumu {
    Üst,
    Orta,
    Son,
}

#[cfg(feature = "olcum-izleyici")]
impl SolListeÖlçümKonumu {
    pub fn adı(self) -> &'static str {
        match self {
            Self::Üst => "üst",
            Self::Orta => "orta",
            Self::Son => "son",
        }
    }
}

/// Sol kaydırma içeriğini GPUI'nin değişken yükseklikli `ListState`
/// öğesiyle sanallaştıran deney paneli.
#[cfg(feature = "olcum-izleyici")]
pub struct SanalSolKolonPaneli {
    kök: WeakEntity<GaleriUygulaması>,
    liste: ListState,
}

#[cfg(feature = "olcum-izleyici")]
impl SanalSolKolonPaneli {
    pub(crate) fn yeni(kök: WeakEntity<GaleriUygulaması>) -> Self {
        Self {
            kök,
            // İlk ölçüm açılış/font ısınmasının dışında bütün yükseklikleri
            // öğrenir; sonraki kareler yalnız görünür aralığı kurar.
            liste: ListState::new(
                crate::metin_girisi_profili::SOL_TOPLAM_KART_SAYISI,
                ListAlignment::Top,
                px(0.),
            )
            .measure_all(),
        }
    }

    pub(crate) fn konumu_ayarla(&mut self, konum: SolListeÖlçümKonumu) {
        // Isınma düzenlemesi geçmiş ve durum kartlarının yüksekliğini
        // değiştirmiş olabilir; ölçüm başlamadan son yüksekliği öğren.
        self.liste.remeasure();
        match konum {
            SolListeÖlçümKonumu::Üst => self.liste.scroll_to(ListOffset::default()),
            SolListeÖlçümKonumu::Orta => self.liste.scroll_to(ListOffset {
                item_ix: 3,
                offset_in_item: px(0.),
            }),
            SolListeÖlçümKonumu::Son => self.liste.scroll_to_end(),
        }
    }

    pub(crate) fn mantıksal_konum(&self) -> (usize, gpui::Pixels) {
        let konum = self.liste.logical_scroll_top();
        (konum.item_ix, konum.offset_in_item)
    }
}

#[cfg(feature = "olcum-izleyici")]
impl Render for SanalSolKolonPaneli {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let kök = self.kök.clone();
        let aralık = crate::görünüm().kolonlar.kart_aralığı;
        let son = self.liste.item_count().saturating_sub(1);
        div()
            .size_full()
            // Dikey olay dış yatay kaydırmaya taşınmasın; list kendi
            // listener'ını önce çalıştırır, bu üst öğe sonra yayılımı keser.
            .on_scroll_wheel(|_, _, bağlam| bağlam.stop_propagation())
            .child(
                list(self.liste.clone(), move |indis, _, bağlam| {
                    let öğe = kök
                        .update(bağlam, |kök, bağlam| kök.sanal_sol_kartı(indis, bağlam))
                        .unwrap_or_else(|_| div().into_any_element());
                    div()
                        .w_full()
                        .when(indis != son, |öğe| öğe.pb(aralık))
                        .child(öğe)
                        .into_any_element()
                })
                .with_sizing_behavior(ListSizingBehavior::Auto)
                .size_full(),
            )
    }
}

/// `C` türetilmiş durumlar ve `§13`/`§19` değer üçlüsü: alan durumunun
/// salt-okunur gözlemi.
///
/// Sonuç **saklanmaz**; her çizimde alandan ödünç okunur. Panelin sakladığı
/// tek şey alan tutamacı ve gözlem aboneliğidir.
pub struct AlanDurumPaneli {
    /// Tercih okumak ve `önem_zemini` düğmesini köke iletmek için; zayıf
    /// tutulur, yoksa kök ile panel birbirini canlı tutardı.
    kök: WeakEntity<GaleriUygulaması>,
    alan: Entity<GirişKutusu>,
    _abonelik: gpui::Subscription,
}

impl AlanDurumPaneli {
    pub(crate) fn yeni(
        kök: WeakEntity<GaleriUygulaması>,
        alan: Entity<GirişKutusu>,
        bağlam: &mut Context<Self>,
    ) -> Self {
        let abonelik = Self::gözle(&alan, bağlam);
        Self {
            kök,
            alan,
            _abonelik: abonelik,
        }
    }

    /// Tür değişince yeniden kurulan alana bağlanır.
    ///
    /// Panel entity'si yaşamaya devam eder; yalnız tutamaç ve abonelik
    /// değişir. Böylece çizim ağacındaki kimliği kararlı kalır.
    pub(crate) fn alanı_bağla(
        &mut self, alan: Entity<GirişKutusu>, bağlam: &mut Context<Self>
    ) {
        self._abonelik = Self::gözle(&alan, bağlam);
        self.alan = alan;
        bağlam.notify();
    }

    fn gözle(alan: &Entity<GirişKutusu>, bağlam: &mut Context<Self>) -> gpui::Subscription {
        // `observe` **bildirim** kanalıdır; `subscribe` olay kanalıdır ve
        // bunun yerine geçmez — alan her durum değişiminde olay yayımlamaz.
        bağlam.observe(alan, |_, _, bağlam| {
            #[cfg(not(feature = "olcum-izleyici"))]
            bağlam.notify();
            #[cfg(feature = "olcum-izleyici")]
            if izleyici_deneyi_durumu().gözlem_panelleri_etkin() {
                izleyici_alan_durumu_bildirimini_say();
                bağlam.notify();
            }
        })
    }

    /// Kökün tercihine yazar; `önem_zemini` düğmesinin yolu.
    pub(crate) fn tercihi_değiştir(
        &self,
        değiştir: impl FnOnce(&mut crate::TezgahTercihleri),
        bağlam: &mut Context<Self>,
    ) {
        self.kök
            .update(bağlam, |kök, bağlam| {
                kök.tezgahı_değiştir(değiştir, bağlam)
            })
            .ok();
    }
}

impl Render for AlanDurumPaneli {
    fn render(&mut self, _pencere: &mut Window, bağlam: &mut Context<Self>) -> impl IntoElement {
        render_ölç(|| self.gövde(bağlam))
    }
}

impl AlanDurumPaneli {
    fn gövde(&mut self, bağlam: &mut Context<Self>) -> gpui::Div {
        // Panel her tuş vuruşunda çizilir; tercihten yalnız gereken dar
        // değer çıkarılır, tam `TezgahTercihleri` klonlanmaz.
        let Ok((önem_zemini, çözücü)) = self.kök.read_with(bağlam, |kök, _| {
            (kök.tezgah_tercihleri().önem_zemini, kök.tezgah_çözücüsü())
        }) else {
            return div();
        };
        let g = crate::görünüm();
        let t = crate::TezgahTokenları::paletten(crate::palet());
        let alan = self.alan.clone();

        // İki kart tek panelde: ikisi de alanın durumunu okur ve aynı
        // bildirimle tazelenir. Aradaki boşluk kolonun kart aralığıdır —
        // panel araya girdiği belli olmayan bir dikiş bırakmaz.
        div()
            .flex()
            .flex_col()
            .gap(g.kolonlar.kart_aralığı)
            .child(
                crate::kart(&g, &t)
                    .gap(g.kolonlar.kart_içi_aralık)
                    .child(crate::bölüm_başlığı(
                        &g,
                        &t,
                        // `ORT-021` anahtarı kök-kapsamlı hizmetten çözülür.
                        &çözücü.çöz(&crate::anahtar("galeri.tezgah.bölüm.turetilmis_durum")),
                    ))
                    .child(crate::sergiler::turetilmis_durum_satırı(
                        önem_zemini,
                        &alan,
                        bağlam,
                    )),
            )
            .child(crate::kart(&g, &t).child(crate::sergiler::değer_durumu(&alan, bağlam)))
    }
}

/// `§26` olay akışı: alanın ürüne söyledikleri.
///
/// Akışın sahibi paneldir; kök olay biriktirmez. Alanın olay aboneliği de
/// burada yaşar — her tuş vuruşunda kökü değil yalnız bu paneli kirletir.
pub struct OlayAkışıPaneli {
    /// En yeni **başta**.
    olaylar: Vec<TezgahOlayı>,
    _abonelik: gpui::Subscription,
}

impl OlayAkışıPaneli {
    pub(crate) fn yeni(alan: Entity<GirişKutusu>, bağlam: &mut Context<Self>) -> Self {
        let abonelik = Self::abone_ol(&alan, bağlam);
        Self {
            olaylar: Vec::new(),
            _abonelik: abonelik,
        }
    }

    /// Tür değişince yeniden kurulan alana bağlanır; akış sıfırlanmaz —
    /// önceki alanın yayımladıkları da "alan ürüne ne söyledi"nin parçası.
    pub(crate) fn alanı_bağla(
        &mut self, alan: Entity<GirişKutusu>, bağlam: &mut Context<Self>
    ) {
        self._abonelik = Self::abone_ol(&alan, bağlam);
        bağlam.notify();
    }

    fn abone_ol(alan: &Entity<GirişKutusu>, bağlam: &mut Context<Self>) -> gpui::Subscription {
        bağlam.subscribe(alan, |panel, _alan, olay, bağlam| {
            // Ölçüm kapısı tüketici ablation'ından bağımsız kalmalı. Olay
            // akışı kapalı fazda bile gerçek düzenleme sayılır; yalnız
            // özet/liste mutasyonu ve panel bildirimi kesilir.
            if matches!(
                olay,
                gpui_bilesenleri::GirişOlayı::DüzenlemeMetniDeğişti { .. }
            ) {
                DÜZENLEME_SAYISI.with(|sayaç| sayaç.set(sayaç.get().saturating_add(1)));
            }
            #[cfg(not(feature = "olcum-izleyici"))]
            panel.kaydet(crate::olay_özeti(olay), bağlam);
            #[cfg(feature = "olcum-izleyici")]
            if izleyici_deneyi_durumu().olay_akışı_etkin() {
                izleyici_olay_akışı_kaydını_say();
                panel.kaydet(crate::olay_özeti(olay), bağlam);
            }
        })
    }

    /// Akışa bir olay ekler.
    ///
    /// Art arda gelen aynı olay yeni satır açmaz, sayacı artırır: metin
    /// yazarken alan her tuşta `DüzenlemeMetniDeğişti` yayımlıyor ve akış
    /// tek bir tuş dizisiyle doluyordu.
    fn kaydet(&mut self, olay: TezgahOlayı, bağlam: &mut Context<Self>) {
        match self.olaylar.first_mut() {
            Some(baş) if baş.ad == olay.ad && baş.özet == olay.özet => {
                baş.sayı = baş.sayı.saturating_add(1);
            }
            _ => {
                self.olaylar.insert(0, olay);
                self.olaylar.truncate(OLAY_AKIŞI_SINIRI);
            }
        }
        bağlam.notify();
    }

    /// Akışı boşaltır.
    pub(crate) fn temizle(&mut self, bağlam: &mut Context<Self>) {
        self.olaylar.clear();
        bağlam.notify();
    }
}

impl Render for OlayAkışıPaneli {
    fn render(&mut self, _pencere: &mut Window, bağlam: &mut Context<Self>) -> impl IntoElement {
        render_ölç(|| {
            let g = crate::görünüm();
            let t = crate::TezgahTokenları::paletten(crate::palet());
            crate::kart(&g, &t).child(crate::sergiler::olay_akışı(&self.olaylar, bağlam))
        })
    }
}

/// `§23`/`§22` yuva görünürlük notu: kabuk yuvaları kartının alan okuyan
/// tek satırları.
///
/// Kartın kendisi tercih eksenidir ve kökte çizilir; not ise "kutu şu an
/// boş mu"ya bakar. Kökün çizim yolunda alan okuması bırakmamak için not
/// kendi gözleyen entity'sinde yaşar — sol kolon bir gün önbelleğe
/// alındığında da bayatlamaz.
pub struct YuvaNotuPaneli {
    kök: WeakEntity<GaleriUygulaması>,
    alan: Entity<GirişKutusu>,
    _abonelik: gpui::Subscription,
}

impl YuvaNotuPaneli {
    pub(crate) fn yeni(
        kök: WeakEntity<GaleriUygulaması>,
        alan: Entity<GirişKutusu>,
        bağlam: &mut Context<Self>,
    ) -> Self {
        let abonelik = Self::gözle(&alan, bağlam);
        Self {
            kök,
            alan,
            _abonelik: abonelik,
        }
    }

    /// Tür değişince yeniden kurulan alana bağlanır.
    pub(crate) fn alanı_bağla(
        &mut self, alan: Entity<GirişKutusu>, bağlam: &mut Context<Self>
    ) {
        self._abonelik = Self::gözle(&alan, bağlam);
        self.alan = alan;
        bağlam.notify();
    }

    fn gözle(alan: &Entity<GirişKutusu>, bağlam: &mut Context<Self>) -> gpui::Subscription {
        bağlam.observe(alan, |_, _, bağlam| {
            #[cfg(not(feature = "olcum-izleyici"))]
            bağlam.notify();
            #[cfg(feature = "olcum-izleyici")]
            if izleyici_deneyi_durumu().gözlem_panelleri_etkin() {
                izleyici_yuva_notu_bildirimini_say();
                bağlam.notify();
            }
        })
    }
}

impl Render for YuvaNotuPaneli {
    fn render(&mut self, _pencere: &mut Window, bağlam: &mut Context<Self>) -> impl IntoElement {
        render_ölç(|| {
            // Panel her tuş vuruşunda çizilir; tercihten yalnız gereken üç
            // dar değer çıkarılır, tam `TezgahTercihleri` klonlanmaz.
            let Ok((yuva_görünürlüğü, açık_yuva_sayısı, parola_yuvası_var)) =
                self.kök.read_with(bağlam, |kök, _| {
                    let tercih = kök.tezgah_tercihleri();
                    (
                        tercih.yuva_görünürlüğü,
                        tercih.açık_yuva_sayısı(),
                        tercih.görünürlük.parola_yuvası_var(),
                    )
                })
            else {
                return div();
            };
            let alan = self.alan.clone();
            // Not yokken boş `div` kalır: kart kendi aralarını kenar
            // boşluklarıyla kurduğu için boş çocuk görünür iz bırakmaz.
            crate::sergiler::yuva_görünürlük_notu(
                yuva_görünürlüğü,
                açık_yuva_sayısı,
                parola_yuvası_var,
                &alan,
                bağlam,
            )
            .unwrap_or_else(div)
        })
    }
}

/// Sağ kolon: yapılandırma bölümlerini kendi entity'sinde çizen panel.
///
/// Bölüm kartlarının içindeki tıklama dinleyicileri köke bağlı kalır:
/// çizim, kökü `update` ile açıp bölümleri kökün kendi bağlamında üretir.
/// Kabuk (`tezgah/govde.rs`) kolonu hazır element olarak alır ve yalnız
/// yerleştirir.
///
/// Kolon `Entity::cached` sınırındadır: tuş vuruşu karelerinde element
/// ağacı kurulmaz, prepaint/paint aralıkları yeniden kullanılır.
///
/// **Geçersizleme `refresh` ile yapılır, `notify` ile değil.** GPUI'de
/// `App::notify` bir entity'nin bildirimini yalnız o entity pencerenin
/// `tracked_entities` kümesindeyken `invalidate_view`e çevirir; önbellekten
/// dönen bir view render edilmediği için o kümeye kendi kimliğiyle girmez.
/// Ölçüldü: kökün bildirimi de, panele doğrudan `notify` de kolonu
/// yeniden kurdurmuyor — kolon açılıştaki hâlinde donuyordu. Kökün
/// `kolonu_geçersizle` yolu bu yüzden `refresh_windows` çağırır. İki yönü
/// birden tutan kapı: `tests/kolon_tazeligi.rs` (ayrıntı: raporun §6'sı).
pub struct BölümlerPaneli {
    kök: WeakEntity<GaleriUygulaması>,
}

thread_local! {
    /// Sağ kolonun çizim sayacı.
    ///
    /// "Kolon bu karede kuruldu mu" sorusunu kare süresi yanıtlayamaz: kolon
    /// kurulumu toplam karenin küçük bir payıdır ve gürültüde kaybolur. Bu
    /// yüzden soru doğrudan sayılır — üçüncü turun önbellek bulgusu da bu
    /// sayaçla çıktı. `tests/kolon_tazeligi.rs` ve ölçüm koşumu okur.
    ///
    /// **Thread-local olmalı.** Süreç genelinde tek bir sayaç, aynı süreçte
    /// paralel koşan testlerin çizimlerini birbirine karıştırıyor ve
    /// sahte başarısızlık üretiyordu. Bir GPUI uygulaması zaten kendi
    /// iş parçacığında çizer, yani kapsam doğal olarak doğru olan.
    static BÖLÜM_ÇİZİMİ: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

/// Bu iş parçacığında sağ kolonun şimdiye kadarki çizim sayısı.
pub fn bölüm_çizim_sayısı() -> u64 {
    BÖLÜM_ÇİZİMİ.with(std::cell::Cell::get)
}

thread_local! {
    /// Tezgâhın kendi `render` gövdelerinde geçen toplam süre (ns).
    ///
    /// `Window::draw` bir karede şunları yapar: view'ların `render`
    /// gövdeleri (element ağacının **kurulumu**), yerleşim, prepaint,
    /// paint ve platform işi (metin shaping, glif rasterizasyonu, sahne
    /// kodlama). Gerçek pencerede `draw` p50 ~20,8 ms ölçüldü ve bu,
    /// headless koşumun 12 katıydı — ama hangi payın kimin olduğu
    /// bilinmiyordu. Bu sayaç yalnız **tezgâhın kendi kurduğu** işi
    /// ölçer; `draw` toplamından çıkarılınca kalan, GPUI ve platform
    /// katmanının payıdır.
    static RENDER_NS: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

/// Tezgâhın `render` gövdelerinde geçen toplam süre (nanosaniye).
pub fn render_toplam_ns() -> u64 {
    RENDER_NS.with(std::cell::Cell::get)
}

thread_local! {
    /// Alanın yayımladığı `DüzenlemeMetniDeğişti` olaylarının sayısı.
    ///
    /// Ölçüm penceresinin kapısı budur. Platform girdi histogramı
    /// **her** geçersizleştiren olayı sayar — metin alanına yapılan bir
    /// fare tıklaması da oraya girer — yani "ilk girdi" ile "ilk gerçek
    /// düzenleme" aynı şey değildir. Ölçüm yazma evresini hedeflediği
    /// için kapı bu sayaca bağlanır.
    static DÜZENLEME_SAYISI: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

/// Alanda yapılan metin düzenlemelerinin sayısı (bu iş parçacığında).
pub fn düzenleme_sayısı() -> u64 {
    DÜZENLEME_SAYISI.with(std::cell::Cell::get)
}

/// Gerçek pencere tüketici-ablation deneyinin dört durumu.
///
/// Bu yalnız galeri ölçüm yüzeyidir; varsayılan Tümü ürün davranışını
/// korur. Abonelik nesneleri fazlar arasında sökülüp yeniden kurulmaz:
/// ölçülen fark callback dispatch'i değil, olay özeti/liste mutasyonu ve
/// panel bildirimlerinin doğurduğu geçersizleme yarıçapıdır.
#[cfg(feature = "olcum-izleyici")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum İzleyiciDeneyiDurumu {
    /// Olay akışı ve iki bildirim gözlemcisi etkin.
    Tümü,
    /// Yalnız olay akışının özeti, kaydı ve bildirimi kapalı.
    OlayAkışıYok,
    /// AlanDurumPaneli ve YuvaNotuPaneli bildirimleri kapalı.
    GözlemPanelleriYok,
    /// Üç tüketicinin yan etkisi de kapalı; alanın kendi yolu değişmez.
    Hiçbiri,
}

#[cfg(feature = "olcum-izleyici")]
impl İzleyiciDeneyiDurumu {
    pub fn ad(self) -> &'static str {
        match self {
            Self::Tümü => "A tümü",
            Self::OlayAkışıYok => "B olay akışı yok",
            Self::GözlemPanelleriYok => "C gözlem panelleri yok",
            Self::Hiçbiri => "D hiçbiri",
        }
    }

    fn olay_akışı_etkin(self) -> bool {
        matches!(self, Self::Tümü | Self::GözlemPanelleriYok)
    }

    fn gözlem_panelleri_etkin(self) -> bool {
        matches!(self, Self::Tümü | Self::OlayAkışıYok)
    }
}

#[cfg(feature = "olcum-izleyici")]
thread_local! {
    static İZLEYİCİ_DENEYİ_DURUMU: std::cell::Cell<İzleyiciDeneyiDurumu> =
        const { std::cell::Cell::new(İzleyiciDeneyiDurumu::Tümü) };
}

/// Ana iş parçacığındaki tüketici-ablation durumunu değiştirir.
///
/// Çağrıdan sonra pencere yenilenmeli ve geçiş karesi ölçüm dışında
/// bırakılmalıdır. Normal uygulama bu fonksiyonu çağırmaz.
#[cfg(feature = "olcum-izleyici")]
pub fn izleyici_deneyi_durumunu_ayarla(durum: İzleyiciDeneyiDurumu) {
    İZLEYİCİ_DENEYİ_DURUMU.with(|hücre| hücre.set(durum));
}

#[cfg(feature = "olcum-izleyici")]
fn izleyici_deneyi_durumu() -> İzleyiciDeneyiDurumu {
    İZLEYİCİ_DENEYİ_DURUMU.with(std::cell::Cell::get)
}

/// Bir ölçüm fazında gerçekten çalıştırılan üç tüketici yan etkisi.
#[cfg(feature = "olcum-izleyici")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct İzleyiciEtkiSayacı {
    pub alan_durumu_bildirimi: u64,
    pub olay_akışı_kaydı: u64,
    pub yuva_notu_bildirimi: u64,
}

#[cfg(feature = "olcum-izleyici")]
thread_local! {
    static İZLEYİCİ_ETKİ_SAYACI: std::cell::Cell<İzleyiciEtkiSayacı> =
        const { std::cell::Cell::new(İzleyiciEtkiSayacı {
            alan_durumu_bildirimi: 0,
            olay_akışı_kaydı: 0,
            yuva_notu_bildirimi: 0,
        }) };
}

#[cfg(feature = "olcum-izleyici")]
fn izleyici_etkisini_say(değiştir: impl FnOnce(&mut İzleyiciEtkiSayacı)) {
    İZLEYİCİ_ETKİ_SAYACI.with(|hücre| {
        let mut sayaç = hücre.get();
        değiştir(&mut sayaç);
        hücre.set(sayaç);
    });
}

#[cfg(feature = "olcum-izleyici")]
fn izleyici_alan_durumu_bildirimini_say() {
    izleyici_etkisini_say(|sayaç| {
        sayaç.alan_durumu_bildirimi = sayaç.alan_durumu_bildirimi.saturating_add(1);
    });
}

#[cfg(feature = "olcum-izleyici")]
fn izleyici_olay_akışı_kaydını_say() {
    izleyici_etkisini_say(|sayaç| {
        sayaç.olay_akışı_kaydı = sayaç.olay_akışı_kaydı.saturating_add(1);
    });
}

#[cfg(feature = "olcum-izleyici")]
fn izleyici_yuva_notu_bildirimini_say() {
    izleyici_etkisini_say(|sayaç| {
        sayaç.yuva_notu_bildirimi = sayaç.yuva_notu_bildirimi.saturating_add(1);
    });
}

#[cfg(feature = "olcum-izleyici")]
pub fn izleyici_etki_sayacı() -> İzleyiciEtkiSayacı {
    İZLEYİCİ_ETKİ_SAYACI.with(std::cell::Cell::get)
}

#[cfg(feature = "olcum-izleyici")]
pub fn izleyici_etki_sayacını_sıfırla() {
    İZLEYİCİ_ETKİ_SAYACI.with(|sayaç| sayaç.set(İzleyiciEtkiSayacı::default()));
}

#[cfg(all(test, feature = "olcum-izleyici"))]
mod izleyici_deneyi_testleri {
    use super::*;

    #[test]
    fn dört_durum_iki_tüketici_eksenini_doğru_kapar() {
        let durumlar = [
            (İzleyiciDeneyiDurumu::Tümü, true, true),
            (İzleyiciDeneyiDurumu::OlayAkışıYok, false, true),
            (İzleyiciDeneyiDurumu::GözlemPanelleriYok, true, false),
            (İzleyiciDeneyiDurumu::Hiçbiri, false, false),
        ];
        for (durum, olay, gözlem) in durumlar {
            assert_eq!(durum.olay_akışı_etkin(), olay);
            assert_eq!(durum.gözlem_panelleri_etkin(), gözlem);
        }
    }

    #[test]
    fn etki_sayacı_sıfırlanır() {
        izleyici_etki_sayacını_sıfırla();
        izleyici_alan_durumu_bildirimini_say();
        izleyici_olay_akışı_kaydını_say();
        izleyici_yuva_notu_bildirimini_say();
        assert_eq!(
            izleyici_etki_sayacı(),
            İzleyiciEtkiSayacı {
                alan_durumu_bildirimi: 1,
                olay_akışı_kaydı: 1,
                yuva_notu_bildirimi: 1,
            }
        );
        izleyici_etki_sayacını_sıfırla();
        assert_eq!(izleyici_etki_sayacı(), İzleyiciEtkiSayacı::default());
    }
}

/// Sayacı sıfırlar; ölçüm penceresini açılış karelerinden ayırmak için.
///
/// Açılış kareleri (font yükleme, ilk ağaç kurulumu) ortalamayı domine
/// eder ve sürekli kullanımı temsil etmez.
pub fn render_sıfırla() {
    RENDER_NS.with(|toplam| toplam.set(0));
    #[cfg(feature = "olcum-izleyici")]
    crate::sol_kart_kurulumunu_sıfırla();
}

thread_local! {
    /// Bölüm kolonunun önbelleği açık mı.
    ///
    /// Varsayılan `olcum-onbelleksiz` özelliğiyle ters çevrilir — durağan
    /// taban derlemesi hâlâ tek bayrakla kurulur. Ayrıca çalışma zamanında
    /// da değiştirilebilir: gerçek pencerede iki hâli **aynı koşum içinde**
    /// sırayla denemek, iki ayrı koşumu karşılaştırmaktan çok daha temiz bir
    /// deneydir. Makine durumu, termal ve yazma temposu böylece iki kovaya
    /// da eşit dağılır; ayrı koşumlarda bunlar farkı gömecek kadar büyür.
    static ÖNBELLEK_AÇIK: std::cell::Cell<bool> =
        const { std::cell::Cell::new(!cfg!(feature = "olcum-onbelleksiz")) };
}

/// Bölüm kolonu önbelleğinin şu an açık olup olmadığını söyler.
pub fn önbellek_açık() -> bool {
    ÖNBELLEK_AÇIK.with(std::cell::Cell::get)
}

/// Bölüm kolonu önbelleğini açar ya da kapatır.
///
/// Ana iş parçacığından çağrılmalıdır: bayrak thread-local'dır ve çizim
/// orada koşar. Çağrının ardından `Window::refresh` gerekir — ağaç ancak o
/// zaman yeni hâliyle kurulur — ve geçişin kendi karesi zorunlu ıskadır,
/// yani ölçüme katılmamalıdır.
pub fn önbelleği_ayarla(açık: bool) {
    ÖNBELLEK_AÇIK.with(|bayrak| bayrak.set(açık));
}

/// Bir `render` gövdesini ölçer.
///
/// `Instant` çifti kare başına birkaç kez koşar (~25 ns), yani ölçtüğü
/// büyüklüğün yanında görünmez; bu yüzden ölçüm bayrağına bağlanmadı ve
/// her derlemede açık kalır.
pub(crate) fn render_ölç<R>(gövde: impl FnOnce() -> R) -> R {
    let başlangıç = std::time::Instant::now();
    let sonuç = gövde();
    let geçen = başlangıç.elapsed().as_nanos() as u64;
    RENDER_NS.with(|toplam| toplam.set(toplam.get().saturating_add(geçen)));
    sonuç
}

impl BölümlerPaneli {
    pub(crate) fn yeni(kök: &Entity<GaleriUygulaması>) -> Self {
        Self {
            kök: kök.downgrade(),
        }
    }

    /// Panelin gövdeye giren önbellekli elementi.
    ///
    /// Stil kolonun flex sızasını taşır: kalan genişliği alır, sıkışabilir.
    /// Önbellekli view içerikten ölçülmez; boyut bu stilden ve satırın
    /// çapraz ekseninden çözülür — kolon kaydıran bir kap olduğu için bu
    /// kısıt sorun değildir.
    pub(crate) fn öğe(panel: &Entity<Self>) -> AnyElement {
        if !önbellek_açık() {
            // Ölçüm tabanı: aynı ağaç, önbelleksiz. Karşılaştırmanın elle
            // düzenlemeye değil bayrağa dayanması için.
            return div()
                .flex_1()
                .min_w(px(0.))
                .min_h(px(0.))
                .child(panel.clone())
                .into_any_element();
        }
        panel
            .clone()
            .cached(
                gpui::StyleRefinement::default()
                    .flex_1()
                    .min_w(px(0.))
                    .min_h(px(0.)),
            )
            .into_any_element()
    }
}

impl Render for BölümlerPaneli {
    fn render(&mut self, pencere: &mut Window, bağlam: &mut Context<Self>) -> impl IntoElement {
        BÖLÜM_ÇİZİMİ.with(|sayaç| sayaç.set(sayaç.get() + 1));
        render_ölç(|| self.gövde(pencere, bağlam))
    }
}

impl BölümlerPaneli {
    fn gövde(&mut self, pencere: &mut Window, bağlam: &mut Context<Self>) -> gpui::AnyElement {
        // Bölümler kökün bağlamında üretilir: kart içeriklerindeki bütün
        // dinleyiciler `tezgahı_değiştir` ve akrabalarına, yani köke bağlı.
        let Ok((bölümler, çözücü)) = self.kök.update(bağlam, |kök, bağlam| {
            (kök.tezgah_bölümleri(pencere, bağlam), kök.tezgah_çözücüsü())
        }) else {
            return div().into_any_element();
        };
        let g = crate::görünüm();
        let t = crate::TezgahTokenları::paletten(crate::palet());
        // `ORT-021` anahtarları kök-kapsamlı hizmetten çözülür; panel ham
        // dize sözlüğü tanımaz.
        let ad = çözücü.çöz(&crate::anahtar("galeri.tezgah.yapılandırma"));
        crate::yapılandırma_kolonu_gövdesi(bölümler, &g, &t, ad, move |anahtar| {
            çözücü.çöz(anahtar)
        })
        .into_any_element()
    }
}

#[cfg(test)]
mod testler {
    /// Kök alanın bildirimini dinlemez; gözlem panellerin işidir.
    ///
    /// GPUI her çizimde kökten render eder ve bir view ancak kendisi (ya da
    /// bir alt view'ı) bildirdiğinde `dirty_views`e girer. Köke geri
    /// eklenecek bir `observe(&alan)` köprüsü işlevsel olarak gereksizdir
    /// ve ilerideki önbellekli bölgelerin gerekçesini deler — bu test onu
    /// kapıda tutar.
    #[test]
    fn kök_alanı_gözlemez() {
        let lib = include_str!("lib.rs");
        let kod: String = lib
            .lines()
            .filter(|satır| !satır.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            !kod.contains(".observe("),
            "kökte `observe` köprüsü var: alan gözlemi panellerin işidir"
        );
    }

    /// `§29` raporu çizim başına kurulmaz.
    ///
    /// `yapılandırma(...).doğrula()` kökte tek yerde — önbellek ıskasında —
    /// koşar. Çizim başına koşan doğrulama, kimlik fabrikasını da kare
    /// hızında tüketiyordu.
    #[test]
    fn rapor_tek_yerden_kurulur() {
        let lib = include_str!("lib.rs");
        assert_eq!(
            lib.matches(".doğrula()").count(),
            1,
            "rapor önbellek ıskası dışında da kuruluyor"
        );
    }

    /// Görünüm çözümü çizim başına koşmaz.
    ///
    /// `tasarım_görünümünü_çöz` kökte tek yerde — tema sürümüne bağlı
    /// önbelleğin ıskasında — çağrılır. Kare başına çözüm, tam bir tema
    /// anlık görüntüsü kuruluşunu da beraberinde koşturuyordu.
    #[test]
    fn görünüm_çözümü_tek_yerden_koşar() {
        let lib = include_str!("lib.rs");
        assert_eq!(
            lib.matches("tasarım_görünümünü_çöz()").count(),
            1,
            "görünüm çözümü önbellek ıskası dışında da koşuyor"
        );
    }

    /// Olay aboneliği ve alan gözlemi panellerde yaşar.
    ///
    /// Akışın sahibi `OlayAkışıPaneli`, durum gözleminin sahibi
    /// `AlanDurumPaneli`dir; ikisi de bildirimlerini kendine yayımlar.
    #[test]
    fn gözlem_kanalları_panellerdedir() {
        let paneller = include_str!("paneller.rs");
        assert!(paneller.contains("bağlam.observe(alan"));
        assert!(paneller.contains("bağlam.subscribe(alan"));
    }

    /// Sağ kolon önbelleklidir ve kökü gözler.
    ///
    /// İkisi birbirinin ön koşuludur: `cached` sınırı olmadan kolon her
    /// karede yeniden kurulur; kök gözlemi olmadan da önbellek tercih ve
    /// tema değişiminde bayat kalır. Bekçi ikisini birlikte tutar.
    #[test]
    fn bölüm_kolonu_alana_bağlanmaz() {
        let paneller = include_str!("paneller.rs");
        // Testin kendi gövdesi aramaya girmesin: tarama panel tanımından
        // test modülüne kadar sürer.
        let gövde = paneller
            .split_once("pub struct BölümlerPaneli")
            .expect("bölüm paneli tanımlı")
            .1
            .split_once("#[cfg(test)]")
            .expect("test modülü panel tanımından sonra gelir")
            .0;
        // Kolon kökün durumundan çizilir; alana bağlanması tuş vuruşunu
        // yeniden kolona taşırdı — birinci turun kaldırdığı bağ budur.
        assert!(
            !gövde.contains("observe(alan") && !gövde.contains("subscribe(alan"),
            "bölüm paneli alana bağlanmış: tuş vuruşu kolonu kirletir"
        );
        // Önbellek ile geçersizleme yolu **birlikte** yaşar: `cached`
        // sınırı GPUI'de `notify` ile patlamaz, yalnız `refresh` patlatır
        // (§6.3). Biri varken diğeri yoksa kolon donar ve bayat
        // yapılandırma gösterir.
        let lib = include_str!("lib.rs");
        // Ölçüm tabanı bayrağı önbelleği kapatır; kapı yalnız açıkken
        // anlamlıdır ama kaynakta `.cached(` her iki hâlde de durur.
        if gövde.contains(".cached(") {
            assert!(
                lib.contains("bağlam.refresh_windows()"),
                "kolon önbellekli ama geçersizleme yolu yok: `refresh` \
                 çağrılmadan `notify` bu sınırı patlatmaz"
            );
            assert!(
                lib.matches("self.kolonu_geçersizle(bağlam)").count() >= 4,
                "kolonu ilgilendiren kök değişimlerinden biri geçersizleme \
                 çağırmıyor (tercih, tema, seçici, dış bildirim)"
            );
        }
    }

    /// Kökün çizim yolunda alan okuması kalmadı.
    ///
    /// `yuva_görünürlük_notu` kökte çizilen son alan okuyucusuydu; artık
    /// kendi gözleyen panelinde yaşar. Kök her karede çizildiği için bugün
    /// bayatlamazdı, ama sol kolon bir gün önbelleğe alındığında
    /// bayatlardı — bekçi imzayı panel bağlamına sabitler.
    #[test]
    fn yuva_notu_panel_bağlamındadır() {
        let sergiler = include_str!("sergiler.rs");
        let imza = sergiler
            .split_once("pub(crate) fn yuva_görünürlük_notu(")
            .expect("not fonksiyonu tanımlı")
            .1
            .split_once(") ->")
            .expect("imza kapanır")
            .0;
        assert!(
            imza.contains("Context<crate::YuvaNotuPaneli>"),
            "yuva notu kök bağlamına dönmüş: alan okuması köke sızar"
        );
    }
}

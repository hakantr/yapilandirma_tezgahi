//! Alanı gözleyen tezgâh panelleri.
//!
//! Kök `GaleriUygulaması` alanın durum değişimlerini dinlemez. GPUI her
//! çizimde kökten render eder; render atlama yalnız `Entity::cached`
//! sınırlarında, `dirty_views` kümesi üzerinden çalışır ve bir view ancak
//! **kendisi** bildirdiğinde (ya da bir alt view'ı bildirdiğinde) kirlenir.
//! Alan durumunu okuyan kartların kendi entity'lerinde yaşaması bu yüzden
//! yalnız düzen değil, ilerideki önbellekli bölgelerin (sağ kolon bölüm
//! entity'leri) **doğruluk ön koşuludur**: önbelleğe alınmış bir bölge alanı
//! okusaydı, alan değiştiğinde bayat kalırdı — okuma gözleyen panelde
//! durursa alanın bildirimi yalnız paneli kirletir.
//!
//! Panel çizimleri `sergiler.rs`'teki `pub(crate)` fonksiyonları kullanır;
//! kart gövdeleri orada kalır çünkü `§16.2` yapısal kanıt testleri
//! (`tezgah_gosterge.rs`, `tezgah_f4.rs`, `tezgah_kabul.rs`) o dosyayı
//! `include_str!` ile okur.

use gpui::{Context, Entity, IntoElement, Render, WeakEntity, Window, div, prelude::*};
use gpui_bilesenleri::GirişKutusu;

use crate::{GaleriUygulaması, TezgahOlayı, OLAY_AKIŞI_SINIRI};

/// Tezgâhın alan gözleyen panel entity'leri; profil girdisinde birlikte
/// taşınır.
#[derive(Clone)]
pub struct TezgahPanelleri {
    pub alan_durumu: Entity<AlanDurumPaneli>,
    pub olay_akışı: Entity<OlayAkışıPaneli>,
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
    pub(crate) fn alanı_bağla(&mut self, alan: Entity<GirişKutusu>, bağlam: &mut Context<Self>) {
        self._abonelik = Self::gözle(&alan, bağlam);
        self.alan = alan;
        bağlam.notify();
    }

    fn gözle(alan: &Entity<GirişKutusu>, bağlam: &mut Context<Self>) -> gpui::Subscription {
        // `observe` **bildirim** kanalıdır; `subscribe` olay kanalıdır ve
        // bunun yerine geçmez — alan her durum değişiminde olay yayımlamaz.
        bağlam.observe(alan, |_, _, bağlam| bağlam.notify())
    }

    /// Kökün tercihine yazar; `önem_zemini` düğmesinin yolu.
    pub(crate) fn tercihi_değiştir(
        &self,
        değiştir: impl FnOnce(&mut crate::TezgahTercihleri),
        bağlam: &mut Context<Self>,
    ) {
        self.kök
            .update(bağlam, |kök, bağlam| kök.tezgahı_değiştir(değiştir, bağlam))
            .ok();
    }
}

impl Render for AlanDurumPaneli {
    fn render(&mut self, _pencere: &mut Window, bağlam: &mut Context<Self>) -> impl IntoElement {
        let Ok(tercih) = self
            .kök
            .read_with(bağlam, |kök, _| kök.tezgah_tercihleri().clone())
        else {
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
                        &crate::tezgah_bölüm_adı(&crate::anahtar(
                            "galeri.tezgah.bölüm.turetilmis_durum",
                        )),
                    ))
                    .child(crate::sergiler::turetilmis_durum_satırı(
                        &tercih, &alan, bağlam,
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
    pub(crate) fn alanı_bağla(&mut self, alan: Entity<GirişKutusu>, bağlam: &mut Context<Self>) {
        self._abonelik = Self::abone_ol(&alan, bağlam);
        bağlam.notify();
    }

    fn abone_ol(alan: &Entity<GirişKutusu>, bağlam: &mut Context<Self>) -> gpui::Subscription {
        bağlam.subscribe(alan, |panel, _alan, olay, bağlam| {
            panel.kaydet(crate::olay_özeti(olay), bağlam);
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
        let g = crate::görünüm();
        let t = crate::TezgahTokenları::paletten(crate::palet());
        crate::kart(&g, &t).child(crate::sergiler::olay_akışı(&self.olaylar, bağlam))
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
            lib.matches(").doğrula()").count(),
            1,
            "rapor önbellek ıskası dışında da kuruluyor"
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
}

//! Gerçek pencere ölçümü için tek bileşenli kök.
//!
//! Bu kök `GaleriUygulaması`nı, tezgâh kabuğunu ve gözlem panellerini hiç
//! kurmaz. Aynı kanonik `GirişKutusu`nu ve aynı gerçek metin değiştirme
//! yolunu kullanır; böylece toplam draw farkı bileşen ile tüketici ağacını
//! ayırır. Yalnız `olcum-izleyici` özelliğinde derlenir.

use gpui::{Context, Entity, IntoElement, Render, Window, div, prelude::*, px, rgb};
use gpui_bilesenleri::GirişKutusu;

use crate::{
    TezgahTercihleri, YardımcıKimlikleri, galeri_bileşen_kimliği, galeri_simge_kataloğu,
    tezgah_teması, ÖrnekKimliğiFabrikası,
};

/// Yalnız bir yaşayan metin alanı taşıyan ölçüm penceresi kökü.
pub struct MinimalGirişÖlçümü {
    alan: Entity<GirişKutusu>,
    /// Bu pencere kökünün `ORT-002`/`ORT-021` hizmet sahipliği; alanla
    /// birlikte yaşar. Minimal kök ayrı bir uygulama köküdür ve kendi tek
    /// hizmet kökünü taşır.
    _hizmetler: crate::MetinHizmetleriKökü,
}

impl MinimalGirişÖlçümü {
    pub fn yeni(pencere: &mut Window, bağlam: &mut Context<Self>) -> Self {
        let kimlik_fabrikası = ÖrnekKimliğiFabrikası::yeni_süreç_kapsamı()
            .expect("minimal ölçüm kimlik soyu kurulmalı");
        let hizmetler = crate::MetinHizmetleriKökü::kur(&kimlik_fabrikası, None);
        let bileşen =
            galeri_bileşen_kimliği(&kimlik_fabrikası, "galeri.metin_girisi", "minimal-olcum");
        // Tam tezgâhtaki alanla yalnız çevre ağacı farklı kalsın:
        // yapılandırma, örnek değer, tema ve simge kataloğu aynıdır.
        let tezgah = TezgahTercihleri::default();
        let yardımcı_kimlikleri = YardımcıKimlikleri::yeni(&kimlik_fabrikası);
        let yapılandırma =
            tezgah.yapılandırma_kimliklerle(&yardımcı_kimlikleri, &hizmetler.motor());
        let örnek = tezgah.örnek_değer();
        let tema = tezgah_teması(&tezgah.kutu_teması());
        let katalog = galeri_simge_kataloğu();
        // Ölçüm bilinen-geçerli varsayılan tercihle koşar; kuruluş burada
        // düşerse ölçüm ortamı arızasıdır ve exact typed sonuç mesajla
        // taşınır.
        let sonuç = GirişKutusu::kur(
            bileşen,
            hizmetler.unicode(),
            hizmetler.alan_damgası(&kimlik_fabrikası),
            (*hizmetler.yerel_kök()).clone(),
            yapılandırma,
            örnek,
            tema,
            pencere,
            bağlam,
        )
        .unwrap_or_else(|hata| panic!("minimal ölçüm alanı kurulamadı: {hata:?}"));
        let alan = sonuç.bileşen;
        alan.update(bağlam, |alan, _| {
            alan.simge_kataloğu = Some(katalog);
        });
        Self {
            alan,
            _hizmetler: hizmetler,
        }
    }

    /// Tam tezgâhla aynı `EntityInputHandler` düzenleme yolunu koşturur.
    pub fn ölçüm_alanına_yaz(
        &mut self,
        metin: &str,
        pencere: &mut Window,
        bağlam: &mut Context<Self>,
    ) {
        let metin = metin.to_owned();
        self.alan.update(bağlam, |alan, bağlam| {
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
}

impl Render for MinimalGirişÖlçümü {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        crate::render_ölç(|| {
            div()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .bg(rgb(0xf4f1ea))
                .child(div().w(px(640.)).child(self.alan.clone()))
        })
    }
}

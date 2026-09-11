//! Galerinin root-yetkili ORT-016/K09 hizmet tüketicisi.

#![allow(non_ascii_idents)]

use gpui::{AssetSource as _, TestAppContext};
use gpui_bilesenleri::{
    SimgeBoyutTercihi, SimgeGerekliliği, SimgeGörselBiçimi, SimgeÇözümAkıbeti, Simgeİsteği,
    TemaKipi, ÇözülmüşYazıYönü,
};
use gpui_bilesenleri_galeri::{
    GaleriVarlıkKaynağı, bileşen_tuş_bağlarını_kur, galeri_simge_cache_gözlemi,
    galeri_simge_hizmeti, galeri_simge_kimliği,
};
use gpui_bilesenleri_temel::SimgeRolü;

fn istek(kimlik: gpui_bilesenleri::SimgeKimliği) -> Simgeİsteği {
    Simgeİsteği::yeni(
        kimlik,
        SimgeGörselBiçimi::Çizgisel,
        SimgeBoyutTercihi::default(),
        SimgeRolü::Olağan,
        None,
        ÇözülmüşYazıYönü::SoldanSağa,
        SimgeGerekliliği::Zorunlu,
        TemaKipi::Açık,
    )
}

#[test]
fn yardimci_eylem_simgeleri_yasayan_hizmette_tam_cozulur() {
    let bağlam = TestAppContext::single();
    bağlam.update(|bağlam| {
        bileşen_tuş_bağlarını_kur(bağlam);
        let hizmet = galeri_simge_hizmeti(bağlam);
        for ad in [
            "input.clear",
            "input.search",
            "input.reveal",
            "input.reveal-off",
            "input.picker",
            "input.product-action",
            "input.missing",
        ] {
            let kimlik = galeri_simge_kimliği(ad, bağlam)
                .unwrap_or_else(|| panic!("{ad} yaşayan snapshotta kayıtlı olmalı"));
            assert!(matches!(
                hizmet.çöz(&istek(kimlik)),
                Ok(SimgeÇözümAkıbeti::TamÇözüldü(_))
            ));
        }
    });
}

#[test]
fn kayitsiz_tanim_kimlik_uydurmaz_ve_varlik_listesi_exacttir() {
    let bağlam = TestAppContext::single();
    bağlam.update(|bağlam| {
        bileşen_tuş_bağlarını_kur(bağlam);
        assert!(galeri_simge_kimliği("input.kayıtsız", bağlam).is_none());
        assert!(galeri_simge_kimliği("başka.clear", bağlam).is_none());
    });

    let kaynak = GaleriVarlıkKaynağı;
    assert!(kaynak.load("yok.svg").unwrap().is_none());
    // Beş yerleşik yardımcı eylem + ürün + nötr fallback + üç durum glifi.
    assert_eq!(kaynak.list("").unwrap().len(), 10);
    for yol in [
        "close-circle.svg",
        "product.svg",
        "question-circle.svg",
        "exclamation-circle.svg",
        "warning.svg",
        "info-circle.svg",
    ] {
        let baytlar = kaynak
            .load(yol)
            .unwrap_or_else(|hata| panic!("{yol} yüklenemedi: {hata:?}"))
            .unwrap_or_else(|| panic!("{yol} varlık kaynağında yok"));
        let svg = String::from_utf8_lossy(&baytlar);
        assert!(svg.starts_with("<svg"), "{yol} geçerli SVG değil");
        assert!(svg.contains("<path"), "{yol} yol içermiyor");
    }
}

#[test]
fn k09_ayni_mantiksal_istek_once_kacirma_sonra_vurus_uretir() {
    let bağlam = TestAppContext::single();
    bağlam.update(|bağlam| {
        bileşen_tuş_bağlarını_kur(bağlam);
        let hizmet = galeri_simge_hizmeti(bağlam);
        let kimlik =
            galeri_simge_kimliği("input.product-action", bağlam).expect("ürün simgesi kayıtlıdır");
        let istek = istek(kimlik);
        let önce = galeri_simge_cache_gözlemi(bağlam);
        assert!(matches!(
            hizmet.çöz(&istek),
            Ok(SimgeÇözümAkıbeti::TamÇözüldü(_))
        ));
        assert!(matches!(
            hizmet.çöz(&istek),
            Ok(SimgeÇözümAkıbeti::TamÇözüldü(_))
        ));
        let sonra = galeri_simge_cache_gözlemi(bağlam);
        assert_eq!(sonra.mantıksal_kaçırma, önce.mantıksal_kaçırma + 1);
        assert_eq!(sonra.mantıksal_vuruş, önce.mantıksal_vuruş + 1);
        assert!(sonra.mantıksal_girdi >= 1);
        assert!(sonra.mantıksal_payload_baytı > 0);
    });
}

#[test]
fn gosterge_glifleri_fiziksel_varliklarda_ayridir() {
    let kaynak = GaleriVarlıkKaynağı;
    let yükle = |yol: &str| kaynak.load(yol).unwrap().expect("kayıtlı varlık");
    let hata = yükle("exclamation-circle.svg");
    let uyarı = yükle("warning.svg");
    let bilgi = yükle("info-circle.svg");
    assert_ne!(hata.as_ref(), uyarı.as_ref());
    assert_ne!(uyarı.as_ref(), bilgi.as_ref());
    assert_ne!(hata.as_ref(), bilgi.as_ref());
}

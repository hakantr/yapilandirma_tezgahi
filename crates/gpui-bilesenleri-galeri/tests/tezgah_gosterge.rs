//! `§16.2` durum göstergesi: tezgâh ekseni ve gözlem paneli.
//!
//! İki ayrı iddia sınanır. Birincisi **eksen**: tezgâhın ankraj seçimi
//! kanonik `durum_göstergesi` alanına doğru çevriliyor mu, seçiliye yeniden
//! basmak yapılandırmayı `None`'a indiriyor mu.
//!
//! İkincisi **gözlemin sınırı**: panel kanonik opak sonucun salt-okunur
//! gözlemidir, `ORT-019` tanı zarfı değildir. Gerekçe burada okunabilir —
//! test tanı zarfı değildir — ama panelde görünmez.

#![allow(non_ascii_idents)]

use gpui_bilesenleri::{
    DurumGöstergesiAçıklamaTercihi, DurumGöstergesiYerleşimGerekçesi,
    DurumGöstergesiYerleşimTercihi, DurumGöstergesiYerleşimi, ÖrnekKimliğiFabrikası,
};
use gpui_bilesenleri_galeri::TezgahTercihleri;

fn kimlik_fabrikası() -> ÖrnekKimliğiFabrikası {
    ÖrnekKimliğiFabrikası::yeni_süreç_kapsamı().expect("test kimlik kapsamı")
}

/// `F2.1` ankraj tercihi kanonik alana çevrilir.
#[test]
fn ankraj_tercihi_kanonik_alana_cevrilir() {
    let mut t = TezgahTercihleri::default();
    t.gösterge_ankrajı = Some(DurumGöstergesiYerleşimTercihi::UygunsaÜstKöşe);
    t.gösterge_açıklaması = DurumGöstergesiAçıklamaTercihi::SağlayıcıVarsayılanı;

    let yapılandırma = t.yapılandırma(&kimlik_fabrikası()).durum_göstergesi;
    let yapılandırma = yapılandırma.expect("ankraj seçiliyken alan kurulur");
    assert_eq!(
        yapılandırma.yerleşim,
        DurumGöstergesiYerleşimTercihi::UygunsaÜstKöşe
    );
    assert_eq!(
        yapılandırma.açıklama,
        DurumGöstergesiAçıklamaTercihi::SağlayıcıVarsayılanı
    );
}

/// `§16.2.4` gösterge kapalıyken alan **yoktur**.
///
/// Kapalılık ayrı bir yerleşim kademesi değil, `durum_göstergesi` alanının
/// `None` olmasıdır. Tezgâhta bunun karşılığı seçili ankraja yeniden
/// basmaktır — bu yüzden ekranda üçüncü bir "Kapalı" düğmesi yok.
#[test]
fn ankrajsiz_yapilandirmada_alan_yok() {
    let mut t = TezgahTercihleri::default();
    t.gösterge_ankrajı = None;
    assert!(
        t.yapılandırma(&kimlik_fabrikası())
            .durum_göstergesi
            .is_none()
    );
}

/// `F2.1` ankraj değişimi kod panelinde görünür.
#[test]
fn ankraj_kod_panelinde_gorunur() {
    let taban = TezgahTercihleri::default();
    let mut t = taban.clone();
    t.gösterge_ankrajı = None;
    let kod = t.kod();
    assert_ne!(kod, taban.kod());
    assert!(
        kod.contains("durum_göstergesi = None"),
        "kapalı gösterge koda yazılmıyor:\n{kod}"
    );
}

/// `F2.3` bugün üretilebilen dört gerekçe.
///
/// `ÜstKöşe*` gerekçeleri (5–7) sınanamaz: fiziksel kabuk kayıtlı üst-köşe
/// geometri adayı sağlamıyor ve kanonik uygulama fail-closed
/// `ÜstKöşeAdayıYok` yayımlıyor. Bunlar `K2` kapısına bağlıdır.
///
/// Gerekçe burada **okunur** ama panelde gösterilmez: `§16.2.5` gerekçeyi
/// yalnız kayıtlı `ORT-019` koduna eşlenebilir kılıyor ve o kod kümesi
/// fiziksel değil. Test bir tanı zarfı değildir, bu yüzden okuyabilir.
#[test]
fn bugun_uretilebilen_gerekceler() {
    let üretilebilir = [
        DurumGöstergesiYerleşimGerekçesi::YapılandırmaylaKapalı,
        DurumGöstergesiYerleşimGerekçesi::BirincilSorunYok,
        DurumGöstergesiYerleşimGerekçesi::SatırSonuTercihEdildi,
        DurumGöstergesiYerleşimGerekçesi::ÜstKöşeAdayıYok,
    ];
    // Varyantlar birbirinden ayrı: dördü de gerçek gerekçe sınıfıdır.
    for (sıra, gerekçe) in üretilebilir.iter().enumerate() {
        for diğer in &üretilebilir[sıra + 1..] {
            assert_ne!(gerekçe, diğer);
        }
    }

    // Üretilemeyen üçü aday beslemesine bağlı; K2 kapısı açılana kadar
    // hiçbir koşum bunları yayımlayamaz.
    let aday_bekleyenler = [
        DurumGöstergesiYerleşimGerekçesi::ÜstKöşeGeometrisiUygunDeğil,
        DurumGöstergesiYerleşimGerekçesi::ÜstKöşeAnatomiyleÇakışıyor,
        DurumGöstergesiYerleşimGerekçesi::ÜstKöşeUygun,
    ];
    assert_eq!(aday_bekleyenler.len(), 3);
}

/// `F2.2` panel gerekçeyi, sorun kimliğini, sürümü ve ileti metnini yazmaz.
///
/// Yapısal kanıt: gözlem panelini çizen kod bu okuyucuları çağırmaz.
/// Yasağı düzyazıda söylemek yetmez — bir sonraki düzenleme sessizce
/// ekleyebilir.
#[test]
fn panel_yasak_alanlari_okumaz() {
    let kaynak = include_str!("../src/sergiler.rs");
    // Gözlem `C · türetilmiş durumlar` kartına taşındı: taslak sol kolonda
    // ayrı bir gözlem satırı göstermiyor ve gösterge çözümü de türetilmiş
    // bir durum.
    let panel = kaynak
        .split_once("pub(crate) fn turetilmis_durum_satırı(")
        .expect("türetilmiş durum kartı bulunur")
        .1
        .split_once("\n}\n")
        .expect("kart gövdesi kapanır")
        .0;

    for yasak in [
        ".gerekçe()",
        ".değer_sürümü()",
        ".sorun_sürümü()",
        "gösterge_girdisi_sürümü",
    ] {
        assert!(
            !panel.contains(yasak),
            "gözlem paneli `{yasak}` okuyor — §16.2.5 bunu yasaklıyor"
        );
    }
    // Sorun yalnız var/yok olarak yazılır; kimlik yüzeye çıkmaz.
    assert!(panel.contains("birincil_sorun().is_some()"));
}

/// `F2.5` panel sonucu saklamaz; her çizimde ödünç okur.
///
/// Yapısal kanıt: galeri tarafında `DurumGöstergesiDurumu`nun hiçbir alanı
/// bir yapıya kopyalanmaz. Saklanmayan sonuç bayatlayamaz — bu yüzden sürüm
/// karşılaştırması da gerekmez.
#[test]
fn panel_sonucu_saklamaz() {
    for (ad, kaynak) in [
        ("sergiler.rs", include_str!("../src/sergiler.rs")),
        ("lib.rs", include_str!("../src/lib.rs")),
        (
            "metin_girisi_tezgahi.rs",
            include_str!("../src/metin_girisi_tezgahi.rs"),
        ),
        // Gözlem panelleri ayrı entity'lere taşındı; kapı onları da kapsar.
        ("paneller.rs", include_str!("../src/paneller.rs")),
    ] {
        // Yorumlar elenir: tipin **adını** anmak onu saklamak değildir ve
        // neden saklanmadığını açıklayan yorum testi tetiklememeli.
        let kod: String = kaynak
            .lines()
            .filter(|satır| !satır.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            !kod.contains("DurumGöstergesiDurumu"),
            "{ad} opak sonucu saklıyor olabilir"
        );
    }
}

/// `DurumGöstergesiYerleşimi` üç sonucu ayrı taşır.
#[test]
fn yerlesim_sonuclari_ayridir() {
    assert_ne!(
        DurumGöstergesiYerleşimi::Yok,
        DurumGöstergesiYerleşimi::SatırSonu
    );
    assert_ne!(
        DurumGöstergesiYerleşimi::SatırSonu,
        DurumGöstergesiYerleşimi::ÜstKöşe
    );
}

/// `§13/12` seçili ankraja yeniden basmak `None`'a indirir.
///
/// Üç durumlu bir eksenin üçüncü değeri bir düğme değil, aynı düğmeye
/// ikinci basıştır. Ayrı bir "Kapalı" düğmesi kapalılığı ankrajla eşdeğer
/// üçüncü bir kademe gibi gösterirdi.
#[test]
fn secili_ankraja_yeniden_basmak_kapatir() {
    let mut t = TezgahTercihleri::default();
    t.gösterge_ankrajı = None;

    t.gösterge_ankrajına_bas(DurumGöstergesiYerleşimTercihi::SatırSonu);
    assert_eq!(
        t.gösterge_ankrajı,
        Some(DurumGöstergesiYerleşimTercihi::SatırSonu)
    );

    // Aynı ankraja ikinci basış kapatır.
    t.gösterge_ankrajına_bas(DurumGöstergesiYerleşimTercihi::SatırSonu);
    assert!(t.gösterge_ankrajı.is_none());

    // Başka bir ankraja basış kapatmaz, ankrajı değiştirir.
    t.gösterge_ankrajına_bas(DurumGöstergesiYerleşimTercihi::SatırSonu);
    t.gösterge_ankrajına_bas(DurumGöstergesiYerleşimTercihi::UygunsaÜstKöşe);
    assert_eq!(
        t.gösterge_ankrajı,
        Some(DurumGöstergesiYerleşimTercihi::UygunsaÜstKöşe)
    );
}

/// `F2.4` açıklama yüzeyinin kurulamama nedeni fiziksel bir eksikliktir.
///
/// `GirişYüzeyBağı` fiziksel API'de yok ve `GirişYapılandırmaHatası`
/// `GirişYüzeyBağıEksik` varyantını taşımıyor. Bu yüzden ekranda
/// gösterilecek şey "sözleşmenin ürettiği kuruluş reddi" değil, "kanonikte
/// beklenen, fiziksel API'de henüz bulunmayan sonuç" etiketidir. Yerel sahte
/// balon ve sessiz `Yok` fallback'i `§16.2.4` ile yasaktır.
#[test]
fn aciklama_yuzeyi_fiziksel_degil() {
    let kaynak = include_str!("../../../../gpui_bilesenleri/crates/gpui-bilesenleri/src/metin_girisi/api.rs");
    let kod: String = kaynak
        .lines()
        .filter(|satır| !satır.trim_start().starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        !kod.contains("GirişYüzeyBağı"),
        "GirişYüzeyBağı fizikselleşmiş — F2.4 yeniden değerlendirilmeli"
    );

    // Tercih seçilebilir kalır: yapılandırma alanı gerçek, açılmayan şey
    // yüzeyin kendisi.
    let mut t = TezgahTercihleri::default();
    t.gösterge_açıklaması = DurumGöstergesiAçıklamaTercihi::SağlayıcıVarsayılanı;
    let yapılandırma = t
        .yapılandırma(&kimlik_fabrikası())
        .durum_göstergesi
        .expect("ankraj varsayılanda seçili");
    assert_eq!(
        yapılandırma.açıklama,
        DurumGöstergesiAçıklamaTercihi::SağlayıcıVarsayılanı
    );
}

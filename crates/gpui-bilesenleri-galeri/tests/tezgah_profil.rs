//! `BİL-010` tezgâh profilinin bölüm üretimi.
//!
//! Profil, tezgâh kabuğuna hangi bölümleri verdiğine kendisi karar verir
//! (`§9` tür süzgeci). Bu dosya kararın doğru olduğunu gerçek bir GPUI
//! penceresinde koşturarak sınar: seçili türde **kurulamayan** eksen hiç
//! bölüm üretmez.

#![allow(non_ascii_idents)]

use gpui::TestAppContext;
use gpui_bilesenleri_galeri::TezgahDeğerKipi;
use gpui_bilesenleri_galeri::{Akış, GaleriUygulaması, bileşen_tuş_bağlarını_kur};

/// Verilen değer türünde profilin ürettiği bölüm kimlikleri ve akışları.
///
/// Bölüm listesi artık `Tezgahİçeriği`nde taşınmaz: sağ kolon önbellekli
/// bölüm panelinin çizimidir ve o çizim de bu API'den okur — test ile
/// ekran aynı kurulumu görür.
fn bölümler(bağlam: &mut TestAppContext, tür: TezgahDeğerKipi) -> Vec<(&'static str, Akış)> {
    bağlam.update(|bağlam| bileşen_tuş_bağlarını_kur(bağlam));
    let (uygulama, görsel) = bağlam.add_window_view(move |_, _| GaleriUygulaması::yeni());
    görsel.update(|pencere, bağlam| {
        uygulama.update(bağlam, |uygulama, bağlam| {
            uygulama.tezgahı_değiştir(|t| t.değer_türü = tür, bağlam);
        });
        uygulama.update(bağlam, |uygulama, bağlam| {
            uygulama
                .tezgah_bölümleri(pencere, bağlam)
                .iter()
                .map(|b| (b.kimlik, b.akış))
                .collect()
        })
    })
}

fn kimlikler(bölümler: &[(&'static str, Akış)]) -> Vec<&'static str> {
    bölümler.iter().map(|(k, _)| *k).collect()
}

#[gpui::test]
fn metin_türünde_sayısal_adım_ve_saat_dilimi_kurulmaz(bağlam: &mut TestAppContext) {
    let bölümler = bölümler(bağlam, TezgahDeğerKipi::Metin);
    let kimlikler = kimlikler(&bölümler);
    // `§9.6` sayısal adım ve ORT-002 saat dilimi metin alanında kurulamaz:
    // gizlenmez, hiç üretilmez.
    assert!(
        !kimlikler.contains(&"sayisal_adim"),
        "metin türünde s96: {kimlikler:?}"
    );
    assert!(
        !kimlikler.contains(&"saat_dilimi"),
        "metin türünde sz: {kimlikler:?}"
    );
    // Her türde kurulan bölümler yerinde.
    for beklenen in [
        "deger_turu",
        "tur_tanimi_ve_maske",
        "bicim_profili",
        "odak_ve_kabul",
    ] {
        assert!(
            kimlikler.contains(&beklenen),
            "{beklenen} eksik: {kimlikler:?}"
        );
    }
}

#[gpui::test]
fn ondalık_türünde_sayısal_adım_kurulur(bağlam: &mut TestAppContext) {
    let kimlikler = kimlikler(&bölümler(bağlam, TezgahDeğerKipi::Ondalık));
    assert!(
        kimlikler.contains(&"sayisal_adim"),
        "ondalık türünde s96 eksik: {kimlikler:?}"
    );
    assert!(
        !kimlikler.contains(&"saat_dilimi"),
        "ondalık türünde sz: {kimlikler:?}"
    );
}

#[gpui::test]
fn tarih_türünde_saat_dilimi_kurulur_adım_kurulmaz(bağlam: &mut TestAppContext) {
    let kimlikler = kimlikler(&bölümler(bağlam, TezgahDeğerKipi::Tarih));
    assert!(
        kimlikler.contains(&"saat_dilimi"),
        "tarih türünde sz eksik: {kimlikler:?}"
    );
    assert!(
        !kimlikler.contains(&"sayisal_adim"),
        "tarih türünde s96: {kimlikler:?}"
    );
}

#[gpui::test]
fn bölüm_kimlikleri_tekrarsızdır(bağlam: &mut TestAppContext) {
    // Kanonik tip bugün dokuz varyant taşıyor (Metin, Tamsayı, Ondalık,
    // ParaBirimi, Yüzde, Tarih, Saat, TarihSaat, Süre); tasarım dört aile
    // öngörüyor ve para/yüzdeyi `Ondalık` biçim profili sayıyor. Fark bir kod
    // göçü borcudur, bu test dokuzunu da tarar.
    for tür in [
        TezgahDeğerKipi::Metin,
        TezgahDeğerKipi::Tamsayı,
        TezgahDeğerKipi::Ondalık,
        TezgahDeğerKipi::ParaBirimi,
        TezgahDeğerKipi::Yüzde,
        TezgahDeğerKipi::Tarih,
        TezgahDeğerKipi::Saat,
        TezgahDeğerKipi::TarihSaat,
        TezgahDeğerKipi::Süre,
    ] {
        let kimlikler = kimlikler(&bölümler(bağlam, tür));
        let mut sıralı = kimlikler.clone();
        sıralı.sort_unstable();
        sıralı.dedup();
        assert_eq!(
            sıralı.len(),
            kimlikler.len(),
            "{tür:?} yinelenen kimlik: {kimlikler:?}"
        );
    }
}

#[gpui::test]
fn tam_genişlik_bölümleri_tasarımın_yerleşimine_uyar(bağlam: &mut TestAppContext) {
    let bölümler = bölümler(bağlam, TezgahDeğerKipi::Metin);
    // Tasarımın §5 yerleşiminde yalnız §7 ve §9 tam genişliktir; kalanlar
    // akışlara dağılır ve iki kolona bölünebilir.
    let tam: Vec<&str> = bölümler
        .iter()
        .filter(|(_, akış)| *akış == Akış::TamGenişlik)
        .map(|(k, _)| *k)
        .collect();
    assert_eq!(tam, vec!["deger_turu", "tur_tanimi_ve_maske"]);
}

#[gpui::test]
fn kapanan_eksenler_her_türde_bölüm_üretir(bağlam: &mut TestAppContext) {
    // `§9.7` sayaç ve `§22` gizli içerik sayısal türde **kapanır** ama
    // kurulamaz değildir: bölüm üretilir, içeriği pasif çizilir. Gizlemek
    // "eksen yok" derdi (harita §4).
    for tür in [
        TezgahDeğerKipi::Metin,
        TezgahDeğerKipi::Tamsayı,
        TezgahDeğerKipi::Ondalık,
        TezgahDeğerKipi::Tarih,
    ] {
        let kimlikler = kimlikler(&bölümler(bağlam, tür));
        for beklenen in [
            "on_ek_son_ek",
            "hacim_ve_sayac",
            "icerik_gorunurlugu",
            "secici_ve_erisim",
        ] {
            assert!(
                kimlikler.contains(&beklenen),
                "{tür:?} türünde {beklenen} eksik: {kimlikler:?}"
            );
        }
    }
}

#[gpui::test]
fn on_bir_bölümün_tamamı_metin_türünde_kurulur(bağlam: &mut TestAppContext) {
    // Saat dilimi ve sayısal adım metin türünde kurulamaz; kalan dokuz
    // bölüm her zaman vardır. Otomatik doldurma port kapısına bağlıdır.
    let kimlikler = kimlikler(&bölümler(bağlam, TezgahDeğerKipi::Metin));
    for beklenen in [
        "deger_turu",
        "tur_tanimi_ve_maske",
        "bicim_profili",
        "on_ek_son_ek",
        "hacim_ve_sayac",
        "icerik_gorunurlugu",
        "secici_ve_erisim",
        "odak_ve_kabul",
    ] {
        assert!(
            kimlikler.contains(&beklenen),
            "{beklenen} eksik: {kimlikler:?}"
        );
    }
}

#[gpui::test]
fn port_kapısı_ekseni_gizlemez(bağlam: &mut TestAppContext) {
    // `YÖN-006.ACC-005`: desteklenmeyen capability görünür ve dürüsttür.
    // Otomatik doldurma portu galeri testinde bağlı değildir; bölüm yine de
    // üretilir ve içeriği pasif + gerekçeli çizilir.
    let kimlikler = kimlikler(&bölümler(bağlam, TezgahDeğerKipi::Metin));
    assert!(
        kimlikler.contains(&"otomatik_doldurma"),
        "port yokken bölüm gizlenmiş: {kimlikler:?}"
    );
}

#[gpui::test]
fn kurulamayan_ile_kapanan_eksen_ayrı_mekanizmadır(bağlam: &mut TestAppContext) {
    // `§9` iki mekanizma: metin türünde sayısal adım **kurulamaz** (bölüm
    // hiç üretilmez), sayısal türde sayaç **kapanır** (bölüm üretilir,
    // içeriği pasif çizilir).
    let metin = kimlikler(&bölümler(bağlam, TezgahDeğerKipi::Metin));
    assert!(
        !metin.contains(&"sayisal_adim"),
        "kurulamayan eksen üretilmiş"
    );
    assert!(
        metin.contains(&"hacim_ve_sayac"),
        "kapanan eksenin bölümü yok"
    );

    let ondalık = kimlikler(&bölümler(bağlam, TezgahDeğerKipi::Ondalık));
    assert!(
        ondalık.contains(&"sayisal_adim"),
        "kurulabilen eksen üretilmemiş"
    );
    assert!(
        ondalık.contains(&"hacim_ve_sayac"),
        "kapanan eksen sayısal türde gizlenmiş: {ondalık:?}"
    );
}

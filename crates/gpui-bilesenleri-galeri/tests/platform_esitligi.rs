//! `YÖN-006.ACC-004` masaüstü ve WASM aynı çekirdeği kullanır.
//!
//! Sarmalayıcılar yalnız platform tanımı taşır: pencere açma, varlık kaynağı
//! ve tuş bağı kaydı. Davranış, yapılandırma veya çizim tanımı içermezler;
//! bu yüzden iki hedef aynı zemini kullanır ve aynı davranışı sergiler.

#![allow(non_ascii_idents)]

use gpui_bilesenleri_galeri::{GaleriHedefi, GaleriUygulaması};

const MASAÜSTÜ: &str = include_str!("../../gpui-bilesenleri-galeri-masaustu/src/main.rs");
const WASM: &str = include_str!("../../gpui-bilesenleri-galeri-wasm/src/lib.rs");

/// Sarmalayıcıda bulunmaması gereken davranış izleri.
///
/// Bunlardan biri sarmalayıcıya sızarsa iki hedef ayrışır ve galeri tek
/// kaynaklı olmaktan çıkar.
const DAVRANIŞ_İZLERİ: &[&str] = &[
    "div()",
    "GirişKutusu",
    "GirişYapılandırması",
    "GirişMaskesi",
    "YardımcıEylem",
    "Sabitİçerik",
    "SayaçYapılandırması",
    "UzunlukSınırı",
    "İçerikGörünürlüğü",
    "KeyBinding",
    "actions!",
    "TemaAnlıkGörüntüsü",
    "SimgeKataloğu",
    "on_click",
    "on_action",
    "impl Render",
];

/// Sarmalayıcıda bulunması beklenen platform tanımları.
const PLATFORM_TANIMLARI: &[(&str, &[&str])] = &[
    (
        "masaüstü",
        &["gpui_platform::application", "open_window", "WindowOptions"],
    ),
    (
        "wasm",
        &["gpui_web::WebPlatform", "open_window", "wasm_bindgen"],
    ),
];

#[test]
fn sarmalayicilar_davranis_tanimi_tasimaz() {
    for (ad, kaynak) in [("masaüstü", MASAÜSTÜ), ("wasm", WASM)] {
        for iz in DAVRANIŞ_İZLERİ {
            assert!(
                !kaynak.contains(iz),
                "{ad} sarmalayıcısı davranış tanımı taşıyor: {iz}"
            );
        }
    }
}

#[test]
fn sarmalayicilar_yalniz_platform_kurulumu_yapar() {
    for (ad, beklenenler) in PLATFORM_TANIMLARI {
        let kaynak = if *ad == "masaüstü" {
            MASAÜSTÜ
        } else {
            WASM
        };
        for beklenen in *beklenenler {
            assert!(
                kaynak.contains(beklenen),
                "{ad} sarmalayıcısında platform tanımı eksik: {beklenen}"
            );
        }
    }
}

#[test]
fn iki_hedef_ayni_bilesen_ve_varlik_kaydini_yapar() {
    // Tuş bağları ve simge varlıkları iki hedefte de aynı çağrıyla kurulur;
    // biri unutulursa o hedefte tuşlar veya simgeler çalışmaz.
    for (ad, kaynak) in [("masaüstü", MASAÜSTÜ), ("wasm", WASM)] {
        assert!(
            kaynak.contains("bileşen_tuş_bağlarını_kur"),
            "{ad} tuş bağlarını kurmuyor"
        );
        assert!(
            kaynak.contains("GaleriVarlıkKaynağı"),
            "{ad} simge varlık kaynağını kurmuyor"
        );
    }
}

#[test]
fn iki_hedef_ayni_katalogu_ve_bilgi_mimarisini_acar() {
    let masaüstü = GaleriUygulaması::hedef(GaleriHedefi::Masaüstü);
    let wasm = GaleriUygulaması::hedef(GaleriHedefi::Wasm);

    // Hedef rozeti dışında model birebir aynıdır.
    assert_eq!(masaüstü.model.katalog.len(), wasm.model.katalog.len());
    let masaüstü_kimlikleri: Vec<_> = masaüstü
        .model
        .katalog
        .iter()
        .map(|kayıt| kayıt.sözleşme.to_string())
        .collect();
    let wasm_kimlikleri: Vec<_> = wasm
        .model
        .katalog
        .iter()
        .map(|kayıt| kayıt.sözleşme.to_string())
        .collect();
    assert_eq!(masaüstü_kimlikleri, wasm_kimlikleri);
    assert_ne!(masaüstü.model.hedef, wasm.model.hedef);
}

#[test]
fn sarmalayicilar_kucuk_kalir() {
    // Sarmalayıcı büyümesi davranışın oraya kaymaya başladığının ilk
    // işaretidir; sınır bilinçli olarak dardır.
    for (ad, kaynak) in [("masaüstü", MASAÜSTÜ), ("wasm", WASM)] {
        let satır = kaynak.lines().count();
        assert!(
            satır <= 80,
            "{ad} sarmalayıcısı {satır} satır; platform kurulumu için fazla"
        );
    }
}

//! WASM galeri başlatıcısı.
//!
//! Bu sandık tek iş parçacıklı GPUI web profilini ve `Auto` grafik backend'ini
//! (WebGPU, yoksa WebGL2) kullanır. Tarayıcı konağı bu işlevi `wasm-bindgen`
//! ürün katmanı üzerinden çağırır. Bağlayıcı yalnız bu platform
//! başlatıcısında kalır ve ortak galeri API'sine sızmaz.

#![allow(non_ascii_idents)]

#[path = "platform.rs"]
pub mod platform;
// `ORT-018` ölçüm yüzeyi ayrı dosyadadır: sarmalayıcı yalnız platform
// kurulumu yapar, ölçüm koşumu oraya sızmaz.
#[path = "ölçüm.rs"]
pub mod ölçüm;

// Tek kapı: tarayıcı dışı derlemede bu sandığın gövdesi yoktur.
#[cfg(target_family = "wasm")]
use {
    gpui::{App, AppContext, WindowOptions},
    gpui_bilesenleri_galeri::{
        GaleriUygulaması, GaleriVarlıkKaynağı, PlatformPortları, bileşen_tuş_bağlarını_kur,
        galeri_yazı_tiplerini_kur,
    },
    platform::galeri_durumu,
    std::{rc::Rc, sync::Arc},
    wasm_bindgen::prelude::*,
};

/// Web platformunu başlatır ve ortak galeri çekirdeğini açar.
#[cfg(target_family = "wasm")]
#[wasm_bindgen(js_name = baslat)]
pub fn başlat() {
    gpui_web::init_logging();
    galeri_durumu("grafik-bekleniyor", "");
    // `YÖN-006 §7.1`: backend `Auto`dur; seçileni `init_logging` konsola yazar.
    let platform = Rc::new(gpui_web::WebPlatform::new_with_backend(
        false,
        gpui_web::WebBackendPreference::Auto,
    ));
    let http_istemcisi = Arc::new(platform.fetch_http_client());
    let uygulama = gpui::Application::with_platform(platform)
        .with_http_client(http_istemcisi)
        .with_assets(GaleriVarlıkKaynağı)
        .run_embedded(|bağlam: &mut App| {
            galeri_durumu("gpui-baslatildi", "");
            // `BİL-010` tuş yolları tarayıcıda da eylem sistemi üzerinden gelir.
            bileşen_tuş_bağlarını_kur(bağlam);
            // Tarayıcı sistem yazı tiplerini vermez; kitaplık yüzleri yalnız
            // bu kayıtla çözülür.
            if let Err(hata) = galeri_yazı_tiplerini_kur(bağlam) {
                galeri_durumu("yazi-tipi-hatasi", &hata.to_string());
            }
            let pencere = bağlam.open_window(WindowOptions::default(), |_, bağlam| {
                bağlam.new(|bağlam| {
                    let mut uygulama = GaleriUygulaması::wasm();
                    // Saat dilimi kimlik kapısı uygulama kökünün motorudur.
                    let dilim = platform::TarayıcıSaatDilimi::yeni(uygulama.metin_motoru());
                    uygulama.platform_portlarını_kur(
                        PlatformPortları {
                            saat_dilimi: Some(Arc::new(dilim)),
                            imleç: Some(Arc::new(platform::TarayıcıİmleciTercihi)),
                            otomatik_doldurma: Some(Arc::new(platform::TarayıcıOtomatikDoldurma)),
                        },
                        bağlam,
                    );
                    uygulama
                })
            });
            match pencere {
                Ok(tutamaç) => {
                    ölçüm::pencereyi_sakla(tutamaç);
                    galeri_durumu("pencere-acildi", "");
                }
                Err(hata) => galeri_durumu("pencere-hatasi", &hata.to_string()),
            }
        });
    ölçüm::uygulamayı_sakla(uygulama);
}

/// Yerel derlemede bu sandığın tarayıcı girişi olmadığını bildirir.
#[cfg(not(target_family = "wasm"))]
pub const fn yalnız_wasm_hedefidir() -> bool {
    true
}

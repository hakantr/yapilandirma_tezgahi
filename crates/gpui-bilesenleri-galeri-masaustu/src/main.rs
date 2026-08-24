//! Masaüstü galeri başlatıcısı.
//!
//! Bu sarmalayıcı yalnız platform kurulumu yapar: varlık kaynağı, tuş bağı
//! kaydı ve pencere açma. Davranış, yapılandırma veya çizim tanımı taşımaz;
//! bu yüzden masaüstü ile WASM aynı galeri çekirdeğini aynı şekilde açar.

#![allow(non_ascii_idents)]

#[path = "platform.rs"]
mod platform;

use std::sync::Arc;

use gpui::{App, AppContext, Bounds, WindowBounds, WindowOptions, px, size};
use gpui_bilesenleri_galeri::{
    GaleriUygulaması, GaleriVarlıkKaynağı, PlatformPortları, bileşen_tuş_bağlarını_kur,
    galeri_yazı_tiplerini_kur,
};

fn main() {
    let dar_kabul_koşumu = std::env::args().any(|argüman| argüman == "--dar");
    let geniş_kabul_koşumu = std::env::args().any(|argüman| argüman == "--geniş");
    gpui_platform::application()
        .with_assets(GaleriVarlıkKaynağı)
        .run(move |bağlam: &mut App| {
            // Tuş yolları GPUI eylem sistemine bağlıdır; bu kayıt olmadan
            // platform Backspace/ok tuşlarını teslim edecek hedef bulamaz.
            bileşen_tuş_bağlarını_kur(bağlam);
            // Kitaplık yüzleri iki hedefte de kayıtlı olmalı: masaüstü yalnız
            // işletim sisteminde kurulu aileleri görür ve bunlar makineden
            // makineye değişir.
            if let Err(hata) = galeri_yazı_tiplerini_kur(bağlam) {
                eprintln!("galeri yazı tipleri kaydedilemedi: {hata}");
            }

            let içerik_boyutu = if dar_kabul_koşumu {
                size(px(760.), px(640.))
            } else if geniş_kabul_koşumu {
                // `§4` geniş kabul koşumu: iki kolon eşiği (`892px` @%100)
                // artı sol gezinme şeridi. Varsayılan `960` pencerede tezgâh
                // tek kolona düşer ve iki kolonlu yerleşim hiç sınanmaz.
                size(px(1600.), px(1000.))
            } else {
                size(px(960.), px(640.))
            };
            let sınırlar = Bounds::centered(None, içerik_boyutu, bağlam);
            let açıldı = bağlam.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(sınırlar)),
                    ..Default::default()
                },
                |_, bağlam| {
                    bağlam.new(|_| {
                        let mut uygulama = GaleriUygulaması::yeni();
                        // Sarmalayıcı yalnız bildirimi kurar; öncelik sırası
                        // ve düşme kuralı çekirdektedir.
                        uygulama.platform_portlarını_kur(PlatformPortları {
                            saat_dilimi: Some(Arc::new(platform::SistemSaatDilimi)),
                            imleç: Some(Arc::new(platform::SistemİmleciTercihi)),
                            otomatik_doldurma: Some(Arc::new(platform::SistemOtomatikDoldurma)),
                        });
                        uygulama
                    })
                },
            );

            if açıldı.is_ok() {
                bağlam.activate(true);
            }
        });
}

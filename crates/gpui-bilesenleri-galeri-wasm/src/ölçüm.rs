//! `ORT-018` `bil-010.input.commit` ölçüm yüzeyi.
//!
//! Ölçüm gerçek galeri penceresinde, tezgâhın yaşayan alanı üzerinde koşar.
//! Ayrı bir taklit alanda ölçmek sayıyı ürünle karşılaştırılamaz kılardı.
//!
//! Bu dosya sarmalayıcıdan ayrıdır: `lib.rs` yalnız platform kurulumu yapar
//! ve ölçüm koşumu oraya sızmaz.

#[cfg(target_family = "wasm")]
use {
    gpui::{ApplicationHandle, WindowHandle},
    gpui_bilesenleri_galeri::GaleriUygulaması,
    std::cell::RefCell,
    wasm_bindgen::prelude::*,
};

#[cfg(target_family = "wasm")]
thread_local! {
    /// Web platformu dış olay döngüsünü kullandığı için `run_embedded`
    /// hemen döner. Tutamaç saklanmazsa uygulama grafik backend'i
    /// hazırlanmadan düşer.
    static UYGULAMA: RefCell<Option<ApplicationHandle>> = const { RefCell::new(None) };
    /// Ölçüm pencere kökünden koşar; tutamaç açılışta saklanır.
    static PENCERE: RefCell<Option<WindowHandle<GaleriUygulaması>>> = const { RefCell::new(None) };
}

/// Uygulama tutamacını saklar; düşerse grafik backend'i hazırlanmadan uygulama
/// biter.
#[cfg(target_family = "wasm")]
pub fn uygulamayı_sakla(uygulama: ApplicationHandle) {
    UYGULAMA.with(|saklı| {
        saklı.replace(Some(uygulama));
    });
}

/// Pencere tutamacını saklar; ölçüm kökten koşar.
#[cfg(target_family = "wasm")]
pub fn pencereyi_sakla(pencere: WindowHandle<GaleriUygulaması>) {
    PENCERE.with(|saklı| {
        saklı.replace(Some(pencere));
    });
}

/// `ORT-018` `bil-010.input.commit` ölçümünü tarayıcıda koşturur.
///
/// Ölçüm gerçek galeri penceresinde, tezgâhın yaşayan alanı üzerinde
/// koşar. Ayrı bir taklit alanda ölçmek sayıyı ürünle
/// karşılaştırılamaz kılardı. Süreler milisaniye olarak, virgülle
/// ayrılmış döner; konak istatistiği kendi hesaplar.
#[cfg(target_family = "wasm")]
#[wasm_bindgen(js_name = olcumKostur)]
pub fn ölçüm_koştur(ısınma: u32, tekrar: u32) -> String {
    let Some(pencere) = PENCERE.with(|saklı| saklı.borrow().clone()) else {
        return String::new();
    };
    UYGULAMA.with(|tutamaç| {
        let saklı = tutamaç.borrow();
        let Some(uygulama) = saklı.as_ref() else {
            return String::new();
        };
        uygulama.update(|bağlam| {
            pencere
                .update(bağlam, |kök, pencere, bağlam| {
                    kök.ölçüm_koştur(ısınma, tekrar, pencere, bağlam)
                        .into_iter()
                        .map(|süre| format!("{süre:.6}"))
                        .collect::<Vec<_>>()
                        .join(",")
                })
                .unwrap_or_default()
        })
    })
}

/// `ORT-018` toplu ölçüm: `tekrar` kabulün toplam süresi (ms).
#[cfg(target_family = "wasm")]
#[wasm_bindgen(js_name = olcumTopluMs)]
pub fn ölçüm_toplu_ms(ısınma: u32, tekrar: u32) -> f64 {
    let Some(pencere) = PENCERE.with(|saklı| saklı.borrow().clone()) else {
        return -1.0;
    };
    UYGULAMA.with(|tutamaç| {
        let saklı = tutamaç.borrow();
        let Some(uygulama) = saklı.as_ref() else {
            return -1.0;
        };
        uygulama.update(|bağlam| {
            pencere
                .update(bağlam, |kök, pencere, bağlam| {
                    kök.ölçüm_toplu_ms(ısınma, tekrar, pencere, bağlam)
                })
                .unwrap_or(-1.0)
        })
    })
}

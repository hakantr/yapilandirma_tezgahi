//! Gerçek pencerede girdi gecikmesi ve kare maliyeti ölçümü.
//!
//! Headless kare ölçümü (`gpui-bilesenleri-galeri/tests/kare_olcumu.rs`)
//! yalnız `render` gövdelerinin işini görür; yerleşim, prepaint, paint ve
//! platform katmanı (shaping, rasterizasyon, sahne kodlama) orada yoktur.
//! Bu modül eksik kalan yarıyı ölçer ve ancak `olcum` özelliğiyle derlenir.
//!
//! **Ölçüm penceresi.** Sayaç, pencere açılışıyla değil ilk gerçek metin
//! **düzenlemesiyle** başlar (`düzenleme_sayısı`). Platformun girdi
//! histogramı her geçersizleştiren olayı sayar — alana yapılan bir fare
//! tıklaması da oraya girer — oysa ölçülmek istenen yazma evresidir.
//! Pencerenin başında histogramların anlık görüntüsü alınır ve sonda fark
//! hesaplanır; böylece açılış kareleri ile odaklanma sırasındaki olaylar
//! sonuca karışmaz.
//!
//! **Ne ölçülüyor, ne ölçülmüyor.**
//!
//! - `girdi→draw sonu`: platform olayının gelişinden, o olayın yol açtığı
//!   karenin platform `draw` çağrısının **tamamlanmasına** kadar. Bu,
//!   pikselin ekranda değiştiği an **değildir**; sunum kuyruğu ve panel
//!   gecikmesi bunun dışındadır.
//! - `render gövdeleri` / `render sonrası draw aşamaları`: bu ayrım
//!   **sahiplik değil, aşamadır**. İkinci dilim "GPUI'nin işi" sanılmamalı
//!   — içinde tezgâhın ürettiği ağacın yerleşimi, prepaint'i, paint'i ve
//!   metin shaping'i vardır; sağ kolon önbelleği tam da o aşamaların
//!   aralıklarını (`reuse_prepaint` / `reuse_paint`) yeniden kullanır.
//! - `mid_draw_events_dropped` büyükse gecikme rakamı eksik okunmalıdır.

use std::time::Duration;

use gpui::{App, AppContext as _, WindowHandle};
use hdrhistogram::Histogram;

/// Histogramı okunur tek satıra indirger.
///
/// Örneklem sayısı da yazılır: iki örnekle alınmış bir p95 sayı değil
/// gürültüdür ve okuyan bunu görmelidir. Ortalama da basılır çünkü aşama
/// ayrımı ortalamalar üzerinden yapılır ve p50 ile karıştırılmamalıdır.
fn özet(ad: &str, histogram: &Histogram<u64>, bölen: f64, birim: &str) -> String {
    if histogram.is_empty() {
        return format!("{ad:<24} örnek yok");
    }
    let çevir = |değer: u64| değer as f64 / bölen;
    format!(
        "{ad:<24} n={:<5} p50 {:7.3} {birim} · p95 {:7.3} {birim} · \
         p99 {:7.3} {birim} · ort {:7.3} {birim}",
        histogram.len(),
        çevir(histogram.value_at_quantile(0.50)),
        çevir(histogram.value_at_quantile(0.95)),
        çevir(histogram.value_at_quantile(0.99)),
        histogram.mean() / bölen,
    )
}

/// Sondaki histogramdan ölçüm penceresine düşen payı ayıklar.
///
/// Çıkarma başarısız olursa ham histogram döner ve rapor bunu **yazar**:
/// sessizce yanlış sayı üretmek, eksik sayı üretmekten kötüdür.
fn pencere_payı(son: &Histogram<u64>, baş: &Histogram<u64>) -> (Histogram<u64>, bool) {
    let mut fark = son.clone();
    match fark.subtract(baş) {
        Ok(()) => (fark, true),
        Err(_) => (son.clone(), false),
    }
}

/// `--olcum <saniye>` verildiyse ölçümü kurar.
///
/// Argüman ayrıştırma da burada: sarmalayıcı (`main.rs`) yalnız platform
/// kurulumu taşır ve bir bekçi onu 80 satırın altında tutar.
pub fn ölçümü_kur<T: 'static>(pencere: &WindowHandle<T>, bağlam: &mut App) {
    let Some(saniye) = std::env::args()
        .skip_while(|argüman| argüman != "--olcum")
        .nth(1)
        .and_then(|değer| değer.parse().ok())
    else {
        return;
    };
    ölçümü_planla((*pencere).into(), saniye, bağlam);
}

/// Ölçüm penceresi dolunca raporu basar ve uygulamadan çıkar.
fn ölçümü_planla(pencere: gpui::AnyWindowHandle, saniye: u64, bağlam: &mut App) {
    eprintln!(
        "ölçüm modu: alana tıklayıp **yazmaya başlayın** — sayaç ilk metin \
         düzenlemesiyle başlar ve {saniye} sn sürer, sonra rapor basılır."
    );
    bağlam
        .spawn(async move |bağlam| {
            // Kapı: ilk gerçek düzenleme (fare tıklaması değil).
            // En çok iki dakika beklenir; sonra ölçüm yine de başlar ve
            // rapor kendi uyarısını basar.
            for _ in 0..600 {
                if gpui_bilesenleri_galeri::düzenleme_sayısı() > 0 {
                    break;
                }
                bağlam
                    .background_executor()
                    .timer(Duration::from_millis(200))
                    .await;
            }
            // Pencere başlangıcı: histogramların tam anlık görüntüsü.
            let başlangıç = bağlam
                .update_window(pencere, |_, pencere, _| {
                    gpui_bilesenleri_galeri::render_sıfırla();
                    (
                        pencere.input_latency_snapshot(),
                        pencere.frame_duration_snapshot(),
                    )
                })
                .ok();
            bağlam
                .background_executor()
                .timer(Duration::from_secs(saniye))
                .await;
            bağlam
                .update_window(pencere, |_, pencere, _| {
                    raporla(pencere, saniye, başlangıç);
                })
                .ok();
            bağlam.update(|bağlam| bağlam.quit());
        })
        .detach();
}

type Başlangıç = (
    gpui::profiler::InputLatencySnapshot,
    gpui::profiler::FrameDurationSnapshot,
);

fn raporla(pencere: &gpui::Window, saniye: u64, başlangıç: Option<Başlangıç>) {
    const MS: f64 = 1_000_000.;
    let girdi = pencere.input_latency_snapshot();
    let kare = pencere.frame_duration_snapshot();
    let Some((baş_girdi, baş_kare)) = başlangıç else {
        eprintln!("ölçüm penceresi kurulamadı; rapor atlandı");
        return;
    };

    let (gecikme, g_tam) = pencere_payı(&girdi.latency_histogram, &baş_girdi.latency_histogram);
    let (çizim, ç_tam) = pencere_payı(
        &kare.draw_duration_histogram,
        &baş_kare.draw_duration_histogram,
    );
    let (sunum, _) = pencere_payı(
        &kare.present_interval_histogram,
        &baş_kare.present_interval_histogram,
    );
    let (olaylar, _) = pencere_payı(
        &girdi.events_per_frame_histogram,
        &baş_girdi.events_per_frame_histogram,
    );

    println!("\n— gerçek pencere ölçümü · {saniye} sn (yalnız ölçüm penceresi) —");
    println!(
        "ortam                    {:.0}×{:.0} px · ölçek {:.1}× · \
         erişilebilirlik {} · derleme {}",
        f32::from(pencere.viewport_size().width),
        f32::from(pencere.viewport_size().height),
        pencere.scale_factor(),
        if pencere.is_a11y_active() {
            "etkin"
        } else {
            "kapalı"
        },
        if cfg!(debug_assertions) {
            "hata ayıklama"
        } else {
            "optimize"
        },
    );
    println!("{}", özet("girdi→draw sonu", &gecikme, MS, "ms"));
    println!("{}", özet("çizim (draw)", &çizim, MS, "ms"));
    println!("{}", özet("sunum aralığı", &sunum, MS, "ms"));
    println!("{}", özet("kare başına olay", &olaylar, 1., "olay"));
    println!(
        "çizim ortasında düşen olay: {}",
        girdi
            .mid_draw_events_dropped
            .saturating_sub(baş_girdi.mid_draw_events_dropped)
    );

    // Aşama ayrımı — **sahiplik değil**. İkinci dilim GPUI'nin malı
    // değildir: içinde tezgâhın ürettiği ağacın yerleşimi, prepaint'i,
    // paint'i ve metin shaping'i vardır. Karşılaştırma ortalamalar
    // üzerindendir; p50 ile karıştırılmamalıdır.
    if !çizim.is_empty() {
        let render_ort =
            gpui_bilesenleri_galeri::render_toplam_ns() as f64 / çizim.len() as f64 / MS;
        let çizim_ort = çizim.mean() / MS;
        println!(
            "aşama (ortalama)         render gövdeleri {render_ort:6.3} ms · \
             render sonrası draw aşamaları {:6.3} ms · toplam draw {çizim_ort:6.3} ms",
            (çizim_ort - render_ort).max(0.),
        );
    }
    if !g_tam || !ç_tam {
        eprintln!(
            "uyarı: histogram farkı alınamadı; sayılar pencere başlangıcından \
             veri taşıyor olabilir."
        );
    }
    if gecikme.is_empty() {
        eprintln!("uyarı: ölçüm penceresinde girdi örneği yok — yazılmadı mı?");
    }
}

//! Gerçek pencerede `input-to-present` ölçümü.
//!
//! Headless kare ölçümü (`gpui-bilesenleri-galeri/tests/kare_olcumu.rs`)
//! yalnız CPU çizim işini görür: sunum, vsync ve giriş kuyruğu orada yoktur.
//! Bu modül eksik kalan yarıyı ölçer ve ancak `olcum` özelliğiyle derlenir.
//!
//! Ölçülen değerler GPUI'nin kendi pencere profilcisinden gelir; tezgâh
//! kendi zamanlayıcısını kurmaz:
//!
//! - **Girdi gecikmesi** (`input_latency_snapshot`): platform olayının
//!   geldiği andan, o olayın yol açtığı karenin çizildiği ana kadar.
//!   `mid_draw_events_dropped` çizim sürerken gelip ölçüme giremeyen
//!   olayları sayar — büyükse gecikme rakamı eksik okunmalıdır.
//! - **Sunum aralığı** (`frame_duration_snapshot`): art arda sunulan iki
//!   kare arasındaki süre. FPS ve düşen kare buradan çıkar; yalnız pencere
//!   canlıyken örneklenir.
//! - **Çizim süresi**: headless koşumun ölçtüğü işin gerçek penceredeki
//!   karşılığı; iki ölçüm ancak bu sütun üzerinden karşılaştırılabilir.
//!
//! Koşum penceresi boyunca **gerçekten yazmak gerekir**: ölçülen şey
//! kullanıcı girdisidir, uygulamanın kendi kendine çizmesi değil.

use std::time::Duration;

use gpui::{App, AppContext as _, WindowHandle};
use hdrhistogram::Histogram;

/// Histogramı okunur tek satıra indirger.
///
/// Örneklem sayısı da yazılır: iki örnekle alınmış bir p95 sayı değil
/// gürültüdür ve okuyan bunu görmelidir.
fn özet(ad: &str, histogram: &Histogram<u64>, bölen: f64, birim: &str) -> String {
    if histogram.is_empty() {
        return format!("{ad:<22} örnek yok");
    }
    let çevir = |değer: u64| değer as f64 / bölen;
    format!(
        "{ad:<22} n={:<5} p50 {:7.3} {birim} · p95 {:7.3} {birim} · \
         p99 {:7.3} {birim} · en çok {:7.3} {birim}",
        histogram.len(),
        çevir(histogram.value_at_quantile(0.50)),
        çevir(histogram.value_at_quantile(0.95)),
        çevir(histogram.value_at_quantile(0.99)),
        çevir(histogram.max()),
    )
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
        "ölçüm modu: alana tıklayıp yazmaya başlayın — sayaç **ilk tuş \
         vuruşuyla** başlar ve {saniye} sn sürer, sonra rapor basılır."
    );
    bağlam
        .spawn(async move |bağlam| {
            // Sayaç **ilk tuş vuruşuyla** başlar, pencere açılışıyla
            // değil. Ardışık iki koşum yapılırken ikincisi kaçırılıp boş
            // rapor üretmişti; kullanıcının pencereyi bulup odaklanması
            // için geçen süre ölçümü yemiyor artık. En çok iki dakika
            // beklenir, sonra ölçüm yine de başlar (rapor kendi uyarısını
            // basar).
            for _ in 0..600 {
                let girdi_var = bağlam
                    .update_window(pencere, |_, pencere, _| {
                        !pencere
                            .input_latency_snapshot()
                            .latency_histogram
                            .is_empty()
                    })
                    .unwrap_or(false);
                if girdi_var {
                    break;
                }
                bağlam
                    .background_executor()
                    .timer(Duration::from_millis(200))
                    .await;
            }
            // Ölçüm penceresi açılış karelerini dışarıda bırakır: ilk
            // kareler font yükler ve bütün ağacı ilk kez kurar, yani
            // sürekli kullanımı temsil etmez. Pencere başındaki kare
            // sayısı ve sıfırlanan sayaç, ayrıştırmayı yalnız bu
            // penceredeki karelere dayandırır.
            let başlangıç_karesi = bağlam
                .update_window(pencere, |_, pencere, _| {
                    gpui_bilesenleri_galeri::render_sıfırla();
                    pencere.frame_duration_snapshot().draw_duration_histogram.len()
                })
                .unwrap_or(0);
            bağlam
                .background_executor()
                .timer(Duration::from_secs(saniye))
                .await;
            let _ = bağlam.update_window(pencere, |_, pencere, _| {
                let girdi = pencere.input_latency_snapshot();
                let kare = pencere.frame_duration_snapshot();
                // Nanosaniyeden milisaniyeye; sunum aralığı da ms.
                const MS: f64 = 1_000_000.;
                // Teşhis: headless koşumla arasındaki farkı ayrıştırmak
                // için ortam. Ölçek çarpanı 2 ise piksel işi dört katıdır;
                // erişilebilirlik ağacı etkinse her kare bir de AX
                // güncellemesi taşır ve ikisi de headless koşumda yoktur.
                let boyut = pencere.viewport_size();
                println!("\n— gerçek pencere ölçümü · {saniye} sn —");
                println!(
                    "ortam                  {:.0}×{:.0} px · ölçek {:.1}× · \
                     erişilebilirlik {} · derleme {}",
                    f32::from(boyut.width),
                    f32::from(boyut.height),
                    pencere.scale_factor(),
                    if pencere.is_a11y_active() { "etkin" } else { "kapalı" },
                    if cfg!(debug_assertions) { "hata ayıklama" } else { "optimize" },
                );
                println!("{}", özet("girdi→kare", &girdi.latency_histogram, MS, "ms"));
                println!(
                    "{}",
                    özet("çizim süresi", &kare.draw_duration_histogram, MS, "ms")
                );
                println!(
                    "{}",
                    özet("sunum aralığı", &kare.present_interval_histogram, MS, "ms")
                );
                println!(
                    "{}",
                    özet(
                        "kare başına olay",
                        &girdi.events_per_frame_histogram,
                        1.,
                        "olay"
                    )
                );
                println!(
                    "çizim ortasında düşen olay: {}",
                    girdi.mid_draw_events_dropped
                );
                // `draw` ayrıştırması: tezgâhın kendi `render` gövdeleri
                // (element ağacının kurulumu) toplam çizimin ne kadarı?
                // Kalan pay GPUI'nin yerleşim/prepaint/paint işi ile
                // platform katmanına (metin shaping, glif rasterizasyonu,
                // sahne kodlama) aittir.
                let pencere_kareleri = kare
                    .draw_duration_histogram
                    .len()
                    .saturating_sub(başlangıç_karesi);
                if pencere_kareleri > 0 {
                    let render_ms = gpui_bilesenleri_galeri::render_toplam_ns() as f64
                        / pencere_kareleri as f64
                        / MS;
                    // Pencere içi ortalama çizim: toplam işten açılış
                    // karelerinin payı düşülür.
                    let toplam = kare.draw_duration_histogram.mean()
                        * kare.draw_duration_histogram.len() as f64;
                    let çizim_ms = (toplam / pencere_kareleri as f64) / MS;
                    println!(
                        "ayrıştırma             {pencere_kareleri} kare · kare başına \
                         tezgâh render {render_ms:6.3} ms · çizim ≤{çizim_ms:6.3} ms · \
                         tezgâh payı ≥%{:.0}",
                        if çizim_ms > 0. { render_ms / çizim_ms * 100. } else { 0. },
                    );
                }
                if girdi.latency_histogram.is_empty() {
                    eprintln!(
                        "uyarı: hiç girdi örneği yok — pencereye yazılmadıysa bu \
                         rapor gecikme hakkında bir şey söylemez."
                    );
                }
            });
            bağlam.update(|bağlam| bağlam.quit());
        })
        .detach();
}

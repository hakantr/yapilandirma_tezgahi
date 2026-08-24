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
//!
//! **İki kip.** `--olcum <sn>` mutlak sayıları verir (gecikme, kare
//! maliyeti, aşama ayrımı). `--olcum-ab <sn>` ise sağ kolon önbelleğinin
//! kazancını ölçer: önbelleği koşum içinde beşer saniyelik ABBA fazlarıyla
//! açıp kapatır ve iki kova tutar. İkinci kip, iki ayrı binary'yi arka
//! arkaya koşturmanın işe yaramamasından doğdu — koşumlar arası gürültü
//! (~4,4 ms) aranan etkiden (~0,9 ms) büyüktü.

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

/// `--olcum <saniye>` ya da `--olcum-ab <saniye>` verildiyse ölçümü kurar.
///
/// Argüman ayrıştırma da burada: sarmalayıcı (`main.rs`) yalnız platform
/// kurulumu taşır ve bir bekçi onu 80 satırın altında tutar.
pub fn ölçümü_kur<T: 'static>(pencere: &WindowHandle<T>, bağlam: &mut App) {
    if let Some(saniye) = argüman_saniyesi("--olcum-ab") {
        dönüşümlü_ölçümü_planla((*pencere).into(), saniye, bağlam);
        return;
    }
    if let Some(saniye) = argüman_saniyesi("--olcum") {
        ölçümü_planla((*pencere).into(), saniye, bağlam);
    }
}

fn argüman_saniyesi(ad: &str) -> Option<u64> {
    std::env::args()
        .skip_while(|argüman| argüman != ad)
        .nth(1)
        .and_then(|değer| değer.parse().ok())
}

/// Ölçüm kapısı: ilk gerçek metin düzenlemesi.
///
/// Fare tıklaması da platformun girdi histogramını doldurur; ölçülmek
/// istenen ise yazma evresidir. En çok iki dakika beklenir, sonra ölçüm
/// yine de başlar — ama `false` döner ve rapor bunu **yazar**. Sessizce
/// yazılmamış bir koşumu ölçmek, hiç ölçmemekten kötüdür: sayılar dolu
/// görünür ama neyi ölçtükleri belirsizdir.
async fn kapıyı_bekle(bağlam: &mut gpui::AsyncApp) -> bool {
    for _ in 0..600 {
        if gpui_bilesenleri_galeri::düzenleme_sayısı() > 0 {
            return true;
        }
        bağlam
            .background_executor()
            .timer(Duration::from_millis(200))
            .await;
    }
    false
}

/// Ölçüm penceresi dolunca raporu basar ve uygulamadan çıkar.
fn ölçümü_planla(pencere: gpui::AnyWindowHandle, saniye: u64, bağlam: &mut App) {
    eprintln!(
        "ölçüm modu: alana tıklayıp **yazmaya başlayın** — sayaç ilk metin \
         düzenlemesiyle başlar ve {saniye} sn sürer, sonra rapor basılır."
    );
    bağlam
        .spawn(async move |bağlam| {
            kapıyı_bekle(bağlam).await;
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

/// Bir fazın uzunluğu.
///
/// Beş saniye, tipik yazma temposunda faz başına birkaç yüz kare demek.
/// Daha kısa fazlarda geçiş karesinin payı büyür; daha uzunda makine
/// durumu fazın içinde kayar ve dönüşümlü tasarımın bütün amacı budur.
const FAZ_SN: u64 = 5;
/// Geçiş karesi beklenirken iki yoklama arasındaki süre.
///
/// Toplam bekleme bunun 60 katıyla sınırlıdır (3 sn): kare hiç gelmezse
/// faz yine de ölçülür, yalnız geçiş karesini dışarıda bırakma güvencesi
/// kalmaz.
const OTURMA_MS: u64 = 50;

/// Aynı koşum içinde önbellekli/önbelleksiz fazları sırayla ölçer.
///
/// İki ayrı binary'yi arka arkaya koşturmak işe yaramadı: aynı binary'nin
/// iki koşumu arasındaki fark (~4,4 ms), aranan etkiden (~0,9 ms) beş kat
/// büyük çıktı. Baskın değişken derleme değil, koşum — termal durum, arka
/// plan yükü ve elle yazma temposu. Fazları tek süreç içinde dönüşümlü
/// koşturmak bu üçünü de iki kovaya eşit dağıtır.
fn dönüşümlü_ölçümü_planla(pencere: gpui::AnyWindowHandle, saniye: u64, bağlam: &mut App) {
    let faz_sayısı = (saniye / FAZ_SN).max(4);
    eprintln!(
        "dönüşümlü ölçüm: alana tıklayıp **durmadan yazın** — sayaç ilk metin \
         düzenlemesiyle başlar, {faz_sayısı} faz × {FAZ_SN} sn sürer (ABBA sırası). \
         Önbellek koşum içinde açılıp kapanır; tempoyu sabit tutmaya çalışın."
    );
    bağlam
        .spawn(async move |bağlam| {
            let kapı_açıldı = kapıyı_bekle(bağlam).await;
            let mut kova: [Option<Histogram<u64>>; 2] = [None, None];
            let mut render_ns = [0u64; 2];
            let mut eksik_faz = false;
            for sıra in 0..faz_sayısı {
                // ABBA: doğrusal kayma (ısınma, termal kısma, arka plan
                // yükü) iki kovaya da eşit dağılsın diye. Düz AB sırası
                // kaymanın tamamını ikinci hâle yükler.
                let önbellekli = matches!(sıra % 4, 0 | 3);
                let indis = usize::from(!önbellekli);
                match faz_ölç(bağlam, pencere, önbellekli).await {
                    Some((fark, ns)) => {
                        render_ns[indis] = render_ns[indis].saturating_add(ns);
                        match &mut kova[indis] {
                            Some(birikim) => {
                                if birikim.add(&fark).is_err() {
                                    eksik_faz = true;
                                }
                            }
                            None => kova[indis] = Some(fark),
                        }
                    }
                    None => eksik_faz = true,
                }
            }
            bağlam
                .update_window(pencere, |_, pencere, _| {
                    dönüşümlü_raporla(
                        pencere,
                        faz_sayısı,
                        &kova,
                        &render_ns,
                        Kuşku {
                            eksik_faz,
                            kapı_açılmadı: !kapı_açıldı,
                        },
                    );
                })
                .ok();
            bağlam.update(|bağlam| bağlam.quit());
        })
        .detach();
}

/// Tek fazı ölçer: bayrağı kur, otur, histogram farkını al.
///
/// `None` dönmesi fazın **sayılmadığı** anlamına gelir; çağıran bunu
/// rapora uyarı olarak yazar. Sessizce eksik kova üretmek, eksik olduğunu
/// söylemekten kötüdür.
async fn faz_ölç(
    bağlam: &mut gpui::AsyncApp,
    pencere: gpui::AnyWindowHandle,
    önbellekli: bool,
) -> Option<(Histogram<u64>, u64)> {
    let geçiş_öncesi = bağlam
        .update_window(pencere, |_, pencere, _| {
            gpui_bilesenleri_galeri::önbelleği_ayarla(önbellekli);
            pencere.refresh();
            pencere.frame_duration_snapshot().draw_duration_histogram.len()
        })
        .ok()?;
    // Geçiş karesi zorunlu ıskadır: bayrak değişince ağaç yeniden kurulur
    // ve önbellekli hâle geçişte önbellek de o karede dolar. **Süreyle**
    // beklemek yetmez — yazma seyrekse o kare sabit bekleme bittikten
    // sonra gelir ve doğrudan ölçüme sızar; üstelik yalnız A kovasına,
    // çünkü pahalı olan geçiş odur. O yüzden kare sayarak beklenir.
    for _ in 0..60 {
        bağlam
            .background_executor()
            .timer(Duration::from_millis(OTURMA_MS))
            .await;
        let şimdi = bağlam
            .update_window(pencere, |_, pencere, _| {
                pencere.frame_duration_snapshot().draw_duration_histogram.len()
            })
            .ok()?;
        if şimdi > geçiş_öncesi {
            break;
        }
    }
    let baş = bağlam
        .update_window(pencere, |_, pencere, _| {
            gpui_bilesenleri_galeri::render_sıfırla();
            pencere.frame_duration_snapshot().draw_duration_histogram
        })
        .ok()?;
    bağlam
        .background_executor()
        .timer(Duration::from_secs(FAZ_SN))
        .await;
    let (son, ns) = bağlam
        .update_window(pencere, |_, pencere, _| {
            (
                pencere.frame_duration_snapshot().draw_duration_histogram,
                gpui_bilesenleri_galeri::render_toplam_ns(),
            )
        })
        .ok()?;
    let (fark, tam) = pencere_payı(&son, &baş);
    tam.then_some((fark, ns))
}

/// Raporun sonuna basılacak şüpheler.
///
/// Ayrı bayraklar tek `bool`a indirgenmedi çünkü okuyanın hangisinin
/// gerçekleştiğini bilmesi gerekir: eksik faz sayıları dengesizleştirir,
/// açılmamış kapı ise koşumun tamamını geçersiz kılar.
struct Kuşku {
    eksik_faz: bool,
    kapı_açılmadı: bool,
}

/// Kovanın anlamlı sayılması için gereken en az kare sayısı.
///
/// Altında kalan bir kova ortalama üretir ama o ortalama gürültüdür;
/// tek pahalı kare 20 karelik bir kovayı milisaniyelerce kaydırır.
const EN_AZ_KARE: u64 = 100;

fn dönüşümlü_raporla(
    pencere: &gpui::Window,
    faz_sayısı: u64,
    kova: &[Option<Histogram<u64>>; 2],
    render_ns: &[u64; 2],
    kuşku: Kuşku,
) {
    const MS: f64 = 1_000_000.;
    println!("\n— dönüşümlü A/B ölçümü · {faz_sayısı} faz × {FAZ_SN} sn · sıra ABBA —");
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

    let adlar = ["A önbellekli", "B taban"];
    let mut ortalama = [None; 2];
    for (indis, ad) in adlar.iter().enumerate() {
        match &kova[indis] {
            Some(histogram) if !histogram.is_empty() => {
                println!("{}", özet(ad, histogram, MS, "ms"));
                let render = render_ns[indis] as f64 / histogram.len() as f64 / MS;
                println!(
                    "{:<24} render gövdeleri {render:6.3} ms · render sonrası {:6.3} ms",
                    "",
                    (histogram.mean() / MS - render).max(0.),
                );
                ortalama[indis] = Some((histogram.mean() / MS, render));
            }
            _ => println!("{ad:<24} örnek yok"),
        }
    }

    if let (Some((a_draw, a_render)), Some((b_draw, b_render))) = (ortalama[0], ortalama[1]) {
        println!(
            "fark (A−B, ortalama)     toplam draw {:+.3} ms (%{:+.1}) · \
             render gövdeleri {:+.3} ms · render sonrası {:+.3} ms",
            a_draw - b_draw,
            (a_draw - b_draw) / b_draw * 100.,
            a_render - b_render,
            (a_draw - a_render) - (b_draw - b_render),
        );
    }

    if kuşku.kapı_açılmadı {
        eprintln!(
            "GEÇERSİZ: ölçüm kapısı iki dakikada açılmadı — pencereye hiç \
             metin yazılmadı ve ölçüm yine de başladı. Bu koşumun sayıları \
             yazma evresini temsil etmez; koşum tekrarlanmalıdır."
        );
    }
    let az_örnek: Vec<&str> = adlar
        .iter()
        .enumerate()
        .filter(|(indis, _)| {
            kova[*indis]
                .as_ref()
                .is_none_or(|histogram| histogram.len() < EN_AZ_KARE)
        })
        .map(|(_, ad)| *ad)
        .collect();
    if !az_örnek.is_empty() {
        eprintln!(
            "GEÇERSİZ: {} kovası {EN_AZ_KARE} karenin altında. Fazlar \
             boyunca kesintisiz ve hızlı yazılmalı; seyrek yazmada faz \
             başına birkaç kare düşer ve ortalama tek bir pahalı kareyle \
             savrulur.",
            az_örnek.join(" ve "),
        );
    }
    if kuşku.eksik_faz {
        eprintln!(
            "uyarı: en az bir faz sayılamadı; kovalar eşit sayıda faz \
             taşımıyor olabilir ve karşılaştırma bu payla okunmalıdır."
        );
    }
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

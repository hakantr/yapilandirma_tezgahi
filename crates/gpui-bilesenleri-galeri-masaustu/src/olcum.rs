//! Gerçek pencerede girdi gecikmesi ve kare maliyeti ölçümü.
//!
//! Headless kare ölçümü (`gpui-bilesenleri-galeri/tests/kare_olcumu.rs`)
//! CPU tarafındaki element kuruluşu, yerleşim, prepaint ve sahne üretimini
//! görür; yerel pencerenin platform/GPU sunum yolu orada yoktur. Bu modül
//! gerçek pencere yolunu ölçer ve ancak `olcum` özelliğiyle derlenir.
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
//! **Üç kip.** `--olcum <sn>` mutlak sayıları verir (gecikme, kare
//! maliyeti, aşama ayrımı). `--olcum-ab <sn>` ise sağ kolon önbelleğinin
//! kazancını ölçer: önbelleği koşum içinde beşer saniyelik ABBA fazlarıyla
//! açıp kapatır ve iki kova tutar. `--olcum-izleyici <sn>` olay akışı ile
//! iki bildirim gözlemcisini 2×2 ablation düzeninde ölçer; bileşenin kendi
//! düzenleme, ayrıştırma ve doğrulama yolu bütün fazlarda aynıdır. Bu kip
//! fiziksel klavye gecikmesi ölçmez: yaşayan alanı gerçek pencerede kendi
//! giriş yolundan programatik değiştirir. `--olcum-giris <sn>` de aynı
//! otomatik yükü tek kovada ölçer; yanına `--minimal-giris` verildiğinde
//! `GaleriUygulaması` yerine yalnız bir `GirişKutusu` taşıyan kökü açar.
//! Dönüşümlü kipler, iki ayrı binary'yi arka
//! arkaya koşturmanın işe yaramamasından doğdu — koşumlar arası gürültü
//! (~4,4 ms) aranan etkiden (~0,9 ms) büyüktü.

use std::time::Duration;

use gpui::{App, AppContext as _, WindowHandle};
#[cfg(feature = "olcum-izleyici")]
use gpui::{Bounds, Pixels, WindowBounds, WindowOptions};
use gpui_bilesenleri_galeri::GaleriUygulaması;
#[cfg(feature = "olcum-izleyici")]
use gpui_bilesenleri_galeri::{
    MinimalGirişÖlçümü, SolListeÖlçümKonumu, İzleyiciDeneyiDurumu, İzleyiciEtkiSayacı,
};
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

/// Ölçüm argümanlarından biri verildiyse ilgili koşumu kurar.
///
/// Argüman ayrıştırma da burada: sarmalayıcı (`main.rs`) yalnız platform
/// kurulumu taşır ve bir bekçi onu 80 satırın altında tutar.
pub fn ölçümü_kur(pencere: &WindowHandle<GaleriUygulaması>, bağlam: &mut App) {
    pencere
        .update(bağlam, |_, pencere, _| pencere.activate_window())
        .ok();
    #[cfg(feature = "olcum-izleyici")]
    if let Some(saniye) = argüman_saniyesi("--olcum-giris") {
        let konum = sol_liste_ölçüm_konumu();
        let liste = if gpui_bilesenleri_galeri::sol_liste_sanallaştırması_açık() {
            "sanal ListState"
        } else {
            "olağan flex-scroll"
        };
        giriş_ölçümünü_planla(
            *pencere,
            saniye,
            format!("tam tezgâh · {liste} · {}", konum.adı()),
            Some(konum),
            bağlam,
        );
        return;
    }
    #[cfg(feature = "olcum-izleyici")]
    if let Some(saniye) = argüman_saniyesi("--olcum-izleyici") {
        izleyici_ölçümünü_planla(*pencere, saniye, bağlam);
        return;
    }
    if let Some(saniye) = argüman_saniyesi("--olcum-ab") {
        dönüşümlü_ölçümü_planla((*pencere).into(), saniye, bağlam);
        return;
    }
    if let Some(saniye) = argüman_saniyesi("--olcum") {
        ölçümü_planla((*pencere).into(), saniye, bağlam);
    }
}

/// `--minimal-giris` verilmişse tezgâh yerine tek alanlı pencereyi açar.
///
/// Dönüş değeri çağıranın normal galeri penceresini açmaması gerektiğini
/// bildirir. Minimal kök `GaleriUygulaması`nı hiç oluşturmaz.
#[cfg(feature = "olcum-izleyici")]
pub fn minimal_giriş_penceresini_aç(sınırlar: Bounds<Pixels>, bağlam: &mut App) -> bool {
    if !std::env::args().any(|argüman| argüman == "--minimal-giris") {
        return false;
    }
    let açıldı = bağlam.open_window(
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(sınırlar)),
            ..Default::default()
        },
        |pencere, bağlam| bağlam.new(|bağlam| MinimalGirişÖlçümü::yeni(pencere, bağlam)),
    );
    match açıldı {
        Ok(pencere) => {
            bağlam.activate(true);
            pencere
                .update(bağlam, |_, pencere, _| pencere.activate_window())
                .ok();
            if let Some(saniye) = argüman_saniyesi("--olcum-giris") {
                giriş_ölçümünü_planla(
                    pencere,
                    saniye,
                    "minimal GirişKutusu".to_owned(),
                    None,
                    bağlam,
                );
            }
        }
        Err(hata) => eprintln!("minimal giriş ölçüm penceresi açılamadı: {hata}"),
    }
    true
}

#[cfg(feature = "olcum-izleyici")]
fn sol_liste_ölçüm_konumu() -> SolListeÖlçümKonumu {
    let değer = std::env::args()
        .skip_while(|argüman| argüman != "--sol-konum")
        .nth(1);
    match değer.as_deref() {
        Some("orta") => SolListeÖlçümKonumu::Orta,
        Some("son") => SolListeÖlçümKonumu::Son,
        _ => SolListeÖlçümKonumu::Üst,
    }
}

fn argüman_saniyesi(ad: &str) -> Option<u64> {
    std::env::args()
        .skip_while(|argüman| argüman != ad)
        .nth(1)
        .and_then(|değer| değer.parse().ok())
}

/// Yalnız ölçüm binary'sinde bulunan yapısal A/B bayraklarını uygular.
#[cfg(feature = "olcum-izleyici")]
pub fn deney_bayraklarını_kur() {
    let sanal_sol_liste = std::env::args().any(|argüman| argüman == "--sanal-sol-liste");
    gpui_bilesenleri_galeri::sol_liste_sanallaştırmasını_ayarla(sanal_sol_liste);
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
/// Programatik düzenlemeler arasındaki süre.
///
/// 50 ms, 60 Hz ekranda önceki düzenlemenin karesine yetişmesi için üç
/// sunum aralığı bırakır; faz başına 100 gerçek düzenleme üretir.
#[cfg(feature = "olcum-izleyici")]
const OTOMATİK_DÜZENLEME_MS: u64 = 50;
#[cfg(feature = "olcum-izleyici")]
const OTOMATİK_FAZ_DÜZENLEMESİ: u64 = FAZ_SN * 1_000 / OTOMATİK_DÜZENLEME_MS;
/// Geçiş karesi beklenirken iki yoklama arasındaki süre.
///
/// Toplam bekleme bunun 60 katıyla sınırlıdır (3 sn): kare hiç gelmezse
/// faz yine de ölçülür, yalnız geçiş karesini dışarıda bırakma güvencesi
/// kalmaz.
const OTURMA_MS: u64 = 50;

/// Tek alan ve tam tezgâh için ortak programatik yazma sözleşmesi.
#[cfg(feature = "olcum-izleyici")]
trait OtomatikGirişKökü: gpui::Render + Sized + 'static {
    fn ölçüm_metinini_yaz(
        &mut self,
        metin: &str,
        pencere: &mut gpui::Window,
        bağlam: &mut gpui::Context<Self>,
    );

    fn ölçüm_sol_konumunu_ayarla(
        &mut self,
        _konum: SolListeÖlçümKonumu,
        _bağlam: &mut gpui::Context<Self>,
    ) {
    }

    fn ölçüm_sol_mantıksal_konumu(&self, _bağlam: &App) -> Option<(usize, gpui::Pixels)> {
        None
    }
}

#[cfg(feature = "olcum-izleyici")]
impl OtomatikGirişKökü for GaleriUygulaması {
    fn ölçüm_metinini_yaz(
        &mut self,
        metin: &str,
        pencere: &mut gpui::Window,
        bağlam: &mut gpui::Context<Self>,
    ) {
        self.ölçüm_alanına_yaz(metin, pencere, bağlam);
    }

    fn ölçüm_sol_konumunu_ayarla(
        &mut self,
        konum: SolListeÖlçümKonumu,
        bağlam: &mut gpui::Context<Self>,
    ) {
        GaleriUygulaması::ölçüm_sol_konumunu_ayarla(self, konum, bağlam);
    }

    fn ölçüm_sol_mantıksal_konumu(&self, bağlam: &App) -> Option<(usize, gpui::Pixels)> {
        Some(GaleriUygulaması::ölçüm_sol_mantıksal_konumu(
            self, bağlam,
        ))
    }
}

#[cfg(feature = "olcum-izleyici")]
impl OtomatikGirişKökü for MinimalGirişÖlçümü {
    fn ölçüm_metinini_yaz(
        &mut self,
        metin: &str,
        pencere: &mut gpui::Window,
        bağlam: &mut gpui::Context<Self>,
    ) {
        self.ölçüm_alanına_yaz(metin, pencere, bağlam);
    }
}

/// İki kökü aynı 50 ms düzenleme temposuyla ölçer.
#[cfg(feature = "olcum-izleyici")]
fn giriş_ölçümünü_planla<K: OtomatikGirişKökü>(
    pencere: WindowHandle<K>,
    saniye: u64,
    kip: String,
    sol_konum: Option<SolListeÖlçümKonumu>,
    bağlam: &mut App,
) {
    let düzenleme_sayısı = saniye.saturating_mul(1_000) / OTOMATİK_DÜZENLEME_MS;
    eprintln!(
        "tek giriş ölçümü: {kip}; {düzenleme_sayısı} düzenleme × \
         {OTOMATİK_DÜZENLEME_MS} ms. Fiziksel klavye gerekmez."
    );
    bağlam
        .spawn(async move |bağlam| {
            // Font yükleme ve ilk ağaç kuruluşu ölçüme girmesin.
            bağlam
                .background_executor()
                .timer(Duration::from_secs(1))
                .await;
            pencere
                .update(bağlam, |kök, pencere, bağlam| {
                    kök.ölçüm_metinini_yaz("izleyici ölçümü ısınma", pencere, bağlam);
                    if let Some(konum) = sol_konum {
                        kök.ölçüm_sol_konumunu_ayarla(konum, bağlam);
                    }
                })
                .ok();
            bağlam
                .background_executor()
                .timer(Duration::from_millis(250))
                .await;
            let başlangıç = pencere
                .update(bağlam, |_, pencere, _| {
                    gpui_bilesenleri_galeri::render_sıfırla();
                    pencere.frame_duration_snapshot().draw_duration_histogram
                })
                .ok();
            let mut uygulanan_düzenleme = 0;
            for sıra in 0..düzenleme_sayısı {
                let metin = if sıra % 2 == 0 {
                    "izleyici ölçümü alfa"
                } else {
                    "izleyici ölçümü beta"
                };
                if pencere
                    .update(bağlam, |kök, pencere, bağlam| {
                        kök.ölçüm_metinini_yaz(metin, pencere, bağlam);
                    })
                    .is_err()
                {
                    break;
                }
                uygulanan_düzenleme += 1;
                bağlam
                    .background_executor()
                    .timer(Duration::from_millis(OTOMATİK_DÜZENLEME_MS))
                    .await;
            }
            pencere
                .update(bağlam, |kök, pencere, bağlam| {
                    let mantıksal_konum = kök.ölçüm_sol_mantıksal_konumu(bağlam);
                    giriş_ölçümünü_raporla(
                        pencere,
                        &kip,
                        düzenleme_sayısı,
                        uygulanan_düzenleme,
                        başlangıç.as_ref(),
                        mantıksal_konum,
                    );
                })
                .ok();
            bağlam.update(|bağlam| bağlam.quit());
        })
        .detach();
}

#[cfg(feature = "olcum-izleyici")]
fn giriş_ölçümünü_raporla(
    pencere: &gpui::Window,
    kip: &str,
    beklenen_düzenleme: u64,
    uygulanan_düzenleme: u64,
    başlangıç: Option<&Histogram<u64>>,
    sol_konum: Option<(usize, gpui::Pixels)>,
) {
    const MS: f64 = 1_000_000.;
    let son = pencere.frame_duration_snapshot().draw_duration_histogram;
    let Some(başlangıç) = başlangıç else {
        eprintln!("GEÇERSİZ: başlangıç draw histogramı alınamadı");
        return;
    };
    let (çizim, tam) = pencere_payı(&son, başlangıç);
    println!("\n— tek giriş ölçümü · {kip} —");
    println!(
        "ortam                    {:.0}×{:.0} px · ölçek {:.1}× · etkin {} · derleme {}",
        f32::from(pencere.viewport_size().width),
        f32::from(pencere.viewport_size().height),
        pencere.scale_factor(),
        if pencere.is_window_active() {
            "evet"
        } else {
            "hayır"
        },
        if cfg!(debug_assertions) {
            "hata ayıklama"
        } else {
            "optimize"
        },
    );
    println!("{}", özet("çizim (draw)", &çizim, MS, "ms"));
    println!("düzenleme                {uygulanan_düzenleme}/{beklenen_düzenleme}");
    if let Some((indis, ofset)) = sol_konum {
        let sol_kart = gpui_bilesenleri_galeri::sol_kart_kurulum_sayısı();
        println!(
            "sol görünürlük            üst öğe {indis} · öğe içi {:.1} px · \
             kart kurulumu {sol_kart}",
            f32::from(ofset),
        );
        if !çizim.is_empty() {
            println!(
                "sol kart/kare            {:.2}",
                sol_kart as f64 / çizim.len() as f64,
            );
        }
    }
    if !çizim.is_empty() {
        let kök_render =
            gpui_bilesenleri_galeri::render_toplam_ns() as f64 / çizim.len() as f64 / MS;
        println!(
            "aşama (ortalama)         ölçülen kök render {kök_render:6.3} ms · \
             kalan draw {:6.3} ms",
            (çizim.mean() / MS - kök_render).max(0.),
        );
    }
    if !tam {
        eprintln!("GEÇERSİZ: draw histogramı farkı alınamadı");
    }
    if uygulanan_düzenleme != beklenen_düzenleme {
        eprintln!(
            "GEÇERSİZ: {beklenen_düzenleme} düzenlemenin yalnız \
             {uygulanan_düzenleme} tanesi uygulanabildi"
        );
    }
    if çizim.len() < beklenen_düzenleme.saturating_mul(9) / 10 {
        eprintln!(
            "GEÇERSİZ: {} draw, {beklenen_düzenleme} düzenlemenin %90'ından az",
            çizim.len(),
        );
    }
}

/// İstenen süreyi eksiksiz ABBA bloklarına indirger.
///
/// Yarım blok, A ve B'yi doğrusal zaman kaymasına karşı dengesiz bırakır.
/// Dört fazdan kısa istek de tek tam blok olarak çalışır.
fn abba_faz_sayısı(saniye: u64) -> u64 {
    let ham = (saniye / FAZ_SN).max(4);
    ham - ham % 4
}

/// Dört durumlu deneyde zaman konumunu dengeleyen tek tam desen.
///
/// Her durum her dört fazlık blok içindeki 1., 2., 3. ve 4. konumda bir
/// kez bulunur. Böylece doğrusal ısınma/termal kayma tek bir duruma
/// yüklenmez. İki durumlu deneydeki ABBA'nın dört durumlu karşılığıdır.
#[cfg(feature = "olcum-izleyici")]
const İZLEYİCİ_SIRASI: [İzleyiciDeneyiDurumu; 16] = [
    İzleyiciDeneyiDurumu::Tümü,
    İzleyiciDeneyiDurumu::OlayAkışıYok,
    İzleyiciDeneyiDurumu::Hiçbiri,
    İzleyiciDeneyiDurumu::GözlemPanelleriYok,
    İzleyiciDeneyiDurumu::OlayAkışıYok,
    İzleyiciDeneyiDurumu::GözlemPanelleriYok,
    İzleyiciDeneyiDurumu::Tümü,
    İzleyiciDeneyiDurumu::Hiçbiri,
    İzleyiciDeneyiDurumu::GözlemPanelleriYok,
    İzleyiciDeneyiDurumu::Hiçbiri,
    İzleyiciDeneyiDurumu::OlayAkışıYok,
    İzleyiciDeneyiDurumu::Tümü,
    İzleyiciDeneyiDurumu::Hiçbiri,
    İzleyiciDeneyiDurumu::Tümü,
    İzleyiciDeneyiDurumu::GözlemPanelleriYok,
    İzleyiciDeneyiDurumu::OlayAkışıYok,
];

#[cfg(feature = "olcum-izleyici")]
fn izleyici_faz_sayısı(saniye: u64) -> u64 {
    let ham = saniye.div_ceil(FAZ_SN).max(İZLEYİCİ_SIRASI.len() as u64);
    ham.div_ceil(İZLEYİCİ_SIRASI.len() as u64) * İZLEYİCİ_SIRASI.len() as u64
}

#[cfg(feature = "olcum-izleyici")]
fn izleyici_indisi(durum: İzleyiciDeneyiDurumu) -> usize {
    match durum {
        İzleyiciDeneyiDurumu::Tümü => 0,
        İzleyiciDeneyiDurumu::OlayAkışıYok => 1,
        İzleyiciDeneyiDurumu::GözlemPanelleriYok => 2,
        İzleyiciDeneyiDurumu::Hiçbiri => 3,
    }
}

/// Tüketici yan etkilerini aynı süreçte dört dengeli kovada ölçer.
#[cfg(feature = "olcum-izleyici")]
fn izleyici_ölçümünü_planla(
    pencere: WindowHandle<GaleriUygulaması>,
    saniye: u64,
    bağlam: &mut App,
) {
    let faz_sayısı = izleyici_faz_sayısı(saniye);
    eprintln!(
        "izleyici ablation ölçümü: programatik gerçek giriş yolu, {faz_sayısı} \
         faz × {FAZ_SN} sn. Dört durum zaman konumuna dengeli dağılır; alanın \
         kendi düzenleme, ayrıştırma ve doğrulama yolu bütün fazlarda açıktır."
    );
    bağlam
        .spawn(async move |bağlam| {
            let mut kovalar: [İzleyiciKovası; 4] = std::array::from_fn(|_| Default::default());
            let mut eksik_faz = false;
            for sıra in 0..faz_sayısı {
                let durum = İZLEYİCİ_SIRASI[sıra as usize % İZLEYİCİ_SIRASI.len()];
                match izleyici_fazını_ölç(bağlam, pencere, durum).await {
                    Some(faz) => {
                        if !kovalar[izleyici_indisi(durum)].ekle(faz) {
                            eksik_faz = true;
                        }
                    }
                    None => eksik_faz = true,
                }
            }
            pencere
                .update(bağlam, |_, pencere, _| {
                    gpui_bilesenleri_galeri::izleyici_deneyi_durumunu_ayarla(
                        İzleyiciDeneyiDurumu::Tümü,
                    );
                    izleyici_raporla(
                        pencere,
                        faz_sayısı,
                        &kovalar,
                        Kuşku {
                            eksik_faz,
                            kapı_açılmadı: false,
                        },
                    );
                })
                .ok();
            bağlam.update(|bağlam| bağlam.quit());
        })
        .detach();
}

#[cfg(feature = "olcum-izleyici")]
struct İzleyiciFazı {
    çizim: Histogram<u64>,
    render_ns: u64,
    kare: u64,
    düzenleme: u64,
    etkiler: İzleyiciEtkiSayacı,
}

#[cfg(feature = "olcum-izleyici")]
#[derive(Default)]
struct İzleyiciKovası {
    çizim: Option<Histogram<u64>>,
    render_ns: u64,
    faz_kareleri: Vec<u64>,
    faz_düzenlemeleri: Vec<u64>,
    etkiler: İzleyiciEtkiSayacı,
}

#[cfg(feature = "olcum-izleyici")]
impl İzleyiciKovası {
    fn ekle(&mut self, faz: İzleyiciFazı) -> bool {
        if !histogramı_ekle(&mut self.çizim, faz.çizim) {
            return false;
        }
        self.render_ns = self.render_ns.saturating_add(faz.render_ns);
        self.faz_kareleri.push(faz.kare);
        self.faz_düzenlemeleri.push(faz.düzenleme);
        self.etkiler.alan_durumu_bildirimi = self
            .etkiler
            .alan_durumu_bildirimi
            .saturating_add(faz.etkiler.alan_durumu_bildirimi);
        self.etkiler.olay_akışı_kaydı = self
            .etkiler
            .olay_akışı_kaydı
            .saturating_add(faz.etkiler.olay_akışı_kaydı);
        self.etkiler.yuva_notu_bildirimi = self
            .etkiler
            .yuva_notu_bildirimi
            .saturating_add(faz.etkiler.yuva_notu_bildirimi);
        true
    }
}

#[cfg(feature = "olcum-izleyici")]
fn histogramı_ekle(hedef: &mut Option<Histogram<u64>>, yeni: Histogram<u64>) -> bool {
    match hedef {
        Some(birikim) => birikim.add(&yeni).is_ok(),
        None => {
            *hedef = Some(yeni);
            true
        }
    }
}

/// Tek izleyici fazı: durumu kur, geçiş karesini at, sonra histogram ve
/// tüketici sayaçlarının yalnız o faza ait farkını al.
#[cfg(feature = "olcum-izleyici")]
async fn izleyici_fazını_ölç(
    bağlam: &mut gpui::AsyncApp,
    pencere: WindowHandle<GaleriUygulaması>,
    durum: İzleyiciDeneyiDurumu,
) -> Option<İzleyiciFazı> {
    let geçiş_öncesi = pencere
        .update(bağlam, |_, pencere, _| {
            gpui_bilesenleri_galeri::izleyici_deneyi_durumunu_ayarla(durum);
            pencere.refresh();
            pencere
                .frame_duration_snapshot()
                .draw_duration_histogram
                .len()
        })
        .ok()?;
    let mut geçiş_görüldü = false;
    for _ in 0..60 {
        bağlam
            .background_executor()
            .timer(Duration::from_millis(OTURMA_MS))
            .await;
        let şimdi = pencere
            .update(bağlam, |_, pencere, _| {
                pencere
                    .frame_duration_snapshot()
                    .draw_duration_histogram
                    .len()
            })
            .ok()?;
        if şimdi > geçiş_öncesi {
            geçiş_görüldü = true;
            break;
        }
    }
    if !geçiş_görüldü {
        return None;
    }
    let (baş_çizim, baş_düzenleme) = pencere
        .update(bağlam, |_, pencere, _| {
            gpui_bilesenleri_galeri::render_sıfırla();
            gpui_bilesenleri_galeri::izleyici_etki_sayacını_sıfırla();
            (
                pencere.frame_duration_snapshot().draw_duration_histogram,
                gpui_bilesenleri_galeri::düzenleme_sayısı(),
            )
        })
        .ok()?;
    for sıra in 0..OTOMATİK_FAZ_DÜZENLEMESİ {
        let metin = if sıra % 2 == 0 {
            "izleyici ölçümü alfa"
        } else {
            "izleyici ölçümü beta"
        };
        pencere
            .update(bağlam, |kök, pencere, bağlam| {
                kök.ölçüm_alanına_yaz(metin, pencere, bağlam);
            })
            .ok()?;
        bağlam
            .background_executor()
            .timer(Duration::from_millis(OTOMATİK_DÜZENLEME_MS))
            .await;
    }
    let (son_çizim, son_düzenleme, render_ns, etkiler) = pencere
        .update(bağlam, |_, pencere, _| {
            (
                pencere.frame_duration_snapshot().draw_duration_histogram,
                gpui_bilesenleri_galeri::düzenleme_sayısı(),
                gpui_bilesenleri_galeri::render_toplam_ns(),
                gpui_bilesenleri_galeri::izleyici_etki_sayacı(),
            )
        })
        .ok()?;
    let (çizim, çizim_tam) = pencere_payı(&son_çizim, &baş_çizim);
    çizim_tam.then_some(İzleyiciFazı {
        kare: çizim.len(),
        çizim,
        render_ns,
        düzenleme: son_düzenleme.saturating_sub(baş_düzenleme),
        etkiler,
    })
}

/// Aynı koşum içinde önbellekli/önbelleksiz fazları sırayla ölçer.
///
/// İki ayrı binary'yi arka arkaya koşturmak işe yaramadı: aynı binary'nin
/// iki koşumu arasındaki fark (~4,4 ms), aranan etkiden (~0,9 ms) beş kat
/// büyük çıktı. Baskın değişken derleme değil, koşum — termal durum, arka
/// plan yükü ve elle yazma temposu. Fazları tek süreç içinde dönüşümlü
/// koşturmak bu üçünü de iki kovaya eşit dağıtır.
fn dönüşümlü_ölçümü_planla(
    pencere: gpui::AnyWindowHandle, saniye: u64, bağlam: &mut App
) {
    let faz_sayısı = abba_faz_sayısı(saniye);
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
            let mut faz_kareleri: [Vec<u64>; 2] = std::array::from_fn(|_| Vec::new());
            let mut eksik_faz = false;
            for sıra in 0..faz_sayısı {
                // ABBA: doğrusal kayma (ısınma, termal kısma, arka plan
                // yükü) iki kovaya da eşit dağılsın diye. Düz AB sırası
                // kaymanın tamamını ikinci hâle yükler.
                let önbellekli = matches!(sıra % 4, 0 | 3);
                let indis = usize::from(!önbellekli);
                match faz_ölç(bağlam, pencere, önbellekli).await {
                    Some((fark, ns)) => {
                        let kare_sayısı = fark.len();
                        match &mut kova[indis] {
                            Some(birikim) => {
                                if birikim.add(&fark).is_err() {
                                    eksik_faz = true;
                                    continue;
                                }
                            }
                            None => kova[indis] = Some(fark),
                        }
                        render_ns[indis] = render_ns[indis].saturating_add(ns);
                        faz_kareleri[indis].push(kare_sayısı);
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
                        &faz_kareleri,
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
            pencere
                .frame_duration_snapshot()
                .draw_duration_histogram
                .len()
        })
        .ok()?;
    // Geçiş karesi zorunlu ıskadır: bayrak değişince ağaç yeniden kurulur
    // ve önbellekli hâle geçişte önbellek de o karede dolar. **Süreyle**
    // beklemek yetmez — yazma seyrekse o kare sabit bekleme bittikten
    // sonra gelir ve doğrudan ölçüme sızar; üstelik yalnız A kovasına,
    // çünkü pahalı olan geçiş odur. O yüzden kare sayarak beklenir.
    let mut geçiş_görüldü = false;
    for _ in 0..60 {
        bağlam
            .background_executor()
            .timer(Duration::from_millis(OTURMA_MS))
            .await;
        let şimdi = bağlam
            .update_window(pencere, |_, pencere, _| {
                pencere
                    .frame_duration_snapshot()
                    .draw_duration_histogram
                    .len()
            })
            .ok()?;
        if şimdi > geçiş_öncesi {
            geçiş_görüldü = true;
            break;
        }
    }
    if !geçiş_görüldü {
        return None;
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
/// Her beş saniyelik fazda aranacak en az kare.
///
/// Toplam kova eşiği tek bir yoğun fazla aşılabilir; bu alt kapı boş ya da
/// neredeyse boş bir fazın ABBA dengesini sessizce bozmasını engeller.
const EN_AZ_FAZ_KARESİ: u64 = 5;

#[cfg(feature = "olcum-izleyici")]
fn izleyici_raporla(
    pencere: &gpui::Window,
    faz_sayısı: u64,
    kovalar: &[İzleyiciKovası; 4],
    kuşku: Kuşku,
) {
    const MS: f64 = 1_000_000.;
    const DURUMLAR: [İzleyiciDeneyiDurumu; 4] = [
        İzleyiciDeneyiDurumu::Tümü,
        İzleyiciDeneyiDurumu::OlayAkışıYok,
        İzleyiciDeneyiDurumu::GözlemPanelleriYok,
        İzleyiciDeneyiDurumu::Hiçbiri,
    ];
    println!(
        "\n— izleyici ablation ölçümü · {faz_sayısı} faz × {FAZ_SN} sn · \
         dört durum dengeli —"
    );
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
    println!(
        "sınır                    gerçek pencere + programatik replace_text_in_range; \
         abonelikler bağlı, yalnız özet/liste ve panel notify yan etkileri değişir"
    );

    let mut ortalamalar = [None; 4];
    for durum in DURUMLAR {
        let indis = izleyici_indisi(durum);
        let kova = &kovalar[indis];
        println!("\n{}", durum.ad());
        match &kova.çizim {
            Some(çizim) if !çizim.is_empty() => {
                println!("{}", özet("  çizim (draw)", çizim, MS, "ms"));
                println!("{:<24} kare {:?}", "  faz dağılımı", kova.faz_kareleri);
                println!(
                    "{:<24} düzenleme {:?}",
                    "  faz dağılımı", kova.faz_düzenlemeleri
                );
                let render = kova.render_ns as f64 / çizim.len() as f64 / MS;
                let çizim_ort = çizim.mean() / MS;
                println!(
                    "{:<24} render gövdeleri {render:6.3} ms · render sonrası {:6.3} ms",
                    "  aşama",
                    (çizim_ort - render).max(0.),
                );
                println!(
                    "{:<24} alan={} · olay={} · yuva={}",
                    "  tüketici etkileri",
                    kova.etkiler.alan_durumu_bildirimi,
                    kova.etkiler.olay_akışı_kaydı,
                    kova.etkiler.yuva_notu_bildirimi,
                );
                ortalamalar[indis] = Some((çizim_ort, render));
            }
            _ => println!("  örnek yok"),
        }
    }

    if let Some((taban_draw, taban_render)) = ortalamalar[0] {
        println!("\n— tümüne göre kapatma farkı (durum−tümü, ortalama) —");
        for durum in DURUMLAR.into_iter().skip(1) {
            let indis = izleyici_indisi(durum);
            if let Some((draw, render)) = ortalamalar[indis] {
                println!(
                    "{:<24} draw {:+.3} ms (%{:+.1}) · render {:+.3} ms · \
                     render sonrası {:+.3} ms",
                    durum.ad(),
                    draw - taban_draw,
                    (draw - taban_draw) / taban_draw * 100.,
                    render - taban_render,
                    (draw - render) - (taban_draw - taban_render),
                );
            }
        }
    }

    if let [Some((a, _)), Some((b, _)), Some((c, _)), Some((d, _))] = ortalamalar {
        println!("\n— 2×2 etki kontrolü (etkin−kapalı, draw ortalaması) —");
        println!("olay akışı · gözlem açık   {:+.3} ms", a - b);
        println!("olay akışı · gözlem kapalı {:+.3} ms", c - d);
        println!("gözlem · olay açık         {:+.3} ms", a - c);
        println!("gözlem · olay kapalı       {:+.3} ms", b - d);
        println!("bütün tüketiciler          {:+.3} ms", a - d);
        println!("etkileşim                  {:+.3} ms", (a - b) - (c - d));
    }

    if kuşku.kapı_açılmadı {
        eprintln!(
            "GEÇERSİZ: ölçüm kapısı iki dakikada açılmadı — pencereye gerçek \
             metin yazılmadı."
        );
    }
    if kuşku.eksik_faz {
        eprintln!(
            "GEÇERSİZ: en az bir faz, histogram farkı ya da geçiş karesi \
             sayılamadı."
        );
    }
    let beklenen_faz = (faz_sayısı / 4) as usize;
    let zayıf: Vec<String> = DURUMLAR
        .iter()
        .filter_map(|durum| {
            let kova = &kovalar[izleyici_indisi(*durum)];
            let toplam_az = kova
                .çizim
                .as_ref()
                .is_none_or(|histogram| histogram.len() < EN_AZ_KARE);
            let faz_zayıf = kova.faz_kareleri.len() != beklenen_faz
                || kova
                    .faz_kareleri
                    .iter()
                    .any(|sayı| *sayı < EN_AZ_FAZ_KARESİ)
                || kova.faz_düzenlemeleri.len() != beklenen_faz
                || kova
                    .faz_düzenlemeleri
                    .iter()
                    .any(|sayı| *sayı != OTOMATİK_FAZ_DÜZENLEMESİ);
            (toplam_az || faz_zayıf).then(|| {
                format!(
                    "{}: kare={:?}, düzenleme={:?}",
                    durum.ad(),
                    kova.faz_kareleri,
                    kova.faz_düzenlemeleri
                )
            })
        })
        .collect();
    if !zayıf.is_empty() {
        eprintln!(
            "GEÇERSİZ: her durum {beklenen_faz} faz, toplam en az {EN_AZ_KARE} \
             kare, faz başına en az {EN_AZ_FAZ_KARESİ} kare ve tam \
             {OTOMATİK_FAZ_DÜZENLEMESİ} düzenleme taşımalı; {}.",
            zayıf.join("; "),
        );
    }
}

fn dönüşümlü_raporla(
    pencere: &gpui::Window,
    faz_sayısı: u64,
    kova: &[Option<Histogram<u64>>; 2],
    render_ns: &[u64; 2],
    faz_kareleri: &[Vec<u64>; 2],
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
                println!("{:<24} faz kareleri {:?}", "", faz_kareleri[indis]);
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
            "GEÇERSİZ: en az bir faz ya da geçiş karesi sayılamadı; kovalar \
             eksiksiz ve dengeli bir ABBA bloğu taşımıyor."
        );
    }
    let beklenen_faz = (faz_sayısı / 2) as usize;
    let zayıf_fazlar: Vec<String> = adlar
        .iter()
        .enumerate()
        .filter_map(|(indis, ad)| {
            let sayılar = &faz_kareleri[indis];
            (sayılar.len() != beklenen_faz || sayılar.iter().any(|sayı| *sayı < EN_AZ_FAZ_KARESİ))
                .then(|| format!("{ad}={sayılar:?}"))
        })
        .collect();
    if !zayıf_fazlar.is_empty() {
        eprintln!(
            "GEÇERSİZ: her kova {beklenen_faz} faz taşımalı ve her fazda en az \
             {EN_AZ_FAZ_KARESİ} kare olmalı; {}.",
            zayıf_fazlar.join(", "),
        );
    }
}

#[cfg(test)]
mod testler {
    use super::*;

    #[test]
    fn faz_sayısı_yalnız_tam_abba_blokları_üretir() {
        assert_eq!(abba_faz_sayısı(0), 4);
        assert_eq!(abba_faz_sayısı(19), 4);
        assert_eq!(abba_faz_sayısı(60), 12);
        assert_eq!(abba_faz_sayısı(65), 12);
        assert_eq!(abba_faz_sayısı(79), 12);
        assert_eq!(abba_faz_sayısı(80), 16);
    }

    #[test]
    #[cfg(feature = "olcum-izleyici")]
    fn izleyici_faz_sayısı_yalnız_tam_dengeli_desen_üretir() {
        assert_eq!(izleyici_faz_sayısı(0), 16);
        assert_eq!(izleyici_faz_sayısı(79), 16);
        assert_eq!(izleyici_faz_sayısı(80), 16);
        assert_eq!(izleyici_faz_sayısı(81), 32);
        assert_eq!(izleyici_faz_sayısı(160), 32);
    }

    #[test]
    #[cfg(feature = "olcum-izleyici")]
    fn izleyici_sırası_durumları_zaman_konumuna_dengeler() {
        let mut toplam = [0usize; 4];
        for durum in İZLEYİCİ_SIRASI {
            toplam[izleyici_indisi(durum)] += 1;
        }
        assert_eq!(toplam, [4; 4]);

        for konum in 0..4 {
            let mut konumdaki = [0usize; 4];
            for blok in 0..4 {
                let durum = İZLEYİCİ_SIRASI[blok * 4 + konum];
                konumdaki[izleyici_indisi(durum)] += 1;
            }
            assert_eq!(konumdaki, [1; 4]);
        }
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

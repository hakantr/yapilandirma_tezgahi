//! Kare maliyeti ölçüm koşumu — performans mimarisi turlarının hakemi.
//!
//! Varsayılan `cargo test` koşumunda **atlanır**; elle şöyle koşturulur:
//!
//! ```bash
//! KARE_OLCUM=1 cargo test --profile akici-dev -p gpui-bilesenleri-galeri \
//!     --test kare_olcumu -- --nocapture
//! ```
//!
//! Profil önemlidir: `dev` (optimizasyonsuz) sayıları birkaç kat şişirir.
//!
//! Karşılaştırma tabanı (kolon önbelleği kapalı) aynı komuta
//! `--features olcum-onbelleksiz` eklenerek alınır; elle düzenleme
//! gerekmez, iki koşum aynı kodu paylaşır.
//!
//! Her senaryo, kolonun o karede gerçekten kurulduğunu da sayar ve
//! sayılar ancak o kapı geçilirse yorumlanır (`kolon N/N`).
//!
//! Ölçülen şey **CPU işidir**: element ağacının kurulumu, yerleşim,
//! prepaint ve paint'in sahneye yazımı, artı mutasyonlu senaryolarda
//! bildirim/efekt zinciri. GPU sunumu (present/vsync) headless koşumda
//! yoktur; gerçek input-to-present bu sayının üstüne platform sunum
//! süresini ekler. Shaping her hedefte `CosmicTextSystem` ile gerçektir
//! (saf Rust, macOS ve Linux'ta aynı sonuç); Noop metin sistemi
//! kullanılmaz çünkü kare maliyetinin metin payını sıfır gösterirdi.
//!
//! Senaryolar:
//! - **D · temiz kare**: mutasyon yok; önbellekli kolon yeniden kullanılır.
//! - **K · tuş vuruşu**: alan metni gerçek giriş yolundan değişir; alan ve
//!   onu gözleyen paneller kirlenir, kolon **kurulmaz**. Hedef senaryo.
//! - **S · seçici**: açık seçici değişir; kolon tazelenir ve açılan
//!   seçicinin liste içeriği de kurulur (tembel liste yolu).
//! - **T · tercih**: `tezgahı_değiştir` — sürüm artar, `§29` raporu ve kod
//!   metni yeniden üretilir, kolon tazelenir. En pahalı olağan yol.

#![allow(non_ascii_idents)]

use std::sync::Arc;
use std::time::Instant;

use gpui::{
    AnyWindowHandle, Bounds, Context, TestApp, Window, WindowBounds, WindowOptions, point, px, size,
};
use gpui_bilesenleri_galeri::{
    GaleriUygulaması, GaleriVarlıkKaynağı, bileşen_tuş_bağlarını_kur, bölüm_çizim_sayısı,
    galeri_yazı_tiplerini_kur,
};

const ISINMA: usize = 30;
const TEKRAR: usize = 200;

fn uygulama_kur() -> TestApp {
    // Cosmic saf Rust'tır ve macOS ile Linux'ta aynı shaping'i verir; iki
    // hedefin sayıları karşılaştırılabilir kalır. (`MacTextSystem` dışa
    // açık değil; CoreText farkı bu ölçümün konusu olmadığı için mesele
    // değil.)
    TestApp::with_text_system_and_assets(
        Arc::new(gpui_wgpu::CosmicTextSystem::new("IBM Plex Sans")),
        Arc::new(GaleriVarlıkKaynağı),
    )
}

/// Değişiklik yokken tek bir karenin maliyeti.
///
/// Mutasyon olmadığı için efekt döngüsü çizim yapmaz; kare açıkça
/// istenir. Bu, ekranın taban çizim maliyetidir.
fn temiz_kare_süresi(uygulama: &mut TestApp, pencere: AnyWindowHandle) -> (f64, u64) {
    let önce = bölüm_çizim_sayısı();
    let başlangıç = Instant::now();
    uygulama.update(|bağlam| {
        pencere
            .update(bağlam, |_, pencere, bağlam| {
                pencere.draw(bağlam).clear(bağlam);
            })
            .expect("pencere açık");
    });
    (
        başlangıç.elapsed().as_secs_f64() * 1000.0,
        bölüm_çizim_sayısı() - önce,
    )
}

/// Bir senaryonun ölçüm çıktısı.
struct Sonuç {
    süreler: Vec<f64>,
    /// Ölçülen tekrarlarda sağ kolonun kaç kez kurulduğu.
    ///
    /// Ölçülen çizimlerin **yanı sıra** efekt döngüsünün kendiliğinden
    /// yaptığı çizimler de sayılır; bu yüzden beklenti "tekrar başına en az
    /// bir kurulum"dur. Sayının işi hız değil geçerliliktir: üçüncü turda
    /// `Entity::cached` denemesi tam buradan çürütüldü — süreler makul
    /// görünürken sayaç sıfırdı, yani kolon donmuştu.
    kolon_çizimi: u64,
}

/// Bir senaryoyu koşturur: her yinelemede mutasyonu uygular ve girdiden
/// ekrana kadar geçen **bütün CPU işini** zamanlar.
///
/// Ölçüm penceresi bilerek `update` bloğunun tamamıdır: mutasyonun
/// kendisi, efekt döngüsü (bildirimler, `refresh`, abonelik zincirleri) ve
/// o döngünün kirli pencere için yaptığı çizim(ler). Blok içine yerleştirilen
/// bir `draw`'ı ölçmek — bir ara öyleydi — yanlış kareyi ölçüyordu:
/// `refresh` efekt kuyruğuna girdiği için o çizim kolonun **kurulmadığı**
/// kareydi, kolon ise sonraki, ölçülmeyen çizimde kuruluyordu. Kolon
/// sayacı (`kolon N/N`) ölçülen işin gerçekten neyi içerdiğini söyler.
///
/// D senaryosunda mutasyon yoktur, yani pencere kirlenmez ve efekt
/// döngüsü çizim yapmaz; orada ölçüm açık bir `draw` ile alınır
/// ([`temiz_kare_süresi`]) ve "değişiklik yokken bir karenin maliyeti"
/// anlamına gelir.
fn senaryo(
    uygulama: &mut TestApp,
    pencere: AnyWindowHandle,
    mutasyon: &mut dyn FnMut(&mut GaleriUygulaması, &mut Window, &mut Context<GaleriUygulaması>),
) -> Sonuç {
    let mut süreler = Vec::with_capacity(TEKRAR);
    let mut kolon_başlangıcı = 0;
    for sıra in 0..(ISINMA + TEKRAR) {
        if sıra == ISINMA {
            kolon_başlangıcı = bölüm_çizim_sayısı();
        }
        let başlangıç = Instant::now();
        uygulama.update(|bağlam| {
            pencere
                .update(bağlam, |kök, pencere, bağlam| {
                    let görsel = kök
                        .downcast::<GaleriUygulaması>()
                        .expect("kök görünüm tezgâhtır");
                    görsel.update(bağlam, |uygulama, bağlam| {
                        mutasyon(uygulama, pencere, bağlam);
                    });
                })
                .expect("pencere açık");
        });
        let süre_ms = başlangıç.elapsed().as_secs_f64() * 1000.0;
        if sıra >= ISINMA {
            süreler.push(süre_ms);
        }
    }
    Sonuç {
        süreler,
        kolon_çizimi: bölüm_çizim_sayısı() - kolon_başlangıcı,
    }
}

fn özet(ad: &str, sonuç: &mut Sonuç) -> String {
    let süreler = &mut sonuç.süreler;
    süreler.sort_by(|a, b| a.total_cmp(b));
    let toplam: f64 = süreler.iter().sum();
    let ort = toplam / süreler.len() as f64;
    let yüzdelik = |p: f64| süreler[((süreler.len() as f64 * p) as usize).min(süreler.len() - 1)];
    format!(
        "{ad:<14} ort {ort:7.3} ms · p50 {:7.3} ms · p95 {:7.3} ms · en az {:7.3} ms · \
         kolon {:>3}/{}",
        yüzdelik(0.50),
        yüzdelik(0.95),
        süreler[0],
        sonuç.kolon_çizimi,
        TEKRAR,
    )
}

#[test]
fn kare_maliyeti() {
    if std::env::var("KARE_OLCUM").is_err() {
        eprintln!(
            "kare ölçümü atlandı — KARE_OLCUM=1 cargo test --profile akici-dev \
             -p gpui-bilesenleri-galeri --test kare_olcumu -- --nocapture"
        );
        return;
    }

    let mut uygulama = uygulama_kur();
    uygulama.update(|bağlam| {
        bileşen_tuş_bağlarını_kur(bağlam);
        galeri_yazı_tiplerini_kur(bağlam).expect("kitaplık yüzleri kaydedilir");
    });
    let pencere = uygulama.open_window_with_options(
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: point(px(0.), px(0.)),
                size: size(px(1600.), px(1000.)),
            })),
            ..Default::default()
        },
        |_, _| GaleriUygulaması::yeni(),
    );
    let tutamaç: AnyWindowHandle = pencere.handle().into();

    // D · temiz kare — mutasyon yok; kare açıkça istenir.
    let mut d = {
        let mut süreler = Vec::with_capacity(TEKRAR);
        let mut kolon = 0;
        for sıra in 0..(ISINMA + TEKRAR) {
            let (süre, kurulum) = temiz_kare_süresi(&mut uygulama, tutamaç);
            if sıra >= ISINMA {
                süreler.push(süre);
                kolon += kurulum;
            }
        }
        Sonuç {
            süreler,
            kolon_çizimi: kolon,
        }
    };

    // K · tuş vuruşu — metin gerçek giriş yolundan değişir; iki içerik
    // arasında gidip gelinir ki uzunluk kaymasın ve her kare gerçekten
    // kirli olsun.
    let mut dönüşüm = false;
    let mut k = senaryo(&mut uygulama, tutamaç, &mut |uygulama, pencere, bağlam| {
        dönüşüm = !dönüşüm;
        let metin = if dönüşüm {
            "ölçüm a 123"
        } else {
            "ölçüm b 456"
        };
        uygulama.ölçüm_alanına_yaz(metin, pencere, bağlam);
    });

    // S · seçici — açılıp kapanır; açık karede liste içeriği de kurulur.
    // Rapor ve kod önbellekleri tutar (tercih sürümü değişmez).
    let mut s = senaryo(&mut uygulama, tutamaç, &mut |uygulama, _, bağlam| {
        uygulama.seçiciyi_değiştir("imleç", bağlam);
    });

    // T · tercih — sürüm artar; rapor ve kod metni yeniden üretilir.
    let mut t = senaryo(&mut uygulama, tutamaç, &mut |uygulama, _, bağlam| {
        uygulama.tezgahı_değiştir(|tercih| tercih.sayaç = !tercih.sayaç, bağlam);
    });

    println!("kare maliyeti · {TEKRAR} tekrar · 1600×1000 · CPU (headless)");
    println!("{}", özet("D · temiz", &mut d));
    println!("{}", özet("K · tuş vuruşu", &mut k));
    println!("{}", özet("S · seçici", &mut s));
    println!("{}", özet("T · tercih", &mut t));

    // Ölçümün kendi geçerlilik kapısı: süreler ancak kolonun her karede
    // gerçekten kurulduğu biliniyorsa yorumlanabilir. Donmuş bir kolon
    // sayıları olduğundan ucuz gösterirdi — üçüncü turda tam da bu oldu.
    // Kolon önbelleklidir: D ve K'de kurulmamalı (kazanç), T'de kurulmalı
    // (tazelik). Sayı bu yüzden sürelerin nasıl okunacağını da söyler.
    #[cfg(not(feature = "olcum-onbelleksiz"))]
    {
        assert_eq!(
            d.kolon_çizimi, 0,
            "temiz karede kolon kuruluyor: önbellek isabet etmiyor"
        );
        assert_eq!(
            k.kolon_çizimi, 0,
            "tuş vuruşunda kolon kuruluyor: alan bildirimi kolona sızıyor"
        );
    }
    // Taban koşumunda kolon her karede kurulur; karşılaştırmanın anlamı da
    // budur.
    #[cfg(feature = "olcum-onbelleksiz")]
    for (ad, sonuç) in [("D", &d), ("K", &k)] {
        assert!(
            sonuç.kolon_çizimi >= TEKRAR as u64,
            "{ad}: taban koşumunda kolon {}/{TEKRAR} kuruldu",
            sonuç.kolon_çizimi
        );
    }
    // S ve T ölçülen işin **içinde** kolonu kurar: geçersizleme efekt
    // döngüsünde koşar ve o döngü ölçüm penceresinin içindedir. Tekrar
    // başına en az bir kurulum yoksa süreler yanlış kareyi ölçüyordur.
    for (ad, sonuç) in [("S", &s), ("T", &t)] {
        assert!(
            sonuç.kolon_çizimi >= TEKRAR as u64,
            "{ad}: kolon {}/{TEKRAR} — ölçülen iş kolon kurulumunu içermiyor",
            sonuç.kolon_çizimi
        );
    }
}

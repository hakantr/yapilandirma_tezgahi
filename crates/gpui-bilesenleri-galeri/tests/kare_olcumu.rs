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
//! Her senaryo, kolonun o karede gerçekten kurulduğunu da sayar ve
//! sayılar ancak o kapı geçilirse yorumlanır (`kolon N/N`).
//!
//! Ölçülen şey **CPU kare maliyeti**dir: element ağacının kurulumu,
//! yerleşim, prepaint ve paint'in sahneye yazımı — yani üç turun
//! değiştirdiği bütün iş. GPU sunumu (present/vsync) headless koşumda
//! yoktur; gerçek input-to-present bu sayının üstüne platform sunum
//! süresini ekler. macOS'ta gerçek CoreText shaping (`MacTextSystem`),
//! Linux'ta Cosmic shaping kullanılır; Noop metin sistemi kullanılmaz
//! çünkü kare maliyetinin metin payını sıfır gösterirdi.
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
    AnyWindowHandle, Bounds, Context, TestApp, Window, WindowBounds, WindowOptions, point, px,
    size,
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

/// Bir senaryoyu koşturur: her yinelemede mutasyonu uygular ve **hemen
/// ardından** gelen çizimi zamanlar — yani "girdi → ilk kare" maliyetini.
///
/// Mutasyon ile `draw` bilerek aynı `update` bloğundadır. Blok kapanınca
/// efekt döngüsü koşar ve kirli pencereyi kendiliğinden bir kez daha çizer
/// (`app.rs`, test kipi); o ikinci çizim ölçülmez. Ayrı bloklara bölünen
/// bir sıra — bir ara denendi — mutasyon sonrası **ikinci** kareyi ölçer
/// ve girdi maliyetini gizler. Kolon kökün durumundan okuduğu için
/// mutasyon, aynı blokta yapılan çizimde zaten görünür.
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
        let süre_ms = uygulama.update(|bağlam| {
            pencere
                .update(bağlam, |kök, pencere, bağlam| {
                    let görsel = kök
                        .downcast::<GaleriUygulaması>()
                        .expect("kök görünüm tezgâhtır");
                    görsel.update(bağlam, |uygulama, bağlam| {
                        mutasyon(uygulama, pencere, bağlam);
                    });
                    let başlangıç = Instant::now();
                    pencere.draw(bağlam).clear(bağlam);
                    başlangıç.elapsed().as_secs_f64() * 1000.0
                })
                .expect("pencere açık")
        });
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

    // D · temiz kare — mutasyon yok.
    let mut d = senaryo(&mut uygulama, tutamaç, &mut |_, _, _| {});

    // K · tuş vuruşu — metin gerçek giriş yolundan değişir; iki içerik
    // arasında gidip gelinir ki uzunluk kaymasın ve her kare gerçekten
    // kirli olsun.
    let mut dönüşüm = false;
    let mut k = senaryo(&mut uygulama, tutamaç, &mut |uygulama, pencere, bağlam| {
        dönüşüm = !dönüşüm;
        let metin = if dönüşüm { "ölçüm a 123" } else { "ölçüm b 456" };
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
    assert_eq!(
        d.kolon_çizimi, 0,
        "temiz karede kolon kuruluyor: önbellek isabet etmiyor"
    );
    assert_eq!(
        k.kolon_çizimi, 0,
        "tuş vuruşunda kolon kuruluyor: alan bildirimi kolona sızıyor"
    );
    assert!(
        t.kolon_çizimi > 0,
        "tercih değişiminde kolon kurulmuyor: yüzey bayat kalır"
    );
}

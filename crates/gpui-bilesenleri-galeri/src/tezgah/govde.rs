//! Tezgâh gövdesinin çizimi.
//!
//! Tezgâh galerinin orta bölgesine gömülü değil, kendi ekranı: genişlik
//! sabit (`956px`) ve altında yatay kaydırma var. Kip seçimi bir zamanlar
//! `container_query` ile ölçülen kaptan yapılıyordu; gömülülük bitince o
//! mekanizma çağrısız kaldı ve silindi.
//!
//! Bu modül F1'in **yapısal akış** işidir: kart sırası, iki kaydıran kolon
//! ve erişilebilir bölge/klavye turu. **Görsel tamamlanma
//! kapılıdır:** köşe yarıçapı `ORT-003`, tipografi ve opaklık `ORT-004/017`
//! fiziksel göçünü bekler. Bu yüzden burada `rounded()`, `text_size()`,
//! `font_family()` ve `opacity()` **çağrılmaz** — yerel fallback kurmak,
//! kapalı bir kapıyı açıkmış gibi göstermek olurdu.

use gpui::{AnyElement, Role, SharedString, div, prelude::*, px};
use gpui_bilesenleri_kabuk::YerelleştirmeAnahtarı;

use super::{
    Akış, KolonMetriği, TezgahBölümü, TezgahTokenları, Tezgahİçeriği, akış_bölümleri,
    ÇözülmüşTezgahGörünümü,
};

/// Tezgâh gövdesini çizer.
///
/// `çöz`, `ORT-021` anahtarını güncel locale sürümünde dizeye çevirir; kabuk
/// ham dize kaynağı tanımaz (`YÖN-006.ACC-008`).
pub fn tezgah_gövdesi(
    mut içerik: Tezgahİçeriği,
    g: std::sync::Arc<ÇözülmüşTezgahGörünümü>,
    t: TezgahTokenları,
    metin_ölçeği: f32,
    çöz: impl Fn(&YerelleştirmeAnahtarı) -> SharedString,
) -> impl IntoElement {
    let _ = &t;
    let tezgah_adı = çöz(&içerik.başlık);
    let önizleme_adı = çöz(&içerik.önizleme_başlığı);

    // Önizleme blokları sabit, ek kartlar ve kod paneli kayar.
    let sabit_bloklar = std::mem::take(&mut içerik.önizleme);
    let mut kayan_bloklar = std::mem::take(&mut içerik.sol_ek);
    // Kod paneli sol kolonun en altındadır: "yalnız A bölümü" notu profilin
    // kendi içeriğindedir, kabuk metin uydurmaz.
    kayan_bloklar.extend(içerik.kod.take());
    let sol = önizleme_kolonu(
        sabit_bloklar,
        kayan_bloklar,
        içerik.sol_sanal,
        içerik.sol_kaydırma,
        &g.kolonlar,
        önizleme_adı.clone(),
    );
    let sol_kolon_genişliği = g.kolonlar.önizleme_kolonu;

    // Tezgâh **sabit genişliklidir**: tasarımın kökü `min-width: 1216px`
    // taşır ve pencere daralınca sayfa yatay kaydırır. Kolon sayısını
    // ölçülen genişlikten seçmek — ki bir süre öyleydi — düğmeleri dar
    // pencerede birbirine sıkıştırıyor, taslağın hizasını bozuyordu.
    // Asgari genişlik garanti olduğu için gövde her zaman iki kolondur.
    let _ = metin_ölçeği;
    // Sağ kolon hazır gelir: bölümlerin kurulumu önbellekli panelin kendi
    // çizimindedir, kabuk yalnız yerleştirir.
    let sağ = içerik.yapılandırma;

    let gövde = div()
        .flex()
        .size_full()
        .min_h(px(0.))
        .gap(g.kolonlar.kolon_aralığı)
        .child(sol.w(sol_kolon_genişliği).flex_shrink_0())
        .child(sağ);

    // Klavye turu üst araç çubuğu → sol → orta → sağ sırasını korur;
    // tezgâh kendi içinde iki bölge sunar (`YÖN-006 §3.4`).
    div()
        .id("tezgah-govde")
        .role(Role::Group)
        .aria_label(tezgah_adı.clone())
        .size_full()
        .min_h(px(0.))
        .child(gövde)
}

/// Sol kolon: kabuk denetimleri, yaşayan önizleme, ek bloklar ve kod paneli.
///
/// Kendi içinde kaydırır; gövde kaydırmaz.
fn önizleme_kolonu(
    sabit: Vec<AnyElement>,
    kayan: Vec<AnyElement>,
    sanal: bool,
    kaydırma: gpui::ScrollHandle,
    metrik: &KolonMetriği,
    ad: SharedString,
) -> gpui::Stateful<gpui::Div> {
    let kayan_gövde = if sanal {
        // `ListState` kendi kaydırma ve görünür-aralık hesabını yapar.
        // Dışarıda ikinci bir scroll kabı kurmak iki ayrı ofset üretirdi.
        div()
            .flex_1()
            .min_h(px(0.))
            .children(kayan)
            .into_any_element()
    } else {
        div()
            .id("tezgah-onizleme-kaydırma")
            .flex_1()
            .min_h(px(0.))
            .overflow_y_scroll()
            .track_scroll(&kaydırma)
            // Dikey tekerlek burada tüketilir: dıştaki yatay kaydırma
            // katmanı, yalnız yatay ekseni olan bir kapta dikey deltayı
            // yataya çevirmesin.
            .on_scroll_wheel(|_, _, bağlam| bağlam.stop_propagation())
            .flex()
            .flex_col()
            .gap(metrik.kart_aralığı)
            .children(kayan)
            .into_any_element()
    };
    div()
        // `overflow_y_scroll` için `id` zorunludur.
        .id("tezgah-onizleme")
        .role(Role::Region)
        .aria_label(ad)
        // Kart grubu klavye turunda tek durak olur; içindeki denetimler
        // yerel sırayla gezilir.
        .tab_group()
        .tab_stop(false)
        // Flex çocuğun küçülebilmesi için; aksi hâlde kaydırma kurulmaz.
        .min_h(px(0.))
        .flex()
        .flex_col()
        .gap(metrik.kart_aralığı)
        // Kabuk denetimleri ve yaşayan alan **yerinde kalır**: aşağıdaki
        // kartlara bakarken de alanın o tercihle nasıl göründüğü görülsün.
        // Taslakta bu blok `position: sticky`; GPUI'de karşılığı yok, o
        // yüzden kolon ikiye ayrıldı — üst blok kaydırma kabının dışında.
        .child(
            div()
                .flex_shrink_0()
                .flex()
                .flex_col()
                .gap(metrik.kart_aralığı)
                .children(sabit),
        )
        .child(kayan_gövde)
}

/// Sağ kolonun tam gövdesi: bölümleri kartlara sarar, akışlara dizer.
///
/// Önbellekli bölüm paneli çizimde bunu çağırır; kolonun dış boyutu
/// panelin sarmalayıcı stilinden gelir (`size_full` o sınırları doldurur).
/// Sıra profilin verdiği sıradır; kabuk onu değiştirmez.
pub(crate) fn yapılandırma_kolonu_gövdesi(
    mut bölümler: Vec<TezgahBölümü>,
    g: &ÇözülmüşTezgahGörünümü,
    t: &TezgahTokenları,
    ad: SharedString,
    çöz: impl Fn(&YerelleştirmeAnahtarı) -> SharedString,
) -> impl IntoElement {
    let tam_genişlik: Vec<AnyElement> = akış_bölümleri(&mut bölümler, Akış::TamGenişlik)
        .into_iter()
        .map(|bölüm| bölüm_kartı(bölüm, g, t, &çöz))
        .collect();
    let akışlar: Vec<Vec<AnyElement>> = Akış::AKIŞLAR
        .into_iter()
        .map(|akış| {
            akış_bölümleri(&mut bölümler, akış)
                .into_iter()
                .map(|bölüm| bölüm_kartı(bölüm, g, t, &çöz))
                .collect()
        })
        .collect();
    let metrik = &g.kolonlar;

    let mut kolon = div()
        .id("tezgah-yapilandirma")
        .role(Role::Region)
        .aria_label(ad)
        .tab_group()
        .tab_stop(false)
        // `min_h` kaydırmadan **önce**: flex çocuğu varsayılan olarak
        // içeriğinin altına küçülmez ve kaydırma hiç kurulmaz. İki kaydıran
        // yüzey de aynı sırayı izler.
        .min_h(px(0.))
        .overflow_y_scroll()
        // Önizleme kolonuyla aynı gerekçe: dikey tekerlek burada tüketilir,
        // dıştaki yatay katman onu ikinci kez yataya çevirmesin.
        .on_scroll_wheel(|_, _, bağlam| bağlam.stop_propagation())
        .flex()
        .flex_col()
        .size_full()
        .gap(metrik.kart_aralığı)
        .children(tam_genişlik);

    for kartlar in akışlar {
        if kartlar.is_empty() {
            continue;
        }
        // Her akış **tek kolon**. Yapılandırma kolonu `524px`; onu ikiye
        // bölmek kartları `248px`e indiriyor ve düğme ızgaraları her
        // satırda kırılıyordu. Kartlar tam genişlikte, alt alta.
        kolon = kolon.child(akış_bloğu(kartlar, metrik));
    }
    kolon
}

/// Bir akışın kartlarını tek kolona dizer.
///
/// İki kolonlu dağılım ve ölçülen kaptan kip seçimi tezgâhın galeri içine
/// gömülü olduğu dönemden kalmaydı; tezgâh kendi ekranına taşınıp sabit
/// genişliğe (yatay kaydırmalı) geçince ikisi de çağrısız kaldı. Testleri
/// yeşildi ama ölçtükleri kod ekranda kullanılmıyordu.
fn akış_bloğu(kartlar: Vec<AnyElement>, metrik: &KolonMetriği) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        // Kaydıran kolonun flex çocuğu varsayılan olarak sıkışır. Üç akış
        // alt alta dizildiğinde ilk blok kabın tamamını alıyor, sonraki
        // ikisi sıfır yüksekliğe iniyor ve on bölüm hiç çizilmiyordu —
        // kaydırma sınırı da ilk akışın sonunda kalıyordu.
        .flex_shrink_0()
        .gap(metrik.kart_aralığı)
        .children(kartlar)
}

/// Bir yapılandırma bölümünün kartı.
///
/// Yüz `ORT-017` profilinden gelir: dolgu, köşe yarıçapı ve başlık
/// tipografisi burada hesaplanmaz.
fn bölüm_kartı(
    bölüm: TezgahBölümü,
    g: &ÇözülmüşTezgahGörünümü,
    t: &TezgahTokenları,
    çöz: &impl Fn(&YerelleştirmeAnahtarı) -> SharedString,
) -> AnyElement {
    let başlık = çöz(&bölüm.başlık);
    super::kart(g, t)
        .id(bölüm.kimlik)
        .role(Role::Group)
        .aria_label(başlık.clone())
        .tab_group()
        .tab_stop(false)
        .gap(g.kolonlar.kart_içi_aralık)
        .child(super::bölüm_başlığı(g, t, &başlık))
        .child(bölüm.içerik)
        .into_any_element()
}

#[cfg(test)]
mod testler {
    /// Kaydıran kolonun akış blokları sıkışmamalı.
    ///
    /// GPUI'de `overflow_y_scroll` bir kabın flex çocukları varsayılan
    /// olarak sıkışır: üç akış alt alta dizildiğinde ilki kabın tamamını
    /// alıyor, sonraki ikisi sıfır yüksekliğe iniyordu. Sonuç, on
    /// yapılandırma bölümünün (yapıştırma, port kapıları, hacim ve sayaç,
    /// içerik görünürlüğü, otomatik doldurma, doğrulama, seçici, sayısal
    /// adım, odak/kabul, saat dilimi) hiç çizilmemesiydi — kaydırma sınırı
    /// bile ilk akışın sonunda duruyordu, yani ekranda hiçbir ipucu yoktu.
    ///
    /// Yerleşim ölçüsü test API'sinden okunamadığı için bildirimi kaynakta
    /// tutuyoruz: kaldıran biri bu testi düşürür.
    #[test]
    fn akış_blokları_sıkışmaz() {
        let kaynak = include_str!("govde.rs");
        let gövde = kaynak
            .split_once("fn akış_bloğu(")
            .expect("akış_bloğu tanımı durur")
            .1
            .split_once("\n}")
            .expect("gövde kapanır")
            .0;
        assert!(
            gövde.contains(".flex_shrink_0()"),
            "akış bloğu sıkışmayı kapatmıyor: kaydıran kolonda ikinci ve \
             üçüncü akış sıfır yüksekliğe iner"
        );
    }
}

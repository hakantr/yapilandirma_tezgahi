//! Tezgâhın üst şeridi.
//!
//! Tasarımın `§6`'sı tezgâha **kendi kabuğunu** veriyor: solda başlık ve
//! bileşen seçici, sağda tema otoritesi ile çizim hedefi. Galerinin ağaç
//! menüsü, kategori sayfaları ve aile kartları burada yoktur — tezgâh
//! onların içine gömülü bir sayfa değil, kendi ekranıdır.
//!
//! Bileşen seçici bugün tek öğe taşır. Yine de açılır bir seçici olarak
//! durur: sözleşmesi biten her bileşen bu listeye eklenecek ve o gün
//! yerleşimin değişmesi gerekmeyecek.

use gpui::{Div, SharedString, Stateful, div, prelude::*};

use super::{TezgahTokenları, ÇözülmüşTezgahGörünümü};

/// Üst şeridin bir denetim öbeği: etiket üstte, denetim altında.
///
/// Ham `select` yoktur — GPUI'de yerleşik bir açılır kutu yok ve tezgâh
/// ikinci bir yüzey modeli kurmaz. Dışlayan seçimler kuşak olarak durur;
/// seçili değer zaten okunur.
pub fn kabuk_öbeği(
    g: &ÇözülmüşTezgahGörünümü,
    t: &TezgahTokenları,
    etiket: impl Into<SharedString>,
) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(g.önizleme_kabuğu.parça_aralığı)
        .child(super::eksen_etiketi(g, t, etiket))
}

/// Tezgâhın başlığı.
///
/// `h1` karşılığı. Altındaki açıklama satırı kaldırıldı: ekranın ne olduğu
/// başlıktan ve hemen yanındaki bileşen seçicisinden okunuyor, ikinci bir
/// cümle üst şeridi gereksiz yere iki satıra çıkarıyordu.
pub fn kabuk_başlığı(
    g: &ÇözülmüşTezgahGörünümü,
    t: &TezgahTokenları,
    başlık: impl Into<SharedString>,
) -> Div {
    div().flex().flex_col().child(
        super::stili_uygula(div(), &g.bölüm_başlığı)
            .text_color(t.ana_metin)
            .child(başlık.into()),
    )
}

/// Bileşen seçici · bugün tek öğe.
///
/// `Role::ComboBox` taşır: liste tek öğelik olsa da rol seçim kümesini
/// bildirir ve ekran okuyucu "bir seçenekten biri" der. Tek düğmeye
/// indirmek, listenin büyüyeceği gün rolü de değiştirmek demek olurdu.
pub fn bileşen_seçici(
    kimlik: impl Into<gpui::ElementId>,
    g: &ÇözülmüşTezgahGörünümü,
    t: &TezgahTokenları,
    seçili: impl Into<SharedString>,
    seçenek_sayısı: usize,
) -> Stateful<Div> {
    let seçili = seçili.into();
    super::stili_uygula(div(), &g.gövde)
        .id(kimlik)
        .role(gpui::Role::ComboBox)
        .aria_label(seçili.clone())
        .aria_expanded(false)
        .aria_size_of_set(seçenek_sayısı)
        .flex()
        .items_center()
        .gap(g.segment.yatay_dolgu)
        .h(g.anahtar_yüksekliği)
        .px(g.hap.yatay_dolgu)
        .rounded(g.segment.yarıçap)
        .border_1()
        .border_color(t.kenarlık)
        .bg(t.kağıt)
        .text_color(t.ana_metin)
        .child(seçili)
}

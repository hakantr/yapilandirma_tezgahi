//! Tezgâhın tek yüz kümesi.
//!
//! Tasarımın `§4.3` kuralı: birbirini dışlayan eksen **segment kuşağı**,
//! bağımsız bool **hap düğme** olur. Bu ayrım bütün bölümlerde aynıdır.
//!
//! **Köşe dili tek kademedir (kullanıcı kararı, Ağu 2026):** taslağın hap
//! (`999px`) kenarlı düğmeleri ve rozetleri ekranda köşeli seçeneklerden
//! ayrı bir dil kuruyordu; bütün düğme **ve rozet** yüzleri kart
//! kademesinden çizilir. `hap*` adları tarihseldir — bağımsız bool ekseni
//! anlatır, şekli değil.
//!
//! Yüzler ham ölçü ve ham renk taşımaz: ölçü [`ÇözülmüşTezgahGörünümü`]den,
//! renk [`TezgahTokenları`]ndan gelir (`ORT-004.ACC-001`). Köşe yarıçapı
//! `ORT-003 KutuŞekliTercihi`nden çözülmüştür; burada yerel yuvarlatma
//! denklemi kurulmaz.
//!
//! **Açıklama balonu bu listede yok.** `?` yardım yüzeyi `ORT-006`
//! `Araçİpucu` konağına kapılıdır ve doğrudan `deferred(anchored(..))`
//! fallback'i kurulmaz — kapalı bir kapıyı açıkmış gibi göstermek olurdu.

use gpui::{Div, SharedString, Stateful, div, prelude::*, px};

use super::{TezgahTokenları, ÇözülmüşTezgahGörünümü};

/// Metin stilini bir öğeye uygular.
///
/// Aile, boyut ve satır yüksekliği temadan çözülmüştür; yüz bunları
/// yeniden hesaplamaz. `pub`: kod paneli gibi kendi çerçevesini kuran ama
/// tipografiyi profilden alan çizimler de bunu kullanır.
pub fn stili_uygula(el: Div, stil: &gpui::TextStyle) -> Div {
    el.text_size(stil.font_size)
        .font_family(stil.font_family.clone())
        .text_color(stil.color)
}

/// Hap düğme · bağımsız bool eksen.
///
/// Seçili iken vurgu üçlüsü (kenar + zemin + metin) birlikte değişir; renk
/// tek kanal değildir, kenarlık da taşır.
pub fn hap(
    kimlik: impl Into<gpui::ElementId>,
    g: &ÇözülmüşTezgahGörünümü,
    t: &TezgahTokenları,
    etiket: impl Into<SharedString>,
    seçili: bool,
) -> Stateful<Div> {
    let etiket = etiket.into();
    hap_gövdesi(div().id(kimlik), g, t, seçili)
        .role(gpui::Role::Button)
        .aria_label(etiket.clone())
        .cursor_pointer()
        .hover(|el| el.text_color(t.ana_metin))
        .focus(|el| el.border_color(t.vurgu))
        .child(etiket)
}

/// Hap ölçüsü ve seçili/seçili değil renk üçlüsü.
///
/// Seçili iken kenar, zemin ve metin **birlikte** değişir: renk tek kanal
/// değildir, yüksek karşıtlık kipinde zemin farkı kaybolabilir.
fn hap_gövdesi(
    el: Stateful<Div>,
    g: &ÇözülmüşTezgahGörünümü,
    t: &TezgahTokenları,
    seçili: bool,
) -> Stateful<Div> {
    el.flex()
        .items_center()
        .px(g.hap.yatay_dolgu)
        .py(g.hap.dikey_dolgu)
        // Kart kademesi: köşe dili tek kademedir (kullanıcı kararı, modül
        // başındaki not). Hap yarıçapı düğme yüzlerinde kullanılmaz.
        .rounded(g.kart.yarıçap)
        .border_1()
        .text_size(g.gövde.font_size)
        .font_family(g.gövde.font_family.clone())
        .map(|el| {
            if seçili {
                el.border_color(t.vurgu)
                    .bg(t.vurgu_zemin)
                    .text_color(t.vurgu)
            } else {
                el.border_color(t.kenarlık)
                    .bg(t.kağıt)
                    .text_color(t.ikincil_metin)
            }
        })
}

/// Seçenek düğmesi · köşeli, satırı dolduran.
///
/// Tasarımın baskın düğme biçimi budur (`border-radius: 3px`, 228
/// kullanım). [`hap`] ile aynı gövdeyi paylaşır; fark artık yalnız
/// yerleşimdir (ortalanmış, geniş) — köşe dili kullanıcı kararıyla tek
/// kademedir (modül başındaki not).
pub fn seçenek(
    kimlik: impl Into<gpui::ElementId>,
    g: &ÇözülmüşTezgahGörünümü,
    t: &TezgahTokenları,
    etiket: impl Into<SharedString>,
    seçili: bool,
) -> Stateful<Div> {
    let etiket = etiket.into();
    hap_gövdesi(div().id(kimlik), g, t, seçili)
        .justify_center()
        .role(gpui::Role::Button)
        .aria_label(etiket.clone())
        .cursor_pointer()
        .hover(|el| el.text_color(t.ana_metin))
        .focus(|el| el.border_color(t.vurgu))
        .child(etiket)
}

/// Gösterge düğmesi · kısa, köşeli, tek satır.
///
/// Taslağın `.gos-etiket`i: `22px` yüksek, `3px` köşeli. Yuva varlığı,
/// gösterge ankrajı ve açıklama yüzeyi bu yüzü paylaşır — üçü de kısa
/// etiketli aç/kapa seçimleridir ve hap kenarları onları eksen
/// düğmeleriyle karıştırıyordu.
pub fn gösterge_düğmesi(
    kimlik: impl Into<gpui::ElementId>,
    g: &ÇözülmüşTezgahGörünümü,
    t: &TezgahTokenları,
    etiket: impl Into<SharedString>,
    seçili: bool,
) -> Stateful<Div> {
    let etiket = etiket.into();
    stili_uygula(div(), &g.rozet_metni)
        .id(kimlik)
        .role(gpui::Role::Button)
        .aria_label(etiket.clone())
        .flex()
        .items_center()
        .justify_center()
        .h(g.anahtar_yüksekliği)
        .px(g.hap.yatay_dolgu)
        .rounded(g.kart.yarıçap)
        .cursor_pointer()
        .map(|el| {
            if seçili {
                el.bg(t.vurgu_zemin).text_color(t.vurgu)
            } else {
                el.text_color(t.soluk)
            }
        })
        .hover(|el| el.text_color(t.ikincil_metin))
        .child(etiket)
}

/// Pasif gösterge düğmesi · kurulamayan bir seçenek.
///
/// [`gösterge_düğmesi`] ile **aynı** ölçü ve tipografi; tek fark rengin
/// devre dışı kutu rolünden gelmesi ve odak almaması. Ayrı bir yüzle
/// çizilseydi — bir süre öyleydi — yan yana duran iki düğme farklı yazı
/// tipi ve boyutta görünüyor, satır dağınık duruyordu.
pub fn pasif_gösterge_düğmesi(
    kimlik: impl Into<gpui::ElementId>,
    g: &ÇözülmüşTezgahGörünümü,
    etiket: impl Into<SharedString>,
    gerekçe: impl Into<SharedString>,
) -> Stateful<Div> {
    let etiket = etiket.into();
    stili_uygula(div(), &g.rozet_metni)
        .id(kimlik)
        .role(gpui::Role::Button)
        .aria_label(SharedString::from(format!("{etiket} — {}", gerekçe.into())))
        .tab_stop(false)
        .flex()
        .items_center()
        .justify_center()
        .h(g.anahtar_yüksekliği)
        .px(g.hap.yatay_dolgu)
        .rounded(g.kart.yarıçap)
        .text_color(g.devre_dışı.ön_plan)
        .child(etiket)
}

/// Durum hapı · düğme gövdesinde ama **etkileşmez**.
///
/// Bir eksenin seçili olup olmadığını gösterir; tıklanmaz, odak almaz ve
/// bu yüzden `Button` rolü taşımaz. Tıklanamayan bir öğeye buton rolü
/// vermek erişilebilir ağaçta yanlış söz verir.
pub fn durum_hapı(
    kimlik: impl Into<gpui::ElementId>,
    g: &ÇözülmüşTezgahGörünümü,
    t: &TezgahTokenları,
    seçili: bool,
) -> Stateful<Div> {
    hap_gövdesi(div().id(kimlik), g, t, seçili).tab_stop(false)
}

/// Pasif hap · gerekçe **zorunlu**.
///
/// Opaklık düşürülmez: pasiflik `ORT-004` devre dışı kutu rolüyle çizilir.
/// Kademeli görünürlük ayrı bir kavramdır ve `GörselOpaklıkKademesi` yalnız
/// onu kullanan profilde zorunludur.
pub fn hap_pasif(
    kimlik: impl Into<gpui::ElementId>,
    g: &ÇözülmüşTezgahGörünümü,
    etiket: impl Into<SharedString>,
    gerekçe: impl Into<SharedString>,
) -> Stateful<Div> {
    let etiket = etiket.into();
    let gerekçe = gerekçe.into();
    div()
        .id(kimlik)
        .role(gpui::Role::Button)
        .aria_label(SharedString::from(format!("{etiket} — {gerekçe}")))
        .tab_stop(false)
        .flex()
        .items_center()
        .px(g.hap.yatay_dolgu)
        .py(g.hap.dikey_dolgu)
        .rounded(g.kart.yarıçap)
        .border_1()
        .border_color(g.devre_dışı.kenarlık)
        .bg(g.devre_dışı.arka_plan)
        .text_color(g.devre_dışı.ön_plan)
        .text_size(g.gövde.font_size)
        .child(etiket)
}

/// Segment kuşağı · birbirini dışlayan eksen.
pub fn segment_kuşağı(
    kimlik: impl Into<gpui::ElementId>,
    g: &ÇözülmüşTezgahGörünümü,
    t: &TezgahTokenları,
    ad: impl Into<SharedString>,
) -> Stateful<Div> {
    kuşak(g, t)
        .id(kimlik)
        .role(gpui::Role::RadioGroup)
        .aria_label(ad.into())
}

/// Kuşak · yan yana duran denetimleri tek çerçevede toplar.
///
/// Rolsüzdür: birbirini dışlamayan denetimleri de taşıyabilir. Dışlayan
/// eksen için [`segment_kuşağı`] kullanılır.
pub fn kuşak(g: &ÇözülmüşTezgahGörünümü, t: &TezgahTokenları) -> Div {
    div()
        .flex()
        .items_center()
        .gap(g.segment.yatay_dolgu)
        .p(g.segment.dikey_dolgu)
        .rounded(g.segment.yarıçap)
        .border_1()
        .border_color(t.ince)
        .bg(t.yüzey)
}

/// Türetilmiş rozet · seçilemez.
///
/// Noktalı çerçeve seçilemezliğin görsel karşılığıdır; rozet odak almaz.
pub fn rozet(
    g: &ÇözülmüşTezgahGörünümü,
    t: &TezgahTokenları,
    metin: impl Into<SharedString>,
) -> Div {
    stili_uygula(
        div()
            .px(g.rozet.yatay_dolgu)
            .py(g.rozet.dikey_dolgu)
            // Kart kademesi: köşe dili tek kademedir (modül başındaki not);
            // rozeti ayıran şekil değil kesikli çerçevedir.
            .rounded(g.kart.yarıçap)
            .border_1()
            .border_dashed()
            .border_color(t.ince),
        &g.rozet_metni,
    )
    .text_color(t.soluk)
    .child(metin.into())
}

/// Küçük anahtar · kısa bool eksen.
pub fn küçük_anahtar(
    kimlik: impl Into<gpui::ElementId>,
    g: &ÇözülmüşTezgahGörünümü,
    t: &TezgahTokenları,
    etiket: impl Into<SharedString>,
    seçili: bool,
) -> Stateful<Div> {
    let etiket = etiket.into();
    div()
        .id(kimlik)
        .role(gpui::Role::Switch)
        .aria_label(etiket.clone())
        .aria_toggled(if seçili {
            gpui::Toggled::True
        } else {
            gpui::Toggled::False
        })
        .flex()
        .items_center()
        .h(g.anahtar_yüksekliği)
        .px(g.hap.yatay_dolgu)
        .rounded(g.kart.yarıçap)
        .text_size(g.rozet_metni.font_size)
        .font_family(g.rozet_metni.font_family.clone())
        .map(|el| {
            if seçili {
                el.bg(t.vurgu_zemin).text_color(t.vurgu)
            } else {
                el.bg(t.kağıt).text_color(t.ikincil_metin)
            }
        })
        .child(etiket)
}

/// Simge düğmesi · yalnız erişilebilir ad taşır.
///
/// Bağımsız eylem içindir. Birbirini dışlayan bir eksenin öğesi için
/// [`segment_simgesi`] kullanılır: `RadioGroup` içindeki bir `Button`
/// erişilebilir ağaçta seçim semantiğini kaybettirir.
pub fn simge_düğmesi(
    kimlik: impl Into<gpui::ElementId>,
    g: &ÇözülmüşTezgahGörünümü,
    t: &TezgahTokenları,
    ad: impl Into<SharedString>,
    seçili: bool,
) -> Stateful<Div> {
    simge_gövdesi(kimlik, g, t, seçili)
        .role(gpui::Role::Button)
        .aria_label(ad.into())
}

/// Segment simgesi · dışlayan bir eksenin öğesi.
///
/// [`segment_kuşağı`] içinde durur ve seçili olup olmadığını `aria_selected`
/// ile bildirir; ekran okuyucu "4'ün 2'si seçili" diyebilsin.
pub fn segment_simgesi(
    kimlik: impl Into<gpui::ElementId>,
    g: &ÇözülmüşTezgahGörünümü,
    t: &TezgahTokenları,
    ad: impl Into<SharedString>,
    seçili: bool,
) -> Stateful<Div> {
    simge_gövdesi(kimlik, g, t, seçili)
        .role(gpui::Role::RadioButton)
        .aria_label(ad.into())
        .aria_selected(seçili)
}

/// Kare simge kutusunun ölçüsü ve seçili zemini.
fn simge_gövdesi(
    kimlik: impl Into<gpui::ElementId>,
    g: &ÇözülmüşTezgahGörünümü,
    t: &TezgahTokenları,
    seçili: bool,
) -> Stateful<Div> {
    div()
        .id(kimlik)
        .flex()
        .items_center()
        .justify_center()
        .size(g.simge_düğmesi)
        .rounded(g.segment.yarıçap)
        .when(seçili, |el| el.bg(t.vurgu_zemin))
}

/// Liste öğesi · yüzer bir listenin dışlayan satırı.
///
/// Punto, yazı ailesi ve biçim listeleri aynı yapıdadır: bir tetikleyicinin
/// açtığı seçim kümesi. Hap değildir — hapın çerçevesi ve dolgusu bir
/// satır listesinde ağır durur — ama rolü segment öğesiyle aynıdır:
/// `RadioButton` ve `aria_selected`.
pub fn liste_öğesi(
    kimlik: impl Into<gpui::ElementId>,
    g: &ÇözülmüşTezgahGörünümü,
    t: &TezgahTokenları,
    etiket: impl Into<SharedString>,
    seçili: bool,
) -> Stateful<Div> {
    let etiket = etiket.into();
    stili_uygula(div(), &g.eksen_etiketi)
        .id(kimlik)
        .role(gpui::Role::RadioButton)
        .aria_label(etiket.clone())
        .aria_selected(seçili)
        .flex()
        .items_center()
        .px(g.segment.yatay_dolgu)
        .rounded(g.segment.yarıçap)
        .cursor_pointer()
        .map(|el| {
            if seçili {
                el.bg(t.vurgu_zemin).text_color(t.vurgu)
            } else {
                el.text_color(t.ana_metin)
            }
        })
}

/// Kart · bir yapılandırma bölümünün gövdesi.
pub fn kart(g: &ÇözülmüşTezgahGörünümü, t: &TezgahTokenları) -> Div {
    div()
        .flex()
        .flex_col()
        // Flex çocuğu varsayılan olarak içeriğinden küçülemez: uzun ve
        // boşluksuz bir tanımlayıcı (hata adı, tam nitelikli tür) kartı
        // genişletip kolonu taşırırdı. GPUI'de `overflow-wrap` yok; taşmayı
        // sarmayla değil, küçülmeye izin vererek keseriz.
        .min_w(px(0.))
        .p(g.kart.yatay_dolgu)
        .rounded(g.kart.yarıçap)
        .border_1()
        .border_color(t.ince)
        .bg(t.yüzey)
}

/// Bölüm başlığı.
///
/// Büyük harfe çevirme **dizede** yapılır: `text-transform` karşılığı GPUI'de
/// yoktur ve harf aralığı da tipografi kararına dayandırılmaz.
pub fn bölüm_başlığı(
    g: &ÇözülmüşTezgahGörünümü, t: &TezgahTokenları, metin: &str
) -> Div {
    stili_uygula(div(), &g.bölüm_başlığı)
        .text_color(t.ikincil_metin)
        .child(SharedString::from(metin.to_uppercase()))
}

/// Eksen etiketi · bir denetim grubunun üstünde durur.
pub fn eksen_etiketi(
    g: &ÇözülmüşTezgahGörünümü,
    t: &TezgahTokenları,
    metin: impl Into<SharedString>,
) -> Div {
    stili_uygula(div(), &g.eksen_etiketi)
        .text_color(t.soluk)
        .child(metin.into())
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{GaleriTeması, TezgahGörünümProfili, galeri_paleti, galeri_teması};
    use gpui_bilesenleri::{TanımKimliği, TemaKipi};
    use std::sync::Arc;

    fn çözülmüş() -> ÇözülmüşTezgahGörünümü {
        let kimlik = gpui_bilesenleri::GörünümProfiliKimliği(
            TanımKimliği::denetimli(Arc::from("galeri.tezgah"), Arc::from("tasarım")).unwrap(),
        );
        crate::paleti_kur(galeri_paleti(GaleriTeması::Kağıt, TemaKipi::Açık));
        TezgahGörünümProfili::tasarım(kimlik)
            .çöz(&galeri_teması())
            .expect("tasarım profili çözülür")
    }

    /// Yüzler ölçüyü profilden alır: hap yarıçapı kart yarıçapından büyüktür
    /// çünkü biri `Hap`, diğeri `Köşeli` kademesidir.
    #[test]
    fn yüzler_ölçüyü_profilden_alır() {
        let g = çözülmüş();
        assert!(g.hap.yarıçap > g.kart.yarıçap);
        assert_eq!(g.kart.yatay_dolgu, g.kart.dikey_dolgu);
        assert!(g.anahtar_yüksekliği > gpui::px(0.));
    }

    /// Pasif yüz `ORT-004` devre dışı rolünü kullanır; opaklık düşürmez.
    #[test]
    fn pasif_yüz_devre_dışı_rolünü_kullanır() {
        let g = çözülmüş();
        assert_ne!(g.devre_dışı.ön_plan, g.devre_dışı.arka_plan);
    }

    /// Çizim katmanı ham renk ve ham ölçü taşımaz.
    ///
    /// `F1` görsel kabulünün değişmezi: yüzler ile gövde rengi
    /// [`TezgahTokenları`]ndan, ölçüyü [`ÇözülmüşTezgahGörünümü`]nden okur.
    /// Ham `rgb(..)` bir palet kaçağı, ham `px(<sayı>)` bir profil kaçağıdır
    /// — ikisi de tema ve metin ölçeğinden bağımsız kalır. Yalnız `px(0.)`
    /// serbesttir: bu bir ölçü değil, flex taşma sıfırlamasıdır.
    #[test]
    fn çizim_katmanı_ham_renk_ve_ölçü_taşımaz() {
        for (ad, kaynak) in [
            ("yuzler.rs", include_str!("yuzler.rs")),
            ("govde.rs", include_str!("govde.rs")),
        ] {
            let gövde = kaynak
                .split_once("#[cfg(test)]")
                .map_or(kaynak, |(önce, _)| önce);
            assert!(!gövde.contains("rgb("), "{ad} ham renk taşıyor");
            for (sıra, satır) in gövde.lines().enumerate() {
                let ham_ölçü = satır
                    .replace("px(0.)", "")
                    .split("px(")
                    .skip(1)
                    .any(|kalan| kalan.starts_with(|c: char| c.is_ascii_digit()));
                assert!(!ham_ölçü, "{ad}:{} ham ölçü taşıyor: {satır}", sıra + 1);
            }
        }
    }

    /// Tezgâh yüzleri `text_xs`/`text_sm` gibi ham ölçü taşımaz.
    ///
    /// `sergiler.rs` hem eski galeri sergilerini hem tezgâhı taşıyor, o
    /// yüzden dosya bütünüyle ölçülemiyor. Bu test tezgâhın kendi
    /// yüzlerini adla sayıyor: ham ölçü `ORT-004` metin ölçeğinden
    /// etkilenmez ve `%200`de o satır küçük kalır — ekran yarı ölçekli bir
    /// karma olur. Liste yeni bir tezgâh yüzü eklendiğinde büyür.
    #[test]
    fn tezgâh_yüzleri_ham_ölçü_taşımaz() {
        const YÜZLER: &[&str] = &[
            "kapalı_eksen",
            "etiketli_alan",
            "maske_tanımı",
            "maske_özeti",
            "sayı_biçimi_şeridi",
            "yer_tutucu_satırı",
            "saat_dilimi_satırı",
            "köşe_kaydırma_çubuğu",
            "değer_durumu",
            "olay_akışı",
            "yuva_görünürlük_notu",
        ];
        let kaynak = include_str!("../sergiler.rs");
        let mut şu_an: Option<&str> = None;
        for (sıra, satır) in kaynak.lines().enumerate() {
            if let Some(kalan) = satır.split("fn ").nth(1)
                && satır.trim_start().starts_with(['f', 'p'])
            {
                let ad = kalan.split('(').next().unwrap_or_default();
                şu_an = YÜZLER.contains(&ad).then_some(
                    YÜZLER
                        .iter()
                        .find(|y| **y == ad)
                        .copied()
                        .unwrap_or_default(),
                );
            }
            let Some(yüz) = şu_an else { continue };
            assert!(
                !satır.contains(".text_xs()") && !satır.contains(".text_sm()"),
                "sergiler.rs:{} — `{yüz}` ham ölçü taşıyor: {satır}",
                sıra + 1
            );
        }
    }

    /// Her yüzün çizimde bir kullanımı vardır.
    ///
    /// Kullanılmayan bir yüz sessizce ölüdür: `pub` olduğu için derleyici
    /// uyarmaz, ama tezgâh onu çizmiyorsa tasarımın o parçası ekranda yok
    /// demektir. `F1` görsel kabulü tam da bunu ölçer.
    #[test]
    fn ölü_yüz_yoktur() {
        let çizim = concat!(include_str!("../sergiler.rs"), include_str!("govde.rs"));
        let yüzler: Vec<&str> = include_str!("yuzler.rs")
            .lines()
            .filter_map(|satır| satır.strip_prefix("pub fn "))
            .filter_map(|kalan| kalan.split('(').next())
            .collect();
        assert!(yüzler.len() >= 12, "yüz kümesi beklenenden küçük");
        for yüz in yüzler {
            assert!(
                çizim.contains(&format!("crate::{yüz}("))
                    || çizim.contains(&format!("super::{yüz}(")),
                "`{yüz}` yüzü hiçbir yerde çizilmiyor"
            );
        }
    }

    /// `§II.12/8` tezgâhtaki her tıklanabilir öğe rol ve ad taşır.
    ///
    /// `.id()` tek başına yetmez: rolsüz bir `div` erişilebilir ağaçta
    /// tıklanabilir görünmez ve adsız bir düğme ekran okuyucuya boş kutu
    /// olarak okunur. Bu yüzden tezgâh çizimindeki `on_click` **yalnız** bir
    /// yüz üzerinden kurulabilir; yüzler rolü ve `aria_label`ı kendileri
    /// verir.
    ///
    /// Tezgâh denetimi, **tercihi değiştiren** denetimdir: tıklama gövdesinde
    /// `tezgahı_değiştir` geçiyorsa o denetim tezgâha aittir. Ayrım fonksiyon
    /// adına değil bu çağrıya dayanır, çünkü `sergiler.rs` tezgâhın yanında
    /// ORT/KAB laboratuvarlarını ve aile sergilerini de taşır; onlar kendi
    /// durumlarını değiştirir ve `§II.12` tezgâh kontrol listesidir. Ad
    /// listesi tutmak da sürdürülemezdi: yeni bir sergi eklendiğinde
    /// listeyi güncellemeyi unutmak testi sessizce kör ederdi.
    ///
    /// Tezgâhta bugün tek bir bileşen sunuluyor — metin girişi. Sözleşmesi
    /// biten her yeni bileşen tezgâha taşındığında denetimleri de bu
    /// yüzlerden geçer.
    #[test]
    fn tezgâh_tıklanabilirleri_yüz_üzerinden_kurulur() {
        // Yerel sarmalayıcılar ve doğrudan yüz çağrıları. İkisi de geçerli:
        // sarmalayıcı yüzü çağırır, doğrudan çağrı zaten yüzün kendisidir.
        const YÜZLER: [&str; 11] = [
            "tercih_düğmesi(",
            "simge_düğmesi(",
            "segment_simgesi(",
            "çalışma_anahtarı(",
            "pasif_simge_düğmesi(",
            "liste_öğesi(",
            "crate::hap(",
            "crate::küçük_anahtar(",
            "gösterge_düğmesi(",
            "crate::seçenek(",
            "geniş_seçenek(",
        ];

        let satırlar: Vec<&str> = include_str!("../sergiler.rs").lines().collect();
        let mut denetim = 0usize;
        for (sıra, satır) in satırlar.iter().enumerate() {
            if !satır.contains(".on_click(") {
                continue;
            }
            let gövde = satırlar[sıra..(sıra + 6).min(satırlar.len())].join("\n");
            if !gövde.contains("tezgahı_değiştir") {
                continue;
            }
            denetim += 1;
            let pencere = satırlar[sıra.saturating_sub(30)..=sıra].join("\n");
            assert!(
                YÜZLER.iter().any(|yüz| pencere.contains(yüz))
                    || pencere.contains("aria_label")
                    || pencere.contains(".role("),
                "sergiler.rs:{} rolsüz tezgâh denetimi: {satır}",
                sıra + 1
            );
        }
        // Sayı küçük görünür ama doğrudur: tezgâh denetimlerinin çoğu
        // `tercih_düğmesi`/`simge_düğmesi` yardımcılarından geçer ve
        // `tezgahı_değiştir` çağrısı **onların** gövdesindedir. Burada
        // sayılanlar, yardımcıların kendisi ile listelerin kendi tıklama
        // kuran satırlarıdır.
        assert!(denetim >= 5, "tezgâh denetimi taranmadı: {denetim}");
    }

    /// Bölüm başlığı büyük harfe dizede çevrilir.
    #[test]
    fn bölüm_başlığı_dizede_büyütülür() {
        assert_eq!("§7 değer türü".to_uppercase(), "§7 DEĞER TÜRÜ");
    }
}

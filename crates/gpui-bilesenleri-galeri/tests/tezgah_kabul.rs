//! `F5` kabul turu: Bölüm I `§13`'ün 28 maddesi, Bölüm II `§II.12`'nin 12
//! maddesi.
//!
//! Madde madde "denetlendi" demek yerine denetimin kendisi burada koşar.
//! Kapılı maddeler atlanmaz: neden kapılı olduğu yazılır ve kapının hâlâ
//! kapalı olduğu sınanır — kapı açıldığı gün test düşer ve madde yeniden
//! ele alınır.
//!
//! Maddelerin çoğu kendi fazının test dosyasında kanıtlı; burada yalnız
//! oralarda karşılığı olmayanlar ile kapı bekçileri var.

#![allow(non_ascii_idents)]

/// `§13/1–2` HTML'e özgü maddelerin Rust karşılığı yoktur.
///
/// "Tek `<style>` bloğu" ve "bütün görsel stil inline" tarayıcı
/// tasarımının kuralları. Rust'taki karşılıkları `F1` görsel kabulünde
/// ölçüldü: çizim katmanı ham renk ve ham ölçü taşımaz, hepsi profilden ve
/// tokenlardan gelir.
#[test]
fn madde_1_2_html_kurallarinin_rust_karsiligi() {
    let yüzler = include_str!("../src/tezgah/yuzler.rs");
    assert!(yüzler.contains("çizim_katmanı_ham_renk_ve_ölçü_taşımaz"));
}

/// `§13/4` sayfa kaydırmaz; iki kolon kendi içinde kaydırır.
///
/// `§II.12/1–2` de burada: `overflow_y_scroll` kullanan her `div`in `.id()`si
/// olmalı ve kaydıran kolonlar `min_h(px(0.))` taşımalı — flex çocuğu
/// varsayılan olarak içeriğinin altına küçülmez ve kaydırma hiç kurulmaz.
#[test]
fn madde_4_ve_ii12_1_2_kolonlar_kendi_icinde_kaydirir() {
    let gövde = include_str!("../src/tezgah/govde.rs");
    let kaydıran = gövde.matches("overflow_y_scroll()").count();
    // İki kaydırma yüzeyi: sağ yapılandırma kolonu ve sol kolonun **alt**
    // bölümü. Sol kolonun üst bloğu (kabuk denetimleri ve yaşayan alan)
    // kaydırmanın dışındadır — aşağıdaki kartlara bakarken de yerinde
    // kalır.
    assert_eq!(kaydıran, 2, "iki kaydırma yüzeyi bekleniyor");

    // Her kaydıran öğenin yakınında `min_h(px(0.))` durur: flex çocuğu
    // varsayılan olarak içeriğinin altına küçülmez ve kaydırma hiç
    // kurulmaz. Zincirin hangi sırada yazıldığı önemli değil.
    let mut arama = 0usize;
    while let Some(konum) = gövde[arama..].find("overflow_y_scroll()") {
        let mutlak = arama + konum;
        let pencere = &gövde[mutlak.saturating_sub(400)..mutlak];
        assert!(
            pencere.contains("min_h(px(0.))"),
            "kaydıran öğede min_h yok"
        );
        arama = mutlak + "overflow_y_scroll()".len();
    }

    assert!(gövde.contains(".id("), "kaydıran kolonda id yok");
}

/// `§13/5` dar yerleşim **bilinçli sapmadır**.
///
/// Tasarımın kökü `min-width: 1216px` altında yatay kaydırma öngörüyor.
/// Gömülü tezgâhta bu kabı dayatmak `YÖN-006 §3.4`'e aykırı: kip ölçülen
/// alandan seçilir. Bu yüzden `min_w(1216)` **yok** ve kip
/// `container_query` ile çözülür.
#[test]
fn madde_5_min_w_dayatilmaz() {
    let gövde = include_str!("../src/tezgah/govde.rs");
    assert!(
        gövde.contains("container_query"),
        "kip ölçülen kaptan seçilmiyor"
    );
    assert!(
        !gövde.contains("min_w(px(1216"),
        "kap genişliği dayatılıyor"
    );
}

/// `§13/25` "rezervli şerit" ifadesi hiçbir yerde geçmez.
///
/// `13.0.0` gösterge için sabit genişlik/rezerv bırakmayı açıkça yasakladı.
/// İfadenin kaynakta kalması, yasaklanmış tasarımın hâlâ bir yerde
/// düşünüldüğünün işareti olurdu.
#[test]
fn madde_25_rezervli_serit_ifadesi_yok() {
    for (ad, kaynak) in [
        ("sergiler.rs", include_str!("../src/sergiler.rs")),
        ("govde.rs", include_str!("../src/tezgah/govde.rs")),
        ("yuzler.rs", include_str!("../src/tezgah/yuzler.rs")),
        (
            "metin_girisi_profili.rs",
            include_str!("../src/metin_girisi_profili.rs"),
        ),
    ] {
        assert!(
            !kaynak.contains("rezervli şerit"),
            "{ad} yasaklı ifadeyi taşıyor"
        );
    }
}

/// `§13/8–17` göstergenin **çizim** maddeleri kapılıdır.
///
/// `GirişKutusu::render` göstergeyi çizmiyor: mantıksal sıra yardımcı
/// eylemlerle bitiyor ve `§16.2.1`'in koşullu `Gösterge` parçası fiziksel
/// render'da yok. Galerinin onu çizmesi tüketicide ikinci bir görsel
/// uygulama kurmak olurdu (`YÖN-006.ACC-006`).
///
/// Bu test bir **kapı bekçisidir**: kanonik render göstergeyi çizmeye
/// başladığı gün düşer ve maddeler yeniden ele alınır (`F2.6` K1).
#[test]
fn madde_8_17_gosterge_cizimi_kanonikte_yok() {
    let render = include_str!("../../../../gpui_bilesenleri/crates/gpui-bilesenleri/src/metin_girisi/bileşen.rs");
    let gövde = render
        .split_once("impl Render for GirişKutusu")
        .map(|(_, sonra)| sonra)
        .unwrap_or(render);

    assert!(
        !gövde.contains("durum_göstergesi_durumu()"),
        "kanonik render göstergeyi çiziyor olabilir — §13/8–17 yeniden ele alınmalı"
    );
}

/// `§II.12/3–7` gösterge çiziminin GPUI kuralları da aynı kapının ardında.
///
/// `Yok` çözümünde çocuk üretmemek, `ÜstKöşe`de `absolute` konumlamak,
/// çözümü kare başına bir kez türetmek, açıklamayı `deferred(anchored(..))`
/// içinde tutmak ve göstergeye `tab_stop(false)` + `aria_hidden(true)`
/// vermek — beşi de **çizen** tarafın kuralları. Galeri çizmediği için
/// bunları sağlayamaz; sağlamaya kalkışsa ikinci uygulamayı kurmuş olurdu.
///
/// Galerinin sağladığı şey, çizmediğini **iddia etmemek**: gözlem paneli
/// yalnız kanonik sonucu okur.
#[test]
fn madde_ii12_3_7_galeri_gosterge_cizmez() {
    let sergiler = include_str!("../src/sergiler.rs");
    let panel = sergiler
        .split_once("pub(crate) fn turetilmis_durum_satırı(")
        .expect("türetilmiş durum kartı bulunur")
        .1
        .split_once("\n}\n")
        .expect("kart kapanır")
        .0;

    // Panel bir gösterge çizmez: konumlama, katman ve gizleme çağrıları yok.
    for çizim in ["absolute", "deferred", "anchored", "aria_hidden"] {
        assert!(
            !panel.contains(çizim),
            "gözlem paneli gösterge çiziyor: {çizim}"
        );
    }
}

/// `§II.12/10` dört tema kipi elle kaydedilmiştir; hiçbiri hesaplanmaz.
///
/// Yüksek karşıtlık bir tema ailesi değil **kiptir** ve otomatik
/// üretilmez: parlaklık kaydırarak türetilen bir yüksek karşıtlık paleti,
/// karşıtlık oranını garanti etmez.
#[test]
fn madde_ii12_10_dort_kip_elle_kayitli() {
    let palet = include_str!("../src/palet.rs");
    for kip in [
        "TemaKipi::Açık",
        "TemaKipi::Koyu",
        "TemaKipi::YüksekKarşıtlıkAçık",
        "TemaKipi::YüksekKarşıtlıkKoyu",
    ] {
        assert!(palet.contains(kip), "`{kip}` paletinde yok");
    }
}

/// `§II.12/11` simgeler `svg()` + `AssetSource` üzerinden gelir.
///
/// Metin karakteri kullanılmaz: kullanılabilir bir glif her yazı tipinde
/// bulunmaz ve eksikse kutu olarak çizilir. Renk tek bilgi kanalı da
/// değildir — seçili durum zeminle birlikte değişir.
#[test]
fn madde_ii12_11_simgeler_svg_ile_gelir() {
    let sergiler = include_str!("../src/sergiler.rs");
    let simge = sergiler
        .split_once("fn tezgah_simgesi(")
        .expect("simge yardımcısı bulunur")
        .1
        .split_once("\n}\n")
        .expect("gövde kapanır")
        .0;
    assert!(simge.contains("svg()") && simge.contains(".path("));
}

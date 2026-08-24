//! `F4` B, C ve D bölümleri.
//!
//! Üçünün ortak özelliği: hiçbiri `GirişYapılandırması`'na yazılmaz.
//! B port kapılarını raporlar, C türetilmiş durumu okur, D önizleme
//! bağlamıdır. Bu yüzden testler "kanonik alana çevrildi mi" diye değil,
//! "koda sızmadı mı" ve "yapısal sınır korunuyor mu" diye sorar.

#![allow(non_ascii_idents)]

use gpui_bilesenleri_galeri::TezgahTercihleri;

/// `B`/`C`/`D` bölümleri kod paneline **girmez**.
///
/// Kod paneli yalnız A bölümünü sunar. Port durumu, türetilmiş durum ve
/// tema bağlamı oraya yazılsaydı, kopyalayan kişiye platform yeteneğini
/// yapılandırma satırıyla açabileceği sözü verilmiş olurdu.
#[test]
fn b_c_d_bolumleri_koda_yazilmaz() {
    let kod = TezgahTercihleri::default().kod();
    for yasak in [
        "port",
        "GirişÖzelDurumu",
        "görsel_durum",
        "yazı_ailesi",
        "MerkeziFallback",
    ] {
        assert!(!kod.contains(yasak), "`{yasak}` koda sızdı:\n{kod}");
    }
}

/// `C` bölümü seçilebilir eksen taşımaz.
///
/// `GirişÖzelDurumu` ve `ORT-004` erişim durumu modelden türer; bir hap ile
/// seçilebilir olsalardı türetilmiş bir değeri yapılandırılabilir gibi
/// göstermiş olurduk. Yapısal kanıt: kartın çizimi hiçbir tercih
/// değiştirmiyor.
#[test]
fn turetilmis_durum_karti_secilemez() {
    let kaynak = include_str!("../src/sergiler.rs");
    let kart = kaynak
        .split_once("pub(crate) fn turetilmis_durum_satırı(")
        .expect("kart bulunur")
        .1
        .split_once("\n}\n")
        .expect("kart gövdesi kapanır")
        .0;

    assert!(
        !kart.contains("tezgahı_değiştir"),
        "türetilmiş durum kartı tercih değiştiriyor"
    );
    assert!(
        !kart.contains("on_click"),
        "türetilmiş durum kartı tıklanabilir denetim taşıyor"
    );
}

/// `B` bölümü port varlığını seçilebilir bir tercih gibi göstermez.
///
/// Port varlığı platformun bildirimidir. `ACC-005`: port yoksa kontrol
/// pasif ve gerekçeli kalır — kart gizlenmez, çünkü gizlenen kapı o yolun
/// hiç olmadığı izlenimini verir.
#[test]
fn port_karti_secilemez_ve_gizlenmez() {
    let kaynak = include_str!("../src/sergiler.rs");
    let kart = kaynak
        .split_once("pub(crate) fn port_satırı(")
        .expect("kart bulunur")
        .1
        .split_once("\n}\n")
        .expect("kart gövdesi kapanır")
        .0;

    assert!(
        !kart.contains("tezgahı_değiştir"),
        "port kartı tercih değiştiriyor"
    );
    assert!(!kart.contains("on_click"), "port kartı tıklanabilir");
    // Dört port da her koşulda satır üretir: koşullu `when` ile gizlenmiyor.
    for port in [
        "otomatik_doldurma",
        "saat_dilimi",
        "imleç",
        "uzak_doğrulama",
    ] {
        assert!(kart.contains(port), "`{port}` kartta yok");
    }
}

/// `D` bölümü sessiz font fallback yapmaz.
///
/// `ACC-034` katalog dışı aileyi `MerkeziFallback` rolüne düşürüyor, ama o
/// rol kanonik kodda yok. Uyarı bu yüzden rolün adını değil **durumu**
/// yazar ve aile seçicisinin yanında durur: taslakta ayrı bir katalog kartı
/// yok, aile seçimi üst şeritte.
#[test]
fn katalog_disi_aile_sessizce_cizilmez() {
    let kaynak = include_str!("../src/sergiler.rs");
    let şerit = kaynak
        .split_once("fn görünüm_ekseni(")
        .expect("üst şeridin ikinci satırı bulunur")
        .1
        .split_once("\n}\n")
        .expect("gövde kapanır")
        .0;

    assert!(
        şerit.contains("KİTAPLIK_AİLELERİ.contains"),
        "katalog üyeliği hiç sorulmuyor"
    );
    assert!(
        şerit.contains("katalog dışı"),
        "katalog dışı aile sessizce çiziliyor"
    );
    // Rol adı rozet değeri olamaz: olmayan bir kademeyi varmış gibi gösterir.
    assert!(
        !şerit.contains("türetilmiş_rozet(\"MerkeziFallback"),
        "olmayan rol rozet değeri olarak yazılmış"
    );
}

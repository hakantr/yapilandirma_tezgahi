//! Galerinin gömdüğü yazı tipi yüzleri sağlam mı?
//!
//! Kaydın kendisi burada ölçülemez: GPUI test platformu `NoopTextSystem`
//! kullanır, `add_fonts` boş geçer ve `all_font_names()` boş döner. Bu yüzden
//! test yalnız gömülü baytların geçerli bir yazı tipi kapsayıcısı olduğunu ve
//! `KİTAPLIK_AİLELERİ` içindeki her ailenin bir yüz dosyasıyla karşılandığını
//! doğrular. Ailelerin çözülmesi çalışan uygulamada doğrulanır.

#![allow(non_ascii_idents)]

use gpui_bilesenleri_galeri::{
    KİTAPLIK_AİLELERİ, aile_gösterilebilir_mi, gömülü_yüzler, kayıtlı_yüz_sayısı,
};

/// TrueType ve OpenType kapsayıcılarının imzaları.
const İMZALAR: [[u8; 4]; 4] = [[0x00, 0x01, 0x00, 0x00], *b"true", *b"OTTO", *b"ttcf"];

#[test]
fn gomulu_yuzler_gecerli_yazi_tipi_dosyasi() {
    for (sıra, yüz) in gömülü_yüzler().iter().enumerate() {
        assert!(
            yüz.len() > 10_000,
            "{sıra}. yüz beklenmedik ölçüde küçük: {} bayt",
            yüz.len()
        );
        let imza: [u8; 4] = yüz[..4].try_into().expect("dosya en az dört bayt");
        assert!(
            İMZALAR.contains(&imza),
            "{sıra}. yüz geçerli bir yazı tipi kapsayıcısı değil: {imza:?}"
        );
    }
}

#[test]
fn kayitli_yuz_sayisi_beklenen() {
    // Beş aile, her biri altı statik yüz: ince, düz, koyu — üçü de dik ve
    // eğik.
    assert_eq!(kayıtlı_yüz_sayısı(), 30);
}

#[test]
fn her_kitaplik_ailesinin_yuzu_var() {
    // `include_bytes!` yolları derleme zamanında çözüldüğü için burada
    // dosya adlarını yeniden okumak yerine ailenin boşluksuz karşılığını
    // kaynak metninde ararız: bir aile listeye eklenip yüzü unutulursa
    // bu test düşer.
    let kaynak = include_str!("../src/yazi_tipleri.rs");
    for aile in KİTAPLIK_AİLELERİ {
        let dosya_öneki = aile.replace(' ', "");
        assert!(
            kaynak.contains(&dosya_öneki),
            "kitaplık ailesinin gömülü yüzü yok: {aile}"
        );
    }
}

#[test]
fn yedek_adlar_listede_gosterilmez() {
    // `TextSystem::all_font_names` platform listesine sabit bir yedek yığını
    // ekler. Bu adlar kurulu olduklarını göstermez; WASM'de hiçbiri çözülmez
    // ve seçildiklerinde sessizce yedeğe düşerler.
    for ad in [
        "Helvetica",
        "Segoe UI",
        "Arial",
        "DejaVu Sans",
        ".SystemUIFont",
    ] {
        assert!(
            !aile_gösterilebilir_mi(ad),
            "gpui yedek adı listede kalmamalı: {ad}"
        );
    }
    // Nokta ile başlayan iç adlar da seçim değildir.
    assert!(!aile_gösterilebilir_mi(".ZedSans"));
    // Gerçek bir aile elenmemeli.
    assert!(aile_gösterilebilir_mi("Menlo"));
}

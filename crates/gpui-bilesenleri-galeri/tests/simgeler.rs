//! Galerinin `ORT-016` simge kaydı ve GPUI varlık kaynağı doğrulaması.

#![allow(non_ascii_idents)]

use gpui::AssetSource as _;
use gpui_bilesenleri::{SimgeBoyutu, SimgeKimliği, SimgeVaryantı, SimgeYönü, Simgeİsteği};
use gpui_bilesenleri_galeri::{GaleriVarlıkKaynağı, galeri_simge_kataloğu};

fn istek(kimlik: &str) -> Simgeİsteği {
    Simgeİsteği {
        kimlik: SimgeKimliği::yeni(kimlik).unwrap(),
        varyant: SimgeVaryantı::Olağan,
        boyut: SimgeBoyutu::Normal,
        yön: SimgeYönü::Değişmez,
        tema_sürümü: 1,
    }
}

#[test]
fn yardimci_eylem_simgeleri_katalogda_cozulur() {
    let katalog = galeri_simge_kataloğu();
    for kimlik in [
        "input.clear",
        "input.search",
        "input.reveal",
        "input.reveal-off",
        "input.picker",
    ] {
        let çözülmüş = katalog
            .çöz(&istek(kimlik), false)
            .unwrap_or_else(|hata| panic!("{kimlik} çözülemedi: {hata:?}"));
        // Fallback düzeyi 0 olmalı: gerçek varlık bulunmalı, eksik simgeye
        // düşmemeli.
        assert_eq!(çözülmüş.fallback_düzeyi, 0, "{kimlik} eksik simgeye düştü");
    }
}

#[test]
fn cozulen_varlik_gpui_varlik_kaynagindan_yuklenir() {
    let katalog = galeri_simge_kataloğu();
    let kaynak = GaleriVarlıkKaynağı;
    for kimlik in [
        "input.clear",
        "input.search",
        "input.reveal",
        "input.picker",
    ] {
        let çözülmüş = katalog.çöz(&istek(kimlik), false).unwrap();
        let yol = match çözülmüş.çizim {
            gpui_bilesenleri::ÇözülmüşSimgeÇizimi::TekTonlu(tek) => tek.varlık,
            gpui_bilesenleri::ÇözülmüşSimgeÇizimi::İkiTonlu(iki) => iki.birincil.varlık,
        };
        let baytlar = kaynak
            .load(yol.as_ref())
            .unwrap_or_else(|hata| panic!("{yol} yüklenemedi: {hata:?}"))
            .unwrap_or_else(|| panic!("{yol} varlık kaynağında yok"));
        let svg = String::from_utf8_lossy(&baytlar);
        assert!(svg.starts_with("<svg"), "{yol} geçerli SVG değil");
        assert!(svg.contains("<path"), "{yol} yol içermiyor");
    }
}

#[test]
fn varlik_kaynagi_bilinmeyen_yolu_bulamaz() {
    assert!(GaleriVarlıkKaynağı.load("yok.svg").unwrap().is_none());
    // Beş yardımcı eylem yuvası + üç `§16.2` gösterge glifi.
    assert_eq!(GaleriVarlıkKaynağı.list("").unwrap().len(), 8);
}

/// `§16.2.3` gösterge glifleri katalogda çözülür ve üçü ayrı varlıktır.
///
/// Bileşen bu üç kimliği `GirişKutusu::gösterge_simge_kimliği` ile çözer;
/// katalog onları tanımazsa `simge_varlığı` `None` döner ve gösterge
/// **sessizce çizilmez**. Sessiz kayıp, sorunu olan bir alanın hiçbir işaret
/// taşımaması demek olurdu — bu yüzden kayıt burada ölçülür.
///
/// Üç glif ayrı olmalı: renk tek bilgi kanalı değildir (`ORT-009`).
#[test]
fn gosterge_glifleri_katalogda_cozulur_ve_ayridir() {
    let katalog = galeri_simge_kataloğu();
    let varlık = |kimlik: &str| {
        let çözülmüş = katalog
            .çöz(&istek(kimlik), false)
            .unwrap_or_else(|hata| panic!("{kimlik} çözülmeli: {hata:?}"));
        match çözülmüş.çizim {
            gpui_bilesenleri::ÇözülmüşSimgeÇizimi::TekTonlu(tek) => tek.varlık,
            gpui_bilesenleri::ÇözülmüşSimgeÇizimi::İkiTonlu(iki) => iki.birincil.varlık,
        }
    };

    let hata = varlık("input.status-error");
    let uyarı = varlık("input.status-warning");
    let bilgi = varlık("input.status-info");

    assert_ne!(hata, uyarı);
    assert_ne!(uyarı, bilgi);
    assert_ne!(hata, bilgi);

    // Temizleme eylemiyle aynı glif olmamalı: aynı çizimi hem "temizle"
    // eylemi hem "hata" durumu için kullanmak ikisini tek anlama indirir.
    assert_ne!(hata, varlık("input.clear"));

    // Varlık kaynağı üçünü de sunmalı; katalog kaydı tek başına yetmez.
    let kaynak = GaleriVarlıkKaynağı;
    for ad in [hata, uyarı, bilgi] {
        assert!(
            kaynak.load(&ad).unwrap().is_some(),
            "varlık kaynağı {ad} baytlarını sunmalı"
        );
    }
}

//! Galerinin gerçek `ORT-016` snapshot ve GPUI bağdaştırıcı tüketicisi.

#![allow(non_ascii_idents)]

use std::sync::Arc;

use gpui::AssetSource as _;
use gpui_bilesenleri::{
    SimgeBoyutTercihi, SimgeGerekliliği, SimgeGörselBiçimi, SimgeÇizimBağlamı, SimgeÇizimPlanı,
    SimgeÇözümAkıbeti, Simgeİsteği, TemaKipi, ÇözülmüşSimge, ÇözülmüşYazıYönü,
};
use gpui_bilesenleri_galeri::{
    GaleriVarlıkKaynağı, galeri_simge_kimliği, galeri_simge_çizim_bağlamı,
};
use gpui_bilesenleri_temel::SimgeRolü;

fn istek(kimlik: &str) -> Simgeİsteği {
    Simgeİsteği::yeni(
        galeri_simge_kimliği(kimlik).expect("galeri input kimliği geçerlidir"),
        SimgeGörselBiçimi::Çizgisel,
        SimgeBoyutTercihi::default(),
        SimgeRolü::Olağan,
        None,
        ÇözülmüşYazıYönü::SoldanSağa,
        SimgeGerekliliği::İsteğeBağlı,
        TemaKipi::Açık,
    )
}

fn çöz(bağlam: &SimgeÇizimBağlamı, kimlik: &str) -> Arc<ÇözülmüşSimge> {
    match bağlam.snapshot.çöz(&istek(kimlik)) {
        SimgeÇözümAkıbeti::TamÇözüldü(simge) => simge,
        SimgeÇözümAkıbeti::YedekleÇözüldü { makbuz, .. } => {
            panic!("{kimlik} yedeğe düşmemeli: {makbuz:?}")
        }
        SimgeÇözümAkıbeti::ÇizimYok => panic!("{kimlik} çizimsiz kalmamalı"),
    }
}

fn varlık_yolu(bağlam: &SimgeÇizimBağlamı, kimlik: &str) -> gpui::SharedString {
    let simge = çöz(bağlam, kimlik);
    let başvuru = match simge.çizim() {
        SimgeÇizimPlanı::TekTonlu(tek) => &tek.varlık,
        SimgeÇizimPlanı::İkiTonlu(iki) => &iki.birincil.varlık,
    };
    bağlam
        .bağdaştırıcı
        .svg_yolu(başvuru)
        .unwrap_or_else(|| panic!("{kimlik} için GPUI varlık yolu bulunmalı"))
}

#[test]
fn yardimci_eylem_simgeleri_snapshotta_tam_cozulur() {
    let bağlam = galeri_simge_çizim_bağlamı();
    for kimlik in [
        "input.clear",
        "input.search",
        "input.reveal",
        "input.reveal-off",
        "input.picker",
    ] {
        let _ = çöz(&bağlam, kimlik);
    }
}

#[test]
fn cozulen_varlik_gpui_bagdastiricisindan_ve_kaynagindan_yuklenir() {
    let bağlam = galeri_simge_çizim_bağlamı();
    let kaynak = GaleriVarlıkKaynağı;
    for kimlik in [
        "input.clear",
        "input.search",
        "input.reveal",
        "input.picker",
    ] {
        let yol = varlık_yolu(&bağlam, kimlik);
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
    assert!(galeri_simge_kimliği("başka.clear").is_err());
}

/// `§16.2.3` üç gösterge glifini aynı doğrulanmış snapshot ve bağdaştırıcı
/// üzerinden çözer; renk dışında ayrı varlık kimliği de korunur.
#[test]
fn gosterge_glifleri_snapshotta_cozulur_ve_ayridir() {
    let bağlam = galeri_simge_çizim_bağlamı();
    let hata = varlık_yolu(&bağlam, "input.status-error");
    let uyarı = varlık_yolu(&bağlam, "input.status-warning");
    let bilgi = varlık_yolu(&bağlam, "input.status-info");

    assert_ne!(hata, uyarı);
    assert_ne!(uyarı, bilgi);
    assert_ne!(hata, bilgi);
    assert_ne!(hata, varlık_yolu(&bağlam, "input.clear"));

    let kaynak = GaleriVarlıkKaynağı;
    for ad in [&hata, &uyarı, &bilgi] {
        assert!(
            kaynak.load(ad.as_ref()).unwrap().is_some(),
            "varlık kaynağı {ad} baytlarını sunmalı"
        );
    }
}

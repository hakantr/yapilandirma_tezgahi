//! Galerinin gerçek GPUI penceresinde çizildiğini doğrular.
//!
//! `YÖN-006.ACC-007` sergi hatası galeriyi düşürmez. Bu dosya çizim yolunu
//! baştan sona koşturur: yerleşim, yaşayan `BİL-010` alanları, simge
//! çözümü ve aile sayfaları. Çizim panikleri burada yakalanır; gözle
//! görülene kadar beklemez.

#![allow(non_ascii_idents)]

use gpui::TestAppContext;
use gpui_bilesenleri_galeri::TezgahDeğerKipi;
use gpui_bilesenleri_galeri::{
    BİL_AİLELERİ, GaleriHedefi, GaleriUygulaması, KAB_AİLELERİ, ORT_AİLELERİ,
    bileşen_tuş_bağlarını_kur,
};

fn galeri_çiz(bağlam: &mut TestAppContext, hedef: GaleriHedefi, aile: Option<&str>) {
    bağlam.update(|bağlam| bileşen_tuş_bağlarını_kur(bağlam));
    let (uygulama, görsel) = bağlam.add_window_view(move |_, _| GaleriUygulaması::hedef(hedef));
    if let Some(aile) = aile {
        let aile = aile.to_owned();
        görsel.update(|_, bağlam| {
            uygulama.update(bağlam, |uygulama, _| {
                assert!(
                    uygulama.model.aileyi_aç(aile.as_str()),
                    "aile açılamadı: {aile}"
                );
            });
        });
    }
    // Bekleyen etkileri (notify → yeniden çizim) boşaltır; çizim yolundaki
    // panik burada yüzeye çıkar.
    görsel.run_until_parked();
}

#[gpui::test]
fn genel_bakis_masaustunde_cizilir(bağlam: &mut TestAppContext) {
    galeri_çiz(bağlam, GaleriHedefi::Masaüstü, None);
}

#[gpui::test]
fn genel_bakis_wasmde_cizilir(bağlam: &mut TestAppContext) {
    galeri_çiz(bağlam, GaleriHedefi::Wasm, None);
}

#[gpui::test]
fn metin_girisi_ailesi_cizilir(bağlam: &mut TestAppContext) {
    // `BİL-010` yaşayan alanları, maske şablonları ve simgeleriyle çizilir.
    galeri_çiz(bağlam, GaleriHedefi::Masaüstü, Some("BİL-010"));
}

#[gpui::test]
fn butun_bilesen_aileleri_cizilir(bağlam: &mut TestAppContext) {
    for aile in BİL_AİLELERİ {
        galeri_çiz(bağlam, GaleriHedefi::Masaüstü, Some(aile));
    }
}

#[gpui::test]
fn butun_ortak_ve_kabuk_aileleri_cizilir(bağlam: &mut TestAppContext) {
    for aile in ORT_AİLELERİ.iter().chain(KAB_AİLELERİ.iter()) {
        galeri_çiz(bağlam, GaleriHedefi::Wasm, Some(aile));
    }
}

#[gpui::test]
fn tezgah_her_deger_turunde_cizilir(bağlam: &mut TestAppContext) {
    // Tür süzgeci render yolunda da çalışmalı: bölüm listesi türe göre
    // değişiyor ve çizim sırasında hiçbir türde panik üretmemeli.
    bağlam.update(|bağlam| bileşen_tuş_bağlarını_kur(bağlam));
    let (uygulama, görsel) =
        bağlam.add_window_view(move |_, _| GaleriUygulaması::hedef(GaleriHedefi::Masaüstü));
    görsel.update(|_, bağlam| {
        uygulama.update(bağlam, |uygulama, _| {
            assert!(uygulama.model.aileyi_aç("BİL-010"), "tezgâh açılamadı");
        });
    });
    for tür in [
        TezgahDeğerKipi::Metin,
        TezgahDeğerKipi::Tamsayı,
        TezgahDeğerKipi::Ondalık,
        TezgahDeğerKipi::Tarih,
    ] {
        görsel.update(|_, bağlam| {
            uygulama.update(bağlam, |uygulama, bağlam| {
                uygulama.tezgahı_değiştir(|t| t.değer_türü = tür, bağlam);
            });
        });
        görsel.run_until_parked();
    }
}

#[gpui::test]
fn tezgah_erisilebilir_metin_olceginde_cizilir(bağlam: &mut TestAppContext) {
    // `%200` metin ölçeğinde iki kolon eşiği aşılır ve gövde tek kolona
    // iner; kırpma yerine erişilebilir yerleşim kipine geçilmeli.
    bağlam.update(|bağlam| bileşen_tuş_bağlarını_kur(bağlam));
    let (uygulama, görsel) =
        bağlam.add_window_view(move |_, _| GaleriUygulaması::hedef(GaleriHedefi::Masaüstü));
    görsel.update(|_, bağlam| {
        uygulama.update(bağlam, |uygulama, bağlam| {
            assert!(uygulama.model.aileyi_aç("BİL-010"), "tezgâh açılamadı");
            uygulama.tezgahı_değiştir(|t| t.tema.metin_ölçeği = 2.0, bağlam);
        });
    });
    görsel.run_until_parked();
}

//! `yapilandirma_tezgahi_live_host_integration` kanıt kolları.
//!
//! `BİL-010 §29` fallible kuruluşun exact kanalları, `§14` varsayılan
//! sağlayıcı akıbeti ve statik yasak-kalıntı taramaları. Kuruluş çağrıları
//! gerçek `ORT-002` köküyle, gerçek GPUI penceresinde koşar.

#![allow(non_ascii_idents)]

use gpui::TestAppContext;
use gpui_bilesenleri::{
    BileşenKimliği, Değer, GirişKuruluşHatası, GirişMaskesi, GirişYapılandırmaHatası,
    GirişYapılandırması, MetinGirişMaskesi, TanımKimliği, UzunlukSınırı, UzunlukSınırıDavranışı,
    VarsayılanDeğer, VarsayılanDeğerHatası,
};
use gpui_bilesenleri_temel::{
    BağlamSürümü, CanlıBağlamDamgası, GüvenliMetin, MetinDamgası, UnicodeVeYerelMetinHizmetleri,
    ÖrnekKimliğiFabrikası,
};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

struct BoşKonak;

impl gpui::Render for BoşKonak {
    fn render(
        &mut self,
        _: &mut gpui::Window,
        _: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        gpui::div()
    }
}

fn fabrika() -> ÖrnekKimliğiFabrikası {
    ÖrnekKimliğiFabrikası::yeni_süreç_kapsamı().expect("test kimlik kapsamı")
}

fn unicode_kök() -> Arc<UnicodeVeYerelMetinHizmetleri> {
    UnicodeVeYerelMetinHizmetleri::yerlesik(fabrika())
}

fn damga(kök: &UnicodeVeYerelMetinHizmetleri, sürüm: u64) -> MetinDamgası {
    kök.metin_damgası_fabrikası().damga(CanlıBağlamDamgası {
        bağlam: fabrika().sonraki().expect("test bağlam kimliği"),
        sürüm: BağlamSürümü(sürüm),
    })
}

fn bileşen(ad: &'static str) -> BileşenKimliği {
    BileşenKimliği {
        tanım: TanımKimliği::denetimli(Arc::from("galeri.test"), Arc::from(ad))
            .expect("test bileşen tanımı geçerlidir"),
        örnek: fabrika().sonraki().expect("test örnek kimliği"),
    }
}

fn tema() -> Arc<gpui_bilesenleri::TemaAnlıkGörüntüsü> {
    gpui_bilesenleri_galeri::galeri_teması()
}

/// `§29` yapısal ve teknik kuruluş hataları ayrı exact kanallardan döner.
///
/// Yapısal hata capability adımına ulaşmadan `Yapılandırma(rapor)` üretir;
/// teknik hazırlama başarısızlığı (damga tükenmesi) `Teknik { hata, rapor }`
/// üretir ve exact `GirişHatası` korunur. İki kolda da entity yoktur.
#[gpui::test]
fn kurulus_yapisal_ve_teknik_hatalari_ayirir(bağlam: &mut TestAppContext) {
    let kök = unicode_kök();
    let (_konak, görsel) = bağlam.add_window_view(|_, _| BoşKonak);

    // Yapısal: sıfır grafem sınırı geçersizdir. Maske de kuruludur ve damga
    // tükenmiştir — yapısal kapı yerindeyse capability adımına ulaşılmaz ve
    // akıbet yine `Yapılandırma` olur.
    let mut yapısal = GirişYapılandırması::tek_satırlı_metin();
    yapısal.uzunluk_sınırı = Some(UzunlukSınırı {
        en_fazla_grafem: 0,
        davranış: UzunlukSınırıDavranışı::Reddet,
    });
    yapısal.maske = Some(GirişMaskesi::Metin(MetinGirişMaskesi {
        desen: "(000) 000".into(),
        yer_tutucu_grafemi: "_".into(),
        sabitleri_göster: true,
    }));
    let akıbet = görsel.update(|pencere, bağlam| {
        gpui_bilesenleri::GirişKutusu::kur(
            bileşen("yapısal"),
            kök.clone(),
            damga(&kök, u64::MAX),
            yapısal,
            "",
            tema(),
            pencere,
            bağlam,
        )
    });
    match akıbet {
        Err(GirişKuruluşHatası::Yapılandırma(rapor)) => assert!(
            rapor
                .hatalar
                .contains(&GirişYapılandırmaHatası::GeçersizUzunlukSınırı),
            "{:?}",
            rapor.hatalar
        ),
        diğer => panic!("yapısal hata exact `Yapılandırma` kanalından dönmeli: {diğer:?}"),
    }

    // Teknik: yapılandırma geçerli, damga tükenmiş — maske derlemesi damga
    // ister ve exact `SürümTükendi` teknik kanaldan döner.
    let mut teknik = GirişYapılandırması::tek_satırlı_metin();
    teknik.maske = Some(GirişMaskesi::Metin(MetinGirişMaskesi {
        desen: "(000) 000".into(),
        yer_tutucu_grafemi: "_".into(),
        sabitleri_göster: true,
    }));
    let akıbet = görsel.update(|pencere, bağlam| {
        gpui_bilesenleri::GirişKutusu::kur(
            bileşen("teknik"),
            kök.clone(),
            damga(&kök, u64::MAX),
            teknik,
            "",
            tema(),
            pencere,
            bağlam,
        )
    });
    match akıbet {
        Err(GirişKuruluşHatası::Teknik { hata, rapor }) => {
            assert!(
                matches!(hata, gpui_bilesenleri::GirişHatası::SürümTükendi(_)),
                "exact teknik hata korunur: {hata:?}"
            );
            assert!(
                rapor.hatalar.is_empty(),
                "teknik kola gelindiğinde yapısal rapor temizdir"
            );
        }
        diğer => panic!("damga tükenmesi exact `Teknik` kanalından dönmeli: {diğer:?}"),
    }
}

/// `§14` varsayılan **sağlayıcı** hatası entity'yi öldürmez: kuruluş `Ok`
/// döner, akıbet `varsayılan_değer_hatası` ekseninde exact typed durur ve
/// metin boş kalır ("var olan durur").
#[gpui::test]
fn varsayilan_saglayici_hatasi_entityyi_oldurmez(bağlam: &mut TestAppContext) {
    let kök = unicode_kök();
    let (_konak, görsel) = bağlam.add_window_view(|_, _| BoşKonak);

    let mut y = GirişYapılandırması::tek_satırlı_metin();
    y.varsayılan_değer =
        VarsayılanDeğer::Sağlayıcı(
            Arc::new(|| Err(VarsayılanDeğerHatası::SağlayıcıBaşarısız)),
        );
    let sonuç = görsel
        .update(|pencere, bağlam| {
            gpui_bilesenleri::GirişKutusu::kur(
                bileşen("sağlayıcı-hata"),
                kök.clone(),
                damga(&kök, 0),
                y,
                "",
                tema(),
                pencere,
                bağlam,
            )
        })
        .expect("sağlayıcı hatası kuruluşu düşürmez");
    assert_eq!(
        sonuç.varsayılan_değer_hatası,
        Some(VarsayılanDeğerHatası::SağlayıcıBaşarısız)
    );
    görsel.update(|_, bağlam| {
        assert_eq!(sonuç.bileşen.read(bağlam).metin(), "");
    });
}

/// Açık başlangıç metni ve varsayılan sağlayıcı ayrı eksenlerdir: metin
/// varken sağlayıcı **hiç çağrılmaz**; boş açılışta sağlayıcı değeri
/// uygulanır.
#[gpui::test]
fn acik_baslangic_metni_saglayiciyi_cagirmaz(bağlam: &mut TestAppContext) {
    let kök = unicode_kök();
    let (_konak, görsel) = bağlam.add_window_view(|_, _| BoşKonak);

    let çağrıldı = Arc::new(AtomicBool::new(false));
    let sağlayıcı = {
        let çağrıldı = Arc::clone(&çağrıldı);
        VarsayılanDeğer::Sağlayıcı(Arc::new(move || {
            çağrıldı.store(true, Ordering::SeqCst);
            Ok(Some(Değer::Metin(GüvenliMetin::yeni(
                "sağlayıcıdan",
                false,
                true,
            ))))
        }))
    };

    // Açık başlangıç metni: sağlayıcı çağrılmaz, metin korunur.
    let mut y = GirişYapılandırması::tek_satırlı_metin();
    y.varsayılan_değer = sağlayıcı.clone();
    let sonuç = görsel
        .update(|pencere, bağlam| {
            gpui_bilesenleri::GirişKutusu::kur(
                bileşen("açık-metin"),
                kök.clone(),
                damga(&kök, 0),
                y,
                "elle-girilmiş",
                tema(),
                pencere,
                bağlam,
            )
        })
        .expect("geçerli yapılandırma kurulur");
    assert!(
        !çağrıldı.load(Ordering::SeqCst),
        "açık metin varken sağlayıcı hiç çağrılmamalı"
    );
    assert!(sonuç.varsayılan_değer_hatası.is_none());
    görsel.update(|_, bağlam| {
        assert_eq!(sonuç.bileşen.read(bağlam).metin(), "elle-girilmiş");
    });

    // Boş açılış: sağlayıcı çağrılır ve değeri uygulanır.
    let mut y = GirişYapılandırması::tek_satırlı_metin();
    y.varsayılan_değer = sağlayıcı;
    let sonuç = görsel
        .update(|pencere, bağlam| {
            gpui_bilesenleri::GirişKutusu::kur(
                bileşen("boş-açılış"),
                kök.clone(),
                damga(&kök, 0),
                y,
                "",
                tema(),
                pencere,
                bağlam,
            )
        })
        .expect("geçerli yapılandırma kurulur");
    assert!(
        çağrıldı.load(Ordering::SeqCst),
        "boş açılışta sağlayıcı koşar"
    );
    assert!(sonuç.varsayılan_değer_hatası.is_none());
    görsel.update(|_, bağlam| {
        assert_eq!(sonuç.bileşen.read(bağlam).metin(), "sağlayıcıdan");
    });
}

/// Statik yasak-kalıntı taraması: eski API çağrısı, manuel bağlam literal'i,
/// doğrudan yerel alan mutasyonu ve sahte sealed impl kaynakta sıfırdır.
#[test]
fn yasak_kalintilar_kaynakta_sifir() {
    let kaynaklar = [
        ("src/lib.rs", include_str!("../src/lib.rs")),
        ("src/galeri.rs", include_str!("../src/galeri.rs")),
        (
            "src/metin_hizmetleri.rs",
            include_str!("../src/metin_hizmetleri.rs"),
        ),
        (
            "src/metin_girisi_profili.rs",
            include_str!("../src/metin_girisi_profili.rs"),
        ),
        (
            "src/metin_girisi_tezgahi.rs",
            include_str!("../src/metin_girisi_tezgahi.rs"),
        ),
        ("src/paneller.rs", include_str!("../src/paneller.rs")),
        ("src/sergiler.rs", include_str!("../src/sergiler.rs")),
        ("src/tezgah.rs", include_str!("../src/tezgah.rs")),
        (
            "src/tezgah/arayuz.rs",
            include_str!("../src/tezgah/arayuz.rs"),
        ),
        (
            "src/tezgah/govde.rs",
            include_str!("../src/tezgah/govde.rs"),
        ),
        (
            "src/tezgah/kabuk.rs",
            include_str!("../src/tezgah/kabuk.rs"),
        ),
        (
            "src/tezgah/profil.rs",
            include_str!("../src/tezgah/profil.rs"),
        ),
        (
            "src/tezgah/tokenlar.rs",
            include_str!("../src/tezgah/tokenlar.rs"),
        ),
        (
            "src/tezgah/yerlesim.rs",
            include_str!("../src/tezgah/yerlesim.rs"),
        ),
        (
            "src/tezgah/yuzler.rs",
            include_str!("../src/tezgah/yuzler.rs"),
        ),
        ("src/onboarding.rs", include_str!("../src/onboarding.rs")),
        ("src/palet.rs", include_str!("../src/palet.rs")),
        ("src/simgeler.rs", include_str!("../src/simgeler.rs")),
        (
            "src/yazi_tipleri.rs",
            include_str!("../src/yazi_tipleri.rs"),
        ),
        ("tests/yon006.rs", include_str!("yon006.rs")),
        ("tests/tezgah.rs", include_str!("tezgah.rs")),
        (
            "tests/host_entegrasyonu.rs",
            include_str!("host_entegrasyonu.rs"),
        ),
        (
            "masaustu/src/main.rs",
            include_str!("../../gpui-bilesenleri-galeri-masaustu/src/main.rs"),
        ),
        (
            "masaustu/src/platform.rs",
            include_str!("../../gpui-bilesenleri-galeri-masaustu/src/platform.rs"),
        ),
        (
            "wasm/src/lib.rs",
            include_str!("../../gpui-bilesenleri-galeri-wasm/src/lib.rs"),
        ),
        (
            "wasm/src/platform.rs",
            include_str!("../../gpui-bilesenleri-galeri-wasm/src/platform.rs"),
        ),
    ];
    // Tarama dizgileri birleştirmeyle kurulur ki bu dosyanın kendisi
    // kalıntı olarak eşleşmesin.
    let yasaklar = [
        format!("GirişKutusu::{}", "yeni"),
        format!(".yerel.{}", "saat_dilimi"),
        format!(".yerel.{}", "sürüm"),
        format!("impl İletiÇözümleyicisi {}", "for"),
        format!("İletiÇözüm{}", "Profili {"),
    ];
    for (ad, kaynak) in kaynaklar {
        for yasak in &yasaklar {
            assert!(
                !kaynak.contains(yasak.as_str()),
                "{ad}: yasak kalıntı bulundu: {yasak}"
            );
        }
        // Manuel yerel bağlam literal'i yasaktır; dönüş tipini izleyen
        // fonksiyon gövdesi açılışı (`-> Tip {`) literal değildir, ayıklanır.
        let literal_adayı = format!("YerelMetin{}", "Bağlamı {");
        let mut arama = 0usize;
        while let Some(konum) = kaynak[arama..].find(literal_adayı.as_str()) {
            let mutlak = arama + konum;
            let öncesi = &kaynak[..mutlak];
            assert!(
                öncesi.ends_with("-> "),
                "{ad}: manuel `YerelMetinBağlamı` literal'i bulundu"
            );
            arama = mutlak + literal_adayı.len();
        }
    }
}

/// Uygulama kökü başına tek hizmet kökü: `ORT-002` edinme kökü ile `ORT-021`
/// kütük/hizmet mühürleri galeri kaynağında yalnız bileşim-kökü modülünde
/// (`metin_hizmetleri.rs`) kurulur.
#[test]
fn hizmet_kokleri_yalniz_bilesim_kokunde_kurulur() {
    let bileşim_kökü_dışı = [
        ("src/lib.rs", include_str!("../src/lib.rs")),
        ("src/galeri.rs", include_str!("../src/galeri.rs")),
        (
            "src/metin_girisi_profili.rs",
            include_str!("../src/metin_girisi_profili.rs"),
        ),
        (
            "src/metin_girisi_tezgahi.rs",
            include_str!("../src/metin_girisi_tezgahi.rs"),
        ),
        ("src/paneller.rs", include_str!("../src/paneller.rs")),
        ("src/sergiler.rs", include_str!("../src/sergiler.rs")),
        ("src/tezgah.rs", include_str!("../src/tezgah.rs")),
        (
            "src/tezgah/arayuz.rs",
            include_str!("../src/tezgah/arayuz.rs"),
        ),
        (
            "src/tezgah/govde.rs",
            include_str!("../src/tezgah/govde.rs"),
        ),
        (
            "src/tezgah/kabuk.rs",
            include_str!("../src/tezgah/kabuk.rs"),
        ),
        (
            "src/tezgah/profil.rs",
            include_str!("../src/tezgah/profil.rs"),
        ),
        (
            "src/tezgah/tokenlar.rs",
            include_str!("../src/tezgah/tokenlar.rs"),
        ),
        (
            "src/tezgah/yerlesim.rs",
            include_str!("../src/tezgah/yerlesim.rs"),
        ),
        (
            "src/tezgah/yuzler.rs",
            include_str!("../src/tezgah/yuzler.rs"),
        ),
        ("src/onboarding.rs", include_str!("../src/onboarding.rs")),
        ("src/palet.rs", include_str!("../src/palet.rs")),
        ("src/simgeler.rs", include_str!("../src/simgeler.rs")),
        (
            "src/yazi_tipleri.rs",
            include_str!("../src/yazi_tipleri.rs"),
        ),
        (
            "masaustu/src/main.rs",
            include_str!("../../gpui-bilesenleri-galeri-masaustu/src/main.rs"),
        ),
        (
            "masaustu/src/platform.rs",
            include_str!("../../gpui-bilesenleri-galeri-masaustu/src/platform.rs"),
        ),
        (
            "wasm/src/lib.rs",
            include_str!("../../gpui-bilesenleri-galeri-wasm/src/lib.rs"),
        ),
        (
            "wasm/src/platform.rs",
            include_str!("../../gpui-bilesenleri-galeri-wasm/src/platform.rs"),
        ),
    ];
    for (ad, kaynak) in bileşim_kökü_dışı {
        for kurucu in [
            "UnicodeVeYerelMetinHizmetleri::yerlesik(",
            "İletiÇözümHizmeti::muhurle(",
            "İletiKataloğuKütüğü::muhurle(",
        ] {
            assert!(
                !kaynak.contains(kurucu),
                "{ad}: hizmet kökü bileşim kökü dışında kuruluyor: {kurucu}"
            );
        }
    }
    // Bileşim kökü modülü gerçekten kuruyor; tarama boşa dönmüyor.
    let kök_modülü = include_str!("../src/metin_hizmetleri.rs");
    assert!(kök_modülü.contains("UnicodeVeYerelMetinHizmetleri::yerlesik("));
    assert!(kök_modülü.contains("İletiÇözümHizmeti::muhurle("));
}

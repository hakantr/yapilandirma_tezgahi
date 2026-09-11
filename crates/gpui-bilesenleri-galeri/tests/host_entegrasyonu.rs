//! `yapilandirma_tezgahi_live_host_integration` kanıt kolları.
//!
//! `BİL-010 §29` fallible kuruluşun exact kanalları, `§14` varsayılan
//! sağlayıcı akıbeti ve statik yasak-kalıntı taramaları. Kuruluş çağrıları
//! gerçek `ORT-002` köküyle, gerçek GPUI penceresinde koşar.

#![allow(non_ascii_idents)]

use gpui::TestAppContext;
use gpui_bilesenleri::{
    BileşenKimliği, BiçimHizmetHazırlığı, BiçimHizmetKökü, Değer, GirişKuruluşHatası,
    GirişKutusuKurucusu, GirişKısıtBileşimKöküExt, GirişMaskesi, GirişYapılandırmaHatası,
    GirişYapılandırması, MetinGirişMaskesi, TanımKimliği, UzunlukSınırı, UzunlukSınırıDavranışı,
    VarsayılanDeğer, VarsayılanDeğerHatası, YerelMetinBağlamı, ÇözülmüşGirişKısıtları,
};
use gpui_bilesenleri_temel::{
    BağlamSürümü, BileşimKökü, CanlıBağlamDamgası, GüvenliMetin, MetinDamgası,
    PaylaşılanMetinDilimi, UnicodeVeYerelMetinHizmetleri, Utf8ParçaAkışı, ÖrnekKimliğiFabrikası,
};
use std::num::{NonZeroU32, NonZeroU64};
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

fn metni_oku(dilim: &PaylaşılanMetinDilimi) -> String {
    let mut metin = String::with_capacity(dilim.utf8_bayt_uzunluğu());
    dilim
        .utf8_parçalarını_ziyaret(&mut |baytlar| {
            metin.push_str(std::str::from_utf8(baytlar).expect("mühürlü test UTF-8'i"));
            Utf8ParçaAkışı::Devam
        })
        .expect("mühürlü test dilimi okunur");
    metin
}

fn açık_metni_oku(alan: &gpui_bilesenleri::GirişKutusu) -> String {
    metni_oku(
        &alan
            .açık_metin()
            .expect("host entegrasyon alanı açık roldedir"),
    )
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

/// Tarihsel host nöbetlerini kanonik K01 kurucusu ve yaşayan ORT-001
/// bileşim kökü üzerinden çalıştırır. Maskeli alanların cold ORT-008 planı
/// da gerçek hizmet çiftine bağlanır; test eski serbest kurucuya kaçamaz.
#[allow(clippy::too_many_arguments)]
fn giriş_kur(
    kimlik: BileşenKimliği,
    unicode_hizmetleri: Arc<UnicodeVeYerelMetinHizmetleri>,
    ilk_metin_damgası: MetinDamgası,
    yerel_bağlam: YerelMetinBağlamı,
    yapılandırma: GirişYapılandırması,
    başlangıç_metni: impl Into<String>,
    tema: Arc<gpui_bilesenleri::TemaAnlıkGörüntüsü>,
    pencere: &mut gpui::Window,
    bağlam: &mut gpui::App,
) -> Result<gpui_bilesenleri::GirişKuruluşSonucu, GirişKuruluşHatası> {
    let yapılandırma = Arc::new(yapılandırma);
    let mut kök = BileşimKökü::edin(bağlam)
        .unwrap_or_else(|| BileşimKökü::kur(bağlam).expect("test App bileşim kökü kurulur"));
    let (_, kuruluş) = kök
        .giriş_kısıt_alanını_kur(ÇözülmüşGirişKısıtları::bağımsız(
            yapılandırma.giriş_türü.clone(),
        ))
        .expect("test giriş kısıt alanı kurulur");
    let mut kurucu = GirişKutusuKurucusu::yeni(Arc::clone(&yapılandırma))
        .simge_çizim_hizmeti(gpui_bilesenleri_galeri::galeri_simge_hizmeti(bağlam));
    if let Ok(Some(istek)) = kurucu.biçim_planı_isteği(&yerel_bağlam) {
        let hazırlık = BiçimHizmetHazırlığı::denetimli(
            Arc::from([istek]),
            NonZeroU32::new(1).expect("sabit plan tavanı"),
            NonZeroU64::new(gpui_bilesenleri_galeri::GALERİ_SAHİPLİ_METİN_UTF8_TAVANI as u64)
                .expect("sabit canonical bayt tavanı"),
        )
        .expect("tek plan hazırlığı geçerlidir");
        let biçim_kökü = BiçimHizmetKökü::yerleşik(hazırlık)
            .expect("host testi için gerçek ORT-008 kökü kurulur");
        kurucu = kurucu.biçim_hizmetleri(biçim_kökü.hizmetleri());
    }
    kurucu.kısıtları_çöz(kuruluş).kur(
        kimlik,
        unicode_hizmetleri,
        ilk_metin_damgası,
        yerel_bağlam,
        başlangıç_metni,
        tema,
        pencere,
        bağlam,
    )
}

/// Bu dosyadaki alanlar açık metindir; `GirişDurumu` kabuğunu özel alana
/// uzanmadan, kanonik varyant ve erişiciler üzerinden açar.
fn açık_durum(alan: &gpui_bilesenleri::GirişKutusu) -> &gpui_bilesenleri::GirişÇekirdeği {
    match alan
        .açık_durum()
        .expect("host entegrasyon alanı açık roldedir")
    {
        gpui_bilesenleri::GirişDurumu::Açık(durum) => durum,
        gpui_bilesenleri::GirişDurumu::Gizli(_) => {
            panic!("açık alan testi gizli durum kabuğu üretmemeli")
        }
    }
}

/// Testlerin enjekte yerel bağlamı; üretim köküyle aynı `tr/latn/gregory/UTC`
/// hattından, fabrika üretimiyle kurulur.
fn yerel(kök: &UnicodeVeYerelMetinHizmetleri) -> YerelMetinBağlamı {
    let motor = kök.motor();
    kök.yerel_bağlam_fabrikası().bağlam(
        CanlıBağlamDamgası {
            bağlam: fabrika().sonraki().expect("test bağlam kimliği"),
            sürüm: BağlamSürümü(1),
        },
        motor.dil_etiketi("tr").expect("`tr` tanınır"),
        motor
            .numaralandırma_sistemi("latn")
            .expect("`latn` tanınır"),
        motor.takvim("gregory").expect("`gregory` tanınır"),
        motor.saat_dilimi("UTC").expect("`UTC` tanınır"),
    )
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
        giriş_kur(
            bileşen("yapısal"),
            kök.clone(),
            damga(&kök, u64::MAX),
            yerel(&kök),
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
        giriş_kur(
            bileşen("teknik"),
            kök.clone(),
            damga(&kök, u64::MAX),
            yerel(&kök),
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
            giriş_kur(
                bileşen("sağlayıcı-hata"),
                kök.clone(),
                damga(&kök, 0),
                yerel(&kök),
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
        assert_eq!(açık_metni_oku(sonuç.bileşen.read(bağlam)), "");
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
            giriş_kur(
                bileşen("açık-metin"),
                kök.clone(),
                damga(&kök, 0),
                yerel(&kök),
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
        assert_eq!(açık_metni_oku(sonuç.bileşen.read(bağlam)), "elle-girilmiş");
    });

    // Boş açılış: sağlayıcı çağrılır ve değeri uygulanır.
    let mut y = GirişYapılandırması::tek_satırlı_metin();
    y.varsayılan_değer = sağlayıcı;
    let sonuç = görsel
        .update(|pencere, bağlam| {
            giriş_kur(
                bileşen("boş-açılış"),
                kök.clone(),
                damga(&kök, 0),
                yerel(&kök),
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
        assert_eq!(açık_metni_oku(sonuç.bileşen.read(bağlam)), "sağlayıcıdan");
    });
}

/// [`yerel`] yardımcısının dil parametreli kardeşi.
fn yerel_dil(kök: &UnicodeVeYerelMetinHizmetleri, dil: &str) -> YerelMetinBağlamı {
    let motor = kök.motor();
    kök.yerel_bağlam_fabrikası().bağlam(
        CanlıBağlamDamgası {
            bağlam: fabrika().sonraki().expect("test bağlam kimliği"),
            sürüm: BağlamSürümü(1),
        },
        motor.dil_etiketi(dil).expect("dil etiketi tanınır"),
        motor
            .numaralandırma_sistemi("latn")
            .expect("`latn` tanınır"),
        motor.takvim("gregory").expect("`gregory` tanınır"),
        motor.saat_dilimi("UTC").expect("`UTC` tanınır"),
    )
}

/// Harf dönüşümü taşıyan maske: çıktı enjekte yerel bağlama gerçekten
/// bağlıdır (`uppercase(en, "ά") = "Ά"`, `uppercase(el, "ά") = "Α"`).
fn büyüten_maske() -> GirişYapılandırması {
    let mut y = GirişYapılandırması::tek_satırlı_metin();
    y.maske = Some(GirişMaskesi::Metin(MetinGirişMaskesi {
        desen: ">LLL".into(),
        yer_tutucu_grafemi: "_".into(),
        sabitleri_göster: true,
    }));
    y
}

fn maskeli_kur(
    görsel: &mut gpui::VisualTestContext,
    ad: &'static str,
    kök: &Arc<UnicodeVeYerelMetinHizmetleri>,
    yerel_bağlam: YerelMetinBağlamı,
    metin: &'static str,
) -> gpui::Entity<gpui_bilesenleri::GirişKutusu> {
    görsel
        .update(|pencere, bağlam| {
            giriş_kur(
                bileşen(ad),
                kök.clone(),
                damga(kök, 0),
                yerel_bağlam,
                büyüten_maske(),
                metin,
                tema(),
                pencere,
                bağlam,
            )
        })
        .expect("geçerli maskeli yapılandırma kurulur")
        .bileşen
}

/// **Kabul K1:** kuruluş, çağıranın sağladığı yerel bağlamı gerçekten
/// kullanır: aynı girdi `en` ve `el` bağlamlarında farklı maske çıktısı
/// üretir (Yunanca tonos yalnız `el` büyütmesinde düşer) ve yaşayan bağlam
/// enjekte edilenin kendisidir.
#[gpui::test]
fn kurulus_enjekte_yerel_baglami_gercekten_kullanir(bağlam: &mut TestAppContext) {
    let kök = unicode_kök();
    let (_konak, görsel) = bağlam.add_window_view(|_, _| BoşKonak);
    let en_alan = maskeli_kur(görsel, "enjekte-en", &kök, yerel_dil(&kök, "en"), "ά");
    let el_alan = maskeli_kur(görsel, "enjekte-el", &kök, yerel_dil(&kök, "el"), "ά");
    görsel.update(|_, bağlam| {
        assert_eq!(en_alan.read(bağlam).yerel_bağlam().dil().bcp47(), "en");
        assert_eq!(el_alan.read(bağlam).yerel_bağlam().dil().bcp47(), "el");
        let en_metin = açık_metni_oku(en_alan.read(bağlam));
        let el_metin = açık_metni_oku(el_alan.read(bağlam));
        assert!(
            en_metin.contains('Ά'),
            "en büyütmesi tonos korur: {en_metin:?}"
        );
        assert!(
            el_metin.contains('Α'),
            "el büyütmesi tonos düşürür: {el_metin:?}"
        );
        assert!(!el_metin.contains('Ά'), "el çıktısında tonos kalmaz");
        assert_ne!(
            en_metin, el_metin,
            "kuruluş hazırlığı enjekte bağlamla koşar"
        );
    });
}

/// **Kabul K2:** aynı yerel bağlamın yeniden verilmesi gerçek no-op'tur:
/// `Ok(())`, sıfır bildirim, hiçbir eksen kıpırdamaz.
#[gpui::test]
fn ayni_yerel_baglama_gecis_gercek_noop(bağlam: &mut TestAppContext) {
    let kök = unicode_kök();
    let (_konak, görsel) = bağlam.add_window_view(|_, _| BoşKonak);
    let yerel_bağlam = yerel_dil(&kök, "tr");
    let alan = maskeli_kur(görsel, "aynı-yerel", &kök, yerel_bağlam.clone(), "ab");

    let sayaç = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let gözlem = Arc::clone(&sayaç);
    let _abone = görsel.update(|_, bağlam| {
        bağlam.observe(&alan, move |_, _| {
            gözlem.fetch_add(1, Ordering::SeqCst);
        })
    });
    let önce = görsel.update(|_, bağlam| {
        let a = alan.read(bağlam);
        let durum = açık_durum(a);
        (
            açık_metni_oku(a),
            metni_oku(durum.ham_giriş_metni()),
            *durum.seçim(),
            durum.değer_sürümü(),
        )
    });
    görsel.update(|_, bağlam| {
        alan.update(bağlam, |alan, bağlam| {
            alan.yerel_bağlamı_değiştir(yerel_bağlam, bağlam)
                .expect("aynı bağlam no-op kabul edilir");
        });
    });
    görsel.run_until_parked();
    görsel.update(|_, bağlam| {
        let a = alan.read(bağlam);
        let durum = açık_durum(a);
        assert_eq!(açık_metni_oku(a), önce.0);
        assert_eq!(metni_oku(durum.ham_giriş_metni()), önce.1);
        assert_eq!(*durum.seçim(), önce.2);
        assert_eq!(durum.değer_sürümü(), önce.3);
    });
    assert_eq!(sayaç.load(Ordering::SeqCst), 0, "no-op bildirim üretmez");
}

/// **Kabul K3:** kompozisyon yokken gerçekten farklı yerel bağlama geçiş
/// başarılıdır; değişen planda metin yenilenir, tam bir bildirim üretilir
/// ve `değer_sürümü` tam bir ilerler.
#[gpui::test]
fn kompozisyonsuz_farkli_yerel_gecisi_basarili(bağlam: &mut TestAppContext) {
    let kök = unicode_kök();
    let (_konak, görsel) = bağlam.add_window_view(|_, _| BoşKonak);
    let alan = maskeli_kur(görsel, "yerel-geçiş", &kök, yerel_dil(&kök, "en"), "ά");

    let sayaç = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let gözlem = Arc::clone(&sayaç);
    let _abone = görsel.update(|_, bağlam| {
        bağlam.observe(&alan, move |_, _| {
            gözlem.fetch_add(1, Ordering::SeqCst);
        })
    });
    let sürüm_önce = görsel.update(|_, bağlam| açık_durum(alan.read(bağlam)).değer_sürümü());
    görsel.update(|_, bağlam| {
        alan.update(bağlam, |alan, bağlam| {
            alan.yerel_bağlamı_değiştir(yerel_dil(&kök, "el"), bağlam)
                .expect("kompozisyonsuz geçiş başarılı");
        });
    });
    görsel.run_until_parked();
    görsel.update(|_, bağlam| {
        let a = alan.read(bağlam);
        assert_eq!(a.yerel_bağlam().dil().bcp47(), "el");
        assert!(
            açık_metni_oku(a).contains('Α'),
            "yeni yerelin planı uygulanır"
        );
        assert_eq!(açık_durum(a).değer_sürümü(), sürüm_önce + 1);
    });
    assert_eq!(
        sayaç.load(Ordering::SeqCst),
        1,
        "başarılı geçiş tek bildirim"
    );
}

/// **Kabul K4+K5+K6+K7:** etkin IME kompozisyonunda farklı yerel bağlam
/// exact `CompositionEtkin` üretir; ret kolunda metin, ham metin, seçim,
/// kompozisyon değeri, IME aralığı, `değer_sürümü` ve etkin yerel bağlam
/// değişmez, bildirim çıkmaz. `insertText`-commit kompozisyonu düşürür
/// (asılı eksen kalmaz) ve yeniden deneme başarıyla sonuçlanır.
#[gpui::test]
fn etkin_kompozisyonda_ret_kurtarma_ve_asili_eksen_yoklugu(bağlam: &mut TestAppContext) {
    let kök = unicode_kök();
    let (_konak, görsel) = bağlam.add_window_view(|_, _| BoşKonak);
    let alan = maskeli_kur(görsel, "ime-ret", &kök, yerel_dil(&kök, "en"), "");

    görsel.update(|pencere, bağlam| {
        alan.update(bağlam, |alan, bağlam| {
            alan.test_ime_birleşimini_değiştir(None, "ん", None, pencere, bağlam);
        });
    });
    görsel.run_until_parked();

    let sayaç = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let gözlem = Arc::clone(&sayaç);
    let _abone = görsel.update(|_, bağlam| {
        bağlam.observe(&alan, move |_, _| {
            gözlem.fetch_add(1, Ordering::SeqCst);
        })
    });
    let önce = görsel.update(|_, bağlam| {
        let a = alan.read(bağlam);
        let durum = açık_durum(a);
        let ime = a
            .açık_etkin_ime()
            .expect("IME host alanı açık roldedir")
            .expect("kompozisyon gerçekten etkin");
        (
            açık_metni_oku(a),
            metni_oku(durum.ham_giriş_metni()),
            *durum.seçim(),
            ime.metin.clone(),
            ime.utf16_aralığı.clone(),
            durum.değer_sürümü(),
            a.yerel_bağlam().dil().bcp47().to_owned(),
        )
    });

    let akıbet = görsel.update(|_, bağlam| {
        alan.update(bağlam, |alan, bağlam| {
            alan.yerel_bağlamı_değiştir(yerel_dil(&kök, "el"), bağlam)
        })
    });
    görsel.run_until_parked();
    match akıbet {
        Err(gpui_bilesenleri::GirişHatası::CompositionEtkin) => {}
        diğer => panic!("exact `CompositionEtkin` beklenir: {diğer:?}"),
    }
    görsel.update(|_, bağlam| {
        let a = alan.read(bağlam);
        let durum = açık_durum(a);
        let ime = a
            .açık_etkin_ime()
            .expect("IME host alanı açık roldedir")
            .expect("ret IME birleşimini korur");
        assert_eq!(açık_metni_oku(a), önce.0, "metin korunur");
        assert_eq!(
            metni_oku(durum.ham_giriş_metni()),
            önce.1,
            "ham metin korunur"
        );
        assert_eq!(*durum.seçim(), önce.2, "seçim korunur");
        assert_eq!(ime.metin, önce.3, "kompozisyon değeri korunur");
        assert_eq!(ime.utf16_aralığı, önce.4, "IME aralığı korunur");
        assert_eq!(durum.değer_sürümü(), önce.5, "değer sürümü yanmaz");
        assert_eq!(
            a.yerel_bağlam().dil().bcp47(),
            önce.6,
            "etkin yerel korunur"
        );
    });
    assert_eq!(sayaç.load(Ordering::SeqCst), 0, "ret kolu bildirim üretmez");

    // `insertText`-commit: kompozisyon değeri commit'le düşer, asılı eksen
    // kalmaz; yeniden deneme başarıyla sonuçlanır.
    görsel.update(|pencere, bağlam| {
        alan.update(bağlam, |alan, bağlam| {
            alan.test_ime_metnini_değiştir(None, "ん", pencere, bağlam);
        });
    });
    görsel.run_until_parked();
    görsel.update(|_, bağlam| {
        alan.update(bağlam, |alan, bağlam| {
            assert!(
                alan.açık_etkin_ime()
                    .expect("IME host alanı açık roldedir")
                    .is_none(),
                "`insertText`-commit kompozisyonu düşürür; sahte/asılı kompozisyon kalmaz"
            );
            alan.yerel_bağlamı_değiştir(yerel_dil(&kök, "el"), bağlam)
                .expect("kompozisyon bittikten sonra yeniden deneme başarılı");
            assert_eq!(alan.yerel_bağlam().dil().bcp47(), "el");
        });
    });
}

/// Eylem tabanlı testlerin konağı: alan gerçekten çizilir ki odak/eylem
/// dağıtımı üretim yolundan koşsun.
struct AlanKonağı {
    alan: gpui::Entity<gpui_bilesenleri::GirişKutusu>,
}

impl gpui::Render for AlanKonağı {
    fn render(
        &mut self,
        _: &mut gpui::Window,
        _: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        self.alan.clone()
    }
}

/// **Kabul K9+K10:** açık maskeli başlangıç (`>LLL`, `"ab"`) kuruluşta
/// `"AB_"`/ham `"AB"` tabanını kurar; düzenlemeden sonra gerçek `Escape`
/// eylemi boş/varsayılan tabana değil aynı `"AB_"` tabanına döner.
#[gpui::test]
fn acik_maskeli_baslangic_tabani_ve_escape(bağlam: &mut TestAppContext) {
    bağlam.update(gpui_bilesenleri_galeri::bileşen_tuş_bağlarını_kur);
    let kök = unicode_kök();
    let yerel_bağlam = yerel_dil(&kök, "tr");
    let kök_kopya = kök.clone();
    let (konak, görsel) = bağlam.add_window_view(move |pencere, bağlam| {
        let alan = giriş_kur(
            bileşen("maskeli-taban"),
            kök_kopya.clone(),
            damga(&kök_kopya, 0),
            yerel_bağlam,
            büyüten_maske(),
            "ab",
            tema(),
            pencere,
            bağlam,
        )
        .expect("geçerli maskeli yapılandırma kurulur")
        .bileşen;
        AlanKonağı { alan }
    });
    görsel.update(|pencere, _| pencere.activate_window());
    görsel.run_until_parked();
    let alan = görsel.update(|_, bağlam| konak.read(bağlam).alan.clone());

    görsel.update(|_, bağlam| {
        let a = alan.read(bağlam);
        let durum = açık_durum(a);
        assert_eq!(
            açık_metni_oku(a),
            "AB_",
            "kuruluş maskeyi ve büyütmeyi uygular"
        );
        assert_eq!(
            metni_oku(durum.ham_giriş_metni()),
            "AB",
            "ham değer dönüştürülmüş girdidir"
        );
        assert_eq!(
            metni_oku(durum.düzenleme_başlangıcı().düzenleme_metni()),
            "AB_",
            "başlangıç kaydı nihai şablonlu metindir"
        );
    });

    görsel.update(|pencere, bağlam| {
        alan.update(bağlam, |alan, bağlam| {
            let odak = alan.odak().clone();
            pencere.focus(&odak, bağlam);
        });
    });
    görsel.run_until_parked();
    görsel.update(|pencere, bağlam| {
        alan.update(bağlam, |alan, bağlam| {
            alan.test_ime_metnini_değiştir(None, "c", pencere, bağlam);
        });
    });
    görsel.run_until_parked();
    görsel.update(|_, bağlam| {
        assert_ne!(
            açık_metni_oku(alan.read(bağlam)),
            "AB_",
            "düzenleme gözlenir"
        );
    });

    görsel.dispatch_action(gpui_bilesenleri::DuzenlemeyiIptalEt);
    görsel.run_until_parked();
    görsel.update(|_, bağlam| {
        let a = alan.read(bağlam);
        let durum = açık_durum(a);
        assert_eq!(
            açık_metni_oku(a),
            "AB_",
            "`Escape` şablonlu `\"AB_\"` tabanına döner"
        );
        assert_eq!(
            metni_oku(durum.ham_giriş_metni()),
            "AB",
            "ham değer de tabana döner"
        );
        assert!(
            a.açık_etkin_ime()
                .expect("IME host alanı açık roldedir")
                .is_none(),
            "IME ekseni temiz"
        );
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
        (
            "src/minimal_giris_olcumu.rs",
            include_str!("../src/minimal_giris_olcumu.rs"),
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
        format!(".{}", "materyalize_et()"),
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
        (
            "src/minimal_giris_olcumu.rs",
            include_str!("../src/minimal_giris_olcumu.rs"),
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

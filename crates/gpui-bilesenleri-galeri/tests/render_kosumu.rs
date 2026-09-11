//! Galerinin gerçek GPUI penceresinde çizildiğini doğrular.
//!
//! `YÖN-006.ACC-007` sergi hatası galeriyi düşürmez. Bu dosya çizim yolunu
//! baştan sona koşturur: yerleşim, yaşayan `BİL-010` alanları, simge
//! çözümü ve aile sayfaları. Çizim panikleri burada yakalanır; gözle
//! görülene kadar beklemez.

#![allow(non_ascii_idents)]

use gpui::TestAppContext;
use gpui_bilesenleri::{
    AtamaSonucu, DegeriKabulEt, GizliAlıcıHazırlıkReddi, GizliDeğerGörünümü, GizliKabulGözlemi,
    GizliTeslimRetNedeni, HarfDönüşümü, KırpmaPolitikası, MetinDeğişikliği, MetinDüzenlemePortu,
    YardımcıEylemGörünürlüğü, YardımcıEylemTürü,
};
use gpui_bilesenleri_galeri::{
    BİL_AİLELERİ, GALERİ_SAHİPLİ_METİN_UTF8_TAVANI, GaleriHedefi, GaleriUygulaması, KAB_AİLELERİ,
    ORT_AİLELERİ, bileşen_tuş_bağlarını_kur, galeri_simge_cache_gözlemi, galeri_simge_kimliği,
};
use gpui_bilesenleri_galeri::{K03ProgramatikKanıtGözlemi, TezgahDeğerKipi, olay_özeti};
use std::{cell::RefCell, rc::Rc};

fn galeri_çiz(bağlam: &mut TestAppContext, hedef: GaleriHedefi, aile: Option<&str>) {
    bağlam.update(bileşen_tuş_bağlarını_kur);
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

fn çizimi_akıt(görsel: &mut gpui::VisualTestContext) {
    görsel.update(|pencere, bağlam| {
        pencere.activate_window();
        pencere.draw(bağlam).clear(bağlam);
    });
    görsel.run_until_parked();
}

#[gpui::test]
fn k07_urun_yuvasi_root_yetkili_simgeyi_kurulusta_ve_renderda_tasir(bağlam: &mut TestAppContext) {
    bağlam.update(bileşen_tuş_bağlarını_kur);
    let (uygulama, görsel) =
        bağlam.add_window_view(|_, _| GaleriUygulaması::hedef(GaleriHedefi::Masaüstü));
    görsel.update(|_, bağlam| {
        uygulama.update(bağlam, |uygulama, bağlam| {
            assert!(uygulama.model.aileyi_aç("BİL-010"));
            uygulama.tezgahı_değiştir(
                |tezgah| {
                    tezgah.ürün_eylemi = true;
                    tezgah.yuva_görünürlüğü = YardımcıEylemGörünürlüğü::HerZaman;
                },
                bağlam,
            );
        });
    });
    çizimi_akıt(görsel);

    görsel.update(|_, bağlam| {
        let beklenen = galeri_simge_kimliği("input.product-action", bağlam)
            .expect("ürün simgesi yaşayan snapshotta kayıtlıdır");
        let alan = uygulama
            .read(bağlam)
            .yaşayan_tezgah_alanı()
            .expect("tezgâh alanı kurulmuştur");
        let alan = alan.read(bağlam);
        let yuvalar = alan
            .yapılandırma()
            .bildirim()
            .yardımcı_eylemler
            .as_deref()
            .expect("ürün yuvası kurulmuştur");
        let ürün = yuvalar
            .iter()
            .find(|yuva| matches!(yuva.tür, YardımcıEylemTürü::Ürün(_)))
            .expect("ürün yuvası exact türle bulunur");
        assert_eq!(ürün.simge.as_ref(), Some(&beklenen));
    });
}

#[gpui::test]
fn k08_k09_gercek_plan_gecisi_ve_sicak_soguk_gpui_cizimi(bağlam: &mut TestAppContext) {
    bağlam.update(bileşen_tuş_bağlarını_kur);
    let (uygulama, görsel) =
        bağlam.add_window_view(|_, _| GaleriUygulaması::hedef(GaleriHedefi::Masaüstü));
    görsel.update(|_, bağlam| {
        uygulama.update(bağlam, |uygulama, bağlam| {
            assert!(uygulama.model.aileyi_aç("BİL-140"));
            bağlam.notify();
        });
    });
    çizimi_akıt(görsel);

    görsel.update(|_, bağlam| {
        let uygulama = uygulama.read(bağlam);
        assert_eq!(
            uygulama.k08_plan_nesli(),
            Some(1),
            "K08 kuruluş tanısı: {:?}",
            uygulama.k08_plan_hatası()
        );
        assert_eq!(uygulama.k08_plan_hatası(), None);
        let cache = galeri_simge_cache_gözlemi(bağlam);
        assert!(cache.mantıksal_kaçırma >= 1, "ilk çözüm soğuk olmalı");
        assert!(cache.mantıksal_vuruş >= 1, "ikinci çözüm sıcak olmalı");
        assert!(cache.geometri_kaçırma >= 1, "ilk paint soğuk olmalı");
        assert!(cache.geometri_vuruş >= 1, "ikinci paint sıcak olmalı");
        assert!(cache.mantıksal_payload_baytı > 0);
        assert!(cache.geometri_payload_baytı > 0);
    });

    görsel.update(|_, bağlam| {
        uygulama.update(bağlam, |uygulama, bağlam| {
            uygulama.k08_planını_ilerlet(bağlam);
        });
    });
    çizimi_akıt(görsel);
    görsel.update(|_, bağlam| {
        let uygulama = uygulama.read(bağlam);
        assert_eq!(
            uygulama.k08_plan_nesli(),
            Some(2),
            "K08 geçiş tanısı: {:?}",
            uygulama.k08_plan_hatası()
        );
        assert_eq!(uygulama.k08_plan_hatası(), None);
    });
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

/// K03 kanıtı yalnız tercih modelini sınamaz: gerçek BİL-010 tezgâhı,
/// yaşayan kanonik alanı ve kabul eylemini aynı GPUI penceresinde koşturur.
/// Türkçe harf dönüşümü ve kabulde kırpma sonrasında çizilen sahipli metin
/// yalnız galerinin kesin tüketici bütçesiyle okunur.
#[gpui::test]
fn k03_gercek_tezgah_tuketicisi_donusturur_kirpar_ve_cizer(bağlam: &mut TestAppContext) {
    bağlam.update(bileşen_tuş_bağlarını_kur);
    let (uygulama, görsel) =
        bağlam.add_window_view(move |_, _| GaleriUygulaması::hedef(GaleriHedefi::Masaüstü));

    görsel.update(|_, bağlam| {
        uygulama.update(bağlam, |uygulama, bağlam| {
            assert!(uygulama.model.aileyi_aç("BİL-010"), "tezgâh açılamadı");
            uygulama.tezgahı_değiştir(
                |tezgah| {
                    tezgah.harf_dönüşümü = HarfDönüşümü::Büyük;
                    tezgah.kırpma = KırpmaPolitikası::KabuldeKırp;
                },
                bağlam,
            );
        });
    });
    görsel.run_until_parked();

    let alan = görsel.update(|_, bağlam| {
        uygulama
            .read(bağlam)
            .yaşayan_tezgah_alanı()
            .expect("BİL-010 tezgâhı yaşayan kanonik alan kurar")
    });
    görsel.update(|pencere, bağlam| {
        alan.update(bağlam, |alan, bağlam| {
            let yetki = MetinDüzenlemePortu::dış_düzenleme_yetkisi(alan)
                .expect("gerçek açık tezgâh dış düzenleme yetkisi verir");
            let anlık = MetinDüzenlemePortu::anlık_görüntü(alan, &yetki)
                .expect("yetkili açık tezgâh snapshotı okunur");
            let sonuç = MetinDüzenlemePortu::dış_değişikliği_uygula(
                alan,
                yetki,
                gpui_bilesenleri::DışMetinDeğişikliğiİsteği {
                    değişiklik: MetinDeğişikliği {
                        utf8_aralığı: 0..anlık.metin.utf8_bayt_uzunluğu(),
                        yeni_metin: "  iyi 👩\u{200d}👩  ".to_owned(),
                    },
                    beklenen_değer_sürümü: anlık.değer_sürümü,
                    beklenen_yapılandırma_sürümü: anlık.yapılandırma_sürümü,
                },
                bağlam,
            );
            assert!(
                matches!(
                    sonuç,
                    gpui_bilesenleri::GirişSonucu::Uygulandı { .. }
                        | gpui_bilesenleri::GirişSonucu::DeğişiklikYok { .. }
                ),
                "gerçek tezgâh alanına dış düzenleme uygulanır: {sonuç:?}"
            );
            pencere.focus(alan.odak(), bağlam);
        });
    });
    görsel.run_until_parked();
    görsel.dispatch_action(DegeriKabulEt);
    görsel.run_until_parked();

    görsel.update(|_, bağlam| {
        let görünüm = alan
            .read(bağlam)
            .açık_gösterim_metni(true)
            .expect("gerçek tezgâh alanı açık roldedir");
        assert!(
            görünüm.utf8_bayt_uzunluğu() <= GALERİ_SAHİPLİ_METİN_UTF8_TAVANI,
            "sergi sahipli tüketici tavanını aşmamalı"
        );
        assert_eq!(
            görünüm
                .bütçeli_materyalize_et(GALERİ_SAHİPLİ_METİN_UTF8_TAVANI)
                .expect("görünür galeri metni bütçeli okunur")
                .as_ref(),
            "İYİ 👩\u{200d}👩"
        );
    });
}

/// Paket C'nin görünür tüketici matrisi gerçek GPUI entity'leri üzerinde
/// birlikte koşar: başarılı gizli teslim tek seferdir, hazırlık reddi
/// tamponu korur, reveal sağlayıcısız sınırda fail-closed kalır ve K04
/// programatik atamasının çift sürüm beklentisi exact eski beklenti üretir.
#[gpui::test]
fn k03_gizli_teslim_ve_programatik_atama_kapanir_reveal_acik_kalir(bağlam: &mut TestAppContext) {
    const GİZLİ: &str = "yalnız-test-gizlisi🙂";

    bağlam.update(bileşen_tuş_bağlarını_kur);
    let (uygulama, görsel) =
        bağlam.add_window_view(move |_, _| GaleriUygulaması::hedef(GaleriHedefi::Masaüstü));
    let alanlar = görsel.update(|pencere, bağlam| {
        uygulama.update(bağlam, |uygulama, bağlam| {
            assert!(uygulama.model.aileyi_aç("BİL-010"), "tezgâh açılamadı");
            uygulama
                .k03_kanıt_alanları(pencere, bağlam)
                .expect("K03 kanıt alanları gerçek kurucudan kurulur")
        })
    });

    // Genel olay görünümünün kendi çıktısı kaydedilir; ham gizli event ya da
    // payload testi kolaylaştırmak için ayrı bir yan kanala alınmaz.
    let olaylar = Rc::new(RefCell::new(Vec::new()));
    let (yetkili_abonelik, ret_aboneliği) = görsel.update(|_, bağlam| {
        let yetkili_olaylar = Rc::clone(&olaylar);
        let yetkili = bağlam.subscribe(&alanlar.parola, move |_, olay, _| {
            yetkili_olaylar.borrow_mut().push(olay_özeti(olay));
        });
        let ret_olaylar = Rc::clone(&olaylar);
        let ret = bağlam.subscribe(&alanlar.parola_reddi, move |_, olay, _| {
            ret_olaylar.borrow_mut().push(olay_özeti(olay));
        });
        (yetkili, ret)
    });

    görsel.update(|pencere, bağlam| {
        alanlar.parola.update(bağlam, |alan, bağlam| {
            alan.test_ime_metnini_değiştir(None, GİZLİ, pencere, bağlam);
        });
    });
    görsel.run_until_parked();
    let reveal_öncesi = görsel.update(|_, bağlam| {
        alanlar
            .k03_kanıt_gözlemi(bağlam)
            .yetkili_gözlem
            .expect("yetkili alan maskeli gözlem verir")
    });
    assert_eq!(reveal_öncesi.durum, GizliDeğerGörünümü::YeniGiriş);
    assert!(reveal_öncesi.düzenleme_kirli);
    assert!(!reveal_öncesi.geçici_gösterim_etkin);

    görsel.update(|pencere, bağlam| {
        uygulama.update(bağlam, |uygulama, bağlam| {
            uygulama
                .k03_reveal_isteğini_çalıştır(pencere, bağlam)
                .expect("fail-closed reveal isteği olağan hataya dönüşmez");
        });
    });
    görsel.run_until_parked();
    let reveal_sonrası = görsel.update(|_, bağlam| {
        alanlar
            .k03_kanıt_gözlemi(bağlam)
            .yetkili_gözlem
            .expect("reveal sonrasında da yalnız maskeli gözlem vardır")
    });
    assert_eq!(
        reveal_sonrası, reveal_öncesi,
        "saklamayan GPUI adapterı yokken reveal bütün yaşam metadata'sını korumalı"
    );

    görsel.update(|pencere, bağlam| {
        uygulama.update(bağlam, |uygulama, bağlam| {
            assert!(uygulama.k03_gizli_kabulü_çalıştır(false, pencere, bağlam));
        });
    });
    görsel.run_until_parked();
    let başarılı = görsel.update(|_, bağlam| alanlar.k03_kanıt_gözlemi(bağlam));
    assert_eq!(başarılı.yetkili_hazırlık_sayısı, 1);
    assert_eq!(başarılı.yetkili_teslim_sayısı, 1);
    assert_eq!(
        başarılı.yetkili_terminal,
        Some(GizliKabulGözlemi::TeslimEdildi)
    );
    assert_eq!(
        başarılı.yetkili_gözlem.expect("maskeli gözlem").durum,
        GizliDeğerGörünümü::MevcutOpak
    );

    // Kayıtlı opak değer ikinci Enter'da yeniden teslim edilmez.
    görsel.update(|pencere, bağlam| {
        uygulama.update(bağlam, |uygulama, bağlam| {
            assert!(uygulama.k03_gizli_kabulü_çalıştır(false, pencere, bağlam));
        });
    });
    görsel.run_until_parked();
    let ikinci = görsel.update(|_, bağlam| alanlar.k03_kanıt_gözlemi(bağlam));
    assert_eq!(ikinci.yetkili_hazırlık_sayısı, 1);
    assert_eq!(ikinci.yetkili_teslim_sayısı, 1);

    görsel.update(|pencere, bağlam| {
        alanlar.parola_reddi.update(bağlam, |alan, bağlam| {
            alan.test_ime_metnini_değiştir(None, GİZLİ, pencere, bağlam);
        });
    });
    görsel.run_until_parked();
    let ret_öncesi = görsel.update(|_, bağlam| {
        alanlar
            .k03_kanıt_gözlemi(bağlam)
            .reddedilen_gözlem
            .expect("ret alanı maskeli gözlem verir")
    });
    görsel.update(|pencere, bağlam| {
        uygulama.update(bağlam, |uygulama, bağlam| {
            assert!(uygulama.k03_gizli_kabulü_çalıştır(true, pencere, bağlam));
        });
    });
    görsel.run_until_parked();
    let ret = görsel.update(|_, bağlam| alanlar.k03_kanıt_gözlemi(bağlam));
    assert_eq!(ret.reddedilen_hazırlık_sayısı, 1);
    assert_eq!(
        ret.reddedilen_terminal,
        Some(GizliKabulGözlemi::AlıcıReddetti(
            GizliTeslimRetNedeni::AlıcıReddetti(GizliAlıcıHazırlıkReddi::ÜrünPolitikasıReddetti,),
        ))
    );
    assert_eq!(
        ret.reddedilen_gözlem.expect("ret sonrası maskeli gözlem"),
        ret_öncesi,
        "hazırlık reddi gizli tamponun yaşam metadata'sını değiştirmemeli"
    );

    let (atama, eski) = görsel.update(|pencere, bağlam| {
        uygulama.update(bağlam, |uygulama, bağlam| {
            uygulama
                .k03_programatik_senaryoyu_çalıştır(pencere, bağlam)
                .expect("programatik K04 kanıt alanı yaşar")
        })
    });
    assert!(matches!(atama, AtamaSonucu::Uygulandı { .. }));
    assert!(matches!(eski, AtamaSonucu::EskiBeklenti { .. }));
    assert_eq!(
        görsel.update(|_, bağlam| alanlar.k03_kanıt_gözlemi(bağlam).programatik),
        K03ProgramatikKanıtGözlemi::UygulandıVeEskiBeklentiReddedildi
    );
    görsel.run_until_parked();

    assert!(görsel.debug_bounds("k03-gercek-tuketici-kaniti").is_some());
    assert!(görsel.debug_bounds("k03-yetkili-terminal").is_some());
    assert!(görsel.debug_bounds("k03-reddedilen-terminal").is_some());
    assert!(görsel.debug_bounds("k03-programatik-terminal").is_some());
    assert!(görsel.debug_bounds("k03-reveal-acik-ekseni").is_some());
    assert_eq!(GALERİ_SAHİPLİ_METİN_UTF8_TAVANI, 64 * 1024);

    let olaylar = olaylar.borrow();
    assert_eq!(
        olaylar
            .iter()
            .filter(|olay| olay.ad == "GizliKabulSonuçlandı")
            .count(),
        2,
        "bir yetkili ve bir reddedilen kabul tam iki terminal üretmeli; opak değerin ikinci Enter'ı yeni terminal üretmemeli"
    );
    for olay in olaylar.iter() {
        assert_ne!(
            olay.ad, "DeğerKabulEdildi",
            "gizli kabul açık olaya düşmemeli"
        );
        assert!(
            !olay.özet.contains(GİZLİ),
            "demo olay özeti gizliyi taşımamalı"
        );
    }
    drop(olaylar);
    drop((yetkili_abonelik, ret_aboneliği));
}

#[gpui::test]
fn bil040_cjk_dugme_sergisi_cizilir(bağlam: &mut TestAppContext) {
    // Kapalı ve açık sonuçlar gerçek ORT-017 sağlayıcısından hazırlanıp
    // BİL-040 aile sayfasının görünür GPUI ağacına bağlanır.
    galeri_çiz(bağlam, GaleriHedefi::Masaüstü, Some("BİL-040"));
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
    bağlam.update(bileşen_tuş_bağlarını_kur);
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
    bağlam.update(bileşen_tuş_bağlarını_kur);
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

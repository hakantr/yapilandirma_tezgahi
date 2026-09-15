//! Galerinin gerçek GPUI penceresinde çizildiğini doğrular.
//!
//! `YÖN-006.ACC-007` sergi hatası galeriyi düşürmez. Bu dosya çizim yolunu
//! baştan sona koşturur: yerleşim, yaşayan `BİL-010` alanları, simge
//! çözümü ve aile sayfaları. Çizim panikleri burada yakalanır; gözle
//! görülene kadar beklemez.

#![allow(non_ascii_idents)]

use gpui::{IntoElement, Render, TestAppContext, Window, WindowAppearance};
use gpui_bilesenleri::{
    AtamaSonucu, BuyukArtir, DegeriArtir, DegeriAzalt, DegeriKabulEt, DurumPlanGörünümTutamacı,
    FormRevealSonucu, GizliAlıcıHazırlıkReddi, GizliDeğerGörünümü, GizliKabulGözlemi,
    GizliTeslimRetNedeni, HarfDönüşümü, KırpmaPolitikası, MetinDeğişikliği, MetinDüzenlemePortu,
    TumunuSec, YardımcıEylemGörünürlüğü, YardımcıEylemTürü, durum_planı_elementi,
};
use gpui_bilesenleri_galeri::{
    BİL_AİLELERİ, GALERİ_SAHİPLİ_METİN_UTF8_TAVANI, GaleriHedefi, GaleriUygulaması, KAB_AİLELERİ,
    ORT_AİLELERİ, bileşen_tuş_bağlarını_kur, galeri_simge_cache_gözlemi, galeri_simge_kimliği,
};
use gpui_bilesenleri_galeri::{
    K03ProgramatikKanıtGözlemi, TezgahBölütü, TezgahDeğerKipi, olay_özeti,
};
use gpui_bilesenleri_temel::{OdakGeçişAkıbeti, OdakGörünürlükSonucu};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

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

struct YakalananK08Konağı {
    tutamaç: DurumPlanGörünümTutamacı,
}

impl Render for YakalananK08Konağı {
    fn render(&mut self, _: &mut Window, _: &mut gpui::Context<Self>) -> impl IntoElement {
        durum_planı_elementi(&self.tutamaç)
    }
}

fn son_sahne_quadları(görsel: &mut gpui::VisualTestContext) -> (Vec<gpui::Quad>, bool) {
    görsel.update(|pencere, _| {
        (
            pencere.painted_quads(),
            matches!(
                pencere.appearance(),
                WindowAppearance::Dark | WindowAppearance::VibrantDark
            ),
        )
    })
}

fn renk_yakın(sol: f32, sağ: f32) -> bool {
    (sol - sağ).abs() < f32::EPSILON * 8.0
}

fn bilgi_ilerleme_dolgusu(quadlar: &[gpui::Quad], koyu: bool) -> Option<f64> {
    let ışıklılık = if koyu { 0.62 } else { 0.48 };
    quadlar
        .iter()
        .filter_map(|quad| {
            let renk = quad.background.as_solid()?;
            let genişlik = f64::from(quad.bounds.size.width);
            let yükseklik = f64::from(quad.bounds.size.height);
            (renk_yakın(renk.h, 0.56)
                && renk_yakın(renk.s, 0.52)
                && renk_yakın(renk.l, ışıklılık)
                && renk_yakın(renk.a, 1.0)
                && genişlik > yükseklik)
                .then_some(genişlik)
        })
        .max_by(f64::total_cmp)
}

fn başarı_simgesi_var(quadlar: &[gpui::Quad], koyu: bool) -> bool {
    let ışıklılık = if koyu { 0.58 } else { 0.42 };
    quadlar.iter().any(|quad| {
        let Some(renk) = quad.background.as_solid() else {
            return false;
        };
        let genişlik = f64::from(quad.bounds.size.width);
        let yükseklik = f64::from(quad.bounds.size.height);
        renk_yakın(renk.h, 0.36)
            && renk_yakın(renk.s, 0.52)
            && renk_yakın(renk.l, ışıklılık)
            && renk_yakın(renk.a, 1.0)
            && (genişlik - yükseklik).abs() < 0.01
            && genişlik >= 8.0
    })
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

    let ilk_tutamaç = görsel.update(|_, bağlam| {
        let tutamaç = {
            let uygulama = uygulama.read(bağlam);
            assert_eq!(
                uygulama.k08_plan_nesli(),
                Some(1),
                "K08 kuruluş tanısı: {:?}",
                uygulama.k08_plan_hatası()
            );
            assert_eq!(uygulama.k08_yaşam_nesli(), Some(1));
            assert_eq!(uygulama.k08_sunum_aşaması(), Some("%25"));
            assert_eq!(uygulama.k08_plan_hatası(), None);
            uygulama
                .k08_görünüm_tutamacı()
                .expect("ilk K08 entity tutamacı yakalanır")
        };
        let cache = galeri_simge_cache_gözlemi(bağlam);
        assert!(cache.mantıksal_kaçırma >= 1, "ilk çözüm soğuk olmalı");
        assert!(cache.mantıksal_vuruş >= 1, "ikinci çözüm sıcak olmalı");
        assert!(cache.geometri_kaçırma >= 1, "ilk paint soğuk olmalı");
        assert!(cache.geometri_vuruş >= 1, "ikinci paint sıcak olmalı");
        assert!(cache.mantıksal_payload_baytı > 0);
        assert!(cache.geometri_payload_baytı > 0);
        tutamaç
    });
    // Galeri kabuğundaki sibling ilerleme şeridini ve kaydırma konumunu
    // kanıt yüzeyinden çıkar. Kök artık galerinin kurduğu ilk opak K08
    // tutamacının kendisidir; aşağıdaki uygulama geçişleri bu clone üzerinde
    // görünürse entity değişmemiş, yerinde güncellenmiş demektir.
    görsel.update(|pencere, bağlam| {
        pencere.replace_root(bağlam, |_, _| YakalananK08Konağı {
            tutamaç: ilk_tutamaç.clone(),
        });
    });
    çizimi_akıt(görsel);
    let (yüzde_25_quadları, koyu) = son_sahne_quadları(görsel);
    let yüzde_25_dolgusu = bilgi_ilerleme_dolgusu(&yüzde_25_quadları, koyu)
        .expect("kanonik %25 planı gerçek gallery scene dolgusu üretmeli");

    görsel.update(|pencere, bağlam| {
        uygulama.update(bağlam, |uygulama, bağlam| {
            uygulama.k08_planını_ilerlet(pencere, bağlam);
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
        assert_eq!(uygulama.k08_yaşam_nesli(), Some(2));
        assert_eq!(uygulama.k08_sunum_aşaması(), Some("%75"));
        assert_eq!(uygulama.k08_plan_hatası(), None);
    });
    let (yüzde_75_quadları, koyu) = son_sahne_quadları(görsel);
    let yüzde_75_dolgusu = bilgi_ilerleme_dolgusu(&yüzde_75_quadları, koyu)
        .expect("kanonik %75 planı gerçek gallery scene dolgusu üretmeli");
    assert!(
        (yüzde_75_dolgusu / yüzde_25_dolgusu - 3.0).abs() < 0.01,
        "sibling gösteri çubuğu değil, kanonik K08 dolgusu exact üç kat değişmeli: {yüzde_25_dolgusu} -> {yüzde_75_dolgusu}",
    );

    görsel.update(|pencere, bağlam| {
        uygulama.update(bağlam, |uygulama, bağlam| {
            uygulama.k08_planını_ilerlet(pencere, bağlam);
        });
    });
    çizimi_akıt(görsel);
    görsel.update(|_, bağlam| {
        let uygulama = uygulama.read(bağlam);
        assert_eq!(uygulama.k08_plan_nesli(), Some(3));
        assert_eq!(uygulama.k08_yaşam_nesli(), Some(3));
        assert_eq!(uygulama.k08_sunum_aşaması(), Some("Sonuç"));
        assert_eq!(uygulama.k08_plan_hatası(), None);
    });
    let (sonuç_quadları, koyu) = son_sahne_quadları(görsel);
    assert_eq!(
        bilgi_ilerleme_dolgusu(&sonuç_quadları, koyu),
        None,
        "terminal sonuç planı stale %75 kanonik dolguyu taşımamalı",
    );
    assert!(
        başarı_simgesi_var(&sonuç_quadları, koyu),
        "terminal sonuç planı aileye özgü başarı geometrisi üretmeli",
    );

    // Kök hâlâ ilk epochta yakalanan clone'dur. Burada current sonuç
    // geometrisini çizmesi galerinin tutamacı değiştirmediğini kanıtlar.
    let (yakalanan_tutamaç_quadları, koyu) = son_sahne_quadları(görsel);
    assert_eq!(
        bilgi_ilerleme_dolgusu(&yakalanan_tutamaç_quadları, koyu),
        None
    );
    assert!(başarı_simgesi_var(&yakalanan_tutamaç_quadları, koyu));
}

#[gpui::test]
fn bil120_k10_galeri_gercek_alani_reveal_eder_ve_provider_commitiyle_odaklar(
    bağlam: &mut TestAppContext,
) {
    bağlam.update(bileşen_tuş_bağlarını_kur);
    let (uygulama, görsel) =
        bağlam.add_window_view(|_, _| GaleriUygulaması::hedef(GaleriHedefi::Masaüstü));
    çizimi_akıt(görsel);

    let (hedef, beklenen_başvuru) = görsel.update(|_, bağlam| {
        let uygulama = uygulama.read(bağlam);
        assert_eq!(
            uygulama.k10_form_hatası(),
            None,
            "K10 kuruluş tanısı: {:?}",
            uygulama.k10_form_hatası()
        );
        assert!(!uygulama.k10_form_hedefi_açık_mı());
        (
            uygulama
                .k10_form_hedefi()
                .expect("galeri gerçek BİL-010 form hedefini kurar"),
            uygulama
                .k10_form_hedef_başvurusu()
                .cloned()
                .expect("galeri exact ORT-005 hedefini saklar"),
        )
    });
    assert!(görsel.debug_bounds("bil-120-k10-gercek-form").is_some());
    assert!(görsel.debug_bounds("bil-120-k10-hedef").is_none());
    let odak_olayları = Rc::new(Cell::new(0usize));
    let sayaç = Rc::clone(&odak_olayları);
    let odak = görsel.update(|_, bağlam| hedef.read(bağlam).odak().clone());
    let _abonelik = görsel.update(|pencere, bağlam| {
        pencere.on_focus_in(&odak, bağlam, move |_, _| {
            sayaç.set(sayaç.get().saturating_add(1));
        })
    });
    görsel.run_until_parked();

    let sonuç = görsel.update(|pencere, bağlam| {
        uygulama.update(bağlam, |uygulama, bağlam| {
            uygulama.k10_ilk_geçersizi_göster(pencere, bağlam)
        })
    });
    let FormRevealSonucu::Odak(sonuç) = sonuç.expect("K10 reveal zinciri çalışır") else {
        panic!("başlangıçta mühürlenen geçersiz hedef kaybolmamalı")
    };
    assert_eq!(
        sonuç.görünürlük(),
        OdakGörünürlükSonucu::AynıÇevrimdeAçığaÇıkarıldı
    );
    assert_eq!(sonuç.odak().akıbet(), &OdakGeçişAkıbeti::Taşındı);
    assert_eq!(sonuç.odak().istenen_hedef(), Some(&beklenen_başvuru));
    assert_eq!(sonuç.odak().commit_edilen_hedef(), Some(&beklenen_başvuru));
    assert!(sonuç.odak().commit().is_some());
    assert!(sonuç.gpui_odağı_uygulandı());
    assert_eq!(sonuç.odaklanan_alan(), Some(&sonuç.hedef().alan));

    çizimi_akıt(görsel);
    assert!(görsel.debug_bounds("bil-120-k10-hedef").is_some());
    görsel.update(|pencere, bağlam| {
        assert!(hedef.read(bağlam).odak().is_focused(pencere));
        let uygulama = uygulama.read(bağlam);
        assert!(uygulama.k10_form_hedefi_açık_mı());
        assert_eq!(uygulama.k10_form_reveal_sayısı(), 1);
    });
    assert!(
        odak_olayları.get() > 0,
        "GPUI focus-in olayı teslim edilmeli"
    );
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

/// Paket C: gerçek BİL-010 tezgâhının **sayısal** düzenleme zinciri aynı GPUI
/// penceresinde koşar: galerinin kendi isteğinden cold hazırlanan gösterim ve
/// exact düzenleme planlarıyla gösterim → düzenleme → Enter kabulü → yeniden
/// gösterim; programatik atama ve hatalı girdi reddi dâhil. Görünür değer
/// yalnız alanın kendi gösterim yolundan okunur; galeri sayı biçimlemez.
#[gpui::test]
fn sayisal_tezgah_gercek_saglayici_zinciriyle_gosterir_duzenler_kabul_eder(
    bağlam: &mut TestAppContext,
) {
    use gpui_bilesenleri::{
        AtamaNiyeti, AçıkGirişDeğeriGirdisi, AçıkGirişDeğeriGörünümü, GirişDurumu,
        GirişDurumuGözlemi, GirişKutusu,
    };
    use gpui_bilesenleri_galeri::{BiçimUygulaması, BİÇİM_SEÇENEKLERİ};
    use gpui_bilesenleri_temel::KesinOndalıkDeğeri;

    bağlam.update(bileşen_tuş_bağlarını_kur);
    let (uygulama, görsel) =
        bağlam.add_window_view(move |_, _| GaleriUygulaması::hedef(GaleriHedefi::Masaüstü));
    let sıra = BİÇİM_SEÇENEKLERİ
        .iter()
        .position(|seçenek| seçenek.uygulama == BiçimUygulaması::Sayı { gruplama: true })
        .expect("binlik ayraçlı sayı seçeneği listede");
    görsel.update(|_, bağlam| {
        uygulama.update(bağlam, |uygulama, bağlam| {
            assert!(uygulama.model.aileyi_aç("BİL-010"), "tezgâh açılamadı");
            uygulama.tezgahı_değiştir(
                |tezgah| {
                    tezgah.değer_türü = TezgahDeğerKipi::Ondalık;
                    tezgah.türe_uyarla();
                    tezgah.biçim_seçeneğini_uygula(sıra);
                },
                bağlam,
            );
        });
    });
    görsel.run_until_parked();
    // Tür değişince alan baştan kurulur; yaşayan alan bir sonraki çizimde doğar.
    let mut alan = None;
    for _ in 0..4 {
        görsel.update(|pencere, _| pencere.refresh());
        görsel.run_until_parked();
        alan = görsel.update(|_, bağlam| uygulama.read(bağlam).yaşayan_tezgah_alanı());
        if alan.is_some() {
            break;
        }
    }
    let alan = alan.expect("BİL-010 tezgâhı yaşayan sayısal alanı kurar");
    görsel.update(|_, bağlam| {
        let alan = alan.read(bağlam);
        assert!(matches!(
            alan.yapılandırma().bildirim().biçim,
            gpui_bilesenleri::BiçimYapılandırması::Açık(
                gpui_bilesenleri::BiçimTanımı::Ondalık(_)
            )
        ));
    });

    let olaylar = Rc::new(RefCell::new(Vec::<&'static str>::new()));
    görsel.update(|_, bağlam| {
        let toplayıcı = Rc::clone(&olaylar);
        bağlam
            .subscribe(&alan, move |_, olay, _| {
                toplayıcı.borrow_mut().push(olay_özeti(olay).ad);
            })
            .detach();
    });

    let gösterim = |görsel: &mut gpui::VisualTestContext, düzenleniyor: bool| -> String {
        görsel.update(|_, bağlam| {
            alan.read(bağlam)
                .açık_gösterim_metni(düzenleniyor)
                .expect("açık tezgâh alanı")
                .bütçeli_materyalize_et(GALERİ_SAHİPLİ_METİN_UTF8_TAVANI)
                .expect("görünür galeri metni bütçeli okunur")
                .to_string()
        })
    };
    let kabul_edilmiş = |görsel: &mut gpui::VisualTestContext| -> Option<AçıkGirişDeğeriGörünümü> {
        görsel.update(|_, bağlam| {
            let GirişDurumuGözlemi::Açık(GirişDurumu::Açık(çekirdek)) =
                alan.read(bağlam).durum_gözlemi()
            else {
                unreachable!("tezgâh sayısal alanı açıktır")
            };
            çekirdek
                .kabul_edilmiş_değer()
                .map(|değer| değer.değer().clone())
        })
    };
    let metni_yaz = |görsel: &mut gpui::VisualTestContext, metin: &str| {
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
                            yeni_metin: metin.to_owned(),
                        },
                        beklenen_değer_sürümü: anlık.değer_sürümü,
                        beklenen_yapılandırma_sürümü: anlık.yapılandırma_sürümü,
                    },
                    bağlam,
                );
                assert!(
                    matches!(sonuç, gpui_bilesenleri::GirişSonucu::Uygulandı { .. }),
                    "gerçek tezgâh alanına dış düzenleme uygulanır: {sonuç:?}"
                );
                pencere.focus(alan.odak(), bağlam);
            });
        });
        görsel.run_until_parked();
    };
    let ondalık =
        |katsayı: i128, ölçek: u32| KesinOndalıkDeğeri::yeni(katsayı, ölçek).expect("ondalık");

    // Gösterim → düzenleme → Enter kabulü → yeniden gösterim.
    metni_yaz(görsel, "1234,5");
    görsel.update(|pencere, bağlam| {
        assert!(
            alan.read(bağlam).odak().is_focused(pencere),
            "kabul gerçek odaklı alana dağıtılır"
        );
    });
    assert_eq!(
        gösterim(görsel, true),
        "1234,5",
        "odaklı düzenleme metni ham kalır"
    );
    görsel.dispatch_action(DegeriKabulEt);
    görsel.run_until_parked();
    assert_eq!(
        kabul_edilmiş(görsel),
        Some(AçıkGirişDeğeriGörünümü::Ondalık(
            ondalık(12_345, 1)
        )),
        "kabul exact değeri alanın ORT-008 sağlayıcı yolundan kurar"
    );
    assert_eq!(
        gösterim(görsel, false),
        "1.234,50",
        "odak dışı gösterim galerinin cold hazırladığı gösterim planından gelir"
    );
    assert_eq!(gösterim(görsel, true), "1234,5");
    assert!(
        olaylar.borrow().contains(&"DeğerKabulEdildi"),
        "kabul olayı teslim edilmeli: {:?}",
        olaylar.borrow()
    );

    // Programatik atama: düzenleme metni exact düzenleme planından (bütün
    // haneler), odak dışı gösterim gösterim planından (yuvarlanmış).
    let sonuç = görsel.update(|_, bağlam| {
        alan.update(bağlam, |alan: &mut GirişKutusu, bağlam| {
            let GirişDurumuGözlemi::Açık(GirişDurumu::Açık(çekirdek)) = alan.durum_gözlemi()
            else {
                unreachable!("tezgâh sayısal alanı açıktır")
            };
            let değer_sürümü = çekirdek.değer_sürümü();
            let yapılandırma_sürümü = alan.yapılandırma().yapılandırma_sürümü();
            alan.değeri_ata(
                AçıkGirişDeğeriGirdisi::Ondalık(ondalık(987_654_321, 4)),
                AtamaNiyeti::KabulEdilmişDeğeriDeğiştir,
                değer_sürümü,
                yapılandırma_sürümü,
                bağlam,
            )
        })
    });
    görsel.run_until_parked();
    assert!(matches!(sonuç, AtamaSonucu::Uygulandı { .. }), "{sonuç:?}");
    assert_eq!(gösterim(görsel, true), "98.765,4321");
    assert_eq!(gösterim(görsel, false), "98.765,43");
    assert_eq!(
        kabul_edilmiş(görsel),
        Some(AçıkGirişDeğeriGörünümü::Ondalık(
            ondalık(987_654_321, 4)
        ))
    );

    // Hatalı girdi: kabul reddedilir, değer ve metin korunur, ret olayı iner.
    metni_yaz(görsel, "1,2,3");
    görsel.dispatch_action(DegeriKabulEt);
    görsel.run_until_parked();
    assert_eq!(
        kabul_edilmiş(görsel),
        Some(AçıkGirişDeğeriGörünümü::Ondalık(
            ondalık(987_654_321, 4)
        )),
        "hatalı girdi kabul edilmiş değeri değiştirmez"
    );
    assert_eq!(gösterim(görsel, true), "1,2,3", "hatalı metin silinmez");
    assert!(
        olaylar.borrow().contains(&"KabulReddedildi"),
        "ret olayı teslim edilmeli: {:?}",
        olaylar.borrow()
    );

    // `BİL-010 §49` (33.7.0): ara girdi sessiz korunur; semantik rol katmanı,
    // yazım izi makbuzları, erişilebilir değer ve konumsal adım gerçek
    // galeri alanında gözlenir. Galeri hiçbirini kendi kurmaz.
    metni_yaz(görsel, "12,");
    let önceki_olay_sayısı = olaylar.borrow().len();
    görsel.dispatch_action(DegeriKabulEt);
    görsel.run_until_parked();
    assert_eq!(gösterim(görsel, true), "12,", "ara girdi korunur");
    assert_eq!(
        olaylar.borrow().len(),
        önceki_olay_sayısı,
        "ara kabul denemesi olay yaymaz: {:?}",
        olaylar.borrow()
    );
    metni_yaz(görsel, "1.234,5");
    görsel.dispatch_action(DegeriKabulEt);
    görsel.run_until_parked();
    assert_eq!(
        kabul_edilmiş(görsel),
        Some(AçıkGirişDeğeriGörünümü::Ondalık(
            ondalık(12_345, 1)
        ))
    );
    let makbuzlar = görsel.update(|_, bağlam| {
        alan.read(bağlam)
            .açık_yazım_izi_makbuzları()
            .expect("açık tezgâh alanı")
    });
    assert!(
        makbuzlar.kabul.is_some(),
        "kabul yazım izi makbuzu bağlanır"
    );
    let roller = görsel.update(|_, bağlam| {
        alan.read(bağlam)
            .açık_gösterim_metni(false)
            .expect("açık tezgâh alanı")
            .semantik_roller()
            .map(<[gpui_bilesenleri::GirişGösterimRolü]>::to_vec)
    });
    let roller = roller.expect("sağlayıcı gösterimi semantik rol katmanı taşır");
    assert_eq!(
        roller
            .iter()
            .filter(|rol| rol.rol == gpui_bilesenleri_temel::BiçimSemantikRolü::DeğerRakamı)
            .map(|rol| rol.utf8_aralığı.len())
            .sum::<usize>(),
        6,
        "1.234,50 altı değer rakamı taşır: {roller:?}"
    );
    let erişilebilir = görsel.update(|_, bağlam| {
        alan.read(bağlam)
            .açık_erişilebilir_sunum()
            .expect("açık tezgâh alanı")
            .expect("tezgâh alanı erişilebilir ad taşır")
    });
    assert_eq!(
        erişilebilir.değer,
        Some(gpui_bilesenleri_temel::ErişilebilirDeğer::Metin(
            "1.234,50".into()
        )),
        "erişilebilir değer rol katmanından türetilir"
    );
    // Konumsal adım: exact düzenleme planı metni ("1.234,5") yaşayan metne
    // eşittir; caret sonda → son değer rakamı (kesir) → +0,1.
    görsel.update(|pencere, bağlam| {
        alan.update(bağlam, |alan, bağlam| {
            pencere.focus(alan.odak(), bağlam);
        });
    });
    görsel.run_until_parked();
    görsel.dispatch_action(gpui_bilesenleri::SatirSonunaGit);
    görsel.run_until_parked();
    görsel.dispatch_action(DegeriArtir);
    görsel.run_until_parked();
    assert_eq!(
        gösterim(görsel, true),
        "1.234,6",
        "konumsal adım exact düzenleme planıyla yazılır"
    );
    assert_eq!(
        kabul_edilmiş(görsel),
        Some(AçıkGirişDeğeriGörünümü::Ondalık(
            ondalık(12_345, 1)
        )),
        "adım kabul üretmez"
    );
}

/// `BİL-010 33.8.0` galeri karşılığı: yaşayan sayısal tezgâh alanında yüzde
/// (model `1` ve standart model `100`), para ve bilimsel seçeneklerinde gerçek
/// odak/klavye/adım/kabul zinciri; ham (gruplamasız) yazımda konumsal adım;
/// kabul makbuzu kabul edilen metnin kendi ayrıştırmasının gerçek korumasıdır.
#[gpui::test]
fn sayisal_tezgah_yuzde_para_bilimsel_konumsal_adim_gercek_klavyeyle(bağlam: &mut TestAppContext) {
    use gpui_bilesenleri::{
        AçıkGirişDeğeriGörünümü, GirişDurumu, GirişDurumuGözlemi, GirişKutusu, SatirBasinaGit,
        SatirSonunaGit,
    };
    use gpui_bilesenleri_galeri::{BiçimUygulaması, BİÇİM_SEÇENEKLERİ};
    use gpui_bilesenleri_temel::{KesinOndalıkDeğeri, YazımİziAkıbeti};

    bağlam.update(bileşen_tuş_bağlarını_kur);
    let (uygulama, görsel) =
        bağlam.add_window_view(move |_, _| GaleriUygulaması::hedef(GaleriHedefi::Masaüstü));
    let sıra = |uygulama_türü: BiçimUygulaması| {
        BİÇİM_SEÇENEKLERİ
            .iter()
            .position(|seçenek| seçenek.uygulama == uygulama_türü)
            .expect("seçenek listede")
    };
    let ondalık =
        |katsayı: i128, ölçek: u32| KesinOndalıkDeğeri::yeni(katsayı, ölçek).expect("ondalık");

    // Tezgâhı verilen türe/biçime alır ve YENİ yaşayan alanı bulur: seçenek
    // değişimi alanı yeniden kurar, eski entity ile karıştırılmaz.
    let alanı_kur = |görsel: &mut gpui::VisualTestContext,
                     kip: TezgahDeğerKipi,
                     seçenek: BiçimUygulaması,
                     önceki: Option<gpui::EntityId>|
     -> gpui::Entity<GirişKutusu> {
        let sıra = sıra(seçenek);
        görsel.update(|_, bağlam| {
            uygulama.update(bağlam, |uygulama, bağlam| {
                assert!(uygulama.model.aileyi_aç("BİL-010"), "tezgâh açılamadı");
                uygulama.tezgahı_değiştir(
                    |tezgah| {
                        tezgah.değer_türü = kip;
                        tezgah.türe_uyarla();
                        tezgah.biçim_seçeneğini_uygula(sıra);
                    },
                    bağlam,
                );
            });
        });
        görsel.run_until_parked();
        let biçim_uygun = |biçim: &gpui_bilesenleri::BiçimYapılandırması| match (biçim, seçenek) {
            (
                gpui_bilesenleri::BiçimYapılandırması::Açık(
                    gpui_bilesenleri::BiçimTanımı::Yüzde(y),
                ),
                BiçimUygulaması::YüzdeModelYüz,
            ) => y.model_ölçeği == ondalık(100, 0),
            (
                gpui_bilesenleri::BiçimYapılandırması::Açık(
                    gpui_bilesenleri::BiçimTanımı::Yüzde(y),
                ),
                BiçimUygulaması::Yüzde,
            ) => y.model_ölçeği == ondalık(1, 0),
            (
                gpui_bilesenleri::BiçimYapılandırması::Açık(
                    gpui_bilesenleri::BiçimTanımı::Para(_),
                ),
                BiçimUygulaması::Para,
            ) => true,
            (
                gpui_bilesenleri::BiçimYapılandırması::Açık(
                    gpui_bilesenleri::BiçimTanımı::Bilimsel(_),
                ),
                BiçimUygulaması::Bilimsel,
            ) => true,
            _ => false,
        };
        // Seçenek değişimi aynı entity'yi yeniden yapılandırabilir ya da yeni
        // alan kurabilir; beklenen biçim tanımı gerçekten uygulanana dek beklenir.
        let mut son_biçim = None;
        let mut uygun_alan = None;
        for _ in 0..8 {
            görsel.update(|pencere, _| pencere.refresh());
            görsel.run_until_parked();
            if let Some(alan) =
                görsel.update(|_, bağlam| uygulama.read(bağlam).yaşayan_tezgah_alanı())
            {
                let biçim = görsel
                    .update(|_, bağlam| alan.read(bağlam).yapılandırma().bildirim().biçim.clone());
                if biçim_uygun(&biçim) {
                    uygun_alan = Some(alan);
                    break;
                }
                son_biçim = Some(biçim);
            }
        }
        uygun_alan.unwrap_or_else(|| {
            panic!("tezgâh alanı {seçenek:?} için beklenen biçimi taşımalı; önceki={önceki:?}: {son_biçim:?}")
        })
    };
    let gösterim =
        |görsel: &mut gpui::VisualTestContext, alan: &gpui::Entity<GirişKutusu>| -> String {
            görsel.update(|_, bağlam| {
                alan.read(bağlam)
                    .açık_gösterim_metni(true)
                    .expect("açık tezgâh alanı")
                    .bütçeli_materyalize_et(GALERİ_SAHİPLİ_METİN_UTF8_TAVANI)
                    .expect("görünür galeri metni bütçeli okunur")
                    .to_string()
            })
        };
    let geçici = |görsel: &mut gpui::VisualTestContext,
                  alan: &gpui::Entity<GirişKutusu>|
     -> Option<AçıkGirişDeğeriGörünümü> {
        görsel.update(|_, bağlam| {
            let GirişDurumuGözlemi::Açık(GirişDurumu::Açık(çekirdek)) =
                alan.read(bağlam).durum_gözlemi()
            else {
                unreachable!("tezgâh sayısal alanı açıktır")
            };
            çekirdek.geçici_değer().map(|değer| değer.değer().clone())
        })
    };
    let kabul_edilmiş = |görsel: &mut gpui::VisualTestContext,
                         alan: &gpui::Entity<GirişKutusu>|
     -> Option<AçıkGirişDeğeriGörünümü> {
        görsel.update(|_, bağlam| {
            let GirişDurumuGözlemi::Açık(GirişDurumu::Açık(çekirdek)) =
                alan.read(bağlam).durum_gözlemi()
            else {
                unreachable!("tezgâh sayısal alanı açıktır")
            };
            çekirdek
                .kabul_edilmiş_değer()
                .map(|değer| değer.değer().clone())
        })
    };
    // Gerçek klavye: alan odaklanır, metin tuşlarla yazılır.
    let yaz =
        |görsel: &mut gpui::VisualTestContext, alan: &gpui::Entity<GirişKutusu>, metin: &str| {
            görsel.update(|pencere, bağlam| {
                alan.update(bağlam, |alan, bağlam| pencere.focus(alan.odak(), bağlam));
            });
            görsel.run_until_parked();
            görsel.dispatch_action(TumunuSec);
            görsel.run_until_parked();
            for karakter in metin.chars() {
                görsel.simulate_keystrokes(&karakter.to_string());
            }
            görsel.run_until_parked();
        };

    // --- Yüzde, standart model 100: `25` → `%25`; birler basamağı +1 kanonik
    // 0,01; PageUp kanonik 0,1; kabul makbuzu `Tam`.
    let alan = alanı_kur(
        görsel,
        TezgahDeğerKipi::Ondalık,
        BiçimUygulaması::YüzdeModelYüz,
        None,
    );
    yaz(görsel, &alan, "25");
    görsel.dispatch_action(SatirSonunaGit);
    görsel.dispatch_action(DegeriArtir);
    görsel.run_until_parked();
    assert_eq!(
        gösterim(görsel, &alan),
        "26%",
        "ham `25` yazımından konumsal adım"
    );
    assert_eq!(
        geçici(görsel, &alan),
        Some(AçıkGirişDeğeriGörünümü::Ondalık(ondalık(26, 2))),
        "kanonik delta model ölçeğinin tersidir"
    );
    görsel.dispatch_action(BuyukArtir);
    görsel.run_until_parked();
    assert_eq!(gösterim(görsel, &alan), "36%");
    görsel.dispatch_action(DegeriKabulEt);
    görsel.run_until_parked();
    assert_eq!(
        kabul_edilmiş(görsel, &alan),
        Some(AçıkGirişDeğeriGörünümü::Ondalık(ondalık(36, 2))),
        "kabul kanonik kesri taşır"
    );
    let makbuzlar = görsel.update(|_, bağlam| {
        alan.read(bağlam)
            .açık_yazım_izi_makbuzları()
            .expect("açık tezgâh alanı")
    });
    assert_eq!(
        makbuzlar.kabul.expect("kabul makbuzu").akıbet,
        YazımİziAkıbeti::Tam,
        "`36%` exact düzenleme planıyla bayt-eşit geri yazılır"
    );

    // --- Yüzde, model 1: görünür basamak kanonik basamaktır.
    let önceki = alan.entity_id();
    let alan = alanı_kur(
        görsel,
        TezgahDeğerKipi::Ondalık,
        BiçimUygulaması::Yüzde,
        Some(önceki),
    );
    yaz(görsel, &alan, "25");
    görsel.dispatch_action(SatirSonunaGit);
    görsel.dispatch_action(DegeriArtir);
    görsel.run_until_parked();
    assert_eq!(gösterim(görsel, &alan), "26%");
    assert_eq!(
        geçici(görsel, &alan),
        Some(AçıkGirişDeğeriGörünümü::Ondalık(ondalık(26, 0)))
    );

    // --- Para: simge ve grup ayracı değer basamağı değildir; ham gruplamasız
    // yazım da aynı büyüklüğü çözer.
    let önceki = alan.entity_id();
    let alan = alanı_kur(
        görsel,
        TezgahDeğerKipi::ParaBirimi,
        BiçimUygulaması::Para,
        Some(önceki),
    );
    yaz(görsel, &alan, "1234,5");
    görsel.dispatch_action(SatirBasinaGit);
    görsel.dispatch_action(DegeriArtir);
    görsel.run_until_parked();
    let para_metni = gösterim(görsel, &alan);
    assert!(
        para_metni.contains("2.234,5") && para_metni.contains('₺'),
        "binler basamağı simge/ayraçtan bağımsız adımlanır: {para_metni}"
    );
    assert_eq!(
        geçici(görsel, &alan),
        Some(AçıkGirişDeğeriGörünümü::Ondalık(
            ondalık(22_345, 1)
        ))
    );
    görsel.dispatch_action(SatirSonunaGit);
    görsel.dispatch_action(DegeriAzalt);
    görsel.run_until_parked();
    let para_sonra = gösterim(görsel, &alan);
    assert!(para_sonra.contains("2.234,4"), "{para_sonra}");

    // --- Bilimsel: mantis basamağı üs etkisiyle adımlanır; üs rakamları değer
    // basamağı değildir.
    let önceki = alan.entity_id();
    let alan = alanı_kur(
        görsel,
        TezgahDeğerKipi::Ondalık,
        BiçimUygulaması::Bilimsel,
        Some(önceki),
    );
    yaz(görsel, &alan, "1250");
    görsel.dispatch_action(DegeriKabulEt);
    görsel.run_until_parked();
    assert_eq!(
        kabul_edilmiş(görsel, &alan),
        Some(AçıkGirişDeğeriGörünümü::Ondalık(ondalık(1_250, 0)))
    );
    // Kabul odaklı ham metni korur (`1250`); üssüz düz yazımda rakamlar düz
    // büyüklüktür: birler +1 → exact planla `1,251E+03` yazılır.
    assert_eq!(gösterim(görsel, &alan), "1250");
    görsel.dispatch_action(SatirSonunaGit);
    görsel.dispatch_action(DegeriArtir);
    görsel.run_until_parked();
    assert_eq!(gösterim(görsel, &alan), "1,251E+03");
    assert_eq!(
        geçici(görsel, &alan),
        Some(AçıkGirişDeğeriGörünümü::Ondalık(ondalık(1_251, 0)))
    );
    // Kanonik metinde caret üs rakamlarının sonunda: üs rakamı değer basamağı
    // değildir, soldaki son mantis basamağı (10^-3 × 10^3 = 1) adımlanır.
    görsel.dispatch_action(SatirSonunaGit);
    görsel.dispatch_action(DegeriArtir);
    görsel.run_until_parked();
    assert_eq!(gösterim(görsel, &alan), "1,252E+03");
    assert_eq!(
        geçici(görsel, &alan),
        Some(AçıkGirişDeğeriGörünümü::Ondalık(ondalık(1_252, 0)))
    );
}

/// `ORT-003 §4`/`§13` galeri **çalışma zamanı** kanıtı.
///
/// Tezgâh gerçek GPUI penceresinde çizildiğinde doğrulanmış kabuk ve kuşak
/// geometrisi kurulur; bitişik bölüt açılınca sahiplik tek kabuktan kuşağa
/// geçer. Panelde görünen özetler aynı doğrulanmış sonuçtan türer: metin ile
/// geometri ayrışamaz.
#[gpui::test]
fn ort003_tezgah_cizimde_dogrulanmis_geometri_ve_kusak_uretir(bağlam: &mut TestAppContext) {
    bağlam.update(bileşen_tuş_bağlarını_kur);
    let (uygulama, görsel) =
        bağlam.add_window_view(|_, _| GaleriUygulaması::hedef(GaleriHedefi::Masaüstü));
    görsel.update(|_, bağlam| {
        uygulama.update(bağlam, |uygulama, bağlam| {
            assert!(uygulama.model.aileyi_aç("BİL-010"));
            uygulama.tezgahı_değiştir(
                |tezgah| {
                    tezgah.başlangıç_bölütü = None;
                    tezgah.bitiş_bölütü = None;
                },
                bağlam,
            );
        });
    });
    çizimi_akıt(görsel);

    // 1) Tek bölütte sahiplik tek kabuktadır; kuşak tutamağı boştur.
    let tek_kabuk_özeti = görsel.update(|_, bağlam| {
        let alan = uygulama
            .read(bağlam)
            .yaşayan_tezgah_alanı()
            .expect("tezgâh alanı kurulmuştur");
        let kutu = alan.read(bağlam);
        assert_eq!(
            kutu.kabuk_geometri_hatası(),
            None,
            "kabuk hatasız kurulmalı"
        );
        let geometri = kutu
            .kabuk_geometrisi()
            .expect("tek bölütte kabuk geometrisi kurulur");
        assert!(
            kutu.kuşak_geometrisi().is_none(),
            "kuşak tutamağı boş kalır"
        );
        assert!(f32::from(geometri.dış_sınırlar().size.width) > 0.0);
        assert!(
            f32::from(geometri.iç_sınırlar().size.width)
                < f32::from(geometri.dış_sınırlar().size.width),
            "iç kabuk sınır şeridini dışarıda bırakır"
        );
        gpui_bilesenleri_galeri::kutu_geometri_özeti(kutu)
    });
    assert!(
        tek_kabuk_özeti.contains("dış ") && tek_kabuk_özeti.contains("ölçek"),
        "panel özeti doğrulanmış geometriyi yazar: {tek_kabuk_özeti}"
    );

    // 2) Bitişik bölüt açılınca sahiplik kuşağa geçer.
    görsel.update(|_, bağlam| {
        uygulama.update(bağlam, |uygulama, bağlam| {
            uygulama.tezgahı_değiştir(
                |tezgah| {
                    tezgah.başlangıç_bölütü = Some(TezgahBölütü::SabitMetin);
                    tezgah.bitiş_bölütü = Some(TezgahBölütü::Eylem);
                },
                bağlam,
            );
        });
    });
    çizimi_akıt(görsel);

    let kuşak_özeti = görsel.update(|_, bağlam| {
        let alan = uygulama
            .read(bağlam)
            .yaşayan_tezgah_alanı()
            .expect("tezgâh alanı kurulmuştur");
        let kutu = alan.read(bağlam);
        assert_eq!(
            kutu.kabuk_geometri_hatası(),
            None,
            "kuşak hatasız kurulmalı"
        );
        assert!(
            kutu.kabuk_geometrisi().is_none(),
            "kuşakta tek kabuk tutamağı boşalır: iki sahip aynı anda olmaz"
        );
        let kuşak = kutu
            .kuşak_geometrisi()
            .expect("bitişik bölütte kuşak geometrisi kurulur");
        assert_eq!(kuşak.bölütler().len(), 3, "baş + alan + son");
        assert_eq!(kuşak.paylaşılan_sınırlar().len(), 2);

        // Bölütler kuşağı boşluksuz döşer.
        let dış = kuşak.dış_kabuk().dış_sınırlar();
        let mut kenar = f32::from(dış.origin.x);
        for (sıra, b) in kuşak.bölütler().iter().enumerate() {
            assert_eq!(f32::from(b.sınırlar().origin.x), kenar, "bölüt {sıra}");
            kenar += f32::from(b.sınırlar().size.width);
            assert!(
                f32::from(b.iç_sınırlar().size.width) > 0.0,
                "bölüt {sıra} içerik bölgesi çökmemeli"
            );
        }
        assert_eq!(kenar, f32::from(dış.origin.x) + f32::from(dış.size.width));

        // Her paylaşılan sınırın tek çizicisi vardır ve ikisi ayrıdır.
        let çiziciler: Vec<usize> = kuşak
            .paylaşılan_sınırlar()
            .iter()
            .map(|s| s.çizici_bölüt())
            .collect();
        assert_ne!(çiziciler[0], çiziciler[1]);

        gpui_bilesenleri_galeri::kutu_kuşak_özeti(kutu)
    });
    // Panelde görünen metin geometriden türer: bölüt sayısı ve çizici
    // listesi aynı kaynaktan gelir.
    assert!(
        kuşak_özeti.contains("#0 ") && kuşak_özeti.contains("#2 "),
        "{kuşak_özeti}"
    );
    assert!(kuşak_özeti.contains("ayırıcı çizici ["), "{kuşak_özeti}");
}

/// `BİL-010.ACC-031` galeri çalışma zamanı kanıtı: görünmez köşede başlayan
/// basış, görünür bölgede bırakılsa da gönderim üretmez.
///
/// Bağımsız inceleme bu karşı örneği üretim yolunda buldu; galeri gerçek
/// GPUI etkileşimiyle düzeltmeyi tüketici cephesinden ölçer.
#[gpui::test]
fn bil010_acc031_gorunmez_koseden_baslayan_basis_gonderim_uretmez(bağlam: &mut TestAppContext) {
    let (uygulama, görsel, alan, akış) =
        bölütlü_tezgah_kur(bağlam, gpui_bilesenleri_temel::DüğmeŞekli::Hap);
    let _ = &uygulama;
    let gönderim_sayısı = gönderim_sayacı(&akış);

    // Bölütün içerik dikdörtgeninde, görünür kabuğun dışında kalan köşe.
    let (köşe, merkez) = görsel.update(|_, bağlam| {
        let kutu = alan.read(bağlam);
        let kuşak = kutu.kuşak_geometrisi().expect("kuşak geometrisi");
        let iç = kuşak.bölütler()[1].iç_sınırlar();
        let köşe = gpui::point(
            gpui::px(f32::from(iç.origin.x) + f32::from(iç.size.width) - 0.5),
            gpui::px(f32::from(iç.origin.y) + 0.5),
        );
        assert!(
            iç.contains(&köşe),
            "ön koşul: köşe bölütün içerik dikdörtgeninde olmalı"
        );
        assert!(
            !kuşak.dış_kabuk().içerir(köşe),
            "ön koşul: köşe görünür kabuğun dışında olmalı"
        );
        let merkez = gpui::point(
            iç.origin.x + iç.size.width / 2.0,
            iç.origin.y + iç.size.height / 2.0,
        );
        (köşe, merkez)
    });

    görsel.simulate_mouse_down(köşe, gpui::MouseButton::Left, gpui::Modifiers::none());
    görsel.run_until_parked();
    görsel.simulate_mouse_up(merkez, gpui::MouseButton::Left, gpui::Modifiers::none());
    görsel.run_until_parked();
    assert_eq!(
        gönderim_sayısı(görsel),
        0,
        "görünmez köşeden başlayan etkileşim gönderim üretmez"
    );

    // Karşıt kontrol: geçerli basış/geçerli bırakış gerçek gönderim üretir.
    görsel.simulate_mouse_down(merkez, gpui::MouseButton::Left, gpui::Modifiers::none());
    görsel.run_until_parked();
    görsel.simulate_mouse_up(merkez, gpui::MouseButton::Left, gpui::Modifiers::none());
    görsel.run_until_parked();
    assert_eq!(
        gönderim_sayısı(görsel),
        1,
        "geçerli etkileşim tek gönderim üretir"
    );
}

/// `BİL-010 §23.2`/`ORT-005 §2` galeri çalışma zamanı kanıtı: bitişik eylem
/// bölütüne gerçek `Tab` ile ulaşılır ve gerçek `Enter` ile etkinleştirilir.
#[gpui::test]
fn bil010_acc031_bitisik_bolute_klavyeyle_ulasilir_ve_etkinlesir(bağlam: &mut TestAppContext) {
    let (uygulama, görsel, alan, akış) =
        bölütlü_tezgah_kur(bağlam, gpui_bilesenleri_temel::DüğmeŞekli::Yuvarlatılmış);
    let _ = &uygulama;
    let gönderim_sayısı = gönderim_sayacı(&akış);

    görsel.update(|pencere, bağlam| {
        let odak = alan.read(bağlam).odak().clone();
        odak.focus(pencere, bağlam);
    });
    görsel.run_until_parked();

    // Gerçek `tab` tuşu bölüte ulaşır; sentetik odak ataması yapılmaz.
    let mut adım = 0;
    let mut odaklı = None;
    while adım < 6 && odaklı.is_none() {
        görsel.simulate_keystrokes("tab");
        görsel.run_until_parked();
        odaklı =
            görsel.update(|pencere, bağlam| alan.read(bağlam).odaklı_bitişik_bölüt(pencere));
        adım += 1;
    }
    assert_eq!(odaklı, Some(1), "galeri Tab dolaşımı bitişik bölüte ulaşır");
    assert_eq!(gönderim_sayısı(görsel), 0, "odaklanmak gönderim üretmez");

    // Gerçek `enter` basış+bırakışı kanonik gönderim hattına iner.
    görsel.simulate_keystrokes("enter");
    görsel.simulate_event(gpui::KeyUpEvent {
        keystroke: gpui::Keystroke::parse("enter").expect("kanonik tuş"),
    });
    görsel.run_until_parked();
    assert_eq!(
        gönderim_sayısı(görsel),
        1,
        "odaklı bölütte Enter tek gönderim üretir"
    );
}

/// Bölütlü arama tezgâhını kurar ve yaşayan alan ile olay akışını döndürür.
fn bölütlü_tezgah_kur<'a>(
    bağlam: &'a mut TestAppContext,
    şekil: gpui_bilesenleri_temel::DüğmeŞekli,
) -> (
    gpui::Entity<GaleriUygulaması>,
    &'a mut gpui::VisualTestContext,
    gpui::Entity<gpui_bilesenleri::GirişKutusu>,
    gpui::Entity<gpui_bilesenleri_galeri::OlayAkışıPaneli>,
) {
    bağlam.update(bileşen_tuş_bağlarını_kur);
    let (uygulama, görsel) =
        bağlam.add_window_view(|_, _| GaleriUygulaması::hedef(GaleriHedefi::Masaüstü));
    görsel.update(|_, bağlam| {
        uygulama.update(bağlam, |uygulama, bağlam| {
            assert!(uygulama.model.aileyi_aç("BİL-010"));
            uygulama.tezgahı_değiştir(
                |tezgah| {
                    tezgah.arama = true;
                    tezgah.arama_enter_gönderir = true;
                    tezgah.başlangıç_bölütü = None;
                    tezgah.bitiş_bölütü = Some(TezgahBölütü::Eylem);
                    tezgah.şekil_oto = false;
                    tezgah.köşe_pikseli = None;
                    tezgah.şekil = şekil;
                },
                bağlam,
            );
        });
    });
    çizimi_akıt(görsel);
    let alan = görsel.update(|_, bağlam| {
        uygulama
            .read(bağlam)
            .yaşayan_tezgah_alanı()
            .expect("tezgâh alanı kurulmuştur")
    });
    let akış = görsel.update(|_, bağlam| {
        uygulama
            .read(bağlam)
            .yaşayan_olay_akışı()
            .expect("olay akışı paneli kurulmuştur")
    });
    (uygulama, görsel, alan, akış)
}

/// Ekrandaki olay akışından `AramaGönderildi` sayısını okur.
fn gönderim_sayacı(
    akış: &gpui::Entity<gpui_bilesenleri_galeri::OlayAkışıPaneli>,
) -> impl Fn(&mut gpui::VisualTestContext) -> u32 + use<> {
    let akış = akış.clone();
    move |görsel: &mut gpui::VisualTestContext| -> u32 {
        görsel.update(|_, bağlam| {
            akış
                .read(bağlam)
                .olaylar()
                .iter()
                .filter(|olay| olay.ad == "AramaGönderildi")
                .map(|olay| olay.sayı)
                .sum()
        })
    }
}

/// `BİL-010.ACC-031` galeri **çalışma zamanı** kanıtı: bitişik eylem
/// bölütüne gerçek GPUI etkileşimiyle basmak tek gönderim üretir.
///
/// Olay sayısı ve taşınan değer ekrandaki akışın kendisinden okunur; ayrı
/// bir gölge sayaç yoktur. Bölütün görünür bölgesi kuşak geometrisinden
/// türetilir, sabit koordinat kullanılmaz.
#[gpui::test]
fn bil010_acc031_bitisik_bolut_gercek_etkilesimde_tek_gonderim_uretir(
    bağlam: &mut TestAppContext
) {
    bağlam.update(bileşen_tuş_bağlarını_kur);
    let (uygulama, görsel) =
        bağlam.add_window_view(|_, _| GaleriUygulaması::hedef(GaleriHedefi::Masaüstü));
    görsel.update(|_, bağlam| {
        uygulama.update(bağlam, |uygulama, bağlam| {
            assert!(uygulama.model.aileyi_aç("BİL-010"));
            uygulama.tezgahı_değiştir(
                |tezgah| {
                    tezgah.arama = true;
                    tezgah.arama_enter_gönderir = true;
                    tezgah.başlangıç_bölütü = None;
                    tezgah.bitiş_bölütü = Some(TezgahBölütü::Eylem);
                },
                bağlam,
            );
        });
    });
    çizimi_akıt(görsel);

    let alan = görsel.update(|_, bağlam| {
        uygulama
            .read(bağlam)
            .yaşayan_tezgah_alanı()
            .expect("tezgâh alanı kurulmuştur")
    });
    let akış = görsel.update(|_, bağlam| {
        uygulama
            .read(bağlam)
            .yaşayan_olay_akışı()
            .expect("olay akışı paneli kurulmuştur")
    });

    // Sorgu metni gerçek klavye yolundan girilir.
    görsel.update(|pencere, bağlam| {
        let odak = alan.read(bağlam).odak().clone();
        odak.focus(pencere, bağlam);
    });
    görsel.run_until_parked();
    görsel.simulate_keystrokes("k e d i");
    görsel.run_until_parked();

    let gönderim_sayısı = |görsel: &mut gpui::VisualTestContext| -> u32 {
        görsel.update(|_, bağlam| {
            akış
                .read(bağlam)
                .olaylar()
                .iter()
                .filter(|olay| olay.ad == "AramaGönderildi")
                .map(|olay| olay.sayı)
                .sum()
        })
    };
    assert_eq!(gönderim_sayısı(görsel), 0, "yazmak gönderim üretmez");

    // Bölütün görünür merkezine gerçek tıklama.
    let merkez = görsel.update(|_, bağlam| {
        let kutu = alan.read(bağlam);
        let kuşak = kutu.kuşak_geometrisi().expect("kuşak geometrisi");
        assert_eq!(kuşak.bölütler().len(), 2, "alan + sonda eylem bölütü");
        let iç = kuşak.bölütler()[1].iç_sınırlar();
        gpui::point(
            iç.origin.x + iç.size.width / 2.0,
            iç.origin.y + iç.size.height / 2.0,
        )
    });
    görsel.simulate_click(merkez, gpui::Modifiers::none());
    görsel.run_until_parked();

    assert_eq!(
        gönderim_sayısı(görsel),
        1,
        "bölüte basmak tek gönderim üretir"
    );
    // Taşınan değer: gönderim anındaki düzenleme metni alanda durur.
    görsel.update(|_, bağlam| {
        let gpui_bilesenleri::GirişMetniGözlemi::Açık(metin) = alan.read(bağlam).metin_gözlemi()
        else {
            panic!("açık alanın metni gözlenebilir");
        };
        let sahipli = gpui_bilesenleri_galeri::paylaşılan_metni_materyalize_et(&metin)
            .expect("tezgâh sorgusu bütçeye sığar");
        assert_eq!(sahipli, "kedi", "gönderim anındaki sorgu alanda durur");
    });
    // Odak ve seçim taşınmaz: bölüt alanın kuşağının parçasıdır.
    assert!(
        görsel.update(|pencere, bağlam| alan.read(bağlam).odak().is_focused(pencere)),
        "bölüte basmak odağı bırakmaz"
    );

    // Uçuştaki gönderim varken ikinci tıklama yeni gönderim üretmez.
    görsel.simulate_click(merkez, gpui::Modifiers::none());
    görsel.run_until_parked();
    assert_eq!(
        gönderim_sayısı(görsel),
        1,
        "uçuştaki gönderim varken ikinci tıklama yutulur"
    );
}

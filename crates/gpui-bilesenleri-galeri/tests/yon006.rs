use gpui_bilesenleri_galeri::*;
use gpui_bilesenleri_kabuk::BağlamSürümü;
use std::{collections::BTreeSet, sync::Arc};

#[test]
fn yon_006_acc_001_tum_aileler_sergili_ortler_gorunur_tuketicilidir() {
    let (sergiler, _) = yerleşik_kayıtlar();
    let sözleşmeler: BTreeSet<&str> = sergiler.iter().map(|s| s.sözleşme.as_ref()).collect();
    assert!(BİL_AİLELERİ.iter().all(|id| sözleşmeler.contains(id)));
    assert!(KAB_AİLELERİ.iter().all(|id| sözleşmeler.contains(id)));
    assert!(ORT_AİLELERİ.iter().all(|id| {
        sergiler
            .iter()
            .any(|s| s.sözleşme.as_ref() == *id && s.görünür_tüketim)
    }));
}

#[test]
fn yon_006_acc_002_olcutsuz_sergi_yok_ve_kanitsiz_raporu_uretilir() {
    let (sergiler, _) = yerleşik_kayıtlar();
    assert!(sergiler.iter().all(|s| !s.ölçütler.is_empty()));
    let eksik = kanıtsız_ölçütler(["BİL-010.ACC-001", "BİL-010.ACC-999"], &sergiler);
    assert!(eksik.contains("BİL-010.ACC-999"));
}

#[test]
fn yon_006_acc_003_eksen_degisimi_atomik_tek_surumludur() {
    let eski = GaleriModeli::yerleşik().eksenler;
    let yeni = eski.işlemsel_değiştir(|e| {
        e.tema = Arc::from("koyu");
        e.locale = Arc::from("ar");
        e.rtl = true;
    });
    assert_eq!(yeni.sürüm, BağlamSürümü(eski.sürüm.0 + 1));
    assert_eq!(
        (yeni.tema.as_ref(), yeni.locale.as_ref(), yeni.rtl),
        ("koyu", "ar", true)
    );
}

#[test]
fn yon_006_acc_004_masaustu_ve_wasm_ayni_galeri_cekirdegidir() {
    let masaüstü = ortak_bilgi_mimarisi(GaleriHedefi::Masaüstü);
    let wasm = ortak_bilgi_mimarisi(GaleriHedefi::Wasm);
    assert_eq!(masaüstü, wasm);
    assert!(
        include_str!("../../gpui-bilesenleri-galeri-masaustu/src/main.rs")
            .contains("gpui_bilesenleri_galeri")
    );
    assert!(
        include_str!("../../gpui-bilesenleri-galeri-wasm/src/lib.rs")
            .contains("gpui_bilesenleri_galeri")
    );
}

#[test]
fn yon_006_acc_005_desteklenmeyen_capability_durust_ve_gorunurdur() {
    let s = capability_sunumu("sistem-menüsü", false);
    assert!(!s.destekleniyor && s.rozet_görünür && !s.sahte_kontrol);
}

#[test]
fn yon_006_acc_006_kanonik_sandiktan_galeriye_ters_bagimlilik_yoktur() {
    for cargo in [
        include_str!("../../../../gpui_bilesenleri/crates/gpui-bilesenleri-temel/Cargo.toml"),
        include_str!("../../../../gpui_bilesenleri/crates/gpui-bilesenleri/Cargo.toml"),
        include_str!("../../../../gpui_bilesenleri/crates/gpui-bilesenleri-kabuk/Cargo.toml"),
    ] {
        assert!(!cargo.contains("gpui-bilesenleri-galeri"));
    }
}

#[test]
fn yon_006_acc_007_sergi_hatasi_kartta_yalitilir() {
    assert_eq!(sergiyi_yalıt(Ok(())), SergiKartıSonucu::Yaşıyor);
    assert_eq!(
        sergiyi_yalıt(Err("sergi-hatası")),
        SergiKartıSonucu::YalıtılmışHata(Arc::from("sergi-hatası"))
    );
}

/// `YÖN-006.ACC-008` başlık çözümü **gerçek** `ORT-021` hizmetiyle koşar.
///
/// Sahte çözücü ve elle kurulmuş `YerelMetinBağlamı` yoktur:
/// `İletiÇözümleyicisi` mühürlüdür, bağlam `ORT-002` fabrikasından doğar ve
/// katalog gerçek kütüğe kaydedilir.
#[test]
fn yon_006_acc_008_baslik_ort_021_istegiyle_guncel_localede_cozulur() {
    use gpui_bilesenleri_temel::{
        CanlıBağlamDamgası, GüvenliMetin, UnicodeVeYerelMetinHizmetleri, ÖrnekKimliğiFabrikası,
        İletiBütçesi, İletiDüğümü, İletiKataloğuKimliği, İletiKataloğuKütüğü, İletiKataloğuPaketi,
        İletiÇözümHizmeti, İletiÇözümleyicisi, İletiŞablonu,
    };

    let (sergiler, _) = yerleşik_kayıtlar();
    let istek = sergi_başlığı_isteği(&sergiler[0]);
    assert!(istek.argümanlar.is_empty());

    // Gerçek `ORT-002` kökü ve yaşayan yerel bağlam.
    let fabrika = ÖrnekKimliğiFabrikası::yeni_süreç_kapsamı().expect("test kimlik kapsamı");
    let unicode = UnicodeVeYerelMetinHizmetleri::yerlesik(
        ÖrnekKimliğiFabrikası::yeni_süreç_kapsamı().expect("test kimlik kapsamı"),
    );
    let motor = unicode.motor();
    let yerel = Arc::new(
        unicode.yerel_bağlam_fabrikası().bağlam(
            CanlıBağlamDamgası {
                bağlam: fabrika.sonraki().expect("yerel kök kimliği"),
                sürüm: BağlamSürümü(9),
            },
            motor.dil_etiketi("tr").expect("`tr` tanınır"),
            motor
                .numaralandırma_sistemi("latn")
                .expect("`latn` tanınır"),
            motor.takvim("gregory").expect("`gregory` tanınır"),
            motor
                .saat_dilimi("Europe/Istanbul")
                .expect("`Europe/Istanbul` tanınır"),
        ),
    );

    // İlk serginin başlık anahtarını taşıyan gerçek katalog paketi.
    let kütük = İletiKataloğuKütüğü::muhurle(
        fabrika.sonraki().expect("katalog kökü kimliği"),
        İletiBütçesi::default(),
    );
    let şablon = İletiŞablonu {
        anahtar: sergiler[0].başlık_anahtarı.clone(),
        şema: Default::default(),
        kök: Arc::from([İletiDüğümü::Sabit(GüvenliMetin::yeni(
            "Metin Girişi",
            false,
            true,
        ))]),
    };
    let _kayıt = kütük
        .kaydet(İletiKataloğuPaketi {
            kimlik: İletiKataloğuKimliği::yeni("galeri.sergi").expect("katalog kimliği"),
            dil: yerel.dil().clone(),
            sürüm: BağlamSürümü(1),
            şablonlar: [(şablon.anahtar.clone(), şablon)].into(),
        })
        .expect("sergi kataloğu kaydedilir");
    let hizmet = İletiÇözümHizmeti::muhurle(kütük, unicode.motor(), Arc::clone(&yerel));

    let katalog = hizmet.etkin_katalog(&yerel).expect("katalog kayıtlı");
    let başlık =
        sergi_başlığını_çöz(&hizmet, &sergiler[0], &yerel, katalog).expect("başlık çözülür");
    assert_eq!(başlık.as_ref(), "Metin Girişi");
}

#[test]
fn yon_006_acc_009_hedefler_ayni_dort_bolgeli_bilgi_mimarisidir() {
    let m = ortak_bilgi_mimarisi(GaleriHedefi::Masaüstü);
    assert!(m.üst_araç_çubuğu && m.sol_kategori_aile && m.orta_belge && m.sağ_çapa);
    assert!(!m.hedefe_özgü_ikinci_tasarım);
}

#[test]
fn yon_006_acc_010_genel_bakis_kartlari_ve_aile_bolumleri_kararlidir() {
    let model = GaleriModeli::yerleşik();
    assert_eq!(
        model.katalog.len(),
        ORT_AİLELERİ.len() + BİL_AİLELERİ.len() + KAB_AİLELERİ.len()
    );
    let bölümler: Vec<&str> = model
        .aile_bölümleri
        .iter()
        .map(|b| b.kimlik.as_ref())
        .collect();
    assert_eq!(
        bölümler,
        [
            "amaç",
            "kullanım",
            "sergiler",
            "model-api",
            "erişilebilirlik",
            "capability",
            "profil",
            "kanıt"
        ]
    );
}

#[test]
fn yon_006_acc_011_dar_yerlesim_erisilebilir_esdegerdir() {
    assert_eq!(yerleşimi_çöz(700, 100), GaleriYerleşimKipi::Dar);
    assert_eq!(yerleşimi_çöz(700, 100), yerleşimi_çöz(700, 100));
    let e = dar_yerleşim_eşdeğerliği();
    assert!(e.sol_drawer && e.sağ_satır_içi && e.orta_sıra_korunur && e.kapalı_bölgeler_odaksız);
}

#[test]
fn yon_006_hedef_ve_render_yerlesimi_gercek_baslaticidan_cozulur() {
    assert_eq!(GaleriUygulaması::yeni().model.hedef, GaleriHedefi::Masaüstü);
    assert_eq!(GaleriUygulaması::wasm().model.hedef, GaleriHedefi::Wasm);

    let render = include_str!("../src/lib.rs");
    let wasm = include_str!("../../gpui-bilesenleri-galeri-wasm/src/lib.rs");
    assert!(render.contains("pencere.viewport_size()"));
    assert!(render.contains("dar-gezinme-özeti"));
    assert!(wasm.contains("GaleriUygulaması::wasm()"));
    let masaüstü = include_str!("../../gpui-bilesenleri-galeri-masaustu/src/main.rs");
    assert!(masaüstü.contains("--dar"));
    assert!(masaüstü.contains("size(px(760.), px(640.))"));
}

#[test]
fn yon_006_katalog_aile_sayfasina_ve_guvenli_medya_sergisine_acilir() {
    let mut model = GaleriModeli::yerleşik_hedef(GaleriHedefi::Wasm);
    assert!(!model.aileyi_aç("BİL-999"));
    assert!(model.aileyi_aç("BİL-290"));
    assert_eq!(model.sayfa, GaleriSayfası::Aile);
    assert_eq!(model.seçili_aile.as_deref(), Some("BİL-290"));
    model.genel_bakışa_dön();
    assert_eq!(model.sayfa, GaleriSayfası::GenelBakış);
    assert!(model.seçili_aile.is_none());

    let render = include_str!("../src/lib.rs");
    assert!(render.contains("on_click"));
    assert!(render.contains("track_scroll(&self.orta_kaydırma)"));
    assert!(render.contains("bil-290-guvenli-fallback"));
    assert!(render.contains("medya_fallback_planı(true)"));
}

#[test]
fn yon_006_temel_aileler_kanonik_ozelliklerle_canli_sergilenir() {
    let sergiler = include_str!("../src/sergiler.rs");
    for kimlik in [
        // `BİL-010` hazır varyant vitrini göstermez: genel bakışta tek canlı
        // alan, ailede yaşayan yapılandırma tezgâhı bulunur. İkisi de aynı
        // kanonik `GirişKutusu` varlığını tüketir.
        "bil-010-özet-alanı",
        "bil-010-tezgah",
        "bil-020-seçenek-",
        "bil-020-devre-dışı-seçenek",
        "bil-030-onay-kutusu",
        "bil-040-birincil-düğme",
        "bil-040-ikincil-düğme",
        "bil-040-devre-dışı-düğme",
    ] {
        assert!(sergiler.contains(kimlik), "eksik canlı sergi: {kimlik}");
    }
    for kanonik_özellik in [
        "SeçimKipi::Tekli",
        "MantıksalDeğer::Açık",
        "DüğmeVurgusu::Birincil",
        "ErişimDurumu::DevreDışı",
    ] {
        assert!(
            sergiler.contains(kanonik_özellik),
            "kanonik özellik bağlanmamış: {kanonik_özellik}"
        );
    }
}

/// `YÖN-006 §1` galeri kanonik tür, varsayılan veya algoritmayı kopyalayamaz.
///
/// `BİL-010` metin düzenlemesi kanonik bileşenin içindedir; galeri onu
/// yeniden uygulamaz, yalnız yapılandırıp çizer.
#[test]
fn yon_006_metin_girisi_kanonik_bilesenden_tuketilir() {
    let sergiler = include_str!("../src/sergiler.rs");
    let galeri = include_str!("../src/lib.rs");

    // Galeri kendi giriş köprüsünü, IME işleyicisini veya UTF-16 dönüşümünü
    // taşımaz; bunlar `BİL-010` sahipliğindedir.
    for kopyalanmış in [
        "impl gpui::EntityInputHandler for GaleriUygulaması",
        "fn utf16_bayt_indisi",
        "struct GirdiKöprüsü",
    ] {
        assert!(
            !sergiler.contains(kopyalanmış) && !galeri.contains(kopyalanmış),
            "galeri kanonik davranışı kopyalıyor: {kopyalanmış}"
        );
    }

    // Yaşayan alanlar kanonik bileşenden kurulur ve tipli yapılandırma alır.
    for kanonik in [
        "GirişKutusu::kur",
        "GirişYapılandırması::tek_satırlı_metin",
        "GirişMaskesi::Metin",
        "GirişMaskesi::Tarih",
        "YardımcıEylemTürü::Temizle",
        "YardımcıEylemTürü::AramayıBaşlat",
        "YardımcıEylemTürü::ParolayıGöster",
        "YardımcıEylemTürü::SeçiciyiAç",
        "İçerikGörünürlüğü::Gizli",
        "SayaçYapılandırması",
        "UzunlukSınırı",
        "Sabitİçerik::metin",
    ] {
        assert!(
            galeri.contains(kanonik),
            "kanonik yetenek galeride tüketilmiyor: {kanonik}"
        );
    }

    // Tuş yolları ve simge varlıkları başlatıcılarda kayıtlıdır.
    let masaüstü = include_str!("../../gpui-bilesenleri-galeri-masaustu/src/main.rs");
    let wasm = include_str!("../../gpui-bilesenleri-galeri-wasm/src/lib.rs");
    for başlatıcı in [masaüstü, wasm] {
        assert!(başlatıcı.contains("bileşen_tuş_bağlarını_kur"));
        assert!(başlatıcı.contains("GaleriVarlıkKaynağı"));
    }
}

#[test]
fn yon_006_yapisal_etkilesim_aileleri_kendi_durumlarini_sergiler() {
    let sergiler = include_str!("../src/sergiler.rs");
    for kimlik in [
        "bil-050-sekme-çubuğu",
        "bil-060-panel-geçişi",
        "bil-060-panel-içeriği",
        "bil-070-araç-çubuğu",
        "bil-070-taşma-menüsü",
        "bil-080-modal-aç",
        "bil-080-dialog",
        "bil-090-sonuç-listesi",
    ] {
        assert!(sergiler.contains(kimlik), "eksik canlı sergi: {kimlik}");
    }
    for kanonik_özellik in [
        "SekmeKipi::Önizleme",
        "PanelKonumu::Sağ",
        "AraçBölgesi::BirincilBaşlangıç",
        "ModalTürü::OnayDialogu",
        "SeçiciSunumu::Gömülü",
    ] {
        assert!(
            sergiler.contains(kanonik_özellik),
            "kanonik özellik bağlanmamış: {kanonik_özellik}"
        );
    }

    // Genel bakış Ant Design gibi kategori bölümlerine ayrılır ve kartlar
    // bileşen adı + bir satır açıklama taşır.
    let katalog = include_str!("../src/lib.rs");
    assert!(katalog.contains("kategori_bölümleri"));
    assert!(katalog.contains("kategori_açıklaması(kategori)"));
    assert!(katalog.contains("aile_açıklaması(kayıt.sözleşme.as_ref())"));
}

/// Kullanıcı yüzeyinde sözleşme numarası görünmez.
///
/// Sözleşmeler tasarım denetimimizdir; kütüphaneyi kullananın bilmesi
/// gerekmez. Kimlik yalnız katalog anahtarı ve kanıt yüzeyinde kalır.
#[test]
fn yon_006_kullanici_yuzeyinde_sozlesme_numarasi_gecmez() {
    let katalog = include_str!("../src/lib.rs");
    let sergiler = include_str!("../src/sergiler.rs");

    // Kullanıcıya çizilen dizeler kimlik taşımamalı. Yönlendirme `match`
    // kollarındaki kimlikler ve öğe kimlikleri kullanıcı metni değildir.
    for (dosya, kaynak) in [("lib.rs", katalog), ("sergiler.rs", sergiler)] {
        for satır in kaynak.lines() {
            let kırpık = satır.trim();
            // Yönlendirme kolu, öğe kimliği ve yorum satırları hariç.
            if kırpık.starts_with("//")
                || kırpık.starts_with('"') && kırpık.contains("=>")
                || kırpık.contains(".id(")
                || kırpık.contains("aile_görünen_adı")
                || kırpık.contains("aile_açıklaması")
            {
                continue;
            }
            for kimlik in ["BİL-0", "BİL-1", "BİL-2", "KAB-0", "KAB-1", "ORT-0"] {
                if let Some(konum) = kırpık.find(kimlik) {
                    // Yalnız çizilen metinde geçmesi sorundur: `.child("… BİL-010")`
                    let öncesi = &kırpık[..konum];
                    assert!(
                        !öncesi.contains(".child("),
                        "{dosya}: kullanıcı metninde sözleşme numarası: {kırpık}"
                    );
                }
            }
        }
    }
}

#[test]
fn yon_006_veri_ve_geri_bildirim_aileleri_canli_durum_sergiler() {
    let sergiler = include_str!("../src/sergiler.rs");
    for kimlik in [
        "bil-100-tablo",
        "bil-100-sıralama",
        "bil-110-bildirim-geçişi",
        "bil-110-toast",
        "bil-120-form",
        "bil-120-gönder",
        "bil-130-iz",
        "bil-130-değer-",
        "bil-140-banner",
        "bil-140-ilerlet",
    ] {
        assert!(sergiler.contains(kimlik), "eksik canlı sergi: {kimlik}");
    }
    for kanonik_özellik in [
        "SıralamaYönü::Azalan",
        "BildirimTürü::Toast",
        "FormGönderimDurumu::Başarılı",
        "SürekliDeğer::Tek",
        "İlerlemeDeğeri::Belirli",
    ] {
        assert!(
            sergiler.contains(kanonik_özellik),
            "kanonik özellik bağlanmamış: {kanonik_özellik}"
        );
    }
}

#[test]
fn yon_006_zengin_girdi_ve_belge_aileleri_canli_durum_sergiler() {
    let sergiler = include_str!("../src/sergiler.rs");
    for kimlik in [
        "bil-150-takvim",
        "bil-150-gün-",
        "bil-160-disclosure",
        "bil-170-palet",
        "bil-170-renk-",
        "bil-180-aktarım",
        "bil-180-ilerlet",
        "bil-190-arama",
        "bil-190-sonraki",
    ] {
        assert!(sergiler.contains(kimlik), "eksik canlı sergi: {kimlik}");
    }
    for kanonik_özellik in [
        "TakvimEtkileşimKaynağı::İşaretçi",
        "DisclosureTetikleyicisi::TümBaşlık",
        "RenkYüzeyi::Kütüphaneİçi",
        "Aktarımİlerlemesi::Belirli",
        "VurguKaynağı::AramaOturumuBil190",
    ] {
        assert!(sergiler.contains(kanonik_özellik));
    }
}

#[test]
fn yon_006_uretim_araci_aileleri_kanonik_durumlarla_canli_sergilenir() {
    let sergiler = include_str!("../src/sergiler.rs");
    for kimlik in [
        "bil-200-kısayol",
        "bil-200-yakala",
        "bil-210-arama",
        "bil-210-tema",
        "bil-220-profil",
        "bil-220-test",
        "bil-230-kod",
        "bil-230-satır-",
    ] {
        assert!(sergiler.contains(kimlik), "eksik canlı sergi: {kimlik}");
    }
    for kanonik_özellik in [
        "tuşu_yakala",
        "çakışmayı_çöz",
        "yönetilen_ayar_sunumu",
        "bağlantı_eylemleri",
        "sözdizimi_çöz",
    ] {
        assert!(sergiler.contains(kanonik_özellik));
    }
    for sözleşme in ["BİL-200", "BİL-210", "BİL-220", "BİL-230"] {
        assert!(sergiler.contains(sözleşme));
    }
}

#[test]
fn yon_006_sunum_ve_gezinme_aileleri_canli_etkilesim_sunuyor() {
    let sergiler = include_str!("../src/sergiler.rs");
    for kimlik in [
        "bil-250-yüzey",
        "bil-250-geçiş",
        "bil-260-gezinme",
        "bil-260-hedef-",
        "bil-270-görsel",
        "bil-270-sonraki",
        "bil-280-sembol",
        "bil-280-tür",
        "bil-290-poster",
        "bil-290-oynat",
    ] {
        assert!(sergiler.contains(kimlik), "eksik canlı sergi: {kimlik}");
    }
    for kanonik_özellik in [
        "YüzenGrupDurumu::yeni",
        "gezinme_sunumu",
        "görsel_konum_göstergesi",
        "kodu_doğrula",
        "oynatma_niyetini_teslim_et",
        "medya_denetim_bağdaştırıcıları",
    ] {
        assert!(sergiler.contains(kanonik_özellik));
    }
    for sözleşme in ["BİL-250", "BİL-260", "BİL-270", "BİL-280", "BİL-290"] {
        assert!(sergiler.contains(sözleşme));
    }
}

#[test]
fn yon_006_ort_001_008_davranis_laboratuvarlari_canlidir() {
    let sergiler = include_str!("../src/sergiler.rs");
    for sıra in 1..=8 {
        let kimlik = format!("ORT-{sıra:03}");
        assert!(
            sergiler.contains(&kimlik),
            "eksik ORT laboratuvarı: {kimlik}"
        );
    }
    assert!(sergiler.contains("-eylem"));
    for kanonik in [
        "UnicodeMetinMotoru",
        "KutuŞekliÇözücüsü",
        "OrtakGörselDurum",
        "GezintiHaritası",
        "BağlamSürümü",
        "YerelSayıMotoru",
    ] {
        assert!(sergiler.contains(kanonik));
    }
}

#[test]
fn yon_006_ort_009_016_etkilesim_laboratuvarlari_canlidir() {
    let sergiler = include_str!("../src/sergiler.rs");
    for sıra in 9..=16 {
        let kimlik = format!("ORT-{sıra:03}");
        assert!(
            sergiler.contains(&kimlik),
            "eksik ORT laboratuvarı: {kimlik}"
        );
    }
    for kanonik in [
        "Erişilebilirlik ağacı",
        "SürüklemeOturumu",
        "Boyutlandırma oturumu",
        "GörünürAralıkÇözümleyicisi",
        "Geri alma günlüğü",
        "Kurtarma snapshot'ı",
        "YapıştırmaMüzakerecisi",
        "SimgeKataloğu",
    ] {
        assert!(sergiler.contains(kanonik));
    }
}

#[test]
fn yon_006_ort_017_023_urun_davranisi_laboratuvarlari_canlidir() {
    let sergiler = include_str!("../src/sergiler.rs");
    for sıra in 17..=23 {
        let kimlik = format!("ORT-{sıra:03}");
        assert!(
            sergiler.contains(&kimlik),
            "eksik ORT laboratuvarı: {kimlik}"
        );
    }
    for kanonik in [
        "GörünümÇözümleyicisi",
        "PerformansBütçesi",
        "Redaksiyon politikası",
        "AyarÇözümleyicisi",
        "İletiÇözümleyicisi",
        "KomutKataloğu",
        "GezinmeMotoru",
    ] {
        assert!(sergiler.contains(kanonik));
    }
}

#[test]
fn yon_006_kab_010_100_kabuk_simulasyonlari_canlidir() {
    let sergiler = include_str!("../src/sergiler.rs");
    for sıra in 1..=10 {
        let kimlik = format!("KAB-{sıra:02}0");
        assert!(
            sergiler.contains(&kimlik),
            "eksik KAB simülasyonu: {kimlik}"
        );
    }
    for kanonik in [
        "DockKonağı",
        "AltÇalışmaAlanı",
        "PencereKromu",
        "DurumÇubuğu",
        "PencereYaşamDöngüsü",
        "BölünmüşGörünüm",
        "UygulamaMenüsü",
        "PencereYerleşimKaydı",
        "OturumKurtarmaPlanı",
        "GizliSaklamaYetenekleri",
    ] {
        assert!(sergiler.contains(kanonik));
    }
    assert!(sergiler.contains("Hedef: WASM simülasyonu"));
}

#[test]
fn yon_006_gezinme_kullanici_adlarini_birincil_teknik_kimligi_ikincil_gosterir() {
    assert_eq!(aile_görünen_adı("BİL-010"), "Metin Girişi");
    assert_eq!(aile_görünen_adı("BİL-030"), "Onay Kutusu ve Anahtar");
    assert_eq!(aile_görünen_adı("BİL-150"), "Takvim ve Tarih Seçimi");
    assert_eq!(aile_görünen_adı("ORT-009"), "Erişilebilirlik");
    assert_eq!(aile_görünen_adı("KAB-060"), "Bölünmüş Görünüm");

    let model = GaleriModeli::yerleşik();
    assert_eq!(model.katalog[0].sözleşme.as_ref(), "BİL-010");
    let render = include_str!("../src/lib.rs");
    assert!(render.contains("sol-bileşen-gezintisi"));
    assert!(render.contains("dar-aile-listesi-geçişi"));
    assert!(render.contains("dar-aile-listesi"));
    assert!(render.contains("aile_görünen_adı"));

    let sözleşme = include_str!(
        "../../../../gpui_bilesenleri/sozlesmeler/GALERİ_VE_KANIT_UYGULAMASI_SÖZLEŞMESİ.md"
    );
    assert!(sözleşme.contains("yerelleştirilmiş anlamlı bileşen"));
    assert!(sözleşme.contains("tek başına kart, liste satırı, sayfa başlığı"));
    assert!(sözleşme.contains("sol aile listesi kalıcı ve kendi içinde kaydırılabilirdir"));
    assert!(sözleşme.contains("Teknik"));
    assert!(sözleşme.contains("koşulu değildir"));
}

#[test]
fn yon_006_acc_012_ant_yalniz_bilgi_mimarisi_referansidir() {
    let p = tasarım_provenance();
    assert!(p.ant_bilgi_mimarisi_referansı && p.kanonik_tema_ve_profil);
    assert!(!p.react_css_varlık_metin_kopyası && !p.sayısal_token_breakpoint_kopyası);
}

#[test]
fn wasm_runtime_dis_olay_dongusunu_ve_icerik_surumunu_korur() {
    let başlatıcı = include_str!("../../gpui-bilesenleri-galeri-wasm/src/lib.rs");
    // Uygulama tutamacı sandığın herhangi bir dosyasında tutulabilir; önemli
    // olan tutulması. Web platformu dış olay döngüsünü kullandığı için
    // `run_embedded` hemen döner ve tutamaç düşerse uygulama grafik backend'i
    // hazırlanmadan biter. `ORT-018` ölçüm yüzeyi ayrı dosyaya taşınınca
    // saklama da oraya geçti; değişmez sandık düzeyindedir.
    let ölçüm = include_str!("../../gpui-bilesenleri-galeri-wasm/src/ölçüm.rs");
    let konak = include_str!("../../gpui-bilesenleri-galeri-wasm/web/index.html");
    let hazırlama = include_str!("../../../tools/wasm_galeri_hazirla.py");

    assert!(başlatıcı.contains("run_embedded"));
    assert!(
        başlatıcı.contains("ApplicationHandle") || ölçüm.contains("ApplicationHandle"),
        "uygulama tutamacı saklanmalı; yoksa grafik backend'i hazırlanmadan uygulama düşer"
    );
    assert!(başlatıcı.contains("js_name = baslat"));
    assert!(konak.contains("modül.baslat()"));
    assert!(konak.contains("pkg/build.json"));
    assert!(konak.contains("wasm_sha256"));
    assert!(konak.contains("tuval-genisligi"));
    assert!(konak.contains("--gpui-tuval-genişliği"));
    assert!(hazırlama.contains("hashlib.sha256(web_wasm.read_bytes()).hexdigest()"));
}

#[test]
fn yon_006_acc_013_konak_webgpu_yu_on_kosul_saymaz() {
    let başlatıcı = include_str!("../../gpui-bilesenleri-galeri-wasm/src/lib.rs");
    let konak = include_str!("../../gpui-bilesenleri-galeri-wasm/web/index.html");

    // Galeri backend'i zorlamaz; kanonik tercih `Auto`dur.
    assert!(
        başlatıcı.contains("WebBackendPreference::Auto"),
        "galeri backend tercihini açıkça `Auto` bildirmeli"
    );
    for zorlama in [
        "WebBackendPreference::WebGpu",
        "WebBackendPreference::WebGl",
    ] {
        assert!(
            !başlatıcı.contains(zorlama),
            "galeri hiçbir grafik backend'ini zorlamaz: {zorlama}"
        );
    }

    // Başlatma aşaması backend adı taşımaz; WebGPU beklendiğini varsayamaz.
    assert!(!başlatıcı.contains("webgpu-bekleniyor"));

    // Konak `navigator.gpu` yokluğunu tek başına engel saymaz: erken hata
    // yalnız iki backend de yokken verilir.
    assert!(
        konak.contains("WebGL2RenderingContext"),
        "konak WebGL2 yolunu da yoklamalı"
    );
    assert!(
        !konak.contains(r#"if (!navigator.gpu) {"#),
        "WebGPU tek ön koşul kapısı kaldırılmış olmalı"
    );
    assert!(konak.contains("ne WebGPU ne WebGL2"));
}

#[test]
fn yon_006_acc_014_runtime_kaniti_backend_adiyla_kaydedilir() {
    let rapor = include_str!("../../../../gpui_bilesenleri/raporlar/wasm_runtime.md");

    // Kanıt hangi backend'de alındığını adıyla söyler.
    assert!(rapor.contains("WebGPU"));
    // WebGL2 yolu ayrı ve henüz runtime doğrulaması olmayan bir yol olarak
    // görünür; WebGPU koşumu onun kanıtı diye sunulmaz.
    assert!(
        rapor.contains("WebGL2"),
        "rapor WebGL2 yolunun kanıt durumunu ayrıca bildirmeli"
    );
    assert!(
        rapor.contains("diğerinin kanıtı sayılmaz")
            || rapor.contains("kanıtı değildir")
            || rapor.contains("doğrulanmadı"),
        "bir backend'in koşumu diğerinin kanıtı olarak sunulmamalı"
    );
}

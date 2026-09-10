use gpui_bilesenleri_galeri::{KanıtRaporuHatası, KanıtRaporuÖzeti};
use gpui_bilesenleri_uyum::KanıtTürü;
use serde_json::{Value, json};

const H40: &str = "1111111111111111111111111111111111111111";
const H64: &str = "2222222222222222222222222222222222222222222222222222222222222222";
const RUN: &str = "33333333333333333333333333333333";
const EMPTY_SHA: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

fn ortak_koşum(komut: &str, profil: &str, cargo_hedefi: &str) -> Value {
    json!({
        "koşum": RUN,
        "kök_revizyonu": H40,
        "parent_yürütülebilir_sha256": H64,
        "child_yürütülebilir_sha256": H64,
        "parent_korelasyon_sha256": H64,
        "komut_kimliği": komut,
        "komut_görünümü": "redakte komut",
        "komut_girdisi_sha256": H64,
        "feature_kümesi": [],
        "cargo_hedefi": cargo_hedefi,
        "hedef_üçlüsü": "aarch64-apple-darwin",
        "profil": profil,
        "araç_zinciri": "sabit-zincir",
        "girdi_sha256": H64,
        "stdout": { "tam_sha256": H64, "gözlenen_bayt": 0 },
        "stderr": { "tam_sha256": H64, "gözlenen_bayt": 0 },
        "çıktı_sha256": H64,
        "makine_sonucu_sha256": H64,
        "tamamlama": "",
        "başladı": { "saniye": 10, "nanos": 1 },
        "bitti": { "saniye": 10, "nanos": 2 },
        "seçilen_testler": [],
        "libtest_keşfi": null,
        "test_birimleri": [],
        "test_sayaçları": null,
        "derleme_birimleri": [],
        "rapor_birimleri": [],
        "yapısal_birimler": []
    })
}

fn yapısal_kayıt() -> Value {
    let mut koşum = ortak_koşum(
        "yon005.sozlesme-denetimi",
        "yapısal",
        "yon005.sozlesme-denetimi",
    );
    koşum["tamamlama"] = json!("exit_zero_ve_tam_drenaj");
    koşum["yapısal_birimler"] = json!([{
        "kural": "yon005.sozlesme-denetimi",
        "kaynak_konumu": "tools/sozlesme_denetimi.py",
        "kaynak_sha256": H64,
        "analiz_artefaktı_sha256": H64,
        "denetlenen_öğe_sayısı": 1,
        "ihlal_sayısı": 0,
        "tamamlama": "doğrulandı"
    }]);
    json!({
        "kimlik": "yon005.sozlesme-denetimi",
        "hedef": "YÖN-005.ACC-002",
        "tür": "yapısal",
        "durum": "doğrulandı",
        "kaynak": {
            "konum": "tools/sozlesme_denetimi.py",
            "hedef": { "yapısal": { "kimlik": "yon005.sozlesme-denetimi" } },
            "kaynak_sha256": H64
        },
        "koşum": koşum,
        "runtime": null
    })
}

fn davranış_kayıt() -> Value {
    let mut koşum = ortak_koşum("yon005.libtest.ort002", "test", "ort002");
    koşum["tamamlama"] = json!("exit_zero_ve_completion_olayı_doğrulandı");
    koşum["seçilen_testler"] = json!(["grafem_haritasi"]);
    koşum["libtest_keşfi"] = json!({
        "paket": "gpui-bilesenleri-uyum",
        "test_hedefi": "ort002",
        "libtest_adı": "grafem_haritasi",
        "test_artefaktı_sha256": H64,
        "kaynak_haritası_sha256": H64,
        "liste_sha256": H64
    });
    koşum["test_birimleri"] = json!([{
        "paket": "gpui-bilesenleri-uyum",
        "test_hedefi": "ort002",
        "libtest_adı": "grafem_haritasi",
        "kaynak_konumu": "crates/gpui-bilesenleri-uyum/tests/ort002.rs",
        "kaynak_sha256": H64,
        "tamamlama_olayı_sha256": H64,
        "akıbet": "başarılı"
    }]);
    koşum["test_sayaçları"] = json!({
        "keşfedilen": 1,
        "çalıştırılan": 1,
        "başarılı": 1,
        "atlanan": 0,
        "beklenen_başarısız": 0,
        "başarısız": 0,
        "hatalı": 0,
        "beklenmeyen_başarılı": 0
    });
    json!({
        "kimlik": "grafem_haritasi",
        "hedef": "ORT-002.ACC-002",
        "tür": "davranış",
        "durum": "doğrulandı",
        "kaynak": {
            "konum": "crates/gpui-bilesenleri-uyum/tests/ort002.rs",
            "hedef": {
                "davranış": {
                    "paket": "gpui-bilesenleri-uyum",
                    "test_hedefi": "ort002",
                    "libtest_adı": "grafem_haritasi"
                }
            },
            "kaynak_sha256": H64
        },
        "koşum": koşum,
        "runtime": null
    })
}

fn derleme_kayıt() -> Value {
    let mut koşum = ortak_koşum("yon005.derleme.ort002", "check", "k03.ort002-opaklik");
    koşum["tamamlama"] = json!("eşlenmiş_derleme_probları_doğrulandı");
    koşum["derleme_birimleri"] = json!([
        {
            "yetenek": "k03.ort002-opaklik",
            "test_kimliği": "yetkisiz",
            "rol": "yetkisiz_olumsuz",
            "girdi_sha256": H64,
            "çıktı_sha256": H64,
            "çıkış_kodu": 101,
            "beklenen_hata_parmak_izi": H64,
            "gözlenen_hata_parmak_izi": H64
        },
        {
            "yetenek": "k03.ort002-opaklik",
            "test_kimliği": "yetkili",
            "rol": "yetkili_olumlu",
            "girdi_sha256": H64,
            "çıktı_sha256": H64,
            "çıkış_kodu": 0,
            "beklenen_hata_parmak_izi": null,
            "gözlenen_hata_parmak_izi": null
        }
    ]);
    json!({
        "kimlik": "ort002.konum-haritasi-opakligi",
        "hedef": "ORT-002.ACC-016",
        "tür": "derleme",
        "durum": "doğrulandı",
        "kaynak": {
            "konum": "tools/yon005_derleme_problari/src/yetkisiz_olumsuz.rs",
            "hedef": { "derleme": { "kimlik": "k03.ort002-opaklik" } },
            "kaynak_sha256": H64
        },
        "koşum": koşum,
        "runtime": null
    })
}

fn rapor_kayıt() -> Value {
    let mut koşum = ortak_koşum(
        "yon005.rapor.uyum-olcutleri",
        "rapor",
        "yon005.uyum-olcut-raporu",
    );
    koşum["komut_görünümü"] = json!("resolved-python3 -I <registered-report-generator>");
    koşum["tamamlama"] = json!("fresh_rapor_terminali_doğrulandı");
    koşum["araç_zinciri"] = json!("python3-isolated-resolved");
    koşum["stdout"] = json!({ "tam_sha256": EMPTY_SHA, "gözlenen_bayt": 0 });
    koşum["stderr"] = json!({ "tam_sha256": EMPTY_SHA, "gözlenen_bayt": 0 });
    koşum["rapor_birimleri"] = json!([{
        "rapor_kimliği": "yon005.uyum-olcut-raporu",
        "çıktı_konumu": "yon005_uyum_olcut_raporu.json",
        "şema_sürümü": 3,
        "kök_revizyonu": H40,
        "içerik_sha256": H64,
        "hedef": "YÖN-005.ACC-019",
        "kanıt_kimliği": "yon005.rapor-provider",
        "başlatan_yürütülebilir_sha256": H64,
        "başlatıcı_yürütülebilir_sha256": H64,
        "yorumlayıcı_yürütülebilir_sha256": H64,
        "üretici_bildirimi_yürütülebilir_sha256": H64,
        "durum": "doğrulandı"
    }]);
    json!({
        "kimlik": "yon005.rapor-provider",
        "hedef": "YÖN-005.ACC-019",
        "tür": "rapor",
        "durum": "doğrulandı",
        "kaynak": {
            "konum": "tools/yon005_olcut_raporu.py",
            "hedef": {
                "rapor": {
                    "kimlik": "yon005.uyum-olcut-raporu",
                    "çıktı_konumu": "yon005_uyum_olcut_raporu.json",
                    "şema_sürümü": 3
                }
            },
            "kaynak_sha256": H64
        },
        "koşum": koşum,
        "runtime": null
    })
}

fn rapor(kayıtlar: Vec<Value>) -> Value {
    json!({
        "şema": "gpui-bilesenleri-kanit-raporu-v3",
        "şema_sürümü": 3,
        "kök_revizyonu": H40,
        "parent_yürütülebilir_sha256": H64,
        "üretildi": { "saniye": 10, "nanos": 3 },
        "kayıtlar": kayıtlar
    })
}

fn ayrıştır(değer: &Value) -> Result<KanıtRaporuÖzeti, KanıtRaporuHatası> {
    KanıtRaporuÖzeti::ayrıştır(&serde_json::to_vec(değer).expect("fixture JSON'a çevrilir"))
}

#[test]
fn yon_006_acc_016_fiziksel_dort_rol_ve_getterlar_kayipsizdir() {
    let özet = ayrıştır(&rapor(vec![
        yapısal_kayıt(),
        davranış_kayıt(),
        derleme_kayıt(),
        rapor_kayıt(),
    ]))
    .expect("dört fiziksel rol kabul edilir");
    assert_eq!(özet.kök_revizyonu(), H40);
    assert_eq!(özet.parent_yürütülebilir_sha256(), H64);
    assert_eq!(özet.kayıtlar().len(), 4);
    assert!(matches!(özet.kayıtlar()[0].tür(), KanıtTürü::Yapısal));
    assert_eq!(özet.kayıtlar()[1].hedef(), "ORT-002.ACC-002");
    assert_eq!(
        özet.kayıtlar()[2].kaynak_konumu(),
        "tools/yon005_derleme_problari/src/yetkisiz_olumsuz.rs"
    );
    assert!(matches!(özet.kayıtlar()[3].tür(), KanıtTürü::Rapor));
}

#[test]
fn yon_006_acc_016_adas_completion_ve_zaman_sirasi_reddedilir() {
    let mut adaş = davranış_kayıt();
    adaş["koşum"]["seçilen_testler"] = json!(["aynı_ad_başka_hedef"]);
    assert_eq!(
        ayrıştır(&rapor(vec![adaş])).unwrap_err(),
        KanıtRaporuHatası::KardinaliteGeçersiz
    );

    let mut ters_zaman = yapısal_kayıt();
    ters_zaman["koşum"]["başladı"] = json!({ "saniye": 11, "nanos": 0 });
    assert_eq!(
        ayrıştır(&rapor(vec![ters_zaman])).unwrap_err(),
        KanıtRaporuHatası::KoşumBağıGeçersiz
    );
}

#[test]
fn yon_006_acc_016_fiziksel_olmayan_rol_duplicate_ve_trailing_reddedilir() {
    let mut sergi_rolü = yapısal_kayıt();
    sergi_rolü["tür"] = json!("sergi");
    assert_eq!(
        ayrıştır(&rapor(vec![sergi_rolü])).unwrap_err(),
        KanıtRaporuHatası::SağlayıcıFizikselDeğil
    );

    let yinelenen =
        r#"{"şema":"gpui-bilesenleri-kanit-raporu-v3","şema":"gpui-bilesenleri-kanit-raporu-v3"}"#
            .as_bytes();
    assert_eq!(
        KanıtRaporuÖzeti::ayrıştır(yinelenen).unwrap_err(),
        KanıtRaporuHatası::YinelenenAlan
    );

    let mut trailing = serde_json::to_vec(&rapor(vec![yapısal_kayıt()])).unwrap();
    trailing.extend_from_slice(b"{}");
    assert_eq!(
        KanıtRaporuÖzeti::ayrıştır(&trailing).unwrap_err(),
        KanıtRaporuHatası::TrailingVeri
    );
}

#[test]
fn yon_006_acc_016_rapor_revision_hash_ve_terminal_tahrifini_reddeder() {
    let mut yanlış_revizyon = rapor_kayıt();
    yanlış_revizyon["koşum"]["rapor_birimleri"][0]["kök_revizyonu"] = json!("4".repeat(40));
    assert_eq!(
        ayrıştır(&rapor(vec![yanlış_revizyon])).unwrap_err(),
        KanıtRaporuHatası::KardinaliteGeçersiz
    );

    let mut tahrif = rapor_kayıt();
    tahrif["koşum"]["rapor_birimleri"][0]["içerik_sha256"] = json!("5".repeat(64));
    assert_eq!(
        ayrıştır(&rapor(vec![tahrif])).unwrap_err(),
        KanıtRaporuHatası::KardinaliteGeçersiz
    );

    let mut gürültülü = rapor_kayıt();
    gürültülü["koşum"]["stderr"]["gözlenen_bayt"] = json!(1);
    assert_eq!(
        ayrıştır(&rapor(vec![gürültülü])).unwrap_err(),
        KanıtRaporuHatası::KoşumBağıGeçersiz
    );

    let mut yanlış_kaynak = rapor_kayıt();
    yanlış_kaynak["kaynak"]["konum"] = json!("tools/baska_rapor.py");
    assert_eq!(
        ayrıştır(&rapor(vec![yanlış_kaynak])).unwrap_err(),
        KanıtRaporuHatası::KaynakBağıGeçersiz
    );

    let mut yanlış_komut = rapor_kayıt();
    yanlış_komut["koşum"]["komut_kimliği"] = json!("yon005.rapor.baska");
    assert_eq!(
        ayrıştır(&rapor(vec![yanlış_komut])).unwrap_err(),
        KanıtRaporuHatası::KoşumBağıGeçersiz
    );

    let mut yanlış_yorumlayıcı = rapor_kayıt();
    yanlış_yorumlayıcı["koşum"]["rapor_birimleri"][0]["yorumlayıcı_yürütülebilir_sha256"] =
        json!("6".repeat(64));
    assert_eq!(
        ayrıştır(&rapor(vec![yanlış_yorumlayıcı])).unwrap_err(),
        KanıtRaporuHatası::KardinaliteGeçersiz
    );

    let mut yanlış_bildirim = rapor_kayıt();
    yanlış_bildirim["koşum"]["rapor_birimleri"][0]["üretici_bildirimi_yürütülebilir_sha256"] =
        json!("7".repeat(64));
    assert_eq!(
        ayrıştır(&rapor(vec![yanlış_bildirim])).unwrap_err(),
        KanıtRaporuHatası::KardinaliteGeçersiz
    );
}

/// Gerçek YÖN-005 producer çıktısı bu testte dışarıdan verilir. Normal test
/// paketi rastgele koşum kimliğine bağlı değildir; kapanış koşumu bunu
/// `--ignored --exact` ile özellikle çalıştırır.
#[test]
#[ignore = "GPUI_KANIT_TEMEL_RAPORU ile canlı YÖN-005 çıktısı gerekir"]
fn yon_006_acc_016_canli_yon005_temel_raporunu_tuketir() {
    let yol =
        std::env::var_os("GPUI_KANIT_TEMEL_RAPORU").expect("GPUI_KANIT_TEMEL_RAPORU verilmelidir");
    let baytlar = std::fs::read(yol).expect("canlı temel rapor okunur");
    let özet = KanıtRaporuÖzeti::ayrıştır(&baytlar).expect("canlı temel rapor exact kabul edilir");
    let beklenen_çekirdek = include_str!("../../../raporlar/PAKET_C_CEKIRDEK_REVIZYONU.txt").trim();
    assert_eq!(
        özet.kök_revizyonu(),
        beklenen_çekirdek,
        "Paket C yalnız bağlandığı exact çekirdek producer raporunu kabul eder"
    );
    assert_eq!(özet.kayıtlar().len(), 26);
    assert!(özet.kayıtlar().iter().any(|kayıt| {
        kayıt.kimlik() == "grafem_ve_utf16_konumları_zwj_bayrak_ve_birleşimi_bölmez"
            && kayıt.hedef() == "ORT-002.ACC-002"
    }));
    assert!(özet.kayıtlar().iter().any(|kayıt| {
        kayıt.kimlik() == "ort002.konum-haritasi-opakligi" && kayıt.hedef() == "ORT-002.ACC-016"
    }));
    assert!(özet.kayıtlar().iter().any(|kayıt| {
        kayıt.kimlik() == "yon005.rapor-provider"
            && kayıt.hedef() == "YÖN-005.ACC-019"
            && matches!(kayıt.tür(), KanıtTürü::Rapor)
    }));
}

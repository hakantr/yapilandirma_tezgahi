//! YÖN-005'in redakte temel raporunu salt-okunur galeri özetine indirger.
//!
//! Bu parser serialized veriyi typed makbuza yükseltmez. Yalnız fiziksel
//! providerı bulunan Yapısal, Davranış, Derleme ve Rapor kayıtlarını exact
//! kardinaliteyle kabul eder; Sergi rolü fail-closed kalır.

use gpui_bilesenleri_uyum::{DavranışKanıtKonumu, KanıtTürü};
use serde::de::{self, DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde_json::{Map, Value};
use std::{collections::BTreeSet, fmt, path::Path, sync::Arc};

const RAPOR_BAYT_TAVANI: usize = 4 * 1024 * 1024;
const RAPOR_MAGIC: &str = "gpui-bilesenleri-kanit-raporu-v3";
const BOŞ_SHA256: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KanıtRaporuÖzeti {
    kök_revizyonu: Arc<str>,
    parent_yürütülebilir_sha256: Arc<str>,
    kayıtlar: Arc<[KanıtKaydıÖzeti]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KanıtKaydıÖzeti {
    kimlik: Arc<str>,
    hedef: Arc<str>,
    tür: KanıtTürü,
    kaynak_konumu: Arc<str>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KanıtRaporuHatası {
    BaytTavanıAşıldı,
    Utf8Geçersiz,
    JsonGeçersiz,
    YinelenenAlan,
    TrailingVeri,
    AlanKümesiGeçersiz,
    ŞemaDesteklenmiyor,
    KökBağıGeçersiz,
    KayıtKimliğiGeçersiz,
    SağlayıcıFizikselDeğil,
    TerminalBaşarısız,
    KardinaliteGeçersiz,
    KaynakBağıGeçersiz,
    KoşumBağıGeçersiz,
}

impl KanıtRaporuHatası {
    pub const fn güvenli_kod(self) -> &'static str {
        match self {
            Self::BaytTavanıAşıldı => "bayt_tavani_asildi",
            Self::Utf8Geçersiz => "utf8_gecersiz",
            Self::JsonGeçersiz => "json_gecersiz",
            Self::YinelenenAlan => "yinelenen_alan",
            Self::TrailingVeri => "trailing_veri",
            Self::AlanKümesiGeçersiz => "alan_kumesi_gecersiz",
            Self::ŞemaDesteklenmiyor => "sema_desteklenmiyor",
            Self::KökBağıGeçersiz => "kok_bagi_gecersiz",
            Self::KayıtKimliğiGeçersiz => "kayit_kimligi_gecersiz",
            Self::SağlayıcıFizikselDeğil => "saglayici_fiziksel_degil",
            Self::TerminalBaşarısız => "terminal_basarisiz",
            Self::KardinaliteGeçersiz => "kardinalite_gecersiz",
            Self::KaynakBağıGeçersiz => "kaynak_bagi_gecersiz",
            Self::KoşumBağıGeçersiz => "kosum_bagi_gecersiz",
        }
    }
}

impl fmt::Display for KanıtRaporuHatası {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.güvenli_kod())
    }
}

impl std::error::Error for KanıtRaporuHatası {}

impl KanıtRaporuÖzeti {
    pub fn ayrıştır(baytlar: &[u8]) -> Result<Self, KanıtRaporuHatası> {
        if baytlar.len() > RAPOR_BAYT_TAVANI {
            return Err(KanıtRaporuHatası::BaytTavanıAşıldı);
        }
        std::str::from_utf8(baytlar).map_err(|_| KanıtRaporuHatası::Utf8Geçersiz)?;
        let değer = benzersiz_json(baytlar)?;
        let kök = nesne(&değer, KanıtRaporuHatası::AlanKümesiGeçersiz)?;
        exact_alanlar(
            kök,
            &[
                "şema",
                "şema_sürümü",
                "kök_revizyonu",
                "parent_yürütülebilir_sha256",
                "üretildi",
                "kayıtlar",
            ],
            KanıtRaporuHatası::AlanKümesiGeçersiz,
        )?;
        if metin(kök, "şema")? != RAPOR_MAGIC || tam_sayı(kök, "şema_sürümü")? != 3 {
            return Err(KanıtRaporuHatası::ŞemaDesteklenmiyor);
        }
        let kök_revizyonu = metin(kök, "kök_revizyonu")?;
        let parent_sha = metin(kök, "parent_yürütülebilir_sha256")?;
        if !küçük_hex(kök_revizyonu, 40) || !küçük_hex(parent_sha, 64) {
            return Err(KanıtRaporuHatası::KökBağıGeçersiz);
        }
        zamanı_doğrula(kök.get("üretildi"))?;
        let kayıtlar = dizi(kök, "kayıtlar")?;
        if kayıtlar.is_empty() || kayıtlar.len() > 4_096 {
            return Err(KanıtRaporuHatası::KardinaliteGeçersiz);
        }
        let mut görülen = BTreeSet::new();
        let mut özetler = Vec::with_capacity(kayıtlar.len());
        for kayıt in kayıtlar {
            let kayıt = nesne(kayıt, KanıtRaporuHatası::AlanKümesiGeçersiz)?;
            exact_alanlar(
                kayıt,
                &[
                    "kimlik", "hedef", "tür", "durum", "kaynak", "koşum", "runtime",
                ],
                KanıtRaporuHatası::AlanKümesiGeçersiz,
            )?;
            let kimlik = metin(kayıt, "kimlik")?;
            let hedef = metin(kayıt, "hedef")?;
            if !kararlı_metin(kimlik) || !kararlı_metin(hedef) || !görülen.insert(kimlik.to_owned())
            {
                return Err(KanıtRaporuHatası::KayıtKimliğiGeçersiz);
            }
            if metin(kayıt, "durum")? != "doğrulandı" {
                return Err(KanıtRaporuHatası::TerminalBaşarısız);
            }
            if !kayıt.get("runtime").is_some_and(Value::is_null) {
                return Err(KanıtRaporuHatası::KardinaliteGeçersiz);
            }
            let tür = match metin(kayıt, "tür")? {
                "davranış" => KanıtTürü::Davranış(DavranışKanıtKonumu::DışSandık),
                "derleme" => KanıtTürü::Derleme,
                "rapor" => KanıtTürü::Rapor,
                "yapısal" => KanıtTürü::Yapısal,
                "sergi" => return Err(KanıtRaporuHatası::SağlayıcıFizikselDeğil),
                _ => return Err(KanıtRaporuHatası::SağlayıcıFizikselDeğil),
            };
            let kaynak = nesne(
                kayıt
                    .get("kaynak")
                    .ok_or(KanıtRaporuHatası::KaynakBağıGeçersiz)?,
                KanıtRaporuHatası::KaynakBağıGeçersiz,
            )?;
            exact_alanlar(
                kaynak,
                &["konum", "hedef", "kaynak_sha256"],
                KanıtRaporuHatası::KaynakBağıGeçersiz,
            )?;
            let kaynak_konumu = metin_hata(kaynak, "konum", KanıtRaporuHatası::KaynakBağıGeçersiz)?;
            let kaynak_sha = metin_hata(
                kaynak,
                "kaynak_sha256",
                KanıtRaporuHatası::KaynakBağıGeçersiz,
            )?;
            if !güvenli_göreli_yol(kaynak_konumu) || !küçük_hex(kaynak_sha, 64) {
                return Err(KanıtRaporuHatası::KaynakBağıGeçersiz);
            }
            kaynak_hedefini_doğrula(kaynak.get("hedef"), &tür)?;
            let koşum = nesne(
                kayıt
                    .get("koşum")
                    .ok_or(KanıtRaporuHatası::KoşumBağıGeçersiz)?,
                KanıtRaporuHatası::KoşumBağıGeçersiz,
            )?;
            koşumu_doğrula(
                koşum,
                &tür,
                kök_revizyonu,
                parent_sha,
                kaynak_konumu,
                kaynak_sha,
                kaynak.get("hedef"),
                kimlik,
                hedef,
            )?;
            özetler.push(KanıtKaydıÖzeti {
                kimlik: Arc::from(kimlik),
                hedef: Arc::from(hedef),
                tür,
                kaynak_konumu: Arc::from(kaynak_konumu),
            });
        }
        Ok(Self {
            kök_revizyonu: Arc::from(kök_revizyonu),
            parent_yürütülebilir_sha256: Arc::from(parent_sha),
            kayıtlar: özetler.into(),
        })
    }

    pub fn kök_revizyonu(&self) -> &str {
        &self.kök_revizyonu
    }

    pub fn parent_yürütülebilir_sha256(&self) -> &str {
        &self.parent_yürütülebilir_sha256
    }

    pub fn kayıtlar(&self) -> &[KanıtKaydıÖzeti] {
        &self.kayıtlar
    }
}

impl KanıtKaydıÖzeti {
    pub fn kimlik(&self) -> &str {
        &self.kimlik
    }

    pub fn hedef(&self) -> &str {
        &self.hedef
    }

    pub fn tür(&self) -> KanıtTürü {
        self.tür
    }

    pub fn kaynak_konumu(&self) -> &str {
        &self.kaynak_konumu
    }
}

fn koşumu_doğrula(
    koşum: &Map<String, Value>,
    tür: &KanıtTürü,
    kök_revizyonu: &str,
    parent_sha: &str,
    kaynak_konumu: &str,
    kaynak_sha: &str,
    kaynak_hedefi: Option<&Value>,
    kayıt_kimliği: &str,
    kayıt_hedefi: &str,
) -> Result<(), KanıtRaporuHatası> {
    exact_alanlar(
        koşum,
        &[
            "koşum",
            "kök_revizyonu",
            "parent_yürütülebilir_sha256",
            "child_yürütülebilir_sha256",
            "parent_korelasyon_sha256",
            "komut_kimliği",
            "komut_görünümü",
            "komut_girdisi_sha256",
            "feature_kümesi",
            "cargo_hedefi",
            "hedef_üçlüsü",
            "profil",
            "araç_zinciri",
            "girdi_sha256",
            "stdout",
            "stderr",
            "çıktı_sha256",
            "makine_sonucu_sha256",
            "tamamlama",
            "başladı",
            "bitti",
            "seçilen_testler",
            "libtest_keşfi",
            "test_birimleri",
            "test_sayaçları",
            "derleme_birimleri",
            "rapor_birimleri",
            "yapısal_birimler",
        ],
        KanıtRaporuHatası::KoşumBağıGeçersiz,
    )?;
    for (alan, uzunluk) in [
        ("koşum", 32),
        ("child_yürütülebilir_sha256", 64),
        ("parent_korelasyon_sha256", 64),
        ("komut_girdisi_sha256", 64),
        ("girdi_sha256", 64),
        ("çıktı_sha256", 64),
        ("makine_sonucu_sha256", 64),
    ] {
        if !küçük_hex(
            metin_hata(koşum, alan, KanıtRaporuHatası::KoşumBağıGeçersiz)?,
            uzunluk,
        ) {
            return Err(KanıtRaporuHatası::KoşumBağıGeçersiz);
        }
    }
    if metin_hata(koşum, "kök_revizyonu", KanıtRaporuHatası::KoşumBağıGeçersiz)? != kök_revizyonu
        || metin_hata(
            koşum,
            "parent_yürütülebilir_sha256",
            KanıtRaporuHatası::KoşumBağıGeçersiz,
        )? != parent_sha
    {
        return Err(KanıtRaporuHatası::KökBağıGeçersiz);
    }
    for alan in [
        "komut_kimliği",
        "komut_görünümü",
        "cargo_hedefi",
        "hedef_üçlüsü",
        "profil",
        "araç_zinciri",
        "tamamlama",
    ] {
        if !kararlı_metin(metin_hata(
            koşum,
            alan,
            KanıtRaporuHatası::KoşumBağıGeçersiz,
        )?) {
            return Err(KanıtRaporuHatası::KoşumBağıGeçersiz);
        }
    }
    akışı_doğrula(koşum.get("stdout"))?;
    akışı_doğrula(koşum.get("stderr"))?;
    let başladı = zamanı_doğrula(koşum.get("başladı"))?;
    let bitti = zamanı_doğrula(koşum.get("bitti"))?;
    if başladı > bitti
        || dizi(koşum, "feature_kümesi")?
            .iter()
            .any(|özellik| !özellik.as_str().is_some_and(kararlı_metin))
    {
        return Err(KanıtRaporuHatası::KoşumBağıGeçersiz);
    }
    dizi(koşum, "seçilen_testler")?;
    match tür {
        KanıtTürü::Davranış(_) => {
            davranış_koşumunu_doğrula(koşum, kaynak_konumu, kaynak_sha)
        }
        KanıtTürü::Derleme => derleme_koşumunu_doğrula(koşum),
        KanıtTürü::Yapısal => yapısal_koşumu_doğrula(koşum, kaynak_konumu, kaynak_sha),
        KanıtTürü::Rapor => rapor_koşumunu_doğrula(
            koşum,
            kaynak_konumu,
            kaynak_hedefi,
            kayıt_kimliği,
            kayıt_hedefi,
        ),
        KanıtTürü::Sergi => Err(KanıtRaporuHatası::SağlayıcıFizikselDeğil),
    }
}

fn davranış_koşumunu_doğrula(
    koşum: &Map<String, Value>,
    kaynak_konumu: &str,
    kaynak_sha: &str,
) -> Result<(), KanıtRaporuHatası> {
    let seçilen = dizi(koşum, "seçilen_testler")?;
    if metin_hata(koşum, "tamamlama", KanıtRaporuHatası::KoşumBağıGeçersiz)?
        != "exit_zero_ve_completion_olayı_doğrulandı"
        || seçilen.len() != 1
        || !dizi(koşum, "derleme_birimleri")?.is_empty()
        || !dizi(koşum, "rapor_birimleri")?.is_empty()
        || !dizi(koşum, "yapısal_birimler")?.is_empty()
    {
        return Err(KanıtRaporuHatası::KardinaliteGeçersiz);
    }
    let keşif = nesne(
        koşum
            .get("libtest_keşfi")
            .ok_or(KanıtRaporuHatası::KardinaliteGeçersiz)?,
        KanıtRaporuHatası::KardinaliteGeçersiz,
    )?;
    exact_alanlar(
        keşif,
        &[
            "paket",
            "test_hedefi",
            "libtest_adı",
            "test_artefaktı_sha256",
            "kaynak_haritası_sha256",
            "liste_sha256",
        ],
        KanıtRaporuHatası::KardinaliteGeçersiz,
    )?;
    for alan in ["paket", "test_hedefi", "libtest_adı"] {
        if !kararlı_metin(metin_hata(
            keşif,
            alan,
            KanıtRaporuHatası::KardinaliteGeçersiz,
        )?) {
            return Err(KanıtRaporuHatası::KardinaliteGeçersiz);
        }
    }
    for alan in [
        "test_artefaktı_sha256",
        "kaynak_haritası_sha256",
        "liste_sha256",
    ] {
        if !küçük_hex(
            metin_hata(keşif, alan, KanıtRaporuHatası::KardinaliteGeçersiz)?,
            64,
        ) {
            return Err(KanıtRaporuHatası::KardinaliteGeçersiz);
        }
    }
    let birimler = dizi(koşum, "test_birimleri")?;
    if birimler.len() != 1 {
        return Err(KanıtRaporuHatası::KardinaliteGeçersiz);
    }
    let birim = nesne(&birimler[0], KanıtRaporuHatası::KardinaliteGeçersiz)?;
    exact_alanlar(
        birim,
        &[
            "paket",
            "test_hedefi",
            "libtest_adı",
            "kaynak_konumu",
            "kaynak_sha256",
            "tamamlama_olayı_sha256",
            "akıbet",
        ],
        KanıtRaporuHatası::KardinaliteGeçersiz,
    )?;
    let libtest_adı = metin_hata(keşif, "libtest_adı", KanıtRaporuHatası::KardinaliteGeçersiz)?;
    if seçilen[0].as_str() != Some(libtest_adı)
        || metin_hata(koşum, "cargo_hedefi", KanıtRaporuHatası::KoşumBağıGeçersiz)?
            != metin_hata(keşif, "test_hedefi", KanıtRaporuHatası::KardinaliteGeçersiz)?
        || metin_hata(koşum, "profil", KanıtRaporuHatası::KoşumBağıGeçersiz)? != "test"
        || metin_hata(
            keşif,
            "test_artefaktı_sha256",
            KanıtRaporuHatası::KardinaliteGeçersiz,
        )? != metin_hata(
            koşum,
            "child_yürütülebilir_sha256",
            KanıtRaporuHatası::KoşumBağıGeçersiz,
        )?
        || metin_hata(birim, "paket", KanıtRaporuHatası::KardinaliteGeçersiz)?
            != metin_hata(keşif, "paket", KanıtRaporuHatası::KardinaliteGeçersiz)?
        || metin_hata(birim, "test_hedefi", KanıtRaporuHatası::KardinaliteGeçersiz)?
            != metin_hata(keşif, "test_hedefi", KanıtRaporuHatası::KardinaliteGeçersiz)?
        || metin_hata(birim, "libtest_adı", KanıtRaporuHatası::KardinaliteGeçersiz)?
            != metin_hata(keşif, "libtest_adı", KanıtRaporuHatası::KardinaliteGeçersiz)?
        || metin_hata(
            birim,
            "kaynak_konumu",
            KanıtRaporuHatası::KardinaliteGeçersiz,
        )? != kaynak_konumu
        || metin_hata(
            birim,
            "kaynak_sha256",
            KanıtRaporuHatası::KardinaliteGeçersiz,
        )? != kaynak_sha
        || metin_hata(birim, "akıbet", KanıtRaporuHatası::KardinaliteGeçersiz)? != "başarılı"
        || !küçük_hex(
            metin_hata(
                birim,
                "tamamlama_olayı_sha256",
                KanıtRaporuHatası::KardinaliteGeçersiz,
            )?,
            64,
        )
        || metin_hata(
            birim,
            "tamamlama_olayı_sha256",
            KanıtRaporuHatası::KardinaliteGeçersiz,
        )? != metin_hata(
            koşum,
            "makine_sonucu_sha256",
            KanıtRaporuHatası::KoşumBağıGeçersiz,
        )?
    {
        return Err(KanıtRaporuHatası::KardinaliteGeçersiz);
    }
    let sayaç = nesne(
        koşum
            .get("test_sayaçları")
            .ok_or(KanıtRaporuHatası::KardinaliteGeçersiz)?,
        KanıtRaporuHatası::KardinaliteGeçersiz,
    )?;
    exact_alanlar(
        sayaç,
        &[
            "keşfedilen",
            "çalıştırılan",
            "başarılı",
            "atlanan",
            "beklenen_başarısız",
            "başarısız",
            "hatalı",
            "beklenmeyen_başarılı",
        ],
        KanıtRaporuHatası::KardinaliteGeçersiz,
    )?;
    for alan in ["keşfedilen", "çalıştırılan", "başarılı"] {
        if tam_sayı_hata(sayaç, alan, KanıtRaporuHatası::KardinaliteGeçersiz)? != 1 {
            return Err(KanıtRaporuHatası::KardinaliteGeçersiz);
        }
    }
    for alan in [
        "atlanan",
        "beklenen_başarısız",
        "başarısız",
        "hatalı",
        "beklenmeyen_başarılı",
    ] {
        if tam_sayı_hata(sayaç, alan, KanıtRaporuHatası::KardinaliteGeçersiz)? != 0 {
            return Err(KanıtRaporuHatası::KardinaliteGeçersiz);
        }
    }
    Ok(())
}

fn derleme_koşumunu_doğrula(koşum: &Map<String, Value>) -> Result<(), KanıtRaporuHatası> {
    if metin_hata(koşum, "tamamlama", KanıtRaporuHatası::KoşumBağıGeçersiz)?
        != "eşlenmiş_derleme_probları_doğrulandı"
        || !dizi(koşum, "seçilen_testler")?.is_empty()
        || !koşum.get("libtest_keşfi").is_some_and(Value::is_null)
        || !dizi(koşum, "test_birimleri")?.is_empty()
        || !koşum.get("test_sayaçları").is_some_and(Value::is_null)
        || !dizi(koşum, "rapor_birimleri")?.is_empty()
        || !dizi(koşum, "yapısal_birimler")?.is_empty()
    {
        return Err(KanıtRaporuHatası::KardinaliteGeçersiz);
    }
    let birimler = dizi(koşum, "derleme_birimleri")?;
    if birimler.len() != 2 {
        return Err(KanıtRaporuHatası::KardinaliteGeçersiz);
    }
    let olumsuz = nesne(&birimler[0], KanıtRaporuHatası::KardinaliteGeçersiz)?;
    let olumlu = nesne(&birimler[1], KanıtRaporuHatası::KardinaliteGeçersiz)?;
    let alanlar = [
        "yetenek",
        "test_kimliği",
        "rol",
        "girdi_sha256",
        "çıktı_sha256",
        "çıkış_kodu",
        "beklenen_hata_parmak_izi",
        "gözlenen_hata_parmak_izi",
    ];
    exact_alanlar(olumsuz, &alanlar, KanıtRaporuHatası::KardinaliteGeçersiz)?;
    exact_alanlar(olumlu, &alanlar, KanıtRaporuHatası::KardinaliteGeçersiz)?;
    let girdi = metin_hata(
        olumsuz,
        "girdi_sha256",
        KanıtRaporuHatası::KardinaliteGeçersiz,
    )?;
    let yetenek = metin_hata(olumsuz, "yetenek", KanıtRaporuHatası::KardinaliteGeçersiz)?;
    let beklenen = olumsuz
        .get("beklenen_hata_parmak_izi")
        .and_then(Value::as_str);
    let gözlenen = olumsuz
        .get("gözlenen_hata_parmak_izi")
        .and_then(Value::as_str);
    if metin_hata(koşum, "cargo_hedefi", KanıtRaporuHatası::KoşumBağıGeçersiz)? != yetenek
        || metin_hata(koşum, "profil", KanıtRaporuHatası::KoşumBağıGeçersiz)? != "check"
        || metin_hata(olumsuz, "rol", KanıtRaporuHatası::KardinaliteGeçersiz)? != "yetkisiz_olumsuz"
        || tam_sayı_hata(
            olumsuz,
            "çıkış_kodu",
            KanıtRaporuHatası::KardinaliteGeçersiz,
        )? == 0
        || !kararlı_metin(metin_hata(
            olumsuz,
            "test_kimliği",
            KanıtRaporuHatası::KardinaliteGeçersiz,
        )?)
        || !küçük_hex(
            metin_hata(
                olumsuz,
                "çıktı_sha256",
                KanıtRaporuHatası::KardinaliteGeçersiz,
            )?,
            64,
        )
        || !beklenen.is_some_and(|değer| küçük_hex(değer, 64))
        || beklenen != gözlenen
        || metin_hata(olumlu, "rol", KanıtRaporuHatası::KardinaliteGeçersiz)? != "yetkili_olumlu"
        || tam_sayı_hata(olumlu, "çıkış_kodu", KanıtRaporuHatası::KardinaliteGeçersiz)? != 0
        || metin_hata(olumlu, "yetenek", KanıtRaporuHatası::KardinaliteGeçersiz)? != yetenek
        || !kararlı_metin(metin_hata(
            olumlu,
            "test_kimliği",
            KanıtRaporuHatası::KardinaliteGeçersiz,
        )?)
        || !küçük_hex(
            metin_hata(
                olumlu,
                "çıktı_sha256",
                KanıtRaporuHatası::KardinaliteGeçersiz,
            )?,
            64,
        )
        || !olumlu
            .get("beklenen_hata_parmak_izi")
            .is_some_and(Value::is_null)
        || !olumlu
            .get("gözlenen_hata_parmak_izi")
            .is_some_and(Value::is_null)
        || metin_hata(
            olumlu,
            "girdi_sha256",
            KanıtRaporuHatası::KardinaliteGeçersiz,
        )? != girdi
        || girdi != metin_hata(koşum, "girdi_sha256", KanıtRaporuHatası::KoşumBağıGeçersiz)?
    {
        return Err(KanıtRaporuHatası::KardinaliteGeçersiz);
    }
    Ok(())
}

fn yapısal_koşumu_doğrula(
    koşum: &Map<String, Value>,
    kaynak_konumu: &str,
    kaynak_sha: &str,
) -> Result<(), KanıtRaporuHatası> {
    if metin_hata(koşum, "tamamlama", KanıtRaporuHatası::KoşumBağıGeçersiz)?
        != "exit_zero_ve_tam_drenaj"
        || !dizi(koşum, "seçilen_testler")?.is_empty()
        || !koşum.get("libtest_keşfi").is_some_and(Value::is_null)
        || !dizi(koşum, "test_birimleri")?.is_empty()
        || !koşum.get("test_sayaçları").is_some_and(Value::is_null)
        || !dizi(koşum, "derleme_birimleri")?.is_empty()
        || !dizi(koşum, "rapor_birimleri")?.is_empty()
    {
        return Err(KanıtRaporuHatası::KardinaliteGeçersiz);
    }
    let birimler = dizi(koşum, "yapısal_birimler")?;
    if birimler.len() != 1 {
        return Err(KanıtRaporuHatası::KardinaliteGeçersiz);
    }
    let birim = nesne(&birimler[0], KanıtRaporuHatası::KardinaliteGeçersiz)?;
    exact_alanlar(
        birim,
        &[
            "kural",
            "kaynak_konumu",
            "kaynak_sha256",
            "analiz_artefaktı_sha256",
            "denetlenen_öğe_sayısı",
            "ihlal_sayısı",
            "tamamlama",
        ],
        KanıtRaporuHatası::KardinaliteGeçersiz,
    )?;
    if metin_hata(koşum, "cargo_hedefi", KanıtRaporuHatası::KoşumBağıGeçersiz)?
        != metin_hata(birim, "kural", KanıtRaporuHatası::KardinaliteGeçersiz)?
        || metin_hata(koşum, "profil", KanıtRaporuHatası::KoşumBağıGeçersiz)? != "yapısal"
        || metin_hata(
            birim,
            "kaynak_konumu",
            KanıtRaporuHatası::KardinaliteGeçersiz,
        )? != kaynak_konumu
        || metin_hata(
            birim,
            "kaynak_sha256",
            KanıtRaporuHatası::KardinaliteGeçersiz,
        )? != kaynak_sha
        || !küçük_hex(
            metin_hata(
                birim,
                "analiz_artefaktı_sha256",
                KanıtRaporuHatası::KardinaliteGeçersiz,
            )?,
            64,
        )
        || tam_sayı_hata(
            birim,
            "denetlenen_öğe_sayısı",
            KanıtRaporuHatası::KardinaliteGeçersiz,
        )? <= 0
        || tam_sayı_hata(
            birim,
            "ihlal_sayısı",
            KanıtRaporuHatası::KardinaliteGeçersiz,
        )? != 0
        || metin_hata(birim, "tamamlama", KanıtRaporuHatası::KardinaliteGeçersiz)? != "doğrulandı"
    {
        return Err(KanıtRaporuHatası::KardinaliteGeçersiz);
    }
    Ok(())
}

fn rapor_koşumunu_doğrula(
    koşum: &Map<String, Value>,
    kaynak_konumu: &str,
    kaynak_hedefi: Option<&Value>,
    kayıt_kimliği: &str,
    kayıt_hedefi: &str,
) -> Result<(), KanıtRaporuHatası> {
    if kaynak_konumu != "tools/yon005_olcut_raporu.py"
        || kayıt_kimliği != "yon005.rapor-provider"
        || kayıt_hedefi != "YÖN-005.ACC-019"
    {
        return Err(KanıtRaporuHatası::KaynakBağıGeçersiz);
    }
    if metin_hata(koşum, "komut_kimliği", KanıtRaporuHatası::KoşumBağıGeçersiz)?
        != "yon005.rapor.uyum-olcutleri"
        || metin_hata(
            koşum,
            "komut_görünümü",
            KanıtRaporuHatası::KoşumBağıGeçersiz,
        )? != "resolved-python3 -I <registered-report-generator>"
        || metin_hata(koşum, "tamamlama", KanıtRaporuHatası::KoşumBağıGeçersiz)?
            != "fresh_rapor_terminali_doğrulandı"
        || metin_hata(koşum, "profil", KanıtRaporuHatası::KoşumBağıGeçersiz)? != "rapor"
        || metin_hata(koşum, "araç_zinciri", KanıtRaporuHatası::KoşumBağıGeçersiz)?
            != "python3-isolated-resolved"
    {
        return Err(KanıtRaporuHatası::KoşumBağıGeçersiz);
    }
    if !dizi(koşum, "feature_kümesi")?.is_empty()
        || !dizi(koşum, "seçilen_testler")?.is_empty()
        || !koşum.get("libtest_keşfi").is_some_and(Value::is_null)
        || !dizi(koşum, "test_birimleri")?.is_empty()
        || !koşum.get("test_sayaçları").is_some_and(Value::is_null)
        || !dizi(koşum, "derleme_birimleri")?.is_empty()
        || !dizi(koşum, "yapısal_birimler")?.is_empty()
    {
        return Err(KanıtRaporuHatası::KardinaliteGeçersiz);
    }
    for akış in ["stdout", "stderr"] {
        let özet = nesne(
            koşum
                .get(akış)
                .ok_or(KanıtRaporuHatası::KoşumBağıGeçersiz)?,
            KanıtRaporuHatası::KoşumBağıGeçersiz,
        )?;
        if metin_hata(özet, "tam_sha256", KanıtRaporuHatası::KoşumBağıGeçersiz)? != BOŞ_SHA256
            || tam_sayı_hata(özet, "gözlenen_bayt", KanıtRaporuHatası::KoşumBağıGeçersiz)? != 0
        {
            return Err(KanıtRaporuHatası::KoşumBağıGeçersiz);
        }
    }

    let dış = nesne(
        kaynak_hedefi.ok_or(KanıtRaporuHatası::KaynakBağıGeçersiz)?,
        KanıtRaporuHatası::KaynakBağıGeçersiz,
    )?;
    let hedef = nesne(
        dış
            .get("rapor")
            .ok_or(KanıtRaporuHatası::KaynakBağıGeçersiz)?,
        KanıtRaporuHatası::KaynakBağıGeçersiz,
    )?;
    let rapor_kimliği = metin_hata(hedef, "kimlik", KanıtRaporuHatası::KaynakBağıGeçersiz)?;
    let çıktı_konumu = metin_hata(hedef, "çıktı_konumu", KanıtRaporuHatası::KaynakBağıGeçersiz)?;
    let şema_sürümü = tam_sayı_hata(hedef, "şema_sürümü", KanıtRaporuHatası::KaynakBağıGeçersiz)?;
    if rapor_kimliği != "yon005.uyum-olcut-raporu"
        || çıktı_konumu != "yon005_uyum_olcut_raporu.json"
        || şema_sürümü != 3
    {
        return Err(KanıtRaporuHatası::KaynakBağıGeçersiz);
    }
    let birimler = dizi(koşum, "rapor_birimleri")?;
    if birimler.len() != 1 {
        return Err(KanıtRaporuHatası::KardinaliteGeçersiz);
    }
    let birim = nesne(&birimler[0], KanıtRaporuHatası::KardinaliteGeçersiz)?;
    exact_alanlar(
        birim,
        &[
            "rapor_kimliği",
            "çıktı_konumu",
            "şema_sürümü",
            "kök_revizyonu",
            "içerik_sha256",
            "hedef",
            "kanıt_kimliği",
            "başlatan_yürütülebilir_sha256",
            "başlatıcı_yürütülebilir_sha256",
            "yorumlayıcı_yürütülebilir_sha256",
            "üretici_bildirimi_yürütülebilir_sha256",
            "durum",
        ],
        KanıtRaporuHatası::KardinaliteGeçersiz,
    )?;
    let içerik_sha = metin_hata(
        birim,
        "içerik_sha256",
        KanıtRaporuHatası::KardinaliteGeçersiz,
    )?;
    if metin_hata(koşum, "cargo_hedefi", KanıtRaporuHatası::KoşumBağıGeçersiz)? != rapor_kimliği
        || metin_hata(
            birim,
            "rapor_kimliği",
            KanıtRaporuHatası::KardinaliteGeçersiz,
        )? != rapor_kimliği
        || metin_hata(
            birim,
            "çıktı_konumu",
            KanıtRaporuHatası::KardinaliteGeçersiz,
        )? != çıktı_konumu
        || tam_sayı_hata(birim, "şema_sürümü", KanıtRaporuHatası::KardinaliteGeçersiz)?
            != şema_sürümü
        || metin_hata(
            birim,
            "kök_revizyonu",
            KanıtRaporuHatası::KardinaliteGeçersiz,
        )? != metin_hata(koşum, "kök_revizyonu", KanıtRaporuHatası::KoşumBağıGeçersiz)?
        || !küçük_hex(içerik_sha, 64)
        || içerik_sha
            != metin_hata(
                koşum,
                "makine_sonucu_sha256",
                KanıtRaporuHatası::KoşumBağıGeçersiz,
            )?
        || metin_hata(birim, "hedef", KanıtRaporuHatası::KardinaliteGeçersiz)? != kayıt_hedefi
        || metin_hata(
            birim,
            "kanıt_kimliği",
            KanıtRaporuHatası::KardinaliteGeçersiz,
        )? != kayıt_kimliği
        || !küçük_hex(
            metin_hata(
                birim,
                "başlatan_yürütülebilir_sha256",
                KanıtRaporuHatası::KardinaliteGeçersiz,
            )?,
            64,
        )
        || !küçük_hex(
            metin_hata(
                birim,
                "başlatıcı_yürütülebilir_sha256",
                KanıtRaporuHatası::KardinaliteGeçersiz,
            )?,
            64,
        )
        || !küçük_hex(
            metin_hata(
                birim,
                "yorumlayıcı_yürütülebilir_sha256",
                KanıtRaporuHatası::KardinaliteGeçersiz,
            )?,
            64,
        )
        || metin_hata(
            birim,
            "yorumlayıcı_yürütülebilir_sha256",
            KanıtRaporuHatası::KardinaliteGeçersiz,
        )? != metin_hata(
            koşum,
            "child_yürütülebilir_sha256",
            KanıtRaporuHatası::KoşumBağıGeçersiz,
        )?
        || metin_hata(
            birim,
            "üretici_bildirimi_yürütülebilir_sha256",
            KanıtRaporuHatası::KardinaliteGeçersiz,
        )? != metin_hata(
            birim,
            "yorumlayıcı_yürütülebilir_sha256",
            KanıtRaporuHatası::KardinaliteGeçersiz,
        )?
        || metin_hata(birim, "durum", KanıtRaporuHatası::KardinaliteGeçersiz)? != "doğrulandı"
    {
        return Err(KanıtRaporuHatası::KardinaliteGeçersiz);
    }
    Ok(())
}

fn kaynak_hedefini_doğrula(
    değer: Option<&Value>,
    tür: &KanıtTürü,
) -> Result<(), KanıtRaporuHatası> {
    let hedef = nesne(
        değer.ok_or(KanıtRaporuHatası::KaynakBağıGeçersiz)?,
        KanıtRaporuHatası::KaynakBağıGeçersiz,
    )?;
    let etiket = match tür {
        KanıtTürü::Davranış(_) => "davranış",
        KanıtTürü::Derleme => "derleme",
        KanıtTürü::Yapısal => "yapısal",
        KanıtTürü::Rapor => "rapor",
        KanıtTürü::Sergi => {
            return Err(KanıtRaporuHatası::SağlayıcıFizikselDeğil);
        }
    };
    exact_alanlar(hedef, &[etiket], KanıtRaporuHatası::KaynakBağıGeçersiz)?;
    let iç = nesne(
        hedef
            .get(etiket)
            .ok_or(KanıtRaporuHatası::KaynakBağıGeçersiz)?,
        KanıtRaporuHatası::KaynakBağıGeçersiz,
    )?;
    if etiket == "davranış" {
        exact_alanlar(
            iç,
            &["paket", "test_hedefi", "libtest_adı"],
            KanıtRaporuHatası::KaynakBağıGeçersiz,
        )?;
        for alan in ["paket", "test_hedefi", "libtest_adı"] {
            if !kararlı_metin(metin_hata(iç, alan, KanıtRaporuHatası::KaynakBağıGeçersiz)?) {
                return Err(KanıtRaporuHatası::KaynakBağıGeçersiz);
            }
        }
    } else if etiket == "rapor" {
        exact_alanlar(
            iç,
            &["kimlik", "çıktı_konumu", "şema_sürümü"],
            KanıtRaporuHatası::KaynakBağıGeçersiz,
        )?;
        let çıktı = metin_hata(iç, "çıktı_konumu", KanıtRaporuHatası::KaynakBağıGeçersiz)?;
        if !kararlı_metin(metin_hata(
            iç,
            "kimlik",
            KanıtRaporuHatası::KaynakBağıGeçersiz,
        )?) || !güvenli_göreli_yol(çıktı)
            || tam_sayı_hata(iç, "şema_sürümü", KanıtRaporuHatası::KaynakBağıGeçersiz)? <= 0
        {
            return Err(KanıtRaporuHatası::KaynakBağıGeçersiz);
        }
    } else {
        exact_alanlar(iç, &["kimlik"], KanıtRaporuHatası::KaynakBağıGeçersiz)?;
        if !kararlı_metin(metin_hata(
            iç,
            "kimlik",
            KanıtRaporuHatası::KaynakBağıGeçersiz,
        )?) {
            return Err(KanıtRaporuHatası::KaynakBağıGeçersiz);
        }
    }
    Ok(())
}

fn akışı_doğrula(değer: Option<&Value>) -> Result<(), KanıtRaporuHatası> {
    let akış = nesne(
        değer.ok_or(KanıtRaporuHatası::KoşumBağıGeçersiz)?,
        KanıtRaporuHatası::KoşumBağıGeçersiz,
    )?;
    exact_alanlar(
        akış,
        &["tam_sha256", "gözlenen_bayt"],
        KanıtRaporuHatası::KoşumBağıGeçersiz,
    )?;
    if !küçük_hex(
        metin_hata(akış, "tam_sha256", KanıtRaporuHatası::KoşumBağıGeçersiz)?,
        64,
    ) || tam_sayı_hata(akış, "gözlenen_bayt", KanıtRaporuHatası::KoşumBağıGeçersiz)? < 0
    {
        return Err(KanıtRaporuHatası::KoşumBağıGeçersiz);
    }
    Ok(())
}

fn zamanı_doğrula(değer: Option<&Value>) -> Result<(i64, i64), KanıtRaporuHatası> {
    let zaman = nesne(
        değer.ok_or(KanıtRaporuHatası::KoşumBağıGeçersiz)?,
        KanıtRaporuHatası::KoşumBağıGeçersiz,
    )?;
    exact_alanlar(
        zaman,
        &["saniye", "nanos"],
        KanıtRaporuHatası::KoşumBağıGeçersiz,
    )?;
    let nanos = tam_sayı_hata(zaman, "nanos", KanıtRaporuHatası::KoşumBağıGeçersiz)?;
    let saniye = tam_sayı_hata(zaman, "saniye", KanıtRaporuHatası::KoşumBağıGeçersiz)?;
    if !(0..1_000_000_000).contains(&nanos) {
        return Err(KanıtRaporuHatası::KoşumBağıGeçersiz);
    }
    Ok((saniye, nanos))
}

fn güvenli_göreli_yol(yol: &str) -> bool {
    !yol.is_empty()
        && !Path::new(yol).is_absolute()
        && Path::new(yol)
            .components()
            .all(|bileşen| matches!(bileşen, std::path::Component::Normal(_)))
}

fn küçük_hex(değer: &str, uzunluk: usize) -> bool {
    değer.len() == uzunluk
        && değer
            .bytes()
            .all(|bayt| bayt.is_ascii_digit() || (b'a'..=b'f').contains(&bayt))
}

fn kararlı_metin(değer: &str) -> bool {
    !değer.is_empty() && değer.trim() == değer && !değer.chars().any(char::is_control)
}

fn nesne(
    değer: &Value,
    hata: KanıtRaporuHatası,
) -> Result<&Map<String, Value>, KanıtRaporuHatası> {
    değer.as_object().ok_or(hata)
}

fn dizi<'a>(
    nesne: &'a Map<String, Value>,
    alan: &str,
) -> Result<&'a Vec<Value>, KanıtRaporuHatası> {
    nesne
        .get(alan)
        .and_then(Value::as_array)
        .ok_or(KanıtRaporuHatası::KoşumBağıGeçersiz)
}

fn metin<'a>(nesne: &'a Map<String, Value>, alan: &str) -> Result<&'a str, KanıtRaporuHatası> {
    metin_hata(nesne, alan, KanıtRaporuHatası::AlanKümesiGeçersiz)
}

fn metin_hata<'a>(
    nesne: &'a Map<String, Value>,
    alan: &str,
    hata: KanıtRaporuHatası,
) -> Result<&'a str, KanıtRaporuHatası> {
    nesne.get(alan).and_then(Value::as_str).ok_or(hata)
}

fn tam_sayı(nesne: &Map<String, Value>, alan: &str) -> Result<i64, KanıtRaporuHatası> {
    tam_sayı_hata(nesne, alan, KanıtRaporuHatası::AlanKümesiGeçersiz)
}

fn tam_sayı_hata(
    nesne: &Map<String, Value>,
    alan: &str,
    hata: KanıtRaporuHatası,
) -> Result<i64, KanıtRaporuHatası> {
    nesne.get(alan).and_then(Value::as_i64).ok_or(hata)
}

fn exact_alanlar(
    nesne: &Map<String, Value>,
    alanlar: &[&str],
    hata: KanıtRaporuHatası,
) -> Result<(), KanıtRaporuHatası> {
    if nesne.len() == alanlar.len() && alanlar.iter().all(|alan| nesne.contains_key(*alan)) {
        Ok(())
    } else {
        Err(hata)
    }
}

fn benzersiz_json(baytlar: &[u8]) -> Result<Value, KanıtRaporuHatası> {
    let mut ayrıştırıcı = serde_json::Deserializer::from_slice(baytlar);
    let değer = BenzersizDeğerTohumu
        .deserialize(&mut ayrıştırıcı)
        .map_err(|hata| {
            if hata.to_string().contains("yinelenen JSON anahtarı") {
                KanıtRaporuHatası::YinelenenAlan
            } else {
                KanıtRaporuHatası::JsonGeçersiz
            }
        })?;
    ayrıştırıcı
        .end()
        .map_err(|_| KanıtRaporuHatası::TrailingVeri)?;
    Ok(değer)
}

struct BenzersizDeğerTohumu;

impl<'de> DeserializeSeed<'de> for BenzersizDeğerTohumu {
    type Value = Value;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(BenzersizDeğerZiyaretçisi)
    }
}

struct BenzersizDeğerZiyaretçisi;

impl<'de> Visitor<'de> for BenzersizDeğerZiyaretçisi {
    type Value = Value;
    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("yinelenen anahtarı olmayan JSON değeri")
    }
    fn visit_bool<E>(self, değer: bool) -> Result<Self::Value, E> {
        Ok(Value::Bool(değer))
    }
    fn visit_i64<E>(self, değer: i64) -> Result<Self::Value, E> {
        Ok(Value::Number(değer.into()))
    }
    fn visit_u64<E>(self, değer: u64) -> Result<Self::Value, E> {
        Ok(Value::Number(değer.into()))
    }
    fn visit_f64<E: de::Error>(self, _: f64) -> Result<Self::Value, E> {
        Err(E::custom("ondalıklı JSON desteklenmiyor"))
    }
    fn visit_str<E: de::Error>(self, değer: &str) -> Result<Self::Value, E> {
        Ok(Value::String(değer.to_owned()))
    }
    fn visit_string<E>(self, değer: String) -> Result<Self::Value, E> {
        Ok(Value::String(değer))
    }
    fn visit_none<E>(self) -> Result<Self::Value, E> {
        Ok(Value::Null)
    }
    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(Value::Null)
    }
    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        BenzersizDeğerTohumu.deserialize(deserializer)
    }
    fn visit_seq<A>(self, mut sıra: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut değerler = Vec::new();
        while let Some(değer) = sıra.next_element_seed(BenzersizDeğerTohumu)? {
            değerler.push(değer);
        }
        Ok(Value::Array(değerler))
    }
    fn visit_map<A>(self, mut eşleme: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut nesne = Map::new();
        while let Some(anahtar) = eşleme.next_key::<String>()? {
            if nesne.contains_key(&anahtar) {
                return Err(de::Error::custom(format!(
                    "yinelenen JSON anahtarı: {anahtar}"
                )));
            }
            let değer = eşleme.next_value_seed(BenzersizDeğerTohumu)?;
            nesne.insert(anahtar, değer);
        }
        Ok(Value::Object(nesne))
    }
}

//! Galerinin `ORT-016` simge kaydı ve GPUI varlık kaynağı.
//!
//! Galeri simge çözüm algoritması tanımlamaz: kanonik `SimgeKataloğu`na
//! kayıt yapar ve GPUI'ye varlıkları veren `AssetSource`u kurar. Varlıklar
//! `varliklar/simgeler/ant-design` altındaki MIT lisanslı Ant Design
//! simgeleridir ve derleme zamanında gömülür; çalışma anında ağ erişimi yoktur.

use std::{borrow::Cow, collections::BTreeMap, num::NonZeroUsize, sync::Arc};

use gpui::{AssetSource, SharedString};
use gpui_bilesenleri::{
    AlınmışSimgeVarlığı, BellekSimgeSnapshotDeposu, GpuiSimgeÇizimBağdaştırıcısı, HamSimgeKaynağı,
    SimgeAdAlanıYetkisi, SimgeCacheBütçesi, SimgeCacheKanıtBağı, SimgeDağıtımLisansAkıbeti,
    SimgeDosyaÖzeti, SimgeGörselBiçimi, SimgeKatmanı, SimgeKaynakKaydı, SimgeKimliği,
    SimgeKimliğiKuruluşHatası, SimgeKümesiTanımı, SimgeLisansKaydı, SimgeSnapshotDeposu,
    SimgeSnapshotHazırlamaHatası, SimgeSnapshotYapılandırması, SimgeTanımı, SimgeYönPolitikası,
    SimgeÇizimBağlamı, TanımKimliği,
};
use gpui_bilesenleri_temel::{
    BütçeSınıfı, DerlemeProfili, PerformansBütçesi, PerformansPlatformProfili, PerformansPlatformu,
    ÖlçümKimliği,
};

/// `(kanonik simge kimliği, varlık anahtarı, gömülü SVG)`.
///
/// `ORT-016` varlık anahtarı yol ayracı taşıyamaz; bu yüzden anahtar düz
/// dosya adıdır ve GPUI `svg().path(...)` yolu da aynı addır.
const SİMGELER: &[(&str, &str, &str)] = &[
    (
        "input.clear",
        "close-circle.svg",
        include_str!("../../../varliklar/simgeler/ant-design/outlined/close-circle.svg"),
    ),
    (
        "input.search",
        "search.svg",
        include_str!("../../../varliklar/simgeler/ant-design/outlined/search.svg"),
    ),
    (
        "input.reveal",
        "eye.svg",
        include_str!("../../../varliklar/simgeler/ant-design/outlined/eye.svg"),
    ),
    (
        "input.reveal-off",
        "eye-invisible.svg",
        include_str!("../../../varliklar/simgeler/ant-design/outlined/eye-invisible.svg"),
    ),
    (
        "input.picker",
        "calendar.svg",
        include_str!("../../../varliklar/simgeler/ant-design/outlined/calendar.svg"),
    ),
    // `§16.2.3` gösterge glifleri. Renk tek bilgi kanalı değildir: üç önem
    // üç ayrı glif. Daire-ünlem, üçgen ve daire-i renksiz baskıda da
    // ayrılır; aynı glifin yalnız rengini değiştirmek yasak.
    //
    // `input.clear` ile aynı `close-circle` bilinçli olarak seçilmedi:
    // aynı glifi hem "temizle" eylemi hem "hata" durumu için kullanmak
    // ikisini tek anlama indirirdi.
    (
        "input.status-error",
        "exclamation-circle.svg",
        include_str!("../../../varliklar/simgeler/ant-design/outlined/exclamation-circle.svg"),
    ),
    (
        "input.status-warning",
        "warning.svg",
        include_str!("../../../varliklar/simgeler/ant-design/outlined/warning.svg"),
    ),
    (
        "input.status-info",
        "info-circle.svg",
        include_str!("../../../varliklar/simgeler/ant-design/outlined/info-circle.svg"),
    ),
];

/// Tezgâh yüzeyinin kendi simgeleri.
///
/// Bunlar bileşen simgesi değildir: hizalama tercihini anlatan galeri
/// çizimleridir, bu yüzden `ORT-016` kataloğuna kaydedilmez. Yalnız
/// `svg().path(...)` üzerinden çizilirler.
const TEZGAH_SİMGELERİ: &[(&str, &str)] = &[
    (
        "kip-acik.svg",
        include_str!("../../../varliklar/simgeler/tezgah/kip-acik.svg"),
    ),
    (
        "kip-koyu.svg",
        include_str!("../../../varliklar/simgeler/tezgah/kip-koyu.svg"),
    ),
    (
        "kip-yk-acik.svg",
        include_str!("../../../varliklar/simgeler/tezgah/kip-yk-acik.svg"),
    ),
    (
        "kip-yk-koyu.svg",
        include_str!("../../../varliklar/simgeler/tezgah/kip-yk-koyu.svg"),
    ),
    (
        "kip-sistem.svg",
        include_str!("../../../varliklar/simgeler/tezgah/kip-sistem.svg"),
    ),
    (
        "hizala-genel.svg",
        include_str!("../../../varliklar/simgeler/tezgah/hizala-genel.svg"),
    ),
    (
        "hizala-sol.svg",
        include_str!("../../../varliklar/simgeler/tezgah/hizala-sol.svg"),
    ),
    (
        "hizala-orta.svg",
        include_str!("../../../varliklar/simgeler/tezgah/hizala-orta.svg"),
    ),
    (
        "hizala-sag.svg",
        include_str!("../../../varliklar/simgeler/tezgah/hizala-sag.svg"),
    ),
    (
        "hizala-baslangic.svg",
        include_str!("../../../varliklar/simgeler/tezgah/hizala-baslangic.svg"),
    ),
    (
        "hizala-bitis.svg",
        include_str!("../../../varliklar/simgeler/tezgah/hizala-bitis.svg"),
    ),
    (
        "dikey-ust.svg",
        include_str!("../../../varliklar/simgeler/tezgah/dikey-ust.svg"),
    ),
    (
        "dikey-orta.svg",
        include_str!("../../../varliklar/simgeler/tezgah/dikey-orta.svg"),
    ),
    (
        "dikey-alt.svg",
        include_str!("../../../varliklar/simgeler/tezgah/dikey-alt.svg"),
    ),
    (
        "yazi-koyu.svg",
        include_str!("../../../varliklar/simgeler/tezgah/yazi-koyu.svg"),
    ),
    (
        "yazi-ince.svg",
        include_str!("../../../varliklar/simgeler/tezgah/yazi-ince.svg"),
    ),
    (
        "yazi-egik.svg",
        include_str!("../../../varliklar/simgeler/tezgah/yazi-egik.svg"),
    ),
    (
        "yazi-alti-cizili.svg",
        include_str!("../../../varliklar/simgeler/tezgah/yazi-alti-cizili.svg"),
    ),
    (
        "yazi-ustu-cizili.svg",
        include_str!("../../../varliklar/simgeler/tezgah/yazi-ustu-cizili.svg"),
    ),
    (
        "yazi-buyut.svg",
        include_str!("../../../varliklar/simgeler/tezgah/yazi-buyut.svg"),
    ),
    (
        "yazi-kucult.svg",
        include_str!("../../../varliklar/simgeler/tezgah/yazi-kucult.svg"),
    ),
    (
        "kose-yaricap.svg",
        include_str!("../../../varliklar/simgeler/tezgah/kose-yaricap.svg"),
    ),
    (
        "acilir.svg",
        include_str!("../../../varliklar/simgeler/tezgah/acilir.svg"),
    ),
];

/// GPUI'nin `svg()` öğesine gömülü simge baytlarını veren varlık kaynağı.
#[derive(Clone, Default)]
pub struct GaleriVarlıkKaynağı;

impl AssetSource for GaleriVarlıkKaynağı {
    fn load(&self, yol: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        if let Some((_, içerik)) = TEZGAH_SİMGELERİ.iter().find(|(ad, _)| *ad == yol) {
            return Ok(Some(Cow::Borrowed(içerik.as_bytes())));
        }
        Ok(SİMGELER
            .iter()
            .find(|(_, anahtar, _)| *anahtar == yol)
            .map(|(_, _, svg)| Cow::Borrowed(svg.as_bytes())))
    }

    fn list(&self, yol: &str) -> gpui::Result<Vec<SharedString>> {
        Ok(SİMGELER
            .iter()
            .filter(|(_, anahtar, _)| anahtar.starts_with(yol))
            .map(|(_, anahtar, _)| SharedString::new_static(anahtar))
            .collect())
    }
}

fn tanım(ad_alanı: &str, yerel_ad: &str) -> TanımKimliği {
    TanımKimliği::denetimli(Arc::from(ad_alanı), Arc::from(yerel_ad))
        .expect("galeri simge tanımı geçerlidir")
}

fn galeri_simge_yetkisi() -> Result<SimgeAdAlanıYetkisi, SimgeKimliğiKuruluşHatası> {
    SimgeAdAlanıYetkisi::kökten(Arc::from("input"), tanım("input", "galeri-simge-koku"))
}

/// Galerinin kayıtlı `input.*` semantik kimliğini aynı ad-alanı yetkisinden
/// üretir. Çağıran ham snapshot içeriğine veya varlık yoluna erişmez.
pub fn galeri_simge_kimliği(kimlik: &str) -> Result<SimgeKimliği, SimgeKimliğiKuruluşHatası> {
    let yerel_ad = kimlik
        .strip_prefix("input.")
        .ok_or(SimgeKimliğiKuruluşHatası::YetkisizAdAlanı)?;
    galeri_simge_yetkisi()?.simge_kimliği(Arc::from(yerel_ad))
}

fn performans_bütçesi(ad: &str, sınıf: BütçeSınıfı) -> Arc<PerformansBütçesi> {
    Arc::new(PerformansBütçesi {
        ölçüm: ÖlçümKimliği::yeni(ad).expect("galeri simge ölçüm kimliği geçerlidir"),
        sınıf,
        hedef: 4.0,
        üst_sınır: 8.0,
        birim: "ms".into(),
        platform_profili: PerformansPlatformProfili {
            platform: PerformansPlatformu::Masaüstü,
            derleme: DerlemeProfili::Release,
            donanım: "galeri-runtime-profili".into(),
        },
    })
}

fn cache_bütçesi() -> SimgeCacheBütçesi {
    let pozitif = |değer| NonZeroUsize::new(değer).expect("cache bütçesi pozitiftir");
    SimgeCacheBütçesi {
        mantıksal_plan_girdi_tavanı: pozitif(64),
        mantıksal_plan_bayt_tavanı: pozitif(65_536),
        geometri_girdi_tavanı: pozitif(64),
        geometri_bayt_tavanı: pozitif(65_536),
    }
}

fn cache_kanıt_bağı() -> SimgeCacheKanıtBağı {
    SimgeCacheKanıtBağı {
        mantıksal_cpu: performans_bütçesi(
            "galeri.simge.mantiksal.cpu",
            BütçeSınıfı::ÇizimKaresi,
        ),
        mantıksal_bellek: performans_bütçesi(
            "galeri.simge.mantiksal.bellek",
            BütçeSınıfı::Tahsis,
        ),
        geometri_cpu: performans_bütçesi("galeri.simge.geometri.cpu", BütçeSınıfı::ÇizimKaresi),
        geometri_bellek: performans_bütçesi("galeri.simge.geometri.bellek", BütçeSınıfı::Tahsis),
        // Bu bağ fiziksel present/runtime kanıtı iddia etmez.
        fiziksel_tamamlama: Arc::from([]),
    }
}

/// Gömülü Ant Design varlıklarını ORT-016 alım, provenance, hazırlık ve
/// atomik yayın kapılarından geçirip galerinin çizim bağını kurar.
///
/// Kimlikler BİL-010'un sahip olduğu `input` ad alanındadır; galeri yeni
/// semantik ad uydurmaz. Ham SVG ya da dosya yolu çözüm sonucuna çıkmaz;
/// yol yalnız GPUI bağdaştırıcısında kalır.
pub fn galeri_simge_çizim_bağlamı() -> Arc<SimgeÇizimBağlamı> {
    Arc::new(
        galeri_simge_çizim_bağlamını_kur()
            .unwrap_or_else(|hata| panic!("galeri simge snapshotı kurulamadı: {hata:?}")),
    )
}

fn galeri_simge_çizim_bağlamını_kur() -> Result<SimgeÇizimBağlamı, SimgeSnapshotHazırlamaHatası> {
    let kök = tanım("input", "galeri-simge-koku");
    let yetki =
        galeri_simge_yetkisi().map_err(|_| SimgeSnapshotHazırlamaHatası::YetkisizAdAlanı)?;
    let küme_kimliği = yetki
        .küme_kimliği(Arc::from("galeri-ant"))
        .map_err(|_| SimgeSnapshotHazırlamaHatası::GeçersizKimlik)?;
    let depo = BellekSimgeSnapshotDeposu::kur(&kök);
    let taban = depo.güncel();
    let mut hazırlık = depo.hazırlık(taban.kimlik().clone(), yetki.clone())?;
    let mut varlıklar: Vec<AlınmışSimgeVarlığı> = Vec::with_capacity(SİMGELER.len());
    let mut varlık_yolları = BTreeMap::new();

    for (_, anahtar, svg) in SİMGELER {
        let varlık_anahtarı = yetki
            .varlık_anahtarı(Arc::from(*anahtar))
            .map_err(|_| SimgeSnapshotHazırlamaHatası::GeçersizKimlik)?;
        let varlık = HamSimgeKaynağı {
            anahtar: varlık_anahtarı.clone(),
            biçim: SimgeGörselBiçimi::Çizgisel,
            katman: SimgeKatmanı::Tek,
            svg: Arc::from(*svg),
        }
        .güvenli_al()?;
        hazırlık.varlık_ekle(varlık.clone())?;
        varlık_yolları.insert(varlık_anahtarı, SharedString::new(*anahtar));
        varlıklar.push(varlık);
    }

    let simgeler: Vec<SimgeTanımı> = SİMGELER
        .iter()
        .zip(&varlıklar)
        .map(|((kimlik, _, _), varlık)| {
            let yerel_ad = kimlik
                .strip_prefix("input.")
                .expect("galeri giriş simgesi input ad alanındadır");
            Ok(SimgeTanımı {
                kimlik: yetki
                    .simge_kimliği(Arc::from(yerel_ad))
                    .map_err(|_| SimgeSnapshotHazırlamaHatası::GeçersizKimlik)?,
                küme: küme_kimliği.clone(),
                yön_politikası: SimgeYönPolitikası::Değişmez,
                temel_biçim: SimgeGörselBiçimi::Çizgisel,
                varlıklar: Arc::from([varlık.tanım().clone()]),
            })
        })
        .collect::<Result<_, SimgeSnapshotHazırlamaHatası>>()?;
    let küme = SimgeKümesiTanımı {
        kimlik: küme_kimliği.clone(),
        sürüm: tanım("input", "ant-design-6c18c63"),
        kaynak: SimgeKaynakKaydı {
            resmi_kaynak: "https://github.com/ant-design/ant-design-icons".into(),
            exact_sürüm: "6c18c63fbcfcf71dae09cd6bd6d63a48f8b688f1".into(),
            alınan_kaynak_yolu: "packages/icons-svg/svg/outlined".into(),
            alınma_tarihi: "2026-07-30".into(),
        },
        lisans: SimgeLisansKaydı {
            spdx_kimliği: "MIT".into(),
            lisans_dosyası: "varliklar/simgeler/ant-design/LICENSE".into(),
            lisans_dosyası_özeti: SimgeDosyaÖzeti::sha256(include_bytes!(
                "../../../varliklar/simgeler/ant-design/LICENSE"
            )),
            telif_bildirimi: "Copyright (c) 2018-present Ant UED".into(),
            dağıtım_akıbeti: SimgeDağıtımLisansAkıbeti::Uyumlu,
            yon003_borç_kimliği: None,
        },
        normalleştirme_manifesti: varlıklar
            .iter()
            .map(|varlık| varlık.normalleştirme().clone())
            .collect(),
        simgeler: Arc::from(simgeler),
    };
    hazırlık.küme_ekle(küme)?;
    hazırlık.yapılandır(SimgeSnapshotYapılandırması {
        etkin_küme: küme_kimliği.clone(),
        taban_küme: küme_kimliği,
        nötr_eksik_simge: yetki
            .simge_kimliği(Arc::from("clear"))
            .map_err(|_| SimgeSnapshotHazırlamaHatası::GeçersizKimlik)?,
        cache_bütçesi: cache_bütçesi(),
        cache_kanıt_bağı: cache_kanıt_bağı(),
    })?;
    let snapshot = depo.yayımla(hazırlık.hazırla()?)?;

    Ok(SimgeÇizimBağlamı {
        snapshot,
        bağdaştırıcı: Arc::new(GpuiSimgeÇizimBağdaştırıcısı::kur(varlık_yolları)),
    })
}

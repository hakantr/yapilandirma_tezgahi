//! Galerinin `ORT-016` simge kaydı ve GPUI varlık kaynağı.
//!
//! Galeri simge çözüm algoritması tanımlamaz: kanonik `SimgeKataloğu`na
//! kayıt yapar ve GPUI'ye varlıkları veren `AssetSource`u kurar. Varlıklar
//! `varliklar/simgeler/ant-design` altındaki MIT lisanslı Ant Design
//! simgeleridir ve derleme zamanında gömülür; çalışma anında ağ erişimi yoktur.

use std::{borrow::Cow, sync::Arc};

use gpui::{AssetSource, Div, Global, Pixels, SharedString, canvas, div, prelude::*};
use gpui_bilesenleri_temel::{
    AlınmışSimgeVarlığı, BellekSimgeSnapshotDeposu, BileşimKökü, DoğrulanmışMantıksalSimgeBoyutu,
    HamSimgeKaynağı, SimgeBoyutTercihi, SimgeCacheProfilRolü, SimgeDağıtımLisansAkıbeti,
    SimgeDosyaÖzeti, SimgeGerekliliği, SimgeGörselBiçimi, SimgeHazırlıkTahsisSınırları,
    SimgeKatmanı, SimgeKaynakKaydı, SimgeKimliği, SimgeKümesiTanımı, SimgeLisansKaydı,
    SimgePlatformHizmetKökü, SimgeRolü, SimgeSnapshotDeposu, SimgeSnapshotHazırlamaHatası,
    SimgeSnapshotTemelYapılandırması, SimgeTanımı, SimgeYetkiKökü, SimgeYönPolitikası,
    SimgeÇizimHizmeti, SimgeÇizimYeri, SimgeÇözümAkıbeti, Simgeİsteği, TanımKimliği,
    TemaAnlıkGörüntüsü, YerleşikPerformansBütçeRolü, YerleşikPerformansBütçeSağlayıcısı,
    ÇözülmüşYazıYönü, ÜrünKatalogKuruluşu,
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
    (
        "input.product-action",
        "product.svg",
        include_str!("../../../varliklar/simgeler/ant-design/outlined/product.svg"),
    ),
    (
        "input.missing",
        "question-circle.svg",
        include_str!("../../../varliklar/simgeler/ant-design/outlined/question-circle.svg"),
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
const ÜRÜN_MANİFESTİ: &[u8] =
    b"input:galeri|9790b3dcecd06e3eed1d7d892ff7cf05d7e2521071a18b3abccb2bd5e3c7edd5\n";
const ÜRÜN_MANİFEST_ÖZETİ: [u8; 32] = [
    0xba, 0xcc, 0x7d, 0x85, 0x80, 0xa2, 0x0c, 0xfe, 0x13, 0xe9, 0xcd, 0x24, 0x92, 0xcf, 0xa8, 0x16,
    0x85, 0x12, 0xbf, 0x9b, 0x15, 0x75, 0xe2, 0xa5, 0x2d, 0xb1, 0xb7, 0x39, 0x57, 0xae, 0xfc, 0x50,
];

struct GaleriSimgeKökü {
    hizmet: Arc<SimgeÇizimHizmeti>,
}

impl Global for GaleriSimgeKökü {}

/// Aynı `App` için tek root-yetkili ORT-016/K09 hizmetini döndürür.
pub fn galeri_simge_hizmeti(bağlam: &mut gpui::App) -> Arc<SimgeÇizimHizmeti> {
    if !bağlam.has_global::<GaleriSimgeKökü>() {
        let hizmet = galeri_simge_hizmetini_kur(bağlam)
            .unwrap_or_else(|hata| panic!("galeri simge hizmeti kurulamadı: {hata:?}"));
        bağlam.set_global(GaleriSimgeKökü { hizmet });
    }
    Arc::clone(&bağlam.global::<GaleriSimgeKökü>().hizmet)
}

/// Yalnız yaşayan snapshot'ta gerçekten kayıtlı `input.*` tanımını çözer.
/// Kayıt yokluğunda yeni kimlik türetilmez.
pub fn galeri_simge_kimliği(kimlik: &str, bağlam: &mut gpui::App) -> Option<SimgeKimliği> {
    let yerel_ad = kimlik.strip_prefix("input.")?;
    galeri_simge_hizmeti(bağlam)
        .kayıtlı_simge_kimliği(&tanım("input", yerel_ad))
        .ok()
        .flatten()
}

fn galeri_simge_hizmetini_kur(
    bağlam: &mut gpui::App,
) -> Result<Arc<SimgeÇizimHizmeti>, SimgeSnapshotHazırlamaHatası> {
    ÜrünKatalogKuruluşu::gömülü_manifestten(ÜRÜN_MANİFESTİ, ÜRÜN_MANİFEST_ÖZETİ)
        .map_err(|_| SimgeSnapshotHazırlamaHatası::GeçersizKimlik)?
        .appte_yayımla(bağlam)
        .map_err(|_| SimgeSnapshotHazırlamaHatası::YetkisizAdAlanı)?;
    let mut kök = BileşimKökü::edin(bağlam)
        .unwrap_or_else(|| BileşimKökü::kur(bağlam).expect("galeri App bileşim kökü kurulmalı"));
    let katalog = kök
        .simge_ürün_katalogunu_bağla(bağlam)
        .map_err(|_| SimgeSnapshotHazırlamaHatası::YetkisizAdAlanı)?;
    let ürün_girdisi = kök
        .kayıtlı_ürünü_çöz(&katalog, &tanım("input", "galeri"))
        .map_err(|_| SimgeSnapshotHazırlamaHatası::YetkisizAdAlanı)?;
    let ürün = kök
        .ürünü_kaydet(&katalog, ürün_girdisi)
        .map_err(|_| SimgeSnapshotHazırlamaHatası::YetkisizAdAlanı)?;
    let ürün_yetkisi = kök
        .ürün_yetkisi(&ürün)
        .map_err(|_| SimgeSnapshotHazırlamaHatası::YetkisizAdAlanı)?;
    let mut simge_kökü = SimgeYetkiKökü::bileşim_kökünde_kur(&mut kök, &katalog)
        .map_err(|_| SimgeSnapshotHazırlamaHatası::YetkisizAdAlanı)?;
    let ad_alanı = simge_kökü
        .ürün_ad_alanını_kaydet(ürün_yetkisi, Arc::from("input"))
        .map_err(|_| SimgeSnapshotHazırlamaHatası::YetkisizAdAlanı)?;
    let yetki = simge_kökü
        .yetki_ver(&ad_alanı)
        .map_err(|_| SimgeSnapshotHazırlamaHatası::YetkisizAdAlanı)?;
    let küme_kimliği = yetki
        .küme_kimliği(Arc::from("galeri-ant"))
        .map_err(|_| SimgeSnapshotHazırlamaHatası::GeçersizKimlik)?;
    let tahsis = simge_kökü
        .hazırlık_tahsis_kaydını_kur(
            SimgeHazırlıkTahsisSınırları::denetimli(65_536, 262_144, 64)
                .expect("galeri simge tahsis sınırları geçerlidir"),
        )
        .map_err(|_| SimgeSnapshotHazırlamaHatası::YetkisizAdAlanı)?;
    let depo = BellekSimgeSnapshotDeposu::bileşim_kökünde_kur(&mut simge_kökü)
        .map_err(|_| SimgeSnapshotHazırlamaHatası::YetkisizAdAlanı)?;
    let taban = depo.güncel();
    let mut hazırlık = depo
        .hazırlık(taban.kimlik().clone(), yetki.clone())?
        .hazırlık_tahsisini_bağla(&tahsis)?;
    let mut varlıklar: Vec<AlınmışSimgeVarlığı> = Vec::with_capacity(SİMGELER.len());

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
    let temel = SimgeSnapshotTemelYapılandırması::denetimli(
        küme_kimliği.clone(),
        küme_kimliği,
        yetki
            .simge_kimliği(Arc::from("missing"))
            .map_err(|_| SimgeSnapshotHazırlamaHatası::GeçersizKimlik)?,
    )?;
    let cache_profili = simge_kökü
        .cache_profil_kaydı(SimgeCacheProfilRolü::YayımlanmışSimgeCache)
        .map_err(|_| SimgeSnapshotHazırlamaHatası::YetkisizAdAlanı)?;
    let sağlayıcı = YerleşikPerformansBütçeSağlayıcısı::yerleşik();
    let bütçe = |rol| sağlayıcı.bütçe_kaydı(rol).bütçe().clone();
    let hazırlık = hazırlık.cache_yapılandırmasını_denetle(
        temel,
        cache_profili,
        bütçe(YerleşikPerformansBütçeRolü::Ort016MantıksalCpu),
        bütçe(YerleşikPerformansBütçeRolü::Ort016MantıksalBellek),
        bütçe(YerleşikPerformansBütçeRolü::Ort016GeometriCpu),
        bütçe(YerleşikPerformansBütçeRolü::Ort016GeometriBellek),
        Arc::from([]),
    )?;
    let yayın = depo.yayımla(hazırlık.hazırla()?)?;
    let platform = SimgePlatformHizmetKökü::yerleşik(bağlam)
        .map_err(|_| SimgeSnapshotHazırlamaHatası::YetkisizAdAlanı)?;
    let hizmet = SimgeÇizimHizmeti::kur(yayın, platform.hizmetleri())
        .map_err(|_| SimgeSnapshotHazırlamaHatası::YetkisizAdAlanı)?;
    Ok(Arc::new(hizmet))
}

/// Görünür galerideki iki özdeş tuvalin aynı K09 hizmetini tüketmesini sağlar.
pub(crate) fn galeri_simge_öğesi(
    kimlik: SimgeKimliği,
    hizmet: Arc<SimgeÇizimHizmeti>,
    tema: Arc<TemaAnlıkGörüntüsü>,
    ölçü: Pixels,
) -> Div {
    let boyut = DoğrulanmışMantıksalSimgeBoyutu::denetimli(ölçü.as_f32())
        .expect("galeri simge ölçüsü geçerlidir");
    let istek = Simgeİsteği::yeni(
        kimlik,
        SimgeGörselBiçimi::Çizgisel,
        SimgeBoyutTercihi::Özel(boyut),
        SimgeRolü::Olağan,
        None,
        ÇözülmüşYazıYönü::SoldanSağa,
        SimgeGerekliliği::Zorunlu,
        tema.bağlam.kip,
    );
    div().size(ölçü).child(
        canvas(
            |_, _, _| {},
            move |sınırlar, _, pencere, bağlam| {
                let çözülmüş = match hizmet.çöz(&istek) {
                    Ok(SimgeÇözümAkıbeti::TamÇözüldü(simge))
                    | Ok(SimgeÇözümAkıbeti::YedekleÇözüldü { simge, .. }) => simge,
                    Ok(SimgeÇözümAkıbeti::ÇizimYok) | Err(_) => return,
                };
                let Ok(yer) = SimgeÇizimYeri::tema_için_denetimli(
                    sınırlar,
                    sınırlar,
                    pencere.scale_factor(),
                    Arc::clone(&tema),
                ) else {
                    return;
                };
                let _ = hizmet.çiz(&çözülmüş, yer, pencere, bağlam);
            },
        )
        .size_full(),
    )
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GaleriSimgeCacheGözlemi {
    pub mantıksal_vuruş: u64,
    pub mantıksal_kaçırma: u64,
    pub mantıksal_atım: u64,
    pub mantıksal_girdi: u64,
    pub mantıksal_payload_baytı: u64,
    pub geometri_vuruş: u64,
    pub geometri_kaçırma: u64,
    pub geometri_atım: u64,
    pub geometri_girdi: u64,
    pub geometri_payload_baytı: u64,
    pub dış_arc_payload_baytı: u64,
    pub geçici_payload_baytı: u64,
}

/// Yaşayan snapshot ile atomik değişen iki cache'in sınırlı salt-okunur gözlemi.
pub fn galeri_simge_cache_gözlemi(bağlam: &mut gpui::App) -> GaleriSimgeCacheGözlemi {
    let hizmet = galeri_simge_hizmeti(bağlam);
    let Ok(snapshot) = hizmet.cache_bellek_snapshotı() else {
        return GaleriSimgeCacheGözlemi::default();
    };
    let mantıksal = snapshot.cache().mantıksal();
    let geometri = snapshot.cache().geometri();
    GaleriSimgeCacheGözlemi {
        mantıksal_vuruş: mantıksal.vuruş(),
        mantıksal_kaçırma: mantıksal.kaçırma(),
        mantıksal_atım: mantıksal.atım(),
        mantıksal_girdi: mantıksal.yaşayan_girdi(),
        mantıksal_payload_baytı: mantıksal.yaşayan_payload_baytı(),
        geometri_vuruş: geometri.vuruş(),
        geometri_kaçırma: geometri.kaçırma(),
        geometri_atım: geometri.atım(),
        geometri_girdi: geometri.yaşayan_girdi(),
        geometri_payload_baytı: geometri.yaşayan_payload_baytı(),
        dış_arc_payload_baytı: snapshot.dış_arc().yaşayan_payload_baytı(),
        geçici_payload_baytı: snapshot.hazırlık().geçici_payload_baytı(),
    }
}

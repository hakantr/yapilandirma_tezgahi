//! Galerinin `ORT-016` simge kaydı ve GPUI varlık kaynağı.
//!
//! Galeri simge çözüm algoritması tanımlamaz: kanonik `SimgeKataloğu`na
//! kayıt yapar ve GPUI'ye varlıkları veren `AssetSource`u kurar. Varlıklar
//! `varliklar/simgeler/ant-design` altındaki MIT lisanslı Ant Design
//! simgeleridir ve derleme zamanında gömülür; çalışma anında ağ erişimi yoktur.

use std::{borrow::Cow, collections::BTreeMap, sync::Arc};

use gpui::{AssetSource, SharedString};
use gpui_bilesenleri::{
    AlınmışSimgeVarlığı, HamSimgeVarlığı, SimgeGeometrisi, SimgeKataloğu, SimgeKimliği,
    SimgeKümesiKimliği, SimgeKümesiTanımı, SimgeKümesiTeması, SimgeTanımı, SimgeVarlıkBileşimi,
    SimgeVaryantı,
};

/// Ant Design simge kümesinin `viewBox` kenarı.
const GÖRÜNTÜ_KUTUSU_KENARI: f32 = 1024.0;

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

/// `ORT-016` kataloğunu galeri simgeleriyle kurar.
///
/// SVG içeriği `HamSimgeVarlığı::güvenli_al` sınırından geçer; script,
/// uzak referans veya `url(...)` taşıyan varlık kataloğa giremez.
pub fn galeri_simge_kataloğu() -> Arc<SimgeKataloğu> {
    let küme = SimgeKümesiTanımı {
        kimlik: SimgeKümesiKimliği::yeni("galeri-ant").expect("küme kimliği kararlıdır"),
        kaynak: "ant-design-icons".into(),
        kaynak_sürümü: "6c18c63".into(),
        lisans: "MIT".into(),
        telif: "Copyright (c) 2017-present Ant Design".into(),
        görüntü_kutusu_kenarı: GÖRÜNTÜ_KUTUSU_KENARI,
        temalar: Arc::from([SimgeKümesiTeması::Çizgisel].as_slice()),
    };
    let eksik = SimgeKimliği::yeni("input.clear").expect("eksik simge kimliği kararlıdır");
    let mut katalog = SimgeKataloğu::yeni(küme, eksik).expect("galeri simge kümesi geçerlidir");

    for (kimlik, anahtar, svg) in SİMGELER {
        // Küme derleme zamanında sabittir: kayıt başarısızlığı sessizce
        // geçilirse yardımcı eylem simgeleri çizilmeden kalır ve bu ancak
        // gözle fark edilir. Bu yüzden hata burada görünür olur.
        let varlık = güvenli_varlık(anahtar, svg)
            .unwrap_or_else(|hata| panic!("{anahtar} varlık sınırından geçemedi: {hata:?}"));
        let mut varyantlar = BTreeMap::new();
        varyantlar.insert(SimgeVaryantı::Olağan, SimgeVarlıkBileşimi::Tek(varlık));
        let tanım = SimgeTanımı {
            kimlik: SimgeKimliği::yeni(*kimlik).expect("simge kimliği kararlıdır"),
            küme: SimgeKümesiKimliği::yeni("galeri-ant").expect("küme kimliği kararlıdır"),
            mantıksal_yönlü: false,
            aile_fallback: None,
            varyantlar,
        };
        katalog
            .kaydet(tanım)
            .unwrap_or_else(|hata| panic!("{kimlik} kataloğa kaydedilemedi: {hata:?}"));
    }
    Arc::new(katalog)
}

fn güvenli_varlık(
    anahtar: &str,
    svg: &str,
) -> Result<AlınmışSimgeVarlığı, gpui_bilesenleri::SimgeÇözümlemeHatası> {
    HamSimgeVarlığı {
        anahtar: SharedString::new(anahtar),
        svg: SharedString::new(svg),
        geometri: SimgeGeometrisi {
            geometri_özeti: Arc::from(anahtar),
            görüntü_kutusu_kenarı: GÖRÜNTÜ_KUTUSU_KENARI as u32,
        },
    }
    .güvenli_al()
}

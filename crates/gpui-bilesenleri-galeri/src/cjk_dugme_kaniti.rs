//! `BİL-040` kısa CJK etiketinin gerçek sağlayıcı/tüketici kanıtı.
//!
//! Galeri aralık algoritmasını kopyalamaz. Kaynak, `ORT-002` kalıcı Unicode
//! kökünde mühürlenir; görünür etiket `ORT-017` hizmet kökünden çözülür ve
//! sonuç `BİL-040` yapılandırmasının kapalı/açık profil alanından geçirilir.

use std::sync::{Arc, OnceLock};

use gpui_bilesenleri::{
    DüğmeMetniHizalaması, DüğmeMetniTaşması, DüğmeMetniYapılandırması, EtiketBoşlukPolitikası,
};
use gpui_bilesenleri_temel::{
    AnatomiParçasıKimliği, AnatomiParçasıSınıfı, AnatomiParçasıTanımı, AnatomiSürümü, BağlamSürümü,
    BileşenGörselAnatomisi, BileşenTürüKimliği, CanlıBağlamDamgası, CjkEtiketAralığıİsteği,
    CjkKısaEtiketAralığıProfili, CjkKısaEtiketAralığıProfiliİsteği, CjkKısaEtiketHizmetKökü,
    GörünümYerleşimŞablonuKimliği, GösterimHassasiyetSınıfı, GösterimParçasıKimliği, GüvenliMetin,
    MutlakGösterimAralığı, RenderStratejisiKimliği, TanımKimliği, UnicodeVeYerelMetinHizmetleri,
    Utf8ParçaAkışı, ÇözülmüşYazıYönü, ÖrnekKimliği, ÖrnekKimliğiFabrikası,
};

// Tek görünür örnek bütün kabul matrisini taşır: CJK-CJK, kaynak boşluğu,
// Latin, sayı, noktalama, birleşen işaret ve ZWJ emojisi.
const KAYNAK: &str = "保存 A中1文,e\u{301}👨‍👩‍👧‍👦";

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CjkDüğmeSergiKanıtı {
    pub kaynak: String,
    pub kapalı: String,
    pub açık: String,
    pub varsayılan_kapalı: bool,
    pub kapalıda_aralık_eklendi: bool,
    pub açıkta_aralık_eklendi: bool,
    pub eklenen_aralık_kaynak_tüketmiyor: bool,
    pub kapalı_tüketici_none: bool,
    pub açık_tüketici_bağlı: bool,
    pub erişilebilir_ad_kaynağı_koruyor: bool,
    pub kopya_kaynağı_koruyor: bool,
    pub karakter_matrisi_korundu: bool,
}

pub(crate) fn bil040_cjk_sergi_kanıtı() -> Result<CjkDüğmeSergiKanıtı, String> {
    static KANIT: OnceLock<Result<CjkDüğmeSergiKanıtı, String>> = OnceLock::new();
    KANIT.get_or_init(kanıtı_üret).clone()
}

fn kanıtı_üret() -> Result<CjkDüğmeSergiKanıtı, String> {
    let kimlik_fabrikası = ÖrnekKimliğiFabrikası::yeni_süreç_kapsamı()
        .map_err(|hata| format!("kimlik kapsamı kurulamadı: {hata:?}"))?;
    let bağlam = kimlik_fabrikası
        .sonraki()
        .map_err(|hata| format!("bağlam kimliği kurulamadı: {hata:?}"))?;
    let unicode_kökü = UnicodeVeYerelMetinHizmetleri::yerlesik(kimlik_fabrikası);

    let anatomi_parçası = AnatomiParçasıKimliği::denetimli("ort017.cjk", "etiket")
        .map_err(|hata| format!("CJK anatomi parçası kurulamadı: {hata:?}"))?;
    let gösterim_parçası = unicode_kökü
        .gösterim_parçası_kimliği_fabrikası()
        .parça(anatomi_parçası.0.clone());
    let anatomi = cjk_anatomisi(anatomi_parçası.clone())?;

    let kapalı_kök = CjkKısaEtiketHizmetKökü::yerleşik();
    let kapalı_profil_isteği = kapalı_kök.profil_isteği();
    let kapalı_çözümleyici = kapalı_kök.çözümleyici();
    let (kapalı_istek, kapalı_kopya_kaynağı) =
        cjk_isteği(&unicode_kökü, bağlam, 1, KAYNAK, gösterim_parçası.clone())?;
    let kapalı_kayıt = kapalı_çözümleyici
        .parçalı_kaydı_mühürle(
            &kapalı_profil_isteği,
            &anatomi,
            &anatomi_parçası,
            &gösterim_parçası,
            kapalı_istek,
        )
        .map_err(|hata| format!("kapalı CJK kaydı mühürlenemedi: {hata:?}"))?;
    let kapalı_sonuç = kapalı_çözümleyici
        .çöz(&kapalı_kayıt)
        .map_err(|hata| format!("kapalı CJK etiketi çözülemedi: {hata:?}"))?;
    kapalı_çözümleyici
        .güncel_kayıtla_doğrula(&kapalı_kayıt, &kapalı_sonuç)
        .map_err(|hata| format!("kapalı CJK sonucu bayat: {hata:?}"))?;

    let açık_kök = CjkKısaEtiketHizmetKökü::kayıtlı(CjkKısaEtiketAralığıProfili::Açık);
    let açık_profil_isteği = açık_kök.profil_isteği();
    let açık_çözümleyici = açık_kök.çözümleyici();
    let (açık_istek, açık_kopya_kaynağı) =
        cjk_isteği(&unicode_kökü, bağlam, 2, KAYNAK, gösterim_parçası.clone())?;
    let açık_kayıt = açık_çözümleyici
        .parçalı_kaydı_mühürle(
            &açık_profil_isteği,
            &anatomi,
            &anatomi_parçası,
            &gösterim_parçası,
            açık_istek,
        )
        .map_err(|hata| format!("açık CJK kaydı mühürlenemedi: {hata:?}"))?;
    let açık_sonuç = açık_çözümleyici
        .çöz(&açık_kayıt)
        .map_err(|hata| format!("açık CJK etiketi çözülemedi: {hata:?}"))?;
    açık_çözümleyici
        .güncel_kayıtla_doğrula(&açık_kayıt, &açık_sonuç)
        .map_err(|hata| format!("açık CJK sonucu bayat: {hata:?}"))?;

    let kapalı_metin = kapalı_sonuç.görünür_metin().as_ref().to_owned();
    let açık_metin = açık_sonuç.görünür_metin().as_ref().to_owned();
    let kapalı_yapılandırma = düğme_yapılandırması(kapalı_metin.clone(), None);
    kapalı_yapılandırma
        .doğrula()
        .map_err(|hata| format!("kapalı BİL-040 tüketicisi geçersiz: {hata:?}"))?;
    let açık_yapılandırma = düğme_yapılandırması(açık_metin.clone(), Some(açık_profil_isteği));
    açık_yapılandırma
        .doğrula()
        .map_err(|hata| format!("açık BİL-040 tüketicisi geçersiz: {hata:?}"))?;

    let açık_eşleme = açık_sonuç.kaynak_grafemler();
    let eklenen_aralık_kaynak_tüketmiyor = (0..açık_eşleme.görünür_grafem_sayısı()).any(|sıra| {
        açık_eşleme
            .kaynak_aralığı(sıra)
            .is_some_and(|aralık| aralık.uzunluk() == 0)
    });
    let karakter_matrisi_korundu = kapalı_metin == KAYNAK
        && açık_metin.contains("A中1文,")
        && açık_metin.contains("e\u{301}")
        && açık_metin.contains("👨‍👩‍👧‍👦");

    Ok(CjkDüğmeSergiKanıtı {
        kaynak: KAYNAK.to_owned(),
        kapalı: kapalı_metin,
        açık: açık_metin,
        varsayılan_kapalı: CjkKısaEtiketAralığıProfili::varsayılan()
            == CjkKısaEtiketAralığıProfili::Kapalı
            && kapalı_kök.profil() == CjkKısaEtiketAralığıProfili::Kapalı,
        kapalıda_aralık_eklendi: kapalı_sonuç.aralık_eklendi_mi(),
        açıkta_aralık_eklendi: açık_sonuç.aralık_eklendi_mi(),
        eklenen_aralık_kaynak_tüketmiyor,
        kapalı_tüketici_none: kapalı_yapılandırma.cjk_kısa_etiket_profili.is_none(),
        açık_tüketici_bağlı: açık_yapılandırma.cjk_kısa_etiket_profili.is_some(),
        erişilebilir_ad_kaynağı_koruyor: kapalı_yapılandırma.tam_erişilebilir_ad()
            == Some(KAYNAK)
            && açık_yapılandırma.tam_erişilebilir_ad() == Some(KAYNAK),
        // Galerinin kopyalama girdisi görünür projeksiyon değil, aynı mühürlü
        // kaynak dilimidir. Bu alan görünür kanıtta o ayrımı fail-closed gösterir.
        kopya_kaynağı_koruyor: KAYNAK == kapalı_kopya_kaynağı && KAYNAK == açık_kopya_kaynağı,
        karakter_matrisi_korundu,
    })
}

fn düğme_yapılandırması(
    etiket: String,
    cjk_kısa_etiket_profili: Option<CjkKısaEtiketAralığıProfiliİsteği>,
) -> DüğmeMetniYapılandırması {
    DüğmeMetniYapılandırması {
        etiket: Some(GüvenliMetin::yeni(etiket, false, true)),
        erişilebilir_ad: Some(GüvenliMetin::yeni(KAYNAK, false, true)),
        açıklama: None,
        taşma: DüğmeMetniTaşması::TekSatırSığdır,
        hizalama: DüğmeMetniHizalaması::Orta,
        boşluk_politikası: EtiketBoşlukPolitikası::DışBoşluğuReddet,
        cjk_kısa_etiket_profili,
        yazı_yönü: ÇözülmüşYazıYönü::SoldanSağa,
    }
}

fn cjk_anatomisi(etiket: AnatomiParçasıKimliği) -> Result<BileşenGörselAnatomisi, String> {
    let yerleşim = GörünümYerleşimŞablonuKimliği::denetimli("ort017.cjk", "yerleşim")
        .map_err(|hata| format!("CJK yerleşim kimliği kurulamadı: {hata:?}"))?;
    let render = RenderStratejisiKimliği::denetimli("ort017.cjk", "render")
        .map_err(|hata| format!("CJK render kimliği kurulamadı: {hata:?}"))?;
    Ok(BileşenGörselAnatomisi {
        bileşen_türü: BileşenTürüKimliği::denetimli("ort017.cjk", "düğme")
            .map_err(|hata| format!("CJK bileşen kimliği kurulamadı: {hata:?}"))?,
        sürüm: AnatomiSürümü { ana: 1, alt: 0 },
        parçalar: Arc::from([AnatomiParçasıTanımı {
            kimlik: etiket,
            sınıf: AnatomiParçasıSınıfı::Metin,
            zorunlu: true,
            tekrarlı: false,
            davranış_hedefi: true,
            erişilebilirlik_kaynağı: true,
            dekoratif: false,
        }]),
        yuvalar: Arc::from([]),
        yerleşim_şablonları: Arc::from([yerleşim.clone()]),
        render_stratejileri: Arc::from([render.clone()]),
        yerleşik_fallback: yerleşim,
        güvenli_render: render,
    })
}

fn cjk_isteği(
    unicode_kökü: &Arc<UnicodeVeYerelMetinHizmetleri>,
    bağlam: ÖrnekKimliği,
    sürüm: u64,
    metin: &str,
    parça: GösterimParçasıKimliği,
) -> Result<(CjkEtiketAralığıİsteği, String), String> {
    let damga = unicode_kökü
        .metin_damgası_fabrikası()
        .damga(CanlıBağlamDamgası {
            bağlam,
            sürüm: BağlamSürümü(sürüm),
        });
    let tampon = unicode_kökü
        .motor()
        .kalıcı_utf8_tamponunu_ilkle(metin.to_owned(), damga);
    let snapshot = unicode_kökü
        .motor()
        .utf8_snapshotını_ilkle_doğrula(tampon)
        .map_err(|hata| format!("CJK UTF-8 snapshotı doğrulanamadı: {hata:?}"))?;
    let durum = unicode_kökü
        .motor()
        .artımlı_grafem_haritasını_ilkle(&snapshot, &damga, 0)
        .map_err(|hata| format!("CJK grafem haritası kurulamadı: {hata:?}"))?;
    let kaynak = durum
        .kaynak_görünümü()
        .paylaşılan_dilim(0..metin.len())
        .map_err(|hata| format!("CJK kaynak dilimi kurulamadı: {hata:?}"))?;
    let mut kopya_kaynağı = String::with_capacity(kaynak.utf8_bayt_uzunluğu());
    kaynak
        .utf8_parçalarını_ziyaret(&mut |baytlar| {
            kopya_kaynağı
                .push_str(std::str::from_utf8(baytlar).expect("mühürlü CJK kaynağı UTF-8 kalır"));
            Utf8ParçaAkışı::Devam
        })
        .map_err(|hata| format!("CJK kopya kaynağı okunamadı: {hata:?}"))?;
    let grafem = u32::try_from(durum.harita().grafem_sayısı())
        .map_err(|_| "CJK grafem sayısı u32 bütçesini aştı".to_owned())?;
    let utf8 =
        u32::try_from(metin.len()).map_err(|_| "CJK UTF-8 sayısı u32 bütçesini aştı".to_owned())?;
    let utf16 = u32::try_from(durum.harita().son_sinir().utf16_birim())
        .map_err(|_| "CJK UTF-16 sayısı u32 bütçesini aştı".to_owned())?;
    let mutlak = MutlakGösterimAralığı::denetimli(0, grafem, 0, utf8, 0, utf16)
        .map_err(|hata| format!("CJK mutlak aralığı kurulamadı: {hata:?}"))?;
    let gösterim_fabrikası = unicode_kökü.yetkili_gösterim_dilimi_fabrikası();
    let mut bidi = gösterim_fabrikası.paragraf_bidi_dizini_aç(
        TanımKimliği::denetimli(Arc::from("ort017.cjk"), Arc::from("paragraf"))
            .map_err(|hata| format!("CJK paragraf kimliği kurulamadı: {hata:?}"))?,
        0,
        u64::from(utf8),
    );
    bidi.ilk_parçalı_gösterim_parçasını_yaz(0, kaynak.clone())
        .map_err(|hata| format!("CJK BiDi kaynağı yazılamadı: {hata:?}"))?;
    let revizyon = bidi
        .ilk_revizyonu_mühürle()
        .map_err(|hata| format!("CJK BiDi revizyonu mühürlenemedi: {hata:?}"))?;
    let bidi_kanıtı = bidi
        .kanıt_üret(&revizyon, mutlak)
        .map_err(|hata| format!("CJK BiDi kanıtı üretilemedi: {hata:?}"))?;
    let mut yazıcı = gösterim_fabrikası.yazıcı_aç();
    yazıcı
        .parçalı_parça_yaz(
            parça,
            kaynak.clone(),
            mutlak,
            GösterimHassasiyetSınıfı::AçıkProjeksiyon,
            true,
        )
        .map_err(|hata| format!("CJK gösterim parçası yazılamadı: {hata:?}"))?;
    yazıcı
        .konum_eşlemesi_yaz(0, 0, 0, 0)
        .map_err(|hata| format!("CJK konum eşlemesi yazılamadı: {hata:?}"))?;
    yazıcı
        .paragraf_bidi_kanıtını_bağla(bidi_kanıtı)
        .map_err(|hata| format!("CJK BiDi kanıtı bağlanamadı: {hata:?}"))?;
    let istek = gösterim_fabrikası
        .doğrula_ve_mühürle_cjk(yazıcı, kaynak)
        .map_err(|hata| format!("CJK gösterim isteği mühürlenemedi: {hata:?}"))?;
    Ok((istek, kopya_kaynağı))
}

#[cfg(test)]
mod testler {
    use super::*;

    #[test]
    fn bil040_cjk_sergisi_gercek_saglayicidan_kapali_ve_acik_sonuc_alir() {
        let kanıt = kanıtı_üret().expect("gerçek ORT-017 CJK sağlayıcı zinciri");
        assert_eq!(kanıt.kaynak, KAYNAK);
        assert_eq!(kanıt.kapalı, KAYNAK);
        assert_eq!(kanıt.açık, "保 存 A中1文,e\u{301}👨‍👩‍👧‍👦");
        assert!(kanıt.varsayılan_kapalı);
        assert!(!kanıt.kapalıda_aralık_eklendi);
        assert!(kanıt.açıkta_aralık_eklendi);
        assert!(kanıt.eklenen_aralık_kaynak_tüketmiyor);
        assert!(kanıt.kapalı_tüketici_none);
        assert!(kanıt.açık_tüketici_bağlı);
        assert!(kanıt.erişilebilir_ad_kaynağı_koruyor);
        assert!(kanıt.kopya_kaynağı_koruyor);
        assert!(kanıt.karakter_matrisi_korundu);
    }
}

use gpui_bilesenleri_galeri::*;
use std::{collections::BTreeSet, sync::Arc};

fn akış() -> OnboardingAkışı {
    OnboardingAkışı {
        adımlar: [
            OnboardingAdımı {
                kimlik: OnboardingAdımıKimliği(Arc::from("arama")),
                gereken_özellik: None,
                asgari_ürün_sürümü: 1,
                hedef: Arc::from("toolbar.search"),
                güvenli_fallback: Arc::from("overview"),
            },
            OnboardingAdımı {
                kimlik: OnboardingAdımıKimliği(Arc::from("kanıt")),
                gereken_özellik: Some(Arc::from("kanıt-kipi")),
                asgari_ürün_sürümü: 2,
                hedef: Arc::from("evidence.panel"),
                güvenli_fallback: Arc::from("overview"),
            },
        ]
        .into(),
        etkin: 0,
        durum: OnboardingTamamlanmaDurumu::DevamEdiyor,
        ürün_sürümü: 2,
        özellikler: BTreeSet::from([Arc::from("kanıt-kipi")]),
    }
}

#[test]
fn urun_onboarding_durum_makinesi_tum_kararlari_tasir() {
    let mut a = akış();
    a.karar_ver(OnboardingKararı::Tamamla);
    assert_eq!(a.etkin, 1);
    a.karar_ver(OnboardingKararı::Ertele);
    assert_eq!(a.durum, OnboardingTamamlanmaDurumu::Ertelendi);
    a.karar_ver(OnboardingKararı::YenidenBaşlat);
    assert_eq!(
        (a.etkin, a.durum),
        (0, OnboardingTamamlanmaDurumu::DevamEdiyor)
    );
    a.karar_ver(OnboardingKararı::Atla);
    assert_eq!(a.durum, OnboardingTamamlanmaDurumu::Atlandı);
}

#[test]
fn urun_onboarding_klavye_ve_ekran_okuyucu_eylemleri_aciktir() {
    assert_eq!(
        onboarding_klavye_eylemi(true, false),
        Some(OnboardingErişilebilirEylemi::SonrakiAdımıTamamla)
    );
    assert_eq!(
        onboarding_klavye_eylemi(false, true),
        Some(OnboardingErişilebilirEylemi::AkışıAtla)
    );
}

#[test]
fn urun_onboarding_kayip_hedefte_guvenli_fallback_kullanir() {
    let a = akış();
    let çözüm = onboarding_hedefini_çöz(&a.adımlar[0], false);
    assert!(çözüm.fallback_kullanıldı);
    assert_eq!(çözüm.görünüm.as_ref(), "overview");
}

#[test]
fn urun_onboarding_urun_surumu_ve_ozellik_uygunlugunu_cozer() {
    let mut a = akış();
    assert_eq!(a.uygun_adımlar().len(), 2);
    a.ürün_sürümü = 1;
    assert_eq!(a.uygun_adımlar().len(), 1);
    a.ürün_sürümü = 2;
    a.özellikler.clear();
    assert_eq!(a.uygun_adımlar().len(), 1);
}

#[test]
fn urun_onboarding_ilerleme_deposu_sema_gocu_guvenlidir() {
    let eski = OnboardingİlerlemeKaydı {
        şema_sürümü: 1,
        etkin_adım: 9,
        durum: OnboardingTamamlanmaDurumu::DevamEdiyor,
        ürün_sürümü: 1,
    };
    let yeni = onboarding_kaydını_göçür(eski, 2, 2);
    assert_eq!((yeni.şema_sürümü, yeni.etkin_adım), (2, 1));
}

#[test]
fn urun_onboarding_ortak_sandiklara_sizmaz() {
    let kaynak = include_str!("../src/onboarding.rs");
    assert!(!kaynak.contains("gpui_bilesenleri_temel"));
    assert!(!kaynak.contains("gpui_bilesenleri_kabuk"));
}

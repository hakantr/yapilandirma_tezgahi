use std::{collections::BTreeSet, sync::Arc};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OnboardingAdımıKimliği(pub Arc<str>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OnboardingAdımı {
    pub kimlik: OnboardingAdımıKimliği,
    pub gereken_özellik: Option<Arc<str>>,
    pub asgari_ürün_sürümü: u32,
    pub hedef: Arc<str>,
    pub güvenli_fallback: Arc<str>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OnboardingKararı {
    Tamamla,
    Atla,
    Ertele,
    YenidenBaşlat,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OnboardingTamamlanmaDurumu {
    DevamEdiyor,
    Tamamlandı,
    Atlandı,
    Ertelendi,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OnboardingAkışı {
    pub adımlar: Arc<[OnboardingAdımı]>,
    pub etkin: usize,
    pub durum: OnboardingTamamlanmaDurumu,
    pub ürün_sürümü: u32,
    pub özellikler: BTreeSet<Arc<str>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OnboardingHatası {
    BoşAkış,
    GeçersizEtkinAdım,
}

impl OnboardingAkışı {
    pub fn doğrula(&self) -> Result<(), OnboardingHatası> {
        if self.adımlar.is_empty() {
            Err(OnboardingHatası::BoşAkış)
        } else if self.etkin >= self.adımlar.len() {
            Err(OnboardingHatası::GeçersizEtkinAdım)
        } else {
            Ok(())
        }
    }

    pub fn uygun_adımlar(&self) -> Vec<&OnboardingAdımı> {
        self.adımlar
            .iter()
            .filter(|adım| {
                adım.asgari_ürün_sürümü <= self.ürün_sürümü
                    && adım
                        .gereken_özellik
                        .as_ref()
                        .is_none_or(|özellik| self.özellikler.contains(özellik))
            })
            .collect()
    }

    pub fn karar_ver(&mut self, karar: OnboardingKararı) {
        match karar {
            OnboardingKararı::Tamamla => {
                let son = self.etkin + 1 >= self.adımlar.len();
                if son {
                    self.durum = OnboardingTamamlanmaDurumu::Tamamlandı;
                } else {
                    self.etkin += 1;
                }
            }
            OnboardingKararı::Atla => self.durum = OnboardingTamamlanmaDurumu::Atlandı,
            OnboardingKararı::Ertele => self.durum = OnboardingTamamlanmaDurumu::Ertelendi,
            OnboardingKararı::YenidenBaşlat => {
                self.etkin = 0;
                self.durum = OnboardingTamamlanmaDurumu::DevamEdiyor;
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OnboardingErişilebilirEylemi {
    SonrakiAdımıTamamla,
    AkışıAtla,
}

pub const fn onboarding_klavye_eylemi(
    enter: bool,
    escape: bool,
) -> Option<OnboardingErişilebilirEylemi> {
    if enter {
        Some(OnboardingErişilebilirEylemi::SonrakiAdımıTamamla)
    } else if escape {
        Some(OnboardingErişilebilirEylemi::AkışıAtla)
    } else {
        None
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OnboardingHedefÇözümü {
    pub görünüm: Arc<str>,
    pub fallback_kullanıldı: bool,
}

pub fn onboarding_hedefini_çöz(
    adım: &OnboardingAdımı,
    hedef_yaşıyor: bool,
) -> OnboardingHedefÇözümü {
    OnboardingHedefÇözümü {
        görünüm: if hedef_yaşıyor {
            Arc::clone(&adım.hedef)
        } else {
            Arc::clone(&adım.güvenli_fallback)
        },
        fallback_kullanıldı: !hedef_yaşıyor,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OnboardingİlerlemeKaydı {
    pub şema_sürümü: u16,
    pub etkin_adım: usize,
    pub durum: OnboardingTamamlanmaDurumu,
    pub ürün_sürümü: u32,
}

pub fn onboarding_kaydını_göçür(
    mut kayıt: OnboardingİlerlemeKaydı,
    güncel_şema: u16,
    adım_sayısı: usize,
) -> OnboardingİlerlemeKaydı {
    kayıt.şema_sürümü = güncel_şema;
    kayıt.etkin_adım = kayıt.etkin_adım.min(adım_sayısı.saturating_sub(1));
    kayıt
}

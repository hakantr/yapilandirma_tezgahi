//! Galerinin gerçek BİL-140/K08 plan deposu ve yaşayan GPUI görünümü.
//!
//! Uygulama hizmeti `App` kökünde bir kez yaşar; her pencere kendi kayıt
//! kökünü, sınırlı deposunu ve tek görünüm entitysini taşır. `%25`, `%75` ve
//! `Sonuç` ayrı adlarla aynı yayını taklit etmez: her geçişte üretim
//! ORT-017+BİL-140 çifti, depo yaşamı, sunum kaydı ve hazır plan birlikte
//! ilerler; mevcut opak görünüm tutamacı yerinde güncellenir.

use std::{num::NonZeroU128, sync::Arc};

use gpui::{BorrowAppContext, Global, Window};
use gpui_bilesenleri::{
    AşamaKimliği, DurumAileİçeriği, DurumBileşimAkıbeti, DurumBileşimGirdisi,
    DurumBileşimKayıtDamgası, DurumBileşimProfiliKimliği, DurumCacheProfilRolü, DurumGörünümSeçimi,
    DurumHazırPlanı, DurumMetinleri, DurumPlanAnahtarı, DurumPlanBileşimKöküExt,
    DurumPlanGörünümTutamacı, DurumPlanKayıtKökü, DurumPlanPencereHizmeti, DurumPlanPortu,
    DurumPlanSnapshotYetkisi, DurumPlanSunumYayınAdayı, DurumPlanUygulamaHizmeti,
    DurumPlanıKimliği, DurumSunumuAdayı, DurumSunumuDoğrulayıcısı, DurumSunumuKimliği, DurumYaşamı,
    KanonikDurumPlanDeposu, KanonikDurumSunumuDoğrulayıcısı, YaşayanİşOlgusu,
    durum_planı_görünümünü_kur, durum_planı_uygulamada_kur, İlerlemeAdKaynağı, İlerlemeDeğeri,
    İlerlemeDuyuruProfiliKimliği, İlerlemeKesri, İlerlemeSunumRolü, İlerlemeSunumu, İlerlemeYaşamı,
};
use gpui_bilesenleri_temel::{
    AnatomiSürümü, BağlamSürümü, BileşenKimliği, BileşimKökü, CanlıBağlamDamgası,
    ErişilebilirDüğümKimliği, ErişilebilirÖrüntüKimliği, GörünümProfiliKimliği,
    HareketKoreografisiRolü, PencereKimliği, SemantikÖnem, TanımKimliği, YerelleştirmeAnahtarı,
    ÇözülmüşKullanıcıİletisi, ÖrnekKimliğiFabrikası,
};

use crate::TezgahİletiÇözücüsü;

fn tanım(ad: &str) -> TanımKimliği {
    TanımKimliği::denetimli(Arc::from("bil140"), Arc::from(ad))
        .expect("galeri BİL-140 tanımı geçerlidir")
}

fn bileşim_damgası(nesil: u64) -> DurumBileşimKayıtDamgası {
    DurumBileşimKayıtDamgası::yeni(tanım("yerlesik-bilesim"), nesil)
}

fn bileşim_profili() -> DurumBileşimProfiliKimliği {
    DurumBileşimProfiliKimliği(tanım("yerlesik-bilesim-profili"))
}

fn ilerleme_bileşim_profili() -> DurumBileşimProfiliKimliği {
    DurumBileşimProfiliKimliği(tanım("yerlesik-ilerleme-profili"))
}

fn görünüm_profili() -> GörünümProfiliKimliği {
    GörünümProfiliKimliği(tanım("yerlesik-gorunum"))
}

struct GaleriDurumPlanUygulamaKökü {
    hizmet: DurumPlanUygulamaHizmeti,
}

impl Global for GaleriDurumPlanUygulamaKökü {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum GaleriDurumAşaması {
    Yüzde25,
    Yüzde75,
    Sonuç,
}

impl GaleriDurumAşaması {
    fn sonraki(self) -> Self {
        match self {
            Self::Yüzde25 => Self::Yüzde75,
            Self::Yüzde75 => Self::Sonuç,
            Self::Sonuç => Self::Yüzde25,
        }
    }

    fn profil(self) -> DurumBileşimProfiliKimliği {
        match self {
            Self::Yüzde25 | Self::Yüzde75 => ilerleme_bileşim_profili(),
            Self::Sonuç => bileşim_profili(),
        }
    }

    fn plan_adı(self) -> &'static str {
        match self {
            Self::Yüzde25 => "galeri-plan-yuzde-25",
            Self::Yüzde75 => "galeri-plan-yuzde-75",
            Self::Sonuç => "galeri-plan-sonuc",
        }
    }

    fn sunum_adı(self) -> &'static str {
        match self {
            Self::Yüzde25 => "galeri-ilerleme-25",
            Self::Yüzde75 => "galeri-ilerleme-75",
            Self::Sonuç => "galeri-sonuc",
        }
    }

    fn tanı(self) -> &'static str {
        match self {
            Self::Yüzde25 => "%25",
            Self::Yüzde75 => "%75",
            Self::Sonuç => "Sonuç",
        }
    }

    fn sergi_yüzdesi(self) -> u8 {
        match self {
            Self::Yüzde25 => 25,
            Self::Yüzde75 => 75,
            Self::Sonuç => 100,
        }
    }
}

struct Aşamaİletileri {
    başlık: Arc<ÇözülmüşKullanıcıİletisi>,
    açıklama: Arc<ÇözülmüşKullanıcıİletisi>,
    ilerleme_etiketi: Option<Arc<ÇözülmüşKullanıcıİletisi>>,
}

fn ileti(
    çözücü: &TezgahİletiÇözücüsü,
    anahtar: &'static str,
) -> Result<Arc<ÇözülmüşKullanıcıİletisi>, String> {
    let anahtar = YerelleştirmeAnahtarı::yeni(anahtar).map_err(|hata| format!("{hata:?}"))?;
    çözücü
        .çözülmüş_sonuç(&anahtar)
        .map_err(|hata| format!("{hata:?}"))
}

fn aşama_iletileri(
    aşama: GaleriDurumAşaması,
    çözücü: &TezgahİletiÇözücüsü,
) -> Result<Aşamaİletileri, String> {
    match aşama {
        GaleriDurumAşaması::Yüzde25 | GaleriDurumAşaması::Yüzde75 => {
            let etiket = if aşama == GaleriDurumAşaması::Yüzde25 {
                "galeri.bil140.ilerleme.yüzde25"
            } else {
                "galeri.bil140.ilerleme.yüzde75"
            };
            Ok(Aşamaİletileri {
                başlık: ileti(çözücü, "galeri.bil140.ilerleme.başlık")?,
                açıklama: ileti(çözücü, "galeri.bil140.ilerleme.açıklama")?,
                ilerleme_etiketi: Some(ileti(çözücü, etiket)?),
            })
        }
        GaleriDurumAşaması::Sonuç => Ok(Aşamaİletileri {
            başlık: ileti(çözücü, "galeri.bil140.sonuç.başlık")?,
            açıklama: ileti(çözücü, "galeri.bil140.sonuç.açıklama")?,
            ilerleme_etiketi: None,
        }),
    }
}

fn ilerleme_içeriği(
    bileşen: &BileşenKimliği,
    bağlam: CanlıBağlamDamgası,
    pencere: PencereKimliği,
    pay: u128,
    payda: u128,
    etiket: Arc<ÇözülmüşKullanıcıİletisi>,
) -> DurumAileİçeriği {
    DurumAileİçeriği::İlerleme(İlerlemeSunumu {
        yaşam: İlerlemeYaşamı::yeni(
            DurumYaşamı::yeni(bileşen.clone(), bağlam, None),
            AşamaKimliği(tanım("galeri-asama")),
            bağlam.sürüm,
        ),
        değer: İlerlemeDeğeri::Belirli(
            İlerlemeKesri::yeni(pay, NonZeroU128::new(payda).expect("pozitif payda"))
                .expect("galeri ilerleme kesri geçerlidir"),
        ),
        rol: İlerlemeSunumRolü::Çubuk,
        ad: İlerlemeAdKaynağı::AçıkAd(etiket),
        duyuru_profili: İlerlemeDuyuruProfiliKimliği(tanım("yerlesik-duyuru")),
        yaşayan_iş: YaşayanİşOlgusu {
            bağlam,
            iş: None,
            çalışıyor: true,
        },
        erişilebilir_düğüm: ErişilebilirDüğümKimliği {
            pencere,
            bileşen: bileşen.clone(),
            yerel_parça: tanım("galeri-ilerleme"),
        },
        erişilebilir_örüntü: ErişilebilirÖrüntüKimliği(tanım("progressbar")),
    })
}

fn aşama_adayı(
    bileşen: &BileşenKimliği,
    bağlam: CanlıBağlamDamgası,
    görünüm_sürümü: BağlamSürümü,
    erişilebilir_pencere: PencereKimliği,
    aşama: GaleriDurumAşaması,
    iletiler: &Aşamaİletileri,
) -> DurumSunumuAdayı {
    let içerik = match aşama {
        GaleriDurumAşaması::Yüzde25 => ilerleme_içeriği(
            bileşen,
            bağlam,
            erişilebilir_pencere,
            1,
            4,
            Arc::clone(
                iletiler
                    .ilerleme_etiketi
                    .as_ref()
                    .expect("ilerleme aşamasının etiketi vardır"),
            ),
        ),
        GaleriDurumAşaması::Yüzde75 => ilerleme_içeriği(
            bileşen,
            bağlam,
            erişilebilir_pencere,
            3,
            4,
            Arc::clone(
                iletiler
                    .ilerleme_etiketi
                    .as_ref()
                    .expect("ilerleme aşamasının etiketi vardır"),
            ),
        ),
        GaleriDurumAşaması::Sonuç => DurumAileİçeriği::Sonuç,
    };
    DurumSunumuAdayı {
        kimlik: DurumSunumuKimliği(tanım(aşama.sunum_adı())),
        yaşam: DurumYaşamı::yeni(bileşen.clone(), bağlam, None),
        metinler: DurumMetinleri {
            başlık: Some(Arc::clone(&iletiler.başlık)),
            açıklama: Some(Arc::clone(&iletiler.açıklama)),
            ilerleme_etiketi: iletiler.ilerleme_etiketi.clone(),
        },
        görünüm: DurumGörünümSeçimi {
            önem: if aşama == GaleriDurumAşaması::Sonuç {
                SemantikÖnem::Başarı
            } else {
                SemantikÖnem::Bilgi
            },
            simge: None,
            simge_snapshotı: None,
            profil: görünüm_profili(),
            profil_bağlamı: görünüm_sürümü,
            anatomi: AnatomiSürümü { ana: 1, alt: 0 },
        },
        eylemler: Arc::from([]),
        uygunluk_olguları: Arc::from([]),
        içerik,
    }
}

fn plan_anahtarı(
    bileşen: &BileşenKimliği,
    bağlam: CanlıBağlamDamgası,
    görünüm_sürümü: BağlamSürümü,
    bileşim_nesli: u64,
    aşama: GaleriDurumAşaması,
) -> DurumPlanAnahtarı {
    DurumPlanAnahtarı {
        plan: DurumPlanıKimliği(tanım(aşama.plan_adı())),
        bileşen: bileşen.clone(),
        bağlam,
        iş: None,
        bileşim_kaydı: bileşim_damgası(bileşim_nesli),
        bileşim_profili: aşama.profil(),
        simge_snapshotı: None,
        görünüm_profili: görünüm_profili(),
        görünüm_bağlamı: görünüm_sürümü,
        anatomi: AnatomiSürümü { ana: 1, alt: 0 },
        hareket_rolü: HareketKoreografisiRolü::DurumGeçişi,
    }
}

fn aşama_planını_hazırla(
    depo: &KanonikDurumPlanDeposu,
    yetki: &DurumPlanSnapshotYetkisi,
    bileşen: &BileşenKimliği,
    bağlam: CanlıBağlamDamgası,
    görünüm_sürümü: BağlamSürümü,
    bileşim_nesli: u64,
    erişilebilir_pencere: PencereKimliği,
    aşama: GaleriDurumAşaması,
    çözücü: &TezgahİletiÇözücüsü,
) -> Result<Arc<DurumHazırPlanı>, String> {
    let iletiler = aşama_iletileri(aşama, çözücü)?;
    let doğrulayıcı =
        KanonikDurumSunumuDoğrulayıcısı::yaşayandan(bağlam, None, görünüm_sürümü);
    let yüzey = Arc::new(
        doğrulayıcı
            .doğrula(aşama_adayı(
                bileşen,
                bağlam,
                görünüm_sürümü,
                erişilebilir_pencere,
                aşama,
                &iletiler,
            ))
            .map_err(|hata| format!("{hata:?}"))?,
    );
    let girdi = DurumBileşimGirdisi {
        profil: aşama.profil(),
        kayıt: bileşim_damgası(bileşim_nesli),
        yüzeyler: Arc::from([yüzey]),
        yaşayan_geçerli_içerik_var: true,
    };
    let mut hazırlayıcı = depo
        .sunum_hazırlayıcısı(yetki)
        .map_err(|hata| format!("{hata:?}"))?;
    let bileşim = match hazırlayıcı.bileşimi_çöz(&girdi) {
        DurumBileşimAkıbeti::Hazır(plan) => Arc::new(plan),
        DurumBileşimAkıbeti::BileşimBelirsiz => return Err("BileşimBelirsiz".to_owned()),
        DurumBileşimAkıbeti::KayıtEskidi => return Err("KayıtEskidi".to_owned()),
        DurumBileşimAkıbeti::SnapshotEskidi => return Err("SnapshotEskidi".to_owned()),
    };
    let kayıt = hazırlayıcı
        .sunumu_doğrula(aşama_adayı(
            bileşen,
            bağlam,
            görünüm_sürümü,
            erişilebilir_pencere,
            aşama,
            &iletiler,
        ))
        .map_err(|hata| format!("{hata:?}"))?;
    drop(hazırlayıcı);
    depo.sunum_kaydını_yayımla(
        yetki,
        DurumPlanSunumYayınAdayı::denetimli(bileşim, vec![kayıt])
            .map_err(|hata| format!("{hata:?}"))?,
    )
    .map_err(|hata| format!("{hata:?}"))?;
    let profil = depo.cache_profili();
    depo.soğuk_hazırla(
        plan_anahtarı(bileşen, bağlam, görünüm_sürümü, bileşim_nesli, aşama),
        &profil,
        yetki,
    )
    .map_err(|hata| format!("{hata:?}"))
}

/// Bir galeri penceresinin K08 sahiplik zinciri.
pub(crate) struct GaleriDurumPlanKökü {
    pencere_hizmeti: DurumPlanPencereHizmeti,
    bileşim_kökü: BileşimKökü,
    kayıt_kökü: DurumPlanKayıtKökü,
    depo: Arc<KanonikDurumPlanDeposu>,
    plan: Arc<DurumHazırPlanı>,
    görünüm: DurumPlanGörünümTutamacı,
    bileşen: BileşenKimliği,
    canlı_bağlam: CanlıBağlamDamgası,
    bileşim_nesli: u64,
    erişilebilir_pencere: PencereKimliği,
    aşama: GaleriDurumAşaması,
}

impl GaleriDurumPlanKökü {
    pub(crate) fn kur(
        fabrika: &ÖrnekKimliğiFabrikası,
        çözücü: &TezgahİletiÇözücüsü,
        pencere: &mut Window,
        bağlam: &mut gpui::App,
    ) -> Result<Self, String> {
        if !bağlam.has_global::<GaleriDurumPlanUygulamaKökü>() {
            let hizmet = durum_planı_uygulamada_kur(bağlam).map_err(|hata| format!("{hata:?}"))?;
            bağlam.set_global(GaleriDurumPlanUygulamaKökü { hizmet });
        }
        let pencere_hizmeti = bağlam
            .update_global::<GaleriDurumPlanUygulamaKökü, _>(|kök, bağlam| {
                kök.hizmet.pencereyi_kur(pencere, bağlam)
            })
            .map_err(|hata| format!("{hata:?}"))?;
        let mut bileşim_kökü =
            BileşimKökü::edin(bağlam).ok_or_else(|| "BileşimKöküEksik".to_owned())?;
        let (kayıt_kökü, kayıt) = bileşim_kökü
            .durum_plan_kökünü_kur(pencere, bağlam)
            .map_err(|hata| format!("{hata:?}"))?;
        let bileşen = BileşenKimliği {
            tanım: tanım("galeri-durum-plani"),
            örnek: fabrika.sonraki().map_err(|hata| format!("{hata:?}"))?,
        };
        let canlı_bağlam = CanlıBağlamDamgası {
            bağlam: fabrika.sonraki().map_err(|hata| format!("{hata:?}"))?,
            sürüm: BağlamSürümü(1),
        };
        let erişilebilir_pencere =
            PencereKimliği(fabrika.sonraki().map_err(|hata| format!("{hata:?}"))?);
        let depo = Arc::new(
            KanonikDurumPlanDeposu::kayıtlı_profille_yaşayandan(
                kayıt,
                bileşen.clone(),
                canlı_bağlam,
                None,
                DurumCacheProfilRolü::YaşayanDurumPlanı,
            )
            .map_err(|hata| format!("{hata:?}"))?,
        );
        let aşama = GaleriDurumAşaması::Yüzde25;
        let yetki = depo.yaşayan_snapshot_yetkisi();
        let plan = aşama_planını_hazırla(
            &depo,
            &yetki,
            &bileşen,
            canlı_bağlam,
            canlı_bağlam.sürüm,
            1,
            erişilebilir_pencere,
            aşama,
            çözücü,
        )?;
        let başvuru = depo
            .gösterim_başvurusunu_kur(Arc::clone(&plan), yetki)
            .map_err(|hata| format!("{hata:?}"))?;
        let görünüm =
            durum_planı_görünümünü_kur(başvuru, bağlam).map_err(|hata| format!("{hata:?}"))?;
        Ok(Self {
            pencere_hizmeti,
            bileşim_kökü,
            kayıt_kökü,
            depo,
            plan,
            görünüm,
            bileşen,
            canlı_bağlam,
            bileşim_nesli: 1,
            erişilebilir_pencere,
            aşama,
        })
    }

    pub(crate) fn ilerlet(
        &mut self,
        çözücü: &TezgahİletiÇözücüsü,
        pencere: &mut Window,
        bağlam: &mut gpui::App,
    ) -> Result<(), String> {
        let aşama = self.aşama.sonraki();
        let yeni_sürüm = self
            .canlı_bağlam
            .sürüm
            .0
            .checked_add(1)
            .map(BağlamSürümü)
            .ok_or_else(|| "CanlıBağlamSürümüTükendi".to_owned())?;
        let bileşim_nesli = self
            .bileşim_nesli
            .checked_add(1)
            .ok_or_else(|| "BileşimNesliTükendi".to_owned())?;
        let canlı_bağlam = CanlıBağlamDamgası {
            bağlam: self.canlı_bağlam.bağlam,
            sürüm: yeni_sürüm,
        };
        let beklenen = self.depo.yaşayan_snapshot_yetkisi();
        self.pencere_hizmeti
            .kanonik_yayını_tazele(pencere)
            .map_err(|hata| format!("{hata:?}"))?;
        let kayıt = self
            .bileşim_kökü
            .durum_plan_yayınını_hazırla(&self.kayıt_kökü, &beklenen, pencere, bağlam)
            .map_err(|hata| format!("{hata:?}"))?;
        let yetki = self
            .depo
            .yaşayanı_ilerlet(&beklenen, canlı_bağlam, None, kayıt)
            .map_err(|hata| format!("{hata:?}"))?;
        let plan = aşama_planını_hazırla(
            &self.depo,
            &yetki,
            &self.bileşen,
            canlı_bağlam,
            yeni_sürüm,
            bileşim_nesli,
            self.erişilebilir_pencere,
            aşama,
            çözücü,
        )?;
        let başvuru = self
            .depo
            .gösterim_başvurusunu_kur(Arc::clone(&plan), yetki)
            .map_err(|hata| format!("{hata:?}"))?;
        self.görünüm
            .güncelle(başvuru, bağlam)
            .map_err(|hata| format!("{hata:?}"))?;
        self.plan = plan;
        self.canlı_bağlam = canlı_bağlam;
        self.bileşim_nesli = bileşim_nesli;
        self.aşama = aşama;
        Ok(())
    }

    pub(crate) fn görünüm(&self) -> DurumPlanGörünümTutamacı {
        self.görünüm.clone()
    }

    pub(crate) fn plan_nesli(&self) -> u64 {
        self.plan.damga().nesil()
    }

    pub(crate) fn yaşam_nesli(&self) -> u64 {
        self.depo
            .yaşayan_snapshot_yetkisi()
            .yaşam()
            .yaşam_nesli()
            .get()
    }

    pub(crate) fn aşama(&self) -> &'static str {
        self.aşama.tanı()
    }

    pub(crate) fn sergi_yüzdesi(&self) -> u8 {
        self.aşama.sergi_yüzdesi()
    }
}

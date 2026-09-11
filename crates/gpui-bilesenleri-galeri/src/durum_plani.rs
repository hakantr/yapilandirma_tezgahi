//! Galerinin gerçek BİL-140/K08 plan deposu ve yaşayan GPUI görünümü.
//!
//! Uygulama hizmeti `App` kökünde bir kez yaşar; her pencere kendi kayıt
//! kökünü, sınırlı deposunu ve tek görünüm entitysini taşır. Galeri yeni bir
//! canlı-yayın sağlayıcısı uydurmaz: görünür geçiş aynı yaşayan yetki içinde
//! önceden hazırlanmış iki plan arasında yapılır.

use std::{num::NonZeroU128, sync::Arc};

use gpui::{BorrowAppContext, Global, Window};
use gpui_bilesenleri::{
    AşamaKimliği, DurumAileİçeriği, DurumBileşimAkıbeti, DurumBileşimGirdisi,
    DurumBileşimKayıtDamgası, DurumBileşimProfiliKimliği, DurumCacheProfilRolü, DurumGörünümSeçimi,
    DurumMetinleri, DurumPlanAnahtarı, DurumPlanBileşimKöküExt, DurumPlanGörünümTutamacı,
    DurumPlanPortu, DurumPlanSunumYayınAdayı, DurumPlanUygulamaHizmeti, DurumPlanıKimliği,
    DurumSunumuAdayı, DurumSunumuDoğrulayıcısı, DurumSunumuKimliği, DurumYaşamı,
    KanonikDurumPlanDeposu, KanonikDurumSunumuDoğrulayıcısı, YaşayanİşOlgusu,
    durum_planı_görünümünü_kur, durum_planı_uygulamada_kur, İlerlemeAdKaynağı, İlerlemeDeğeri,
    İlerlemeDuyuruProfiliKimliği, İlerlemeKesri, İlerlemeSunumRolü, İlerlemeSunumu, İlerlemeYaşamı,
};
use gpui_bilesenleri_temel::{
    AnatomiSürümü, BağlamSürümü, BileşenKimliği, BileşimKökü, CanlıBağlamDamgası,
    ErişilebilirDüğümKimliği, ErişilebilirÖrüntüKimliği, GörünümProfiliKimliği,
    HareketKoreografisiRolü, PencereKimliği, SemantikÖnem, TanımKimliği, ÖrnekKimliğiFabrikası,
};

fn tanım(ad: &str) -> TanımKimliği {
    TanımKimliği::denetimli(Arc::from("bil140"), Arc::from(ad))
        .expect("galeri BİL-140 tanımı geçerlidir")
}

fn bileşim_damgası() -> DurumBileşimKayıtDamgası {
    DurumBileşimKayıtDamgası::yeni(tanım("yerlesik-bilesim"), 1)
}

fn bileşim_profili() -> DurumBileşimProfiliKimliği {
    DurumBileşimProfiliKimliği(tanım("yerlesik-bilesim-profili"))
}

fn görünüm_profili() -> GörünümProfiliKimliği {
    GörünümProfiliKimliği(tanım("yerlesik-gorunum"))
}

struct GaleriDurumPlanUygulamaKökü {
    hizmet: DurumPlanUygulamaHizmeti,
}

impl Global for GaleriDurumPlanUygulamaKökü {}

fn aday(
    bileşen: &BileşenKimliği,
    bağlam: CanlıBağlamDamgası,
    ad: &str,
    içerik: DurumAileİçeriği,
) -> DurumSunumuAdayı {
    DurumSunumuAdayı {
        kimlik: DurumSunumuKimliği(tanım(ad)),
        yaşam: DurumYaşamı::yeni(bileşen.clone(), bağlam, None),
        metinler: DurumMetinleri {
            başlık: None,
            açıklama: None,
            ilerleme_etiketi: None,
        },
        görünüm: DurumGörünümSeçimi {
            önem: if matches!(içerik, DurumAileİçeriği::Sonuç) {
                SemantikÖnem::Başarı
            } else {
                SemantikÖnem::Bilgi
            },
            simge: None,
            simge_snapshotı: None,
            profil: görünüm_profili(),
            profil_bağlamı: BağlamSürümü(1),
            anatomi: AnatomiSürümü { ana: 1, alt: 0 },
        },
        eylemler: Arc::from([]),
        uygunluk_olguları: Arc::from([]),
        içerik,
    }
}

fn ilerleme_içeriği(
    bileşen: &BileşenKimliği,
    bağlam: CanlıBağlamDamgası,
    pencere: PencereKimliği,
) -> DurumAileİçeriği {
    DurumAileİçeriği::İlerleme(İlerlemeSunumu {
        yaşam: İlerlemeYaşamı::yeni(
            DurumYaşamı::yeni(bileşen.clone(), bağlam, None),
            AşamaKimliği(tanım("galeri-asama")),
            BağlamSürümü(1),
        ),
        değer: İlerlemeDeğeri::Belirli(
            İlerlemeKesri::yeni(1, NonZeroU128::new(2).expect("pozitif payda"))
                .expect("galeri ilerleme kesri geçerlidir"),
        ),
        rol: İlerlemeSunumRolü::Çubuk,
        ad: İlerlemeAdKaynağı::KayıtlıGüvenliVarsayılan(tanım("yerlesik-ilerleme-adi")),
        duyuru_profili: İlerlemeDuyuruProfiliKimliği(tanım("yerlesik-duyuru")),
        yaşayan_iş: YaşayanİşOlgusu {
            bağlam,
            iş: None,
            çalışıyor: false,
        },
        erişilebilir_düğüm: ErişilebilirDüğümKimliği {
            pencere,
            bileşen: bileşen.clone(),
            yerel_parça: tanım("galeri-ilerleme"),
        },
        erişilebilir_örüntü: ErişilebilirÖrüntüKimliği(tanım("progressbar")),
    })
}

fn plan_anahtarı(
    bileşen: &BileşenKimliği,
    bağlam: CanlıBağlamDamgası,
    ad: &str,
) -> DurumPlanAnahtarı {
    DurumPlanAnahtarı {
        plan: DurumPlanıKimliği(tanım(ad)),
        bileşen: bileşen.clone(),
        bağlam,
        iş: None,
        bileşim_kaydı: bileşim_damgası(),
        bileşim_profili: bileşim_profili(),
        simge_snapshotı: None,
        görünüm_profili: görünüm_profili(),
        görünüm_bağlamı: BağlamSürümü(1),
        anatomi: AnatomiSürümü { ana: 1, alt: 0 },
        hareket_rolü: HareketKoreografisiRolü::DurumGeçişi,
    }
}

/// Bir galeri penceresinin K08 sahiplik zinciri.
pub(crate) struct GaleriDurumPlanKökü {
    _pencere_hizmeti: gpui_bilesenleri::DurumPlanPencereHizmeti,
    _kayıt_kökü: gpui_bilesenleri::DurumPlanKayıtKökü,
    depo: Arc<KanonikDurumPlanDeposu>,
    planlar: [Arc<gpui_bilesenleri::DurumHazırPlanı>; 2],
    görünüm: DurumPlanGörünümTutamacı,
    etkin: usize,
}

impl GaleriDurumPlanKökü {
    pub(crate) fn kur(
        fabrika: &ÖrnekKimliğiFabrikası,
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
        let yetki = depo.yaşayan_snapshot_yetkisi();
        let doğrulayıcı =
            KanonikDurumSunumuDoğrulayıcısı::yaşayandan(canlı_bağlam, None, BağlamSürümü(1));
        let yüzeyler: Arc<[_]> = [
            aday(
                &bileşen,
                canlı_bağlam,
                "galeri-sonuc",
                DurumAileİçeriği::Sonuç,
            ),
            aday(
                &bileşen,
                canlı_bağlam,
                "galeri-ilerleme",
                ilerleme_içeriği(&bileşen, canlı_bağlam, erişilebilir_pencere),
            ),
        ]
        .into_iter()
        .map(|aday| {
            doğrulayıcı
                .doğrula(aday)
                .map(Arc::new)
                .map_err(|hata| format!("{hata:?}"))
        })
        .collect::<Result<Vec<_>, _>>()?
        .into();
        let girdi = DurumBileşimGirdisi {
            profil: bileşim_profili(),
            kayıt: bileşim_damgası(),
            yüzeyler,
            yaşayan_geçerli_içerik_var: true,
        };
        let mut hazırlayıcı = depo
            .sunum_hazırlayıcısı(&yetki)
            .map_err(|hata| format!("{hata:?}"))?;
        let bileşim = match hazırlayıcı.bileşimi_çöz(&girdi) {
            DurumBileşimAkıbeti::Hazır(plan) => Arc::new(plan),
            DurumBileşimAkıbeti::BileşimBelirsiz => {
                return Err("BileşimBelirsiz".to_owned());
            }
            DurumBileşimAkıbeti::KayıtEskidi => return Err("KayıtEskidi".to_owned()),
            DurumBileşimAkıbeti::SnapshotEskidi => return Err("SnapshotEskidi".to_owned()),
        };
        let kayıtlar = [
            aday(
                &bileşen,
                canlı_bağlam,
                "galeri-sonuc",
                DurumAileİçeriği::Sonuç,
            ),
            aday(
                &bileşen,
                canlı_bağlam,
                "galeri-ilerleme",
                ilerleme_içeriği(&bileşen, canlı_bağlam, erişilebilir_pencere),
            ),
        ]
        .into_iter()
        .map(|aday| {
            hazırlayıcı
                .sunumu_doğrula(aday)
                .map_err(|hata| format!("{hata:?}"))
        })
        .collect::<Result<Vec<_>, _>>()?;
        drop(hazırlayıcı);
        depo.sunum_kaydını_yayımla(
            &yetki,
            DurumPlanSunumYayınAdayı::denetimli(bileşim, kayıtlar)
                .map_err(|hata| format!("{hata:?}"))?,
        )
        .map_err(|hata| format!("{hata:?}"))?;
        let profil = depo.cache_profili();
        let plan_a = depo
            .soğuk_hazırla(
                plan_anahtarı(&bileşen, canlı_bağlam, "galeri-plan-a"),
                &profil,
                &yetki,
            )
            .map_err(|hata| format!("{hata:?}"))?;
        let plan_b = depo
            .soğuk_hazırla(
                plan_anahtarı(&bileşen, canlı_bağlam, "galeri-plan-b"),
                &profil,
                &yetki,
            )
            .map_err(|hata| format!("{hata:?}"))?;
        let başvuru = depo
            .gösterim_başvurusunu_kur(Arc::clone(&plan_a), yetki)
            .map_err(|hata| format!("{hata:?}"))?;
        let görünüm =
            durum_planı_görünümünü_kur(başvuru, bağlam).map_err(|hata| format!("{hata:?}"))?;
        Ok(Self {
            _pencere_hizmeti: pencere_hizmeti,
            _kayıt_kökü: kayıt_kökü,
            depo,
            planlar: [plan_a, plan_b],
            görünüm,
            etkin: 0,
        })
    }

    pub(crate) fn ilerlet(&mut self, bağlam: &mut gpui::App) -> Result<(), String> {
        let sonraki = (self.etkin + 1) % self.planlar.len();
        let başvuru = self
            .depo
            .gösterim_başvurusunu_kur(
                Arc::clone(&self.planlar[sonraki]),
                self.depo.yaşayan_snapshot_yetkisi(),
            )
            .map_err(|hata| format!("{hata:?}"))?;
        self.görünüm
            .güncelle(başvuru, bağlam)
            .map_err(|hata| format!("{hata:?}"))?;
        self.etkin = sonraki;
        Ok(())
    }

    pub(crate) fn görünüm(&self) -> DurumPlanGörünümTutamacı {
        self.görünüm.clone()
    }

    pub(crate) fn plan_nesli(&self) -> u64 {
        self.planlar[self.etkin].damga().nesil()
    }
}

//! `BİL-120` galeri kartının gerçek K10 zinciri.
//!
//! Bu modül görünürlük sahibini odak sağlayıcısı gibi saymaz. Reveal portu
//! yalnız sanallaştırılmış hedefi aynı çevrimde çizim ağacına alır; odak
//! kararı kanonik `ORT-005` portundan gelir ve yalnız providerın commit ettiği
//! yaşayan `BİL-010` `FocusHandle`ına uygulanır. `ORT-009` snapshotı da aynı
//! App-kökü hizmetinde yayımlanır.

use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    rc::Rc,
    sync::Arc,
};

use gpui::{AnyElement, App, Context, Entity, Global, IntoElement, Window, div, prelude::*, rgb};
use gpui_bilesenleri::{
    AtamaNiyeti, AtamaSonucu, AçıkGirişDeğeriGirdisi, AçıkGirişDeğeriGörünümü,
    FormAlanıAnlıkGörüntüsü, FormAlanıDeğeri, FormAlanıDoğrulamaAkıbeti, FormAlanıKimliği,
    FormAlanıPortu, FormAlanıTanımı, FormDenetleyicisi, FormDoğrulamaBağlamı, FormGönderimAkıbeti,
    FormGönderimPortu, FormGönderimİsteği, FormHatası, FormKimliği, FormKuruluşu, FormRevealHedefi,
    FormRevealPortu, FormRevealSonucu, FormSorunu, FormTanımı, FormTipografiEşlemesi, GirişDurumu,
    GirişDurumuGözlemi, GirişKutusu, GirişKutusuFormOdakHedefiKuruluşu, GirişYapılandırması,
    GizlenenAlanDeğeriPolitikası, SürümlüAçıkGirişDeğeri, form_denetleyicisini_kur,
};
use gpui_bilesenleri_temel::{
    BağlamSürümü, BileşimKökü, CanlıBağlamDamgası, ErişilebilirEtkileşimKipi,
    ErişilebilirOdakSunumu, ErişilebilirSnapshotKuruluşGirdisi, ErişilebilirUyumProfili,
    ErişilebilirlikHizmetKökü, ErişilebilirlikProfili, HareketTercihi, OdakBölgesiBaşvurusu,
    OdakBölgesiKaydıTanımı, OdakGezinimProfiliKimliği, OdakGörünürlükSonucu, OdakHedefiBaşvurusu,
    OdakHedefiKaydıTanımı, OdakKapsamıBaşvurusu, OdakKapsamıKaydıTanımı, OdakSağlayıcısıKökü,
    OdakUygunlukEksenleri, OdakÖnKapıKararı, PencereKimliği, SüreçGörevTeslimi, SüreçİşTutamacı,
    TanımKimliği, TipografiKapsamıKimliği, TipografiYerleşimiKimliği, YerelleştirmeAnahtarı,
    ÖrnekKimliğiFabrikası, İletiİsteği, İşHatası,
};

use super::{
    GaleriUygulaması, MetinHizmetleriKökü, açık_giriş_kurucusu, galeri_bileşen_kimliği,
    galeri_teması, hazır_ileti, palet,
};

/// `ORT-009` ve onun tek `ORT-005` provider kökü App başına bir kez yaşar;
/// her galeri penceresi bu kökten farklı bir pencere portu alır.
struct GaleriK10UygulamaKökü {
    erişilebilirlik: ErişilebilirlikHizmetKökü,
    odak: OdakSağlayıcısıKökü,
}

impl Global for GaleriK10UygulamaKökü {}

fn tanım(yerel_ad: &'static str) -> TanımKimliği {
    TanımKimliği::denetimli(Arc::from("galeri.k10"), Arc::from(yerel_ad))
        .expect("galerinin sabit K10 tanımı geçerlidir")
}

fn uygunluk() -> OdakUygunlukEksenleri {
    OdakUygunlukEksenleri {
        yaşayan: true,
        görünür: true,
        etkileşim_etkin: true,
        salt_okunur: false,
        olağan_tab_durağı: true,
        yönlü_gezinim_hedefi: true,
        programlı_hedef: true,
        erişilebilirlik_hedefi: true,
        eylem_yeteneği: true,
    }
}

fn uygulama_kökünü_kur(bağlam: &mut App) -> Result<(), String> {
    if bağlam.has_global::<GaleriK10UygulamaKökü>() {
        return Ok(());
    }
    let mut bileşim = BileşimKökü::edin(bağlam).ok_or_else(|| "BileşimKöküEksik".to_owned())?;
    let yetki = bileşim
        .erişilebilirlik_hizmet_yetkisi()
        .map_err(|hata| format!("ORT-009 yetkisi alınamadı: {hata:?}"))?;
    let mut erişilebilirlik = ErişilebilirlikHizmetKökü::bileşim_kökünde_kur(yetki);
    let odak = OdakSağlayıcısıKökü::erişilebilirlik_hizmetinde_kur(&mut erişilebilirlik)
        .map_err(|hata| format!("ORT-005 kökü kurulamadı: {hata:?}"))?;
    bağlam.set_global(GaleriK10UygulamaKökü {
        erişilebilirlik,
        odak,
    });
    Ok(())
}

/// Yalnız bu gerçek metin alanına bağlı form adapterı. Değer ve sürüm,
/// ikinci bir metin kopyası üretmeden yaşayan `BİL-010` snapshotından okunur.
struct GaleriK10AlanPortu {
    kimlik: FormAlanıKimliği,
    kutu: Entity<GirişKutusu>,
}

impl FormAlanıPortu for GaleriK10AlanPortu {
    fn anlık_görüntü(
        &self,
        bağlam: &App,
    ) -> Result<FormAlanıAnlıkGörüntüsü, FormHatası> {
        let alan = self.kutu.read(bağlam);
        let GirişDurumuGözlemi::Açık(durum) = alan.durum_gözlemi() else {
            unreachable!("K10 galeri alanı yalnız açık BİL-010 olarak kurulur")
        };
        let GirişDurumu::Açık(çekirdek) = durum else {
            unreachable!("açık BİL-010 gözlemi açık çekirdek taşır")
        };
        let değer = match çekirdek
            .kabul_edilmiş_değer()
            .map(SürümlüAçıkGirişDeğeri::değer)
        {
            None | Some(AçıkGirişDeğeriGörünümü::Null) => FormAlanıDeğeri::Boş,
            Some(AçıkGirişDeğeriGörünümü::Metin(metin)) => {
                FormAlanıDeğeri::Metin(metin.clone())
            }
            Some(_) => unreachable!("K10 galeri alanı tek satırlı metindir"),
        };
        Ok(FormAlanıAnlıkGörüntüsü {
            kimlik: self.kimlik.clone(),
            değer,
            görünür: true,
            etkin: true,
            gönderime_katılır: true,
            düzenleme_kirli: çekirdek.düzenleme_kirli(),
            kayıt_kirli: çekirdek.kayıt_kirli(),
            dokunuldu: çekirdek.dokunuldu(),
            sürüm: BağlamSürümü(çekirdek.değer_sürümü()),
        })
    }

    fn doğrula(
        &mut self,
        _: &FormDoğrulamaBağlamı,
        _: &mut Window,
        _: &mut App,
    ) -> Result<FormAlanıDoğrulamaAkıbeti, FormHatası> {
        Ok(FormAlanıDoğrulamaAkıbeti::Geçersiz(Arc::from([
            FormSorunu {
                alan: Some(self.kimlik.clone()),
                kod: tanım("zorunlu-alan"),
                ileti: İletiİsteği {
                    anahtar: YerelleştirmeAnahtarı::yeni("form.sorun")
                        .expect("kanonik form sorun anahtarı geçerlidir"),
                    argümanlar: Arc::from([]),
                },
            },
        ])))
    }

    fn sıfırlama_niyeti(
        &mut self,
        beklenen_sürüm: BağlamSürümü,
        _: &mut Window,
        bağlam: &mut App,
    ) -> Result<(), FormHatası> {
        let (değer_sürümü, yapılandırma_sürümü) = {
            let alan = self.kutu.read(bağlam);
            let GirişDurumuGözlemi::Açık(GirişDurumu::Açık(çekirdek)) = alan.durum_gözlemi()
            else {
                unreachable!("K10 galeri alanı yalnız açık BİL-010 olarak kurulur")
            };
            (
                çekirdek.değer_sürümü(),
                alan.yapılandırma().yapılandırma_sürümü(),
            )
        };
        if beklenen_sürüm != BağlamSürümü(değer_sürümü) {
            return Err(FormHatası::EskiAlanSürümü);
        }
        let sonuç = self.kutu.update(bağlam, |alan, bağlam| {
            alan.değeri_ata(
                AçıkGirişDeğeriGirdisi::Null,
                AtamaNiyeti::KabulEdilmişDeğeriDeğiştir,
                değer_sürümü,
                yapılandırma_sürümü,
                bağlam,
            )
        });
        match sonuç {
            AtamaSonucu::Uygulandı { .. } | AtamaSonucu::DeğişiklikYok => Ok(()),
            _ => Err(FormHatası::EskiAlanSürümü),
        }
    }
}

/// Form bu kartta yalnız geçersiz-hedef reveal'ını gösterir. Sürekli
/// geçersiz alan nedeniyle gönderim portuna ulaşılması bir iç değişmezlik
/// ihlalidir; sahte bir iş veya hazır başarı üretilmez.
struct GönderimsizK10Portu;

impl FormGönderimPortu for GönderimsizK10Portu {
    fn gönder(
        &mut self,
        _: FormGönderimİsteği,
        _: Box<dyn FnOnce(SüreçGörevTeslimi<FormGönderimAkıbeti>, &mut App) + Send + 'static>,
        _: &mut App,
    ) -> Result<SüreçİşTutamacı, İşHatası> {
        unreachable!("geçersiz K10 galeri formu gönderim admissionına ulaşamaz")
    }
}

/// Scroll/layout sahibidir; odak kararı veya `FocusHandle` yazımı içermez.
struct GaleriK10RevealPortu {
    hedef_açık: Rc<Cell<bool>>,
    reveal_sayısı: Rc<Cell<usize>>,
}

impl FormRevealPortu for GaleriK10RevealPortu {
    fn görünür_kıl(
        &mut self,
        _: &FormRevealHedefi,
        pencere: &mut Window,
        _: &mut App,
    ) -> Result<OdakGörünürlükSonucu, FormHatası> {
        if !self.hedef_açık.replace(true) {
            self.reveal_sayısı
                .set(self.reveal_sayısı.get().saturating_add(1));
        }
        pencere.refresh();
        Ok(OdakGörünürlükSonucu::AynıÇevrimdeAçığaÇıkarıldı)
    }
}

/// Bir galeri penceresinin gerçek BİL-120/K10 form sahipliği.
pub(crate) struct GaleriK10FormKökü {
    form: Entity<FormDenetleyicisi>,
    fallback: Entity<GirişKutusu>,
    hedef: Entity<GirişKutusu>,
    hedef_başvurusu: OdakHedefiBaşvurusu,
    hedef_açık: Rc<Cell<bool>>,
    reveal_sayısı: Rc<Cell<usize>>,
    son_sonuç: Rc<RefCell<Option<String>>>,
}

impl GaleriK10FormKökü {
    pub(crate) fn kur(
        hizmetler: &MetinHizmetleriKökü,
        kimlik_fabrikası: &ÖrnekKimliğiFabrikası,
        pencere: &mut Window,
        bağlam: &mut Context<GaleriUygulaması>,
    ) -> Result<Self, String> {
        let yerel = hizmetler.yerel_kök();
        let kutu = |yerel_ad: &'static str,
                    erişilebilir_ad: &'static str,
                    pencere: &mut Window,
                    bağlam: &mut Context<GaleriUygulaması>|
         -> Result<Entity<GirişKutusu>, String> {
            let mut yapılandırma = GirişYapılandırması::tek_satırlı_metin();
            yapılandırma.erişilebilir_ad = Some(hazır_ileti(erişilebilir_ad));
            yapılandırma.yer_tutucu = Some(hazır_ileti("Bu alan K10 form kanıtına aittir"));
            let yapılandırma = Arc::from(yapılandırma);
            açık_giriş_kurucusu(Arc::clone(&yapılandırma), &yerel, bağlam)
                .kur(
                    galeri_bileşen_kimliği(kimlik_fabrikası, "galeri.k10", yerel_ad),
                    hizmetler.unicode(),
                    hizmetler.alan_damgası(kimlik_fabrikası),
                    (*yerel).clone(),
                    "",
                    galeri_teması(),
                    pencere,
                    bağlam,
                )
                .map(|sonuç| sonuç.bileşen)
                .map_err(|hata| format!("K10 BİL-010 alanı kurulamadı: {hata:?}"))
        };
        let fallback = kutu("fallback", "K10 güvenli önceki alan", pencere, bağlam)?;
        let hedef = kutu("gecersiz-hedef", "K10 geçersiz form alanı", pencere, bağlam)?;

        uygulama_kökünü_kur(bağlam)?;
        let pencere_kimliği = PencereKimliği(
            kimlik_fabrikası
                .sonraki()
                .map_err(|hata| format!("K10 pencere kimliği üretilemedi: {hata:?}"))?,
        );
        let başlangıç_damgası = CanlıBağlamDamgası {
            bağlam: kimlik_fabrikası
                .sonraki()
                .map_err(|hata| format!("K10 bağlam kimliği üretilemedi: {hata:?}"))?,
            sürüm: BağlamSürümü(1),
        };
        let kapsam = OdakKapsamıBaşvurusu(tanım("form-kapsami"));
        let bölge = OdakBölgesiBaşvurusu(tanım("form-bolgesi"));
        let profil = OdakGezinimProfiliKimliği(tanım("form-profili"));
        let fallback_başvurusu = OdakHedefiBaşvurusu {
            bileşen: fallback.read(bağlam).kimlik().clone(),
            yuva: tanım("fallback-yuvasi"),
        };
        let hedef_başvurusu = OdakHedefiBaşvurusu {
            bileşen: hedef.read(bağlam).kimlik().clone(),
            yuva: tanım("gecersiz-hedef-yuvasi"),
        };

        let fallback_sunumu = fallback
            .read(bağlam)
            .kanonik_erişilebilir_sunum_girdisi(pencere_kimliği, fallback_başvurusu.yuva.clone())
            .map_err(|hata| format!("K10 fallback ORT-009 sunumu kurulamadı: {hata:?}"))?;
        let hedef_sunumu = hedef
            .read(bağlam)
            .kanonik_erişilebilir_sunum_girdisi(pencere_kimliği, hedef_başvurusu.yuva.clone())
            .map_err(|hata| format!("K10 hedef ORT-009 sunumu kurulamadı: {hata:?}"))?;
        let odak_fabrikası = ÖrnekKimliğiFabrikası::yeni_süreç_kapsamı()
            .map_err(|hata| format!("K10 odak olay fabrikası kurulamadı: {hata:?}"))?;
        let (port, kayıt_belirteçleri) =
            bağlam.update_global::<GaleriK10UygulamaKökü, _>(|kök, _| {
                let mut port = kök
                    .odak
                    .portu_kur(pencere_kimliği, başlangıç_damgası, odak_fabrikası)
                    .map_err(|hata| format!("K10 ORT-005 pencere portu kurulamadı: {hata:?}"))?;
                let kayıt_belirteçleri = vec![
                    port.kapsam_kaydet(OdakKapsamıKaydıTanımı {
                        kapsam: kapsam.clone(),
                        üst: None,
                        pencere: pencere_kimliği,
                        modal: false,
                    })
                    .map_err(|hata| format!("K10 odak kapsamı kaydedilemedi: {hata:?}"))?,
                    port.bölge_kaydet(OdakBölgesiKaydıTanımı {
                        bölge: bölge.clone(),
                        üst: None,
                        kapsam: kapsam.clone(),
                        profil: profil.clone(),
                        başlangıç_hedefi: Some(fallback_başvurusu.clone()),
                    })
                    .map_err(|hata| format!("K10 odak bölgesi kaydedilemedi: {hata:?}"))?,
                    port.hedef_kaydet(OdakHedefiKaydıTanımı {
                        hedef: fallback_başvurusu.clone(),
                        pencere: pencere_kimliği,
                        bölge: bölge.clone(),
                        kapsam: kapsam.clone(),
                        profil: profil.clone(),
                        uygunluk: uygunluk(),
                    })
                    .map_err(|hata| format!("K10 fallback hedefi kaydedilemedi: {hata:?}"))?,
                    port.hedef_kaydet(OdakHedefiKaydıTanımı {
                        hedef: hedef_başvurusu.clone(),
                        pencere: pencere_kimliği,
                        bölge: bölge.clone(),
                        kapsam: kapsam.clone(),
                        profil: profil.clone(),
                        uygunluk: uygunluk(),
                    })
                    .map_err(|hata| format!("K10 form hedefi kaydedilemedi: {hata:?}"))?,
                ];
                let damga = port
                    .snapshot(&pencere_kimliği)
                    .map_err(|hata| format!("K10 ORT-005 snapshotı okunamadı: {hata:?}"))?
                    .damga;
                let yayın = kök.erişilebilirlik.snapshotı_kur_ve_yayımla(
                    ErişilebilirSnapshotKuruluşGirdisi {
                        pencere: pencere_kimliği,
                        bağlam: damga,
                        profil: ErişilebilirlikProfili {
                            sürüm: damga.sürüm,
                            uyum: ErişilebilirUyumProfili::Wcag22Aa,
                            etkileşim: ErişilebilirEtkileşimKipi::YoğunMasaüstü,
                            hareket: HareketTercihi::Tam,
                        },
                        düğümler: Arc::from([fallback_sunumu.clone(), hedef_sunumu.clone()]),
                        ilişkiler: Arc::from([]),
                        odak: ErişilebilirOdakSunumu {
                            fiziksel_odak: None,
                            etkin_torun: None,
                        },
                        duyurular: Arc::from([]),
                        odak_yuvaları: Arc::from([
                            fallback_sunumu.odak_yuvasını_eşle(fallback_başvurusu.yuva.clone()),
                            hedef_sunumu.odak_yuvasını_eşle(hedef_başvurusu.yuva.clone()),
                        ]),
                    },
                );
                if !yayın.bulgular().is_empty() {
                    return Err(format!(
                        "K10 ORT-009 snapshotı bulgulu: {} bulgu",
                        yayın.bulgular().len()
                    ));
                }
                Ok((port, kayıt_belirteçleri))
            })?;

        let alan_kimliği = FormAlanıKimliği(tanım("gecersiz-alan"));
        let fiziksel_hedefler = vec![
            GirişKutusuFormOdakHedefiKuruluşu::form_alanı(
                alan_kimliği.clone(),
                hedef_başvurusu.clone(),
                &hedef,
                bağlam,
            )
            .map_err(|hata| format!("K10 form fiziksel hedefi kurulamadı: {hata:?}"))?,
            GirişKutusuFormOdakHedefiKuruluşu::güvenli_fallback(
                fallback_başvurusu,
                &fallback,
                bağlam,
            )
            .map_err(|hata| format!("K10 fiziksel fallback kurulamadı: {hata:?}"))?,
        ];
        let odak = gpui_bilesenleri::FormOdakBağı::kur(
            pencere_kimliği,
            kapsam,
            port,
            fiziksel_hedefler,
            kayıt_belirteçleri,
        )
        .map_err(|hata| format!("K10 form ORT-005 bağı kurulamadı: {hata:?}"))?;

        let alan_tanımı = FormAlanıTanımı {
            kimlik: alan_kimliği.clone(),
            sıra: 0,
            zorunlu: false,
            doğrulama_bağımlılıkları: Arc::from([]),
            json_yolu: None,
            tipografi_yerleşimi: TipografiYerleşimiKimliği(tanım("alan-yerlesimi")),
            gizlenme_politikası: GizlenenAlanDeğeriPolitikası::KoruVeGönder,
        };
        let form_tanımı = FormTanımı {
            kimlik: FormKimliği(tanım("galeri-formu")),
            alanlar: Arc::from([alan_tanımı.clone()]),
            tipografi_kapsamı: TipografiKapsamıKimliği(tanım("tipografi-kapsami")),
        };
        let tipografi = FormTipografiEşlemesi {
            form: form_tanımı.kimlik.clone(),
            kapsam: form_tanımı.tipografi_kapsamı.clone(),
            alan_yerleşimleri: BTreeMap::from([(
                alan_kimliği.clone(),
                alan_tanımı.tipografi_yerleşimi,
            )]),
        };
        let hedef_açık = Rc::new(Cell::new(false));
        let reveal_sayısı = Rc::new(Cell::new(0));
        let kuruluş = FormKuruluşu {
            tanım: form_tanımı,
            tipografi,
            alanlar: BTreeMap::from([(
                alan_kimliği.clone(),
                Box::new(GaleriK10AlanPortu {
                    kimlik: alan_kimliği,
                    kutu: hedef.clone(),
                }) as Box<dyn FormAlanıPortu>,
            )]),
            kontrollü_alan_kuruluşları: BTreeMap::new(),
            gönderim: Box::new(GönderimsizK10Portu),
            reveal: Box::new(GaleriK10RevealPortu {
                hedef_açık: Rc::clone(&hedef_açık),
                reveal_sayısı: Rc::clone(&reveal_sayısı),
            }),
            odak: Some(odak),
        };
        let form = form_denetleyicisini_kur(kuruluş, bağlam)
            .map_err(|hata| format!("K10 formu kurulamadı: {hata}"))?;
        let başlatma = form.update(bağlam, |form, bağlam| {
            form.göndermeyi_başlat(pencere, bağlam)
        });
        if başlatma != Err(FormHatası::GeçersizForm) {
            return Err(format!(
                "K10 geçersiz alanı başlangıçta mühürlenemedi: {başlatma:?}"
            ));
        }

        Ok(Self {
            form,
            fallback,
            hedef,
            hedef_başvurusu,
            hedef_açık,
            reveal_sayısı,
            son_sonuç: Rc::new(RefCell::new(None)),
        })
    }

    pub(crate) fn ilk_geçersizi_göster(
        &self,
        pencere: &mut Window,
        bağlam: &mut App,
    ) -> Result<FormRevealSonucu, FormHatası> {
        let sonuç = self.form.update(bağlam, |form, bağlam| {
            form.ilk_geçersizi_göster(OdakÖnKapıKararı::Serbest, pencere, bağlam)
        });
        *self.son_sonuç.borrow_mut() = Some(format!("{sonuç:?}"));
        sonuç
    }

    pub(crate) fn çiz(&self, bağlam: &mut Context<GaleriUygulaması>) -> AnyElement {
        let p = palet();
        let hedef = self.hedef_açık.get().then(|| {
            div()
                .id("bil-120-k10-hedef")
                .debug_selector(|| "bil-120-k10-hedef".into())
                .mt_2()
                .child(self.hedef.clone())
        });
        let sonuç = self
            .son_sonuç
            .borrow()
            .clone()
            .unwrap_or_else(|| "Henüz reveal isteği yok".to_owned());
        div()
            .id("bil-120-k10-gercek-form")
            .debug_selector(|| "bil-120-k10-gercek-form".into())
            .flex()
            .flex_col()
            .gap_2()
            .rounded_md()
            .border_1()
            .border_color(rgb(p.kenarlık))
            .bg(rgb(p.yüzey))
            .p_3()
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(p.ana_metin))
                    .child("K10 · gerçek form reveal ve odak zinciri"),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(p.ikincil_metin))
                    .child("BİL-010 → ORT-009 → ORT-005 → GPUI FocusHandle"),
            )
            .child(
                div()
                    .id("bil-120-k10-fallback")
                    .mt_2()
                    .child(self.fallback.clone()),
            )
            .children(hedef)
            .child(
                div()
                    .id("bil-120-k10-ilk-gecersizi-goster")
                    .cursor_pointer()
                    .rounded_md()
                    .bg(rgb(p.vurgu_zemin))
                    .text_color(rgb(p.vurgu))
                    .px_3()
                    .py_2()
                    .on_click(bağlam.listener(|uygulama, _, pencere, bağlam| {
                        let _ = uygulama.k10_ilk_geçersizi_göster(pencere, bağlam);
                        bağlam.notify();
                    }))
                    .child("İlk geçersiz alanı göster ve odakla"),
            )
            .child(
                div()
                    .id("bil-120-k10-sonuc")
                    .text_xs()
                    .text_color(rgb(p.ikincil_metin))
                    .child(sonuç),
            )
            .into_any_element()
    }

    pub(crate) fn temayı_değiştir(&self, bağlam: &mut Context<GaleriUygulaması>) {
        for kutu in [&self.fallback, &self.hedef] {
            kutu.update(bağlam, |kutu, bağlam| {
                kutu.temayı_değiştir(galeri_teması(), bağlam)
            });
        }
    }

    pub(crate) fn yerel_bağlamı_değiştir(
        &self,
        kök: &Arc<gpui_bilesenleri::YerelMetinBağlamı>,
        bağlam: &mut Context<GaleriUygulaması>,
    ) -> Option<gpui_bilesenleri::GirişHatası> {
        let mut son_ret = None;
        for kutu in [&self.fallback, &self.hedef] {
            let yeni = (**kök).clone();
            if let Err(hata) = kutu.update(bağlam, |kutu, bağlam| {
                kutu.yerel_bağlamı_değiştir(yeni, bağlam)
            }) {
                son_ret = Some(hata);
            }
        }
        son_ret
    }

    #[doc(hidden)]
    pub(crate) fn hedef(&self) -> Entity<GirişKutusu> {
        self.hedef.clone()
    }

    #[doc(hidden)]
    pub(crate) fn hedef_başvurusu(&self) -> &OdakHedefiBaşvurusu {
        &self.hedef_başvurusu
    }

    #[doc(hidden)]
    pub(crate) fn hedef_açık_mı(&self) -> bool {
        self.hedef_açık.get()
    }

    #[doc(hidden)]
    pub(crate) fn reveal_sayısı(&self) -> usize {
        self.reveal_sayısı.get()
    }
}

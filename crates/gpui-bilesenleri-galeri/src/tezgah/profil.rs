//! Tezgâh kabuğunun `ORT-017` tipli görünüm profili.
//!
//! `ORT-004.ACC-001` iki şeyi birden yasaklar: bileşen çizim kodunda ham renk
//! **ve** dağınık fiziksel ölçü sabiti. Renk `TezgahTokenları` üzerinden
//! çözülür; ölçü ve tipografi burada tek sahiplidir.
//!
//! İki katman vardır ve karıştırılmaz:
//!
//! - **Profil** semantik rol ve ölçü taşır. `Pixels` `ORT-017`in yasak
//!   listesinde değildir; ham renk, fiziksel font, dosya yolu, callback ve
//!   `Entity` ise yasaktır — profil bunların hiçbirini taşımaz.
//! - **Çözülmüş görünüm** profil + tema ile üretilir. `TextStyle` yalnız
//!   burada doğar; tipografi rolü temaya uygulanarak çözülür.
//!
//! Kayıt kapısı **kapılıdır**: `GörünümKayıtDefteri`nin somut uygulayıcısı
//! henüz yok, bu yüzden profil kendi içinde tek sahiplidir ve kayıt
//! defterine bağlıymış gibi sunulmaz.

use gpui::{Pixels, Point, TextStyle, px};
use gpui_bilesenleri::{
    ArayüzYoğunluğu, BağlamSürümü, DüğmeŞekli, GörünümKayıtHatası, GörünümProfiliBaşlığı,
    GörünümProfiliKimliği, KutuŞekliTercihi, KöşeMetrikleri, MantıksalİçBoşluk, TemaAnlıkGörüntüsü,
    TipografiRolü,
};

use super::KolonMetriği;

/// Bir kutu yüzünün ölçü ve şekil metriği.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KutuMetriği {
    pub yatay_dolgu: Pixels,
    pub dikey_dolgu: Pixels,
    /// Köşe yarıçapı `ORT-003 KutuŞekliTercihi`nden çözülür; yerel yuvarlatma
    /// denklemi kurulmaz.
    pub şekil: KutuŞekliTercihi,
}

/// Önizleme kabuğunun ölçüleri.
///
/// Sayılar `Tezgah_yeni_tasarimi/Yerlesim Raporu.dc.html` §1 dökümünden gelir
/// ve başka hiçbir yerde ham `px` olarak tekrarlanmaz.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KabukMetriği {
    pub yükseklik: Pixels,
    /// Tek `gap`; parça başına kenar boşluğu yok.
    pub parça_aralığı: Pixels,
    pub simge_kutusu: Pixels,
    /// Üst-köşe göstergesinin akış dışı ankraj payı.
    pub köşe_ankrajı: Point<Pixels>,
    /// Yazı yönüne duyarlı iç boşluk; RTL'de aynalanır.
    pub iç_boşluk: MantıksalİçBoşluk,
}

/// Tezgâh kabuğunun metrik ve tipografi rolü sahibi.
#[derive(Clone, Debug, PartialEq)]
pub struct TezgahGörünümProfili {
    /// Kanonik profil künyesi. Sürüm profil değişiminde artar: aynı sürüm
    /// aynı değeri verir, tüketici bunu güvenle önbelleğe alabilir.
    pub başlık: GörünümProfiliBaşlığı,
    // --- ölçü: tek sahipli ---
    pub hap: KutuMetriği,
    pub kart: KutuMetriği,
    pub segment: KutuMetriği,
    pub rozet: KutuMetriği,
    pub anahtar_yüksekliği: Pixels,
    pub simge_düğmesi: Pixels,
    pub önizleme_kabuğu: KabukMetriği,
    pub kolonlar: KolonMetriği,
    /// `ORT-003` köşe kademeleri; şekil tercihleri buradan çözülür.
    pub köşe_metrikleri: KöşeMetrikleri,
    // --- tipografi: `ORT-004` rolü; aile ve boyut temadan gelir ---
    pub gövde: TipografiRolü,
    pub bölüm_başlığı: TipografiRolü,
    pub eksen_etiketi: TipografiRolü,
    pub rozet_metni: TipografiRolü,
    pub kod_metni: TipografiRolü,
}

/// Çözülmüş kutu yüzü: yarıçap artık somut.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ÇözülmüşKutuMetriği {
    pub yatay_dolgu: Pixels,
    pub dikey_dolgu: Pixels,
    pub yarıçap: Pixels,
}

/// Profil + tema. `TextStyle` yalnız burada üretilir.
#[derive(Clone, Debug)]
pub struct ÇözülmüşTezgahGörünümü {
    pub hap: ÇözülmüşKutuMetriği,
    pub kart: ÇözülmüşKutuMetriği,
    pub segment: ÇözülmüşKutuMetriği,
    pub rozet: ÇözülmüşKutuMetriği,
    pub anahtar_yüksekliği: Pixels,
    pub simge_düğmesi: Pixels,
    pub önizleme_kabuğu: KabukMetriği,
    pub kolonlar: KolonMetriği,
    pub gövde: TextStyle,
    pub bölüm_başlığı: TextStyle,
    pub eksen_etiketi: TextStyle,
    pub rozet_metni: TextStyle,
    pub kod_metni: TextStyle,
    /// `ORT-004` devre dışı kutu rolü. Pasif yüz opaklık düşürmez: kademeli
    /// görünürlük ayrı bir kavramdır ve `GörselOpaklıkKademesi` yalnız onu
    /// kullanan profilde zorunludur.
    pub devre_dışı: gpui_bilesenleri::KutuRenkleri,
}

/// Ölçünün sonlu ve negatif olmadığı; aksi hâlde tanı üretilir.
fn ölçüyü_denetle(değer: Pixels) -> Result<Pixels, GörünümKayıtHatası> {
    let ham = f32::from(değer);
    if !ham.is_finite() || ham < 0. {
        return Err(GörünümKayıtHatası::GeçersizMetrik);
    }
    Ok(değer)
}

/// `ORT-004 §25` yoğunluk tercihinin tezgâh dolgularına çözümü.
///
/// Yoğunluk temada taşınan tipli bir tercihtir; sayısal karşılığını
/// bileşen sahibinin görünüm profili verir (`ORT-004 §43`). Tezgâhın
/// profili budur, dolayısıyla katsayı burada yaşar — çizim koduna
/// dağılmış ham `px` farkı olarak değil.
///
/// Yalnız dolgular ölçeklenir. Etkileşim hedefi, simge kutusu ve kabuk
/// yüksekliği sabit kalır: `ORT-004 §1240` yoğunluğun `ORT-009` asgari
/// hedefinin altına inmesini yasaklıyor ve Kompakt kipin dokunma hedefini
/// küçültmesi tam da o ihlal olurdu.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DolguÖlçeği {
    yatay: f32,
    dikey: f32,
}

impl DolguÖlçeği {
    /// Dikey eksen daha çok oynar: yoğunluk algısı satır aralığından
    /// okunur, yatay dolgu ise etiket genişliğine bağlı olduğu için aynı
    /// oranda daraltıldığında metin kenara yapışıyor.
    pub const fn yoğunluktan(yoğunluk: ArayüzYoğunluğu) -> Self {
        match yoğunluk {
            ArayüzYoğunluğu::Kompakt => Self {
                yatay: 0.85,
                dikey: 0.6,
            },
            ArayüzYoğunluğu::Normal => Self {
                yatay: 1.,
                dikey: 1.,
            },
            ArayüzYoğunluğu::Geniş => Self {
                yatay: 1.15,
                dikey: 1.45,
            },
        }
    }

    fn yatay(self, değer: Pixels) -> Pixels {
        px(f32::from(değer) * self.yatay)
    }

    fn dikey(self, değer: Pixels) -> Pixels {
        px(f32::from(değer) * self.dikey)
    }
}

impl KutuMetriği {
    fn denetle(&self) -> Result<(), GörünümKayıtHatası> {
        ölçüyü_denetle(self.yatay_dolgu)?;
        ölçüyü_denetle(self.dikey_dolgu)?;
        Ok(())
    }

    /// `ORT-003` şekil tercihini köşe kademelerine uygular.
    ///
    /// `Hap` kademesi bir token değildir: yüksekliğin yarısından türer ve
    /// GPUI çizim tarafında kısa kenara kırpılır.
    fn çöz(
        &self, köşeler: KöşeMetrikleri, yoğunluk: DolguÖlçeği
    ) -> ÇözülmüşKutuMetriği {
        let yarıçap = match self.şekil {
            KutuŞekliTercihi::Yarıçap(değer) => değer,
            diğer => match diğer.çöz(Some(DüğmeŞekli::Yuvarlatılmış)) {
                DüğmeŞekli::DikKöşeli => px(0.),
                DüğmeŞekli::Köşeli => köşeler.köşeli,
                DüğmeŞekli::Yuvarlatılmış => köşeler.yuvarlatılmış,
                DüğmeŞekli::Hap => px(9999.),
            },
        };
        ÇözülmüşKutuMetriği {
            yatay_dolgu: yoğunluk.yatay(self.yatay_dolgu),
            dikey_dolgu: yoğunluk.dikey(self.dikey_dolgu),
            yarıçap,
        }
    }
}

/// `ORT-004` tipografi rolünü temaya uygular.
fn rolü_çöz(rol: TipografiRolü, tema: &TemaAnlıkGörüntüsü) -> TextStyle {
    match rol {
        TipografiRolü::Gövde => tema.tipografi.gövde.clone(),
        TipografiRolü::KüçükGövde => tema.tipografi.küçük_gövde.clone(),
        TipografiRolü::Etiket => tema.tipografi.etiket.clone(),
        TipografiRolü::Başlık => tema.tipografi.başlık.clone(),
        TipografiRolü::TekAralıklı => tema.tipografi.tek_aralıklı.clone(),
    }
}

impl TezgahGörünümProfili {
    /// Tasarımın ölçü dökümünden gelen yerleşik profil.
    pub fn tasarım(kimlik: GörünümProfiliKimliği) -> Self {
        Self {
            başlık: GörünümProfiliBaşlığı {
                kimlik,
                sürüm: BağlamSürümü(1),
                taban: None,
            },
            hap: KutuMetriği {
                yatay_dolgu: px(11.),
                dikey_dolgu: px(5.),
                şekil: KutuŞekliTercihi::Açık(DüğmeŞekli::Hap),
            },
            kart: KutuMetriği {
                yatay_dolgu: px(14.),
                dikey_dolgu: px(14.),
                şekil: KutuŞekliTercihi::Açık(DüğmeŞekli::Köşeli),
            },
            segment: KutuMetriği {
                yatay_dolgu: px(2.),
                dikey_dolgu: px(2.),
                şekil: KutuŞekliTercihi::Açık(DüğmeŞekli::Köşeli),
            },
            rozet: KutuMetriği {
                yatay_dolgu: px(10.),
                dikey_dolgu: px(4.),
                şekil: KutuŞekliTercihi::Açık(DüğmeŞekli::Hap),
            },
            anahtar_yüksekliği: px(22.),
            simge_düğmesi: px(24.),
            önizleme_kabuğu: KabukMetriği {
                yükseklik: px(58.),
                parça_aralığı: px(5.),
                simge_kutusu: px(15.),
                köşe_ankrajı: Point {
                    x: px(5.),
                    y: px(5.),
                },
                // Bölüm I §7.2: üst 8 · sağ 5 · alt 6 · sol 6. Dikey asimetri
                // optik taban çizgisi hizası içindir.
                iç_boşluk: MantıksalİçBoşluk::denetimli(px(6.), px(5.), px(8.), px(6.))
                    .expect("tasarım iç boşluğu sonlu ve negatif değildir"),
            },
            kolonlar: KolonMetriği::tasarım(),
            köşe_metrikleri: KöşeMetrikleri {
                köşeli: px(3.),
                yuvarlatılmış: px(8.),
            },
            gövde: TipografiRolü::Gövde,
            bölüm_başlığı: TipografiRolü::Etiket,
            eksen_etiketi: TipografiRolü::KüçükGövde,
            rozet_metni: TipografiRolü::KüçükGövde,
            kod_metni: TipografiRolü::TekAralıklı,
        }
    }

    /// Profilin bütün ölçüleri sonlu ve negatif olmamalı.
    ///
    /// Ret sessizce yerleşik profile düşmez: çağıran tanıyı görür.
    pub fn doğrula(&self) -> Result<(), GörünümKayıtHatası> {
        self.hap.denetle()?;
        self.kart.denetle()?;
        self.segment.denetle()?;
        self.rozet.denetle()?;
        ölçüyü_denetle(self.anahtar_yüksekliği)?;
        ölçüyü_denetle(self.simge_düğmesi)?;
        ölçüyü_denetle(self.önizleme_kabuğu.yükseklik)?;
        ölçüyü_denetle(self.önizleme_kabuğu.parça_aralığı)?;
        ölçüyü_denetle(self.önizleme_kabuğu.simge_kutusu)?;
        ölçüyü_denetle(self.kolonlar.önizleme_kolonu)?;
        ölçüyü_denetle(self.kolonlar.kolon_aralığı)?;
        ölçüyü_denetle(self.kolonlar.yapılandırma_asgarisi)?;
        Ok(())
    }

    /// Profil + tema → çözülmüş görünüm.
    ///
    /// Şekil tercihi yarıçapa, tipografi rolü `TextStyle`a, yoğunluk
    /// tercihi dolgu ölçeğine çözülür. Ham metrik profilde kalır: çözüm
    /// onu okur, üzerine yazmaz.
    pub fn çöz(
        &self,
        tema: &TemaAnlıkGörüntüsü,
    ) -> Result<ÇözülmüşTezgahGörünümü, GörünümKayıtHatası> {
        self.doğrula()?;
        let d = DolguÖlçeği::yoğunluktan(tema.bağlam.yoğunluk);
        Ok(ÇözülmüşTezgahGörünümü {
            hap: self.hap.çöz(self.köşe_metrikleri, d),
            kart: self.kart.çöz(self.köşe_metrikleri, d),
            segment: self.segment.çöz(self.köşe_metrikleri, d),
            rozet: self.rozet.çöz(self.köşe_metrikleri, d),
            anahtar_yüksekliği: self.anahtar_yüksekliği,
            simge_düğmesi: self.simge_düğmesi,
            önizleme_kabuğu: self.önizleme_kabuğu,
            kolonlar: self.kolonlar,
            gövde: rolü_çöz(self.gövde, tema),
            bölüm_başlığı: rolü_çöz(self.bölüm_başlığı, tema),
            eksen_etiketi: rolü_çöz(self.eksen_etiketi, tema),
            rozet_metni: rolü_çöz(self.rozet_metni, tema),
            kod_metni: rolü_çöz(self.kod_metni, tema),
            devre_dışı: tema
                .ortak_kutu_rolleri
                .as_ref()
                .map(|roller| roller.devre_dışı.clone())
                .ok_or(GörünümKayıtHatası::GeçersizMetrik)?,
        })
    }
}

thread_local! {
    /// Bu karede çizilen çözülmüş görünüm.
    ///
    /// Palet gibi kare başında bir kez kurulur ve çizim ağacı kurulurken
    /// okunur: yüz yardımcılarının çoğu bağlam almadığı için ölçü ve
    /// tipografi buradan çözülür. Galeri tek iş parçacıklı çizim kodudur,
    /// kare içinde görünüm değişmez ve yalnız okunur.
    static ETKİN_GÖRÜNÜM: std::cell::RefCell<Option<std::sync::Arc<ÇözülmüşTezgahGörünümü>>> =
        const { std::cell::RefCell::new(None) };
}

thread_local! {
    /// Bu karede açık olan seçicinin kimliği.
    ///
    /// Palet ve görünüm gibi kare başında kurulur. Seçiciler çizim
    /// ağacının derinlerinde (bölüm içeriklerinde) duruyor ve durumu
    /// oraya parametre olarak taşımak her ara fonksiyona bir alan daha
    /// eklerdi.
    static AÇIK_SEÇİCİ: std::cell::RefCell<Option<gpui::SharedString>> =
        const { std::cell::RefCell::new(None) };
}

/// Kare başında açık seçiciyi kurar.
pub fn açık_seçiciyi_kur(kimlik: Option<gpui::SharedString>) {
    AÇIK_SEÇİCİ.with(|hücre| *hücre.borrow_mut() = kimlik);
}

/// Bu seçici açık mı?
pub fn seçici_açık_mı(kimlik: &str) -> bool {
    AÇIK_SEÇİCİ.with(|hücre| hücre.borrow().as_deref() == Some(kimlik))
}

/// Kare başında çözülmüş görünümü kurar.
pub fn görünümü_kur(görünüm: ÇözülmüşTezgahGörünümü) {
    görünümü_paylaşımlı_kur(std::sync::Arc::new(görünüm));
}

/// Hazır (paylaşımlı) çözümü kurar.
///
/// Çözüm tema sürümüne bağlı önbellekten geliyorsa yeniden sarmalanmaz;
/// aynı `Arc` kare kare paylaşılır.
pub fn görünümü_paylaşımlı_kur(görünüm: std::sync::Arc<ÇözülmüşTezgahGörünümü>) {
    ETKİN_GÖRÜNÜM.with(|hücre| {
        *hücre.borrow_mut() = Some(görünüm);
    });
}

/// Tasarım profilini o anki temadan çözer.
///
/// Kimlik sabittir: tezgâhın tek tasarım profili vardır ve çözüm yalnız
/// temaya göre değişir.
pub fn tasarım_görünümünü_çöz() -> ÇözülmüşTezgahGörünümü {
    let kimlik = GörünümProfiliKimliği(
        gpui_bilesenleri::TanımKimliği::denetimli(
            std::sync::Arc::from("galeri.tezgah"),
            std::sync::Arc::from("tasarım"),
        )
        .expect("tasarım profili kimliği geçerlidir"),
    );
    TezgahGörünümProfili::tasarım(kimlik)
        .çöz(&crate::galeri_teması())
        .expect("tasarım profili çözülür")
}

/// Bu karede etkin çözülmüş görünüm.
///
/// Kurulmamışsa tasarım profili temadan çözülür: çizim sessizce ham ölçüye
/// düşmez. Bu yol **önbelleğe alınmaz** — tema değişince bayat ölçü
/// döndürmemesi için her çağrıda yeniden çözülür; sıcak yolda
/// [`görünümü_kur`] zaten kare başında kurar.
pub fn görünüm() -> std::sync::Arc<ÇözülmüşTezgahGörünümü> {
    ETKİN_GÖRÜNÜM.with(|hücre| {
        if let Some(mevcut) = hücre.borrow().as_ref() {
            return std::sync::Arc::clone(mevcut);
        }
        std::sync::Arc::new(tasarım_görünümünü_çöz())
    })
}

#[cfg(test)]
mod testler {
    use super::*;
    use gpui_bilesenleri::TanımKimliği;
    use std::sync::Arc;

    fn kimlik() -> GörünümProfiliKimliği {
        GörünümProfiliKimliği(
            TanımKimliği::denetimli(Arc::from("galeri.tezgah"), Arc::from("tasarım"))
                .expect("tasarım profili kimliği geçerlidir"),
        )
    }

    #[test]
    fn şekil_tercihi_köşe_kademelerinden_çözülür() {
        let profil = TezgahGörünümProfili::tasarım(kimlik());
        let köşeler = profil.köşe_metrikleri;
        // Hap kademesi token değildir; kısa kenara kırpılmak üzere büyük
        // verilir.
        assert!(
            profil
                .hap
                .çöz(köşeler, DolguÖlçeği::yoğunluktan(ArayüzYoğunluğu::Normal))
                .yarıçap
                > köşeler.yuvarlatılmış
        );
        assert_eq!(
            profil
                .kart
                .çöz(köşeler, DolguÖlçeği::yoğunluktan(ArayüzYoğunluğu::Normal))
                .yarıçap,
            köşeler.köşeli
        );

        let dik = KutuMetriği {
            şekil: KutuŞekliTercihi::Açık(DüğmeŞekli::DikKöşeli),
            ..profil.kart
        };
        assert_eq!(
            dik.çöz(köşeler, DolguÖlçeği::yoğunluktan(ArayüzYoğunluğu::Normal))
                .yarıçap,
            px(0.)
        );

        // Açık piksel tercihi kademeyi ezer.
        let açık = KutuMetriği {
            şekil: KutuŞekliTercihi::Yarıçap(px(11.)),
            ..profil.kart
        };
        assert_eq!(
            açık
                .çöz(köşeler, DolguÖlçeği::yoğunluktan(ArayüzYoğunluğu::Normal))
                .yarıçap,
            px(11.)
        );
    }

    #[test]
    fn geçersiz_ölçü_tanıyla_reddedilir() {
        let mut profil = TezgahGörünümProfili::tasarım(kimlik());
        profil.anahtar_yüksekliği = px(-1.);
        assert_eq!(profil.doğrula(), Err(GörünümKayıtHatası::GeçersizMetrik));

        let mut profil = TezgahGörünümProfili::tasarım(kimlik());
        profil.kart.yatay_dolgu = px(f32::NAN);
        assert_eq!(profil.doğrula(), Err(GörünümKayıtHatası::GeçersizMetrik));
    }

    /// Tasarım profili kendi denetiminden geçer.
    #[test]
    fn tasarım_profili_geçerlidir() {
        TezgahGörünümProfili::tasarım(kimlik()).doğrula().unwrap();
    }

    /// Profil künyesi kanonik `GörünümProfiliBaşlığı`dır: kimlik, sürüm ve
    /// taban birlikte taşınır. Aynı sürüm aynı değeri verir.
    #[test]
    fn profil_künyesi_sürüm_taşır() {
        let a = TezgahGörünümProfili::tasarım(kimlik());
        let b = TezgahGörünümProfili::tasarım(kimlik());
        assert_eq!(a.başlık.sürüm, b.başlık.sürüm);
        assert_eq!(a, b, "aynı sürüm aynı profili verir");
        assert!(a.başlık.taban.is_none(), "tasarım profili tabansızdır");
    }
}

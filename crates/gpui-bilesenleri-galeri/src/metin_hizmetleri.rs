//! `ORT-002`/`ORT-021` uygulama-kökü metin hizmetleri.
//!
//! Galeri uygulaması (`GaleriUygulaması`) gerçek bileşim köküdür: `ORT-002`
//! Unicode/yerel hizmet kökü, yaşayan `YerelMetinBağlamı`, `ORT-021` katalog
//! kütüğü ve mühürlü çözüm hizmeti **burada bir kez** kurulur. Bileşenler
//! (`BİL-010` dâhil) bu kaynakların sahibi değildir; yalnız hosttan verilen
//! public capability değerlerini tüketir.
//!
//! Kurallar:
//! - Uygulama kökü başına tek Unicode kökü ve tek ileti çözüm hizmeti.
//! - Hizmet **kök-kapsamlıdır**: yerel kök değişince katalog kütüğü ve
//!   çözüm hizmeti tek atomda yeniden mühürlenir; eski hizmet yeni kökün
//!   hizmeti sayılmaz (`İletiÇözümHatası::BayatYerelBağlam`).
//! - Kimlik tanıma, alias ve dil-yönü mantığı burada yeniden yazılmaz;
//!   kimlikler motorun doğrulama kapısından, yön çözümü bağlam
//!   fabrikasından gelir.

use std::{collections::BTreeMap, sync::Arc};

use gpui_bilesenleri_temel::{
    BağlamSürümü, CanlıBağlamDamgası, DilEtiketi, GüvenliMetin, KayıtBelirteci, MetinDamgası,
    SaatDilimiKimliği, UnicodeVeYerelMetinHizmetleri, YerelMetinBağlamı, YerelleştirmeAnahtarı,
    ÖrnekKimliği, ÖrnekKimliğiFabrikası, İletiBütçesi, İletiDüğümü, İletiKataloğuKimliği,
    İletiKataloğuKütüğü, İletiKataloğuPaketi, İletiÇözümHatası, İletiÇözümHizmeti,
    İletiÇözümleyicisi, İletiİsteği, İletiŞablonu,
};

/// Tezgâhın `ORT-021` katalog kimliği.
const TEZGAH_KATALOĞU: &str = "galeri.tezgah";

/// Taşınan metin-girişi yüzeyinin Türkçe katalog kayıtları.
///
/// Eskiden `tezgah_bölüm_adı` içinde kod olarak duran sözlük artık gerçek
/// bir `İletiKataloğuPaketi`dir; çözüm mühürlü `İletiÇözümHizmeti`nden
/// geçer. Diğer bileşen ailelerinin placeholder metinleri (aile adı ve
/// açıklamaları) bu katalogda değildir — o aileler henüz taşınmadı.
const TEZGAH_KAYITLARI: &[(&str, &str)] = &[
    ("galeri.tezgah.başlık", "Yapılandırma Tezgâhı"),
    ("galeri.tezgah.önizleme", "Önizleme ve kabuk denetimleri"),
    ("galeri.tezgah.yapılandırma", "Yapılandırma eksenleri"),
    ("galeri.tezgah.bölüm.deger_turu", "Değer türü"),
    (
        "galeri.tezgah.bölüm.tur_tanimi_ve_maske",
        "Tür tanımı ve giriş maskesi",
    ),
    ("galeri.tezgah.bölüm.bicim_profili", "Biçim profili"),
    ("galeri.tezgah.bölüm.on_ek_son_ek", "Ön ek ve son ek"),
    ("galeri.tezgah.bölüm.hacim_ve_sayac", "Hacim ve sayaç"),
    ("galeri.tezgah.bölüm.sayisal_adim", "Sayısal adım"),
    (
        "galeri.tezgah.bölüm.icerik_gorunurlugu",
        "İçerik görünürlüğü",
    ),
    ("galeri.tezgah.bölüm.metin_isleme", "Metin işleme"),
    ("galeri.tezgah.bölüm.yapistirma", "Yapıştırma"),
    (
        "galeri.tezgah.bölüm.bolut_ve_gonderim",
        "Bitişik bölüt ve arama gönderimi",
    ),
    ("galeri.tezgah.bölüm.port_kapilari", "Port kapıları"),
    (
        "galeri.tezgah.bölüm.turetilmis_durum",
        "Türetilmiş durumlar",
    ),
    (
        "galeri.tezgah.bölüm.yapilandirma_dogrulamasi",
        "Yapılandırma doğrulaması",
    ),
    ("galeri.tezgah.bölüm.dis_dogrulama", "Dış doğrulama"),
    ("galeri.tezgah.bölüm.odak_ve_kabul", "Odak, kabul ve erişim"),
    (
        "galeri.tezgah.bölüm.secici_ve_erisim",
        "Seçici ve erişilebilirlik",
    ),
    ("galeri.tezgah.bölüm.otomatik_doldurma", "Otomatik doldurma"),
    ("galeri.tezgah.bölüm.saat_dilimi", "Saat dilimi"),
];

/// Verilen dil için tezgâh katalog paketini kurar.
fn tezgah_kataloğu(dil: DilEtiketi) -> İletiKataloğuPaketi {
    let şablonlar: BTreeMap<YerelleştirmeAnahtarı, İletiŞablonu> = TEZGAH_KAYITLARI
        .iter()
        .map(|(anahtar, metin)| {
            let anahtar = YerelleştirmeAnahtarı::yeni(*anahtar)
                .expect("yerleşik tezgâh katalog anahtarı geçerlidir");
            (
                anahtar.clone(),
                İletiŞablonu {
                    anahtar,
                    şema: Default::default(),
                    kök: Arc::from([İletiDüğümü::Sabit(
                        GüvenliMetin::yeni(*metin, false, true),
                    )]),
                },
            )
        })
        .collect();
    İletiKataloğuPaketi {
        kimlik: İletiKataloğuKimliği::yeni(TEZGAH_KATALOĞU)
            .expect("yerleşik tezgâh katalog kimliği geçerlidir"),
        dil,
        sürüm: BağlamSürümü(1),
        şablonlar,
    }
}

/// Uygulama kökünün metin hizmetleri: `ORT-002` kökü, yaşayan yerel bağlam
/// ve kök-kapsamlı `ORT-021` çözüm hizmeti.
pub(crate) struct MetinHizmetleriKökü {
    unicode: Arc<UnicodeVeYerelMetinHizmetleri>,
    /// Yaşayan yerel kökün `ORT-001` bağlam kimliği; kök yenilense de
    /// soy aynı kalır, yalnız sürüm artar.
    yerel_kök_kimliği: ÖrnekKimliği,
    yerel_kök_sürümü: u64,
    yerel_kök: Arc<YerelMetinBağlamı>,
    ileti_hizmeti: Arc<İletiÇözümHizmeti>,
    /// Katalog kaydının drop-guard belirteci: düşerse kayıt geri alınır ve
    /// yaşayan snapshot'lar `BayatKatalog`a düşer. Hizmetle birlikte yaşar.
    _katalog_kaydı: KayıtBelirteci,
    /// Kararlı anahtar çözümlerinin kare-başı maliyetini kaldıran önbellek.
    ///
    /// Girdiler yaşayan katalog **damgasıyla kuşaklanır**: sunum çözümü her
    /// çağrıda damgayı canlı kütükten okur, damga değiştiyse (yeniden
    /// mühürleme ya da katalog kaybı) önbellek boşalır ve çözüm canlı
    /// koşar. Yalnız **başarılı** çözümler girer — hata yolları nöbettir
    /// ve önbellek hiçbir akıbeti maskeleyemez.
    çözüm_önbelleği: std::rc::Rc<std::cell::RefCell<ÇözümÖnbelleği>>,
    /// Tezgâhın saat dilimi tercih seçenekleri; kimlikler motor-sabittir ve
    /// kuruluşta bir kez kayıt yolundan çözülür.
    dilim_seçenekleri: SaatDilimiSeçenekleri,
}

impl MetinHizmetleriKökü {
    /// Kökü bir kez kurar. Galerinin sunum dili Türkçedir; saat dilimi
    /// tercihten çözülen kimliktir, yoksa `UTC` sunum yedeğine düşülür.
    pub(crate) fn kur(
        fabrika: &ÖrnekKimliğiFabrikası,
        saat_dilimi: Option<&SaatDilimiKimliği>,
    ) -> Self {
        // `ORT-002 §11` edinme kökü kendi kimlik fabrikasını enjekte ister
        // ve onu **tüketir**; host'un fabrikası yaşamaya devam ettiği için
        // köke ayrı bir süreç-kapsamı soyu verilir. İki soy süreç-kapsamı
        // garantisiyle çakışmaz; tek-soy ideali `yerlesik`in değer-alan
        // imzasıyla bilinçli takas edilir.
        let unicode = UnicodeVeYerelMetinHizmetleri::yerlesik(
            ÖrnekKimliğiFabrikası::yeni_süreç_kapsamı()
                .expect("galeri Unicode kökü kimlik kapsamı kurulamadı"),
        );
        let yerel_kök_kimliği = fabrika
            .sonraki()
            .expect("galeri yerel kök kimliği üretim soyu tükendi");
        let yerel_kök_sürümü = 1;
        let yerel_kök = Arc::new(Self::yerel_kök_üret(
            &unicode,
            yerel_kök_kimliği,
            yerel_kök_sürümü,
            saat_dilimi,
        ));
        let (ileti_hizmeti, katalog_kaydı) =
            Self::hizmeti_mühürle(fabrika, &unicode, Arc::clone(&yerel_kök));
        let dilim_seçenekleri = Self::dilim_seçeneklerini_çöz(&unicode);
        Self {
            unicode,
            yerel_kök_kimliği,
            yerel_kök_sürümü,
            yerel_kök,
            ileti_hizmeti,
            _katalog_kaydı: katalog_kaydı,
            çözüm_önbelleği: std::rc::Rc::new(std::cell::RefCell::new(
                ÇözümÖnbelleği::default(),
            )),
            dilim_seçenekleri,
        }
    }

    /// Yaşayan yerel bağlamı güncel `ORT-002` motor/fabrika yolundan üretir.
    ///
    /// Struct literal kurulmaz: dört kimlik motorun doğrulama kapısından
    /// doğar, yazı yönünü fabrikanın mühürlü çözücüsü seçer.
    fn yerel_kök_üret(
        unicode: &UnicodeVeYerelMetinHizmetleri,
        kimlik: ÖrnekKimliği,
        sürüm: u64,
        saat_dilimi: Option<&SaatDilimiKimliği>,
    ) -> YerelMetinBağlamı {
        let motor = unicode.motor();
        let dil = motor
            .dil_etiketi("tr")
            .expect("galeri sunum dili `tr` kayıtlarda tanınır");
        let numaralandırma = motor
            .numaralandırma_sistemi("latn")
            .expect("galeri numaralandırma sistemi `latn` kayıtlarda tanınır");
        let takvim = motor
            .takvim("gregory")
            .expect("galeri takvimi `gregory` kayıtlarda tanınır");
        // `ORT-002 §5.2` çözümü kimlik veremediyse (yalnız GMT farkı) sunum
        // yedeği `UTC`dir; kimlik yine kayıtlardan çözülür, elle mühürlenmez.
        let saat_dilimi = match saat_dilimi {
            Some(kimlik) => kimlik.clone(),
            None => motor
                .saat_dilimi("UTC")
                .expect("`UTC` yerleşik saat dilimi kayıtlarında bulunur"),
        };
        unicode.yerel_bağlam_fabrikası().bağlam(
            CanlıBağlamDamgası {
                bağlam: kimlik,
                sürüm: BağlamSürümü(sürüm),
            },
            dil,
            numaralandırma,
            takvim,
            saat_dilimi,
        )
    }

    /// Katalog kütüğünü ve kök-kapsamlı çözüm hizmetini tek atomda mühürler.
    ///
    /// `İletiÇözümHizmeti::muhurle` için **tek üretim bileşim-kökü çağrısı**
    /// budur; başka hiçbir üretim yolu hizmet mühürlemez.
    fn hizmeti_mühürle(
        fabrika: &ÖrnekKimliğiFabrikası,
        unicode: &UnicodeVeYerelMetinHizmetleri,
        yerel_kök: Arc<YerelMetinBağlamı>,
    ) -> (Arc<İletiÇözümHizmeti>, KayıtBelirteci) {
        let kütük = İletiKataloğuKütüğü::muhurle(
            fabrika
                .sonraki()
                .expect("galeri katalog yayın kökü üretim soyu tükendi"),
            İletiBütçesi::default(),
        );
        let kayıt = kütük
            .kaydet(tezgah_kataloğu(yerel_kök.dil().clone()))
            .unwrap_or_else(|hata| panic!("galeri tezgâh kataloğu kaydedilemedi: {hata}"));
        let hizmet = Arc::new(İletiÇözümHizmeti::muhurle(
            kütük,
            unicode.motor(),
            yerel_kök,
        ));
        (hizmet, kayıt)
    }

    /// Saat dilimi tercihi değiştiyse yerel kökü ve kök-kapsamlı hizmeti
    /// **tek atomda** yeniler; değişmediyse `Ok(None)` döner.
    ///
    /// Eski çözüm hizmeti sessizce kullanılmaz: bağlam, kütük ve hizmet
    /// birlikte değişir, eski bağlam yeni hizmette exact
    /// `BayatYerelBağlam` üretir.
    ///
    /// Sürüm ekseni **doyurulmaz**: `u64` tükenirse farklı bir bağlam aynı
    /// damgayla yayımlanamaz; yenileme hiçbir şeyi değiştirmeden exact
    /// [`YerelKökHatası::SürümEkseniTükendi`] ile reddedilir ve eski kök
    /// kendi (hâlâ doğru) bağlamıyla yürürlükte kalır.
    pub(crate) fn yerel_kökü_gerekirse_yenile(
        &mut self,
        fabrika: &ÖrnekKimliğiFabrikası,
        saat_dilimi: Option<&SaatDilimiKimliği>,
    ) -> Result<Option<Arc<YerelMetinBağlamı>>, YerelKökHatası> {
        let hedef = match saat_dilimi {
            Some(kimlik) => kimlik.clone(),
            None => self
                .unicode
                .motor()
                .saat_dilimi("UTC")
                .expect("`UTC` yerleşik saat dilimi kayıtlarında bulunur"),
        };
        if self.yerel_kök.sunum_saat_dilimi() == &hedef {
            return Ok(None);
        }
        let sonraki_sürüm = self
            .yerel_kök_sürümü
            .checked_add(1)
            .ok_or(YerelKökHatası::SürümEkseniTükendi)?;
        let yeni_kök = Arc::new(Self::yerel_kök_üret(
            &self.unicode,
            self.yerel_kök_kimliği,
            sonraki_sürüm,
            Some(&hedef),
        ));
        let (hizmet, kayıt) =
            Self::hizmeti_mühürle(fabrika, &self.unicode, Arc::clone(&yeni_kök));
        self.yerel_kök_sürümü = sonraki_sürüm;
        self.yerel_kök = Arc::clone(&yeni_kök);
        self.ileti_hizmeti = hizmet;
        self._katalog_kaydı = kayıt;
        // Yeni mühür kuşağı: eski kuşağın çözümleri geçersizdir. (Damga
        // kuşağı bir sonraki çözümde de yakalar; erken boşaltma eski
        // girdilerin belleğini hemen bırakır.)
        *self.çözüm_önbelleği.borrow_mut() = ÇözümÖnbelleği::default();
        Ok(Some(yeni_kök))
    }

    pub(crate) fn unicode(&self) -> Arc<UnicodeVeYerelMetinHizmetleri> {
        Arc::clone(&self.unicode)
    }

    /// `ORT-002` motoru: kimlik doğrulama kapısı.
    pub(crate) fn motor(&self) -> Arc<gpui_bilesenleri_temel::UnicodeMetinMotoru> {
        self.unicode.motor()
    }

    /// Tezgâhın saat dilimi tercih listesi; kimlikler kayıt yolundan doğar.
    ///
    /// Kimlikler motor-sabittir: kuruluşta bir kez çözülür, her erişim
    /// yalnız `Arc` klonlar — kolon her kuruluşunda kayıt yolunu yeniden
    /// gezmez.
    pub(crate) fn dilim_seçenekleri(&self) -> SaatDilimiSeçenekleri {
        self.dilim_seçenekleri.clone()
    }

    fn dilim_seçeneklerini_çöz(
        unicode: &UnicodeVeYerelMetinHizmetleri,
    ) -> SaatDilimiSeçenekleri {
        let motor = unicode.motor();
        let çöz = |ham: &str| {
            motor
                .saat_dilimi(ham)
                .unwrap_or_else(|hata| panic!("yerleşik saat dilimi tanınmalı: {ham} ({hata:?})"))
        };
        SaatDilimiSeçenekleri {
            kullanıcı: Arc::from(vec![
                çöz("Europe/Istanbul"),
                çöz("Europe/London"),
                çöz("America/New_York"),
            ]),
            ürün: çöz("UTC"),
        }
    }

    pub(crate) fn yerel_kök(&self) -> Arc<YerelMetinBağlamı> {
        Arc::clone(&self.yerel_kök)
    }

    #[cfg(test)]
    pub(crate) fn ileti_hizmeti(&self) -> Arc<İletiÇözümHizmeti> {
        Arc::clone(&self.ileti_hizmeti)
    }

    /// Bir alanın yaşayan metin çizgisini başlatan damga.
    ///
    /// Her alan kendi soyu ile başlar; damga kutu tarafından yeniden
    /// üretilmez (`BİL-010.ACC-154`).
    pub(crate) fn alan_damgası(&self, fabrika: &ÖrnekKimliğiFabrikası) -> MetinDamgası {
        self.unicode
            .metin_damgası_fabrikası()
            .damga(CanlıBağlamDamgası {
                bağlam: fabrika
                    .sonraki()
                    .expect("galeri alan damgası üretim soyu tükendi"),
                sürüm: BağlamSürümü(0),
            })
    }

    /// Çizim ağacına verilen klonlanabilir ileti çözücüsü.
    ///
    /// `hata_kaydı` uygulama kökünün yaşayan yuvasıdır: UI sunumu dizeye
    /// inmeden önce typed akıbet oraya yazılır ve kökten gözlenebilir kalır.
    pub(crate) fn çözücü(
        &self,
        hata_kaydı: std::rc::Rc<std::cell::RefCell<Option<TezgahÇözümKaydı>>>,
    ) -> TezgahİletiÇözücüsü {
        TezgahİletiÇözücüsü {
            hizmet: Arc::clone(&self.ileti_hizmeti),
            yerel_kök: Arc::clone(&self.yerel_kök),
            hata_kaydı,
            önbellek: std::rc::Rc::clone(&self.çözüm_önbelleği),
        }
    }
}

/// Yaşayan yerel kökün yenileme akıbeti.
///
/// Sürüm ekseni tükenmesi terminaldir: yeni bir bağlam damgası
/// basılamayacağı için kök değişimi reddedilir; sessiz doyurma yoktur.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum YerelKökHatası {
    SürümEkseniTükendi,
}

/// Tezgâhın saat dilimi tercih seçenekleri.
///
/// Kimlikler `ORT-002` kayıt yolundan çözülmüş gelir; sergi elle kimlik
/// mühürlemez.
#[derive(Clone)]
pub(crate) struct SaatDilimiSeçenekleri {
    pub(crate) kullanıcı: Arc<[SaatDilimiKimliği]>,
    pub(crate) ürün: SaatDilimiKimliği,
}

/// Canlı UI çözüm yolunun typed hata kanalı.
///
/// `İletiÇözümHatası` kayıpsız taşınır; `etkin_katalog`ın iki nedeni
/// (bayat bağlam / kayıtlı zincir yok) burada exact ayrıştırılır. UI bu
/// kanalı tek bir anahtar-metni fallback'ine indirmez.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TezgahÇözümHatası {
    /// Mühürlü çözümleyicinin exact hatası (`BayatYerelBağlam`,
    /// `BayatKatalog`, `İstek(EksikAnahtar)` …).
    Çözüm(İletiÇözümHatası),
    /// Hizmet bu bağlam için kayıtlı katalog zinciri bulamadı (bağlam
    /// hizmetin köküyle eşitken).
    KayıtlıZincirYok,
}

/// Canlı UI çözüm yolunda yakalanan typed akıbet kaydı.
///
/// `çöz` sunum dizesini üretmeden **önce** bunu kökün yaşayan yuvasına
/// yazar; typed payload UI durumunda korunur, dizeye indirgenerek
/// kaybolmaz.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TezgahÇözümKaydı {
    pub(crate) anahtar: YerelleştirmeAnahtarı,
    pub(crate) hata: TezgahÇözümHatası,
}

/// Tezgâh çiziminin `ORT-021` anahtarını dizeye çeviren kök-kapsamlı
/// çözücüsü.
///
/// Kabuğun `çöz` imzasına (`Fn(&YerelleştirmeAnahtarı) -> SharedString`)
/// kapanış olarak verilir. Typed çekirdek [`Self::çöz_sonuç`]tır; `çöz`
/// onun sunum bağdaştırıcısıdır ve hiçbir akıbeti sessizleştirmez: eksik
/// katalog kaydı anahtarın kendisiyle görünür, diğer exact hatalar önce
/// typed olarak kök yuvasına kaydedilir, sonra ekranda varyant adıyla
/// işaretlenir.
#[derive(Clone)]
pub(crate) struct TezgahİletiÇözücüsü {
    hizmet: Arc<İletiÇözümHizmeti>,
    yerel_kök: Arc<YerelMetinBağlamı>,
    /// Kökün yaşayan typed-akıbet yuvası; son çözüm hatasını taşır.
    hata_kaydı: std::rc::Rc<std::cell::RefCell<Option<TezgahÇözümKaydı>>>,
    /// Kökün mühür-kuşağı çözüm önbelleği; yalnız `çöz`ün başarılı sunum
    /// yolu kullanır. Typed çekirdek (`çöz_sonuç`/`yokla`) her zaman canlı
    /// koşar — nöbet önbellekle köreltilmez.
    önbellek: std::rc::Rc<std::cell::RefCell<ÇözümÖnbelleği>>,
}

impl TezgahİletiÇözücüsü {
    /// Kaydedilebilir akıbeti kökün yaşayan yuvasına yazar; yazılan kaydı
    /// döndürür.
    ///
    /// `İstek(EksikAnahtar)` sözleşmeli anahtar-fallback'idir ve kayda
    /// girmez (`None`); diğer her akıbet typed olarak yuvaya düşer.
    fn kaydet(
        &self,
        anahtar: &YerelleştirmeAnahtarı,
        hata: TezgahÇözümHatası,
    ) -> Option<TezgahÇözümKaydı> {
        if matches!(
            hata,
            TezgahÇözümHatası::Çözüm(İletiÇözümHatası::İstek(
                gpui_bilesenleri_temel::İletiİsteğiHatası::EksikAnahtar,
            ))
        ) {
            return None;
        }
        let kayıt = TezgahÇözümKaydı {
            anahtar: anahtar.clone(),
            hata,
        };
        self.hata_kaydı.borrow_mut().replace(kayıt.clone());
        Some(kayıt)
    }

    /// Aynı karenin sistemik tanısı: kanonik başlık anahtarı **sunumdan
    /// önce** yoklanır; kaydedilebilir akıbet yuvaya bu karede düşer ve
    /// tanı satırı aynı render girdisiyle çizilir — görünürlük ikinci bir
    /// kare planına bağlı değildir. Bayat kök/katalog gibi sistemik
    /// akıbetler her anahtarı aynı biçimde vurur; anahtar-yerel akıbetler
    /// zaten kendi öğelerinde işaretle görünür.
    ///
    /// Maliyet dürüstçe: kare başına **bir canlı çözüm** (küçük sabit
    /// şablonun yürünmesi + hazır metin üretimi). Nöbetin bedeli budur ve
    /// bilinçli olarak önbelleğe alınmaz — önbellek nöbeti köreltirdi;
    /// kabuğun diğer kararlı anahtarları ise önbellekten gelir.
    pub(crate) fn yokla(&self) -> Option<TezgahÇözümKaydı> {
        let anahtar = crate::anahtar("galeri.tezgah.başlık");
        match self.çöz_sonuç(&anahtar) {
            Ok(_) => {
                // Kanonik anahtar bu hizmet+kök çiftiyle çözülebiliyorsa
                // sistemik akıbet tanım gereği geçmiştir: mandal bırakılır,
                // tanı satırı söner. Anahtar-yerel varyantlar "son hata"
                // semantiğiyle yuvada kalır.
                self.sistemik_mandalı_bırak();
                None
            }
            // Yalnız BU yoklamanın yazdığı kayıt döner; kanonik anahtarın
            // eksikliği sözleşmeli fallback'tir ve bayat yuva içeriği
            // yoklama sonucu gibi sunulmaz.
            Err(hata) => match self.kaydet(&anahtar, hata) {
                Some(kayıt) => Some(kayıt),
                // `EksikAnahtar`: zincir (kök + katalog + dil) tutarlı
                // koştu, yalnız kanonik kayıt katalogda yok — sözleşmeli
                // anahtar-fallback'i. Sistemik akıbet bu durumda da tanım
                // gereği geçmiştir; mandal burada da bırakılır ki eski
                // bayat-kök/katalog satırı ekranda asılı kalmasın.
                None => {
                    self.sistemik_mandalı_bırak();
                    None
                }
            },
        }
    }

    /// Yuvadaki **sistemik** akıbet mandalını bırakır; anahtar-yerel
    /// varyantlara dokunmaz. Sistemik akıbetler her anahtarı aynı biçimde
    /// vurduğundan, kanonik yoklamanın tutarlı bir zincir görmesi hepsinin
    /// geçtiğinin kanıtıdır.
    fn sistemik_mandalı_bırak(&self) {
        let mut yuva = self.hata_kaydı.borrow_mut();
        if yuva.as_ref().is_some_and(|kayıt| {
            matches!(
                kayıt.hata,
                TezgahÇözümHatası::KayıtlıZincirYok
                    | TezgahÇözümHatası::Çözüm(
                        İletiÇözümHatası::BayatYerelBağlam
                            | İletiÇözümHatası::BayatKatalog
                            | İletiÇözümHatası::KatalogDiliUyuşmuyor
                    )
            )
        }) {
            *yuva = None;
        }
    }

    /// `etkin_katalog`ın `None` dönüşünün exact ayrımı: iki nedeni hizmetin
    /// yaşayan köküyle karşılaştırarak geri kazanılır.
    fn katalogsuzluk(&self) -> TezgahÇözümHatası {
        if self.hizmet.yerel_kök() != self.yerel_kök.as_ref() {
            TezgahÇözümHatası::Çözüm(İletiÇözümHatası::BayatYerelBağlam)
        } else {
            TezgahÇözümHatası::KayıtlıZincirYok
        }
    }

    /// Verilen canlı snapshot ile tek anahtar çözümü.
    fn çöz_katalogla(
        &self,
        anahtar: &YerelleştirmeAnahtarı,
        katalog: Arc<gpui_bilesenleri_temel::İletiKataloğuSnapshot>,
    ) -> Result<gpui::SharedString, TezgahÇözümHatası> {
        let istek = İletiİsteği {
            anahtar: anahtar.clone(),
            argümanlar: Arc::from(Vec::new()),
        };
        self.hizmet
            .çöz(&istek, &self.yerel_kök, katalog)
            .map(|çözülen| gpui::SharedString::new(çözülen.metin().metin()))
            .map_err(TezgahÇözümHatası::Çözüm)
    }

    /// Canlı yolun typed sonucu; UI `çöz` bunun kayıpsız sunumudur.
    /// Önbelleğe hiç bakmaz — nöbet (`yokla`) bu yoldan koşar.
    pub(crate) fn çöz_sonuç(
        &self,
        anahtar: &YerelleştirmeAnahtarı,
    ) -> Result<gpui::SharedString, TezgahÇözümHatası> {
        match self.hizmet.etkin_katalog(&self.yerel_kök) {
            Some(katalog) => self.çöz_katalogla(anahtar, katalog),
            None => Err(self.katalogsuzluk()),
        }
    }

    pub(crate) fn çöz(&self, anahtar: &YerelleştirmeAnahtarı) -> gpui::SharedString {
        // Kararlı anahtarların sunumu damga-kuşaklı önbellekten gelir:
        // damga her çağrıda canlı kütükten okunur; kuşak tutuyorsa kabuk
        // ~20 anahtarı her karede yeniden yürütmez, tutmuyorsa (yeniden
        // mühürleme, katalog kaybı) önbellek boşalır ve çözüm canlı koşar.
        // Hata yolları önbelleğe girmez; önbellek hiçbir akıbeti
        // maskeleyemez.
        let sonuç = match self.hizmet.etkin_katalog(&self.yerel_kök) {
            Some(katalog) => {
                let damga = *katalog.damga();
                {
                    let mut önbellek = self.önbellek.borrow_mut();
                    if önbellek.kuşak != Some(damga) {
                        önbellek.kuşak = Some(damga);
                        önbellek.metinler.clear();
                    } else if let Some(metin) = önbellek.metinler.get(anahtar) {
                        return metin.clone();
                    }
                }
                let sonuç = self.çöz_katalogla(anahtar, katalog);
                if let Ok(metin) = &sonuç {
                    self.önbellek
                        .borrow_mut()
                        .metinler
                        .insert(anahtar.clone(), metin.clone());
                }
                sonuç
            }
            None => Err(self.katalogsuzluk()),
        };
        match sonuç {
            Ok(metin) => metin,
            // Eksik katalog kaydının sözleşmesi: anahtarın kendisi görünür.
            Err(TezgahÇözümHatası::Çözüm(İletiÇözümHatası::İstek(
                gpui_bilesenleri_temel::İletiİsteğiHatası::EksikAnahtar,
            ))) => gpui::SharedString::new(anahtar.as_ref()),
            // Diğer akıbetler eksik-kayıtla **karıştırılamaz**: typed sonuç
            // dizeye inmeden ÖNCE kökün yaşayan yuvasına yazılır (payload
            // UI durumunda korunur), sonra exact varyant ekranda işaretle
            // görünür.
            Err(hata) => {
                self.kaydet(anahtar, hata);
                gpui::SharedString::new(format!("‹{hata:?}› {}", anahtar.as_ref()))
            }
        }
    }
}

/// Sunum önbelleği: yaşayan katalog damgasıyla kuşaklanmış başarılı
/// çözümler. Damga tutmuyorsa içerik geçersizdir.
#[derive(Default)]
pub(crate) struct ÇözümÖnbelleği {
    kuşak: Option<gpui_bilesenleri_temel::İletiKataloğuDamgası>,
    metinler: BTreeMap<YerelleştirmeAnahtarı, gpui::SharedString>,
}

/// Tanı satırlarının exact sunum metinleri.
///
/// Metin üretimi tek yerde durur: çizen kartlar ve testler aynı üreticiyi
/// kullanır, "çizilen metin" ile "sınanan metin" ayrışamaz.
pub(crate) fn son_çözüm_hatası_metni(kayıt: &TezgahÇözümKaydı) -> String {
    format!(
        "Son ileti çözüm hatası · ‹{:?}› {}",
        kayıt.hata,
        kayıt.anahtar.as_ref()
    )
}

pub(crate) fn yerel_kök_hatası_metni(hata: YerelKökHatası) -> String {
    format!("‹{hata:?}› yerel kök yenilenemedi; eski bağlam yürürlükte")
}

#[cfg(test)]
mod testler {
    use super::*;
    use gpui_bilesenleri_temel::{
        SahteLocaleKipi, ÇözülmüşYazıYönü, İletiÇözümHatası
    };

    fn fabrika() -> ÖrnekKimliğiFabrikası {
        ÖrnekKimliğiFabrikası::yeni_süreç_kapsamı().expect("test kimlik kapsamı")
    }

    /// Galeri kökünü render etmeyen boş pencere konağı.
    struct BoşKonak;

    impl gpui::Render for BoşKonak {
        fn render(
            &mut self,
            _: &mut gpui::Window,
            _: &mut gpui::Context<Self>,
        ) -> impl gpui::IntoElement {
            gpui::div()
        }
    }

    fn istek(anahtar: &str) -> İletiİsteği {
        İletiİsteği {
            anahtar: YerelleştirmeAnahtarı::yeni(anahtar).expect("test anahtarı geçerlidir"),
            argümanlar: Arc::from(Vec::new()),
        }
    }

    /// Testlerin typed-akıbet yuvası.
    fn yuva() -> std::rc::Rc<std::cell::RefCell<Option<TezgahÇözümKaydı>>> {
        std::rc::Rc::new(std::cell::RefCell::new(None))
    }

    /// Kök `tr` sunum dilinde LTR çözer; yön elle yazılmaz, gerçek
    /// hizmetten gelir.
    #[test]
    fn turkce_kok_soldan_saga_cozulur() {
        let kök = MetinHizmetleriKökü::kur(&fabrika(), None);
        assert_eq!(kök.yerel_kök().yazı_yönü(), &ÇözülmüşYazıYönü::SoldanSağa);
        assert_eq!(kök.yerel_kök().dil().bcp47(), "tr");
    }

    /// RTL yerel gerçek fabrika yolundan `SağdanSola` çözülür.
    ///
    /// Kanıt seviyesi **hizmet düzeyidir**: yaşayan `GaleriUygulaması` kökü
    /// sunum dilini `tr` sabitler ve canlı bir RTL geçişi, yerel bağlamın
    /// kuruluş/çalışma-anında bileşene atomik enjekte edilebilmesini bekler
    /// (`blocked_by_missing_public_product_seam`, bkz.
    /// `GaleriUygulaması::yerel_kökü_eşitle`). Bu test canlı-host RTL
    /// kanıtı olarak sunulmaz.
    #[test]
    fn rtl_yerel_sagdan_sola_cozulur() {
        let unicode = UnicodeVeYerelMetinHizmetleri::yerlesik(fabrika());
        let motor = unicode.motor();
        let bağlam = unicode.yerel_bağlam_fabrikası().bağlam(
            CanlıBağlamDamgası {
                bağlam: fabrika().sonraki().expect("test kimliği"),
                sürüm: BağlamSürümü(1),
            },
            motor.dil_etiketi("ar").expect("`ar` kayıtlarda tanınır"),
            motor
                .numaralandırma_sistemi("latn")
                .expect("`latn` tanınır"),
            motor.takvim("gregory").expect("`gregory` tanınır"),
            motor.saat_dilimi("UTC").expect("`UTC` tanınır"),
        );
        assert_eq!(bağlam.yazı_yönü(), &ÇözülmüşYazıYönü::SağdanSola);
    }

    /// Yönü çözülemeyen etiket yalnız kanonik `SoldanSağa` sunum yedeğine
    /// düşer; bu bir dil yedeği değildir.
    #[test]
    fn cozulemeyen_yon_kanonik_sunum_yedegine_duser() {
        let unicode = UnicodeVeYerelMetinHizmetleri::yerlesik(fabrika());
        let motor = unicode.motor();
        let bağlam = unicode.yerel_bağlam_fabrikası().bağlam(
            CanlıBağlamDamgası {
                bağlam: fabrika().sonraki().expect("test kimliği"),
                sürüm: BağlamSürümü(1),
            },
            motor
                .dil_etiketi("und")
                .expect("`und` sözdizimsel olarak geçerlidir"),
            motor
                .numaralandırma_sistemi("latn")
                .expect("`latn` tanınır"),
            motor.takvim("gregory").expect("`gregory` tanınır"),
            motor.saat_dilimi("UTC").expect("`UTC` tanınır"),
        );
        assert_eq!(bağlam.yazı_yönü(), &ÇözülmüşYazıYönü::SoldanSağa);
    }

    /// Tezgâh anahtarı gerçek katalogdan, mühürlü hizmet üzerinden çözülür.
    #[test]
    fn tezgah_anahtari_gercek_katalogdan_cozulur() {
        let kök = MetinHizmetleriKökü::kur(&fabrika(), None);
        let çözücü = kök.çözücü(yuva());
        let başlık = çözücü.çöz(
            &YerelleştirmeAnahtarı::yeni("galeri.tezgah.başlık").expect("anahtar geçerlidir"),
        );
        assert_eq!(başlık.as_ref(), "Yapılandırma Tezgâhı");
        // Sözlükteki her kayıt katalogdan çözülür; hiçbiri anahtar
        // fallback'ine düşmez.
        for (anahtar, metin) in TEZGAH_KAYITLARI {
            let çözüm =
                çözücü.çöz(&YerelleştirmeAnahtarı::yeni(*anahtar).expect("anahtar geçerlidir"));
            assert_eq!(
                &çözüm.as_ref(),
                metin,
                "katalog kaydı çözülemedi: {anahtar}"
            );
        }
    }

    /// Eksik ileti anahtarı exact mevcut hata kanalından döner:
    /// `İletiÇözümHatası::İstek(EksikAnahtar)`. UI yolunda anahtarın kendisi
    /// görünür.
    #[test]
    fn eksik_anahtar_exact_hata_kanalindan_doner() {
        let kök = MetinHizmetleriKökü::kur(&fabrika(), None);
        let hizmet = kök.ileti_hizmeti();
        let yerel = kök.yerel_kök();
        let katalog = hizmet
            .etkin_katalog(&yerel)
            .expect("tezgâh kataloğu kayıtlı");
        let hata = hizmet
            .çöz(&istek("galeri.tezgah.bölüm.olmayan_kayit"), &yerel, katalog)
            .expect_err("kayıtsız anahtar çözülmemeli");
        assert_eq!(
            hata,
            İletiÇözümHatası::İstek(gpui_bilesenleri_temel::İletiİsteğiHatası::EksikAnahtar)
        );
        // UI yolu: eksik kayıt ekranda anahtarın kendisiyle görünür — typed
        // hata işareti taşımaz ve yuvaya kayıt düşmez; iki fallback
        // birbirine karışmaz.
        let kayıt_yuvası = yuva();
        let çözüm = kök.çözücü(std::rc::Rc::clone(&kayıt_yuvası)).çöz(
            &YerelleştirmeAnahtarı::yeni("galeri.tezgah.bölüm.olmayan_kayit")
                .expect("anahtar geçerlidir"),
        );
        assert!(!çözüm.contains('‹'), "eksik kayıt işaret taşımaz: {çözüm}");
        assert!(
            kayıt_yuvası.borrow().is_none(),
            "eksik kayıt sözleşmeli fallback'tir; yuvaya hata yazılmaz"
        );
        assert_eq!(çözüm.as_ref(), "galeri.tezgah.bölüm.olmayan_kayit");
    }

    /// Kanonik anahtarın `EksikAnahtar` yoklaması da eski **sistemik**
    /// mandalı bırakır: zincir (kök + katalog + dil) tutarlı koştuysa
    /// sistemik akıbet geçmiştir; kanonik kaydın katalogdan çıkması bayat
    /// satırı ekranda asılı bırakamaz. Anahtar-yerel "son hata" kaydına
    /// dokunulmaz.
    #[test]
    fn eksik_kanonik_anahtar_sistemik_mandali_birakir() {
        let fabrika = fabrika();
        let unicode = UnicodeVeYerelMetinHizmetleri::yerlesik(
            ÖrnekKimliğiFabrikası::yeni_süreç_kapsamı().expect("test kimlik kapsamı"),
        );
        let yerel_kök = Arc::new(MetinHizmetleriKökü::yerel_kök_üret(
            &unicode,
            fabrika.sonraki().expect("test kimliği"),
            1,
            None,
        ));
        let kütük = İletiKataloğuKütüğü::muhurle(
            fabrika.sonraki().expect("test kimliği"),
            İletiBütçesi::default(),
        );
        // Kanonik başlık kaydı **olmayan** gerçek katalog.
        let mut paket = tezgah_kataloğu(yerel_kök.dil().clone());
        assert!(
            paket
                .şablonlar
                .remove(&crate::anahtar("galeri.tezgah.başlık"))
                .is_some(),
            "kanonik kayıt sözlükte olmalı ki eksikliği kurulabilsin"
        );
        let _kayıt = kütük
            .kaydet(paket)
            .expect("kanonik kayıtsız katalog da kaydedilir");
        let kayıt_yuvası = yuva();
        let çözücü = TezgahİletiÇözücüsü {
            hizmet: Arc::new(İletiÇözümHizmeti::muhurle(
                kütük,
                unicode.motor(),
                Arc::clone(&yerel_kök),
            )),
            yerel_kök,
            hata_kaydı: std::rc::Rc::clone(&kayıt_yuvası),
            önbellek: std::rc::Rc::new(std::cell::RefCell::new(ÇözümÖnbelleği::default())),
        };
        // Eski bir sistemik akıbet yuvada mandallı dursun.
        kayıt_yuvası.borrow_mut().replace(TezgahÇözümKaydı {
            anahtar: crate::anahtar("galeri.tezgah.başlık"),
            hata: TezgahÇözümHatası::Çözüm(İletiÇözümHatası::BayatYerelBağlam),
        });
        // Yoklama `EksikAnahtar` görür: kayıt döndürmez **ve** mandalı
        // bırakır — bayat satır sönmeli.
        assert!(çözücü.yokla().is_none());
        assert!(
            kayıt_yuvası.borrow().is_none(),
            "sistemik mandal `EksikAnahtar` yoklamasında da bırakılmalı"
        );
        // Anahtar-yerel "son hata" kaydı yoklamayla silinmez.
        kayıt_yuvası.borrow_mut().replace(TezgahÇözümKaydı {
            anahtar: crate::anahtar("galeri.tezgah.önizleme"),
            hata: TezgahÇözümHatası::Çözüm(İletiÇözümHatası::ÇoğulKoluEksik),
        });
        assert!(çözücü.yokla().is_none());
        assert!(
            kayıt_yuvası.borrow().is_some(),
            "anahtar-yerel kayıt sistemik mandalla birlikte silinmemeli"
        );
    }

    /// Yerel kök değişince hizmet tek atomda yeniden mühürlenir; eski
    /// bağlam yeni hizmette sessizce çözülmez, exact `BayatYerelBağlam`
    /// üretir.
    #[test]
    fn kok_degisince_yeni_hizmet_eski_baglam_bayat() {
        let fabrika = fabrika();
        let mut kök = MetinHizmetleriKökü::kur(&fabrika, None);
        let eski_hizmet = kök.ileti_hizmeti();
        let eski_bağlam = kök.yerel_kök();

        let istanbul = kök
            .unicode()
            .motor()
            .saat_dilimi("Europe/Istanbul")
            .expect("`Europe/Istanbul` kayıtlarda tanınır");
        let yeni_kök = kök
            .yerel_kökü_gerekirse_yenile(&fabrika, Some(&istanbul))
            .expect("sürüm ekseni taze; yenileme reddedilmez")
            .expect("saat dilimi değişimi kökü yenilemeli");
        // Aynı hedefle ikinci çağrı yenileme üretmez.
        assert!(
            kök.yerel_kökü_gerekirse_yenile(&fabrika, Some(&istanbul))
                .expect("değişimsiz çağrı da reddedilmez")
                .is_none()
        );

        let yeni_hizmet = kök.ileti_hizmeti();
        assert!(
            !Arc::ptr_eq(&eski_hizmet, &yeni_hizmet),
            "kök değişiminde hizmet yeniden mühürlenmeli"
        );
        // Yeni hizmet yeni kökle çözer.
        let katalog = yeni_hizmet
            .etkin_katalog(&yeni_kök)
            .expect("yeni kökün kataloğu kayıtlı");
        assert!(
            yeni_hizmet
                .çöz(&istek("galeri.tezgah.başlık"), &yeni_kök, katalog)
                .is_ok()
        );
        // Eski bağlam yeni hizmette exact bayattır.
        assert!(yeni_hizmet.etkin_katalog(&eski_bağlam).is_none());
        let katalog = yeni_hizmet
            .etkin_katalog(&yeni_kök)
            .expect("yeni kökün kataloğu kayıtlı");
        assert_eq!(
            yeni_hizmet
                .çöz(&istek("galeri.tezgah.başlık"), &eski_bağlam, katalog)
                .expect_err("eski bağlam sessizce çözülmemeli"),
            İletiÇözümHatası::BayatYerelBağlam
        );
    }

    /// Katalog değişince (kayıt düşünce) eski snapshot exact `BayatKatalog`
    /// akıbetine düşer; sessiz eski-katalog çözümü yoktur.
    #[test]
    fn katalog_degisince_eski_snapshot_bayat_katalog() {
        let fabrika = fabrika();
        let unicode = UnicodeVeYerelMetinHizmetleri::yerlesik(
            ÖrnekKimliğiFabrikası::yeni_süreç_kapsamı().expect("test kimlik kapsamı"),
        );
        let yerel_kök = Arc::new(MetinHizmetleriKökü::yerel_kök_üret(
            &unicode,
            fabrika.sonraki().expect("test kimliği"),
            1,
            None,
        ));
        let kütük = İletiKataloğuKütüğü::muhurle(
            fabrika.sonraki().expect("test kimliği"),
            İletiBütçesi::default(),
        );
        let kayıt = kütük
            .kaydet(tezgah_kataloğu(yerel_kök.dil().clone()))
            .expect("test kataloğu kaydedilir");
        let hizmet = İletiÇözümHizmeti::muhurle(kütük, unicode.motor(), Arc::clone(&yerel_kök));

        let eski_snapshot = hizmet
            .etkin_katalog(&yerel_kök)
            .expect("katalog kayıtlıyken snapshot alınır");
        // Kayıt belirteci düşer: kütük sürümü ilerler, eski snapshot bayat.
        drop(kayıt);
        assert_eq!(
            hizmet
                .çöz(&istek("galeri.tezgah.başlık"), &yerel_kök, eski_snapshot)
                .expect_err("bayat snapshot çözülmemeli"),
            İletiÇözümHatası::BayatKatalog
        );
    }

    /// `RtlAynala` yalnız sonuç yönünü çevirir; `MetniUzat` ve `AksanEkle`
    /// gerçek bağlamın çözülmüş yönünü korur. Sahte bir `YerelMetinBağlamı`
    /// üretilmez — sonuç `SahteLocaleSonucu` erişicilerinden okunur.
    #[test]
    fn sahte_locale_kipleri_yon_sozlesmesini_korur() {
        let kök = MetinHizmetleriKökü::kur(&fabrika(), None);
        let hizmet = kök.ileti_hizmeti();
        let yerel = kök.yerel_kök();
        let katalog = || {
            hizmet
                .etkin_katalog(&yerel)
                .expect("tezgâh kataloğu kayıtlı")
        };

        let aynalı = hizmet
            .sahte_locale_çöz(
                &istek("galeri.tezgah.başlık"),
                &yerel,
                katalog(),
                SahteLocaleKipi::RtlAynala,
            )
            .expect("RtlAynala çözülür");
        assert_eq!(aynalı.yazı_yönü(), &ÇözülmüşYazıYönü::SağdanSola);

        let uzatılmış = hizmet
            .sahte_locale_çöz(
                &istek("galeri.tezgah.başlık"),
                &yerel,
                katalog(),
                SahteLocaleKipi::MetniUzat { oran: 1.5 },
            )
            .expect("MetniUzat çözülür");
        assert_eq!(uzatılmış.yazı_yönü(), yerel.yazı_yönü());

        let aksanlı = hizmet
            .sahte_locale_çöz(
                &istek("galeri.tezgah.başlık"),
                &yerel,
                katalog(),
                SahteLocaleKipi::AksanEkle,
            )
            .expect("AksanEkle çözülür");
        assert_eq!(aksanlı.yazı_yönü(), yerel.yazı_yönü());
    }

    /// Desktop girişi (`GaleriUygulaması::yeni`) ve WASM girişi (`::wasm`)
    /// aynı `hedef` bileşim kökünden geçer: ikisi de kökte tek Unicode
    /// hizmet kökü ve tek ileti çözüm hizmeti kurar, katalog gerçek
    /// hizmetten çözülür.
    #[test]
    fn desktop_ve_wasm_kokleri_ayni_sahiplik_modelini_kurar() {
        let başlık_anahtarı =
            YerelleştirmeAnahtarı::yeni("galeri.tezgah.başlık").expect("anahtar geçerlidir");
        for uygulama in [
            crate::GaleriUygulaması::yeni(),
            crate::GaleriUygulaması::wasm(),
        ] {
            assert_eq!(
                uygulama.tezgah_çözücüsü().çöz(&başlık_anahtarı).as_ref(),
                "Yapılandırma Tezgâhı"
            );
            let kök = uygulama.metin_hizmetleri.yerel_kök();
            assert_eq!(kök.dil().bcp47(), "tr");
            assert_eq!(
                kök.yazı_yönü(),
                &gpui_bilesenleri_temel::ÇözülmüşYazıYönü::SoldanSağa
            );
        }
    }

    /// İki `GirişKutusu` (tezgâh alanı + sergi alanı) aynı host capability
    /// kökünü tüketir; paralel otorite yoktur. Kuruluş eksenleri (rapor ve
    /// varsayılan sağlayıcı akıbeti) kayıpsız taşınır.
    #[gpui::test]
    fn iki_giris_kutusu_ayni_host_kokunu_tuketir(bağlam: &mut gpui::TestAppContext) {
        bağlam.update(crate::bileşen_tuş_bağlarını_kur);
        let (uygulama, görsel) = bağlam.add_window_view(|_, _| crate::GaleriUygulaması::yeni());
        görsel.update(|pencere, bağlam| {
            uygulama.update(bağlam, |uygulama, bağlam| {
                let tezgah_alanı = uygulama
                    .tezgah_alanını_al(pencere, bağlam)
                    .expect("tezgâh alanı varsayılan tercihle kurulur");
                let alanlar = uygulama
                    .sergi_girişlerini_al(pencere, bağlam)
                    .expect("sergi alanları kurulur");
                let kök = uygulama.metin_hizmetleri.yerel_kök();
                assert_eq!(&tezgah_alanı.read(bağlam).yerel, kök.as_ref());
                assert_eq!(&alanlar.yalın.read(bağlam).yerel, kök.as_ref());
                assert_eq!(&alanlar.tutar.read(bağlam).yerel, kök.as_ref());
                // `GirişKuruluşSonucu` eksenleri kayıpsız: uyarı raporu ve
                // sağlayıcı akıbeti galeri durumunda durur.
                assert!(uygulama.tezgah_kuruluş_raporu.is_some());
                assert!(uygulama.tezgah_varsayılan_değer_hatası.is_none());
                assert_eq!(alanlar.kuruluş_notları.len(), 10);
            });
        });
    }

    /// Saat dilimi tercihi değişince yerel kök ve `ORT-021` hizmeti tek
    /// atomda yenilenir; yaşayan alanlar yeni kökü tüketir, eski bağlam
    /// yeni hizmette exact `BayatYerelBağlam` üretir.
    #[gpui::test]
    fn dilim_degisimi_koku_hizmetiyle_birlikte_yeniler(bağlam: &mut gpui::TestAppContext) {
        bağlam.update(crate::bileşen_tuş_bağlarını_kur);
        let (uygulama, görsel) = bağlam.add_window_view(|_, _| crate::GaleriUygulaması::yeni());
        görsel.update(|pencere, bağlam| {
            uygulama.update(bağlam, |uygulama, bağlam| {
                let alan = uygulama
                    .tezgah_alanını_al(pencere, bağlam)
                    .expect("tezgâh alanı kurulur");
                let alanlar = uygulama
                    .sergi_girişlerini_al(pencere, bağlam)
                    .expect("sergi alanları kurulur");
                let eski_kök = uygulama.metin_hizmetleri.yerel_kök();
                let eski_hizmet = uygulama.metin_hizmetleri.ileti_hizmeti();
                let istanbul = uygulama
                    .metin_hizmetleri
                    .motor()
                    .saat_dilimi("Europe/Istanbul")
                    .expect("`Europe/Istanbul` tanınır");
                let seçim = istanbul.clone();
                uygulama.tezgahı_değiştir(
                    move |t| t.saat_dilimi_tercihi = crate::SaatDilimiTercihi::Kullanıcı(seçim),
                    bağlam,
                );
                let yeni_kök = uygulama.metin_hizmetleri.yerel_kök();
                assert_eq!(yeni_kök.sunum_saat_dilimi(), &istanbul);
                assert!(yeni_kök.damga().sürüm().0 > eski_kök.damga().sürüm().0);
                // Yaşayan alanlar yeni kökü tüketir; eski bağlam bırakıldı.
                assert_eq!(&alan.read(bağlam).yerel, yeni_kök.as_ref());
                assert_eq!(&alanlar.yalın.read(bağlam).yerel, yeni_kök.as_ref());
                // Hizmet yeniden mühürlendi; eski bağlam exact bayat.
                let yeni_hizmet = uygulama.metin_hizmetleri.ileti_hizmeti();
                assert!(!Arc::ptr_eq(&eski_hizmet, &yeni_hizmet));
                let katalog = yeni_hizmet
                    .etkin_katalog(&yeni_kök)
                    .expect("yeni kökün kataloğu kayıtlı");
                assert_eq!(
                    yeni_hizmet
                        .çöz(&istek("galeri.tezgah.başlık"), &eski_kök, katalog)
                        .expect_err("eski bağlam sessizce çözülmemeli"),
                    İletiÇözümHatası::BayatYerelBağlam
                );
            });
        });
    }

    /// Yaşayan IME birleşimi sırasında seçilen tercih **kaybolmaz**: `§30`
    /// dış-yazım reddi hedefi bekleyen kayda düşürür, birleşimin metin
    /// olayları tercihi geri ezmez ve birleşim bitince ileri eşitleme
    /// kazanır.
    #[gpui::test]
    fn birlesim_sirasinda_secilen_tercih_kaybolmaz(bağlam: &mut gpui::TestAppContext) {
        bağlam.update(crate::bileşen_tuş_bağlarını_kur);
        let (uygulama, görsel) = bağlam.add_window_view(|_, _| crate::GaleriUygulaması::yeni());
        let desen_kutusu = görsel.update(|pencere, bağlam| {
            uygulama.update(bağlam, |uygulama, bağlam| {
                uygulama
                    .sergi_girişlerini_al(pencere, bağlam)
                    .expect("sergi alanları kurulur")
                    .desen
                    .clone()
            })
        });
        // Birleşim kanonik giriş yolundan başlar.
        görsel.update(|pencere, bağlam| {
            desen_kutusu.update(bağlam, |kutu, bağlam| {
                gpui::EntityInputHandler::replace_and_mark_text_in_range(
                    kutu, None, "か", None, pencere, bağlam,
                );
            });
        });
        görsel.run_until_parked();
        // Birleşim etkinken kullanıcı hazır bir desen şablonu seçer.
        let hedef = crate::HAZIR_DESENLER[1].1.to_owned();
        görsel.update(|_, bağlam| {
            uygulama.update(bağlam, |uygulama, bağlam| {
                let seçim = hedef.clone();
                uygulama.tezgahı_değiştir(move |t| t.desen = seçim, bağlam);
                // Ret turu atlattı ama hedef bekleyen kayda düştü; birleşim
                // korunur, tercih seçilen hedefi taşır.
                assert!(
                    uygulama
                        .bekleyen_tercih_eşitlemeleri
                        .contains(&desen_kutusu.entity_id()),
                    "reddedilen ileri eşitleme bekleyen kayda düşmeli"
                );
                assert_eq!(uygulama.tezgah.desen, hedef);
                assert_ne!(
                    desen_kutusu.read(bağlam).metin(),
                    hedef.as_str(),
                    "birleşim dış yazımla bozulmamalı"
                );
            });
        });
        // Birleşim sürerken gelen metin olayı tercihi geri ezmez.
        görsel.update(|pencere, bağlam| {
            desen_kutusu.update(bağlam, |kutu, bağlam| {
                gpui::EntityInputHandler::replace_and_mark_text_in_range(
                    kutu, None, "かん", None, pencere, bağlam,
                );
            });
        });
        görsel.run_until_parked();
        görsel.update(|_, bağlam| {
            uygulama.update(bağlam, |uygulama, _| {
                assert_eq!(
                    uygulama.tezgah.desen, hedef,
                    "birleşim metni seçilen tercihi ezmemeli"
                );
            });
        });
        // Birleşim kanonik `unmark_text` atomuyla biter (kompozisyon değeri
        // yalnız orada düşer); ileri eşitleme şimdi uygulanır.
        görsel.update(|pencere, bağlam| {
            desen_kutusu.update(bağlam, |kutu, bağlam| {
                gpui::EntityInputHandler::unmark_text(kutu, pencere, bağlam);
            });
        });
        görsel.run_until_parked();
        görsel.update(|_, bağlam| {
            uygulama.update(bağlam, |uygulama, bağlam| {
                assert_eq!(
                    uygulama.tezgah.desen, hedef,
                    "birleşim bitişi tercihi ezmez; seçilen hedef kazanır"
                );
                assert_eq!(
                    desen_kutusu.read(bağlam).metin(),
                    hedef.as_str(),
                    "kutu bekleyen hedefe eşitlenmeli"
                );
                assert!(
                    uygulama.bekleyen_tercih_eşitlemeleri.is_empty(),
                    "uygulanan hedefin bekleyen kaydı düşmeli"
                );
            });
        });
    }

    /// Asılı kompozisyon ekseni bekleyen kaydı **kalıcılaştıramaz**:
    /// birleşim, işaret kaldırılmadan `replace_text_in_range` ile
    /// kesinleşirse bileşenin kompozisyon-değeri ekseni asılı kalır
    /// (`composition_iptal` yalnız `unmark_text`/iptal yollarında koşar) ve
    /// `CompositionEtkin` artık geçici değildir. Bekleyen kayıt bu retle
    /// **silinir** (dolu kümeden kalıcı rete gerçek geçiş), akıbet exact
    /// typed görünür, kutunun metin olayları bastırılmadan akmaya devam
    /// eder ve eksen gerçek `unmark_text` yoluyla iyileşir.
    ///
    /// Senaryo baştan sona gerçek platform giriş noktalarıyla kurulur
    /// (`replace_and_mark_text_in_range` + `replace_text_in_range` +
    /// `unmark_text`); private durum kurcalanmaz. Kök neden kardeştedir ve
    /// yalnız raporlanır: `replace_text` commit kolu `ime_aralığı`nı
    /// düşürürken kompozisyon-değeri eksenini düşürmez.
    #[gpui::test]
    fn asili_kompozisyon_ekseni_bekleyeni_kalicilastiramaz(bağlam: &mut gpui::TestAppContext) {
        bağlam.update(crate::bileşen_tuş_bağlarını_kur);
        let (uygulama, görsel) = bağlam.add_window_view(|_, _| crate::GaleriUygulaması::yeni());
        görsel.run_until_parked();
        let desen_kutusu = görsel.update(|pencere, bağlam| {
            uygulama.update(bağlam, |uygulama, bağlam| {
                uygulama
                    .sergi_girişlerini_al(pencere, bağlam)
                    .expect("sergi alanları kurulur")
                    .desen
                    .clone()
            })
        });
        // Canlı birleşim + tercih seçimi: ret geçici, hedef bekleyen kayda
        // düşer (küme gerçekten dolu).
        görsel.update(|pencere, bağlam| {
            desen_kutusu.update(bağlam, |kutu, bağlam| {
                gpui::EntityInputHandler::replace_and_mark_text_in_range(
                    kutu, None, "か", None, pencere, bağlam,
                );
            });
        });
        görsel.run_until_parked();
        let hedef = crate::HAZIR_DESENLER[1].1.to_owned();
        görsel.update(|_, bağlam| {
            uygulama.update(bağlam, |uygulama, bağlam| {
                let seçim = hedef.clone();
                uygulama.tezgahı_değiştir(move |t| t.desen = seçim, bağlam);
                assert!(
                    uygulama
                        .bekleyen_tercih_eşitlemeleri
                        .contains(&desen_kutusu.entity_id()),
                    "canlı birleşimde ret geçicidir ve bekleyen kayda düşer"
                );
            });
        });
        // Birleşim işaret kaldırılmadan kesinleşir: işaret aralığı düşer,
        // kompozisyon-değeri ekseni asılı kalır.
        görsel.update(|pencere, bağlam| {
            desen_kutusu.update(bağlam, |kutu, bağlam| {
                gpui::EntityInputHandler::replace_text_in_range(
                    kutu,
                    None,
                    "かんじ",
                    pencere,
                    bağlam,
                );
            });
        });
        görsel.run_until_parked();
        // Olayın yeniden denemesi asılı ekseni **kalıcı** sınıflar: dolu
        // bekleyen kayıt silinir, akıbet exact typed gözlenir ve boyanır.
        görsel.update(|_, bağlam| {
            uygulama.update(bağlam, |uygulama, _| {
                assert!(
                    uygulama.bekleyen_tercih_eşitlemeleri.is_empty(),
                    "asılı eksen bekleyen kaydı kalıcılaştıramaz; kalıcı ret kaydı siler"
                );
                let kayıt = uygulama
                    .tercih_eşitleme_hatası()
                    .expect("asılı eksenin akıbeti exact typed gözlenmeli");
                assert_eq!(kayıt.kutu, desen_kutusu.entity_id());
                assert_eq!(kayıt.hata, gpui_bilesenleri::GirişHatası::CompositionEtkin);
            });
        });
        görsel.run_until_parked();
        assert!(
            görsel.debug_bounds("tanı-tercih-eşitleme-hatası").is_some(),
            "asılı eksenin tanı satırı gerçekten boyanmalı"
        );
        // Kutunun metin olayları bastırılmaz: sıradan bir düzenleme ters
        // yönden tercihe akar ve uyum kurulunca kalıcı ret kaydı düşer.
        görsel.update(|pencere, bağlam| {
            desen_kutusu.update(bağlam, |kutu, bağlam| {
                gpui::EntityInputHandler::replace_text_in_range(kutu, None, "5", pencere, bağlam);
            });
        });
        görsel.run_until_parked();
        let kutu_metni = görsel.update(|_, bağlam| desen_kutusu.read(bağlam).metin().to_owned());
        görsel.update(|_, bağlam| {
            uygulama.update(bağlam, |uygulama, _| {
                assert_eq!(
                    uygulama.tezgah.desen, kutu_metni,
                    "asılı eksende metin olayları bastırılmadan tercihe akmalı"
                );
                assert!(
                    uygulama.tercih_eşitleme_hatası().is_none(),
                    "uyum kurulunca kalıcı ret kaydı düşmeli"
                );
            });
        });
        görsel.run_until_parked();
        assert!(
            görsel.debug_bounds("tanı-tercih-eşitleme-hatası").is_none(),
            "iyileşen karede tanı satırı sönmeli"
        );
        // Eksen gerçek yoldan iyileşir: `unmark_text` kompozisyon değerini
        // düşürür ve ileri eşitleme yeniden uygulanabilir olur.
        görsel.update(|pencere, bağlam| {
            desen_kutusu.update(bağlam, |kutu, bağlam| {
                gpui::EntityInputHandler::unmark_text(kutu, pencere, bağlam);
            });
        });
        görsel.run_until_parked();
        let hedef2 = crate::HAZIR_DESENLER[2].1.to_owned();
        görsel.update(|_, bağlam| {
            uygulama.update(bağlam, |uygulama, bağlam| {
                let seçim = hedef2.clone();
                uygulama.tezgahı_değiştir(move |t| t.desen = seçim, bağlam);
                assert_eq!(
                    desen_kutusu.read(bağlam).metin(),
                    hedef2.as_str(),
                    "iyileşen eksende ileri eşitleme yeniden uygulanır"
                );
                assert!(uygulama.bekleyen_tercih_eşitlemeleri.is_empty());
                assert!(uygulama.tercih_eşitleme_hatası().is_none());
            });
        });
    }

    /// `CompositionEtkin` dışındaki eşitleme retleri **bekleyen kayda
    /// dönüşmez**: terminal `SürümTükendi` sonrasında kutunun metin
    /// olayları bastırılmaz, akıbet exact typed gözlenir ve tanı satırı
    /// gerçekten boyanır; uyum yeniden kurulunca kayıt düşer, satır söner.
    ///
    /// Kanıt seviyesi dürüstçe: tükenme önkoşulu (`değer_sürümü =
    /// u64::MAX`) pub durum alanından enjekte edilir — üretim süresinde
    /// erişilmez; sınanan şey tükenmenin üretilebilirliği değil, kalıcı
    /// retin bekleyen/typed/görünürlük ayrımıdır. Ters-yön olayı da testin
    /// kendisi yayımlar: sınanan, kökün abonelik semantiğidir.
    #[gpui::test]
    fn kalici_esitleme_reti_bekleyene_donusmez_typed_gorunur(bağlam: &mut gpui::TestAppContext) {
        bağlam.update(crate::bileşen_tuş_bağlarını_kur);
        let (uygulama, görsel) = bağlam.add_window_view(|_, _| crate::GaleriUygulaması::yeni());
        görsel.run_until_parked();
        let desen_kutusu = görsel.update(|pencere, bağlam| {
            uygulama.update(bağlam, |uygulama, bağlam| {
                uygulama
                    .sergi_girişlerini_al(pencere, bağlam)
                    .expect("sergi alanları kurulur")
                    .desen
                    .clone()
            })
        });
        assert!(
            görsel.debug_bounds("tanı-tercih-eşitleme-hatası").is_none(),
            "sağlıklı karede tanı satırı çizilmemeli"
        );
        // Terminal önkoşul: kutunun metin/IME sürüm ekseni tükenmiş.
        görsel.update(|_, bağlam| {
            desen_kutusu.update(bağlam, |kutu, _| {
                kutu.durum.değer_sürümü = u64::MAX;
            });
        });
        let hedef = crate::HAZIR_DESENLER[1].1.to_owned();
        görsel.update(|_, bağlam| {
            uygulama.update(bağlam, |uygulama, bağlam| {
                let seçim = hedef.clone();
                uygulama.tezgahı_değiştir(move |t| t.desen = seçim, bağlam);
                assert!(
                    uygulama.bekleyen_tercih_eşitlemeleri.is_empty(),
                    "kalıcı ret bekleyen kayda dönüşmemeli"
                );
                let kayıt = uygulama
                    .tercih_eşitleme_hatası()
                    .expect("kalıcı ret exact typed gözlenmeli");
                assert_eq!(kayıt.kutu, desen_kutusu.entity_id());
                assert_eq!(
                    kayıt.hata,
                    gpui_bilesenleri::GirişHatası::SürümTükendi(
                        gpui_bilesenleri::GirişSürümEkseni::MetinVeIme
                    )
                );
                assert_eq!(
                    uygulama.tezgah.desen, hedef,
                    "tercih seçilen hedefi taşımaya devam eder"
                );
            });
        });
        görsel.run_until_parked();
        assert!(
            görsel.debug_bounds("tanı-tercih-eşitleme-hatası").is_some(),
            "kalıcı retin tanı satırı gerçekten boyanmalı"
        );
        // Kutunun metin olayları bastırılmaz: ters yön (kutu → tercih)
        // akmaya devam eder ve uyum kurulunca kalıcı ret kaydı düşer.
        let kutu_metni = görsel.update(|_, bağlam| desen_kutusu.read(bağlam).metin().to_owned());
        assert_ne!(kutu_metni, hedef, "tükenmiş kutu hedefe eşitlenmiş olamaz");
        görsel.update(|_, bağlam| {
            desen_kutusu.update(bağlam, |_, bağlam| {
                bağlam.emit(gpui_bilesenleri::GirişOlayı::DüzenlemeMetniDeğişti {
                    metin: String::new(),
                    değer_sürümü: 0,
                });
            });
        });
        görsel.run_until_parked();
        görsel.update(|_, bağlam| {
            uygulama.update(bağlam, |uygulama, _| {
                assert_eq!(
                    uygulama.tezgah.desen, kutu_metni,
                    "metin olayı bastırılmadan tercihe akmalı"
                );
                assert!(
                    uygulama.tercih_eşitleme_hatası().is_none(),
                    "uyum kurulunca kalıcı ret kaydı düşmeli"
                );
            });
        });
        görsel.run_until_parked();
        assert!(
            görsel.debug_bounds("tanı-tercih-eşitleme-hatası").is_none(),
            "iyileşen karede tanı satırı sönmeli"
        );
    }

    /// Kartın dilim okuma yolu kökü **birlikte** eşitler: platform
    /// bildirimi kök olayları arasında değişse bile (masaüstü portu tazelik
    /// penceresi dolunca kendiliğinden yeniler) kart yeni dilimi
    /// gösterirken `ORT-021` kökü eskide kalamaz.
    #[gpui::test]
    fn kart_dilim_okumasi_koku_birlikte_esitler(bağlam: &mut gpui::TestAppContext) {
        use gpui_bilesenleri::{GmtFarkı, SaatDilimiKaynağı, ÇözülmüşSaatDilimi};

        struct DeğişkenDilimPortu(std::sync::Mutex<Option<ÇözülmüşSaatDilimi>>);
        impl gpui_bilesenleri::PlatformSaatDilimiPortu for DeğişkenDilimPortu {
            fn dilim(&self) -> Option<ÇözülmüşSaatDilimi> {
                self.0.lock().expect("test kilidi zehirlenmez").clone()
            }
        }

        bağlam.update(crate::bileşen_tuş_bağlarını_kur);
        let (uygulama, görsel) = bağlam.add_window_view(|_, _| crate::GaleriUygulaması::yeni());
        görsel.update(|pencere, bağlam| {
            uygulama.update(bağlam, |uygulama, bağlam| {
                let motor = uygulama.metin_hizmetleri.motor();
                let istanbul = motor
                    .saat_dilimi("Europe/Istanbul")
                    .expect("`Europe/Istanbul` tanınır");
                let londra = motor
                    .saat_dilimi("Europe/London")
                    .expect("`Europe/London` tanınır");
                let bildirim = |kimlik: &_| ÇözülmüşSaatDilimi {
                    kimlik: Some(std::clone::Clone::clone(kimlik)),
                    gmt_farkı: GmtFarkı::UTC,
                    kaynak: SaatDilimiKaynağı::Platform,
                };
                let port = Arc::new(DeğişkenDilimPortu(std::sync::Mutex::new(Some(bildirim(
                    &istanbul,
                )))));
                uygulama.tezgahı_değiştir(
                    |t| t.saat_dilimi_tercihi = crate::SaatDilimiTercihi::Platform,
                    bağlam,
                );
                uygulama.platform_portlarını_kur(
                    crate::PlatformPortları {
                        saat_dilimi: Some(Arc::clone(&port) as _),
                        ..Default::default()
                    },
                    bağlam,
                );
                assert_eq!(
                    uygulama.metin_hizmetleri.yerel_kök().sunum_saat_dilimi(),
                    &istanbul
                );
                // Bildirim kök olayları olmadan değişir.
                *port.0.lock().expect("test kilidi zehirlenmez") = Some(bildirim(&londra));
                // Kartın okuma yolu tek başına yeter: bölümler yeni dilimi
                // gösterirken kök de aynı karede yeni dilime taşınmıştır.
                let bölümler = uygulama.tezgah_bölümleri(pencere, bağlam);
                assert!(!bölümler.is_empty(), "bölüm listesi kurulmalı");
                assert_eq!(
                    uygulama.çözülmüş_saat_dilimi().kimlik.as_ref(),
                    Some(&londra)
                );
                let yeni_kök = uygulama.metin_hizmetleri.yerel_kök();
                assert_eq!(
                    yeni_kök.sunum_saat_dilimi(),
                    &londra,
                    "kart yeni dilimi gösterirken kök eskide kalamaz"
                );
                // Yaşayan alanlar da aynı geçişte yeni kökü tüketir.
                let alanlar = uygulama
                    .sergi_girişleri
                    .clone()
                    .expect("bölüm okuması alanları kurdu");
                assert_eq!(&alanlar.yalın.read(bağlam).yerel, yeni_kök.as_ref());
            });
        });
    }

    /// `§29` **gerçek** kuruluş başarısızlığı yarım entity, panel veya
    /// abonelik bırakmaz.
    ///
    /// Hata elle enjekte edilmez: derlenemeyen bir maske deseni gerçek
    /// tercih yüzeyinden kurulur ve `kur` exact
    /// `Teknik { hata: GeçersizMaske }` ile reddeder. Önizleme typed sonucu
    /// çizer; desen düzeltilince aynı türde yeni deneme geçer.
    #[gpui::test]
    fn kurulus_hatasi_yarim_durum_birakmaz(bağlam: &mut gpui::TestAppContext) {
        bağlam.update(crate::bileşen_tuş_bağlarını_kur);
        // Kök render edilmez: ilk kuruluş denemesi bu testin elindedir.
        let (_konak, görsel) = bağlam.add_window_view(|_, _| BoşKonak);
        görsel.update(|pencere, bağlam| {
            let uygulama = gpui::AppContext::new(bağlam, |_| crate::GaleriUygulaması::yeni());
            uygulama.update(bağlam, |uygulama, bağlam| {
                // Gerçek geçersiz tercih: hedefsiz `\` kaçışı derlenemez.
                uygulama.tezgahı_değiştir(
                    |t| {
                        t.maske = crate::TezgahMaskesi::Desen;
                        t.desen = "\\".into();
                    },
                    bağlam,
                );
                assert!(uygulama.tezgah_alanını_al(pencere, bağlam).is_none());
                match uygulama.tezgah_kuruluş_hatası.as_ref() {
                    Some(gpui_bilesenleri::GirişKuruluşHatası::Teknik { hata, .. }) => {
                        assert!(
                            matches!(hata, gpui_bilesenleri::GirişHatası::GeçersizMaske),
                            "derlenemeyen desen exact `GeçersizMaske` taşımalı: {hata:?}"
                        );
                    }
                    diğer => {
                        panic!("derlenemeyen desen exact `Teknik` ret üretmeli: {diğer:?}")
                    }
                }
                // Yarım durum yok: entity, panel ve kuruluş eksenleri boş.
                assert!(uygulama.tezgah_alanı.is_none());
                assert!(uygulama.tezgah_panelleri.is_none());
                assert!(uygulama.tezgah_kuruluş_raporu.is_none());
                let içerik = uygulama.tezgah_profil_içeriği(pencere, bağlam);
                assert_eq!(içerik.önizleme.len(), 1, "yalnız hata kartı çizilir");
                assert!(içerik.sol_ek.is_empty());
                assert!(içerik.kod.is_none());
                // Aynı türde tercih düzeltmesi ekseni sıfırlar; yeni deneme
                // gerçek `kur` yolundan geçer.
                uygulama.tezgahı_değiştir(|t| t.maske = crate::TezgahMaskesi::Yok, bağlam);
                assert!(uygulama.tezgah_kuruluş_hatası.is_none());
                assert!(uygulama.tezgah_alanını_al(pencere, bağlam).is_some());
                assert!(uygulama.tezgah_panelleri.is_some());
            });
        });
    }

    /// Sürüm ekseni tükenince yenileme **sessizce doyurulmaz**: exact typed
    /// ret döner ve kök/hizmet/sürüm hiçbir eksende değişmez — farklı bir
    /// bağlam asla aynı damgayla yayımlanamaz.
    #[test]
    fn surum_ekseni_tukenince_yenileme_typed_reddedilir() {
        let fabrika = fabrika();
        let mut kök = MetinHizmetleriKökü::kur(&fabrika, None);
        kök.yerel_kök_sürümü = u64::MAX;
        let eski_bağlam = kök.yerel_kök();
        let eski_hizmet = kök.ileti_hizmeti();
        let istanbul = kök
            .motor()
            .saat_dilimi("Europe/Istanbul")
            .expect("`Europe/Istanbul` kayıtlarda tanınır");
        assert_eq!(
            kök.yerel_kökü_gerekirse_yenile(&fabrika, Some(&istanbul))
                .expect_err("tükenmiş eksende yenileme reddedilmeli"),
            YerelKökHatası::SürümEkseniTükendi
        );
        // Ret hiçbir şeyi değiştirmedi: aynı bağlam, aynı hizmet, sabit sürüm.
        assert!(Arc::ptr_eq(&eski_bağlam, &kök.yerel_kök()));
        assert!(Arc::ptr_eq(&eski_hizmet, &kök.ileti_hizmeti()));
        assert_eq!(kök.yerel_kök_sürümü, u64::MAX);
        // Değişim istemeyen çağrı tükenmiş eksende bile geçerli kalır.
        assert!(
            kök.yerel_kökü_gerekirse_yenile(&fabrika, None)
                .expect("değişimsiz çağrı reddedilmez")
                .is_none()
        );
    }

    /// Canlı UI çözüm yolu typed kanalı **kaybetmez**: bayat bağlam
    /// eksik-kayıt fallback'ine indirgenmez, exact varyant ekranda işaretle
    /// görünür ve `çöz_sonuç` aynı hatayı typed döndürür.
    #[test]
    fn ui_yolu_bayat_baglami_anahtar_fallbackine_indirmez() {
        let fabrika = fabrika();
        let mut kök = MetinHizmetleriKökü::kur(&fabrika, None);
        let eski_bağlam = kök.yerel_kök();
        let istanbul = kök
            .motor()
            .saat_dilimi("Europe/Istanbul")
            .expect("`Europe/Istanbul` kayıtlarda tanınır");
        kök.yerel_kökü_gerekirse_yenile(&fabrika, Some(&istanbul))
            .expect("taze eksende yenileme reddedilmez")
            .expect("dilim değişimi kökü yeniler");

        // Karma çözücü: güncel hizmet + bayat bağlam.
        let kayıt_yuvası = yuva();
        let bayat_çözücü = TezgahİletiÇözücüsü {
            hizmet: kök.ileti_hizmeti(),
            yerel_kök: eski_bağlam,
            hata_kaydı: std::rc::Rc::clone(&kayıt_yuvası),
            önbellek: std::rc::Rc::new(std::cell::RefCell::new(ÇözümÖnbelleği::default())),
        };
        let anahtar =
            YerelleştirmeAnahtarı::yeni("galeri.tezgah.başlık").expect("anahtar geçerlidir");
        assert_eq!(
            bayat_çözücü
                .çöz_sonuç(&anahtar)
                .expect_err("bayat bağlam UI yolunda da çözülmemeli"),
            TezgahÇözümHatası::Çözüm(İletiÇözümHatası::BayatYerelBağlam)
        );
        let görünüm = bayat_çözücü.çöz(&anahtar);
        assert!(
            görünüm.contains("BayatYerelBağlam"),
            "exact varyant ekranda görünmeli: {görünüm}"
        );
        assert_ne!(
            görünüm.as_ref(),
            "galeri.tezgah.başlık",
            "bayatlık eksik-kayıt fallback'iyle karışamaz"
        );
        // Typed payload dizeden önce yuvaya yazıldı; kökte gözlenebilir.
        assert_eq!(
            kayıt_yuvası.borrow().clone(),
            Some(TezgahÇözümKaydı {
                anahtar: anahtar.clone(),
                hata: TezgahÇözümHatası::Çözüm(İletiÇözümHatası::BayatYerelBağlam),
            })
        );
        // Güncel çözücü aynı anahtarı olağan çözer.
        assert_eq!(
            kök.çözücü(yuva()).çöz(&anahtar).as_ref(),
            "Yapılandırma Tezgâhı"
        );
    }

    /// Katalog zinciri düşünce canlı UI yolu exact `KayıtlıZincirYok`
    /// üretir; bu akıbet de eksik-kayıt fallback'iyle karışmaz.
    #[test]
    fn ui_yolu_katalogsuz_kalinca_typed_isaret_gosterir() {
        let fabrika = fabrika();
        let unicode = UnicodeVeYerelMetinHizmetleri::yerlesik(
            ÖrnekKimliğiFabrikası::yeni_süreç_kapsamı().expect("test kimlik kapsamı"),
        );
        let yerel_kök = Arc::new(MetinHizmetleriKökü::yerel_kök_üret(
            &unicode,
            fabrika.sonraki().expect("test kimliği"),
            1,
            None,
        ));
        let kütük = İletiKataloğuKütüğü::muhurle(
            fabrika.sonraki().expect("test kimliği"),
            İletiBütçesi::default(),
        );
        let kayıt = kütük
            .kaydet(tezgah_kataloğu(yerel_kök.dil().clone()))
            .expect("test kataloğu kaydedilir");
        let kayıt_yuvası = yuva();
        let çözücü = TezgahİletiÇözücüsü {
            hizmet: Arc::new(İletiÇözümHizmeti::muhurle(
                kütük,
                unicode.motor(),
                Arc::clone(&yerel_kök),
            )),
            yerel_kök,
            hata_kaydı: std::rc::Rc::clone(&kayıt_yuvası),
            önbellek: std::rc::Rc::new(std::cell::RefCell::new(ÇözümÖnbelleği::default())),
        };
        let anahtar =
            YerelleştirmeAnahtarı::yeni("galeri.tezgah.başlık").expect("anahtar geçerlidir");
        assert_eq!(çözücü.çöz(&anahtar).as_ref(), "Yapılandırma Tezgâhı");

        // Kayıt belirteci düşer: kayıtlı zincir kaybolur.
        drop(kayıt);
        assert_eq!(
            çözücü
                .çöz_sonuç(&anahtar)
                .expect_err("katalogsuz zincir çözülmemeli"),
            TezgahÇözümHatası::KayıtlıZincirYok
        );
        let görünüm = çözücü.çöz(&anahtar);
        assert!(
            görünüm.contains("KayıtlıZincirYok"),
            "exact akıbet ekranda görünmeli: {görünüm}"
        );
        // Typed payload dizeden önce yuvaya yazıldı.
        assert_eq!(
            kayıt_yuvası.borrow().clone(),
            Some(TezgahÇözümKaydı {
                anahtar,
                hata: TezgahÇözümHatası::KayıtlıZincirYok,
            })
        );
    }

    /// Sürüm ekseni tükenmesinin **UI yolundaki gözlenebilirliği**: dilim
    /// tercihi gerçek `tezgahı_değiştir` yüzeyinden değişir, ret exact
    /// typed olarak kök erişicisinde durur, yaşayan alanlar eski (tutarlı)
    /// bağlamda kalır ve saat dilimi kartı akıbet satırını boyar.
    ///
    /// Kanıt seviyesi dürüstçe: tükenme **önkoşulu** private sürüm alanı
    /// kurcalanarak enjekte edilir — `u64::MAX` üretim süresinde
    /// erişilemez; sınanan şey tükenmenin üretilebilirliği değil, nöbetin
    /// UI yolundaki davranışıdır.
    #[gpui::test]
    fn surum_tukenmesi_ui_yolundan_gozlenebilir(bağlam: &mut gpui::TestAppContext) {
        bağlam.update(crate::bileşen_tuş_bağlarını_kur);
        let (uygulama, görsel) = bağlam.add_window_view(|_, _| crate::GaleriUygulaması::yeni());
        görsel.update(|pencere, bağlam| {
            uygulama.update(bağlam, |uygulama, bağlam| {
                let alanlar = uygulama
                    .sergi_girişlerini_al(pencere, bağlam)
                    .expect("sergi alanları kurulur");
                let eski_kök = uygulama.metin_hizmetleri.yerel_kök();
                uygulama.metin_hizmetleri.yerel_kök_sürümü = u64::MAX;
                let istanbul = uygulama
                    .metin_hizmetleri
                    .motor()
                    .saat_dilimi("Europe/Istanbul")
                    .expect("`Europe/Istanbul` tanınır");
                let seçim = istanbul.clone();
                uygulama.tezgahı_değiştir(
                    move |t| t.saat_dilimi_tercihi = crate::SaatDilimiTercihi::Kullanıcı(seçim),
                    bağlam,
                );
                // Ret exact typed olarak kök erişicisinden okunur.
                assert_eq!(
                    uygulama.yerel_kök_hatası(),
                    Some(YerelKökHatası::SürümEkseniTükendi)
                );
                // Eski bağlam yürürlükte: kök ve yaşayan alanlar tutarlı.
                assert!(Arc::ptr_eq(
                    &eski_kök,
                    &uygulama.metin_hizmetleri.yerel_kök()
                ));
                assert_eq!(&alanlar.yalın.read(bağlam).yerel, eski_kök.as_ref());
                // Tarih türünde saat dilimi kartı kurulur; akıbet değeri
                // bölüm kurulumuna (çizim yoluna) inilir.
                uygulama
                    .tezgahı_değiştir(|t| t.değer_türü = crate::TezgahDeğerKipi::Tarih, bağlam);
                assert_eq!(
                    uygulama.yerel_kök_hatası(),
                    Some(YerelKökHatası::SürümEkseniTükendi),
                    "akıbet tercih değişimlerinde sessizce silinmez"
                );
                let bölümler = uygulama.tezgah_bölümleri(pencere, bağlam);
                assert!(
                    bölümler.iter().any(|bölüm| bölüm.kimlik == "saat_dilimi"),
                    "saat dilimi kartı akıbetle birlikte kurulmalı"
                );
            });
        });
        // Bir sonraki gerçek kare: saat dilimi kartı exact akıbet satırını
        // **boyar**; metin `yerel_kök_hatası_metni` üreticisinden gelir
        // (`tani_metinleri_exact_varyanti_tasir`). Bu assert son çizilen
        // karenin bir tazeleme (önbellek-ıskası) karesi olmasına dayanır:
        // araya temiz kare sokan bir düzenleme asserti de taşımalıdır.
        görsel.run_until_parked();
        assert!(
            görsel.debug_bounds("tanı-yerel-kök-hatası").is_some(),
            "yerel kök akıbet satırı gerçekten boyanmalı"
        );
    }

    /// Canlı **render** yolu typed hatayı dizeye inmeden korur ve tanı
    /// satırı **aynı karede gerçekten boyanır**: kök render girdisini
    /// kurmadan önce kanonik anahtarı yoklar, sistemik akıbet yuvaya düşer
    /// ve satır o karenin çıktısında `debug_bounds` ile ölçülür.
    ///
    /// Hata üreticisi bir **invariant-ihlali kurgusudur**: üretimde bu
    /// akıbetleri üretebilen erişilebilir yol *tasarım gereği yoktur*
    /// (kök, bağlam+kütük+hizmeti tek atomda değiştirir; kanıtı
    /// `dilim_degisimi_koku_hizmetiyle_birlikte_yeniler` ve
    /// `uretim_yollari_sistemik_cozum_hatasi_uretmez`). Bu kanal bir
    /// nöbettir; test, nöbetin ihlal hâlinde typed ve görünür çalıştığını
    /// private durumu bilerek bozarak sınar.
    #[gpui::test]
    fn canli_render_yolu_typed_hatayi_koruyarak_kaydeder(bağlam: &mut gpui::TestAppContext) {
        bağlam.update(crate::bileşen_tuş_bağlarını_kur);
        let (uygulama, görsel) = bağlam.add_window_view(|_, _| crate::GaleriUygulaması::yeni());
        görsel.run_until_parked();
        // Olağan karede yuva boştur ve tanı satırı boyanmaz.
        görsel.update(|_, bağlam| {
            assert!(uygulama.read(bağlam).son_ileti_çözüm_hatası().is_none());
        });
        assert!(
            görsel.debug_bounds("tanı-son-ileti-çözüm-hatası").is_none(),
            "sağlıklı karede tanı satırı çizilmemeli"
        );

        // İnvariant ihlali kurgusu: kökün yaşayan bağlamı hizmetle uyuşmayan
        // bir bağlamla değiştirilir. Bir sonraki gerçek çizim, sunumdan
        // önceki yoklamada exact `BayatYerelBağlam` üretmek zorundadır.
        görsel.update(|_, bağlam| {
            uygulama.update(bağlam, |uygulama, bağlam| {
                let unicode = uygulama.metin_hizmetleri.unicode();
                let başka_bağlam = MetinHizmetleriKökü::yerel_kök_üret(
                    &unicode,
                    fabrika().sonraki().expect("test kimliği"),
                    1,
                    None,
                );
                uygulama.metin_hizmetleri.yerel_kök = Arc::new(başka_bağlam);
                bağlam.notify();
            });
        });
        // Tek kare: yoklama + kayıt + tanı çizimi aynı karede tamamlanır;
        // ikinci bir kare planına gerek yoktur.
        görsel.run_until_parked();

        let kayıt = görsel.update(|_, bağlam| uygulama.read(bağlam).son_ileti_çözüm_hatası());
        let kayıt = kayıt.expect("gerçek render typed akıbeti yuvaya yazmalı");
        assert_eq!(
            kayıt.hata,
            TezgahÇözümHatası::Çözüm(İletiÇözümHatası::BayatYerelBağlam)
        );
        assert!(
            kayıt.anahtar.as_ref().starts_with("galeri.tezgah"),
            "kayıt gerçek kabuk anahtarını taşımalı: {}",
            kayıt.anahtar.as_ref()
        );
        // Tanı satırı bu karede gerçekten boyandı; metni exact üreticiden
        // gelir (`tani_metinleri_exact_varyanti_tasir`).
        assert!(
            görsel.debug_bounds("tanı-son-ileti-çözüm-hatası").is_some(),
            "tanı satırı aynı karede boyanmalı"
        );

        // İyileşme: invariant geri kurulunca (kök hizmetle yeniden tutarlı)
        // sistemik mandal bırakılır — yuva temizlenir ve tanı satırı söner.
        görsel.update(|_, bağlam| {
            uygulama.update(bağlam, |uygulama, bağlam| {
                let asıl = uygulama.metin_hizmetleri.ileti_hizmeti();
                uygulama.metin_hizmetleri.yerel_kök = Arc::new(asıl.yerel_kök().clone());
                bağlam.notify();
            });
        });
        görsel.run_until_parked();
        görsel.update(|_, bağlam| {
            assert!(
                uygulama.read(bağlam).son_ileti_çözüm_hatası().is_none(),
                "sistemik akıbet geçince mandal bırakılmalı"
            );
        });
        assert!(
            görsel.debug_bounds("tanı-son-ileti-çözüm-hatası").is_none(),
            "iyileşen karede tanı satırı çizilmemeli"
        );
    }

    /// `MetinDamgası` ile `YerelBağlamDamgası` ayrı nominal eksenlerdir:
    /// alanın metin damgası yerel kökün damgasından türetilmez/eşitlenmez
    /// ve çalışma-anı kök yenilemesi yaşayan metin damgasına dokunmaz —
    /// yalnız yerel-türevli hizmet kuşağı değişir.
    #[test]
    fn damga_eksenleri_ayri_ve_kok_yenilemesi_metin_damgasina_dokunmaz() {
        let fabrika = fabrika();
        let mut kök = MetinHizmetleriKökü::kur(&fabrika, None);
        let metin_damgası = kök.alan_damgası(&fabrika);
        let önce = metin_damgası.canli_damga();
        assert_ne!(
            önce.bağlam,
            kök.yerel_kök().damga().bağlam(),
            "iki damga ekseni ayrı soylardan doğar"
        );
        let istanbul = kök
            .motor()
            .saat_dilimi("Europe/Istanbul")
            .expect("`Europe/Istanbul` tanınır");
        kök.yerel_kökü_gerekirse_yenile(&fabrika, Some(&istanbul))
            .expect("taze eksende yenileme reddedilmez")
            .expect("dilim değişimi kökü yeniler");
        assert_eq!(
            metin_damgası.canli_damga(),
            önce,
            "kök yenilemesi metin damgasına dokunmaz"
        );
        assert_ne!(kök.yerel_kök().damga().bağlam(), önce.bağlam);
    }

    /// Üretimden erişilebilir yollar sistemik çözüm hatası **üretmez**:
    /// dilim/tür tercihleri gerçek yüzeyden defalarca değişip kareler
    /// çizilirken yuva boş kalır, yoklama `None` döner ve iki tanı satırı
    /// da boyanmaz. Nöbet kanalının üretim-erişilebilir üreticisi tasarım
    /// gereği yoktur; bu test o iddiayı kanıta çevirir.
    #[gpui::test]
    fn uretim_yollari_sistemik_cozum_hatasi_uretmez(bağlam: &mut gpui::TestAppContext) {
        bağlam.update(crate::bileşen_tuş_bağlarını_kur);
        let (uygulama, görsel) = bağlam.add_window_view(|_, _| crate::GaleriUygulaması::yeni());
        görsel.run_until_parked();

        let dilimler =
            görsel.update(|_, bağlam| uygulama.read(bağlam).metin_hizmetleri.dilim_seçenekleri());
        let türler = [
            crate::TezgahDeğerKipi::Tarih,
            crate::TezgahDeğerKipi::Tamsayı,
            crate::TezgahDeğerKipi::Metin,
        ];
        for (sıra, kimlik) in dilimler.kullanıcı.iter().enumerate() {
            let seçim = kimlik.clone();
            let tür = türler[sıra % türler.len()];
            görsel.update(|_, bağlam| {
                uygulama.update(bağlam, |uygulama, bağlam| {
                    uygulama.tezgahı_değiştir(
                        move |t| {
                            t.saat_dilimi_tercihi = crate::SaatDilimiTercihi::Kullanıcı(seçim);
                            t.değer_türü = tür;
                        },
                        bağlam,
                    );
                });
            });
            görsel.run_until_parked();
        }
        // Portsuz `platform_portlarını_kur` da üretim yüzeyindendir; kökü
        // koşulsuz güvenli eşitler ve hata üretmez.
        görsel.update(|_, bağlam| {
            uygulama.update(bağlam, |uygulama, bağlam| {
                uygulama.platform_portlarını_kur(crate::PlatformPortları::default(), bağlam);
            });
        });
        görsel.run_until_parked();
        // Ürün sabiti ve platform tercihleri de gerçek yüzeyden geçer.
        let ürün = dilimler.ürün.clone();
        görsel.update(|_, bağlam| {
            uygulama.update(bağlam, |uygulama, bağlam| {
                uygulama.tezgahı_değiştir(
                    move |t| t.saat_dilimi_tercihi = crate::SaatDilimiTercihi::Ürün(ürün),
                    bağlam,
                );
            });
        });
        görsel.run_until_parked();
        görsel.update(|_, bağlam| {
            uygulama.update(bağlam, |uygulama, bağlam| {
                uygulama.tezgahı_değiştir(
                    |t| t.saat_dilimi_tercihi = crate::SaatDilimiTercihi::Platform,
                    bağlam,
                );
            });
        });
        görsel.run_until_parked();

        görsel.update(|_, bağlam| {
            uygulama.update(bağlam, |uygulama, _| {
                assert!(
                    uygulama.son_ileti_çözüm_hatası().is_none(),
                    "üretim yolları typed çözüm hatası kaydetmemeli"
                );
                assert!(uygulama.yerel_kök_hatası().is_none());
                assert!(
                    uygulama.tezgah_çözücüsü().yokla().is_none(),
                    "yoklama üretim yollarında temiz kalmalı"
                );
            });
        });
        assert!(görsel.debug_bounds("tanı-son-ileti-çözüm-hatası").is_none());
        assert!(görsel.debug_bounds("tanı-yerel-kök-hatası").is_none());
    }

    /// Tanı satırlarının exact metin üreticileri: çizen kartlar ve testler
    /// aynı fonksiyonu kullanır; "çizilen" ile "sınanan" ayrışamaz.
    #[test]
    fn tani_metinleri_exact_varyanti_tasir() {
        assert_eq!(
            yerel_kök_hatası_metni(YerelKökHatası::SürümEkseniTükendi),
            "‹SürümEkseniTükendi› yerel kök yenilenemedi; eski bağlam yürürlükte"
        );
        let kayıt = TezgahÇözümKaydı {
            anahtar: YerelleştirmeAnahtarı::yeni("galeri.tezgah.başlık")
                .expect("anahtar geçerlidir"),
            hata: TezgahÇözümHatası::Çözüm(İletiÇözümHatası::BayatYerelBağlam),
        };
        assert_eq!(
            son_çözüm_hatası_metni(&kayıt),
            "Son ileti çözüm hatası · ‹Çözüm(BayatYerelBağlam)› galeri.tezgah.başlık"
        );
    }

    /// RTL kökte `AksanEkle`/`MetniUzat` gerçek bağlamın RTL yönünü korur —
    /// yön kipten değil bağlamdan gelir.
    ///
    /// Kanıt seviyesi hizmet düzeyidir; canlı-host RTL geçişi için bkz.
    /// `rtl_yerel_sagdan_sola_cozulur` notu.
    #[test]
    fn rtl_kokte_uzatma_gercek_yonu_korur() {
        let fabrika = fabrika();
        let unicode = UnicodeVeYerelMetinHizmetleri::yerlesik(
            ÖrnekKimliğiFabrikası::yeni_süreç_kapsamı().expect("test kimlik kapsamı"),
        );
        let motor = unicode.motor();
        let arapça = unicode.yerel_bağlam_fabrikası().bağlam(
            CanlıBağlamDamgası {
                bağlam: fabrika.sonraki().expect("test kimliği"),
                sürüm: BağlamSürümü(1),
            },
            motor.dil_etiketi("ar").expect("`ar` tanınır"),
            motor
                .numaralandırma_sistemi("latn")
                .expect("`latn` tanınır"),
            motor.takvim("gregory").expect("`gregory` tanınır"),
            motor.saat_dilimi("UTC").expect("`UTC` tanınır"),
        );
        let arapça = Arc::new(arapça);
        let kütük = İletiKataloğuKütüğü::muhurle(
            fabrika.sonraki().expect("test kimliği"),
            İletiBütçesi::default(),
        );
        let _kayıt = kütük
            .kaydet(tezgah_kataloğu(arapça.dil().clone()))
            .expect("ar test kataloğu kaydedilir");
        let hizmet = İletiÇözümHizmeti::muhurle(kütük, unicode.motor(), Arc::clone(&arapça));
        let katalog = hizmet.etkin_katalog(&arapça).expect("ar kataloğu kayıtlı");
        let uzatılmış = hizmet
            .sahte_locale_çöz(
                &istek("galeri.tezgah.başlık"),
                &arapça,
                katalog,
                SahteLocaleKipi::MetniUzat { oran: 1.5 },
            )
            .expect("MetniUzat çözülür");
        assert_eq!(uzatılmış.yazı_yönü(), &ÇözülmüşYazıYönü::SağdanSola);
    }
}

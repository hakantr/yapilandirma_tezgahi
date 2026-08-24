//! Tezgâh kabuğunun renk tokenları.
//!
//! `ORT-004` renk değerinin sahibi temadır ve `ACC-001` bileşen çizim kodunda
//! ham renk bulunmasını yasaklar. Bu modül tek çeviri noktasıdır: `palet.rs`
//! içindeki ham `u32` rolleri burada bir kez `Hsla`ya çevrilir, tezgâhın geri
//! kalanı yalnız bu yapıyı okur.
//!
//! Dört kip vardır ve hiçbiri diğerinden **türetilmez**: yüksek karşıtlık
//! kipleri `ORT-004 §5.7` uyarınca ayrıca doğrulanmış ve o kip için elle
//! kaydedilmiş sayılır. Türetme, doğrulanmamış bir karşıtlığı doğrulanmış
//! gibi sunardı (`ACC-011`).

use gpui::{Hsla, rgb};
use gpui_bilesenleri::TemaKipi;

use crate::{GaleriPaleti, GaleriTeması, galeri_paleti};

/// Tezgâh kabuğunun bir kipteki bütün renkleri.
///
/// Alanlar `ORT-004` semantik rolleridir; ham `u32` buraya kadar gelir ve
/// burada biter. Renk hiçbir yerde tek bilgi kanalı değildir: hata, uyarı ve
/// bilgi işaretleri renkten bağımsız üç ayrı glif taşır.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TezgahTokenları {
    /// Tokenların üretildiği kip; `TemaAnlıkGörüntüsü::kip` bundan doldurulur.
    pub kip: TemaKipi,
    // Yüzeyler
    pub kağıt: Hsla,
    pub yüzey: Hsla,
    pub kenarlık: Hsla,
    pub ince: Hsla,
    // Metin
    pub ana_metin: Hsla,
    pub ikincil_metin: Hsla,
    pub soluk: Hsla,
    // Vurgu
    pub vurgu: Hsla,
    pub vurgu_zemin: Hsla,
    // Semantik durum
    pub olumlu: Hsla,
    pub tehlike: Hsla,
    pub uyarı: Hsla,
    pub bilgi: Hsla,
    // Kod paneli
    pub kod_zemin: Hsla,
    pub kod_metin: Hsla,
    /// Gölge rengi; opaklığı çizim yerinde değil, gölge tokenında verilir.
    pub gölge: Hsla,
}

impl TezgahTokenları {
    /// Seçilen tema ve kipin tokenları.
    pub fn kip(tema: GaleriTeması, kip: TemaKipi) -> Self {
        Self::paletten(galeri_paleti(tema, kip))
    }

    /// Kare başında kurulmuş paletten okur.
    ///
    /// Çeviri tek yerdedir: palet ham `u32` taşır, tezgâh yalnız `Hsla`
    /// görür.
    pub fn paletten(palet: GaleriPaleti) -> Self {
        let renk = |onaltılık: u32| -> Hsla { rgb(onaltılık).into() };
        Self {
            kip: palet.kip,
            kağıt: renk(palet.kağıt),
            yüzey: renk(palet.yüzey),
            kenarlık: renk(palet.kenarlık),
            ince: renk(palet.ince),
            ana_metin: renk(palet.ana_metin),
            ikincil_metin: renk(palet.ikincil_metin),
            soluk: renk(palet.soluk),
            vurgu: renk(palet.vurgu),
            vurgu_zemin: renk(palet.vurgu_zemin),
            olumlu: renk(palet.olumlu),
            tehlike: renk(palet.tehlike),
            uyarı: renk(palet.uyarı),
            bilgi: renk(palet.bilgi),
            kod_zemin: renk(palet.kod_zemin),
            kod_metin: renk(palet.kod_metin),
            gölge: renk(palet.gölge),
        }
    }

    // `§16.2` gösterge glifi burada **eşlenmez**. Kapı açılana kadar
    // tezgâh tarafında bir renk/glif eşlemesi duruyordu; kanonik
    // `GirişKutusu` artık parçayı kendisi çizip simge kimliğini
    // `ORT-016`dan çözdüğü için o eşleme ikinci bir otorite olurdu —
    // nitekim iki taraf hata glifi üzerinde ayrışmıştı bile.
}

#[cfg(test)]
mod testler {
    use super::*;

    const KİPLER: [TemaKipi; 4] = [
        TemaKipi::Açık,
        TemaKipi::Koyu,
        TemaKipi::YüksekKarşıtlıkAçık,
        TemaKipi::YüksekKarşıtlıkKoyu,
    ];

    /// Dört kip de ayrı ayrı kayıtlıdır; hiçbiri diğerinin kopyası değildir.
    ///
    /// `ORT-004 §5.7` yüksek karşıtlık kiplerinin otomatik üretilmesini
    /// yasaklar. Bu test, YK kipinin olağan kipe indirgenmediğini gösterir.
    #[test]
    fn yüksek_karşıtlık_kipleri_olağan_kipe_indirgenmez() {
        for tema in GaleriTeması::TÜMÜ {
            let açık = TezgahTokenları::kip(tema, TemaKipi::Açık);
            let yk_açık = TezgahTokenları::kip(tema, TemaKipi::YüksekKarşıtlıkAçık);
            let koyu = TezgahTokenları::kip(tema, TemaKipi::Koyu);
            let yk_koyu = TezgahTokenları::kip(tema, TemaKipi::YüksekKarşıtlıkKoyu);

            assert_ne!(
                açık.kağıt,
                yk_açık.kağıt,
                "{} YK açık ayrı palet",
                tema.adı()
            );
            assert_ne!(
                koyu.kağıt,
                yk_koyu.kağıt,
                "{} YK koyu ayrı palet",
                tema.adı()
            );
            assert_ne!(
                açık.vurgu,
                yk_açık.vurgu,
                "{} YK açık vurgu ayrı",
                tema.adı()
            );
            assert_ne!(
                koyu.vurgu,
                yk_koyu.vurgu,
                "{} YK koyu vurgu ayrı",
                tema.adı()
            );
        }
    }

    /// Token kipi paletten gelir; zemin parlaklığından tahmin edilmez.
    #[test]
    fn kip_tahmin_edilmez_paletten_gelir() {
        for tema in GaleriTeması::TÜMÜ {
            for kip in KİPLER {
                assert_eq!(TezgahTokenları::kip(tema, kip).kip, kip);
            }
        }
    }

    /// Dört semantik renk ve gölge her kipte tanımlıdır.
    ///
    /// Eksik rol, çizim yerinde uydurulmuş bir renge yol açardı.
    #[test]
    fn semantik_roller_dört_kipte_de_kurulu() {
        for tema in GaleriTeması::TÜMÜ {
            for kip in KİPLER {
                let t = TezgahTokenları::kip(tema, kip);
                for (ad, renk) in [
                    ("olumlu", t.olumlu),
                    ("tehlike", t.tehlike),
                    ("uyarı", t.uyarı),
                    ("bilgi", t.bilgi),
                ] {
                    assert_ne!(renk, t.kağıt, "{ad} zeminden ayırt edilebilir olmalı");
                    assert_ne!(renk, t.yüzey, "{ad} yüzeyden ayırt edilebilir olmalı");
                }
            }
        }
    }

    /// Yüksek karşıtlık kiplerinde zemin ile ana metin uçlara gider.
    ///
    /// Mekanik denetim tek başına yayın yetkisi değildir; bu test yalnız
    /// kaydedilen değerlerin kip amacına aykırı olmadığını gösterir.
    #[test]
    fn yüksek_karşıtlık_kipleri_uç_zemin_ve_metin_taşır() {
        for tema in GaleriTeması::TÜMÜ {
            let açık = galeri_paleti(tema, TemaKipi::YüksekKarşıtlıkAçık);
            assert_eq!(açık.kağıt, 0xffffff, "{} YK açık zemin", tema.adı());
            assert_eq!(açık.ana_metin, 0x000000, "{} YK açık metin", tema.adı());

            let koyu = galeri_paleti(tema, TemaKipi::YüksekKarşıtlıkKoyu);
            assert_eq!(koyu.kağıt, 0x000000, "{} YK koyu zemin", tema.adı());
            assert_eq!(koyu.ana_metin, 0xffffff, "{} YK koyu metin", tema.adı());
        }
    }
}

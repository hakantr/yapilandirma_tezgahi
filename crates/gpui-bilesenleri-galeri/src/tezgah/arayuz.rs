//! Tezgâh kabuğu ile bileşen profili arasındaki sınır.
//!
//! Kabuk hiçbir bileşen tipini tanımaz. Profil kendi bağlamıyla çizim
//! ağaçlarını üretir ve kabuğa **hazır `AnyElement`** olarak verir; kabuk
//! yalnız yerleştirir. Bu yüzden sınır bir trait değil, bir veri yapısıdır:
//! trait olsaydı `Context<T>` parametresi kabuğu galeri uygulamasına
//! bağlardı ve "bileşen-bağımsız kabuk" iddiası kalmazdı.
//!
//! Tür süzgeci de profilin işidir. Kabuk "bu bölüm bu türde kurulabilir mi"
//! sorusunu sormaz; profil `bölümler()` sonucunu **zaten süzülmüş** verir
//! (`§9`: kurulamayan eksen hiç `child` üretmez, kapanan eksen pasif ve
//! gerekçeli kalır — ikisi de profilin kararıdır).

use gpui::{AnyElement, ScrollHandle};
use gpui_bilesenleri_kabuk::YerelleştirmeAnahtarı;

/// Bir yapılandırma bölümünün hangi akışa düştüğü.
///
/// Tasarımın `§5` yerleşimi: iki bölüm tam genişlik, kalanlar üç akışa
/// dağılır. Akış içindeki kartlar iki kolona bölünür; tek kart kalan akış
/// yarım sütun bırakmaz.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Akış {
    /// Kolon bölünmesine girmez, gövdenin tam genişliğini alır.
    TamGenişlik,
    A,
    B,
    C,
}

impl Akış {
    pub const AKIŞLAR: [Self; 3] = [Self::A, Self::B, Self::C];
}

/// Sağ kolondaki bir yapılandırma bölümü.
///
/// Başlık ham dize değil `ORT-021` anahtarıdır: `YÖN-006.ACC-008` sergi
/// başlığını güncel locale sürümünde çözdürür ve hazır dizeyi kaynak saymaz.
pub struct TezgahBölümü {
    /// Kararlı bölüm kimliği (`"s7"`, `"s9"` …). Çapa gezintisi bunu kullanır.
    pub kimlik: &'static str,
    pub başlık: YerelleştirmeAnahtarı,
    /// `?` yardım yüzeyinin içeriği.
    ///
    /// Yüzeyin kendisi `ORT-006` `Araçİpucu` konağı fiziksel olana kadar
    /// çizilmez; anahtar şimdiden taşınır ki kapı açıldığında metin
    /// aranmasın.
    pub yardım: Option<YerelleştirmeAnahtarı>,
    pub akış: Akış,
    pub içerik: AnyElement,
}

/// Bir bileşen profilinin kabuğa verdiği bütün çizim ağaçları.
///
/// Sıra anlamlıdır ve kabuk onu değiştirmez.
pub struct Tezgahİçeriği {
    /// Tezgâhın erişilebilir adı.
    pub başlık: YerelleştirmeAnahtarı,
    /// Sol kolonun erişilebilir bölge adı.
    pub önizleme_başlığı: YerelleştirmeAnahtarı,
    /// Sol kolonun üst bloğu: kabuk denetimleri ve yaşayan önizleme.
    pub önizleme: Vec<AnyElement>,
    /// Sol kolonun alt blokları: türetilmiş durumlar, gözlem panelleri.
    pub sol_ek: Vec<AnyElement>,
    /// Alt bloğun sıradan flex-scroll yerine ölçüm amaçlı sanal liste olup
    /// olmadığı. Sanal yol kendi kaydırmasını taşır; kabuk ikinci bir
    /// `overflow_y_scroll` katmanı kurmaz.
    pub sol_sanal: bool,
    /// Sıradan sol kaydırmanın kararlı tutamacı. Ölçüm aynı üst düzey öğeyi
    /// A ve B yollarında görünür kılabilsin diye profil sınırından geçer.
    pub sol_kaydırma: ScrollHandle,
    /// "Karşılığı olan kod" paneli; sol kolonun en altında durur.
    pub kod: Option<AnyElement>,
    /// Sağ kolonun hazır çizimi.
    ///
    /// Profil bunu önbellekli bir bölüm paneli entity'si olarak verir:
    /// kolonun bölümleri kabukta değil, panelin kendi çiziminde kurulur
    /// ve kök bildirmedikçe (tuş vuruşlarında) yeniden kurulmaz. Kabuk
    /// yalnız yerleştirir; erişilebilir bölge adı da kolonun kendi
    /// çizimindedir.
    pub yapılandırma: AnyElement,
}

/// Bir akışa düşen bölümleri sırasını bozmadan ayırır.
pub fn akış_bölümleri(
    bölümler: &mut Vec<TezgahBölümü>, akış: Akış
) -> Vec<TezgahBölümü> {
    let mut kalan = Vec::with_capacity(bölümler.len());
    let mut seçilen = Vec::new();
    for bölüm in std::mem::take(bölümler) {
        if bölüm.akış == akış {
            seçilen.push(bölüm);
        } else {
            kalan.push(bölüm);
        }
    }
    *bölümler = kalan;
    seçilen
}

#[cfg(test)]
mod testler {
    use super::*;
    use gpui::prelude::*;
    use gpui_bilesenleri_kabuk::YerelleştirmeAnahtarı;

    fn anahtar(değer: &str) -> YerelleştirmeAnahtarı {
        YerelleştirmeAnahtarı::yeni(değer).expect("test anahtarı geçerlidir")
    }

    fn bölüm(kimlik: &'static str, akış: Akış) -> TezgahBölümü {
        TezgahBölümü {
            kimlik,
            başlık: anahtar(kimlik),
            yardım: None,
            akış,
            içerik: gpui::div().into_any_element(),
        }
    }

    #[test]
    fn akış_bölümleri_sırayı_korur() {
        let mut bölümler = vec![
            bölüm("s7", Akış::TamGenişlik),
            bölüm("s6", Akış::A),
            bölüm("s10", Akış::B),
            bölüm("s6ek", Akış::A),
        ];

        let a = akış_bölümleri(&mut bölümler, Akış::A);
        assert_eq!(
            a.iter().map(|b| b.kimlik).collect::<Vec<_>>(),
            vec!["s6", "s6ek"]
        );
        // Ayrılan bölümler listeden düşer, kalanların sırası bozulmaz.
        assert_eq!(
            bölümler.iter().map(|b| b.kimlik).collect::<Vec<_>>(),
            vec!["s7", "s10"]
        );
    }

    #[test]
    fn boş_akış_boş_liste_verir() {
        let mut bölümler = vec![bölüm("s7", Akış::TamGenişlik)];
        assert!(akış_bölümleri(&mut bölümler, Akış::C).is_empty());
        assert_eq!(bölümler.len(), 1);
    }
}

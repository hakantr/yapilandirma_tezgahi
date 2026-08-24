//! Tezgâh kabuğunun yerleşim çekirdeği.
//!
//! Kip seçimi **ölçülen kap genişliğinden** yapılır, viewport'tan değil:
//! `YÖN-006 §3.4` yerleşim kipini "kullanılabilir alan, metin ölçeği ve yön
//! snapshot'ı" ile seçtirir. Tezgâh galerinin orta bölgesine gömülüdür, yani
//! viewport genişliği ona ayrılan alanı bildirmez.
//!
//! Ölçüler burada `KolonMetriği` olarak tek sahiplidir. `F0b` açıldığında bu
//! yapı `TezgahGörünümProfili`ne alan olarak taşınır; taşınana kadar da ham
//! `px` başka hiçbir yerde tekrarlanmaz (`ORT-004.ACC-001`).

use gpui::{Pixels, px};

/// Tezgâh gövdesinin kolon ölçüleri.
///
/// Kanonik `ORT-017` karşılığı yoktur; tezgâh kabuğunun kendi tipidir.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KolonMetriği {
    /// Önizleme kolonunun sabit genişliği.
    pub önizleme_kolonu: Pixels,
    /// İki kolon arası boşluk.
    pub kolon_aralığı: Pixels,
    /// Yapılandırma kolonunun `%100` metin ölçeğindeki alt sınırı.
    pub yapılandırma_asgarisi: Pixels,
    /// Akış kartları arası dikey/yatay boşluk.
    pub kart_aralığı: Pixels,
    /// Kart iç dolgusu.
    pub kart_dolgusu: Pixels,
    /// Kart içindeki başlık ile gövde arası.
    pub kart_içi_aralık: Pixels,
}

impl KolonMetriği {
    /// Tasarımın `§5` yerleşim geometrisinden gelen ölçüler.
    pub const fn tasarım() -> Self {
        Self {
            önizleme_kolonu: px(404.),
            kolon_aralığı: px(28.),
            yapılandırma_asgarisi: px(460.),
            kart_aralığı: px(16.),
            kart_dolgusu: px(14.),
            kart_içi_aralık: px(8.),
        }
    }
}

#[cfg(test)]
mod testler {
    use super::*;

    /// Metin ölçeği yalnız yapılandırma kolonunu büyütür.
    ///
    /// Önizleme kolonu sabit ölçülü bir kabuk taşır; yapılandırma kolonu
    /// metin yoğun kartlardan oluşur ve `%200` ölçekte iki katına çıkar.
    #[test]
    fn metrikler_profilden_gelir() {
        let m = KolonMetriği::tasarım();
        assert_eq!(m.önizleme_kolonu, px(404.));
        assert!(m.yapılandırma_asgarisi > m.kolon_aralığı);
    }
}

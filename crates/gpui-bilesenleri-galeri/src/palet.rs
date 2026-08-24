//! Galerinin çalışma anında değişen renk paleti.
//!
//! `ORT-004` renk değerinin sahibi temadır; galeri de kendi kabuğunu bir
//! temadan çözmeli ki tema yönetiminin gerçekten çalıştığı görülebilsin.
//! Burada iki tema var — `Kâğıt` ve `Mürekkep` — ve her ikisinin **dört**
//! kipi. Seçim üst bardan yapılır ve pencerenin tamamına uygulanır.
//!
//! Yüksek karşıtlık kipleri olağan kipten **türetilmez**: `ORT-004 §5.7`
//! uyarınca her biri ayrıca doğrulanmış ve o kip için elle kaydedilmiş
//! sayılır. Bir kipi hesaplayarak üretmek, doğrulanmamış bir karşıtlığı
//! doğrulanmış gibi sunmak olurdu (`ORT-004.ACC-011`).
//!
//! Palet kare başına bir kez kurulur ve çizim ağacı kurulurken okunur.
//! Renk okuyan yardımcıların çoğu (`şerit()` gibi) bağlam almadığı için
//! değer bir iş parçacığı yereline konur: galeri tek iş parçacıklı çizim
//! kodudur, kare içinde palet değişmez ve yalnız okunur.

use std::cell::Cell;

use gpui_bilesenleri::TemaKipi;

/// Galerinin seçilebilen temaları.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum GaleriTeması {
    /// Sıcak kâğıt zemin, terracotta vurgu.
    #[default]
    Kağıt,
    /// Soğuk mürekkep zemin, indigo vurgu.
    Mürekkep,
}

impl GaleriTeması {
    pub const TÜMÜ: [Self; 2] = [Self::Kağıt, Self::Mürekkep];

    pub const fn adı(self) -> &'static str {
        match self {
            Self::Kağıt => "Kâğıt",
            Self::Mürekkep => "Mürekkep",
        }
    }
}

/// Galerinin bir karede kullandığı bütün renkler.
///
/// Alanlar `ORT-004` rollerine karşılık gelir; ham RGB değeri yalnız burada
/// durur ve tema seçimiyle değişir.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GaleriPaleti {
    /// Paletin üretildiği kip.
    ///
    /// Zemin parlaklığından tahmin edilmez: yüksek karşıtlık kipleri olağan
    /// kiple aynı aydınlıkta olabilir ve tahmin onları `Açık`/`Koyu` diye
    /// bildirirdi. `TemaAnlıkGörüntüsü::kip` bu alandan doldurulur.
    pub kip: TemaKipi,
    // Tezgâh yüzeyi
    pub kağıt: u32,
    pub yüzey: u32,
    pub kenarlık: u32,
    pub ince: u32,
    pub ana_metin: u32,
    pub ikincil_metin: u32,
    pub soluk: u32,
    pub vurgu: u32,
    pub vurgu_zemin: u32,
    pub kod_zemin: u32,
    pub kod_metin: u32,
    // Semantik durum renkleri.
    //
    // Renk hiçbir yerde tek bilgi kanalı değildir: hata, uyarı ve bilgi
    // işaretleri renkten bağımsız üç ayrı glif taşır.
    pub olumlu: u32,
    pub tehlike: u32,
    pub uyarı: u32,
    pub bilgi: u32,
    /// Gölge rengi. Açık/koyu kipte nötr siyah, yüksek karşıtlık kiplerinde
    /// zeminin kendi mürekkebi kullanılır.
    pub gölge: u32,
    // Katalog kabuğu
    pub kabuk_zemin: u32,
    pub kabuk_kart: u32,
    pub kabuk_kenarlık: u32,
    pub kabuk_ana_metin: u32,
    pub kabuk_ikincil_metin: u32,
    pub kabuk_vurgu: u32,
    pub kabuk_seçili_zemin: u32,
}

/// Seçilen tema ve kipin paleti.
///
/// Sekiz palet de elle yazılıdır. Yüksek karşıtlık kipleri olağan kipe
/// indirgenmez ve ondan hesaplanmaz.
pub const fn galeri_paleti(tema: GaleriTeması, kip: TemaKipi) -> GaleriPaleti {
    match (tema, kip) {
        (GaleriTeması::Kağıt, TemaKipi::Açık) => GaleriPaleti {
            kip,
            kağıt: 0xf2efe9,
            yüzey: 0xfbfaf7,
            kenarlık: 0xddd7cc,
            ince: 0xe7e2d9,
            ana_metin: 0x1d1a16,
            ikincil_metin: 0x6d6659,
            soluk: 0xa49c8e,
            vurgu: 0xa8452a,
            vurgu_zemin: 0xf4e7e1,
            kod_zemin: 0x22201c,
            kod_metin: 0x8d867a,
            olumlu: 0x3f6b4f,
            tehlike: 0xb3261e,
            uyarı: 0x8a6d1f,
            bilgi: 0x2f6b5e,
            gölge: 0x000000,
            kabuk_zemin: 0xf6f4ef,
            kabuk_kart: 0xfffdf9,
            kabuk_kenarlık: 0xe3ded3,
            kabuk_ana_metin: 0x1d1a16,
            kabuk_ikincil_metin: 0x6d6659,
            kabuk_vurgu: 0xa8452a,
            kabuk_seçili_zemin: 0xf4e7e1,
        },
        (GaleriTeması::Kağıt, TemaKipi::Koyu) => GaleriPaleti {
            kip,
            kağıt: 0x1f1d19,
            yüzey: 0x2a2723,
            kenarlık: 0x413c34,
            ince: 0x35312b,
            ana_metin: 0xf1ece3,
            ikincil_metin: 0xb5ac9c,
            soluk: 0x7d766a,
            vurgu: 0xe08a6b,
            vurgu_zemin: 0x3d2a22,
            kod_zemin: 0x141310,
            kod_metin: 0x9b9385,
            olumlu: 0x83b190,
            tehlike: 0xe28d84,
            uyarı: 0xcfae63,
            bilgi: 0x7cc0b0,
            gölge: 0x000000,
            kabuk_zemin: 0x181714,
            kabuk_kart: 0x232019,
            kabuk_kenarlık: 0x3a352d,
            kabuk_ana_metin: 0xf1ece3,
            kabuk_ikincil_metin: 0xb5ac9c,
            kabuk_vurgu: 0xe08a6b,
            kabuk_seçili_zemin: 0x3d2a22,
        },
        // Yüksek karşıtlık kiplerinde katalog kabuğu ile tezgâh yüzeyi
        // arasında ton farkı bırakılmaz: ara tonlar tam da bu kipte
        // istenmeyen karşıtlık kaybını üretir. Bu bir türetme değil, kip
        // için verilmiş ayrı bir karardır.
        (GaleriTeması::Kağıt, TemaKipi::YüksekKarşıtlıkAçık) => GaleriPaleti {
            kip,
            kağıt: 0xffffff,
            yüzey: 0xffffff,
            kenarlık: 0x1d1a16,
            ince: 0x6d6659,
            ana_metin: 0x000000,
            ikincil_metin: 0x2c2822,
            soluk: 0x4a453d,
            vurgu: 0x8f2f10,
            vurgu_zemin: 0xffe6dc,
            kod_zemin: 0x000000,
            kod_metin: 0xcfcac2,
            olumlu: 0x1f4d2e,
            tehlike: 0x8c0f08,
            uyarı: 0x5c4406,
            bilgi: 0x14483c,
            gölge: 0x1d1a16,
            kabuk_zemin: 0xffffff,
            kabuk_kart: 0xffffff,
            kabuk_kenarlık: 0x1d1a16,
            kabuk_ana_metin: 0x000000,
            kabuk_ikincil_metin: 0x2c2822,
            kabuk_vurgu: 0x8f2f10,
            kabuk_seçili_zemin: 0xffe6dc,
        },
        (GaleriTeması::Kağıt, TemaKipi::YüksekKarşıtlıkKoyu) => GaleriPaleti {
            kip,
            kağıt: 0x000000,
            yüzey: 0x0b0b0a,
            kenarlık: 0xf2efe9,
            ince: 0x8c877e,
            ana_metin: 0xffffff,
            ikincil_metin: 0xded9d1,
            soluk: 0xb0aaa1,
            vurgu: 0xffab8a,
            vurgu_zemin: 0x4a1e0c,
            kod_zemin: 0x000000,
            kod_metin: 0xded9d1,
            olumlu: 0xa7dcb6,
            tehlike: 0xffb0a6,
            uyarı: 0xf0cf87,
            bilgi: 0x9adfcd,
            gölge: 0x1d1a16,
            kabuk_zemin: 0x000000,
            kabuk_kart: 0x0b0b0a,
            kabuk_kenarlık: 0xf2efe9,
            kabuk_ana_metin: 0xffffff,
            kabuk_ikincil_metin: 0xded9d1,
            kabuk_vurgu: 0xffab8a,
            kabuk_seçili_zemin: 0x4a1e0c,
        },
        (GaleriTeması::Mürekkep, TemaKipi::Açık) => GaleriPaleti {
            kip,
            kağıt: 0xeef1f7,
            yüzey: 0xffffff,
            kenarlık: 0xccd4e2,
            ince: 0xdde3ee,
            ana_metin: 0x131a26,
            ikincil_metin: 0x55607a,
            soluk: 0x94a0b8,
            vurgu: 0x3046b8,
            vurgu_zemin: 0xe3e8fb,
            kod_zemin: 0x161c28,
            kod_metin: 0x8b97ae,
            olumlu: 0x047857,
            tehlike: 0xb91c1c,
            uyarı: 0xb45309,
            bilgi: 0x2563eb,
            gölge: 0x000000,
            kabuk_zemin: 0xf3f5fa,
            kabuk_kart: 0xffffff,
            kabuk_kenarlık: 0xdfe5ef,
            kabuk_ana_metin: 0x111827,
            kabuk_ikincil_metin: 0x4b5563,
            kabuk_vurgu: 0x3046b8,
            kabuk_seçili_zemin: 0xeef1ff,
        },
        (GaleriTeması::Mürekkep, TemaKipi::Koyu) => GaleriPaleti {
            kip,
            kağıt: 0x14181f,
            yüzey: 0x1c222c,
            kenarlık: 0x2f3846,
            ince: 0x262d38,
            ana_metin: 0xe8edf6,
            ikincil_metin: 0xa3aec4,
            soluk: 0x6e7a90,
            vurgu: 0x8fa4ff,
            vurgu_zemin: 0x232c4a,
            kod_zemin: 0x0e1218,
            kod_metin: 0x8b97ae,
            olumlu: 0x6ee7b7,
            tehlike: 0xfca5a5,
            uyarı: 0xfcd34d,
            bilgi: 0x93c5fd,
            gölge: 0x000000,
            kabuk_zemin: 0x10141a,
            kabuk_kart: 0x181d25,
            kabuk_kenarlık: 0x28303c,
            kabuk_ana_metin: 0xe8edf6,
            kabuk_ikincil_metin: 0xa3aec4,
            kabuk_vurgu: 0x8fa4ff,
            kabuk_seçili_zemin: 0x232c4a,
        },
        (GaleriTeması::Mürekkep, TemaKipi::YüksekKarşıtlıkAçık) => GaleriPaleti {
            kip,
            kağıt: 0xffffff,
            yüzey: 0xffffff,
            kenarlık: 0x131a26,
            ince: 0x55607a,
            ana_metin: 0x000000,
            ikincil_metin: 0x1d2433,
            soluk: 0x3d4761,
            vurgu: 0x1e2f9e,
            vurgu_zemin: 0xdfe4ff,
            kod_zemin: 0x000000,
            kod_metin: 0xc9cfdd,
            olumlu: 0x0b3d24,
            tehlike: 0x7a0c0c,
            uyarı: 0x4a3505,
            bilgi: 0x0c2f6b,
            gölge: 0x131a26,
            kabuk_zemin: 0xffffff,
            kabuk_kart: 0xffffff,
            kabuk_kenarlık: 0x131a26,
            kabuk_ana_metin: 0x000000,
            kabuk_ikincil_metin: 0x1d2433,
            kabuk_vurgu: 0x1e2f9e,
            kabuk_seçili_zemin: 0xdfe4ff,
        },
        (GaleriTeması::Mürekkep, TemaKipi::YüksekKarşıtlıkKoyu) => GaleriPaleti {
            kip,
            kağıt: 0x000000,
            yüzey: 0x0a0c10,
            kenarlık: 0xeef1f7,
            ince: 0x8a93a8,
            ana_metin: 0xffffff,
            ikincil_metin: 0xdde3ee,
            soluk: 0xaab3c6,
            vurgu: 0xb9c4ff,
            vurgu_zemin: 0x1a2350,
            kod_zemin: 0x000000,
            kod_metin: 0xdde3ee,
            olumlu: 0xa7f3d0,
            tehlike: 0xffb4b4,
            uyarı: 0xffe08a,
            bilgi: 0xb6d4ff,
            gölge: 0x131a26,
            kabuk_zemin: 0x000000,
            kabuk_kart: 0x0a0c10,
            kabuk_kenarlık: 0xeef1f7,
            kabuk_ana_metin: 0xffffff,
            kabuk_ikincil_metin: 0xdde3ee,
            kabuk_vurgu: 0xb9c4ff,
            kabuk_seçili_zemin: 0x1a2350,
        },
    }
}

thread_local! {
    /// Bu karede çizilen palet.
    static ETKİN: Cell<GaleriPaleti> =
        const { Cell::new(galeri_paleti(GaleriTeması::Kağıt, TemaKipi::Açık)) };
}

/// Kare başında paleti kurar.
pub fn paleti_kur(palet: GaleriPaleti) {
    ETKİN.with(|hücre| hücre.set(palet));
}

/// Bu karede etkin palet.
pub fn palet() -> GaleriPaleti {
    ETKİN.with(|hücre| hücre.get())
}

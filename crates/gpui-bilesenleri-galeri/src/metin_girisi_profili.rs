//! `BİL-010`'un tezgâh profili.
//!
//! Tezgâh kabuğu (`src/tezgah.rs`) bileşen-bağımsızdır: hiçbir `BİL-*` tipini
//! tanımaz. Bu dosya sınırın diğer yanıdır — `TezgahTercihleri`nden kabuğun
//! anladığı `Tezgahİçeriği`ni üretir. Yeni bir aile yeniden yazıldığında
//! kendi profil dosyasını yazar; kabuk değişmez.
//!
//! **Tür süzgeci burada uygulanır** (`§9`). Kabuk "bu bölüm bu türde
//! kurulabilir mi" sorusunu sormaz; bölüm listesi zaten süzülmüş gelir:
//! seçili türde *kurulamayan* eksen hiç bölüm üretmez. Kurulabilen ama o
//! türde *kapanan* eksen ise bölümde kalır ve pasif çizilir — bu ayrım
//! `raporlar/TEZGAH_EKSEN_DAGITIM_HARITASI.md` §4'te satır satır kayıtlıdır.
//!
//! Uygulama sırası (harita §7): adım 1–4 tamamdır — on bir bölümün tamamı
//! burada kurulur, kapanan eksenler pasif ve gerekçeli çizilir, yüzer panel
//! mekanizması kalkmıştır. Kalan iş gövdeye bağlanmadır (adım 5).

use gpui::{Context, Entity, prelude::*};
use gpui_bilesenleri::GirişKutusu;

use crate::{
    Akış, GaleriUygulaması, TezgahBölümü, TezgahTercihleri, Tezgahİçeriği, anahtar,
    ÇözülmüşSaatDilimi,
};

/// Profilin çizim için ihtiyaç duyduğu her şey.
///
pub struct MetinGirişiProfilGirdisi<'a> {
    pub tercih: &'a TezgahTercihleri,
    pub alan: Entity<GirişKutusu>,
    /// Tezgâhın panel entity'leri: alan gözleyen üç panel ve önbellekli
    /// bölüm kolonu.
    ///
    /// Profil bu kartları kendisi çizmez; alan gözleyen paneller alanın
    /// bildirimiyle, bölüm kolonu kökün bildirimiyle tazelenir. Sağ kolonun
    /// girdileri (alanlar, portlar, rapor, saat dilimi) artık bu yapının
    /// değil, bölüm panelinin çizim yolunun işidir (`tezgah_bölümleri`).
    pub paneller: &'a crate::TezgahPanelleri,
    /// Kod panelinin metni; tercih sürümüne bağlı, kökten hazır gelir.
    pub kod: gpui::SharedString,
    pub en_fazla_yarıçap: f32,
    pub köşe_izi: std::rc::Rc<std::cell::Cell<gpui::Bounds<gpui::Pixels>>>,
}

/// Solda ana eksen, sağda onu tamamlayan grup.
fn ikili_satır(sol: impl IntoElement, sağ: impl IntoElement) -> gpui::Div {
    gpui::div()
        .flex()
        .items_start()
        .justify_between()
        .gap_3()
        .child(sol)
        .child(sağ)
}

/// `§16.2` gösterge kutusu: ankraj ve açıklama yüzeyi.
fn gösterge_kutusu(
    tercih: &TezgahTercihleri,
    bağlam: &mut Context<GaleriUygulaması>,
) -> gpui::Div {
    let g = crate::görünüm();
    let t = crate::TezgahTokenları::paletten(crate::palet());
    crate::kart(&g, &t).child(crate::sergiler::gösterge_satırı(tercih, bağlam))
}

/// Yaşayan alanın kabuğu.
///
/// `§7.2` önizleme kabuğu: yükseklik `ORT-017` profilinden gelir ve burada
/// ham piksel olarak tekrarlanmaz.
fn önizleme_kabuğu(alan: &Entity<GirişKutusu>) -> gpui::Div {
    let g = crate::görünüm();
    gpui::div()
        .flex()
        .items_center()
        .w_full()
        .min_h(g.önizleme_kabuğu.yükseklik)
        .child(alan.clone())
}

/// Sol kolonun ek kartları: `C` türetilmiş durumlar ve `D` aile kataloğu.
///
/// Bunlar yapılandırma ekseni değil, **bağlam**tır: biri modelden türer,
/// diğeri önizlemenin font çözümünü raporlar. Sağ kolona konsalardı
/// eksenlerle aynı görsel ağırlığı taşır ve seçilebilir sanılırlardı.
fn sol_kartlar(
    tercih: &TezgahTercihleri,
    paneller: &crate::TezgahPanelleri,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Vec<gpui::AnyElement> {
    use crate::sergiler::{kabuk_yuvaları, parça_tipografisi, yazı_biçimi_şeridi};

    vec![
        // `§7.1` parça tipografisi ve kabukta bulunan yuvalar. Yaşayan
        // alanın **altında** ve kaydırılabilir bölümde: sabit blokta yalnız
        // şekil, hizalama ve alanın kendisi kalır, kaydırma alanın hemen
        // altından başlar.
        ikili_satır(
            parça_tipografisi(tercih, bağlam),
            yazı_biçimi_şeridi(tercih, bağlam),
        )
        .into_any_element(),
        // Kartın alan okuyan notu kendi gözleyen panelindedir; kökün çizim
        // yolunda alan okuması kalmaz.
        kabuk_yuvaları(tercih, bağlam)
            .child(paneller.yuva_notu.clone())
            .into_any_element(),
        // `ORT-004 §20.1` imleç tercihi de kayan bölümde: sabit blokta
        // yalnız şekil, hizalama ve yaşayan alan kalır.
        crate::sergiler::imleç_satırı(tercih, bağlam).into_any_element(),
        // `§16.2` ankraj ve açıklama yüzeyi; bunlar da kayar.
        gösterge_kutusu(tercih, bağlam).into_any_element(),
        // `C` türetilmiş durumlar + `§13/§19` değer üçlüsü ve `§26` olay
        // akışı kendi entity'lerinde: alanın durumunu onlar okur, alanın
        // bildirimini onlar dinler. Kartların kendisi panellerin çizimidir.
        paneller.alan_durumu.clone().into_any_element(),
        paneller.olay_akışı.clone().into_any_element(),
    ]
}

/// `bölümler` girdisi.
///
/// Dokuz ayrı parametre çağrı yerinde sırayı ezberlemeyi gerektiriyordu ve
/// aynı türden iki `&[String]`/`bool` yan yana geldiğinde derleyici de
/// karışıklığı yakalayamazdı. Kurulumu kökün `tezgah_bölümleri` yolu yapar;
/// bölüm paneli çizimde, testler tür süzgeci kanıtlarında oradan okur.
pub(crate) struct BölümGirdisi<'a> {
    pub tercih: &'a TezgahTercihleri,
    pub alanlar: &'a crate::MetinGirişiAlanları,
    pub saat_dilimi: &'a ÇözülmüşSaatDilimi,
    pub doldurma_var: bool,
    pub portlar: PortDurumu,
    pub sayısal: bool,
    /// `§29` kanonik doğrulama raporu.
    ///
    /// Kart raporu **kurmaz**, hazır alır: yapılandırmayı ikinci kez kurmak
    /// ekranda gösterilenle uygulanan arasında sessiz bir fark açardı.
    /// Rapor tercih sürümüne bağlıdır; kök yalnız tercih değişince kurar.
    pub rapor: &'a gpui_bilesenleri::GirişYapılandırmaRaporu,
}

/// `B` bölümünün port kapıları.
///
/// `ACC-005`: port yoksa kontrol pasif ve gerekçeli kalır; sahte bir düğme
/// gösterilmez. Tercihin tek başına yetmediği yer burasıdır — bağlı port ve
/// verilmiş izin ister.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PortDurumu {
    /// `§6.1` platform otomatik doldurma yeteneği.
    pub otomatik_doldurma: bool,
    /// `ORT-002 §5.2` platform saat dilimi portu.
    pub saat_dilimi: bool,
    /// `ORT-004 §20.1` platform imleç portu.
    pub imleç: bool,
    /// `§15`/`ORT-007` eşzamansız doğrulama portu.
    pub uzak_doğrulama: bool,
}

/// Bölüm başlığı anahtarı; galerinin var olan `galeri.*` desenini sürdürür.
fn bölüm_anahtarı(kimlik: &str) -> gpui_bilesenleri_kabuk::YerelleştirmeAnahtarı {
    anahtar(&format!("galeri.tezgah.bölüm.{kimlik}"))
}

/// `TezgahTercihleri`nden kabuğun anladığı içeriği üretir.
pub fn tezgah_içeriği(
    girdi: MetinGirişiProfilGirdisi<'_>,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Tezgahİçeriği {
    let MetinGirişiProfilGirdisi {
        tercih,
        alan,
        paneller,
        kod,
        en_fazla_yarıçap,
        köşe_izi,
    } = girdi;
    let sayısal = tercih.sayısal_mı();

    Tezgahİçeriği {
        başlık: anahtar("galeri.tezgah.başlık"),
        önizleme_başlığı: anahtar("galeri.tezgah.önizleme"),
        önizleme: önizleme_blokları(tercih, &alan, en_fazla_yarıçap, köşe_izi, sayısal, bağlam),
        // Tasarımın `§5` şeması sol kolona `önizleme → C türetilmiş
        // durumlar → kod paneli` sırasını veriyor. `D` aile kataloğu da
        // önizleme bağlamıdır ve aynı kolonda durur; ikisi de sağ kolonun
        // yapılandırma eksenleri arasına karışmaz.
        sol_ek: sol_kartlar(tercih, paneller, bağlam),
        kod: Some(crate::sergiler::kod_paneli(kod).into_any_element()),
        // Sağ kolon önbellekli bölüm paneli: bölümler orada, kökün
        // bağlamında kurulur ve kök bildirmedikçe yeniden kurulmaz.
        yapılandırma: crate::BölümlerPaneli::öğe(&paneller.bölümler),
    }
}

/// Sol kolonun blokları: kabuk denetimleri, yaşayan alan ve tipografi.
///
/// Yaşayan alan gerçek `GirişKutusu`dur; taklit çizilmez.
#[allow(clippy::too_many_arguments)]
fn önizleme_blokları(
    tercih: &TezgahTercihleri,
    alan: &Entity<GirişKutusu>,
    en_fazla_yarıçap: f32,
    köşe_izi: std::rc::Rc<std::cell::Cell<gpui::Bounds<gpui::Pixels>>>,
    sayısal: bool,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Vec<gpui::AnyElement> {
    use crate::sergiler::{
        dikey_hizalama_şeridi, köşe_şeridi, yardımcı_eylem_şeridi, yatay_hizalama_şeridi,
    };

    vec![
        // `ORT-003` kutu şekli ve yarıçapı.
        // Taslakta üç satır **ikişer gruplu**: solda ana eksen, sağda onu
        // tamamlayan grup. Ayrı satırlara bölmek sol kolonu iki katına
        // uzatıyor ve yaşayan alanı ekranın dışına itiyordu.
        ikili_satır(
            köşe_şeridi(tercih, en_fazla_yarıçap, köşe_izi, bağlam),
            yardımcı_eylem_şeridi(tercih, sayısal, bağlam),
        )
        .into_any_element(),
        ikili_satır(
            yatay_hizalama_şeridi(tercih, sayısal, bağlam),
            dikey_hizalama_şeridi(tercih, bağlam),
        )
        .into_any_element(),
        // `§6` yaşayan alan: bütün tercihler doğrudan buraya uygulanır.
        //
        // Kabuk yüksekliği profilden verilir. `GirişKutusu` kendi
        // yüksekliğini içeriğinden alıyor ve flex sütununda sıfıra
        // sıkışabiliyordu — tezgâhın merkezindeki alan ekranda hiç
        // görünmüyordu.
        önizleme_kabuğu(alan).into_any_element(),
    ]
}

/// Sağ kolonun bölümleri; seçili türde kurulamayan eksen listeye girmez.
pub(crate) fn bölümler(
    girdi: BölümGirdisi<'_>,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Vec<TezgahBölümü> {
    let BölümGirdisi {
        tercih,
        alanlar,
        saat_dilimi,
        doldurma_var,
        portlar,
        sayısal,
        rapor,
    } = girdi;
    use crate::sergiler::{
        adım_satırı, biçim_satırı, bolut_satırı, bölüm_satırı, dis_dogrulama_satırı,
        dogrulama_satırı, doldurma_satırı, görünürlük_satırı, hacim_satırı, kapalı_eksen,
        maske_tanımı, metin_isleme_satırı, metin_içerik_türü_satırı, odak_satırı, port_satırı,
        saat_dilimi_satırı, secici_ve_dogrulama_satırı, tür_satırı, varsayılan_satırı,
        yapistirma_satırı, yer_tutucu_satırı, ön_ek_satırı,
    };

    let mut bölümler = Vec::new();

    // §7 · her türde kurulur, tam genişlik.
    bölümler.push(TezgahBölümü {
        kimlik: "deger_turu",
        başlık: bölüm_anahtarı("deger_turu"),
        yardım: None,
        akış: Akış::TamGenişlik,
        içerik: tür_satırı(tercih, bağlam).into_any_element(),
    });

    // §7 tür tanımı · §9 giriş maskesi, tam genişlik.
    let mut s9 = Vec::new();
    // `MetinTanımı` alt tanımı yalnız `Metin` ailesinde kurulur; başka
    // ailede eksen **yoktur**, kapanmış değildir.
    if crate::tür_ailesi(tercih.değer_türü) == crate::TezgahAilesi::Metin {
        s9.push(metin_içerik_türü_satırı(tercih, bağlam).into_any_element());
    }
    // `§9` şablon düzenleyici yalnız `Özel…` biçiminde açılır. Hazır bir
    // desen seçiliyken düzenleyiciye gerek yok — desen listeden geliyor —
    // ve sayısal türde maske hiç kurulamaz. Koşulsuz çizildiğinde Tamsayı
    // alanında da hazır desen düğmeleri aktif görünüyor, basınca desen
    // kutusuna yazıyor ama maske kurulmuyordu.
    s9.push(
        if tercih.seçili_biçim().uygulama == crate::BiçimUygulaması::ÖzelDesen {
            maske_tanımı(alanlar, bağlam).into_any_element()
        } else {
            crate::sergiler::maske_özeti(tercih, bağlam).into_any_element()
        },
    );
    s9.push(if tercih.bölüm_gezinimi_anlamlı_mı() {
        bölüm_satırı(tercih, bağlam).into_any_element()
    } else {
        // Eksen vardır, yalnız bölümsüz maskede kapanır: gizlemek "bölüm
        // gezinimi diye bir şey yok" derdi.
        kapalı_eksen("Bölüm gezinimi", "yalnız bölümlü maskede kurulur").into_any_element()
    });
    bölümler.push(TezgahBölümü {
        kimlik: "tur_tanimi_ve_maske",
        başlık: bölüm_anahtarı("tur_tanimi_ve_maske"),
        yardım: None,
        akış: Akış::TamGenişlik,
        içerik: gpui::div()
            .flex()
            .flex_col()
            .children(s9)
            .into_any_element(),
    });

    // §8 biçim profili · ORT-008.
    bölümler.push(TezgahBölümü {
        kimlik: "bicim_profili",
        başlık: bölüm_anahtarı("bicim_profili"),
        yardım: None,
        akış: Akış::A,
        içerik: biçim_satırı(tercih, bağlam).into_any_element(),
    });

    // §6 ön ek ve son ek · Sabitİçerik sunum rolü — her türde kurulur.
    bölümler.push(TezgahBölümü {
        kimlik: "on_ek_son_ek",
        başlık: bölüm_anahtarı("on_ek_son_ek"),
        yardım: None,
        akış: Akış::A,
        içerik: ön_ek_satırı(tercih, alanlar, bağlam).into_any_element(),
    });

    // §6 harf dönüşümü, kırpma, boş metin. Her türde bölüm üretilir; harf
    // dönüşümü sayısal türde kapanır.
    bölümler.push(TezgahBölümü {
        kimlik: "metin_isleme",
        başlık: bölüm_anahtarı("metin_isleme"),
        yardım: None,
        akış: Akış::A,
        içerik: metin_isleme_satırı(tercih, sayısal, bağlam).into_any_element(),
    });

    // §10 yapıştırma · taslakta ayrı kart, `akis-b`de: dört seçenek alt
    // alta durduğu için tam genişlik bir bölümde dikey yer israfı oluyordu.
    bölümler.push(TezgahBölümü {
        kimlik: "yapistirma",
        başlık: bölüm_anahtarı("yapistirma"),
        yardım: None,
        akış: Akış::B,
        içerik: yapistirma_satırı(tercih, bağlam).into_any_element(),
    });

    // §29 yapılandırma doğrulaması. Tasarımda `akis-c`nin son kartı; tam
    // genişlik yalnız §7 ve §9'a ayrılmıştır (`§5` yerleşimi).
    bölümler.push(TezgahBölümü {
        kimlik: "yapilandirma_dogrulamasi",
        başlık: bölüm_anahtarı("yapilandirma_dogrulamasi"),
        yardım: None,
        akış: Akış::C,
        içerik: dogrulama_satırı(rapor).into_any_element(),
    });

    // §16 dış sorunlar · temizleme politikası ve gösterim beslemesi.
    bölümler.push(TezgahBölümü {
        kimlik: "dis_dogrulama",
        başlık: bölüm_anahtarı("dis_dogrulama"),
        yardım: None,
        akış: Akış::C,
        içerik: dis_dogrulama_satırı(tercih, bağlam).into_any_element(),
    });

    // B bölümü · port kapıları. Kart her zaman çizilir: kapalı bir portu
    // gizlemek, o yolun hiç olmadığı izlenimi verirdi (`ACC-005`).
    bölümler.push(TezgahBölümü {
        kimlik: "port_kapilari",
        başlık: bölüm_anahtarı("port_kapilari"),
        yardım: None,
        akış: Akış::B,
        içerik: port_satırı(portlar, bağlam).into_any_element(),
    });

    // §23 bitişik bölüt ve arama gönderimi.
    bölümler.push(TezgahBölümü {
        kimlik: "bolut_ve_gonderim",
        başlık: bölüm_anahtarı("bolut_ve_gonderim"),
        yardım: None,
        akış: Akış::A,
        içerik: bolut_satırı(tercih, bağlam).into_any_element(),
    });

    // §9.7–9.8 hacim ve sayaç — sayısal türde eksen **kapanır**, kurulamaz
    // değildir: bölüm üretilir, içeriği pasif çizilir.
    bölümler.push(TezgahBölümü {
        kimlik: "hacim_ve_sayac",
        başlık: bölüm_anahtarı("hacim_ve_sayac"),
        yardım: None,
        akış: Akış::B,
        içerik: hacim_satırı(tercih, sayısal, bağlam).into_any_element(),
    });

    // §22 içerik görünürlüğü — aynı gerekçeyle her türde bölüm üretir.
    bölümler.push(TezgahBölümü {
        kimlik: "icerik_gorunurlugu",
        başlık: bölüm_anahtarı("icerik_gorunurlugu"),
        yardım: None,
        akış: Akış::B,
        içerik: görünürlük_satırı(tercih, sayısal, bağlam).into_any_element(),
    });

    // §24 seçici · §20.1 erişilebilirlik — bugün yalnız yer tutucu ekseni.
    bölümler.push(TezgahBölümü {
        kimlik: "secici_ve_erisim",
        başlık: bölüm_anahtarı("secici_ve_erisim"),
        yardım: None,
        akış: Akış::C,
        içerik: gpui::div()
            .child(yer_tutucu_satırı(tercih, bağlam))
            .child(
                gpui::div()
                    .mt_3()
                    .child(secici_ve_dogrulama_satırı(tercih, bağlam)),
            )
            .into_any_element(),
    });

    // §9.6 sayısal adım — metin türünde **kurulamaz**, bölüm hiç üretilmez.
    if sayısal {
        bölümler.push(TezgahBölümü {
            kimlik: "sayisal_adim",
            başlık: bölüm_anahtarı("sayisal_adim"),
            yardım: None,
            akış: Akış::C,
            içerik: adım_satırı(tercih, bağlam).into_any_element(),
        });
    }

    // §17–20 odak, kabul ve erişim. §14 varsayılan değer tarih/saat/süre
    // türünde kurulamaz; o türde satır üretilmez.
    let mut s17 = vec![odak_satırı(tercih, bağlam).into_any_element()];
    if tercih.varsayılan_uygulanabilir_mi() {
        s17.push(varsayılan_satırı(tercih, bağlam).into_any_element());
    }
    bölümler.push(TezgahBölümü {
        kimlik: "odak_ve_kabul",
        başlık: bölüm_anahtarı("odak_ve_kabul"),
        yardım: None,
        akış: Akış::C,
        içerik: gpui::div()
            .flex()
            .flex_col()
            .children(s17)
            .into_any_element(),
    });

    // §6.1 otomatik doldurma · B bölümü. Port kapısı ekseni gizlemez.
    bölümler.push(TezgahBölümü {
        kimlik: "otomatik_doldurma",
        başlık: bölüm_anahtarı("otomatik_doldurma"),
        yardım: None,
        akış: Akış::B,
        içerik: if doldurma_var {
            doldurma_satırı(tercih, bağlam).into_any_element()
        } else {
            // `YÖN-006.ACC-005`: desteklenmeyen capability görünür ve
            // dürüsttür. Port yoksa alan olağan manuel girişe döner; bunu
            // gizlemek yeteneğin hiç olmadığını söylerdi.
            kapalı_eksen("Otomatik doldurma", "platform yetenek portu bağlı değil")
                .into_any_element()
        },
    });

    // ORT-002 saat dilimi — yalnız tarih/saat biçimlerinde görünür etkiye
    // sahiptir; metin alanında **kurulamaz**.
    if tercih.tarih_türü_mü() {
        bölümler.push(TezgahBölümü {
            kimlik: "saat_dilimi",
            başlık: bölüm_anahtarı("saat_dilimi"),
            yardım: None,
            akış: Akış::C,
            içerik: saat_dilimi_satırı(tercih, saat_dilimi, bağlam).into_any_element(),
        });
    }

    bölümler
}

#[cfg(test)]
mod testler {
    use super::*;

    /// Sözlükte ölü kayıt kalmaz.
    ///
    /// Bir bölüm ekrandan kalkınca kaydı sözlükte kalıyor ve kimse fark
    /// etmiyordu: iki ölü kayıt böyle birikmişti, biri de ekrandan
    /// kaldırılan `D ·` numaralandırmasını taşıyordu. Bilinmeyen anahtar
    /// ekranda ham hâliyle görünüyor, yani ters yön zaten yakalanıyor;
    /// yakalanmayan yön buydu.
    #[test]
    fn bölüm_sözlüğünde_ölü_kayıt_yok() {
        let kaynak = include_str!("galeri.rs");
        let profil = include_str!("metin_girisi_profili.rs");
        // Alan gözleyen kartların başlıkları panel entity'lerinde çözülür;
        // oradaki kullanım da sözlük kaydını canlı tutar.
        let paneller = include_str!("paneller.rs");
        let ön_ek = "\"galeri.tezgah.bölüm.";
        for satır in kaynak.lines() {
            let Some(kalan) = satır.split(ön_ek).nth(1) else {
                continue;
            };
            let Some(kimlik) = kalan.split('"').next() else {
                continue;
            };
            assert!(
                profil.contains(&format!("kimlik: \"{kimlik}\""))
                    || profil.contains(&format!("çöz(\"{kimlik}\")"))
                    || paneller.contains(&format!("galeri.tezgah.bölüm.{kimlik}")),
                "sözlükte ölü kayıt: {kimlik}"
            );
        }
    }

    /// Bölüm anahtarları galerinin var olan `galeri.*` desenini sürdürür.
    ///
    /// Kimlikler kararlıdır: çapa gezintisi ve yerelleştirme kataloğu
    /// bunlara bağlanır.
    #[test]
    fn bölüm_anahtarı_galeri_desenini_sürdürür() {
        assert_eq!(
            bölüm_anahtarı("deger_turu").as_ref(),
            "galeri.tezgah.bölüm.deger_turu"
        );
        assert_eq!(
            bölüm_anahtarı("sayisal_adim").as_ref(),
            "galeri.tezgah.bölüm.sayisal_adim"
        );
    }
}

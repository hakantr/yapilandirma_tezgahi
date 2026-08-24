//! `F3` kalan A bölümü eksenleri: tercih → kanonik alan → kod paneli.
//!
//! Her eksen için üç şey sınanır: varsayılan tabanı bozmadığı, seçimin
//! kanonik alana doğru çevrildiği ve kod panelinde göründüğü. Üçüncüsü
//! önemli — panelde görünmeyen bir eksen, kullanıcı oynattığında kodun
//! sabit kalması demektir.

#![allow(non_ascii_idents)]

use gpui_bilesenleri::{
    BitişikBölütTürü, BoşMetinPolitikası, EscapeDavranışı, GeçerlilikTetikleyicisi,
    GeçerlilikÖnemi, GeçersizOdakDavranışı, HarfDönüşümü, KırpmaPolitikası,
    MetinYapıştırmaDönüşümü, SeçiciGörünürlüğü, ÖrnekKimliğiFabrikası,
};
use gpui_bilesenleri_galeri::{TezgahBölütü, TezgahDeğerKipi, TezgahTercihleri, TezgahYapıştırması};

fn kimlik_fabrikası() -> ÖrnekKimliğiFabrikası {
    ÖrnekKimliğiFabrikası::yeni_süreç_kapsamı().expect("test kimlik kapsamı")
}

/// `§6`/`§10`/`§17` eksenlerinin varsayılanı kanonik tabanı bozmaz.
///
/// Galeri hiçbir varsayılan icat etmemeli: tezgâhın tabanı
/// `GirişYapılandırması::tek_satırlı_metin()` ne veriyorsa odur.
#[test]
fn varsayilanlar_kanonik_tabani_bozmaz() {
    let taban = gpui_bilesenleri::GirişYapılandırması::tek_satırlı_metin();
    let y = TezgahTercihleri::default().yapılandırma(&kimlik_fabrikası());

    assert_eq!(y.harf_dönüşümü, taban.harf_dönüşümü);
    assert_eq!(y.kırpma, taban.kırpma);
    assert_eq!(y.boş_metin, taban.boş_metin);
    assert_eq!(y.escape, taban.escape);
    assert_eq!(y.geçersiz_odak, taban.geçersiz_odak);
}

/// `§6` metin işleme eksenleri kanonik alanlara çevrilir.
#[test]
fn metin_isleme_eksenleri_cevrilir() {
    let mut t = TezgahTercihleri::default();
    t.harf_dönüşümü = HarfDönüşümü::SözcükBaşı;
    t.kırpma = KırpmaPolitikası::HerZamanKırp;
    t.boş_metin = BoşMetinPolitikası::Reddet;

    let y = t.yapılandırma(&kimlik_fabrikası());
    assert_eq!(y.harf_dönüşümü, HarfDönüşümü::SözcükBaşı);
    assert_eq!(y.kırpma, KırpmaPolitikası::HerZamanKırp);
    assert_eq!(y.boş_metin, BoşMetinPolitikası::Reddet);

    let kod = t.kod();
    for beklenen in ["SözcükBaşı", "HerZamanKırp", "Reddet"] {
        assert!(
            kod.contains(beklenen),
            "`{beklenen}` koda yazılmıyor:\n{kod}"
        );
    }
}

/// `§10` yapıştırma dönüşümü, dil etiketleri dâhil çevrilir.
///
/// `TanımlıYerelAyarlarıDene` bir `Vec<DilEtiketi>` taşır. Etiketler sabit:
/// serbest metin olsaydı geçersiz bir etiket `ORT-002` doğrulamasından döner
/// ve eksen çalışmıyormuş gibi görünürdü.
#[test]
fn yapistirma_dil_etiketleriyle_cevrilir() {
    let mut t = TezgahTercihleri::default();
    t.yapıştırma = TezgahYapıştırması::TanımlıYerelAyarlarıDene;

    let y = t.yapılandırma(&kimlik_fabrikası());
    match y.yapıştırma {
        MetinYapıştırmaDönüşümü::TanımlıYerelAyarlarıDene { yerel_ayarlar } => {
            let etiketler: Vec<&str> = yerel_ayarlar.iter().map(|e| e.0.as_ref()).collect();
            assert_eq!(etiketler, ["tr-TR", "en-US"], "sıra anlamlıdır");
        }
        // `MetinYapıştırmaDönüşümü` `Debug` türetmez; varyantı adlandırmak
        // için yazdırmaya çalışmıyoruz.
        _ => panic!("yapıştırma `TanımlıYerelAyarlarıDene` varyantına düşmedi"),
    }

    let kod = t.kod();
    assert!(kod.contains("DilEtiketi::yeni(\"tr-TR\")"), "{kod}");
}

/// Her yapıştırma seçeneği kanonik bir varyanta düşer.
#[test]
fn her_yapistirma_secenegi_kanonige_duser() {
    for seçenek in TezgahYapıştırması::TÜMÜ {
        let mut t = TezgahTercihleri::default();
        t.yapıştırma = seçenek;
        let çözülmüş = t.yapılandırma(&kimlik_fabrikası()).yapıştırma;
        let eşleşir = match seçenek {
            TezgahYapıştırması::Katı => matches!(çözülmüş, MetinYapıştırmaDönüşümü::Katı),
            TezgahYapıştırması::GeçerliKarakterleriSüz => {
                matches!(çözülmüş, MetinYapıştırmaDönüşümü::GeçerliKarakterleriSüz)
            }
            TezgahYapıştırması::YerelBiçimiAyıkla => {
                matches!(çözülmüş, MetinYapıştırmaDönüşümü::YerelBiçimiAyıkla)
            }
            TezgahYapıştırması::TanımlıYerelAyarlarıDene => matches!(
                çözülmüş,
                MetinYapıştırmaDönüşümü::TanımlıYerelAyarlarıDene { .. }
            ),
        };
        assert!(eşleşir, "{} kanoniğe düşmüyor", seçenek.adı());
    }
}

/// `§17` Escape ve geçersiz değerle odak kaybı çevrilir.
#[test]
fn escape_ve_gecersiz_odak_cevrilir() {
    let mut t = TezgahTercihleri::default();
    t.escape = EscapeDavranışı::DeğişiklikleriKoru;
    t.geçersiz_odak = GeçersizOdakDavranışı::OdağıKoru;

    let y = t.yapılandırma(&kimlik_fabrikası());
    assert_eq!(y.escape, EscapeDavranışı::DeğişiklikleriKoru);
    assert_eq!(y.geçersiz_odak, GeçersizOdakDavranışı::OdağıKoru);

    let kod = t.kod();
    assert!(
        kod.contains("DeğişiklikleriKoru") && kod.contains("OdağıKoru"),
        "{kod}"
    );
}

/// `§23` bölüt kuşağı yalnız en az bir bölüt seçiliyken kurulur.
#[test]
fn bolut_kusagi_bos_kurulmaz() {
    let t = TezgahTercihleri::default();
    assert!(
        t.yapılandırma(&kimlik_fabrikası())
            .bitişik_bölütler
            .is_none()
    );

    let mut t = TezgahTercihleri::default();
    t.başlangıç_bölütü = Some(TezgahBölütü::SabitMetin);
    let kuşak = t
        .yapılandırma(&kimlik_fabrikası())
        .bitişik_bölütler
        .expect("bölüt seçiliyken kuşak kurulur");
    assert_eq!(
        kuşak.başlangıç.map(|b| b.tür),
        Some(BitişikBölütTürü::Sabit)
    );
    assert!(kuşak.bitiş.is_none());

    let kod = t.kod();
    assert!(kod.contains("BitişikBölütKuşağı"), "{kod}");
}

/// `§23` yuva kademesi her iki bölüte birden uygulanır.
#[test]
fn bolut_kademesi_her_iki_boluete_uygulanir() {
    let mut t = TezgahTercihleri::default();
    t.başlangıç_bölütü = Some(TezgahBölütü::SabitMetin);
    t.bitiş_bölütü = Some(TezgahBölütü::Eylem);
    t.bölüt_kademeli = false;

    let kuşak = t
        .yapılandırma(&kimlik_fabrikası())
        .bitişik_bölütler
        .expect("kuşak kurulur");
    assert!(!kuşak.başlangıç.expect("başlangıç").opaklık_kademeli);
    assert!(!kuşak.bitiş.expect("bitiş").opaklık_kademeli);
    assert_eq!(kuşak.bitiş.expect("bitiş").tür, BitişikBölütTürü::Eylem);
}

/// `§23.3` arama gönderimi `AramayıBaşlat` yuvasına bağlıdır.
///
/// Alanı arama alanı yapan şey yuvadır; yuva yokken `arama_gönderimi =
/// Some` olmak `UyumsuzAramaGönderimi` yapılandırma hatasıdır. Tezgâh bu
/// hatayı üretmez — ekseni yuvaya bağlar.
#[test]
fn arama_gonderimi_yuvaya_baglidir() {
    let mut t = TezgahTercihleri::default();
    t.arama = false;
    assert!(
        t.yapılandırma(&kimlik_fabrikası())
            .arama_gönderimi
            .is_none()
    );

    t.arama = true;
    t.arama_temizleme_gönderir = true;
    let gönderim = t
        .yapılandırma(&kimlik_fabrikası())
        .arama_gönderimi
        .expect("yuva açıkken gönderim kurulur");
    assert!(gönderim.enter_gönderir && gönderim.temizleme_gönderir);

    // Kuruluş raporu temiz: eksen `UyumsuzAramaGönderimi` üretmiyor.
    let rapor = t.yapılandırma(&kimlik_fabrikası()).doğrula();
    assert!(
        !rapor
            .hatalar
            .contains(&gpui_bilesenleri::GirişYapılandırmaHatası::UyumsuzAramaGönderimi),
        "arama gönderimi yuvasız kurulmuş: {:?}",
        rapor.hatalar
    );
}

/// `§24` seçici uyarlaması yuvaya bağlıdır.
///
/// Yuva kapalıyken bir görünürlük politikası ulaşılamayan bir hattı tarif
/// ederdi. Yüzey geometrisi tezgâhın alanı değil: `ORT-006` varsayılanı
/// kullanılır, ikinci bir yüzey modeli kurulmaz.
#[test]
fn secici_uyarlamasi_yuvaya_baglidir() {
    let mut t = TezgahTercihleri::default();
    t.seçici = false;
    assert!(t.yapılandırma(&kimlik_fabrikası()).seçici.is_none());

    t.seçici = true;
    t.seçici_görünürlüğü = SeçiciGörünürlüğü::HerZamanGöster;
    let uyarlama = t
        .yapılandırma(&kimlik_fabrikası())
        .seçici
        .expect("yuva açıkken uyarlama kurulur");
    assert_eq!(uyarlama.görünürlük, SeçiciGörünürlüğü::HerZamanGöster);
    assert!(uyarlama.açılma_tetikleyicileri.is_empty());
}

/// `§15` zorunluluk kuralı listeye eklenir ve tabanı bozmaz.
#[test]
fn zorunluluk_kurali_listeye_eklenir() {
    let t = TezgahTercihleri::default();
    let taban_kural_sayısı = t.yapılandırma(&kimlik_fabrikası()).doğrulama.kurallar.len();

    let mut t = TezgahTercihleri::default();
    t.zorunlu = true;
    t.doğrulama_tetikleyicisi = GeçerlilikTetikleyicisi::Değişimde;
    t.doğrulama_önemi = GeçerlilikÖnemi::Uyarı;

    let kurallar = t.yapılandırma(&kimlik_fabrikası()).doğrulama.kurallar;
    assert_eq!(kurallar.len(), taban_kural_sayısı + 1);
    let kural = kurallar.last().expect("zorunluluk kuralı");
    assert_eq!(kural.tetikleyici, GeçerlilikTetikleyicisi::Değişimde);
    assert_eq!(kural.önem, GeçerlilikÖnemi::Uyarı);
    assert!(matches!(
        kural.kural,
        gpui_bilesenleri::GeçerlilikKuralTürü::Zorunlu
    ));

    // Kimlik sayısal adım sınırının kimliğiyle çakışmaz.
    let mut t2 = t.clone();
    t2.değer_türü = TezgahDeğerKipi::Ondalık;
    t2.türe_uyarla();
    t2.sayısal_adım = true;
    t2.adım_sınırı = true;
    let kimlikler: Vec<u64> = t2
        .yapılandırma(&kimlik_fabrikası())
        .doğrulama
        .kurallar
        .iter()
        .map(|k| k.kimlik.0)
        .collect();
    let benzersiz: std::collections::BTreeSet<u64> = kimlikler.iter().copied().collect();
    assert_eq!(kimlikler.len(), benzersiz.len(), "kural kimliği çakışıyor");
}

/// `ORT-009` erişilebilir ad kod panelinde görünür.
///
/// Yapılandırmada kuruluyordu ama panelde yoktu: kopyalanan kod adsız bir
/// alan kurardı ve alan erişilebilir ağaca adsız girerdi.
#[test]
fn erisilebilir_ad_kod_panelinde_gorunur() {
    let kod = TezgahTercihleri::default().kod();
    assert!(kod.contains("erişilebilir_ad"), "{kod}");
}

/// `§7` ekran dört kamusal aile gösterir.
///
/// Sözleşme dört aile tanımlıyor; para ve yüzde `Ondalık` ailesinin biçim
/// profilleri, tarih/saat/tarih-saat ise `TarihZaman`ın kipleri. Fiziksel
/// `TezgahDeğerKipi` hâlâ dokuz düz varyant (`§8/16` borcu), ama ekran onu
/// dokuz ayrı türmüş gibi sunmaz.
#[test]
fn dokuz_varyant_dort_aileye_duser() {
    use TezgahDeğerKipi as T;
    use gpui_bilesenleri_galeri::{TezgahAilesi as A, tür_ailesi};

    assert_eq!(tür_ailesi(T::Metin), A::Metin);
    assert_eq!(tür_ailesi(T::Tamsayı), A::Tamsayı);
    // Para ve yüzde beşinci bir tür değil.
    for tür in [T::Ondalık, T::ParaBirimi, T::Yüzde] {
        assert_eq!(tür_ailesi(tür), A::Ondalık, "{tür:?} Ondalık ailesinde");
    }
    // Tarih kipleri tek ailede toplanır.
    for tür in [T::Tarih, T::Saat, T::TarihSaat, T::Süre] {
        assert_eq!(
            tür_ailesi(tür),
            A::TarihZaman,
            "{tür:?} TarihZaman ailesinde"
        );
    }
}

/// Aile seçimi fiziksel türü ailenin varsayılanına kurar.
///
/// `Ondalık`a geçerken para profilini taşımak, ekranda `Ondalık` yazıp
/// değerde para kurmak olurdu.
#[test]
fn aile_secimi_varsayilan_turu_kurar() {
    use gpui_bilesenleri_galeri::TezgahAilesi as A;
    assert_eq!(A::Metin.varsayılan_tür(), TezgahDeğerKipi::Metin);
    assert_eq!(A::Ondalık.varsayılan_tür(), TezgahDeğerKipi::Ondalık);
    assert_eq!(A::TarihZaman.varsayılan_tür(), TezgahDeğerKipi::Tarih);
}

/// `MetinTanımı::içerik_türü` artık koda yazılır (borç 16 kapandı).
///
/// `GirişYapılandırması` `giriş_türü: GirişTürü` taşıyor; içerik türü
/// `MetinTanımı`nın içinde yapılandırmaya ve kod paneline geçer, sapma
/// yokken satır yazılmaz.
#[test]
fn metin_icerik_turu_koda_yazilir() {
    let mut t = TezgahTercihleri::default();
    t.metin_içerik_türü = gpui_bilesenleri::MetinİçerikTürü::EPosta;
    let kod = t.kod();
    assert!(
        kod.contains("içerik_türü: MetinİçerikTürü::EPosta"),
        "içerik türü koda yazılmalı:\n{kod}"
    );
    let yapılandırma = t.yapılandırma(&kimlik_fabrikası());
    assert!(matches!(
        yapılandırma.giriş_türü,
        gpui_bilesenleri::GirişTürü::Metin(tanım)
            if tanım.içerik_türü == gpui_bilesenleri::MetinİçerikTürü::EPosta
    ));

    // Varsayılan (Düz) sapma değildir; satır yazılmaz.
    assert!(!TezgahTercihleri::default().kod().contains("giriş_türü"));
}

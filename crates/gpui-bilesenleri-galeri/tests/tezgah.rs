//! `BİL-010` tezgâhı: tercihler kanonik yapılandırmaya doğru çevriliyor mu?
//!
//! Galeri hiçbir varsayılan icat etmemeli: taban her zaman
//! `GirişYapılandırması::tek_satırlı_metin()` ve tercihler yalnız onun
//! alanlarını değiştirmeli. Gösterilen kod da gerçekten uygulanan
//! yapılandırmayı anlatmalı.

#![allow(non_ascii_idents)]

use gpui_bilesenleri::{
    DüğmeŞekli, DışHataTemizleme, EnterDavranışı, GeçiciGösterimPolitikası,
    GirişDikeyHizalama,
    GirişMaskesi, GirişYatayHizalama, KabulSeçimi, KutuŞekliTercihi, MetinİçerikTürü, OdakSeçimi,
    OndalıkDeğer, SabitİçerikSunumRolü, SayımBirimi, UzunlukSınırıDavranışı,
    ÖrnekKimliğiFabrikası, İçerikGörünürlüğü,
};
use gpui_bilesenleri_galeri::{
    AdımÖlçeği, KİTAPLIK_AİLELERİ, TezgahDeğerKipi, TezgahGeçiciGösterimi, TezgahGörünürlüğü,
    TezgahMaskesi, TezgahTeması, TezgahTercihleri,
};

/// Tezgâhın tür satırında sunduğu değer türleri.
const TÜM_DEĞER_TÜRLERİ: [TezgahDeğerKipi; 9] = [
    TezgahDeğerKipi::Metin,
    TezgahDeğerKipi::Tamsayı,
    TezgahDeğerKipi::Ondalık,
    TezgahDeğerKipi::ParaBirimi,
    TezgahDeğerKipi::Yüzde,
    TezgahDeğerKipi::Tarih,
    TezgahDeğerKipi::Saat,
    TezgahDeğerKipi::TarihSaat,
    TezgahDeğerKipi::Süre,
];

const TÜM_İÇERİK_TÜRLERİ: [MetinİçerikTürü; 4] = [
    MetinİçerikTürü::Düz,
    MetinİçerikTürü::EPosta,
    MetinİçerikTürü::Telefon,
    MetinİçerikTürü::Url,
];

fn kimlik_fabrikası() -> ÖrnekKimliğiFabrikası {
    ÖrnekKimliğiFabrikası::yeni_süreç_kapsamı().expect("test kimlik kapsamı")
}

#[test]
fn varsayilan_tercih_taban_yapilandirmayi_bozmaz() {
    let t = TezgahTercihleri::default();
    let y = t.yapılandırma(&kimlik_fabrikası());
    assert!(matches!(y.giriş_türü, gpui_bilesenleri::GirişTürü::Metin(_)));
    assert!(y.etkin && !y.salt_okunur);
    assert_eq!(y.odak_seçimi, OdakSeçimi::TümünüSeç);
    assert_eq!(y.kabul_seçimi, KabulSeçimi::TümünüSeç);

    // Açılış durumu tasarımın kendisi: telefon maskesi ve `+90` ön eki
    // seçili gelir (`metinkutusu.cozulmus.html` 419-421, 453. satırlar).
    // Galeri yine de kendi varsayılanını icat etmiyor; taban hâlâ
    // `GirişYapılandırması::tek_satırlı_metin()` ve tercihler yalnız onun
    // alanlarını değiştiriyor.
    // Açılış `Düz` içerik türüyle: maske ve ülke kodu eki beklentiyi
    // daraltır ve ekranda "Düz metin" yazarken kutu telefon numarası
    // bekliyordu. İkisi de içerik türü `Telefon` seçilince kurulur.
    assert!(y.maske.is_none(), "düz metin açılışında maske kurulmamalı");
    assert!(y.ön_ek.is_none(), "düz metin açılışında ön ek kurulmamalı");
}

#[test]
fn on_ek_son_ek_ve_yer_tutucu_tercihi_yapilandirmaya_gecer() {
    let mut t = TezgahTercihleri::default();
    t.ön_ek = true;
    t.son_ek = true;
    let y = t.yapılandırma(&kimlik_fabrikası());
    assert!(y.ön_ek.is_some());
    assert!(y.son_ek.is_some());
    assert!(y.yer_tutucu.is_some());

    t.yer_tutucu = false;
    assert!(t.yapılandırma(&kimlik_fabrikası()).yer_tutucu.is_none());
}

#[test]
fn yardimci_eylem_tercihi_uc_yuva_sinirini_asmaz() {
    let mut t = TezgahTercihleri::default();
    t.temizle = true;
    t.arama = true;
    t.parola_düğmesi = true;
    t.seçici = true;
    let y = t.yapılandırma(&kimlik_fabrikası());
    let yuvalar = y.yardımcı_eylemler.expect("yuva kümesi");
    // `§23` en fazla üç yuva.
    assert_eq!(yuvalar.len(), 3);
}

#[test]
fn yardimci_eylem_kapaliyken_kume_kurulmaz() {
    let mut t = TezgahTercihleri::default();
    t.temizle = false;
    assert!(
        t.yapılandırma(&kimlik_fabrikası())
            .yardımcı_eylemler
            .is_none()
    );
}

#[test]
fn sayac_ve_uzunluk_siniri_tercihi_gecer() {
    let mut t = TezgahTercihleri::default();
    assert!(t.yapılandırma(&kimlik_fabrikası()).sayaç.is_none());
    assert!(t.yapılandırma(&kimlik_fabrikası()).uzunluk_sınırı.is_none());
    t.sayaç = true;
    t.uzunluk_sınırı = true;
    let y = t.yapılandırma(&kimlik_fabrikası());
    assert!(y.sayaç.is_some());
    assert_eq!(y.uzunluk_sınırı.unwrap().en_fazla_grafem, 12);
}

#[test]
fn gizli_icerik_tercihi_parola_maskesi_kurar() {
    let mut t = TezgahTercihleri::default();
    t.görünürlük = TezgahGörünürlüğü::Gizli;
    assert!(matches!(
        t.yapılandırma(&kimlik_fabrikası()).içerik_görünürlüğü,
        İçerikGörünürlüğü::Gizli { .. }
    ));
}

/// `§22` `GeçiciGöster` politikayla kurulur; politika başka görünürlükte
/// yazılmaz. Politikasız `GeçiciGöster` kanonik doğrulamada hatadır, tezgâh
/// o hâli hiç üretmemeli.
#[test]
fn gecici_goster_tercihi_politikayla_kurulur() {
    let mut t = TezgahTercihleri::default();
    t.görünürlük = TezgahGörünürlüğü::GeçiciGöster;
    t.geçici_gösterim = TezgahGeçiciGösterimi::ZamanSınırlı;
    let y = t.yapılandırma(&kimlik_fabrikası());
    assert!(matches!(
        y.geçici_gösterim,
        Some(GeçiciGösterimPolitikası::ZamanSınırlı { .. })
    ));
    assert!(
        y.doğrula().hatalar.is_empty(),
        "tezgâhın ürettiği GeçiciGöster yapılandırması geçerli olmalı"
    );

    // Politika yalnız `GeçiciGöster`le yazılır; başka görünürlükte
    // okunmayan bir tercih tarif ederdi.
    t.görünürlük = TezgahGörünürlüğü::Gizli;
    assert!(t.yapılandırma(&kimlik_fabrikası()).geçici_gösterim.is_none());

    // Kod paneli de politikayı yalnız `GeçiciGöster`de yazar.
    t.görünürlük = TezgahGörünürlüğü::GeçiciGöster;
    assert!(t.kod().contains("GeçiciGösterimPolitikası::ZamanSınırlı"));
    t.görünürlük = TezgahGörünürlüğü::Gizli;
    assert!(!t.kod().contains("GeçiciGösterimPolitikası"));
}

/// `§16` dış hata temizleme tercihi yapılandırmaya geçer; kod paneli yalnız
/// varsayılandan sapan `Koru`yu yazar.
#[test]
fn dis_hata_temizleme_tercihi_gecer() {
    let mut t = TezgahTercihleri::default();
    assert_eq!(
        t.yapılandırma(&kimlik_fabrikası())
            .doğrulama
            .dış_hata_temizleme,
        DışHataTemizleme::YerelDüzenlemedeTemizle
    );
    assert!(!t.kod().contains("dış_hata_temizleme"));

    t.dış_hata_temizleme = DışHataTemizleme::YenidenBildirimeKadarKoru;
    assert_eq!(
        t.yapılandırma(&kimlik_fabrikası())
            .doğrulama
            .dış_hata_temizleme,
        DışHataTemizleme::YenidenBildirimeKadarKoru
    );
    assert!(t.kod().contains("DışHataTemizleme::YenidenBildirimeKadarKoru"));
}

#[test]
fn kose_sekli_ve_hizalama_tercihi_gecer() {
    let mut t = TezgahTercihleri::default();
    t.şekil = DüğmeŞekli::Hap;
    t.hizalama = GirişYatayHizalama::Sağ;
    let y = t.yapılandırma(&kimlik_fabrikası());
    assert_eq!(y.şekil, KutuŞekliTercihi::Açık(DüğmeŞekli::Hap));
    assert_eq!(y.hizalama.yatay, GirişYatayHizalama::Sağ);
}

#[test]
fn enter_ve_erisim_tercihi_gecer() {
    let mut t = TezgahTercihleri::default();
    t.enter = EnterDavranışı::DeğeriİşleVeSonrakineGeç;
    t.salt_okunur = true;
    t.etkin = false;
    let y = t.yapılandırma(&kimlik_fabrikası());
    assert_eq!(y.enter, EnterDavranışı::DeğeriİşleVeSonrakineGeç);
    assert!(y.salt_okunur);
    assert!(!y.etkin);
}

#[test]
fn kod_yalniz_sapan_alanlari_yazar() {
    let kod = TezgahTercihleri::default().kod();
    assert!(kod.starts_with("let mut yapılandırma = GirişYapılandırması::tek_satırlı_metin();"));
    // Varsayılanda yalnız açık olan `temizle` yuvası görünür.
    assert!(kod.contains("YardımcıEylemTürü::Temizle"));
    assert!(!kod.contains("değer_türü"));
    assert!(!kod.contains("salt_okunur"));
}

#[test]
fn kod_secilen_her_tercihi_yansitir() {
    let mut t = TezgahTercihleri::default();
    t.değer_türü = TezgahDeğerKipi::Ondalık;
    t.ön_ek = true;
    t.sayaç = true;
    t.şekil = DüğmeŞekli::Hap;
    t.salt_okunur = true;
    let kod = t.kod();
    for beklenen in [
        "giriş_türü",
        "GirişTürü::Ondalık(OndalıkTanımı::default())",
        "Sabitİçerik::metin(\"₺\", false)",
        "SayaçYapılandırması",
        "DüğmeŞekli::Hap",
        "salt_okunur = true",
    ] {
        assert!(kod.contains(beklenen), "kodda eksik: {beklenen}");
    }
}

#[test]
fn desen_tercihi_metin_maskesi_kurar() {
    let mut t = TezgahTercihleri::default();
    t.maske = TezgahMaskesi::Desen;
    t.desen = ">00 L?? 00999".to_owned();
    match t.yapılandırma(&kimlik_fabrikası()).maske {
        Some(GirişMaskesi::Metin(m)) => assert_eq!(&*m.desen, ">00 L?? 00999"),
        diğer => panic!("beklenmeyen maske: {diğer:?}"),
    }
    assert!(t.kod().contains(">00 L?? 00999"));
}

#[test]
fn on_ek_ve_son_ek_metni_duzenlenebilir() {
    let mut t = TezgahTercihleri::default();
    t.ön_ek = true;
    t.ön_ek_metni = "USD".to_owned();
    t.son_ek = true;
    t.son_ek_metni = "net".to_owned();
    let y = t.yapılandırma(&kimlik_fabrikası());
    assert_eq!(y.ön_ek.unwrap().düz_metin(), "USD");
    assert_eq!(y.son_ek.unwrap().düz_metin(), "net");

    // Boş metin yuvayı kurmaz: görünmez bir ek yanıltıcı olur.
    t.ön_ek_metni.clear();
    assert!(t.yapılandırma(&kimlik_fabrikası()).ön_ek.is_none());
}

#[test]
fn maske_secenekleri_deger_turune_gore_suzulur() {
    let mut t = TezgahTercihleri::default();
    assert_eq!(
        t.maske_seçenekleri(),
        &[TezgahMaskesi::Yok, TezgahMaskesi::Desen]
    );
    // `§9.3` sayısal alanda bağımsız maske yoktur.
    t.değer_türü = TezgahDeğerKipi::Ondalık;
    assert_eq!(t.maske_seçenekleri(), &[TezgahMaskesi::Yok]);
}

#[test]
fn tur_degisince_uymayan_tercihler_kapanir() {
    let mut t = TezgahTercihleri::default();
    t.maske = TezgahMaskesi::Desen;
    t.görünürlük = TezgahGörünürlüğü::Gizli;
    t.parola_düğmesi = true;
    t.uzunluk_sınırı = true;
    t.sayaç = true;

    t.değer_türü = TezgahDeğerKipi::Ondalık;
    t.türe_uyarla();

    // Desen maskesi sayısal türde kurulamaz; metne özgü tercihler de kapanır.
    assert_eq!(t.maske, TezgahMaskesi::Yok);
    assert_eq!(t.görünürlük, TezgahGörünürlüğü::Açık);
    assert!(!t.parola_düğmesi);
    assert!(!t.uzunluk_sınırı && !t.sayaç);
    let y = t.yapılandırma(&kimlik_fabrikası());
    assert!(y.maske.is_none() && y.sayaç.is_none() && y.uzunluk_sınırı.is_none());
}

/// `§6` para tür değildir: kip `Ondalık` türe iner ve karşılığını Para
/// biçim profili verir; birim biçimin içindedir.
#[test]
fn para_kipi_bicim_profilinden_kurulur() {
    let mut t = TezgahTercihleri::default();
    t.değer_türü = TezgahDeğerKipi::ParaBirimi;
    let y = t.yapılandırma(&kimlik_fabrikası());
    assert!(matches!(y.giriş_türü, gpui_bilesenleri::GirişTürü::Ondalık(_)));
    assert!(matches!(
        y.biçim,
        gpui_bilesenleri::BiçimYapılandırması::Açık(gpui_bilesenleri::BiçimTanımı::Para(_))
    ));
    assert!(t.kod().contains("GirişTürü::Ondalık"));
}

/// Önizleme **boş** açılır: `§19` Escape'in iki dalı ancak öyle görülür.
///
/// Kutu türe uygun bir örnekle açılıyordu; gerekçe hizalama, ayraç ve
/// temizleme simgesinin ancak içerik varken görünmesiydi. Ama dolu kutuda
/// "boşa dön" ile "son kabul edilene dön" aynı görünüyor: ilk Escape
/// zaten bir değere dönüyordu. Boş açılışta ilk Escape kutuyu temizler,
/// kabulden sonraki Escape o değere döner.
#[test]
fn onizleme_bos_acilir() {
    for tür in TÜM_DEĞER_TÜRLERİ {
        let mut t = TezgahTercihleri::default();
        t.değer_türü = tür;
        t.türe_uyarla();
        assert_eq!(t.örnek_değer(), "", "{tür:?} boş açılmalı");
    }

    // Maske ve gizleme de bir şey eklemez: hepsi boş.
    let mut t = TezgahTercihleri::default();
    t.maske = TezgahMaskesi::Desen;
    assert_eq!(t.örnek_değer(), "");
    t.görünürlük = TezgahGörünürlüğü::Gizli;
    assert_eq!(t.örnek_değer(), "");
}

#[test]
fn dikey_hizalama_ve_ek_tonu_yapilandirmaya_gecer() {
    let mut t = TezgahTercihleri::default();
    t.dikey = GirişDikeyHizalama::Alt;
    t.ön_ek = true;
    t.ek_sunum_rolü = SabitİçerikSunumRolü::DeğerleEş;
    let y = t.yapılandırma(&kimlik_fabrikası());
    assert_eq!(y.hizalama.dikey, GirişDikeyHizalama::Alt);
    assert_eq!(y.ön_ek.unwrap().sunum_rolü, SabitİçerikSunumRolü::DeğerleEş);

    let kod = t.kod();
    assert!(kod.contains("GirişDikeyHizalama::Alt"));
    assert!(kod.contains("SabitİçerikSunumRolü::DeğerleEş"));
}

#[test]
fn ortali_hizalama_tercihi_gecer() {
    let mut t = TezgahTercihleri::default();
    t.hizalama = GirişYatayHizalama::Orta;
    assert_eq!(
        t.yapılandırma(&kimlik_fabrikası()).hizalama.yatay,
        GirişYatayHizalama::Orta
    );
    assert!(t.kod().contains("GirişYatayHizalama::Orta"));
}

#[test]
fn kose_pikseli_hazir_kademeyi_gecersiz_kilar() {
    let mut t = TezgahTercihleri::default();
    t.şekil = DüğmeŞekli::Hap;
    assert_eq!(
        t.yapılandırma(&kimlik_fabrikası()).şekil,
        KutuŞekliTercihi::Açık(DüğmeŞekli::Hap)
    );

    // Piksel verildiğinde kademe değil ürünün ölçüsü uygulanır.
    t.köşe_pikseli = Some(14.0);
    assert_eq!(
        t.yapılandırma(&kimlik_fabrikası()).şekil,
        KutuŞekliTercihi::Yarıçap(gpui::px(14.0))
    );
    assert!(t.kod().contains("KutuŞekliTercihi::Yarıçap(px(14.))"));
    assert!(!t.kod().contains("DüğmeŞekli::Hap"));
}

#[test]
fn dis_tiklamada_odagi_birakma_tercihi_gecer() {
    let mut t = TezgahTercihleri::default();
    // `ACC-054` varsayılan açık; kod varsayılanı yazmaz.
    assert!(
        t.yapılandırma(&kimlik_fabrikası())
            .dış_tıklamada_odağı_bırak
    );
    assert!(!t.kod().contains("dış_tıklamada_odağı_bırak"));

    t.dış_tıklamada_odağı_bırak = false;
    assert!(
        !t.yapılandırma(&kimlik_fabrikası())
            .dış_tıklamada_odağı_bırak
    );
    assert!(t.kod().contains("dış_tıklamada_odağı_bırak = false;"));
}

#[test]
fn uzunluk_davranisi_ve_sayac_birimi_yapilandirmaya_gecer() {
    let mut t = TezgahTercihleri::default();
    t.uzunluk_sınırı = true;
    t.uzunluk_davranışı = UzunlukSınırıDavranışı::Reddet;
    t.sayaç = true;
    t.sayaç_birimi = SayımBirimi::KodNoktası;
    t.sayaç_sınırı_göster = false;

    let y = t.yapılandırma(&kimlik_fabrikası());
    assert_eq!(
        y.uzunluk_sınırı.unwrap().davranış,
        UzunlukSınırıDavranışı::Reddet
    );
    let sayaç = y.sayaç.unwrap();
    assert_eq!(sayaç.birim, SayımBirimi::KodNoktası);
    assert!(!sayaç.sınırı_göster);

    let kod = t.kod();
    assert!(kod.contains("UzunlukSınırıDavranışı::Reddet"));
    assert!(kod.contains("SayımBirimi::KodNoktası"));
    assert!(kod.contains("sınırı_göster: false"));
}

#[test]
fn tezgah_temasi_kagit_paletini_tasir() {
    // Tezgâh kendi görsel dilini taşır; genel galeri teması değişmemeli.
    let tezgah = gpui_bilesenleri_galeri::tezgah_teması(&TezgahTeması::default());
    let galeri = gpui_bilesenleri_galeri::galeri_teması();
    assert_ne!(tezgah.bağlam.kimlik, galeri.bağlam.kimlik);
    assert_ne!(tezgah.renkler.yüzey, galeri.renkler.yüzey);
    // `Hap` yarıçapı kutu yüksekliğinin yarısından türer; tasarımın kutusu
    // 58 pikseldir.
    assert_eq!(tezgah.ölçüler.etkileşim_hedefi, gpui::px(58.));
}

#[test]
fn tipografi_temaya_yazilir_ve_surum_artar() {
    // `ORT-004 §4` bileşen ham font ailesi okuyamaz; tipografi temanındır.
    // Tezgâhın yazı denetimleri bu yüzden `GirişYapılandırması`na değil,
    // anlık görüntüye yazar.
    let mut tema = TezgahTeması::default();
    let ilk = gpui_bilesenleri_galeri::tezgah_teması(&tema);
    assert_eq!(ilk.bağlam.sürüm, 1);

    tema.yazı_ailesi = "Lilex".to_string();
    tema.punto = 18.;
    tema.ağırlık = gpui_bilesenleri_galeri::YazıAğırlığı::Koyu;
    tema.sürümü_artır();
    let ikinci = gpui_bilesenleri_galeri::tezgah_teması(&tema);

    // Anlık görüntü değişmezdir: yeni değer yeni sürümle gelir.
    assert_eq!(ikinci.bağlam.sürüm, 2);
    assert_ne!(
        ilk.tipografi.gövde.font_family,
        ikinci.tipografi.gövde.font_family
    );
    assert_ne!(
        ilk.tipografi.gövde.font_size,
        ikinci.tipografi.gövde.font_size
    );
    assert_ne!(
        ilk.tipografi.gövde.font_weight,
        ikinci.tipografi.gövde.font_weight
    );

    // Yapılandırma tipografi taşımaz: alan yüzeyi kirlenmedi.
    let kod = TezgahTercihleri::default().kod();
    assert!(!kod.contains("font"));
    assert!(!kod.contains("punto"));
}

#[test]
fn metin_olcegi_punto_ile_birlikte_uygulanir() {
    // `TemaBağlamı::metin_ölçeği` erişilebilirlik ölçeğidir; punto tercihinden
    // ayrıdır ve onunla çarpılır.
    let mut tema = TezgahTeması::default();
    tema.punto = 14.;
    let bir = gpui_bilesenleri_galeri::tezgah_teması(&tema);

    tema.metin_ölçeği = 1.5;
    tema.sürümü_artır();
    let buçuk = gpui_bilesenleri_galeri::tezgah_teması(&tema);

    assert_eq!(bir.bağlam.metin_ölçeği, 1.0);
    assert_eq!(buçuk.bağlam.metin_ölçeği, 1.5);
    assert_ne!(
        bir.tipografi.gövde.font_size,
        buçuk.tipografi.gövde.font_size
    );
}

#[test]
fn varsayilan_aile_kitaplikta() {
    // Varsayılan aile kayıtlı bir yüze çözülmeli: kaydı olmayan bir ad
    // (`system-ui`, `serif`) hiçbir yüze bağlanmaz ve o adla istenen kalın
    // veya eğik sessizce uygulanmaz — var olmayan bir tercih göstermiş
    // oluruz. Sistem aileleri makineye bağlı olduğu için varsayılan asla
    // onlardan seçilmez.
    let varsayılan = TezgahTeması::default().yazı_ailesi;
    assert!(
        KİTAPLIK_AİLELERİ.contains(&varsayılan.as_str()),
        "varsayılan aile kitaplıkta değil: {varsayılan}"
    );
}

#[test]
fn agirlik_ucusu_gomulu_yuzlere_baglanir() {
    // Üç ağırlığın da gömülü karşılığı var: Light 300, Regular 400,
    // SemiBold 600. `BOLD` (700) istenirse yalnız Lilex'te yüz bulunur.
    use gpui_bilesenleri_galeri::YazıAğırlığı;
    let mut tema = TezgahTeması::default();
    let normal = gpui_bilesenleri_galeri::tezgah_teması(&tema);
    tema.ağırlık = YazıAğırlığı::Koyu;
    tema.sürümü_artır();
    let kalın = gpui_bilesenleri_galeri::tezgah_teması(&tema);
    tema.ağırlık = YazıAğırlığı::İnce;
    tema.sürümü_artır();
    let ince = gpui_bilesenleri_galeri::tezgah_teması(&tema);

    assert_eq!(normal.tipografi.gövde.font_weight, gpui::FontWeight::NORMAL);
    assert_eq!(
        kalın.tipografi.gövde.font_weight,
        gpui::FontWeight::SEMIBOLD
    );
    assert_eq!(ince.tipografi.gövde.font_weight, gpui::FontWeight::LIGHT);

    // Eğik yüz de gömülü: `Italic` ve `SemiBoldItalic` var.
    tema.eğik = true;
    tema.sürümü_artır();
    let eğik = gpui_bilesenleri_galeri::tezgah_teması(&tema);
    assert_eq!(eğik.tipografi.gövde.font_style, gpui::FontStyle::Italic);
}

#[test]
fn bicim_secenegi_yapilandirmaya_gecer() {
    use gpui_bilesenleri::{BiçimTanımı, BiçimYapılandırması};
    use gpui_bilesenleri_galeri::{BiçimUygulaması, BİÇİM_SEÇENEKLERİ};

    let sıra = |aranan: BiçimUygulaması| {
        BİÇİM_SEÇENEKLERİ
            .iter()
            .position(|s| s.uygulama == aranan)
            .expect("seçenek listede")
    };

    // Maske seçenekleri gösterimi değil girişi sınırlar; biçim `Genel` kalır.
    let mut t = TezgahTercihleri::default();
    assert_eq!(
        t.yapılandırma(&kimlik_fabrikası()).biçim,
        BiçimYapılandırması::Genel
    );

    // Binlik ayraçlı sayı yalnız sayısal türde kurulur.
    let önceki = t.biçim_seçeneği;
    t.biçim_seçeneğini_uygula(sıra(BiçimUygulaması::Sayı { gruplama: true }));
    assert_eq!(
        t.biçim_seçeneği, önceki,
        "metin türünde sayısal biçim seçilememeli"
    );

    t.değer_türü = TezgahDeğerKipi::Ondalık;
    t.türe_uyarla();
    t.biçim_seçeneğini_uygula(sıra(BiçimUygulaması::Sayı { gruplama: true }));
    match t.yapılandırma(&kimlik_fabrikası()).biçim {
        BiçimYapılandırması::Açık(BiçimTanımı::Ondalık(biçim)) => {
            assert_eq!(
                biçim.basamak_gruplama,
                Some(gpui_bilesenleri::BasamakGruplama::YerelVarsayılan)
            );
            assert_eq!(
                biçim.duyarlılık,
                Some(gpui_bilesenleri::OndalıkDuyarlılık::Sabit(2))
            );
        }
        diğer => panic!("beklenmeyen biçim: {diğer:?}"),
    }

    // Tür metne dönünce sayısal biçim kalmaz.
    t.değer_türü = TezgahDeğerKipi::Metin;
    t.türe_uyarla();
    assert_eq!(
        t.yapılandırma(&kimlik_fabrikası()).biçim,
        BiçimYapılandırması::Genel
    );
}

#[test]
fn maske_secenegi_deseni_kurar() {
    use gpui_bilesenleri_galeri::{BiçimUygulaması, BİÇİM_SEÇENEKLERİ};
    let sıra = BİÇİM_SEÇENEKLERİ
        .iter()
        .position(|s| s.uygulama == BiçimUygulaması::Desen("00000"))
        .expect("posta kodu seçeneği listede");

    let mut t = TezgahTercihleri::default();
    t.biçim_seçeneğini_uygula(sıra);
    assert_eq!(t.maske, TezgahMaskesi::Desen);
    match t.yapılandırma(&kimlik_fabrikası()).maske {
        Some(GirişMaskesi::Metin(m)) => assert_eq!(&*m.desen, "00000"),
        diğer => panic!("beklenmeyen maske: {diğer:?}"),
    }
}

#[test]
fn telefon_deseninde_bastaki_sifir_sabittir() {
    use gpui_bilesenleri_galeri::{BiçimUygulaması, BİÇİM_SEÇENEKLERİ};
    // Baştaki `0` numaranın değişmez ön eki: kullanıcı yazmaz, maske
    // çizer. Kaçışsız `0` zorunlu rakam yuvası olurdu ve ilk konuma
    // herhangi bir rakam yazılabilirdi (kullanıcı kararı, Ağu 2026).
    let sıra = BİÇİM_SEÇENEKLERİ
        .iter()
        .position(|s| s.etiket.starts_with("Telefon"))
        .expect("telefon seçeneği listede");
    assert_eq!(
        BİÇİM_SEÇENEKLERİ[sıra].uygulama,
        BiçimUygulaması::Desen("\\0(000) 000 00 00"),
        "baştaki sıfır `\\` kaçışıyla sabit olmalı"
    );

    let mut t = TezgahTercihleri::default();
    t.biçim_seçeneğini_uygula(sıra);
    match t.yapılandırma(&kimlik_fabrikası()).maske {
        Some(GirişMaskesi::Metin(m)) => assert_eq!(&*m.desen, "\\0(000) 000 00 00"),
        diğer => panic!("beklenmeyen maske: {diğer:?}"),
    }
}

#[test]
fn kanonik_karsiligi_olmayan_secenek_uygulanmaz() {
    // Tasarımın listesinde `ORT-008`'de karşılığı olmayan satırlar var
    // (bilimsel, kesir, muhasebe, tarih/saat). Bunlar listede durur ama
    // seçilemez: seçilebilseler çalışmayan bir tercih göstermiş olurduk.
    use gpui_bilesenleri_galeri::{BiçimUygulaması, BİÇİM_SEÇENEKLERİ};

    let mut t = TezgahTercihleri::default();
    t.değer_türü = TezgahDeğerKipi::Ondalık;
    t.türe_uyarla();

    let mut eksik_sayısı = 0;
    for (sıra, seçenek) in BİÇİM_SEÇENEKLERİ.iter().enumerate() {
        if !matches!(seçenek.uygulama, BiçimUygulaması::Eksik(_)) {
            continue;
        }
        eksik_sayısı += 1;
        assert!(
            seçenek.eksiklik_nedeni().is_some(),
            "eksikliğin nedeni yazılmalı"
        );
        t.biçim_seçeneğini_uygula(sıra);
        assert_ne!(t.biçim_seçeneği, sıra, "eksik seçenek uygulanmamalı");
    }
    // Tarih/saat `§8.1`, bilimsel `§7.1` ve kesir `§7.2` motorlarıyla
    // kapandı; geriye yalnız muhasebe yerleşimi kaldı.
    assert_eq!(eksik_sayısı, 1);
}

#[test]
fn imlec_tercihi_tema_anlik_goruntusune_gecer() {
    use gpui_bilesenleri::{MetinİmleciHareketKaynağı, MetinİmleciHareketi};
    use gpui_bilesenleri_galeri::İmleçHızı;

    // Varsayılan `Platform`: tema hareketi ezmez, çözüm aşağı düşer.
    let mut tema = TezgahTeması::default();
    let anlık = gpui_bilesenleri_galeri::tezgah_teması(&tema);
    assert_eq!(anlık.metin_imleci.hareket, None);

    // Açık hız seçimi temanın tanımı olur ve platformun önüne geçer.
    tema.imleç_hızı = İmleçHızı::Hızlı;
    tema.imleç_kalınlığı = 3.0;
    tema.sürümü_artır();
    let anlık = gpui_bilesenleri_galeri::tezgah_teması(&tema);
    let token = anlık.imleç.expect("tezgâh imleç tokenı kurar");
    assert_eq!(token.kalınlık, gpui::px(3.0));
    let çözüm = gpui_bilesenleri::metin_imleci_hareketini_çöz(
        anlık.metin_imleci.hareket,
        anlık.bağlam.hareket,
        None,
    );
    assert!(çözüm.yanıp_söner_mi());
    assert_eq!(çözüm.kaynak, MetinİmleciHareketKaynağı::Tema);
    assert_eq!(
        çözüm.hareket,
        MetinİmleciHareketi::YanıpSönen {
            dönem: std::time::Duration::from_millis(500),
            görünür_süre: std::time::Duration::from_millis(250),
        }
    );

    // `Sabit` yanıp sönmeyi kapatır.
    tema.imleç_hızı = İmleçHızı::Sabit;
    tema.sürümü_artır();
    let anlık = gpui_bilesenleri_galeri::tezgah_teması(&tema);
    let çözüm = gpui_bilesenleri::metin_imleci_hareketini_çöz(
        anlık.metin_imleci.hareket,
        anlık.bağlam.hareket,
        None,
    );
    assert!(!çözüm.yanıp_söner_mi());
}

#[test]
fn uzerine_yazma_tercihi_yapilandirmaya_gecer() {
    // `§12.1` tezgâh alanın açılış kipini verir; `Insert` çalışma anında
    // değiştirmeye devam eder.
    let mut t = TezgahTercihleri::default();
    assert!(
        !t.yapılandırma(&kimlik_fabrikası()).üzerine_yazma,
        "varsayılan ekleme kipidir"
    );
    t.üzerine_yazma = true;
    assert!(t.yapılandırma(&kimlik_fabrikası()).üzerine_yazma);
}

/// `§9.6` adım tercihi kanonik yapılandırmaya geçer.
#[test]
fn adim_tercihi_kucuk_buyuk_ciftini_ve_siniri_kurar() {
    let mut t = TezgahTercihleri::default();
    t.değer_türü = TezgahDeğerKipi::Ondalık;
    t.türe_uyarla();
    t.sayısal_adım = true;
    t.adım_ölçeği = AdımÖlçeği::Çeyrek;
    t.adım_hizala = true;
    t.adım_sınırı = true;
    t.adım_sarma = true;

    let y = t.yapılandırma(&kimlik_fabrikası());
    let adım = y.sayısal_adım.clone().expect("adım kurulur");
    assert_eq!(
        adım.küçük,
        OndalıkDeğer::yeni(25, 2).expect("kanonik ondalık")
    );
    assert_eq!(
        adım.büyük,
        OndalıkDeğer::yeni(1, 0).expect("kanonik ondalık")
    );
    assert!(adım.kata_hizala && adım.sarma);

    // Sınır adımın kendi alanı değil; `§15` aralık kuralından gelir.
    let sınır = y.sayısal_sınır();
    assert_eq!(
        sınır.en_az,
        Some(OndalıkDeğer::yeni(0, 0).expect("kanonik ondalık"))
    );
    assert_eq!(
        sınır.en_fazla,
        Some(OndalıkDeğer::yeni(100, 0).expect("kanonik ondalık"))
    );
}

/// Tezgâh çalışmayan ya da geçersiz bir tercih sunmaz.
///
/// Adımın her kombinasyonu `§29` doğrulamasından geçmeli: geçersiz bir
/// yapılandırma alanı `YerelGeçersiz` yüzeyine düşürür ve kullanıcı bunu
/// bileşenin kusuru sanır.
#[test]
fn adim_tercihinin_her_bilesimi_gecerli_yapilandirma_uretir() {
    for tür in [
        TezgahDeğerKipi::Tamsayı,
        TezgahDeğerKipi::Ondalık,
        TezgahDeğerKipi::ParaBirimi,
        TezgahDeğerKipi::Metin,
    ] {
        for ölçek in AdımÖlçeği::TÜMÜ {
            for hizala in [false, true] {
                for sınır in [false, true] {
                    for sarma in [false, true] {
                        let mut t = TezgahTercihleri::default();
                        t.değer_türü = tür;
                        t.sayısal_adım = true;
                        t.adım_ölçeği = ölçek;
                        t.adım_hizala = hizala;
                        t.adım_sınırı = sınır;
                        t.adım_sarma = sarma;
                        // Tezgâh her tercih değişiminde bu geçişi koşar.
                        t.türe_uyarla();
                        let rapor = t.yapılandırma(&kimlik_fabrikası()).doğrula();
                        assert!(
                            rapor.hatalar.is_empty(),
                            "{tür:?}/{ölçek:?}/hizala={hizala}/sınır={sınır}/sarma={sarma} \
                             geçersiz yapılandırma üretti: {:?}",
                            rapor.hatalar
                        );
                    }
                }
            }
        }
    }
}

/// Sayısal olmayan türde adım kapanır: iş yapmayan tercih gösterilmez.
#[test]
fn metin_turunde_adim_kapanir() {
    let mut t = TezgahTercihleri::default();
    t.değer_türü = TezgahDeğerKipi::Ondalık;
    t.sayısal_adım = true;
    t.türe_uyarla();
    assert!(t.sayısal_adım);

    t.değer_türü = TezgahDeğerKipi::Metin;
    t.türe_uyarla();
    assert!(!t.sayısal_adım);
    assert!(t.yapılandırma(&kimlik_fabrikası()).sayısal_adım.is_none());
}

/// Sınır kapanınca sarma da kapanır: sarma sonlu sınır çiftini ister.
#[test]
fn sinir_kapaninca_sarma_da_kapanir() {
    let mut t = TezgahTercihleri::default();
    t.değer_türü = TezgahDeğerKipi::Ondalık;
    t.sayısal_adım = true;
    t.adım_sınırı = true;
    t.adım_sarma = true;
    t.türe_uyarla();
    assert!(t.adım_sarma);

    t.adım_sınırı = false;
    t.türe_uyarla();
    assert!(!t.adım_sarma, "sınırsız sarma geçersiz yapılandırmadır");
}

/// Tamsayı alanda kesirli adım kurulamaz.
#[test]
fn tamsayi_alanda_kesirli_adim_duser() {
    let mut t = TezgahTercihleri::default();
    t.değer_türü = TezgahDeğerKipi::Ondalık;
    t.sayısal_adım = true;
    t.adım_ölçeği = AdımÖlçeği::Çeyrek;
    t.türe_uyarla();
    assert_eq!(t.adım_ölçeği, AdımÖlçeği::Çeyrek);

    t.değer_türü = TezgahDeğerKipi::Tamsayı;
    t.türe_uyarla();
    assert_eq!(t.adım_ölçeği, AdımÖlçeği::Birim);
}

/// `§14` varsayılan tercihi türe göre değer üretir.
#[test]
fn varsayilan_tercihi_ture_gore_deger_uretir() {
    let mut t = TezgahTercihleri::default();
    t.değer_türü = TezgahDeğerKipi::Tamsayı;
    t.varsayılan_değer = true;
    t.türe_uyarla();
    assert!(matches!(
        t.yapılandırma(&kimlik_fabrikası()).varsayılan_değer,
        gpui_bilesenleri::VarsayılanDeğer::Sabit(gpui_bilesenleri::Değer::Tamsayı(
            gpui_bilesenleri::TamsayıDeğeri::İşaretli(42)
        ))
    ));
}

/// `§14` varsayılan uygulanamayan türde tercih kapanır.
#[test]
fn tarih_turunde_varsayilan_kapanir() {
    let mut t = TezgahTercihleri::default();
    t.varsayılan_değer = true;
    t.sıfırlama = gpui_bilesenleri::SıfırlamaDavranışı::VarsayılanaDön;
    t.türe_uyarla();
    assert!(t.varsayılan_değer);

    t.değer_türü = TezgahDeğerKipi::Tarih;
    t.türe_uyarla();
    assert!(!t.varsayılan_değer, "uygulanmayan tercih açık kalmamalı");
    assert_eq!(t.sıfırlama, gpui_bilesenleri::SıfırlamaDavranışı::BoşaDön);
}

/// `§9.5` bölüm gezinimi yalnız bölümlü maskede kurulur.
#[test]
fn bolum_gezinimi_yalniz_bolumlu_maskede_kurulur() {
    let mut t = TezgahTercihleri::default();
    t.değer_türü = TezgahDeğerKipi::Tarih;
    t.maske = TezgahMaskesi::Tarih;
    t.bölüm_gezinimi = true;
    t.türe_uyarla();
    let y = t.yapılandırma(&kimlik_fabrikası());
    let Some(gpui_bilesenleri::GirişMaskesi::Tarih(maske)) = y.maske else {
        panic!("tarih maskesi kurulmalı");
    };
    assert!(maske.bölüm_gezinimi.is_some());

    // Maske kalkınca tercih de kapanır.
    t.maske = TezgahMaskesi::Yok;
    t.türe_uyarla();
    assert!(!t.bölüm_gezinimi);
}

/// Tezgâhın hiçbir tercih bileşimi geçersiz yapılandırma üretmez.
#[test]
fn varsayilan_ve_bolum_tercihleri_gecerli_yapilandirma_uretir() {
    for tür in [
        TezgahDeğerKipi::Metin,
        TezgahDeğerKipi::Tamsayı,
        TezgahDeğerKipi::Ondalık,
        TezgahDeğerKipi::ParaBirimi,
        TezgahDeğerKipi::Tarih,
    ] {
        for varsayılan in [false, true] {
            for bölüm in [false, true] {
                for sıfırlama in [
                    gpui_bilesenleri::SıfırlamaDavranışı::BoşaDön,
                    gpui_bilesenleri::SıfırlamaDavranışı::VarsayılanaDön,
                    gpui_bilesenleri::SıfırlamaDavranışı::ÜstBileşeneBırak,
                ] {
                    let mut t = TezgahTercihleri::default();
                    t.değer_türü = tür;
                    t.varsayılan_değer = varsayılan;
                    t.sıfırlama = sıfırlama;
                    t.bölüm_gezinimi = bölüm;
                    t.türe_uyarla();
                    let rapor = t.yapılandırma(&kimlik_fabrikası()).doğrula();
                    assert!(
                        rapor.hatalar.is_empty(),
                        "{tür:?}/varsayılan={varsayılan}/bölüm={bölüm} geçersiz: {:?}",
                        rapor.hatalar
                    );
                }
            }
        }
    }
}

// --------------------------------------------- Bölüm I §13 kabul ölçütleri

/// `§13/3` kod paneli yalnız A bölümünü — kamusal `GirişYapılandırması`
/// alanlarını — sunar.
///
/// B (platform yetenekleri ve portlar) ve D (tema, önizleme bağlamı) buraya
/// yazılmaz: port ve izin isteyen bir tercihi yapılandırma satırı gibi
/// göstermek, kopyalayan kişiye çalışacağı sözünü verirdi.
#[test]
fn kod_paneli_yalniz_a_bolumunu_yazar() {
    let mut t = TezgahTercihleri::default();
    t.otomatik_doldurma = true;
    t.doldurma_amacı = gpui_bilesenleri::OtomatikDoldurmaAmacı::TekKullanımlıkKod;
    t.tema.punto = 18.;

    let kod = t.kod();
    assert!(
        !kod.contains("otomatik_doldurma"),
        "B bölümü koda sızdı:\n{kod}"
    );
    assert!(!kod.contains("punto"), "D bölümü koda sızdı:\n{kod}");
}

/// `§13/3` kod paneli A eksenlerini gerçekten yansıtır.
///
/// Biçim ve `üzerine_yazma` uzun süre panelde yoktu: kullanıcı ekseni
/// oynatıyor, kod sabit kalıyordu — panel sessizce yalan söylüyordu.
#[test]
fn kod_paneli_a_eksenlerini_yansitir() {
    let taban = TezgahTercihleri::default().kod();

    let mut t = TezgahTercihleri::default();
    t.üzerine_yazma = !t.üzerine_yazma;
    assert_ne!(t.kod(), taban, "üzerine yazma kipi koda yansımıyor");

    let mut t = TezgahTercihleri::default();
    t.değer_türü = TezgahDeğerKipi::Ondalık;
    t.türe_uyarla();
    let sıra = gpui_bilesenleri_galeri::BİÇİM_SEÇENEKLERİ
        .iter()
        .position(|seçenek| seçenek.etiket.starts_with("Sayı"))
        .expect("sayı biçimi listede vardır");
    t.biçim_seçeneğini_uygula(sıra);
    assert!(
        t.kod().contains("yapılandırma.biçim"),
        "biçim ekseni koda yansımıyor:\n{}",
        t.kod()
    );
}

/// `§13/19` Tamsayı ailesinde ondalık derinliği anlamlı değildir.
///
/// Kanonik `BiçimTanımı::Tamsayı` kesir taşımaz; tür Tamsayı kaldıkça
/// derinlik hiç kurulamaz.
#[test]
fn tamsayi_ailesinde_ondalik_derinligi_yok() {
    let mut t = TezgahTercihleri::default();
    t.değer_türü = TezgahDeğerKipi::Tamsayı;
    t.türe_uyarla();
    assert!(!t.ondalık_anlamlı_mı());

    t.değer_türü = TezgahDeğerKipi::Ondalık;
    t.türe_uyarla();
    assert!(t.ondalık_anlamlı_mı());
}

/// `§13/23` `ParolayıGöster` yuvası yalnız `Gizli` ve `GeçiciGöster`de bulunur.
///
/// `Opak`ta reveal yoktur: değer hiç alınmamıştır, gösterilecek bir şey yok.
#[test]
fn parola_yuvasi_yalniz_maskeli_durumlarda() {
    use gpui_bilesenleri_galeri::TezgahGörünürlüğü as G;
    assert!(!G::Açık.parola_yuvası_var());
    assert!(G::Gizli.parola_yuvası_var());
    assert!(G::GeçiciGöster.parola_yuvası_var());
    assert!(!G::Opak.parola_yuvası_var());
}

/// `§22` dört görünürlük durumu kanonik karşılığını kurar.
#[test]
fn dort_gorunurluk_durumu_kanonige_coz() {
    use gpui_bilesenleri_galeri::TezgahGörünürlüğü as G;
    let kimlik = kimlik_fabrikası();
    for durum in G::TÜMÜ {
        let mut t = TezgahTercihleri::default();
        t.görünürlük = durum;
        let çözülmüş = t.yapılandırma(&kimlik).içerik_görünürlüğü;
        let eşleşir = match durum {
            G::Açık => matches!(çözülmüş, İçerikGörünürlüğü::Açık),
            G::Gizli => matches!(çözülmüş, İçerikGörünürlüğü::Gizli { .. }),
            G::GeçiciGöster => matches!(çözülmüş, İçerikGörünürlüğü::GeçiciGöster { .. }),
            G::Opak => matches!(çözülmüş, İçerikGörünürlüğü::Opak { .. }),
        };
        assert!(eşleşir, "{} kanoniğe çözülmüyor", durum.adı());
    }
}

/// `§13/20` ondalık derinliğinin üst sınırı `12`dir.
#[test]
fn ondalik_derinligi_on_ikiye_kadar() {
    assert_eq!(gpui_bilesenleri_galeri::EN_ÇOK_ONDALIK, 12);
}

/// `§6`/`§13/18` tür ekseni kanonik (borç 16 kapandı): yapılandırma dört
/// tanım taşıyan aileyi doğrudan taşır; kısıtlar aynı tanımdan beslenir ve
/// içerik türü/bit genişliği çözümde kaybolmaz.
#[test]
fn tur_ekseni_kanonik_aileyi_tasir() {
    let yapılandırma = gpui_bilesenleri::GirişYapılandırması::tek_satırlı_metin();
    assert!(matches!(
        yapılandırma.giriş_türü,
        gpui_bilesenleri::GirişTürü::Metin(_)
    ));

    let kanonik = gpui_bilesenleri::ÇözülmüşGirişKısıtları::bağımsız(
        gpui_bilesenleri::GirişTürü::Ondalık(gpui_bilesenleri::OndalıkTanımı::default()),
    );
    assert!(matches!(
        kanonik.giriş_türü,
        gpui_bilesenleri::GirişTürü::Ondalık(_)
    ));
}

/// Değer türü ile içerik türünün her bileşimi geçerli yapılandırma üretmeli.
///
/// Tezgâh çelişkili bir yapılandırmayı sergilemek için değil, çalışan
/// davranışı göstermek için var: bir bileşim `YerelGeçersiz` yüzeyine
/// düşerse kullanıcı bunu bileşenin kusuru sanır.
#[test]
fn her_tur_ve_icerik_turu_bilesimi_gecerli_yapilandirma_uretir() {
    for tür in TÜM_DEĞER_TÜRLERİ {
        for içerik in TÜM_İÇERİK_TÜRLERİ {
            for maske in [
                TezgahMaskesi::Yok,
                TezgahMaskesi::Desen,
                TezgahMaskesi::Tarih,
            ] {
                for görünürlük in TezgahGörünürlüğü::TÜMÜ {
                    let mut t = TezgahTercihleri::default();
                    t.değer_türü = tür;
                    t.içerik_türünü_seç(içerik);
                    t.maske = maske;
                    t.görünürlük = görünürlük;
                    t.sayaç = true;
                    t.uzunluk_sınırı = true;
                    t.ön_ek = true;
                    t.son_ek = true;
                    // Tezgâh her tercih değişiminde bu geçişi koşar.
                    t.türe_uyarla();
                    let rapor = t.yapılandırma(&kimlik_fabrikası()).doğrula();
                    assert!(
                        rapor.hatalar.is_empty(),
                        "{tür:?}/{içerik:?}/{maske:?}/{görünürlük:?} geçersiz \
                         yapılandırma üretti: {:?}",
                        rapor.hatalar
                    );
                }
            }
        }
    }
}

/// İçerik türü seçimi maskeyi ve ön eki kendine uyarlar.
///
/// Ekranda "Düz metin" yazarken kutunun telefon numarası beklemesi
/// programcıya alanın ne istediğini yanlış söylüyordu: içerik türü, maske
/// ve ön ek aynı soruya üç ayrı yanıt veriyordu.
#[test]
fn icerik_turu_maskeyi_ve_on_eki_kendine_uyarlar() {
    let mut t = TezgahTercihleri::default();
    // Açılış düz metin: serbest girdi, maske ve ülke kodu yok.
    assert_eq!(t.metin_içerik_türü, MetinİçerikTürü::Düz);
    assert_eq!(t.maske, TezgahMaskesi::Yok);
    assert!(!t.ön_ek);

    t.içerik_türünü_seç(MetinİçerikTürü::Telefon);
    assert_eq!(t.maske, TezgahMaskesi::Desen);
    assert!(t.ön_ek, "telefon ülke kodu ön ekiyle açılır");
    assert_eq!(t.ön_ek_metni, "+90");
    // Biçim listesi de deseni gösterir; iki yer aynı şeyi söylemeli.
    assert!(t.seçili_biçim().etiket.contains("Telefon") || t.maske == TezgahMaskesi::Desen);

    t.içerik_türünü_seç(MetinİçerikTürü::Url);
    assert_eq!(t.maske, TezgahMaskesi::Yok, "URL desenle yazılmaz");
    assert_eq!(t.ön_ek_metni, "https://");

    t.içerik_türünü_seç(MetinİçerikTürü::EPosta);
    assert_eq!(t.maske, TezgahMaskesi::Yok);
    assert!(!t.ön_ek, "e-postanın sabit ön eki yok");

    t.içerik_türünü_seç(MetinİçerikTürü::Düz);
    assert_eq!(t.maske, TezgahMaskesi::Yok);
    assert!(!t.ön_ek);
}

/// Kullanıcının kendi yazdığı ön ek metni tür değişiminde silinmez.
///
/// Otomatik türetme yalnız tezgâhın **kendi** yazdığı değerleri tazeler;
/// aksi hâlde ön ek kutusuna yazılan her metin bir sonraki seçimde
/// kaybolurdu.
#[test]
fn elle_yazilan_on_ek_otomatik_turetmeden_korunur() {
    let mut t = TezgahTercihleri::default();
    t.ön_ek = true;
    t.ön_ek_metni = "Sipariş".to_owned();
    t.içerik_türünü_seç(MetinİçerikTürü::Telefon);
    assert_eq!(t.ön_ek_metni, "Sipariş", "elle yazılan ön ek korunur");
    t.içerik_türünü_seç(MetinİçerikTürü::Düz);
    assert_eq!(t.ön_ek_metni, "Sipariş");
    assert!(t.ön_ek, "elle yazılan ön ek kapatılmaz");
}

/// İçerik türü yalnız metin ailesinde anlamlıdır.
#[test]
fn sayisal_ve_tarih_turunde_icerik_turu_duze_doner() {
    for tür in TÜM_DEĞER_TÜRLERİ {
        let mut t = TezgahTercihleri::default();
        t.içerik_türünü_seç(MetinİçerikTürü::Telefon);
        t.değer_türü = tür;
        t.türe_uyarla();
        if tür == TezgahDeğerKipi::Metin {
            assert_eq!(t.metin_içerik_türü, MetinİçerikTürü::Telefon);
        } else {
            assert_eq!(
                t.metin_içerik_türü,
                MetinİçerikTürü::Düz,
                "{tür:?} metin ailesinde değil, içerik türü taşıyamaz"
            );
        }
    }
}

/// Sekme durağı tercihi kanonik odak yapılandırmasına geçer.
///
/// `§29` uyarısı ("sekme durağı kapalı ama Enter sonrakine geçiyor") bu
/// tercihi anlatıyordu ama ekranda karşılığı yoktu: uyarı hiç
/// kurulamıyordu.
#[test]
fn sekme_duragi_tercihi_yapilandirmaya_gecer() {
    let t = TezgahTercihleri::default();
    assert!(t.yapılandırma(&kimlik_fabrikası()).odak.sekme_durağı);

    let mut t = TezgahTercihleri::default();
    t.sekme_durağı = false;
    let y = t.yapılandırma(&kimlik_fabrikası());
    assert!(!y.odak.sekme_durağı);
    assert!(t.kod().contains("odak.sekme_durağı = false"));

    // Enter sonrakine geçerken sekme durağı kapalıysa `§29` uyarır.
    t.enter = EnterDavranışı::DeğeriİşleVeSonrakineGeç;
    let rapor = t.yapılandırma(&kimlik_fabrikası()).doğrula();
    assert!(
        !rapor.uyarılar.is_empty(),
        "sekme durağı kapalıyken Enter geçişi uyarı üretmeli"
    );
}

/// `§23` üç yuva sınırı dolduğunda dördüncü yuva açılamaz.
///
/// Sınır yapılandırma üretiminde `take(3)` ile sessizce kırpılıyordu:
/// kullanıcı dördüncü düğmeye basıyor, hiçbir şey olmuyor ve nedenini
/// göremiyordu.
#[test]
fn uc_yuva_dolunca_dorduncusu_kurulamaz() {
    let mut t = TezgahTercihleri::default();
    t.görünürlük = TezgahGörünürlüğü::Gizli;
    t.temizle = true;
    t.parola_düğmesi = true;
    t.arama = true;
    assert_eq!(t.açık_yuva_sayısı(), 3);
    assert!(!t.yuva_eklenebilir_mi());

    // Dördüncüsü kurulsa bile yapılandırma `§23` sınırını aşmaz.
    t.seçici = true;
    let y = t.yapılandırma(&kimlik_fabrikası());
    let yuvalar = y.yardımcı_eylemler.as_ref().expect("yuva listesi");
    assert_eq!(yuvalar.len(), 3, "sınır aşılmamalı");
    assert!(y.doğrula().hatalar.is_empty());

    // Parola yuvası görünürlüğe bağlı: `Açık`ta sayılmaz, yer açılır.
    t.görünürlük = TezgahGörünürlüğü::Açık;
    assert_eq!(t.açık_yuva_sayısı(), 3, "temizle + arama + seçici");
    t.arama = false;
    assert!(t.yuva_eklenebilir_mi());
}

/// Seçici, devre dışılık ve odak politikası bileşimleri geçerli kalmalı.
///
/// Kanonik iki kural daha var: seçici sayısal ve süre türlerinde
/// kurulamaz (`UyumsuzSeçici`), `OdağıKoru` da odaklanamayan alanda
/// kurulamaz (`GeçersizOdakPolitikası`). Tezgâh ikisine de düşebiliyordu.
#[test]
fn secici_ve_odak_politikasi_bilesimleri_gecerli_kalir() {
    use gpui_bilesenleri::GeçersizOdakDavranışı;
    for tür in TÜM_DEĞER_TÜRLERİ {
        for seçici in [false, true] {
            for etkin in [false, true] {
                for odak in [
                    GeçersizOdakDavranışı::OdağıKoru,
                    GeçersizOdakDavranışı::OdakKaybınaİzinVer,
                    GeçersizOdakDavranışı::EskiDeğereDönVeİzinVer,
                ] {
                    let mut t = TezgahTercihleri::default();
                    t.değer_türü = tür;
                    t.seçici = seçici;
                    t.etkin = etkin;
                    t.geçersiz_odak = odak;
                    t.türe_uyarla();
                    let rapor = t.yapılandırma(&kimlik_fabrikası()).doğrula();
                    assert!(
                        rapor.hatalar.is_empty(),
                        "{tür:?}/seçici={seçici}/etkin={etkin}/{odak:?} geçersiz \
                         yapılandırma üretti: {:?}",
                        rapor.hatalar
                    );
                }
            }
        }
    }
}

/// Tercih uzayında deterministik gezinti: hiçbir bileşim geçersiz olamaz.
///
/// Eksenler tek tek sınandığında geçerli görünüyor; hatalar **çiftlerden**
/// doğuyor (devre dışı alan + `OdağıKoru`, `Yüzde` + desen maskesi, dört
/// yardımcı yuva). Elle sayılan çiftler yetmez: burada tohumlu bir üreteç
/// tercih uzayını gezer ve her durakta `§29` doğrulaması koşar. Tohum
/// sabit olduğu için düşen bileşim yeniden üretilebilir.
#[test]
fn tercih_uzayinda_hicbir_bilesim_gecersiz_yapilandirma_uretmez() {
    use gpui_bilesenleri::{
        BoşMetinPolitikası, EnterDavranışı, EscapeDavranışı, GeçerlilikTetikleyicisi,
        GeçerlilikÖnemi, GeçersizOdakDavranışı, HarfDönüşümü, KırpmaPolitikası,
        SabitİçerikSunumRolü, SayımBirimi, SeçiciGörünürlüğü, UzunlukSınırıDavranışı,
        ÇalışırkenEnterPolitikası,
    };
    use gpui_bilesenleri_galeri::{TezgahBölütü, TezgahYapıştırması};

    // Doğrusal eşleşik üreteç: `Math::random` yok, tohum sabit.
    let mut durum: u64 = 0x5DEE_CE66_D137_2A9F;
    let mut sonraki = |üst: usize| -> usize {
        durum = durum
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        ((durum >> 33) as usize) % üst.max(1)
    };
    let bit = |ü: &mut dyn FnMut(usize) -> usize| ü(2) == 1;

    for tur in 0..4_000 {
        let mut t = TezgahTercihleri::default();
        t.değer_türü = TÜM_DEĞER_TÜRLERİ[sonraki(TÜM_DEĞER_TÜRLERİ.len())];
        t.metin_içerik_türü = TÜM_İÇERİK_TÜRLERİ[sonraki(TÜM_İÇERİK_TÜRLERİ.len())];
        t.maske = [
            TezgahMaskesi::Yok,
            TezgahMaskesi::Desen,
            TezgahMaskesi::Tarih,
        ][sonraki(3)];
        t.görünürlük = TezgahGörünürlüğü::TÜMÜ[sonraki(4)];
        t.biçim_seçeneği = sonraki(24);
        t.ondalık_basamak = sonraki(13);
        t.geçersiz_odak = [
            GeçersizOdakDavranışı::OdakKaybınaİzinVer,
            GeçersizOdakDavranışı::OdağıKoru,
            GeçersizOdakDavranışı::EskiDeğereDönVeİzinVer,
        ][sonraki(3)];
        // Enter yalnız sahiplik ve odak ekseni taşır; kabul sonrası caret
        // yerleşimi ayrı bir tercihtir ve aşağıda kendi ekseninde sınanır.
        t.enter = [
            EnterDavranışı::ÜstBileşeneBırak,
            EnterDavranışı::DeğeriİşleVeKal,
            EnterDavranışı::DeğeriİşleVeSonrakineGeç,
        ][sonraki(3)];
        t.kabul_seçimi = [
            KabulSeçimi::TümünüSeç,
            KabulSeçimi::SonaGit,
            KabulSeçimi::İmleciKoru,
        ][sonraki(3)];
        t.odak_seçimi = [OdakSeçimi::TümünüSeç, OdakSeçimi::SonaGit][sonraki(2)];
        t.escape = [
            EscapeDavranışı::ÜstBileşeneBırak,
            EscapeDavranışı::DeğişiklikleriKoru,
            EscapeDavranışı::EskiDeğereDön,
        ][sonraki(3)];
        t.harf_dönüşümü = [
            HarfDönüşümü::Yok,
            HarfDönüşümü::Büyük,
            HarfDönüşümü::Küçük,
            HarfDönüşümü::SözcükBaşı,
        ][sonraki(4)];
        t.kırpma = [
            KırpmaPolitikası::Yok,
            KırpmaPolitikası::KabuldeKırp,
            KırpmaPolitikası::HerZamanKırp,
        ][sonraki(3)];
        t.boş_metin = [
            BoşMetinPolitikası::BoşDeğer,
            BoşMetinPolitikası::BoşMetinKoru,
            BoşMetinPolitikası::Reddet,
        ][sonraki(3)];
        t.uzunluk_davranışı =
            [UzunlukSınırıDavranışı::Reddet, UzunlukSınırıDavranışı::Kırp][sonraki(2)];
        t.sayaç_birimi = [
            SayımBirimi::Grafem,
            SayımBirimi::KodNoktası,
            SayımBirimi::Utf16Birimi,
        ][sonraki(3)];
        t.doğrulama_tetikleyicisi = [
            GeçerlilikTetikleyicisi::Değişimde,
            GeçerlilikTetikleyicisi::Kabulde,
            GeçerlilikTetikleyicisi::OdakKaybında,
            GeçerlilikTetikleyicisi::Açıkİstekte,
        ][sonraki(4)];
        t.doğrulama_önemi = [
            GeçerlilikÖnemi::Hata,
            GeçerlilikÖnemi::Uyarı,
            GeçerlilikÖnemi::Bilgi,
        ][sonraki(3)];
        t.adım_ölçeği = AdımÖlçeği::TÜMÜ[sonraki(4)];
        t.seçici_görünürlüğü = [
            SeçiciGörünürlüğü::Gizli,
            SeçiciGörünürlüğü::UyumluTürdeGöster,
            SeçiciGörünürlüğü::HerZamanGöster,
        ][sonraki(3)];
        t.çalışırken_enter = [
            ÇalışırkenEnterPolitikası::Yoksay,
            ÇalışırkenEnterPolitikası::ORT007PolitikasınaBırak,
        ][sonraki(2)];
        t.ek_sunum_rolü = [
            SabitİçerikSunumRolü::İkincil,
            SabitİçerikSunumRolü::DeğerleEş,
        ][sonraki(2)];
        t.sıfırlama = [
            gpui_bilesenleri::SıfırlamaDavranışı::BoşaDön,
            gpui_bilesenleri::SıfırlamaDavranışı::VarsayılanaDön,
            gpui_bilesenleri::SıfırlamaDavranışı::ÜstBileşeneBırak,
        ][sonraki(3)];
        t.doldurma_amacı = [
            gpui_bilesenleri::OtomatikDoldurmaAmacı::Ad,
            gpui_bilesenleri::OtomatikDoldurmaAmacı::KullanıcıAdı,
            gpui_bilesenleri::OtomatikDoldurmaAmacı::YeniParola,
            gpui_bilesenleri::OtomatikDoldurmaAmacı::GeçerliParola,
            gpui_bilesenleri::OtomatikDoldurmaAmacı::TekKullanımlıkKod,
            gpui_bilesenleri::OtomatikDoldurmaAmacı::EPosta,
            gpui_bilesenleri::OtomatikDoldurmaAmacı::Telefon,
        ][sonraki(7)];
        t.hizalama = [
            GirişYatayHizalama::Genel,
            GirişYatayHizalama::Başlangıç,
            GirişYatayHizalama::Orta,
            GirişYatayHizalama::Bitiş,
            GirişYatayHizalama::Sol,
            GirişYatayHizalama::Sağ,
        ][sonraki(6)];
        t.dikey = [
            GirişDikeyHizalama::Üst,
            GirişDikeyHizalama::Orta,
            GirişDikeyHizalama::Alt,
        ][sonraki(3)];
        t.şekil = [
            DüğmeŞekli::DikKöşeli,
            DüğmeŞekli::Köşeli,
            DüğmeŞekli::Yuvarlatılmış,
            DüğmeŞekli::Hap,
        ][sonraki(4)];
        // `§23` bitişik bölütler; `None` da bir seçenektir.
        let bölüt = |n: usize| match n {
            0 => None,
            1 => Some(TezgahBölütü::SabitMetin),
            _ => Some(TezgahBölütü::Eylem),
        };
        t.başlangıç_bölütü = bölüt(sonraki(3));
        t.bitiş_bölütü = bölüt(sonraki(3));
        t.yapıştırma = TezgahYapıştırması::TÜMÜ[sonraki(4)];

        for alan in [
            &mut t.ön_ek,
            &mut t.son_ek,
            &mut t.yer_tutucu,
            &mut t.temizle,
            &mut t.arama,
            &mut t.parola_düğmesi,
            &mut t.seçici,
            &mut t.zorunlu,
            &mut t.uzunluk_sınırı,
            &mut t.sayaç,
            &mut t.sayaç_sınırı_göster,
            &mut t.sayısal_adım,
            &mut t.adım_hizala,
            &mut t.adım_sınırı,
            &mut t.adım_sarma,
            &mut t.binler_ayracı,
            &mut t.varsayılan_değer,
            &mut t.bölüm_gezinimi,
            &mut t.otomatik_doldurma,
            &mut t.salt_okunur,
            &mut t.etkin,
            &mut t.sekme_durağı,
            &mut t.dış_tıklamada_odağı_bırak,
            &mut t.üzerine_yazma,
            &mut t.arama_enter_gönderir,
            &mut t.arama_temizleme_gönderir,
            &mut t.bölüt_kademeli,
            &mut t.bölüm_atla,
            &mut t.bölüm_dolunca_ilerle,
            &mut t.bölüm_artır,
            &mut t.bölüm_taşar,
            &mut t.bölüm_ayraç,
            &mut t.şekil_oto,
        ] {
            *alan = bit(&mut sonraki);
        }

        // Tezgâh her tercih değişiminde bu geçişi koşar.
        t.türe_uyarla();
        let rapor = t.yapılandırma(&kimlik_fabrikası()).doğrula();
        assert!(
            rapor.hatalar.is_empty(),
            "tur {tur}: {:?} geçersiz yapılandırma üretti: {:?}\n{t:#?}",
            t.değer_türü,
            rapor.hatalar
        );
    }
}

/// `§29` ilk hatada durma tercihi kanonik doğrulamaya geçer.
///
/// Kanonik alan `çekirdek.rs`'te iş yapıyordu — açıkken alan ilk
/// başarısız kuraldan sonra kalanları koşturmaz — ama ekranda karşılığı
/// yoktu: tezgâh yalnız "hepsini koştur" dalını gösteriyordu.
#[test]
fn ilk_hatada_dur_tercihi_yapilandirmaya_gecer() {
    let t = TezgahTercihleri::default();
    assert!(!t.yapılandırma(&kimlik_fabrikası()).doğrulama.ilk_hatada_dur);

    let mut t = TezgahTercihleri::default();
    t.ilk_hatada_dur = true;
    assert!(t.yapılandırma(&kimlik_fabrikası()).doğrulama.ilk_hatada_dur);
    assert!(t.kod().contains("doğrulama.ilk_hatada_dur = true"));
}

/// Sayaç üç birimi de sunar: aynı metinde üçü farklı sonuç verir.
///
/// `Utf16Birimi` JS ve Win32 sınırlarının saydığı birimdir; tezgâh ikisini
/// gösterip üçüncüsünü saklıyordu.
#[test]
fn sayac_birimlerinin_ucu_de_kurulabilir() {
    for birim in [
        SayımBirimi::Grafem,
        SayımBirimi::KodNoktası,
        SayımBirimi::Utf16Birimi,
    ] {
        let mut t = TezgahTercihleri::default();
        t.sayaç = true;
        t.sayaç_birimi = birim;
        let y = t.yapılandırma(&kimlik_fabrikası());
        assert_eq!(y.sayaç.expect("sayaç açık").birim, birim);
        assert!(y.doğrula().hatalar.is_empty(), "{birim:?} geçersiz");
    }
}

/// `§26` her kanonik olay varyantı akışta bir ada ve özete çevrilir.
///
/// Panel programcının `match` yazarken göreceği adı gösterir; tezgâh olayı
/// yeniden adlandırırsa panel ile kod farklı diller konuşur.
#[test]
fn her_olay_varyanti_akis_satirina_cevrilir() {
    use gpui_bilesenleri::{
        AramaKaynağı, GirişOlayı, KabulNedeni, SayımBirimi, YardımcıEylemTürü
    };
    let olaylar = [
        GirişOlayı::DüzenlemeMetniDeğişti {
            metin: "a".to_owned(),
            değer_sürümü: 3,
        },
        GirişOlayı::GeçiciDeğerDeğişti {
            değer: None,
            değer_sürümü: 4,
        },
        GirişOlayı::DeğerKabulEdildi {
            değer: None,
            neden: KabulNedeni::Enter,
        },
        GirişOlayı::KabulReddedildi { sorunlar: vec![] },
        GirişOlayı::OdakGeçişiReddedildi { sorunlar: vec![] },
        GirişOlayı::EskiDeğereDönüldü,
        GirişOlayı::YapıştırmaSüzüldü {
            atılan_grafem_sayısı: 2,
        },
        GirişOlayı::YapılandırmaReddedildi { hatalar: vec![] },
        GirişOlayı::ÜzerineYazmaDeğişti { açık: true },
        GirişOlayı::GirişReddedildi {
            değer_türü: gpui_bilesenleri::GirişDeğerTürü::Tamsayı,
        },
        GirişOlayı::UzunlukSınırıUygulandı {
            atılan: 1,
            birim: SayımBirimi::Grafem,
            politika: gpui_bilesenleri::AşımPolitikası::Kırp,
        },
        GirişOlayı::YardımcıEylemİstendi(YardımcıEylemTürü::Temizle),
        GirişOlayı::AramaGönderildi {
            metin: "x".to_owned(),
            kaynak: AramaKaynağı::Alan,
            değer_sürümü: 5,
        },
    ];
    let mut görülen = std::collections::BTreeSet::new();
    for olay in &olaylar {
        let satır = gpui_bilesenleri_galeri::olay_özeti(olay);
        assert!(!satır.ad.is_empty(), "olay adsız kalamaz");
        assert_eq!(satır.sayı, 1);
        // Ad kanonik varyantın adıdır; iki varyant aynı ada düşerse panel
        // hangisinin olduğunu söyleyemez.
        assert!(görülen.insert(satır.ad), "ad yinelendi: {}", satır.ad);
    }
    assert_eq!(görülen.len(), olaylar.len());
}

/// `§23` yuva kipi, etkinlik kapısı ve gönderim bağı yapılandırmaya geçer.
///
/// Tezgâh her yuvayı `YardımcıEylemYuvası::kademeli` ile kuruyordu: dört
/// görünürlük kipinden yalnız biri ekranda vardı, `etkin` kapısı ile
/// `çalışma` alanı hiç denenemiyordu.
#[test]
fn yuva_kipi_etkinlik_ve_gonderim_bagi_gecer() {
    use gpui_bilesenleri::{
        YardımcıEylemGörünürlüğü, YardımcıEylemTürü, YardımcıEylemÇalışması
    };

    let taban = TezgahTercihleri::default();
    let y = taban.yapılandırma(&kimlik_fabrikası());
    let yuvalar = y.yardımcı_eylemler.as_ref().expect("varsayılanda temizle");
    assert_eq!(
        yuvalar[0].görünürlük,
        YardımcıEylemGörünürlüğü::DeğerVarkenKademeli
    );
    assert!(yuvalar[0].etkin);

    for kip in [
        YardımcıEylemGörünürlüğü::HerZaman,
        YardımcıEylemGörünürlüğü::DeğerVarken,
        YardımcıEylemGörünürlüğü::EtkileşimdeKademeli,
        YardımcıEylemGörünürlüğü::DeğerVarkenKademeli,
    ] {
        let mut t = TezgahTercihleri::default();
        t.yuva_görünürlüğü = kip;
        t.yuvalar_etkin = false;
        let y = t.yapılandırma(&kimlik_fabrikası());
        let yuvalar = y.yardımcı_eylemler.as_ref().expect("yuva listesi");
        assert!(yuvalar.iter().all(|yuva| yuva.görünürlük == kip));
        assert!(yuvalar.iter().all(|yuva| !yuva.etkin));
        assert!(y.doğrula().hatalar.is_empty(), "{kip:?} geçersiz");
    }

    // Gönderim bağı yalnız arama yuvasına iner.
    let mut t = TezgahTercihleri::default();
    t.arama = true;
    t.arama_gönderime_bağlı = true;
    let y = t.yapılandırma(&kimlik_fabrikası());
    for yuva in y.yardımcı_eylemler.as_ref().expect("yuva listesi").iter() {
        let beklenen = if yuva.tür == YardımcıEylemTürü::AramayıBaşlat {
            YardımcıEylemÇalışması::AlanınGönderimineBağlı
        } else {
            YardımcıEylemÇalışması::Yok
        };
        assert_eq!(yuva.çalışma, beklenen, "{:?} yuvası", yuva.tür);
    }

    // Üretilen kod kopyalanıp derlenebilmeli: kanonikte olmayan bir
    // zincir metodu yazılmamalı.
    let kod = t.kod();
    assert!(kod.contains("yuva.çalışma = YardımcıEylemÇalışması::AlanınGönderimineBağlı"));
    assert!(!kod.contains(".çalışmayla("));
}

/// `§23.1` ürün kendi yardımcı eylemini yuvaya koyabilir.
///
/// Galeri `YardımcıEylemTürü::Ürün` dalında `unreachable!()` çağırıyordu:
/// yerleşik dört tür dışına çıkmak tezgâhta hiç denenemiyordu.
#[test]
fn urun_eylemi_yuvaya_kurulabilir() {
    use gpui_bilesenleri::YardımcıEylemTürü;

    let mut t = TezgahTercihleri::default();
    t.ürün_eylemi = true;
    let y = t.yapılandırma(&kimlik_fabrikası());
    let yuvalar = y.yardımcı_eylemler.as_ref().expect("yuva listesi");
    let ürün = yuvalar
        .iter()
        .find(|yuva| matches!(yuva.tür, YardımcıEylemTürü::Ürün(_)))
        .expect("ürün yuvası kuruldu");
    // `ORT-009` adsız düğme erişilebilir ağaca girmez.
    assert!(ürün.erişilebilir_ad.is_some());
    assert!(y.doğrula().hatalar.is_empty());

    // Ürün eylemi de `§23` üç yuva sınırına girer.
    assert_eq!(t.açık_yuva_sayısı(), 2, "temizle + ürün");

    // Üretilen kod kopyalanabilmeli: ham `EylemKimliği` basılmamalı.
    let kod = t.kod();
    assert!(kod.contains("YardımcıEylemTürü::Ürün(ürün_eylem_kimliği)"));
    assert!(!kod.contains("TanımKimliği {"));
}

/// `ORT-009` erişilebilir ad iki ayrı eksendir ve `§29` uyarıları görülür.
///
/// Tezgâh hem alanın hem yuvaların adını sabit kuruyordu: uyarı metinleri
/// ekranda yazılıydı ama onlara ulaşan bir yol yoktu.
#[test]
fn erisilebilir_ad_eksenleri_uyari_uretir() {
    use gpui_bilesenleri::GirişYapılandırmaUyarısı as U;

    let taban = TezgahTercihleri::default();
    let rapor = taban.yapılandırma(&kimlik_fabrikası()).doğrula();
    assert!(!rapor.uyarılar.contains(&U::ErişilebilirAdYok));
    assert!(!rapor.uyarılar.contains(&U::YardımcıEylemAdsız));

    let mut t = TezgahTercihleri::default();
    t.erişilebilir_ad = false;
    let rapor = t.yapılandırma(&kimlik_fabrikası()).doğrula();
    assert!(rapor.uyarılar.contains(&U::ErişilebilirAdYok));
    // Yuva adları hâlâ kurulu: iki eksen bağımsız.
    assert!(!rapor.uyarılar.contains(&U::YardımcıEylemAdsız));
    assert!(t.kod().contains("erişilebilir_ad = None"));

    let mut t = TezgahTercihleri::default();
    t.yuva_adları = false;
    let rapor = t.yapılandırma(&kimlik_fabrikası()).doğrula();
    assert!(rapor.uyarılar.contains(&U::YardımcıEylemAdsız));
    assert!(!rapor.uyarılar.contains(&U::ErişilebilirAdYok));
}

/// `ORT-003 §3.1` bitişik bölütün kendi sınırı seçilebilir.
#[test]
fn bolut_kendi_sinirini_tasiyabilir() {
    let mut t = TezgahTercihleri::default();
    t.başlangıç_bölütü = Some(gpui_bilesenleri_galeri::TezgahBölütü::SabitMetin);
    let y = t.yapılandırma(&kimlik_fabrikası());
    let kuşak = y.bitişik_bölütler.as_ref().expect("kuşak kuruldu");
    assert!(kuşak.başlangıç.as_ref().expect("başlangıç").kendi_sınırı);

    t.bölüt_sınırı = false;
    let y = t.yapılandırma(&kimlik_fabrikası());
    let kuşak = y.bitişik_bölütler.as_ref().expect("kuşak kuruldu");
    assert!(!kuşak.başlangıç.as_ref().expect("başlangıç").kendi_sınırı);
    assert!(y.doğrula().hatalar.is_empty());
}

/// Yer tutucu kod panelinde de görünür.
///
/// Taban `tek_satırlı_metin()` `yer_tutucu: None` kurar; ekrandaki
/// "Değer girin…" tezgâhın kendi sapmasıdır. Panel yalnız kapalı dalı
/// yazdığı sürece kopyalanan kod yer tutucusuz bir alan üretiyor ve
/// ekrandakiyle aynı olmuyordu.
#[test]
fn yer_tutucu_kod_panelinde_yazilir() {
    let taban = gpui_bilesenleri::GirişYapılandırması::tek_satırlı_metin();
    assert!(taban.yer_tutucu.is_none(), "taban yer tutucu kurmaz");

    let t = TezgahTercihleri::default();
    assert!(t.yapılandırma(&kimlik_fabrikası()).yer_tutucu.is_some());
    assert!(
        t.kod().contains("yer_tutucu = Some("),
        "açık yer tutucu kod panelinde görünmeli"
    );

    let mut t = TezgahTercihleri::default();
    t.yer_tutucu = false;
    assert!(t.yapılandırma(&kimlik_fabrikası()).yer_tutucu.is_none());
    assert!(!t.kod().contains("yer_tutucu = Some("));
}

/// `§13` hassas değer panele yazılmaz.
///
/// Panel bir tanı yüzeyidir; `GüvenliMetin::tanı_metni` yalnız açıkça
/// güvenli sınıflandırılmış metni verir. Parola kipindeki bir alanın ham
/// değerini yandaki karta yazmak, kutuda gizlediğimizi iki santim yana
/// kopyalamak olurdu.
#[test]
fn hassas_deger_panele_yazilmaz() {
    use gpui_bilesenleri::{Değer, GüvenliMetin};

    let açık = Değer::Metin(GüvenliMetin::yeni("görünür", false, true));
    assert_eq!(gpui_bilesenleri_galeri::değer_özeti(&açık), "görünür");

    let hassas = Değer::Metin(GüvenliMetin::yeni("parola123", true, false));
    let özet = gpui_bilesenleri_galeri::değer_özeti(&hassas);
    assert!(!özet.contains("parola123"), "hassas metin sızdı: {özet}");
    assert_eq!(özet, "‹gizli›");

    // Loglanamaz sınıflandırma da tanı sınırını geçmez.
    let loglanamaz = Değer::Metin(GüvenliMetin::yeni("iz", false, false));
    assert_eq!(gpui_bilesenleri_galeri::değer_özeti(&loglanamaz), "‹gizli›");
}

/// `§28`/`§29.0` önem yapılandırılmaz, **kuraldan** türer.
///
/// Bir süre tezgâhta seçilebiliyordu ve `GirişKutusu.önem` alanına
/// doğrudan yazılıyordu; `sorunları_uygula` ilk doğrulamada onu eziyordu.
/// Ürün önemi artık kuralın üzerinde bildirir; zemin uygulaması ayrı bir
/// sunum tercihidir ve doğrulama onu yazmaz.
#[test]
fn onem_kuraldan_turer_zemin_tercihi_ayri_kalir() {
    use gpui_bilesenleri::GeçerlilikÖnemi;

    let t = TezgahTercihleri::default();
    assert!(!t.önem_zemini, "varsayılan yalnız kenarlık");

    for önem in [
        GeçerlilikÖnemi::Bilgi,
        GeçerlilikÖnemi::Uyarı,
        GeçerlilikÖnemi::Hata,
    ] {
        let mut t = TezgahTercihleri::default();
        t.zorunlu = true;
        t.doğrulama_önemi = önem;
        t.önem_zemini = true;
        let y = t.yapılandırma(&kimlik_fabrikası());
        assert!(y.doğrula().hatalar.is_empty());
        // Önem kuralın üzerinde taşınır: alanın üzerinde bir yazma yolu yok.
        assert!(
            y.doğrulama.kurallar.iter().any(|k| k.önem == önem),
            "{önem:?} kurala inmeli"
        );
    }
}

/// `Süre` değer türü ve süre biçimi tezgâhta kurulabilir.
///
/// `TARİH_KİPLERİ` üç kip sunuyordu; yorum süreyi ailenin dördüncü kipi
/// sayıyordu ama listede yoktu. Dokuz değer türünden biri ekranda hiç
/// seçilemiyordu ve `ORT-008` süre gösterimi hiç denenemiyordu.
#[test]
fn sure_turu_ve_bicimi_kurulabilir() {
    use gpui_bilesenleri::{BiçimTanımı, BiçimYapılandırması, SüreBirimi};

    // Kip listesi süreyi taşır.
    assert!(
        gpui_bilesenleri_galeri::TARİH_KİPLERİ
            .iter()
            .any(|(_, tür)| *tür == TezgahDeğerKipi::Süre),
        "süre kipi tür satırında sunulmalı"
    );

    let mut t = TezgahTercihleri::default();
    t.değer_türü = TezgahDeğerKipi::Süre;
    t.türe_uyarla();
    let y = t.yapılandırma(&kimlik_fabrikası());
    assert!(matches!(
        y.giriş_türü,
        gpui_bilesenleri::GirişTürü::TarihZaman(tanım)
            if tanım.kip == gpui_bilesenleri::TarihZamanKipi::Aralık
    ));
    assert!(y.doğrula().hatalar.is_empty());

    // Süre biçimi birim çifti kurar ve `ORT-008` doğrulamasından geçer.
    let sıra = gpui_bilesenleri_galeri::BİÇİM_SEÇENEKLERİ
        .iter()
        .position(|s| {
            matches!(
                s.uygulama,
                gpui_bilesenleri_galeri::BiçimUygulaması::Süre(..)
            )
        })
        .expect("süre biçimi listede");
    let mut t = TezgahTercihleri::default();
    t.biçim_seçeneğini_uygula(sıra);
    // Biçim kendi türünü kurar; tarih biçimlerinin kuralıyla aynı.
    assert_eq!(t.değer_türü, TezgahDeğerKipi::Süre);
    match t.biçim_çöz() {
        BiçimYapılandırması::Açık(BiçimTanımı::Süre(biçim)) => {
            assert!(biçim.doğrula().is_ok(), "en küçük birim en büyüğü aşamaz");
            assert!(biçim.en_küçük_birim <= biçim.en_büyük_birim);
        }
        diğer => panic!("süre biçimi bekleniyordu: {diğer:?}"),
    }
    assert!(t.kod().contains("BiçimTanımı::Süre(SüreBiçimi"));
    assert!(t.kod().contains("SüreBirimi::"));
    let _ = SüreBirimi::Saat;
}

/// `§23`/`§29.0` `üzerinde` **türetilmiştir**: senaryo olarak kurulamaz.
///
/// Bir süre `GirişKutusu.üzerinde` kamusaldı ve tezgâh onu senaryo olarak
/// yazıyordu; tek gerçek yazar `on_hover` olduğu için kurulan değer ilk
/// fare hareketinde siliniyordu. Kanal kapandı — kademeli kipin
/// yapılandırması yine kurulabilir, yalnız işaretçi durumu taklit edilmez.
#[test]
fn kademeli_yuva_kipi_isaretci_taklidi_olmadan_kurulur() {
    let mut t = TezgahTercihleri::default();
    t.yuva_görünürlüğü = gpui_bilesenleri::YardımcıEylemGörünürlüğü::EtkileşimdeKademeli;
    let y = t.yapılandırma(&kimlik_fabrikası());
    assert!(y.doğrula().hatalar.is_empty());
    assert_eq!(
        y.yardımcı_eylemler
            .as_ref()
            .and_then(|yuvalar| yuvalar.first())
            .map(|yuva| yuva.görünürlük),
        Some(gpui_bilesenleri::YardımcıEylemGörünürlüğü::EtkileşimdeKademeli),
        "kip yuvalara inmeli"
    );
}

#[test]
fn dort_uyarinin_dordu_de_tetiklenebilir() {
    use gpui_bilesenleri::{
        BoşMetinPolitikası, EnterDavranışı, GirişYapılandırmaUyarısı as U
    };

    let mut görülen = std::collections::BTreeSet::new();
    let ekle = |t: &TezgahTercihleri, görülen: &mut std::collections::BTreeSet<String>| {
        for u in t.yapılandırma(&kimlik_fabrikası()).doğrula().uyarılar {
            görülen.insert(format!("{u:?}"));
        }
    };

    // `SekmeDurağıYokkenSonrakineGeçiş`
    let mut t = TezgahTercihleri::default();
    t.sekme_durağı = false;
    t.enter = EnterDavranışı::DeğeriİşleVeSonrakineGeç;
    ekle(&t, &mut görülen);

    // `ErişilebilirAdYok`
    let mut t = TezgahTercihleri::default();
    t.erişilebilir_ad = false;
    ekle(&t, &mut görülen);

    // `YardımcıEylemAdsız`
    let mut t = TezgahTercihleri::default();
    t.yuva_adları = false;
    ekle(&t, &mut görülen);

    // `BoşMetinReddiZorunluKuralıOlmadan`
    let mut t = TezgahTercihleri::default();
    t.boş_metin = BoşMetinPolitikası::Reddet;
    t.zorunlu = false;
    ekle(&t, &mut görülen);

    for beklenen in [
        U::SekmeDurağıYokkenSonrakineGeçiş,
        U::ErişilebilirAdYok,
        U::YardımcıEylemAdsız,
        U::BoşMetinReddiZorunluKuralıOlmadan,
    ] {
        assert!(
            görülen.contains(&format!("{beklenen:?}")),
            "{beklenen:?} tezgâhtan tetiklenemiyor; görülenler: {görülen:?}"
        );
    }
}

/// `ORT-004` metin düzenleme iç boşluğu tercihi temaya geçer.
///
/// Tema `metin_düzenleme_iç_boşluğu` alanını taşıyordu ve galeri onu
/// `None` bırakıyordu: kütüphane varsayılanı dışına çıkılamıyor, kutunun
/// iç boşluğu hiç denenemiyordu. `None` sıfır dolgu demek değil, "fark
/// bildirilmedi" demek.
#[test]
fn ic_bosluk_tercihi_temaya_gecer() {
    use gpui_bilesenleri_galeri::TezgahİçBoşluğu;

    // `Tema` bildirim üretmez: kütüphane varsayılanı geçerli kalır.
    assert!(TezgahİçBoşluğu::Tema.kanonik().is_none());

    // Diğer ikisi geçerli bir fark kurar; boş fark kanonikte reddedilir.
    for değer in [TezgahİçBoşluğu::Dar, TezgahİçBoşluğu::Geniş] {
        assert!(
            değer.kanonik().is_some(),
            "{} farkı kurulamadı",
            değer.adı()
        );
    }

    let mut t = TezgahTercihleri::default();
    assert_eq!(t.tema.iç_boşluk, TezgahİçBoşluğu::Tema);
    let taban = gpui_bilesenleri_galeri::tezgah_teması(&t.tema);
    assert!(taban.metin_düzenleme_iç_boşluğu.is_none());

    t.tema.iç_boşluk = TezgahİçBoşluğu::Geniş;
    let geniş = gpui_bilesenleri_galeri::tezgah_teması(&t.tema);
    assert!(geniş.metin_düzenleme_iç_boşluğu.is_some());
    // Dar ile geniş aynı farkı üretmemeli, yoksa eksen tek değer taşır.
    t.tema.iç_boşluk = TezgahİçBoşluğu::Dar;
    let dar = gpui_bilesenleri_galeri::tezgah_teması(&t.tema);
    assert_ne!(
        dar.metin_düzenleme_iç_boşluğu,
        geniş.metin_düzenleme_iç_boşluğu
    );
}

/// Kod paneli tercihi olduğu gibi yazar; sabit değer basmaz.
///
/// Panel "karşılığı olan kod" diyor: bir tercih değişip panel değişmiyorsa
/// kopyalanan kod ekrandakinden başka bir alan üretir. İki eksen böyle
/// kaçmıştı — bölüt sınırı kodda `true` sabitlenmişti, yuva adı hiç
/// yazılmıyordu.
#[test]
fn kod_paneli_a_bolumu_eksenlerini_tam_yazar() {
    use gpui_bilesenleri_galeri::TezgahBölütü;

    // `§23` bölüt sınırı: kod sabit `true` basıyordu.
    let mut t = TezgahTercihleri::default();
    t.başlangıç_bölütü = Some(TezgahBölütü::SabitMetin);
    assert!(t.kod().contains("kendi_sınırı: true"));
    t.bölüt_sınırı = false;
    assert!(
        t.kod().contains("kendi_sınırı: false"),
        "bölüt sınırı kapalıyken kod hâlâ true yazıyor"
    );

    // `§25` otomatik doldurma **bilinçli olarak** yazılmaz: `§13/3` kod
    // paneline yalnız A bölümünü koyar. Port ve izin isteyen bir tercihi
    // yapılandırma satırı gibi göstermek, kopyalayana çalışacağı sözünü
    // verirdi. Onu buraya eklemeyi denedim ve `kod_paneli_yalniz_a_bolumunu_yazar`
    // testi yakaladı.
    let mut t = TezgahTercihleri::default();
    t.otomatik_doldurma = true;
    assert!(!t.kod().contains("otomatik_doldurma"));

    // `ORT-009` yuva adı: adsız dal da yazılmalı, yoksa kopyalanan kod
    // sessizce erişilebilir ağacın dışında kalır.
    let t = TezgahTercihleri::default();
    assert!(t.kod().contains(".adla("));
    let mut t = TezgahTercihleri::default();
    t.yuva_adları = false;
    let kod = t.kod();
    assert!(!kod.contains(".adla("));
    assert!(kod.contains("adsız yuva erişilebilir ağaca girmez"));
}

/// A bölümü tercihleri değişince kod paneli de değişir.
///
/// Panel "karşılığı olan kod" diyor; sapan bir tercih panelde
/// görünmüyorsa kopyalanan kod ekrandakinden başka bir alan üretir. Yer
/// tutucu, bölüt sınırı ve yuva adı bu boşluktan tek tek kaçmıştı; bu
/// test aynı sınıfı toptan kapatır.
///
/// B (port/izin) ve D (tema/sunum) tercihleri **bilinçli olarak** dışarıda:
/// `§13/3` kod paneline yalnız A bölümünü koyar.
#[test]
fn a_bolumu_tercihleri_kod_panelini_degistirir() {
    let taban = TezgahTercihleri::default().kod();

    // Her giriş: (ad, tercihi sapmaya götüren kapanış).
    let sapmalar: Vec<(&str, Box<dyn Fn(&mut TezgahTercihleri)>)> = vec![
        (
            "yer_tutucu",
            Box::new(|t: &mut TezgahTercihleri| t.yer_tutucu = false),
        ),
        (
            "temizle",
            Box::new(|t: &mut TezgahTercihleri| t.temizle = false),
        ),
        ("arama", Box::new(|t: &mut TezgahTercihleri| t.arama = true)),
        (
            "ürün_eylemi",
            Box::new(|t: &mut TezgahTercihleri| t.ürün_eylemi = true),
        ),
        (
            "yuva_adları",
            Box::new(|t: &mut TezgahTercihleri| t.yuva_adları = false),
        ),
        (
            "erişilebilir_ad",
            Box::new(|t: &mut TezgahTercihleri| t.erişilebilir_ad = false),
        ),
        (
            "sekme_durağı",
            Box::new(|t: &mut TezgahTercihleri| t.sekme_durağı = false),
        ),
        (
            "ilk_hatada_dur",
            Box::new(|t: &mut TezgahTercihleri| t.ilk_hatada_dur = true),
        ),
        (
            "zorunlu",
            Box::new(|t: &mut TezgahTercihleri| t.zorunlu = true),
        ),
        (
            "salt_okunur",
            Box::new(|t: &mut TezgahTercihleri| t.salt_okunur = true),
        ),
        (
            "etkin",
            Box::new(|t: &mut TezgahTercihleri| t.etkin = false),
        ),
        (
            "üzerine_yazma",
            Box::new(|t: &mut TezgahTercihleri| t.üzerine_yazma = true),
        ),
        ("ön_ek", Box::new(|t: &mut TezgahTercihleri| t.ön_ek = true)),
        (
            "son_ek",
            Box::new(|t: &mut TezgahTercihleri| t.son_ek = true),
        ),
        ("sayaç", Box::new(|t: &mut TezgahTercihleri| t.sayaç = true)),
        (
            "uzunluk_sınırı",
            Box::new(|t: &mut TezgahTercihleri| t.uzunluk_sınırı = true),
        ),
        (
            "varsayılan_değer",
            Box::new(|t: &mut TezgahTercihleri| t.varsayılan_değer = true),
        ),
        (
            "başlangıç_bölütü",
            Box::new(|t: &mut TezgahTercihleri| {
                t.başlangıç_bölütü = Some(gpui_bilesenleri_galeri::TezgahBölütü::SabitMetin);
            }),
        ),
        (
            "bölüt_sınırı",
            Box::new(|t: &mut TezgahTercihleri| {
                t.başlangıç_bölütü = Some(gpui_bilesenleri_galeri::TezgahBölütü::SabitMetin);
                t.bölüt_sınırı = false;
            }),
        ),
        (
            "görünürlük",
            Box::new(|t: &mut TezgahTercihleri| {
                t.görünürlük = TezgahGörünürlüğü::Gizli;
            }),
        ),
        (
            "şekil",
            Box::new(|t: &mut TezgahTercihleri| t.şekil = DüğmeŞekli::Hap),
        ),
        // `§29.0` iki seçim ekseni ayrı alanlar: kod paneli ikisini de
        // ayrı ayrı yazmalı, biri diğerinden türetilmemeli.
        (
            "odak_seçimi",
            Box::new(|t: &mut TezgahTercihleri| t.odak_seçimi = OdakSeçimi::SonaGit),
        ),
        (
            "kabul_seçimi",
            Box::new(|t: &mut TezgahTercihleri| t.kabul_seçimi = KabulSeçimi::İmleciKoru),
        ),
    ];

    for (ad, sap) in sapmalar {
        let mut t = TezgahTercihleri::default();
        sap(&mut t);
        t.türe_uyarla();
        assert_ne!(t.kod(), taban, "{ad} sapması kod panelini değiştirmedi");
    }
}

/// Kendi türünü kuran biçimler, tür değişince düşer.
///
/// Tarih biçimleri için bu kontrol vardı; süre biçimi eklenince eksik
/// kaldı ve ekranda "Metin" yazarken biçim profili "Süre · 02:45"
/// kalıyordu — iki alan aynı soruya farklı yanıt veriyordu.
#[test]
fn tur_degisince_kendi_turunu_kuran_bicim_duser() {
    use gpui_bilesenleri_galeri::BiçimUygulaması;

    let süre_sırası = gpui_bilesenleri_galeri::BİÇİM_SEÇENEKLERİ
        .iter()
        .position(|s| matches!(s.uygulama, BiçimUygulaması::Süre(..)))
        .expect("süre biçimi listede");
    let tarih_sırası = gpui_bilesenleri_galeri::BİÇİM_SEÇENEKLERİ
        .iter()
        .position(|s| matches!(s.uygulama, BiçimUygulaması::Tarih(_)))
        .expect("tarih biçimi listede");
    let bölümlü_sırası = gpui_bilesenleri_galeri::BİÇİM_SEÇENEKLERİ
        .iter()
        .position(|s| matches!(s.uygulama, BiçimUygulaması::BölümlüTarih))
        .expect("bölümlü tarih listede");

    for (sıra, ad) in [
        (süre_sırası, "süre"),
        (tarih_sırası, "tarih"),
        (bölümlü_sırası, "bölümlü tarih"),
    ] {
        let mut t = TezgahTercihleri::default();
        t.biçim_seçeneğini_uygula(sıra);
        assert_eq!(t.biçim_seçeneği, sıra, "{ad} biçimi kurulmalı");

        // Tür satırından metne dönmek biçimi de düşürür.
        t.değer_türü = TezgahDeğerKipi::Metin;
        t.türe_uyarla();
        assert_eq!(
            t.biçim_seçeneği, 0,
            "{ad} biçimi tür değişince düşmedi; ekranda tür ile biçim \
             farklı şey söyler"
        );
    }
}

/// Maske özeti, o türde gerçekten kurulabilen maskeyi anlatır.
///
/// Tek bir metin tarih alanında kurulamayan `Özel…` desenini öneriyordu:
/// `maske_seçenekleri` tarih türünde `Desen` döndürmüyor.
#[test]
fn maske_secenekleri_ture_gore_daralir() {
    let mut t = TezgahTercihleri::default();

    // Metin: desen kurulur, tarih maskesi kurulmaz.
    assert!(t.maske_seçenekleri().contains(&TezgahMaskesi::Desen));
    assert!(!t.maske_seçenekleri().contains(&TezgahMaskesi::Tarih));

    // Tarih ailesi: tarih maskesi kurulur, desen kurulmaz.
    for tür in [
        TezgahDeğerKipi::Tarih,
        TezgahDeğerKipi::Saat,
        TezgahDeğerKipi::TarihSaat,
    ] {
        t.değer_türü = tür;
        t.türe_uyarla();
        assert!(
            t.maske_seçenekleri().contains(&TezgahMaskesi::Tarih),
            "{tür:?} tarih maskesi almalı"
        );
        assert!(
            !t.maske_seçenekleri().contains(&TezgahMaskesi::Desen),
            "{tür:?} desen maskesi almamalı"
        );
    }

    // Sayısal: hiçbiri.
    for tür in [
        TezgahDeğerKipi::Tamsayı,
        TezgahDeğerKipi::Ondalık,
        TezgahDeğerKipi::ParaBirimi,
        TezgahDeğerKipi::Yüzde,
    ] {
        t.değer_türü = tür;
        t.türe_uyarla();
        assert_eq!(t.maske_seçenekleri(), &[TezgahMaskesi::Yok], "{tür:?}");
    }
}

/// Dokuz değer türünün hepsi ekrandan seçilebilir.
///
/// Tür satırı dört **aile** gösteriyor; fiziksel `TezgahDeğerKipi` ise
/// dokuz varyant taşıyor. Kipsiz kalan varyantlar ekranda hiç
/// seçilemiyordu: `ParaBirimi` ile `Yüzde` hiçbir kip listesinde yoktu ve
/// para biçimi de kendi türünü istediği için birlikte kilitleniyordu —
/// biçimi seçmek için tür, türü seçmek için biçim gerekiyordu.
#[test]
fn dokuz_deger_turu_de_bir_kipten_secilebilir() {
    use gpui_bilesenleri_galeri::{ONDALIK_KİPLERİ, TARİH_KİPLERİ, TezgahAilesi, tür_ailesi};

    // Tek başına aile düğmesiyle gelen türler.
    let doğrudan = [TezgahDeğerKipi::Metin, TezgahDeğerKipi::Tamsayı];
    let kipli: Vec<TezgahDeğerKipi> = ONDALIK_KİPLERİ
        .iter()
        .chain(TARİH_KİPLERİ.iter())
        .map(|(_, tür)| *tür)
        .collect();

    for tür in TÜM_DEĞER_TÜRLERİ {
        let erişilir = doğrudan.contains(&tür) || kipli.contains(&tür);
        assert!(erişilir, "{tür:?} ekrandan seçilemiyor");
    }

    // Kip listeleri kendi ailelerinde kalmalı; başka ailenin kipi orada
    // görünürse tür satırı yanlış yere düğme koyar.
    for (_, tür) in ONDALIK_KİPLERİ {
        assert_eq!(tür_ailesi(tür), TezgahAilesi::Ondalık, "{tür:?}");
    }
    for (_, tür) in TARİH_KİPLERİ {
        assert_eq!(tür_ailesi(tür), TezgahAilesi::TarihZaman, "{tür:?}");
    }

    // Para biçimi artık kurulabilir: türü kiptten seçiliyor.
    let mut t = TezgahTercihleri::default();
    t.değer_türü = TezgahDeğerKipi::ParaBirimi;
    t.türe_uyarla();
    let para = gpui_bilesenleri_galeri::BİÇİM_SEÇENEKLERİ
        .iter()
        .find(|s| matches!(s.uygulama, gpui_bilesenleri_galeri::BiçimUygulaması::Para))
        .expect("para biçimi listede");
    assert!(
        t.seçenek_uygun_mu(para),
        "para biçimi kendi türünde uygun olmalı"
    );
}

/// `ORT-004 §25` anlık görüntüsü yoğunluk ve hareket tercihini de taşır.
///
/// Bu iki tercih uzun süre yalnız kendi seçicilerinde okundu: tema kurulumu
/// taban değerini (`Normal`/`Tam`) geçiriyordu, dolayısıyla üst şeritteki
/// altı düğme hiçbir şeyi değiştirmiyordu. Sayısal karşılığı tema üretmez
/// (`ORT-004 §43`), ama bağlamın doğru kurulması temanın işidir.
#[test]
fn yogunluk_ve_hareket_tercihi_baglama_gecer() {
    use gpui_bilesenleri::{ArayüzYoğunluğu, HareketTercihi};

    let mut tema = TezgahTeması::default();
    tema.yoğunluk = ArayüzYoğunluğu::Geniş;
    tema.hareket = HareketTercihi::Kapalı;
    tema.sürümü_artır();
    let anlık = gpui_bilesenleri_galeri::tezgah_teması(&tema);

    assert_eq!(anlık.bağlam.yoğunluk, ArayüzYoğunluğu::Geniş);
    assert_eq!(anlık.bağlam.hareket, HareketTercihi::Kapalı);
}

/// `ORT-004` imleç hareketi çözüm sırası: tema açık tercihi hareket
/// tercihinin önündedir, ama tema varsayılandayken hareket tercihi yanıp
/// sönmeyi durdurur.
///
/// Tezgâhta iki ayrı düğme bu sıraya girer: `İmleçHızı::Platform` temayı
/// varsayılanda bırakır, diğer hızlar açık tercih kurar. İkisinin birlikte
/// doğru çözülmesi hareket düğmesinin görünür etkisidir.
#[test]
fn hareket_kapali_iken_platform_hizi_imleci_sabitler() {
    use gpui_bilesenleri::{HareketTercihi, MetinİmleciHareketKaynağı, MetinİmleciHareketi};
    use gpui_bilesenleri_galeri::İmleçHızı;

    let mut tema = TezgahTeması::default();
    tema.imleç_hızı = İmleçHızı::Platform;
    tema.hareket = HareketTercihi::Kapalı;
    tema.sürümü_artır();
    let anlık = gpui_bilesenleri_galeri::tezgah_teması(&tema);

    let çözüm = gpui_bilesenleri::metin_imleci_hareketini_çöz(
        anlık.metin_imleci.hareket,
        anlık.bağlam.hareket,
        None,
    );
    assert_eq!(çözüm.hareket, MetinİmleciHareketi::Sabit);
    assert_eq!(çözüm.kaynak, MetinİmleciHareketKaynağı::HareketTercihi);

    // Açık tema tercihi hareket kapalıyken bile kazanır: ürün bilerek
    // seçmiştir ve sıra `ORT-004`ündür.
    tema.imleç_hızı = İmleçHızı::Hızlı;
    tema.sürümü_artır();
    let açık = gpui_bilesenleri_galeri::tezgah_teması(&tema);
    let çözüm = gpui_bilesenleri::metin_imleci_hareketini_çöz(
        açık.metin_imleci.hareket,
        açık.bağlam.hareket,
        None,
    );
    assert_eq!(çözüm.kaynak, MetinİmleciHareketKaynağı::Tema);
    assert!(matches!(
        çözüm.hareket,
        MetinİmleciHareketi::YanıpSönen { .. }
    ));
}

/// Yoğunluk tercihi tezgâh dolgularını gerçekten değiştirir.
///
/// `ORT-004 §43` sayısal karşılığı bileşen sahibinin görünüm profiline
/// bırakır; tezgâhın profili `TezgahGörünümProfili`dir ve katsayı orada
/// çözülür. Üç kip birbirinden ayırt edilebilir olmalı: aksi hâlde düğme
/// seçili görünür ama hiçbir şey olmaz.
#[test]
fn yogunluk_tezgah_dolgularini_olcekler() {
    use gpui_bilesenleri::ArayüzYoğunluğu;

    let kimlik = gpui_bilesenleri::GörünümProfiliKimliği(
        gpui_bilesenleri::TanımKimliği::denetimli(
            std::sync::Arc::from("galeri.tezgah"),
            std::sync::Arc::from("tasarım"),
        )
        .unwrap(),
    );
    let ölç = |yoğunluk| {
        let mut tema = TezgahTeması::default();
        tema.yoğunluk = yoğunluk;
        tema.sürümü_artır();
        let anlık = gpui_bilesenleri_galeri::tezgah_teması(&tema);
        gpui_bilesenleri_galeri::tezgah::TezgahGörünümProfili::tasarım(kimlik.clone())
            .çöz(&anlık)
            .unwrap()
    };

    let kompakt = ölç(ArayüzYoğunluğu::Kompakt);
    let normal = ölç(ArayüzYoğunluğu::Normal);
    let geniş = ölç(ArayüzYoğunluğu::Geniş);

    assert!(kompakt.hap.dikey_dolgu < normal.hap.dikey_dolgu);
    assert!(normal.hap.dikey_dolgu < geniş.hap.dikey_dolgu);
    assert!(kompakt.kart.yatay_dolgu < geniş.kart.yatay_dolgu);

    // `ORT-004 §1240`: yoğunluk `ORT-009` asgari etkileşim hedefinin
    // altına inemez. Kompakt kip dolguyu daraltır, hedefi değil.
    assert_eq!(kompakt.anahtar_yüksekliği, normal.anahtar_yüksekliği);
    assert_eq!(kompakt.simge_düğmesi, geniş.simge_düğmesi);
    assert_eq!(
        kompakt.önizleme_kabuğu.yükseklik,
        geniş.önizleme_kabuğu.yükseklik
    );
}

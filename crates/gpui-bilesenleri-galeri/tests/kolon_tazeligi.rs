//! Önbellekli sağ kolonun tazelik ve kazanç kapısı.
//!
//! Kolon `Entity::cached` sınırındadır. Bu dosya iki yönü birden sabitler,
//! çünkü biri olmadan diğeri değersizdir:
//!
//! 1. **Tazelik:** kolonu ilgilendiren her kök değişimi (tercih, tür, tema,
//!    açık seçici, dış bildirim) kolonu yeniden kurdurur. Kurmazsa ekranda
//!    bayat bir yapılandırma yüzeyi kalır.
//! 2. **Kazanç:** tuş vuruşu ve temiz kare kolonu **kurdurmaz**. Kurdurursa
//!    önbellek hiçbir işe yaramıyor demektir.
//!
//! Tarihçe, bu kapının neden davranışsal olduğunu açıklıyor. İkinci turda
//! kolon `cached`e alınmıştı ve yapısal bekçiler (`.cached(` dizgesi var mı,
//! `observe(kök` var mı) yeşildi; WASM'de gözle yapılan denemeler de
//! "çalışıyor" diyordu. Üçüncü turun ölçüm koşumuna eklenen çizim sayacı
//! tersini gösterdi: kolon ilk çizimden sonra **hiç** yeniden kurulmuyordu,
//! çünkü GPUI'de `notify` bir `cached` sınırını patlatmaz (`App::notify`
//! yalnız pencerenin `tracked_entities` kümesindeki entity'ler için
//! `invalidate_view` çağırır ve önbellekten dönen view o kümeye kendi
//! kimliğiyle girmez). Çalışan yol `refresh`tir; kök artık
//! `kolonu_geçersizle` ile onu çağırıyor.
//!
//! Ayrıntı: `raporlar/PERFORMANS_MIMARISI.md` §6.

#![allow(non_ascii_idents)]

use gpui::{AnyWindowHandle, Context, TestApp, TestAppWindow, Window};
use gpui_bilesenleri_galeri::{
    GaleriUygulaması, TezgahDeğerKipi, bileşen_tuş_bağlarını_kur, bölüm_çizim_sayısı,
};

fn kur() -> (TestApp, TestAppWindow<GaleriUygulaması>) {
    let mut uygulama = TestApp::new();
    uygulama.update(bileşen_tuş_bağlarını_kur);
    let pencere = uygulama.open_window(|_, _| GaleriUygulaması::yeni());
    (uygulama, pencere)
}

/// Mutasyonu uygular ve efekt döngüsü bittikten sonra kolonun kaç kez
/// kurulduğunu döner.
///
/// Ölçüm noktası bilinçli olarak "efektler bittikten sonra"dır: bu, "değişim
/// ekrana yansıdı mı" sorusudur ve `refresh` yolu efekt üzerinden işlediği
/// için tek doğru soru budur. Tek bir `draw`'a bakmak, mutasyonun etkisini
/// bir kare ıskalayabilir.
fn değişimden_sonra(
    uygulama: &mut TestApp,
    pencere: AnyWindowHandle,
    mutasyon: impl FnOnce(&mut GaleriUygulaması, &mut Window, &mut Context<GaleriUygulaması>),
) -> u64 {
    let önce = bölüm_çizim_sayısı();
    uygulama.update(|bağlam| {
        pencere
            .update(bağlam, |kök, pencere, bağlam| {
                let görsel = kök
                    .downcast::<GaleriUygulaması>()
                    .expect("kök görünüm tezgâhtır");
                görsel.update(bağlam, |uygulama, bağlam| {
                    mutasyon(uygulama, pencere, bağlam);
                });
            })
            .expect("pencere açık");
    });
    bölüm_çizim_sayısı() - önce
}

/// Hiçbir şey değiştirmeden bir kare çizer ve kolonun kurulup kurulmadığını
/// döner. Önbellek çalışıyorsa sıfırdır.
fn temiz_kare(uygulama: &mut TestApp, pencere: AnyWindowHandle) -> u64 {
    let önce = bölüm_çizim_sayısı();
    uygulama.update(|bağlam| {
        pencere
            .update(bağlam, |_, pencere, bağlam| {
                pencere.draw(bağlam).clear(bağlam);
            })
            .expect("pencere açık");
    });
    bölüm_çizim_sayısı() - önce
}

#[test]
fn kök_değişimleri_kolonu_tazeler() {
    let (mut uygulama, pencere) = kur();
    let tutamaç: AnyWindowHandle = pencere.handle().into();

    // Tercih: bölüm içerikleri, `§29` raporu ve kod paneli buna bağlı.
    assert!(
        değişimden_sonra(&mut uygulama, tutamaç, |uygulama, _, bağlam| {
            uygulama.tezgahı_değiştir(|tercih| tercih.sayaç = !tercih.sayaç, bağlam);
        }) > 0,
        "tercih değişimi kolonu tazelemiyor: yapılandırma yüzeyi bayat kalır"
    );

    // Değer türü: `§9` tür süzgeci bölüm **listesini** değiştirir.
    assert!(
        değişimden_sonra(&mut uygulama, tutamaç, |uygulama, _, bağlam| {
            uygulama.tezgahı_değiştir(
                |tercih| tercih.değer_türü = TezgahDeğerKipi::Ondalık,
                bağlam,
            );
        }) > 0,
        "tür değişimi kolonu tazelemiyor: bölüm listesi bayat kalır"
    );

    // Tema: palet ve çözülmüş görünüm kolonun bütün yüzlerini besler.
    assert!(
        değişimden_sonra(&mut uygulama, tutamaç, |uygulama, _, bağlam| {
            uygulama.galeri_kipini_değiştir(bağlam);
        }) > 0,
        "tema değişimi kolonu tazelemiyor: kolon eski palette kalır"
    );

    // Açık seçici: kolondaki açılır listelerin yüzü ve tembel içeriği buna
    // bakar — açılmayan bir liste kullanıcının erişemediği bir eksendir.
    assert!(
        değişimden_sonra(&mut uygulama, tutamaç, |uygulama, _, bağlam| {
            uygulama.seçiciyi_değiştir("adım", bağlam);
        }) > 0,
        "seçici değişimi kolonu tazelemiyor: liste açılmaz"
    );

    // `§16` dış bildirim: port kapıları kartı `doğrulama_portu`nu okur.
    assert!(
        değişimden_sonra(&mut uygulama, tutamaç, |uygulama, pencere, bağlam| {
            uygulama.tezgah_dış_bildirimi(true, pencere, bağlam);
        }) > 0,
        "dış bildirim kolonu tazelemiyor: port rozeti bayat kalır"
    );
}

#[test]
#[cfg_attr(
    feature = "olcum-onbelleksiz",
    ignore = "ölçüm tabanı bayrağı önbelleği kapatır; kazanç kapısı o koşumda anlamsız"
)]
fn tuş_vuruşu_ve_temiz_kare_kolonu_kurmaz() {
    let (mut uygulama, pencere) = kur();
    let tutamaç: AnyWindowHandle = pencere.handle().into();

    // Isınma: ilk yazma alanı odaklar ve `Window::focus` `refresh()`
    // çağırır (odak halkası ve tuş yönlendirmesi bütün pencereyi
    // ilgilendirir), yani o kare önbellekleri atlar. Bu bir kusur değil,
    // gerçek akışın kendisi: kullanıcı bir kez odaklanır, sonra yazar.
    // Ölçüm yazma evresini hedefler.
    değişimden_sonra(&mut uygulama, tutamaç, |uygulama, pencere, bağlam| {
        uygulama.ölçüm_alanına_yaz("ısınma", pencere, bağlam);
    });

    assert_eq!(
        temiz_kare(&mut uygulama, tutamaç),
        0,
        "temiz karede kolon kuruluyor: önbellek hiç isabet etmiyor"
    );

    // Asıl kazanç: odaklı alanda yazmak alanı ve onu gözleyen panelleri
    // kirletir, kolonu değil.
    for sıra in 0..3 {
        assert_eq!(
            değişimden_sonra(&mut uygulama, tutamaç, |uygulama, pencere, bağlam| {
                uygulama.ölçüm_alanına_yaz(&format!("tazelik {sıra}"), pencere, bağlam);
            }),
            0,
            "tuş vuruşu #{sıra}: kolon kuruluyor — alan bildirimi kolona sızıyor"
        );
    }

    // Vuruşlardan sonra da önbellek ayakta olmalı.
    assert_eq!(
        temiz_kare(&mut uygulama, tutamaç),
        0,
        "tuş vuruşlarından sonra önbellek bozulmuş"
    );
}

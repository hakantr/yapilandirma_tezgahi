//! Sağ kolonun tazelik kapısı.
//!
//! Kolon, kökün durumundan (tercih, rapor, portlar, açık seçici, tema)
//! çizilir. Bu dosya tek bir şeyi sabitler: **her çizimde kolon gerçekten
//! kurulur**, yani ekrandaki yapılandırma yüzeyi bayat olamaz.
//!
//! Kapının gerekçesi tarihseldir. İkinci turda kolon `Entity::cached`
//! sınırına alınmıştı; kod okuması ve yapısal bekçiler doğru görünüyordu,
//! WASM'de gözle yapılan denemeler de "çalışıyor" diyordu. Üçüncü turun
//! ölçüm koşumuna eklenen çizim sayacı tersini gösterdi: kolon açılıştaki
//! ilk çizimden sonra **hiç** yeniden kurulmuyordu — ne kökün bildirimi,
//! ne panele doğrudan `notify`, ne de `refresh_windows` önbelleği
//! patlatıyordu. Bu bir hız kazancı değil, bayat yapılandırma yüzeyi
//! demekti; önbellek geri alındı.
//!
//! Bu yüzden test sayı değil **davranış** sabitler ve `cached` yeniden
//! denenirse ilk düşecek yer burasıdır.

#![allow(non_ascii_idents)]

use gpui::{AnyWindowHandle, Context, TestApp, Window};
use gpui_bilesenleri_galeri::{
    GaleriUygulaması, bileşen_tuş_bağlarını_kur, bölüm_çizim_sayısı,
};

/// Mutasyonu uygular, efektleri boşaltır, sonra pencereyi bir kez çizer ve
/// o çizimde kolonun kaç kez kurulduğunu döner.
///
/// Efekt boşaltma **çizimden önce** olmalı: bildirim zinciri efekt
/// döngüsünde işlenir. `TestApp::update` bloğun sonunda `run_until_parked`
/// koşar; test platformu kendiliğinden çizim yapmaz, bu yüzden aşağıdaki
/// `draw` karedeki tek çizimdir.
fn çiz_ve_say(
    uygulama: &mut TestApp,
    pencere: AnyWindowHandle,
    mutasyon: impl FnOnce(&mut GaleriUygulaması, &mut Window, &mut Context<GaleriUygulaması>),
) -> u64 {
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
fn kolon_her_çizimde_kurulur() {
    let mut uygulama = TestApp::new();
    uygulama.update(|bağlam| bileşen_tuş_bağlarını_kur(bağlam));
    let pencere = uygulama.open_window(|_, _| GaleriUygulaması::yeni());
    let tutamaç: AnyWindowHandle = pencere.handle().into();

    // Değişiklik yokken bile kolon kurulur: GPUI gerçekleşen her çizimde
    // kökten render eder ve kolon o çizimin parçasıdır. Sıfır dönmesi,
    // kolonun bir önbellekte donduğu anlamına gelir.
    for sıra in 0..3 {
        assert_eq!(
            çiz_ve_say(&mut uygulama, tutamaç, |_, _, _| {}),
            1,
            "temiz kare #{sıra}: kolon kurulmuyor — donmuş bir önbellek var"
        );
    }

    // Asıl kapı: tercih değişimi ekrandaki yapılandırma yüzeyine ulaşmalı.
    // Kolon kurulmazsa kullanıcı eski bölümleri görür.
    assert_eq!(
        çiz_ve_say(&mut uygulama, tutamaç, |uygulama, _, bağlam| {
            uygulama.tezgahı_değiştir(|tercih| tercih.sayaç = !tercih.sayaç, bağlam);
        }),
        1,
        "tercih değişiminden sonra kolon kurulmuyor: yüzey bayat kalır"
    );

    // Değer türü değişimi bölüm **listesini** değiştirir (`§9` tür
    // süzgeci): sayısal adım gelir, maske satırı kapanır. Kolon bu karede
    // de kurulmalı.
    assert_eq!(
        çiz_ve_say(&mut uygulama, tutamaç, |uygulama, _, bağlam| {
            uygulama.tezgahı_değiştir(
                |tercih| tercih.değer_türü = gpui_bilesenleri_galeri::TezgahDeğerKipi::Ondalık,
                bağlam,
            );
        }),
        1,
        "tür değişiminden sonra kolon kurulmuyor: bölüm listesi bayat kalır"
    );

    // Tuş vuruşu da kolonu kurar (bugünkü gerçek). Bu satır bir hedef
    // değil, **kayıt**: kolonu tuş vuruşundan ayırma denemesi geri
    // gelirse, yukarıdaki tazelik kapılarıyla birlikte kanıtlanmalı.
    assert_eq!(
        çiz_ve_say(&mut uygulama, tutamaç, |uygulama, pencere, bağlam| {
            uygulama.ölçüm_alanına_yaz("tazelik", pencere, bağlam);
        }),
        1,
        "tuş vuruşu karesinde kolon kurulmuyor: bugün beklenen davranış bu değil"
    );
}

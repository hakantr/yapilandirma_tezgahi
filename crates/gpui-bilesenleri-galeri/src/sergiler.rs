//! Galeriye ait GPUI görünüm bağdaştırıcıları.
//!
//! Davranış ve kabul kuralları kanonik BİL sözleşmelerinde kalır. Bu modül,
//! yayımlanmış aileleri gerçek GPUI hedefleri olarak görünür ve etkileşimli
//! kılar; yalnız açıklama veya ekran görüntüsü üretmez.

use crate::{GaleriUygulaması, aile_görünen_adı};
use gpui::{
    AnyElement, Context, Div, Entity, IntoElement, SharedString, Stateful, div, prelude::*, px,
    rgb, svg,
};
use gpui_bilesenleri::{
    Aktarımİlerlemesi, AraçBölgesi, BağlantıYetenekleri, BildirimTürü, DisclosureTetikleyicisi,
    DosyaAktarımYönü, DüğmeVurgusu, ErişimDurumu, FormGönderimDurumu, GezinmeHedefiKimliği,
    GezinmeYönelimi, GezinmeÖğesi, GirişKutusu, KısayolÇakışmaKararı, KısayolÇakışması,
    MantıksalDeğer, MedyaAnlıkGörüntüsü, MedyaDurumu, ModalTürü, PanelKonumu, RenkYüzeyi, Rgba8,
    SekmeKipi, SeçiciSunumu, SeçimKipi, SözdizimiVurgusu, SürekliDeğer, SıralamaYönü,
    TakvimEtkileşimKaynağı, VurguKaynağı, YakalamaBağlamı, YüzenGrupDurumu, bağlantı_eylemleri,
    gezinme_sunumu, görsel_konum_göstergesi, kodu_doğrula, medya_denetim_bağdaştırıcıları,
    oynatma_niyetini_teslim_et, sözdizimi_çöz, tuşu_yakala, yönetilen_ayar_sunumu, çakışmayı_çöz,
    İlerlemeDeğeri,
};

/// Tezgâhın ölçü sistemi.
///
/// Değerler `varliklar/tezgah/metinkutusu.cozulmus.html` içindeki tasarımdan
/// birebir alındı.
///
/// Burada yalnız tezgâh **kabuğunun** ölçüleri kalır. Kutu ölçüleri —
/// düğme yüksekliği, dolgusu, köşe yarıçapı, şerit dolgusu — `ORT-017`
/// görünüm profiline taşındı ve yüzler üzerinden okunur; ham sabit olarak
/// tutulsalardı tema ve metin ölçeğinden bağımsız kalırlardı.
pub(crate) mod ölçü {
    /// Izgara ve şerit sütunları arası (`gap: 6px`).
    pub const ARALIK: f32 = 6.;
    /// Yüzer listelerin taban genişliği.
    ///
    /// GPUI'de `overflow-wrap` yok. Kap bunun altına inince uzun bir metin
    /// harf harf alt alta sarılır — okunmaz bir sütun olur. Taban, en uzun
    /// aile adının rahat sığdığı ölçüdür.
    pub const LİSTE_ASGARİSİ: f32 = 220.;
    /// Ön ek ve son ek alanlarının genişliği.
    ///
    /// Sabit: içerik genişliğine bırakılırsa durum etiketi "açık"tan
    /// "kapalı"ya döndüğünde blok ve altındaki metin kutusu oynuyor.
    pub const EK_ALANI: f32 = 208.;
    /// Açık/kapalı etiketinin genişliği; iki sözcük de aynı yeri kaplar.
    pub const DURUM_ETİKETİ: f32 = 52.;
    /// Üst şerit seçicilerinin taban genişliği (`select` karşılığı).
    pub const SEÇİCİ_ASGARİSİ: f32 = 160.;
    /// Açılır listenin azami yüksekliği; uzun liste pencereyi taşırmasın.
    pub const AÇILIR_YÜKSEKLİĞİ: f32 = 320.;
    /// Kod bloğunun taban yüksekliği; boş kodda da blok olarak durur.
    pub const KOD_YÜKSEKLİĞİ: f32 = 180.;
    /// Özel yarıçap panelinin genişliği; kaydırıcı ve iki etiket sığar.
    pub const YARIÇAP_PANELİ: f32 = 260.;
    /// Tezgâh içeriğinin azami genişliği.
    ///
    /// `404px` önizleme kolonu + `28px` aralık + `524px` yapılandırma
    /// kolonu. Geniş kolonda kısa etiketli düğmeler satır boyunca dağılıyor
    /// ve kartlar çoğunlukla boş duruyordu; dar kolonda aynı düğmeler alt
    /// alta sarılıp okunur bir blok oluşturuyor.
    pub const İÇERİK_GENİŞLİĞİ: f32 = 956.;
    /// Tezgâh içeriğinin asgari genişliği.
    ///
    /// Azami genişlikle aynı: tezgâh **sabit** genişliktedir ve pencere
    /// daralınca sayfa yatay kaydırır. Kolonları daraltmak yerine kaydırmak
    /// bilinçli — simge şeritleri ve düğme ızgaraları bu genişliğe göre
    /// hizalanmış.
    pub const İÇERİK_ASGARİSİ: f32 = 956.;
    /// Simge çizimi (`svg width/height: 15`).
    pub const SİMGE: f32 = 15.;
    /// Değer türü düğmesi (`height: 30px`).
    pub const TÜR_DÜĞMESİ: f32 = 30.;
    /// Aile düğmesinin yatay dolgusu.
    ///
    /// Izgaradan çıkınca düğme içerik genişliğine iniyor; dört kısa ad
    /// (`Metin`, `Ondalık`) yan yana sıkışık durmasın diye dolgu geniş.
    pub const TÜR_GENİŞ_DOLGU: f32 = 20.;
    /// Önizleme kutusu (`height: 58px`) — tema ölçüsünden gelir, burada
    /// yalnız blok araları tanımlıdır.
    pub const BLOK_ARASI: f32 = 12.;
}

fn kenarlık() -> u32 {
    crate::palet().kabuk_kenarlık
}
fn ikincil_metin() -> u32 {
    crate::palet().kabuk_ikincil_metin
}
fn kabuk_vurgusu() -> u32 {
    crate::palet().kabuk_vurgu
}

pub(crate) struct SergiDurumu {
    /// `BİL-010` yaşayan bileşen varlıkları; galeri metni kendisi tutmaz.
    pub girişi: crate::MetinGirişiAlanları,
    /// `BİL-010` tezgâh tercihleri ve yaşayan önizleme alanı.
    pub tezgah: crate::TezgahTercihleri,
    pub tezgah_alanı: Entity<GirişKutusu>,
    /// Açık yüzer tercih kutusu. `render` sırasında varlık okunamaz, bu
    /// yüzden durum buradan taşınır.
    /// `§25` platform otomatik doldurmayı sunuyor mu? Tercih yalnız o zaman
    /// çizilir.
    pub doldurma_var: bool,
    /// `B` bölümünün port kapıları.
    pub portlar: crate::PortDurumu,
    /// `§29` kanonik doğrulama raporu.
    pub rapor: gpui_bilesenleri::GirişYapılandırmaRaporu,
    /// `§26` alanın yayımladığı son olaylar; en yeni **başta**.
    pub olaylar: Vec<crate::TezgahOlayı>,
    /// Köşe kaydırma çubuğunun izinin ekrandaki yeri.
    pub köşe_izi: std::rc::Rc<std::cell::Cell<gpui::Bounds<gpui::Pixels>>>,
    /// İşletim sisteminde kurulu yazı tipi aileleri; WASM'de boştur.
    pub sistem_aileleri: std::rc::Rc<Vec<String>>,
    /// `ORT-002 §5.2` çözülmüş saat dilimi ve kaynağı.
    pub saat_dilimi: crate::ÇözülmüşSaatDilimi,
    pub düğme_sayacı: u32,
    pub seçili: u8,
    pub onaylı: bool,
    pub sekme: u8,
    pub panel_açık: bool,
    pub araç_taşması_açık: bool,
    pub modal_açık: bool,
    pub seçici_sonucu: u8,
    pub tablo_azalan: bool,
    pub bildirim_açık: bool,
    pub form_gönderildi: bool,
    pub sürekli_değer: u8,
    pub ilerleme: u8,
    pub takvim_günü: u8,
    pub disclosure_açık: bool,
    pub renk_seçimi: u8,
    pub aktarım: u8,
    pub arama_eşleşmesi: u8,
    pub kısayol_değiştirildi: bool,
    pub ayar_koyu: bool,
    pub bağlantı_başarılı: bool,
    pub kod_satırı: u8,
    pub yüzen_grup_açık: bool,
    pub gezinme_hedefi: u8,
    pub görsel_konumu: u8,
    pub kod_sembolü_qr: bool,
    pub medya_niyeti: bool,
    pub ort_durumları: u32,
    pub kab_durumları: u16,
}

pub(crate) fn öne_çıkan_sergiler(
    durum: SergiDurumu,
    bağlam: &mut Context<GaleriUygulaması>,
) -> impl IntoElement {
    div()
        .id("canlı-temel-sergiler")
        .mb_6()
        .rounded_lg()
        .border_1()
        .border_color(rgb(kenarlık()))
        .bg(rgb(crate::palet().kabuk_kart))
        .p_4()
        .child(div().text_lg().child("Canlı temel bileşenler"))
        .child(
            div()
                .mt_1()
                .text_sm()
                .text_color(rgb(ikincil_metin()))
                .child("Tıklayın, yazın ve durum değişimini doğrudan gözlemleyin."),
        )
        .child(
            div()
                .mt_4()
                .flex()
                .flex_row()
                .flex_wrap()
                .gap_3()
                .child(düğme_sergisi(durum.düğme_sayacı, bağlam))
                .child(metin_girişi_özeti(durum.girişi.clone()))
                .child(seçim_sergisi(durum.seçili, bağlam))
                .child(mantıksal_giriş_sergisi(durum.onaylı, bağlam)),
        )
}

/// Genel bakıştaki canlı metin girişi özeti.
///
/// Burası katalog vitrinidir: tek bir yaşayan alan, bileşenin canlı olduğunu
/// gösterir. Yapılandırmanın tamamı ailenin kendi tezgâhındadır; vitrine
/// sabit varyant dizmek o tezgâhı tekrar etmek olur.
fn metin_girişi_özeti(alanlar: crate::MetinGirişiAlanları) -> Stateful<Div> {
    sergi_kartı(
        "bil-010-özet",
        "Metin Girişi",
        "Maske, biçim, doğrulama ve IME destekli metin alanı",
    )
    .child(
        div()
            .mt_4()
            .text_xs()
            .text_color(rgb(ikincil_metin()))
            .child("Yer tutucu · temizleme · sayaç · uzunluk sınırı"),
    )
    .child(div().id("bil-010-özet-alanı").mt_1().child(alanlar.yalın))
    .child(
        div()
            .mt_3()
            .text_xs()
            .text_color(rgb(ikincil_metin()))
            .child("Tüm yapılandırma yüzeyi için aileyi açın."),
    )
}

pub(crate) fn aile_sergisi(
    sözleşme: &str,
    durum: SergiDurumu,
    bağlam: &mut Context<GaleriUygulaması>,
) -> AnyElement {
    match sözleşme {
        "ORT-001" | "ORT-002" | "ORT-003" | "ORT-004" | "ORT-005" | "ORT-006" | "ORT-007"
        | "ORT-008" | "ORT-009" | "ORT-010" | "ORT-011" | "ORT-012" | "ORT-013" | "ORT-014"
        | "ORT-015" | "ORT-016" | "ORT-017" | "ORT-018" | "ORT-019" | "ORT-020" | "ORT-021"
        | "ORT-022" | "ORT-023" => {
            ort_laboratuvarı(sözleşme, durum.ort_durumları, bağlam).into_any_element()
        }
        "KAB-010" | "KAB-020" | "KAB-030" | "KAB-040" | "KAB-050" | "KAB-060" | "KAB-070"
        | "KAB-080" | "KAB-090" | "KAB-100" => {
            kabuk_simülasyonu(sözleşme, durum.kab_durumları, bağlam).into_any_element()
        }
        // `BİL-010` ekranı yalnız tezgâhtır: hazır örnek kartları kaldırıldı.
        // Alanın nasıl davrandığını sabit bir vitrin değil, programcının
        // kendi kurduğu yapılandırma göstermeli.
        "BİL-010" => tezgah_sergisi(
            durum.tezgah.clone(),
            durum.tezgah_alanı.clone(),
            &durum.girişi,
            durum.köşe_izi.clone(),
            TezgahPlatformu {
                sistem_aileleri: durum.sistem_aileleri.clone(),
                saat_dilimi: durum.saat_dilimi.clone(),
                doldurma_var: durum.doldurma_var,
                portlar: durum.portlar,
                rapor: durum.rapor.clone(),
                olaylar: durum.olaylar.clone(),
            },
            bağlam,
        )
        .into_any_element(),
        "BİL-020" => seçim_sergisi(durum.seçili, bağlam).into_any_element(),
        "BİL-030" => mantıksal_giriş_sergisi(durum.onaylı, bağlam).into_any_element(),
        "BİL-040" => düğme_sergisi(durum.düğme_sayacı, bağlam).into_any_element(),
        "BİL-050" => sekme_sergisi(durum.sekme, bağlam).into_any_element(),
        "BİL-060" => panel_sergisi(durum.panel_açık, bağlam).into_any_element(),
        "BİL-070" => araç_çubuğu_sergisi(durum.araç_taşması_açık, bağlam).into_any_element(),
        "BİL-080" => modal_sergisi(durum.modal_açık, bağlam).into_any_element(),
        "BİL-090" => seçici_sergisi(durum.seçici_sonucu, bağlam).into_any_element(),
        "BİL-100" => veri_tablosu_sergisi(durum.tablo_azalan, bağlam).into_any_element(),
        "BİL-110" => bildirim_sergisi(durum.bildirim_açık, bağlam).into_any_element(),
        "BİL-120" => form_sergisi(durum.form_gönderildi, bağlam).into_any_element(),
        "BİL-130" => sürekli_değer_sergisi(durum.sürekli_değer, bağlam).into_any_element(),
        "BİL-140" => ilerleme_sergisi(durum.ilerleme, bağlam).into_any_element(),
        "BİL-150" => takvim_sergisi(durum.takvim_günü, bağlam).into_any_element(),
        "BİL-160" => disclosure_sergisi(durum.disclosure_açık, bağlam).into_any_element(),
        "BİL-170" => renk_sergisi(durum.renk_seçimi, bağlam).into_any_element(),
        "BİL-180" => aktarım_sergisi(durum.aktarım, bağlam).into_any_element(),
        "BİL-190" => arama_sergisi(durum.arama_eşleşmesi, bağlam).into_any_element(),
        "BİL-200" => kısayol_sergisi(durum.kısayol_değiştirildi, bağlam).into_any_element(),
        "BİL-210" => ayar_sergisi(durum.ayar_koyu, bağlam).into_any_element(),
        "BİL-220" => bağlantı_sergisi(durum.bağlantı_başarılı, bağlam).into_any_element(),
        "BİL-230" => kod_sergisi(durum.kod_satırı, bağlam).into_any_element(),
        "BİL-250" => yüzen_eylem_sergisi(durum.yüzen_grup_açık, bağlam).into_any_element(),
        "BİL-260" => gezinme_sergisi(durum.gezinme_hedefi, bağlam).into_any_element(),
        "BİL-270" => görsel_sergisi(durum.görsel_konumu, bağlam).into_any_element(),
        "BİL-280" => kod_sembolü_sergisi(durum.kod_sembolü_qr, bağlam).into_any_element(),
        "BİL-290" => medya_sergisi(durum.medya_niyeti, bağlam).into_any_element(),
        _ => div()
            .mt_4()
            .rounded_md()
            .border_1()
            .border_color(rgb(kenarlık()))
            .bg(rgb(crate::palet().kabuk_zemin))
            .p_4()
            .text_sm()
            .text_color(rgb(ikincil_metin()))
            .child("Bu aile için etkileşimli görünüm bağdaştırıcısı sıradaki pakettedir.")
            .into_any_element(),
    }
}

fn sergi_kartı(
    kimlik: &'static str,
    başlık: &'static str,
    açıklama: &'static str,
) -> Stateful<Div> {
    div()
        .id(kimlik)
        .w(px(280.))
        .min_h(px(230.))
        .rounded_lg()
        .border_1()
        .border_color(rgb(kenarlık()))
        .bg(rgb(crate::palet().kabuk_kart))
        .p_4()
        .child(
            div()
                .text_sm()
                .text_color(rgb(kabuk_vurgusu()))
                .child(başlık),
        )
        .child(
            div()
                .mt_1()
                .text_xs()
                .text_color(rgb(ikincil_metin()))
                .child(açıklama),
        )
}

fn ort_laboratuvarı(
    sözleşme: &str,
    durumlar: u32,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Stateful<Div> {
    let (sıra, açıklama, önce, sonra, eylem, kanonik) = match sözleşme {
        "ORT-001" => (
            0,
            "Hedef başlatma ve yaşayan GPUI kökü",
            "Kök: Hazırlanıyor",
            "Kök: Yaşıyor · WASM",
            "Kökü başlat",
            "Platform hedefi + uygulama yaşamı",
        ),
        "ORT-002" => (
            1,
            "Unicode metin, yerel yön ve güvenli sınır",
            "Merhaba · LTR",
            "مرحبا · RTL",
            "Yazı yönünü değiştir",
            "UnicodeMetinMotoru",
        ),
        "ORT-003" => (
            2,
            "Kutu şekli, köşe ve paylaşılan kenar",
            "Dikdörtgen köşeler",
            "Yuvarlatılmış köşeler",
            "Şekli değiştir",
            "KutuŞekliÇözücüsü",
        ),
        "ORT-004" => (
            3,
            "Etkileşim durumu ve semantik tema rolü",
            "Açık tema · Olağan",
            "Koyu tema · Odaklı",
            "Tema ve durumu değiştir",
            "OrtakGörselDurum",
        ),
        "ORT-005" => (
            4,
            "Odak sırası ve klavye bölge turu",
            "Odak: Birinci hedef",
            "Odak: İkinci hedef",
            "Sonraki odağa geç",
            "GezintiHaritası",
        ),
        "ORT-006" => (
            5,
            "Tutturulmuş yüzen yüzey ve dışarı tıklama",
            "Yüzen yüzey: Kapalı",
            "Yüzen yüzey: Açık",
            "Yüzeyi aç/kapat",
            "Yüzen yüzey yaşam döngüsü",
        ),
        "ORT-007" => (
            6,
            "Sürümlü eşzamansız iş ve eski sonuç reddi",
            "İş #1: Bekliyor",
            "İş #2: Güncel sonuç kabul edildi",
            "Yeni işi tamamla",
            "İş nesli + BağlamSürümü",
        ),
        "ORT-008" => (
            7,
            "Yerel sayı biçimi ve ayrıştırma eşdeğerliği",
            "1,234.50 · en",
            "1.234,50 · tr",
            "Yerel biçimi değiştir",
            "YerelSayıMotoru",
        ),
        "ORT-009" => (
            8,
            "Rol, erişilebilir ad, durum ve canlı duyuru",
            "Düğüm: button · Ad: Kaydet",
            "Duyuru: Değişiklikler kaydedildi",
            "Erişilebilir eylemi çalıştır",
            "Erişilebilirlik ağacı",
        ),
        "ORT-010" => (
            9,
            "Sürükleme yükü, hedef kabulü ve bırakma sonucu",
            "Kart A · Kaynak listede",
            "Kart A · Hedef listeye bırakıldı",
            "Kartı hedefe bırak",
            "SürüklemeOturumu",
        ),
        "ORT-011" => (
            10,
            "Eksen, minimum/maksimum ve sürükleme deltası",
            "Panel genişliği: 240 px",
            "Panel genişliği: 320 px",
            "Paneli genişlet",
            "Boyutlandırma oturumu",
        ),
        "ORT-012" => (
            11,
            "Pencerelenmiş koleksiyon ve güvenli overscan",
            "Görünür satırlar: 1–20 / 10.000",
            "Görünür satırlar: 21–40 / 10.000",
            "Sonraki pencereye geç",
            "GörünürAralıkÇözümleyicisi",
        ),
        "ORT-013" => (
            12,
            "Tek işlem kaydıyla geri alma ve yineleme",
            "Başlık: Taslak · Sürüm 2",
            "Geri alındı: Taslak · Sürüm 1",
            "Geri al / yinele",
            "Geri alma günlüğü",
        ),
        "ORT-014" => (
            13,
            "Kirli durum, debounce ve kurtarma kaydı",
            "Belge: Kaydedildi",
            "Belge: Otomatik kaydedildi",
            "Değiştir ve kaydet",
            "Kurtarma snapshot'ı",
        ),
        "ORT-015" => (
            14,
            "Düz metin/zengin yük ve güvenli yapıştırma",
            "Pano teklifi: Düz metin",
            "Pano teklifi: Zengin + düz fallback",
            "Zengin biçimi seç",
            "YapıştırmaMüzakerecisi",
        ),
        "ORT-016" => (
            15,
            "Tema varyantı, yön ve erişilebilir simge adı",
            "Simge varyantı: Çizgi",
            "Simge varyantı: Dolu",
            "Simge varyantını değiştir",
            "SimgeKataloğu",
        ),
        "ORT-017" => (
            16,
            "Anatomi, yuva ve davranıştan bağımsız görünüm profili",
            "Profil: Rahat · Boşluk 12",
            "Profil: Kompakt · Boşluk 8",
            "Profili değiştir",
            "GörünümÇözümleyicisi",
        ),
        "ORT-018" => (
            17,
            "Platform profili, yüzdebirlik ve ölçüm bütçesi",
            "Çizim p95: 14 ms · Bütçe 16 ms",
            "Çizim p95: 8 ms · Bütçe 16 ms",
            "Optimize edilmiş örneği göster",
            "PerformansBütçesi",
        ),
        "ORT-019" => (
            18,
            "Hassas içeriği tanı, pano ve erişilebilir addan ayırma",
            "Hassas değer: ••••••••",
            "Güvenli tanı: secret.redacted",
            "Güvenli tanıyı göster",
            "Redaksiyon politikası",
        ),
        "ORT-020" => (
            19,
            "Katman önceliği, kapsam ve sürümlü ayar deposu",
            "Kapsam: Kullanıcı · Açık",
            "Kapsam: Çalışma alanı · Koyu",
            "Ayar kapsamını değiştir",
            "AyarÇözümleyicisi",
        ),
        "ORT-021" => (
            20,
            "Locale fallback, çoğul ve yaşayan ileti çözümü",
            "TR: 1 bileşen seçildi",
            "EN: 2 components selected",
            "Locale ve çoğulu değiştir",
            "İletiÇözümleyicisi",
        ),
        "ORT-022" => (
            21,
            "Yaşayan komut, etkinlik ve tek yürütme niyeti",
            "Komut: Dosya Aç · Etkin",
            "Komut yürütüldü: Dosya Aç",
            "Komutu çalıştır",
            "KomutKataloğu",
        ),
        "ORT-023" => (
            22,
            "Tipli rota, geri/ileri geçmişi ve ayrılma koruması",
            "Rota: /bileşenler · Geçmiş 1",
            "Rota: /ayarlar · Geçmiş 2",
            "Ayarlar rotasına git",
            "GezinmeMotoru",
        ),
        _ => unreachable!("yalnız kayıtlı ORT laboratuvarı çağrılır"),
    };
    let bit = 1_u32 << sıra;
    let etkin = durumlar & bit != 0;
    let koyu = sözleşme == "ORT-004" && etkin;
    sergi_kartı(
        "ort-canlı-laboratuvar",
        aile_görünen_adı(sözleşme),
        açıklama,
    )
    .child(
        div()
            .id(format!("{}-örnek", sözleşme.to_lowercase()))
            .mt_4()
            .min_h(px(82.))
            .flex()
            .items_center()
            .justify_center()
            .rounded(if sözleşme == "ORT-003" && etkin {
                px(24.)
            } else {
                px(6.)
            })
            .border_1()
            .border_color(rgb(if etkin { kabuk_vurgusu() } else { kenarlık() }))
            .bg(rgb(if koyu { 0x111827 } else { 0xf8fafc }))
            .px_3()
            .text_sm()
            .text_color(rgb(if koyu { 0xffffff } else { 0x111827 }))
            .child(if etkin { sonra } else { önce }),
    )
    .child(
        div()
            .id(format!("{}-eylem", sözleşme.to_lowercase()))
            .mt_3()
            .cursor_pointer()
            .rounded_md()
            .border_1()
            .border_color(rgb(kabuk_vurgusu()))
            .px_3()
            .py_2()
            .text_sm()
            .text_color(rgb(kabuk_vurgusu()))
            .child(eylem)
            .on_click(bağlam.listener(move |bu, _, _, bağlam| {
                bu.sergi_ort_durumları ^= bit;
                bağlam.notify();
            })),
    )
    .child(
        div()
            .mt_3()
            .text_xs()
            .text_color(rgb(ikincil_metin()))
            .child(format!("Kanonik model: {kanonik} · Durum: {etkin}")),
    )
}

fn kabuk_simülasyonu(
    sözleşme: &str,
    durumlar: u16,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Stateful<Div> {
    let (sıra, açıklama, önce, sonra, eylem, kanonik) = match sözleşme {
        "KAB-010" => (
            0,
            "Dock bölgeleri, panel tutturma ve görünürlük",
            "Sol dock: Daraltılmış",
            "Sol dock: Dosya Gezgini açık",
            "Dock panelini aç/kapat",
            "DockKonağı",
        ),
        "KAB-020" => (
            1,
            "Alt çalışma alanı sekmesi, yükseklik ve odak dönüşü",
            "Alt alan: Kapalı",
            "Alt alan: Terminal · 240 px",
            "Terminal alanını aç/kapat",
            "AltÇalışmaAlanı",
        ),
        "KAB-030" => (
            2,
            "Başlık, pencere denetimleri ve güvenli sürükleme bölgesi",
            "Başlık: GPUI Galeri · Olağan",
            "Başlık: GPUI Galeri · Tam ekran",
            "Pencere kipini değiştir",
            "PencereKromu",
        ),
        "KAB-040" => (
            3,
            "Durum öğeleri, öncelik ve taşma çözümü",
            "main · Çevrimdışı · 0 hata",
            "main · Eşitlendi · 0 hata",
            "Eşitleme durumunu değiştir",
            "DurumÇubuğu",
        ),
        "KAB-050" => (
            4,
            "Pencere açma, görünürlük ve kontrollü kapanış",
            "Pencere: Etkin",
            "Kapanış isteği: Kaydetme bekleniyor",
            "Kapanış isteği gönder",
            "PencereYaşamDöngüsü",
        ),
        "KAB-060" => (
            5,
            "Bölme oranı, minimum boyut ve klavye yeniden boyutlandırma",
            "Bölünmüş görünüm: %50 / %50",
            "Bölünmüş görünüm: %30 / %70",
            "Bölme oranını değiştir",
            "BölünmüşGörünüm",
        ),
        "KAB-070" => (
            6,
            "Platform menüsü, komut etkinliği ve kısayol sunumu",
            "Uygulama menüsü: Kapalı",
            "Dosya · Düzen · Görünüm",
            "Uygulama menüsünü aç/kapat",
            "UygulamaMenüsü",
        ),
        "KAB-080" => (
            7,
            "Çoklu pencere kaydı ve ekran sınırına uyarlanmış yerleşim",
            "Pencere sayısı: 1 · Ana",
            "Pencere sayısı: 2 · Ana + Önizleme",
            "Önizleme penceresi ekle/kapat",
            "PencereYerleşimKaydı",
        ),
        "KAB-090" => (
            8,
            "Oturum geri yükleme, bozuk kayıt yalıtımı ve güvenli mod",
            "Oturum: Olağan başlatıldı",
            "Güvenli mod: Eklentiler devre dışı",
            "Güvenli modu değiştir",
            "OturumKurtarmaPlanı",
        ),
        "KAB-100" => (
            9,
            "Veri konumu, kasa capability ve gizli değer politikası",
            "WASM capability: Kalıcı kasa yok",
            "Politika: Her oturumda yeniden iste",
            "Fallback politikasını göster",
            "GizliSaklamaYetenekleri",
        ),
        _ => unreachable!("yalnız kayıtlı KAB simülasyonu çağrılır"),
    };
    let bit = 1_u16 << sıra;
    let etkin = durumlar & bit != 0;
    sergi_kartı("kab-canlı-simülasyon", aile_görünen_adı(sözleşme), açıklama)
        .child(
            div()
                .id(format!("{}-örnek", sözleşme.to_lowercase()))
                .mt_4()
                .overflow_hidden()
                .rounded_lg()
                .border_1()
                .border_color(rgb(if etkin { kabuk_vurgusu() } else { kenarlık() }))
                .bg(rgb(crate::palet().kabuk_kart))
                .child(
                    div()
                        .h(px(28.))
                        .flex()
                        .items_center()
                        .bg(rgb(crate::palet().kabuk_ana_metin))
                        .px_3()
                        .text_xs()
                        .text_color(rgb(crate::palet().kabuk_kart))
                        .child("GPUI Kabuk"),
                )
                .child(
                    div()
                        .h(px(82.))
                        .flex()
                        .items_center()
                        .justify_center()
                        .bg(rgb(if sözleşme == "KAB-090" && etkin {
                            0xfffbeb
                        } else {
                            0xf8fafc
                        }))
                        .px_3()
                        .text_sm()
                        .child(if etkin { sonra } else { önce }),
                ),
        )
        .child(
            div()
                .id(format!("{}-eylem", sözleşme.to_lowercase()))
                .mt_3()
                .cursor_pointer()
                .rounded_md()
                .border_1()
                .border_color(rgb(kabuk_vurgusu()))
                .px_3()
                .py_2()
                .text_sm()
                .text_color(rgb(kabuk_vurgusu()))
                .child(eylem)
                .on_click(bağlam.listener(move |bu, _, _, bağlam| {
                    bu.sergi_kab_durumları ^= bit;
                    bağlam.notify();
                })),
        )
        .child(
            div()
                .mt_3()
                .text_xs()
                .text_color(rgb(ikincil_metin()))
                .child(format!(
                    "Kanonik model: {kanonik} · Hedef: WASM simülasyonu · Durum: {etkin}"
                )),
        )
}

fn düğme_sergisi(
    düğme_sayacı: u32, bağlam: &mut Context<GaleriUygulaması>
) -> Stateful<Div> {
    let birincil = düğme_renkleri(DüğmeVurgusu::Birincil, ErişimDurumu::Etkin);
    let ikincil = düğme_renkleri(DüğmeVurgusu::İkincil, ErişimDurumu::Etkin);
    let devre_dışı = düğme_renkleri(DüğmeVurgusu::Birincil, ErişimDurumu::DevreDışı);
    sergi_kartı(
        "bil-040-canlı-sergi",
        "Düğme",
        "Birincil, ikincil ve devre dışı durumlar",
    )
    .child(
        div()
            .mt_5()
            .flex()
            .flex_wrap()
            .gap_2()
            .child(
                div()
                    .id("bil-040-birincil-düğme")
                    .cursor_pointer()
                    .rounded_md()
                    .border_1()
                    .border_color(rgb(birincil.2))
                    .bg(rgb(birincil.0))
                    .px_4()
                    .py_2()
                    .text_sm()
                    .text_color(rgb(birincil.1))
                    .child("Kaydet")
                    .on_click(bağlam.listener(|bu, _, _, bağlam| {
                        bu.sergi_düğme_sayacı = bu.sergi_düğme_sayacı.saturating_add(1);
                        bağlam.notify();
                    })),
            )
            .child(
                div()
                    .id("bil-040-ikincil-düğme")
                    .cursor_pointer()
                    .rounded_md()
                    .border_1()
                    .border_color(rgb(ikincil.2))
                    .bg(rgb(ikincil.0))
                    .px_4()
                    .py_2()
                    .text_sm()
                    .text_color(rgb(ikincil.1))
                    .child("Vazgeç"),
            )
            .child(
                div()
                    .id("bil-040-devre-dışı-düğme")
                    .rounded_md()
                    .border_1()
                    .border_color(rgb(devre_dışı.2))
                    .bg(rgb(devre_dışı.0))
                    .px_4()
                    .py_2()
                    .text_sm()
                    .text_color(rgb(devre_dışı.1))
                    .child("Devre dışı"),
            ),
    )
    .child(
        div()
            .mt_4()
            .text_xs()
            .text_color(rgb(ikincil_metin()))
            .child(format!(
                "Vurgu ve erişim kanonik · Etkinleştirme: {düğme_sayacı}"
            )),
    )
}

fn düğme_renkleri(vurgu: DüğmeVurgusu, erişim: ErişimDurumu) -> (u32, u32, u32) {
    if erişim == ErişimDurumu::DevreDışı {
        return (0xe5e7eb, 0x9ca3af, 0xe5e7eb);
    }
    match vurgu {
        DüğmeVurgusu::Birincil => (kabuk_vurgusu(), 0xffffff, kabuk_vurgusu()),
        DüğmeVurgusu::İkincil => (0xffffff, 0x111827, kenarlık()),
        DüğmeVurgusu::Hayalet | DüğmeVurgusu::Çerçevesiz => {
            (0xffffff, kabuk_vurgusu(), 0xffffff)
        }
    }
}

/// Tezgâhın platformdan okuduğu gerçekler.
///
/// Tercih değil bildirimdir: hangi yazı aileleri kurulu, hangi saat dilimi
/// çözüldü, otomatik doldurma sunuluyor mu. Üçü de aynı yerden geldiği için
/// tek bağlamda taşınır.
pub(crate) struct TezgahPlatformu {
    pub sistem_aileleri: std::rc::Rc<Vec<String>>,
    pub saat_dilimi: crate::ÇözülmüşSaatDilimi,
    pub doldurma_var: bool,
    /// `B` bölümünün port kapıları.
    pub portlar: crate::PortDurumu,
    /// `§29` kanonik doğrulama raporu; kart onu okur, yeniden kurmaz.
    pub rapor: gpui_bilesenleri::GirişYapılandırmaRaporu,
    /// `§26` alanın yayımladığı son olaylar; en yeni **başta**.
    pub olaylar: Vec<crate::TezgahOlayı>,
}

/// Tezgâh kabuğunun okuduğu uygulama durumu.
///
/// Üçü de tercih değil **kabuk** durumudur: tema ailesi ve kip galeri
/// uygulamasının, hedef ise derleme hedefinin bilgisidir. `TezgahTercihleri`
/// içine kopyalansalardı `GirişYapılandırması`'na sızma riski doğardı — `D`
/// bölümü koda yazılmaz.
#[derive(Clone)]
pub(crate) struct TezgahKabukDurumu {
    pub tema: crate::GaleriTeması,
    pub kip: gpui_bilesenleri::TemaKipi,
    pub hedef: crate::GaleriHedefi,
}

/// Tezgâhın kendi ekranı: üst şerit + iki kolonlu gövde.
///
/// Tasarımın `§5`/`§6` kabuğu budur. Galerinin ağaç menüsü, kategori
/// sayfaları ve aile kartları burada **yoktur**: tezgâh onların içine
/// gömülü bir sayfa değil, kendi ekranıdır.
///
/// Bileşen seçici bugün tek öğe taşır — sözleşmesi biten tek kamusal
/// bileşen `BİL-010`. Seçici olarak durmasının nedeni, listenin
/// büyüyeceği gün yerleşimin değişmemesi.
pub(crate) fn tezgah_ekranı(
    tercih: crate::TezgahTercihleri,
    alan: Entity<GirişKutusu>,
    alanlar: &crate::MetinGirişiAlanları,
    durum_izi: std::rc::Rc<std::cell::Cell<gpui::Bounds<gpui::Pixels>>>,
    platform: TezgahPlatformu,
    kabuk: TezgahKabukDurumu,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Stateful<Div> {
    let g = crate::görünüm();
    let t = crate::TezgahTokenları::paletten(crate::palet());

    div()
        .id("tezgah-ekranı")
        .size_full()
        .relative()
        .flex()
        .flex_col()
        // `items_center` değil: kap asgari genişlikten darken ortalamak
        // içeriğin **sol** kenarını kaydırma erişiminin dışına itiyor —
        // kullanıcı sağa kaydırabiliyor ama başa dönemiyor.
        .items_start()
        .min_h(px(0.))
        .bg(t.kağıt)
        .px(g.önizleme_kabuğu.parça_aralığı)
        .pt(g.önizleme_kabuğu.parça_aralığı)
        .child(
            // Yatay kaydırma **ara katmanda**. Kökte `overflow_x_scroll`
            // olduğunda kök bir kaydırma kabına dönüşüyor ve içindeki
            // `flex_1` zinciri "kalan yükseklik" yerine "içerik yüksekliği"
            // hesaplıyordu — pencerenin alt dörtte biri boş kalıyordu.
            div()
                .id("tezgah-kaydırma")
                .flex_1()
                .min_h(px(0.))
                .w_full()
                .overflow_x_scroll()
                // GPUI, yalnız yatay kaydırması olan bir kapta dikey
                // tekerlek deltasını yataya çevirir (`div.rs:3218`). Bu
                // çevirim burada **istenir**: dar pencerede boş alanda
                // tekerlek yatay gezinmeyi sağlar. İstenmeyen şey, dikey
                // kaydırması olan bir kolonun üzerindeyken aynı olayın iki
                // eksende birden iş görmesi — o kolonlar olayı kendileri
                // tüketiyor (`önizleme_kolonu`, `yapılandırma_kolonu`).
                .child(
                    // Orta sarmalayıcı: pencere ne kadar genişse genişlesin, içerik
                    // tasarımın `1480px`ini aşmaz. Her şeyin genişliği doldurması
                    // satırları okunamaz uzunluğa çıkarırdı.
                    div()
                        .h_full()
                        // Tasarımda sarmalayıcı `margin: 0 auto` ile ortalanır.
                        // `justify_center` kullanmıyoruz: pencere asgari
                        // genişlikten darken ortalamak içeriğin sol kenarını
                        // kaydırma erişiminin dışına iter; `auto` kenar
                        // boşluğu ise negatife düşmez.
                        .mx_auto()
                        // Tasarımın orta sarmalayıcısı: `min-width: 1216px`,
                        // `max-width: 1480px`. Alt sınır olmadan simge şeritleri ve
                        // düğme ızgaraları dar pencerede birbirine giriyordu —
                        // taslağın hizası bu iki sayıya dayanıyor.
                        .min_w(px(ölçü::İÇERİK_ASGARİSİ))
                        .max_w(px(ölçü::İÇERİK_GENİŞLİĞİ))
                        .flex()
                        .flex_col()
                        .min_h(px(0.))
                        .child(tezgah_üst_şeridi(
                            &tercih,
                            kabuk,
                            &platform.sistem_aileleri.clone(),
                            &g,
                            &t,
                            bağlam,
                        ))
                        .child(div().flex_1().min_h(px(0.)).mt(px(ölçü::BLOK_ARASI)).child(
                            tezgah_sergisi(tercih, alan, alanlar, durum_izi, platform, bağlam),
                        )),
                ),
        )
}

/// `§6` üst şerit: solda kimlik, sağda tema otoritesi ve çizim hedefi.
///
/// Tasarımın iki satırlık sağ bloğu: üstte tema ailesi · kip · hedef ·
/// metin ölçeği, altta yazı ailesi · yoğunluk · hareket.
fn tezgah_üst_şeridi(
    tercih: &crate::TezgahTercihleri,
    kabuk: TezgahKabukDurumu,
    sistem_aileleri: &[String],
    g: &crate::ÇözülmüşTezgahGörünümü,
    t: &crate::TezgahTokenları,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Div {
    div()
        .flex()
        .flex_row()
        .flex_wrap()
        .items_start()
        .justify_between()
        .gap(px(ölçü::BLOK_ARASI))
        .pb(px(ölçü::BLOK_ARASI))
        .border_b_1()
        .border_color(t.kenarlık)
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(ölçü::ARALIK))
                .child(crate::kabuk_başlığı(g, t, "Yapılandırma Tezgâhı"))
                .child(bileşen_seçicisi(bağlam)),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .items_end()
                .gap(px(ölçü::ARALIK))
                .child(tema_otoritesi(tercih, kabuk, bağlam))
                .child(görünüm_ekseni(tercih, sistem_aileleri, bağlam)),
        )
}

/// Bileşen seçicisi.
///
/// Bugün tek öğe taşıyor — sözleşmesi biten tek kamusal bileşen `BİL-010`.
/// Yine de **gerçek bir açılır seçici**: pasif bir etiket olsaydı listenin
/// büyüyeceği gün hem yüz hem rol değişirdi. Açıldığında tek satır görünür
/// ve o satır seçilidir; seçilemeyen bir liste değil, tek üyeli bir liste.
fn bileşen_seçicisi(bağlam: &mut Context<GaleriUygulaması>) -> Stateful<Div> {
    let liste = div()
        .child(liste_öğesi("bileşen-metin-kutusu", "Metin kutusu", true).child("Metin kutusu"));
    div()
        .id("bileşen-seçici-kabı")
        .cursor_pointer()
        .on_click(bağlam.listener(|bu, _, _, bağlam| {
            bu.seçiciyi_değiştir("bileşen", bağlam);
        }))
        .child(eksen_seçimi_açık(
            "Metin kutusu",
            true,
            crate::seçici_açık_mı("bileşen"),
            liste,
        ))
        .min_w(px(ölçü::SEÇİCİ_ASGARİSİ))
}

/// Üst şeridin açılır seçicisi.
///
/// Tetikleyiciye basmak listeyi açar; açıkken yeniden basmak kapatır.
/// Seçili değer kapalıyken de görünür — `<select>` böyle davranır.
fn şerit_seçicisi(
    kimlik: &'static str,
    etiket: &str,
    içerik: impl IntoElement + 'static,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Stateful<Div> {
    // Tetikleyici ile tıklama **aynı** öğede. Dış bir sarmalayıcıya
    // `on_click` koymak işe yaramıyordu: içteki `.id()`li tetikleyici
    // tıklamayı yakalayıp dışarı bırakmıyor.
    let g = crate::görünüm();
    let t = crate::TezgahTokenları::paletten(crate::palet());
    let açık = crate::seçici_açık_mı(kimlik);

    crate::stili_uygula(div(), &g.eksen_etiketi)
        .id(SharedString::new_static(kimlik))
        .role(gpui::Role::ComboBox)
        .aria_label(SharedString::new(etiket))
        .aria_expanded(açık)
        .relative()
        .flex()
        .items_center()
        .justify_between()
        .gap_1()
        .h(g.anahtar_yüksekliği)
        .px(g.hap.yatay_dolgu)
        .rounded(g.segment.yarıçap)
        .border_1()
        .border_color(if açık { t.vurgu } else { t.kenarlık })
        .bg(t.yüzey)
        .text_color(t.ana_metin)
        .cursor_pointer()
        .on_click(bağlam.listener(move |bu, _, _, bağlam| {
            bu.seçiciyi_değiştir(kimlik, bağlam);
        }))
        .child(etiket.to_owned())
        .child(tezgah_simgesi("acilir.svg").size(px(10.)))
        .when(açık, |tetikleyici| {
            // Liste **üste biner**, akışa girmez: akışa giren liste
            // açıldığında altındaki her şeyi aşağı itiyordu ve kullanıcı
            // seçim yaparken baktığı yer kayıyordu. `<select>` böyle
            // davranmaz.
            //
            // Bu bir `ORT-006` yüzer yüzeyi değildir ve o konağa bağlanmaz;
            // `BİL-020` seçim listesi sözleşmesi geldiğinde oraya taşınır.
            // Liste tetikleyiciye ikinci tıklamayla, ya da başka bir
            // seçici açılınca kapanır. Dış tıklama kapanışı **burada
            // kurulmaz** ve yerel bir kapatıcı katman eklenmemelidir:
            // pencereyi kaplayan `occlude()` bir katman GPUI hit testinde
            // kardeş sırasından ve `deferred` önceliğinden bağımsız olarak
            // listenin kendi tıklamalarını da yutuyor. Kapanış `ORT-006`
            // konağı fiziksel olduğunda oradan gelir — dış tıklama,
            // `Escape` ve odak kaybı yüzeyin kendi sözleşmesindedir.
            // Göç planı §8 borç 17.
            tetikleyici.child(gpui::deferred(
                div()
                    .id("açılır-liste")
                    .occlude()
                    .absolute()
                    .top(g.anahtar_yüksekliği)
                    .left(px(0.))
                    .min_w(px(ölçü::LİSTE_ASGARİSİ))
                    .max_h(px(ölçü::AÇILIR_YÜKSEKLİĞİ))
                    .overflow_y_scroll()
                    .p(px(ölçü::ARALIK))
                    .rounded(g.kart.yarıçap)
                    .border_1()
                    .border_color(t.kenarlık)
                    .bg(t.kağıt)
                    .child(içerik),
            ))
        })
}

/// Üst şeridin birinci satırı: tema ailesi · kip · hedef · metin ölçeği.
fn tema_otoritesi(
    tercih: &crate::TezgahTercihleri,
    kabuk: TezgahKabukDurumu,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Div {
    use gpui_bilesenleri::TemaKipi as K;

    let aile_listesi = crate::GaleriTeması::TÜMÜ
        .iter()
        .map(|tema| {
            let tema = *tema;
            liste_öğesi(
                format!("tema-{}", tema.adı()),
                tema.adı(),
                kabuk.tema == tema,
            )
            .child(tema.adı())
            .on_click(bağlam.listener(move |bu, _, _, bağlam| {
                bu.galeri_temasını_seç(tema, bağlam);
            }))
        })
        .collect::<Vec<_>>();

    // Kip simgeyle seçilir: dört kipin adı ("YüksekKarşıtlıkAçık") şeride
    // sığmaz ve kısaltmak ("YK") ne olduğunu söylemez. Ad `aria_label`da
    // tam durur — simge tek bilgi kanalı değildir.
    let kip_simgeleri = [
        ("kip-acik.svg", "Açık", K::Açık),
        ("kip-koyu.svg", "Koyu", K::Koyu),
        (
            "kip-yk-acik.svg",
            "Yüksek karşıtlık · açık",
            K::YüksekKarşıtlıkAçık,
        ),
        (
            "kip-yk-koyu.svg",
            "Yüksek karşıtlık · koyu",
            K::YüksekKarşıtlıkKoyu,
        ),
    ]
    .into_iter()
    .fold(
        segment_şeridi("tema-kipi", "Tema kipi"),
        |kuşak, (simge, ad, kip)| {
            kuşak.child(
                segment_simgesi(
                    format!("kip-{ad}"),
                    ad,
                    kabuk.kip == kip,
                    bağlam,
                    move |_| {},
                )
                .child(tezgah_simgesi(simge))
                .on_click(bağlam.listener(move |bu, _, _, bağlam| {
                    bu.galeri_kipini_seç(kip, bağlam);
                })),
            )
        },
    )
    // Sistem kipi bir kip değil, kipi **kimin seçtiğinin** anahtarıdır:
    // açıkken kalıcı seçimler silinmez, sistemin güncel kipine göre biri
    // etkinleşir. Bu yüzden kuşağın içinde ama ayrı bir anahtar.
    .child(
        simge_düğmesi(
            "kip-sistem",
            "Sistem kipini izle",
            tercih.tema.sistem_kipini_izle,
            bağlam,
            |k| k.tema.sistem_kipini_izle = !k.tema.sistem_kipini_izle,
        )
        .child(tezgah_simgesi("kip-sistem.svg")),
    );

    let ölçek_listesi = [1.0_f32, 1.25, 1.5, 2.0]
        .into_iter()
        .map(|ölçek| {
            let ad = SharedString::new(format!("{ölçek:.2}×").replace('.', ","));
            let seçili = (tercih.tema.metin_ölçeği - ölçek).abs() < f32::EPSILON;
            liste_öğesi(format!("ölçek-{ölçek}"), ad.clone(), seçili)
                .child(ad)
                .on_click(bağlam.listener(move |bu, _, _, bağlam| {
                    bu.tezgahı_değiştir(move |k| k.tema.metin_ölçeği = ölçek, bağlam);
                }))
        })
        .collect::<Vec<_>>();

    şerit_satırı()
        .justify_end()
        .gap(px(ölçü::ARALIK))
        .child(
            şerit_seçicisi(
                "tema-ailesi",
                kabuk.tema.adı(),
                div().children(aile_listesi),
                bağlam,
            )
            .min_w(px(ölçü::SEÇİCİ_ASGARİSİ)),
        )
        .child(kip_simgeleri)
        .child(hedef_kuşağı(kabuk, bağlam))
        .child(şerit_seçicisi(
            "metin-ölçeği",
            &format!("{:.2}×", tercih.tema.metin_ölçeği).replace('.', ","),
            div().children(ölçek_listesi),
            bağlam,
        ))
}

/// Çizim hedefi · `WASM` ile `Masaüstü` aynı kataloğu açar.
///
/// Hedef bir tercih değil **derleme gerçeğidir**: WASM ikilisi masaüstüne
/// geçemez. Bu yüzden kuşak seçili olanı gösterir, ötekini pasif bırakır —
/// tıklanabilir yapmak çalışmayan bir düğme olurdu.
fn hedef_kuşağı(
    kabuk: TezgahKabukDurumu,
    _bağlam: &mut Context<GaleriUygulaması>,
) -> Stateful<Div> {
    let g = crate::görünüm();
    let t = crate::TezgahTokenları::paletten(crate::palet());
    [
        ("WASM", crate::GaleriHedefi::Wasm),
        ("Masaüstü", crate::GaleriHedefi::Masaüstü),
    ]
    .into_iter()
    .fold(
        segment_şeridi("çizim-hedefi", "Çizim hedefi"),
        |kuşak, (ad, hedef)| {
            let seçili = kabuk.hedef == hedef;
            kuşak.child(if seçili {
                // Taslakta hedef kuşağı `4px` çerçeveli, etiketleri köşeli:
                // hap değil.
                crate::durum_hapı(format!("hedef-{ad}"), &g, &t, true)
                    .rounded(g.kart.yarıçap)
                    .child(ad)
                    .into_any_element()
            } else {
                pasif_simge_düğmesi(
                    if ad == "WASM" {
                        "hedef-wasm"
                    } else {
                        "hedef-masaüstü"
                    },
                    "çizim hedefi derleme gerçeğidir; ikili çalışırken değişmez",
                )
                .w_auto()
                .px(g.hap.yatay_dolgu)
                .child(ad)
                .into_any_element()
            })
        },
    )
}

/// Üst şeridin ikinci satırı: yazı ailesi · yoğunluk · hareket.
fn görünüm_ekseni(
    tercih: &crate::TezgahTercihleri,
    sistem_aileleri: &[String],
    bağlam: &mut Context<GaleriUygulaması>,
) -> Div {
    use gpui_bilesenleri::{ArayüzYoğunluğu as Y, HareketTercihi as H};

    let yoğunluk_listesi = [
        ("Kompakt", Y::Kompakt),
        ("Normal", Y::Normal),
        ("Geniş", Y::Geniş),
    ]
    .into_iter()
    .map(|(ad, değer)| {
        liste_öğesi(format!("yoğunluk-{ad}"), ad, tercih.tema.yoğunluk == değer)
            .child(ad)
            .on_click(bağlam.listener(move |bu, _, _, bağlam| {
                bu.tezgahı_değiştir(move |k| k.tema.yoğunluk = değer, bağlam);
            }))
    })
    .collect::<Vec<_>>();

    let hareket_listesi = [
        ("Hareket · Tam", H::Tam),
        ("Hareket · Azaltılmış", H::Azaltılmış),
        ("Hareket · Kapalı", H::Kapalı),
    ]
    .into_iter()
    .map(|(ad, değer)| {
        liste_öğesi(format!("hareket-{ad}"), ad, tercih.tema.hareket == değer)
            .child(ad)
            .on_click(bağlam.listener(move |bu, _, _, bağlam| {
                bu.tezgahı_değiştir(move |k| k.tema.hareket = değer, bağlam);
            }))
    })
    .collect::<Vec<_>>();

    let yoğunluk_adı = match tercih.tema.yoğunluk {
        Y::Kompakt => "Kompakt",
        Y::Normal => "Normal",
        Y::Geniş => "Geniş",
    };
    let hareket_adı = match tercih.tema.hareket {
        H::Tam => "Hareket · Tam",
        H::Azaltılmış => "Hareket · Azaltılmış",
        H::Kapalı => "Hareket · Kapalı",
    };

    şerit_satırı()
        .justify_end()
        .gap(px(ölçü::ARALIK))
        .child(
            şerit_seçicisi(
                "yazı-ailesi",
                &tercih.tema.yazı_ailesi.clone(),
                aile_listesi(tercih, sistem_aileleri, bağlam),
                bağlam,
            )
            .min_w(px(ölçü::SEÇİCİ_ASGARİSİ)),
        )
        // `ACC-034`: aile yalnız seçili hedefin kataloğundaysa uygulanır.
        // Katalog dışı aile `MerkeziFallback` rolüne düşmeli — o rol kanonik
        // kodda henüz yok, bu yüzden rozet rolü değil **durumu** yazar.
        // Sessizce çizmek, kuralın yasakladığı öteki hata olurdu.
        .when(
            !crate::KİTAPLIK_AİLELERİ.contains(&tercih.tema.yazı_ailesi.as_str()),
            |şerit| şerit.child(türetilmiş_rozet("katalog dışı")),
        )
        .child(şerit_seçicisi(
            "punto",
            &format!("{:.0} px", tercih.tema.punto),
            punto_listesi(tercih, bağlam),
            bağlam,
        ))
        .child(şerit_seçicisi(
            "yoğunluk",
            yoğunluk_adı,
            div().children(yoğunluk_listesi),
            bağlam,
        ))
        // `ORT-004` metin düzenleme iç boşluğu. Tema alanı `None`
        // bırakılıyordu: kütüphane varsayılanı dışına çıkılamıyor, kutunun
        // iç boşluğu hiç denenemiyordu.
        .child(şerit_seçicisi(
            "iç-boşluk",
            &format!("İç boşluk · {}", tercih.tema.iç_boşluk.adı()),
            div().children(crate::TezgahİçBoşluğu::TÜMÜ.map(|değer| {
                liste_öğesi(
                    format!("iç-boşluk-{}", değer.adı()),
                    değer.adı(),
                    tercih.tema.iç_boşluk == değer,
                )
                .child(değer.adı())
                .on_click(bağlam.listener(move |bu, _, _, bağlam| {
                    bu.tezgahı_değiştir(move |k| k.tema.iç_boşluk = değer, bağlam);
                }))
            })),
            bağlam,
        ))
        .child(şerit_seçicisi(
            "hareket",
            hareket_adı,
            div().children(hareket_listesi),
            bağlam,
        ))
}

/// `BİL-010` yapılandırma tezgâhı: ailenin tek ekranı.
///
/// Bu aile hazır varyant vitrini göstermez; programcı alanı burada kurar ve
/// karşılığı olan kodu altında görür. Yerleşim tek bir dar sütundur: üstte
/// görünümü ve yardımcı eylemleri değiştiren simge grupları, ortada yaşayan
/// önizleme kutusu, altında değer ve biçim tercihleri.
///
/// `BİL-010` tezgâhı.
///
/// Sergi artık düzeni kendisi kurmaz: profil `Tezgahİçeriği` üretir, tezgâh
/// kabuğu onu iki kolonlu düzende çizer. Kabuk hiçbir `BİL-*` tipini
/// tanımadığı için yeni bir aile yeniden yazıldığında yalnız kendi profilini
/// verir (harita §7 · adım 5).
fn tezgah_sergisi(
    tercih: crate::TezgahTercihleri,
    alan: Entity<GirişKutusu>,
    alanlar: &crate::MetinGirişiAlanları,
    durum_izi: std::rc::Rc<std::cell::Cell<gpui::Bounds<gpui::Pixels>>>,
    platform: TezgahPlatformu,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Div {
    let TezgahPlatformu {
        sistem_aileleri: _,
        saat_dilimi,
        doldurma_var,
        portlar,
        rapor,
        olaylar,
    } = platform;
    // `ORT-003 §2` yarıçap kısa kenarın yarısını aşamaz; tek satırlı alanda
    // kısıtlayan kenar kutu yüksekliğidir.
    let en_fazla_yarıçap =
        f32::from(crate::tezgah_teması(&tercih.tema).ölçüler.etkileşim_hedefi) / 2.;
    let metin_ölçeği = tercih.tema.metin_ölçeği;

    let içerik = crate::tezgah_içeriği(
        crate::MetinGirişiProfilGirdisi {
            tercih: &tercih,
            alanlar,
            alan,
            saat_dilimi: &saat_dilimi,
            doldurma_var,
            portlar,
            rapor: &rapor,
            olaylar: &olaylar,
            en_fazla_yarıçap,
            köşe_izi: durum_izi,
        },
        bağlam,
    );

    div().size_full().flex().flex_col().min_h(px(0.)).child(
        // Gövde kalan **bütün** yüksekliği alır. Sabit bir yükseklik
        // (720px) pencerenin altında boşluk bırakıyordu ve iki kolonun
        // kendi kaydırması pencere yerine o kutuyla sınırlı kalıyordu.
        div().flex_1().min_h(px(0.)).child(crate::tezgah_gövdesi(
            içerik,
            crate::görünüm(),
            crate::TezgahTokenları::paletten(crate::palet()),
            metin_ölçeği,
            crate::tezgah_bölüm_adı,
        )),
    )
}

// ---------------------------------------------------------------- şeritler

/// `ORT-003` köşe kademeleri ve ürünün kendi piksel ölçüsü.
pub(crate) fn köşe_şeridi(
    tercih: &crate::TezgahTercihleri,
    en_fazla_yarıçap: f32,
    iz: std::rc::Rc<std::cell::Cell<gpui::Bounds<gpui::Pixels>>>,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Div {
    use gpui_bilesenleri::DüğmeŞekli;
    let kademe = tercih.köşe_pikseli.is_none() && !tercih.şekil_oto;
    // Blok içeriği önce kurulur: `eksen_bloğu` hem içeriği hem bağlamı alır.
    let çubuk = köşe_kaydırma_çubuğu(tercih, en_fazla_yarıçap, iz, bağlam);
    şerit()
        // `oto`: şekil görünüm profilinden gelir. Taslakta ilk seçenek ve
        // etiketi yazıyla duruyor — dört geometrik simgenin yanında beşinci
        // bir simge, "profil kararı"nı bir şekil kademesi gibi gösterirdi.
        .child(
            simge_düğmesi(
                "köşe-oto",
                "oto · profilden",
                tercih.şekil_oto,
                bağlam,
                |t| {
                    t.şekil_oto = true;
                    t.köşe_pikseli = None;
                },
            )
            .child(crate::stili_uygula(div(), &crate::görünüm().rozet_metni).child("oto")),
        )
        .children(
            [
                ("Dik köşeli", DüğmeŞekli::DikKöşeli, px(0.)),
                ("Köşeli", DüğmeŞekli::Köşeli, px(2.)),
                ("Yuvarlatılmış", DüğmeŞekli::Yuvarlatılmış, px(4.)),
                ("Hap", DüğmeŞekli::Hap, px(999.)),
            ]
            .into_iter()
            .map(|(ad, şekil, yarıçap)| {
                let seçili = kademe && tercih.şekil == şekil;
                simge_düğmesi(format!("köşe-{ad}"), ad, seçili, bağlam, move |t| {
                    t.şekil = şekil;
                    // Kademe seçmek ürünün piksel ölçüsünü ve profil
                    // devrini bırakır: üçü aynı anda geçerli olamaz.
                    t.köşe_pikseli = None;
                    t.şekil_oto = false;
                })
                .child(
                    div()
                        .size(px(13.))
                        .border_1()
                        .border_color(rgb(if seçili {
                            crate::tezgah_vurgu()
                        } else {
                            crate::tezgah_ikincil_metin()
                        }))
                        .rounded(yarıçap),
                )
            }),
        )
        .child(ayırıcı())
        .child(yarıçap_bloğu(tercih, çubuk, bağlam))
}

/// `§7.1` özel yarıçap bloğu · tıklanınca açılır.
///
/// Taslakta `<details>`: kapalıyken yalnız simge görünür. Hep açık durmak
/// şeridi iki satıra çıkarıyor ve adlandırılmış kademelerle aynı görsel
/// ağırlığı veriyordu — oysa yarıçap aynı tercihin **üçüncü** varyantı,
/// bağımsız bir eksen değil.
fn yarıçap_bloğu(
    tercih: &crate::TezgahTercihleri,
    çubuk: impl IntoElement + 'static,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Div {
    let açık = crate::seçici_açık_mı("özel-yarıçap");
    let g = crate::görünüm();
    let t = crate::TezgahTokenları::paletten(crate::palet());

    div()
        .relative()
        .child(
            div()
                .id("özel-yarıçap")
                .role(gpui::Role::Button)
                .aria_label("Özel yarıçap")
                .aria_expanded(açık)
                .flex()
                .items_center()
                .justify_center()
                .size(g.simge_düğmesi)
                .rounded(g.segment.yarıçap)
                .cursor_pointer()
                .when(açık || tercih.köşe_pikseli.is_some(), |d| {
                    d.bg(t.vurgu_zemin)
                })
                .on_click(bağlam.listener(|bu, _, _, bağlam| {
                    bu.seçiciyi_değiştir("özel-yarıçap", bağlam);
                }))
                .child(tezgah_simgesi("kose-yaricap.svg")),
        )
        .when(açık, |blok| {
            // Açılır liste gibi **üste biner**: akışa giren blok, açıldığında
            // altındaki her şeyi aşağı itiyor ve kullanıcı kaydırıcıya
            // uzanırken sayfa kayıyordu.
            blok.child(
                gpui::deferred(
                    div()
                        .id("yarıçap-paneli")
                        .occlude()
                        .absolute()
                        .top(g.simge_düğmesi)
                        .left(px(0.))
                        .w(px(ölçü::YARIÇAP_PANELİ))
                        .p(px(ölçü::ARALIK))
                        .rounded(g.kart.yarıçap)
                        .border_1()
                        .border_color(t.kenarlık)
                        .bg(t.kağıt)
                        .child(çubuk),
                )
                .with_priority(1),
            )
        })
}

/// Tasarımın punto listesi: 12–20 px.
///
/// Aralık `metinkutusu.cozulmus.html` 355–362. satırlardan alındı; yazı
/// biçimi grubundaki büyüt/küçült düğmeleri aynı aralıkta hareket eder.
const PUNTOLAR: [f32; 5] = [12., 14., 16., 18., 20.];

fn punto_listesi(
    tercih: &crate::TezgahTercihleri, bağlam: &mut Context<GaleriUygulaması>
) -> Div {
    let seçili = tercih.tema.punto;
    div().children(PUNTOLAR.map(|punto| {
        let seçili_mi = (punto - seçili).abs() < f32::EPSILON;
        liste_öğesi(
            format!("punto-{punto:.0}"),
            format!("{punto:.0}"),
            seçili_mi,
        )
        .child(format!("{punto:.0}"))
        .on_click(bağlam.listener(move |bu, _, _, bağlam| {
            bu.tezgahı_değiştir(move |t| t.tema.punto = punto, bağlam);
        }))
    }))
}

/// Yüzer aile listesi: kitaplık yüzleri ve yazı sisteminin bildirdikleri.
///
/// İki bölüm ayrı durur çünkü garantileri farklıdır. Kitaplık yüzleri iki
/// hedefte de kayıtlıdır ve her makinede aynı görünür. İkinci bölüm yazı
/// sisteminin o an bildirdiği ailelerdir: masaüstünde işletim sisteminde
/// kurulu olanlar, tarayıcıda ise yalnız kayıtlı yüzler — tarayıcı sistem
/// listesini vermez. Bu ayrım seçimin başka bir makinede çözülmeyebileceğini
/// programcıya söyler.
fn aile_listesi(
    tercih: &crate::TezgahTercihleri,
    sistem_aileleri: &[String],
    bağlam: &mut Context<GaleriUygulaması>,
) -> Div {
    let seçili = tercih.tema.yazı_ailesi.clone();
    let satır = |ad: &str,
                 seçili_mi: bool,
                 bağlam: &mut Context<GaleriUygulaması>|
     -> gpui::Stateful<Div> {
        let yazılacak = ad.to_owned();
        liste_öğesi(format!("aile-{ad}"), ad.to_owned(), seçili_mi)
            .child(ad.to_owned())
            .on_click(bağlam.listener(move |bu, _, _, bağlam| {
                let ad = yazılacak.clone();
                bu.tezgahı_değiştir(move |t| t.tema.yazı_ailesi = ad, bağlam);
            }))
    };

    let kitaplık: Vec<gpui::Stateful<Div>> = crate::KİTAPLIK_AİLELERİ
        .into_iter()
        .map(|ad| satır(ad, seçili == ad, bağlam))
        .collect();
    let sistem: Vec<gpui::Stateful<Div>> = sistem_aileleri
        .iter()
        .map(|ad| satır(ad, &seçili == ad, bağlam))
        .collect();
    let sistem_boş = sistem.is_empty();

    div()
        // Liste dar bir kolonda durur; asgari genişlik olmadan kap sıfıra
        // kadar daralıyor ve uzun açıklama **harf harf** alt alta sarılıyordu.
        // GPUI'de `overflow-wrap` yok: taşmayı sarmayla değil, kaba taban
        // genişliği vererek keseriz.
        .min_w(px(ölçü::LİSTE_ASGARİSİ))
        .child(kutu_başlığı("Kitaplık yazı tipleri", true))
        .children(kitaplık)
        .child(
            div()
                .mt_2()
                .child(kutu_başlığı("Yazı sisteminde bulunanlar", false)),
        )
        .child(
            div()
                .text_xs()
                .text_color(rgb(crate::tezgah_soluk()))
                .child(
                    "Masaüstünde işletim sisteminin aileleri; tarayıcıda yalnız kayıtlı yüzler.",
                ),
        )
        .when(sistem_boş, |d| {
            d.child(
                div()
                    .text_xs()
                    .text_color(rgb(crate::tezgah_soluk()))
                    .child("Bu hedef başka aile bildirmiyor."),
            )
        })
        .when(!sistem_boş, |d| {
            d.child(
                div()
                    .id("aile-sistem-listesi")
                    .max_h(px(180.))
                    .overflow_y_scroll()
                    .children(sistem),
            )
        })
}

/// `ORT-004` yazı biçimi: ağırlık, eğiklik ve çizgiler.
///
/// Düğme dizisi ve simgeleri `metinkutusu.cozulmus.html` 364–372. satırlardan
/// alındı: Koyu · İnce · Eğik · Altı çizili · Üstü çizili · A+ · A−.
pub(crate) fn yazı_biçimi_şeridi(
    tercih: &crate::TezgahTercihleri,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Div {
    use crate::YazıAğırlığı;
    let punto = tercih.tema.punto;
    let ağırlık = tercih.tema.ağırlık;
    // Ağırlık üçlüdür ama tasarımda iki düğme var: seçili olana yeniden
    // basmak düz ağırlığa döner.
    let ağırlık_düğmesi = |kimlik: &'static str,
                           başlık: &'static str,
                           simge: &'static str,
                           hedef: YazıAğırlığı,
                           bağlam: &mut Context<GaleriUygulaması>| {
        simge_düğmesi(kimlik, başlık, ağırlık == hedef, bağlam, move |t| {
            t.tema.ağırlık = if t.tema.ağırlık == hedef {
                YazıAğırlığı::Düz
            } else {
                hedef
            };
        })
        .child(tezgah_simgesi(simge))
    };

    şerit()
        .child(ağırlık_düğmesi(
            "yazı-koyu",
            "Koyu",
            "yazi-koyu.svg",
            YazıAğırlığı::Koyu,
            bağlam,
        ))
        .child(ağırlık_düğmesi(
            "yazı-ince",
            "İnce · açık ton",
            "yazi-ince.svg",
            YazıAğırlığı::İnce,
            bağlam,
        ))
        .child(
            simge_düğmesi("yazı-eğik", "Eğik", tercih.tema.eğik, bağlam, |t| {
                t.tema.eğik = !t.tema.eğik
            })
            .child(tezgah_simgesi("yazi-egik.svg")),
        )
        .child(
            simge_düğmesi(
                "yazı-altı-çizili",
                "Altı çizili",
                tercih.tema.altı_çizili,
                bağlam,
                |t| t.tema.altı_çizili = !t.tema.altı_çizili,
            )
            .child(tezgah_simgesi("yazi-alti-cizili.svg")),
        )
        .child(
            simge_düğmesi(
                "yazı-üstü-çizili",
                "Üstü çizili",
                tercih.tema.üstü_çizili,
                bağlam,
                |t| t.tema.üstü_çizili = !t.tema.üstü_çizili,
            )
            .child(tezgah_simgesi("yazi-ustu-cizili.svg")),
        )
        .child(ayırıcı())
        .child(
            simge_düğmesi(
                "punto-artır",
                "Yazı boyutunu büyüt",
                false,
                bağlam,
                move |t| {
                    t.tema.punto = sonraki_punto(punto, 1);
                },
            )
            .child(tezgah_simgesi("yazi-buyut.svg")),
        )
        .child(
            simge_düğmesi(
                "punto-azalt",
                "Yazı boyutunu küçült",
                false,
                bağlam,
                move |t| t.tema.punto = sonraki_punto(punto, -1),
            )
            .child(tezgah_simgesi("yazi-kucult.svg")),
        )
}

/// Punto listesinde bir adım ilerler; liste dışına çıkmaz.
///
/// Büyüt/küçült düğmeleri ile punto listesi aynı değer kümesinde kalmalı:
/// aksi hâlde düğmeyle 15'e çıkılır ama listede 15 görünmez.
fn sonraki_punto(şimdiki: f32, yön: i32) -> f32 {
    let sıra = PUNTOLAR
        .iter()
        .position(|punto| (punto - şimdiki).abs() < f32::EPSILON)
        .unwrap_or(1) as i32;
    let hedef = (sıra + yön).clamp(0, PUNTOLAR.len() as i32 - 1);
    PUNTOLAR[hedef as usize]
}

/// `§23` yardımcı eylem simgeleri; en fazla üç yuva çizilir.
pub(crate) fn yardımcı_eylem_şeridi(
    tercih: &crate::TezgahTercihleri,
    sayısal: bool,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Div {
    // `§23` üç yuva sınırı doluyken **kapalı** olanlar açılamaz. Sınır
    // yapılandırma üretiminde sessizce kırpılıyordu: kullanıcı dördüncü
    // düğmeye basıyor, hiçbir şey olmuyordu.
    let dolu = !tercih.yuva_eklenebilir_mi();
    const SINIR: &str = "Üç yuva sınırı dolu · başka bir yuvayı kapatın";
    let yuva = |kimlik: &'static str,
                ad: &'static str,
                simge: &'static str,
                açık: bool,
                yaz: fn(&mut crate::TezgahTercihleri),
                bağlam: &mut Context<GaleriUygulaması>| {
        if dolu && !açık {
            pasif_simge_düğmesi(kimlik, format!("{ad} · {SINIR}"))
                .child(tezgah_simgesi(simge))
                .into_any_element()
        } else {
            simge_düğmesi(kimlik, ad, açık, bağlam, yaz)
                .child(tezgah_simgesi(simge))
                .into_any_element()
        }
    };

    let mut şerit = şerit()
        .child(yuva(
            "eylem-temizle",
            "Temizle",
            "close-circle.svg",
            tercih.temizle,
            |t| t.temizle = !t.temizle,
            bağlam,
        ))
        .child(yuva(
            "eylem-arama",
            "Aramayı başlat",
            "search.svg",
            tercih.arama,
            |t| t.arama = !t.arama,
            bağlam,
        ));
    // `§22` `ParolayıGöster` yuvası yalnız `Gizli` ve `GeçiciGöster`de
    // bulunur. Sayısal türde gizleme hiç kurulamaz — eksen `child` üretmez.
    // Metin türünde eksen vardır ama görünürlük `Açık`/`Opak` iken kapanır:
    // pasif ve gerekçeli kalır, çünkü kullanıcı görünürlüğü değiştirerek
    // açabilir. `Opak`ta reveal yoktur; değer elde değildir.
    if !sayısal {
        şerit = şerit.child(if tercih.görünürlük.parola_yuvası_var() {
            yuva(
                "eylem-parola",
                "Parolayı göster",
                "eye.svg",
                tercih.parola_düğmesi,
                |t| t.parola_düğmesi = !t.parola_düğmesi,
                bağlam,
            )
        } else {
            pasif_simge_düğmesi(
                "eylem-parola",
                match tercih.görünürlük {
                    crate::TezgahGörünürlüğü::Opak => {
                        "Parolayı göster · Opak'ta reveal yoktur, değer elde değil"
                    }
                    _ => "Parolayı göster · yalnız Gizli ve Geçici göster durumlarında",
                },
            )
            .child(tezgah_simgesi("eye.svg"))
            .into_any_element()
        });
    }
    şerit.child(yuva(
        "eylem-seçici",
        "Seçiciyi aç",
        "calendar.svg",
        tercih.seçici,
        |t| t.seçici = !t.seçici,
        bağlam,
    ))
}

/// `§23` yuvaların o an neden görünmediğini söyleyen satır.
///
/// Varsayılan kip `DeğerVarkenKademeli`: kutu boşken **hiçbir** yuva
/// çizilmez. Önizleme boş açıldığı için kullanıcı yuvayı açıyor, kutuda
/// hiçbir şey görmüyor ve düğmeyi bozuk sanıyordu. Kip ekranda yazılı ama
/// iki bilgi arasındaki bağı kurmak okuyucuya kalıyordu.
pub(crate) fn yuva_görünürlük_notu(
    tercih: &crate::TezgahTercihleri,
    alan: &Entity<GirişKutusu>,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Option<Div> {
    use gpui_bilesenleri::YardımcıEylemGörünürlüğü as G;

    let değer_var = !alan.read(bağlam).metin().is_empty();
    let değere_bağlı = matches!(
        tercih.yuva_görünürlüğü,
        G::DeğerVarken | G::DeğerVarkenKademeli
    );
    let mut satırlar: Vec<&'static str> = Vec::new();
    if !değer_var && değere_bağlı && tercih.açık_yuva_sayısı() > 0 {
        satırlar.push("Kutu boşken yuvalar gizli: seçili kip değere bağlı.");
    }
    // `§22` göz simgesi gizlenmiş içeriği açar; içerik zaten açıkken
    // açacağı bir şey yok ve yuva pasif durur. Gerekçe `aria_label`'da
    // vardı ama göz kararıyla okunmuyordu: düğme bozuk sanılıyordu.
    if !tercih.görünürlük.parola_yuvası_var() {
        satırlar.push(
            "Göz simgesi pasif: içerik zaten açık. Görünürlüğü Gizli ya da \
             Geçici göster yapın.",
        );
    }
    if satırlar.is_empty() {
        return None;
    }
    let g = crate::görünüm();
    let t = crate::TezgahTokenları::paletten(crate::palet());
    Some(div().children(satırlar.into_iter().map(move |metin| {
        div().mt_1().child(
            crate::stili_uygula(div(), &g.gövde)
                .text_color(t.soluk)
                .child(metin),
        )
    })))
}

/// `§13`/`§19` alanın taşıdığı üç değer.
///
/// Kutuda tek bir metin görünür ama alan üç ayrı şey tutuyor: şu an
/// yazılan, en son kabul edilen ve `Escape`'in döneceği. Hangisinin ne
/// olduğu ancak kabul ve iptal denendiğinde anlaşılıyordu; panel üçünü
/// yan yana koyar.
pub(crate) fn değer_durumu(
    alan: &Entity<GirişKutusu>,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Div {
    let g = crate::görünüm();
    let t = crate::TezgahTokenları::paletten(crate::palet());
    let kutu = alan.read(bağlam);
    let durum = &kutu.durum;

    let boş = |metin: &str| {
        if metin.is_empty() {
            "‹boş›".to_owned()
        } else {
            metin.to_owned()
        }
    };
    let değer = |değer: Option<&gpui_bilesenleri::Değer>| {
        değer.map_or_else(|| "‹yok›".to_owned(), crate::değer_özeti)
    };

    // Girilen metin gizli kipte panele **yazılmaz**: kutuda maskelediğimiz
    // değeri iki santim yana kopyalamak gizlemeyi anlamsız kılardı.
    let gizli = !matches!(
        kutu.yapılandırma.bildirim().içerik_görünürlüğü,
        gpui_bilesenleri::İçerikGörünürlüğü::Açık
    );
    let girilen = if gizli {
        "‹gizli›".to_owned()
    } else {
        boş(&durum.düzenleme_metni)
    };
    let dönülecek = if gizli {
        "‹gizli›".to_owned()
    } else {
        boş(&durum.düzenleme_başlangıcı.düzenleme_metni)
    };
    let kabul = değer(durum.kabul_edilmiş_değer.as_ref());
    let kirli = durum.düzenleme_kirli;

    let satır = |etiket: &'static str, içerik: String| {
        şerit_satırı()
            .justify_between()
            .mt_1()
            .child(eksen_etiketi_yüzü(etiket))
            .child(
                crate::stili_uygula(div(), &g.gövde)
                    .text_color(t.ana_metin)
                    .child(içerik),
            )
    };

    div()
        .child(
            şerit_satırı()
                .justify_between()
                .child(eksen_etiketi_yüzü("Değer durumu"))
                .child(türetilmiş_rozet(if kirli {
                    "düzenleniyor"
                } else {
                    "temiz"
                })),
        )
        // `§11` seçim aralığı. Grafem sırası okunur birimdir: `ORT-002`
        // bir grafemi çok baytlı ve çok kod noktalı sayabiliyor, bayt
        // konumu panelde yanıltıcı olurdu.
        .child(satır("Seçim", {
            let baş = durum.seçim.başlangıç.grafem_sırası;
            let son = durum.seçim.bitiş.grafem_sırası;
            if durum.seçim_boş_mu() {
                format!("imleç · grafem {baş}")
            } else {
                format!(
                    "{}–{} grafem · {}",
                    baş.min(son),
                    baş.max(son),
                    if durum.seçim.ileri { "ileri" } else { "geri" }
                )
            }
        }))
        .child(satır("Girilen", girilen))
        .child(satır("Kabul edilmiş", kabul))
        // `§19` `Escape`'in hedefi. Kabul edilmiş değerden ayrıdır:
        // düzenleme başlangıcı alana **odaklanıldığı andaki** metindir.
        .child(satır("Escape döner", dönülecek))
}

/// `§26` olay akışı: alanın ürüne söyledikleri.
///
/// Yapılandırma yüzeyi "alan nasıl kurulur"u anlatır; bu panel "kurulan
/// alan ne yayımlar"ı. Programcı `match` yazarken göreceği kanonik varyant
/// adları burada olduğu gibi durur — tezgâh olayı yeniden adlandırmaz.
pub(crate) fn olay_akışı(
    olaylar: &[crate::TezgahOlayı],
    bağlam: &mut Context<GaleriUygulaması>,
) -> Div {
    let g = crate::görünüm();
    let t = crate::TezgahTokenları::paletten(crate::palet());

    let başlık = şerit_satırı()
        .justify_between()
        .child(eksen_etiketi_yüzü("Yayımlanan olaylar"))
        .child(if olaylar.is_empty() {
            türetilmiş_rozet("akış boş").into_any_element()
        } else {
            gösterge_düğmesi("olay-temizle", "Temizle", false, bağlam, |_| {})
                .on_click(bağlam.listener(|bu, _, _, bağlam| {
                    bu.tezgah_olaylarını_temizle(bağlam);
                }))
                .into_any_element()
        });

    let mut kök = div().child(başlık);
    if olaylar.is_empty() {
        return kök.child(
            div().mt(px(ölçü::ARALIK)).child(
                crate::stili_uygula(div(), &g.gövde)
                    .text_color(t.soluk)
                    .child("Alanla etkileşin: yazın, kabul edin, yapıştırın."),
            ),
        );
    }
    for olay in olaylar {
        // Ad ile özet ayrı ağırlıkta: ad kanonik varyanttır ve aranır,
        // özet yalnız o örneğin yükü.
        let mut satır = şerit_satırı()
            .justify_between()
            .mt_1()
            .child(crate::stili_uygula(div(), &g.gövde).child(olay.ad));
        let kuyruk = if olay.sayı > 1 {
            format!("{} · ×{}", olay.özet, olay.sayı)
        } else {
            olay.özet.clone()
        };
        if !kuyruk.is_empty() {
            satır = satır.child(
                crate::stili_uygula(div(), &g.gövde)
                    .text_color(t.soluk)
                    .child(kuyruk),
            );
        }
        kök = kök.child(satır);
    }
    kök
}

/// `§23` yuva kipi ve etkinlik kapısı.
///
/// Yuvaların **hangileri açık** sorusu burada sorulmaz: onlar metin
/// kutusunun sağ üstünde simge olarak duruyor ve tıklanabilir. Bu kart
/// bir zamanlar aynı yuvaları ikinci kez metin düğmesi olarak
/// çiziyordu — simgenin adı ekranda görünmediği için eksen yokmuş gibi
/// okunuyordu. Ad `aria_label`'da var; tekrar çizmek iki ayrı doğruluk
/// kaynağı üretiyordu.
pub(crate) fn kabuk_yuvaları(
    tercih: &crate::TezgahTercihleri,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Div {
    use gpui_bilesenleri::YardımcıEylemGörünürlüğü as G;

    div()
        // `§23.1` ürün yuvası simgesizdir ve kabukta boş bir kare olarak
        // durur. Simgesini ürün sağlar; alan eylem kimliğinden türetmez,
        // bu yüzden şeritte hangi karenin o olduğu ancak buradan okunur.
        .when(tercih.ürün_eylemi, |k| {
            k.child(
                div().mb_1().child(
                    crate::stili_uygula(div(), &crate::görünüm().gövde)
                        .text_color(crate::TezgahTokenları::paletten(crate::palet()).soluk)
                        .child(
                            "Ürün yuvası kabukta simgesiz durur: simge kimliğini ürün \
                             sağlar, alan eylem kimliğinden türetmez.",
                        ),
                ),
            )
        })
        // `§23` yuvanın **ne zaman görüneceği** ve `BİL-040` etkinleştirme
        // kapısı. Dört kipten yalnız `DeğerVarkenKademeli` kurulabiliyordu.
        //
        // Açılır listede, kuşakta değil: dört uzun etiket sol kolonun
        // genişliğini taşıyor ve dördüncüsü ekrandan düşüyordu.
        .child({
            let etiket = format!(
                "Yuva kipi · {}",
                match tercih.yuva_görünürlüğü {
                    G::DeğerVarkenKademeli => "değer varken kademeli",
                    G::DeğerVarken => "değer varken",
                    G::EtkileşimdeKademeli => "etkileşimde kademeli",
                    G::HerZaman => "her zaman",
                }
            );
            let içerik = div()
                .child(kutu_başlığı("Ne zaman görünür", true))
                .children(
                    [
                        ("Değer varken kademeli", G::DeğerVarkenKademeli),
                        ("Değer varken", G::DeğerVarken),
                        ("Etkileşimde kademeli", G::EtkileşimdeKademeli),
                        ("Her zaman", G::HerZaman),
                    ]
                    .into_iter()
                    .map(|(ad, kip)| {
                        div().mt_1().child(tercih_düğmesi(
                            format!("yuva-kip-{ad}"),
                            ad,
                            tercih.yuva_görünürlüğü == kip,
                            bağlam,
                            move |t| t.yuva_görünürlüğü = kip,
                        ))
                    }),
                )
                .child(
                    div().mt_2().child(
                        crate::stili_uygula(div(), &crate::görünüm().gövde)
                            .text_color(crate::TezgahTokenları::paletten(crate::palet()).soluk)
                            .child(
                                "Kip tüm yuvalara birlikte uygulanır. Kanonikte her yuva \
                                 kendi kipini taşır; ürün onları ayrı ayrı kurabilir.",
                            ),
                    ),
                )
                .child(
                    div()
                        .mt_2()
                        .child(kutu_başlığı("Etkinleştirme kapısı", true))
                        .child(div().mt_1().child(tercih_düğmesi(
                            "yuva-etkin",
                            "Yuvalar etkin",
                            tercih.yuvalar_etkin,
                            bağlam,
                            |t| t.yuvalar_etkin = !t.yuvalar_etkin,
                        ))),
                );
            şerit_seçicisi("yuva-kipi", &etiket, içerik, bağlam)
        })
}

/// `§7.1` parça tipografisi · yalnız önizleme **değerine** uygulanır.
///
/// Yerleşim düzeyi atamasını ezer: kabuk ailesi değişmeden değerin ailesi
/// ayrı seçilebilir. `Rolden devral` seçiliyken atama yoktur ve değer
/// kabuktan gelen aileyi kullanır.
pub(crate) fn parça_tipografisi(
    tercih: &crate::TezgahTercihleri,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Div {
    let seçili = tercih
        .parça_ailesi
        .clone()
        .unwrap_or_else(|| "Rolden devral".to_owned());
    let liste = div()
        .child(
            liste_öğesi(
                "parça-devral",
                "Rolden devral",
                tercih.parça_ailesi.is_none(),
            )
            .child("Rolden devral")
            .on_click(bağlam.listener(|bu, _, _, bağlam| {
                bu.tezgahı_değiştir(|t| t.parça_ailesi = None, bağlam);
            })),
        )
        .children(crate::KİTAPLIK_AİLELERİ.map(|ad| {
            liste_öğesi(
                format!("parça-{ad}"),
                ad,
                tercih.parça_ailesi.as_deref() == Some(ad),
            )
            .child(ad)
            .on_click(bağlam.listener(move |bu, _, _, bağlam| {
                bu.tezgahı_değiştir(move |t| t.parça_ailesi = Some(ad.to_owned()), bağlam);
            }))
        }));

    şerit_satırı().child(şerit_seçicisi("parça-ailesi", &seçili, liste, bağlam))
}

/// `§21` yatay hizalama. `Genel` türden çözülür, etiket bunu söyler.
pub(crate) fn yatay_hizalama_şeridi(
    tercih: &crate::TezgahTercihleri,
    sayısal: bool,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Stateful<Div> {
    use gpui_bilesenleri::GirişYatayHizalama as H;
    let genel = if sayısal {
        "Genel · sağa çözülür"
    } else {
        "Genel · sola çözülür"
    };
    segment_şeridi("yatay-hizalama", "Yatay hizalama").children(
        [
            (genel, H::Genel, "hizala-genel.svg"),
            ("Sol", H::Sol, "hizala-sol.svg"),
            ("Orta", H::Orta, "hizala-orta.svg"),
            ("Sağ", H::Sağ, "hizala-sag.svg"),
            (
                "Başlangıç · yazı yönüne duyarlı",
                H::Başlangıç,
                "hizala-baslangic.svg",
            ),
            ("Bitiş · yazı yönüne duyarlı", H::Bitiş, "hizala-bitis.svg"),
        ]
        .into_iter()
        .map(|(başlık, değer, im)| {
            segment_simgesi(
                format!("yatay-{başlık}"),
                başlık,
                tercih.hizalama == değer,
                bağlam,
                move |t| t.hizalama = değer,
            )
            .child(tezgah_simgesi(im))
        }),
    )
}

/// `§21` dikey hizalama.
pub(crate) fn dikey_hizalama_şeridi(
    tercih: &crate::TezgahTercihleri,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Stateful<Div> {
    use gpui_bilesenleri::GirişDikeyHizalama as D;
    segment_şeridi("dikey-hizalama", "Dikey hizalama").children(
        [
            ("Üst", D::Üst, "dikey-ust.svg"),
            ("Orta", D::Orta, "dikey-orta.svg"),
            ("Alt", D::Alt, "dikey-alt.svg"),
        ]
        .into_iter()
        .map(|(başlık, değer, im)| {
            segment_simgesi(
                format!("dikey-{başlık}"),
                başlık,
                tercih.dikey == değer,
                bağlam,
                move |t| t.dikey = değer,
            )
            .child(tezgah_simgesi(im))
        }),
    )
}

// ------------------------------------------------------------ tercih blokları

/// `§9` maske ve sayı biçimi satırı.
pub(crate) fn biçim_satırı(
    tercih: &crate::TezgahTercihleri,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Div {
    // Blok içeriği önce kurulur: `eksen_bloğu` hem içeriği hem bağlamı alır.
    let liste = biçim_listesi(tercih, bağlam);
    let mut kök = div().mt(px(ölçü::BLOK_ARASI)).child(
        şerit_satırı()
            .gap(px(ölçü::BLOK_ARASI))
            .justify_between()
            .child(
                // `şerit_seçicisi`: tetikleyici ile tıklama aynı öğede.
                // `eksen_seçimi` açık durumu okuyor ama işleyici kurmuyordu,
                // yani biçim listesi hiç açılmıyordu.
                div()
                    // Tasarımda `max-width: 232px`; tetikleyici satırı kaplamaz.
                    .max_w(px(232.))
                    .child(şerit_seçicisi(
                        "biçim",
                        tercih.seçili_biçim().etiket,
                        liste,
                        bağlam,
                    )),
            )
            .child(sayı_biçimi_şeridi(tercih, bağlam)),
    );

    // Maske biçim seçiminden **türer** (`metin_girisi_tezgahi.rs` biçim
    // uygulaması): kullanıcı maskeyi doğrudan seçmez. Türetilen değer
    // seçilebilir bir hap olarak çizilseydi, tıklanabilir olduğu sözünü
    // verirdi; rozet noktalı çerçevesiyle bunun tersini söyler.
    kök = kök.child(
        şerit_satırı()
            .mt(px(ölçü::ARALIK))
            .child(eksen_etiketi_yüzü("Maske · türetilmiş, seçilemez"))
            .child(türetilmiş_rozet(tercih.maske.adı())),
    );

    // `ORT-008 §6` işaret konumu: para simgesi ve yüzde işareti önde ya da
    // sonda yazılabilir. Eksen yalnız o iki kipte anlamlıdır; başka kipte
    // okunmayan bir tercih olurdu.
    if matches!(
        tercih.değer_türü,
        crate::TezgahDeğerKipi::ParaBirimi | crate::TezgahDeğerKipi::Yüzde
    ) {
        use gpui_bilesenleri::İşaretKonumu as K;
        let örnek = |konum: K| match (tercih.değer_türü, konum) {
            (crate::TezgahDeğerKipi::Yüzde, K::Önde) => "Önde · %50",
            (crate::TezgahDeğerKipi::Yüzde, K::Sonda) => "Sonda · 50%",
            (_, K::Önde) => "Önde · ₺1.234,56",
            (_, K::Sonda) => "Sonda · 1.234,56 ₺",
        };
        kök = kök.child(
            div()
                .mt(px(ölçü::ARALIK))
                .child(eksen_etiketi_yüzü("İşaret konumu"))
                .child(
                    div()
                        .mt_1()
                        .flex()
                        .gap(px(ölçü::ARALIK))
                        .children([K::Önde, K::Sonda].map(|konum| {
                            gösterge_düğmesi(
                                format!("işaret-konumu-{konum:?}"),
                                örnek(konum),
                                tercih.işaret_konumu == konum,
                                bağlam,
                                move |t| t.işaret_konumu = konum,
                            )
                        })),
                ),
        );
    }

    // Şablon düzenleyici burada **çizilmez**: `§9` maske bölümünde duruyor
    // ve `Özel…` seçiliyken orada açılıyor. İki yerde çizmek aynı
    // `alanlar.desen` kutusunu ekranda ikiye böler.
    kök
}

/// Tasarımın üç öbekli biçim listesi.
///
/// Kanonik karşılığı olmayan satırlar silinmez, pasif çizilir ve nedeni
/// yanlarında yazar. Silmek tasarımı bozar; çalışıyormuş gibi göstermek ise
/// olmayan bir yeteneği satar — ikisi de yanlış.
fn biçim_listesi(
    tercih: &crate::TezgahTercihleri,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Stateful<Div> {
    use crate::{BiçimÖbeği, BİÇİM_SEÇENEKLERİ};

    let seçili_sıra = tercih.biçim_seçeneği;
    let mut kök = div()
        .id("biçim-listesi")
        .max_h(px(320.))
        // Satırlar uzun ve kart genişliği sabit değil: dikey kaydırma tek
        // başına yetmiyordu, satır sağ kenardan kesiliyordu. `min_w(0)`
        // kabın küçülmesine izin verir, `overflow_x_hidden` da kesilen
        // satırın kartın dışına boyanmasını durdurur.
        .min_w(px(0.))
        .overflow_x_hidden()
        .overflow_y_scroll();
    let mut önceki: Option<BiçimÖbeği> = None;

    for (sıra, seçenek) in BİÇİM_SEÇENEKLERİ.iter().enumerate() {
        if önceki != Some(seçenek.öbek) {
            kök = kök.child(
                div()
                    .when(önceki.is_some(), |d| d.mt_2())
                    .child(kutu_başlığı(seçenek.öbek.başlığı(), false)),
            );
            önceki = Some(seçenek.öbek);
        }
        let uygun = tercih.seçenek_uygun_mu(seçenek);
        let seçili = sıra == seçili_sıra;
        // Neden iki türlü: ya sözleşmede karşılığı yok, ya da seçili değer
        // türünde kurulamaz. İkisi farklı şey; ayrı yazılır.
        let neden = seçenek.eksiklik_nedeni().or(if uygun {
            None
        } else {
            Some("seçili değer türünde kurulamaz")
        });

        // Kurulamayan satırın gerekçesi erişilebilir ada girer: soluk bir
        // metin ekran okuyucuya "neden basılamıyor"u söylemez.
        let ad = match neden {
            Some(neden) => format!("{} · {neden}", seçenek.etiket),
            None => seçenek.etiket.to_owned(),
        };
        let t = crate::TezgahTokenları::paletten(crate::palet());
        let satır = liste_öğesi(format!("biçim-{sıra}"), ad, seçili)
            .h_auto()
            .py_0p5()
            .when(!uygun, |d| {
                d.cursor_default().text_color(t.soluk).tab_stop(false)
            })
            .child(seçenek.etiket)
            .when_some(neden, |d, neden| {
                d.child(div().text_color(t.soluk).child(format!("· {neden}")))
            })
            .when(uygun, |d| {
                d.on_click(bağlam.listener(move |bu, _, _, bağlam| {
                    bu.tezgahı_değiştir(move |t| t.biçim_seçeneğini_uygula(sıra), bağlam);
                }))
            });
        kök = kök.child(satır);
    }
    kök
}

/// `§9.3` sayı biçimi: binler ayracı ve ondalık basamak.
///
/// Üç düğme de her zaman çizilir; anlamsız olan pasifleşir. Tasarımın
/// açıklamaları bunu böyle söylüyor: "sayısal değerler seçildiğinde aktif
/// olacak" — yani düğme kaybolmuyor, kapalı duruyor. Kaybolan düğme
/// yerleşimi oynatır ve tercihin var olduğunu gizler.
fn sayı_biçimi_şeridi(
    tercih: &crate::TezgahTercihleri,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Div {
    let ayraç_etkin = tercih.sayısal_mı();
    let basamak_etkin = tercih.ondalık_anlamlı_mı();
    // `§13/19`: Tamsayı ailesinde ondalık derinliği **hiç görünmez**. Bu
    // "kapanan eksen" değil: tür Tamsayı kaldıkça derinlik kurulamaz ve
    // `BiçimTanımı::Tamsayı` kesir taşımaz. Pasif çizmek, tür değişmeden
    // açılabilecekmiş izlenimi verirdi.
    let ondalık_ekseni_var = tercih.değer_türü != crate::TezgahDeğerKipi::Tamsayı;
    let basamak = tercih.ondalık_basamak;

    let ayraç: gpui::AnyElement = if ayraç_etkin {
        simge_düğmesi(
            "binler-ayracı",
            "Binler ayracı",
            tercih.binler_ayracı,
            bağlam,
            |t| t.binler_ayracı = !t.binler_ayracı,
        )
        .child(crate::stili_uygula(div(), &crate::görünüm().eksen_etiketi).child("0.0"))
        .into_any_element()
    } else {
        pasif_simge_düğmesi(
            "ayraç-pasif",
            "Binler ayracı · sayısal değerler seçildiğinde aktif olacak",
        )
        .child(crate::stili_uygula(div(), &crate::görünüm().eksen_etiketi).child("0.0"))
        .into_any_element()
    };

    let azalt: gpui::AnyElement = if basamak_etkin {
        simge_düğmesi(
            "ondalık-azalt",
            "Ondalık haneyi azalt",
            false,
            bağlam,
            move |t| t.ondalık_basamak = basamak.saturating_sub(1),
        )
        .child(crate::stili_uygula(div(), &crate::görünüm().eksen_etiketi).child("←,0"))
        .into_any_element()
    } else {
        pasif_simge_düğmesi(
            "ondalık-azalt-pasif",
            "Ondalık haneyi azalt · ondalık ve para değerleri seçildiğinde aktif olacak",
        )
        .child(crate::stili_uygula(div(), &crate::görünüm().eksen_etiketi).child("←,0"))
        .into_any_element()
    };

    let artır: gpui::AnyElement = if basamak_etkin {
        simge_düğmesi(
            "ondalık-artır",
            "Ondalık haneyi artır",
            false,
            bağlam,
            move |t| t.ondalık_basamak = (basamak + 1).min(crate::EN_ÇOK_ONDALIK),
        )
        .child(crate::stili_uygula(div(), &crate::görünüm().eksen_etiketi).child("→,00"))
        .into_any_element()
    } else {
        pasif_simge_düğmesi(
            "ondalık-artır-pasif",
            "Ondalık haneyi artır · ondalık ve para değerleri seçildiğinde aktif olacak",
        )
        .child(crate::stili_uygula(div(), &crate::görünüm().eksen_etiketi).child("→,00"))
        .into_any_element()
    };

    şerit()
        .child(ayraç)
        .when(ondalık_ekseni_var, |ş| ş.child(azalt).child(artır))
        .when(basamak_etkin, |ş| {
            ş.child(ayırıcı()).child(
                crate::stili_uygula(div().px_1(), &crate::görünüm().eksen_etiketi)
                    .text_color(crate::TezgahTokenları::paletten(crate::palet()).vurgu)
                    .child(format!("{basamak} hane")),
            )
        })
}

/// `§6` içerik yuvaları, `§9.7–9.8` sınır ve sayaç.
///
/// Metin ve seçenek taşıyan tercihler yüzer kutuda durur: yerleşimde yer
/// kaplarlarsa bir tercihi açmak sayfayı aşağı iter ve kullanıcı baktığı yeri
/// kaybeder.
/// `§6` ön ek ve son ek · `Sabitİçerik` sunum rolü.
///
/// Tasarımın `s6ek` bölümü. Ek tonu ek kapalıyken de çizilir: tercih gerçek
/// ve saklanır, bir ek açılır açılmaz görünür olur. Duruma göre kaybolan
/// düğme ızgarayı oynatır ve tercihin varlığını gizler.
pub(crate) fn ön_ek_satırı(
    tercih: &crate::TezgahTercihleri,
    alanlar: &crate::MetinGirişiAlanları,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Div {
    use gpui_bilesenleri::SabitİçerikSunumRolü;

    // Metin kutusunun üstündeki durum etiketi zaten hangi ekin açık
    // olduğunu söylüyor; ayrı bir tetikleyici düğme ve altında bir
    // `Kapat`/`Aç` düğmesi aynı bilgiyi üç kez gösteriyordu.
    let ön_ek_içeriği = div()
        .child(kutu_başlığı_anahtarlı(
            "kutu-ön-ek-anahtar",
            "Ön ek",
            tercih.ön_ek,
            bağlam,
            |t| t.ön_ek = !t.ön_ek,
        ))
        .child(
            div()
                .id("kutu-ön-ek-alanı")
                .child(alanlar.ön_ek_metni.clone()),
        );

    let son_ek_içeriği = div()
        .child(kutu_başlığı_anahtarlı(
            "kutu-son-ek-anahtar",
            "Son ek",
            tercih.son_ek,
            bağlam,
            |t| t.son_ek = !t.son_ek,
        ))
        .child(
            div()
                .id("kutu-son-ek-alanı")
                .child(alanlar.son_ek_metni.clone()),
        );

    let soluk = tercih.ek_sunum_rolü == SabitİçerikSunumRolü::İkincil;
    div().child(
        ızgara_dörtlü()
            // Sabit genişlik: içerik genişliğine bırakınca durum etiketi
            // değişince blok da oynuyordu.
            .child(hücre(ön_ek_içeriği).w(px(ölçü::EK_ALANI)))
            .child(hücre(son_ek_içeriği).w(px(ölçü::EK_ALANI)))
            .child(hücre(tercih_düğmesi(
                "ton-soluk",
                "Ek soluk",
                soluk,
                bağlam,
                |t| t.ek_sunum_rolü = SabitİçerikSunumRolü::İkincil,
            )))
            .child(hücre(tercih_düğmesi(
                "ton-değerle",
                "Ek normal",
                !soluk,
                bağlam,
                |t| t.ek_sunum_rolü = SabitİçerikSunumRolü::DeğerleEş,
            ))),
    )
}

/// `§9.7–9.8` hacim ve sayaç.
///
/// Tasarımın `s97` bölümü. Sayısal türde grafem sınırı ve sayaç uygulanmaz;
/// eksen **vardır ama kapanır**, bu yüzden gizlenmez, pasif ve gerekçeli
/// çizilir (`§9` tür süzgeci · harita §4).
pub(crate) fn hacim_satırı(
    tercih: &crate::TezgahTercihleri,
    sayısal: bool,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Div {
    use gpui_bilesenleri::{SayımBirimi, UzunlukSınırıDavranışı};

    let sınır_içeriği = div()
        .child(kutu_başlığı(
            "Uzunluk sınırı · 12",
            tercih.uzunluk_sınırı,
        ))
        .child(ızgara_dörtlü().child(hücre(tercih_düğmesi(
            "kutu-sınır-anahtar",
            if tercih.uzunluk_sınırı {
                "Kapat"
            } else {
                "Aç"
            },
            tercih.uzunluk_sınırı,
            bağlam,
            |t| t.uzunluk_sınırı = !t.uzunluk_sınırı,
        ))))
        .child(
            ızgara_dörtlü()
                .mt_1p5()
                .child(hücre(tercih_düğmesi(
                    "kutu-sınır-kırp",
                    "Kırp",
                    tercih.uzunluk_davranışı == UzunlukSınırıDavranışı::Kırp,
                    bağlam,
                    |t| t.uzunluk_davranışı = UzunlukSınırıDavranışı::Kırp,
                )))
                .child(hücre(tercih_düğmesi(
                    "kutu-sınır-reddet",
                    "Reddet",
                    tercih.uzunluk_davranışı == UzunlukSınırıDavranışı::Reddet,
                    bağlam,
                    |t| t.uzunluk_davranışı = UzunlukSınırıDavranışı::Reddet,
                ))),
        );

    let sayaç_içeriği = div()
        .child(kutu_başlığı("Sayaç", tercih.sayaç))
        .child(ızgara_dörtlü().child(hücre(tercih_düğmesi(
            "kutu-sayaç-anahtar",
            if tercih.sayaç { "Kapat" } else { "Aç" },
            tercih.sayaç,
            bağlam,
            |t| t.sayaç = !t.sayaç,
        ))))
        .child(
            ızgara_dörtlü()
                .mt_1p5()
                .child(hücre(tercih_düğmesi(
                    "kutu-sayaç-grafem",
                    "Grafem",
                    tercih.sayaç_birimi == SayımBirimi::Grafem,
                    bağlam,
                    |t| t.sayaç_birimi = SayımBirimi::Grafem,
                )))
                .child(hücre(tercih_düğmesi(
                    "kutu-sayaç-kod",
                    "Kod noktası",
                    tercih.sayaç_birimi == SayımBirimi::KodNoktası,
                    bağlam,
                    |t| t.sayaç_birimi = SayımBirimi::KodNoktası,
                )))
                // JS ve Win32 sınırları bu birimi sayar; üç birim aynı
                // metinde farklı sonuç verir ve tezgâh ikisini gösterip
                // üçüncüsünü saklıyordu.
                .child(hücre(tercih_düğmesi(
                    "kutu-sayaç-utf16",
                    "UTF-16 birimi",
                    tercih.sayaç_birimi == SayımBirimi::Utf16Birimi,
                    bağlam,
                    |t| t.sayaç_birimi = SayımBirimi::Utf16Birimi,
                ))),
        )
        .child(
            ızgara_dörtlü()
                .mt_1p5()
                .child(hücre(tercih_düğmesi(
                    "kutu-sayaç-sınır",
                    "Sınırı göster",
                    tercih.sayaç_sınırı_göster,
                    bağlam,
                    |t| t.sayaç_sınırı_göster = true,
                )))
                .child(hücre(tercih_düğmesi(
                    "kutu-sayaç-yalnız",
                    "Yalnız sayı",
                    !tercih.sayaç_sınırı_göster,
                    bağlam,
                    |t| t.sayaç_sınırı_göster = false,
                ))),
        );

    div().child(
        ızgara_dörtlü()
            .child(if sayısal {
                hücre(devre_dışı_düğme(
                    "Uzunluk sınırı",
                    "sayısal türde grafem sınırı uygulanmaz",
                ))
            } else {
                eksen_bloğu("Uzunluk sınırı", tercih.uzunluk_sınırı, sınır_içeriği)
            })
            .child(if sayısal {
                hücre(devre_dışı_düğme(
                    "Sayaç",
                    "sayısal türde grafem sayacı uygulanmaz",
                ))
            } else {
                eksen_bloğu("Sayaç", tercih.sayaç, sayaç_içeriği)
            }),
    )
}

/// `§22` içerik görünürlüğü.
///
/// Tasarımın `s22` bölümü. Gizleme metne özgüdür; sayısal biçimde kurulamaz
/// ve pasif çizilir.
pub(crate) fn görünürlük_satırı(
    tercih: &crate::TezgahTercihleri,
    sayısal: bool,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Div {
    use crate::TezgahGörünürlüğü as G;
    if sayısal {
        return div().child(ızgara_dörtlü().child(hücre(devre_dışı_düğme(
            "Gizli içerik",
            "gizleme metne özgüdür, sayısal biçimde kurulamaz",
        ))));
    }
    // Dört durum birbirini dışlar: kuşak, dört bağımsız hap değil.
    // Taslakta dört durum `2x2` ızgarada ve düğmeler köşeli: adlar
    // ("Geçici göster") tek satıra dört yan yana sığmaz.
    let ızgara = div()
        .flex()
        .flex_col()
        .gap(px(ölçü::ARALIK))
        .children(G::TÜMÜ.chunks(2).map(|çift| {
            ızgara_dörtlü().children(çift.iter().map(|değer| {
                let değer = *değer;
                hücre(geniş_seçenek(
                    format!("yuva-görünürlük-{}", değer.adı()),
                    değer.adı(),
                    tercih.görünürlük == değer,
                    bağlam,
                    move |t| t.görünürlük = değer,
                ))
            }))
        }));
    div()
        .child(ızgara)
        // `§22` geri dönüş politikası yalnız `GeçiciGöster`de anlamlıdır:
        // kanonik doğrulama politikasız `GeçiciGöster`i hata sayar, diğer
        // görünürlüklerde ise politika okunmayan bir tercih olurdu.
        .when(
            tercih.görünürlük == crate::TezgahGörünürlüğü::GeçiciGöster,
            |kap| {
                use crate::TezgahGeçiciGösterimi as P;
                kap.child(
                    div()
                        .mt(px(ölçü::ARALIK))
                        .child(eksen_etiketi_yüzü("Geri dönüş politikası"))
                        .child(
                            div()
                                .mt_1()
                                .flex()
                                .flex_col()
                                .gap(px(ölçü::ARALIK))
                                .children(P::TÜMÜ.map(|değer| {
                                    geniş_seçenek(
                                        format!("geçici-gösterim-{}", değer.adı()),
                                        değer.adı(),
                                        tercih.geçici_gösterim == değer,
                                        bağlam,
                                        move |t| t.geçici_gösterim = değer,
                                    )
                                })),
                        )
                        // Süre serbest yazılmaz; tezgâh sabit deneme
                        // süresini rozetle bildirir.
                        .when(tercih.geçici_gösterim == P::ZamanSınırlı, |kap| {
                            kap.child(
                                şerit_satırı()
                                    .mt(px(ölçü::ARALIK))
                                    .child(eksen_etiketi_yüzü("Süre · sabit deneme değeri"))
                                    .child(türetilmiş_rozet("3 sn")),
                            )
                        }),
                )
            },
        )
        .child(
            şerit_satırı()
                .mt(px(ölçü::ARALIK))
                .child(eksen_etiketi_yüzü(
                    "ParolayıGöster yuvası · yalnız Gizli ve Geçici göster",
                ))
                .child(türetilmiş_rozet(
                    if tercih.görünürlük.parola_yuvası_var() {
                        "var"
                    } else {
                        "yok"
                    },
                )),
        )
}

/// `§24` yer tutucu.
///
/// Tasarımın `s24` bölümünün bugünkü tek ekseni. Yer tutucu erişilebilir adın
/// yerine geçmez ve yalnız değer boşken görünür; erişilebilir ad ekseni F3'te
/// eklenir.
pub(crate) fn yer_tutucu_satırı(
    tercih: &crate::TezgahTercihleri,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Div {
    let yer_tutucu_içeriği = div()
        .child(kutu_başlığı("Yer tutucu", tercih.yer_tutucu))
        .child(
            crate::stili_uygula(div(), &crate::görünüm().eksen_etiketi)
                .text_color(crate::TezgahTokenları::paletten(crate::palet()).soluk)
                .child("Değer girin…"),
        )
        .child(div().mt_2().child(tercih_düğmesi(
            "kutu-yer-tutucu-anahtar",
            if tercih.yer_tutucu { "Kapat" } else { "Aç" },
            tercih.yer_tutucu,
            bağlam,
            |t| t.yer_tutucu = !t.yer_tutucu,
        )));

    div().child(ızgara_dörtlü().child(eksen_bloğu(
        "Yer tutucu",
        tercih.yer_tutucu,
        yer_tutucu_içeriği,
    )))
}

/// `§7` `MetinTanımı::içerik_türü` · yalnız `Metin` ailesinde.
///
/// Seçim **koda yazılmaz**: `GirişYapılandırması` `giriş_türü: GirişTürü`
/// taşımadığı için (`§8/16` borcu) alt tanımın yazılacağı bir alan yok.
/// Rozet bunu söyler — sessizce yazılmış gibi göstermek, kod panelini
/// yalancı çıkarırdı.
pub(crate) fn metin_içerik_türü_satırı(
    tercih: &crate::TezgahTercihleri,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Div {
    use gpui_bilesenleri::MetinİçerikTürü as İ;

    let kuşak = [
        ("Düz", İ::Düz),
        ("EPosta", İ::EPosta),
        ("Telefon", İ::Telefon),
        ("Url", İ::Url),
    ]
    .into_iter()
    .fold(şerit(), |kuşak, (ad, tür)| {
        kuşak.child(tercih_düğmesi(
            format!("içerik-{ad}"),
            ad,
            tercih.metin_içerik_türü == tür,
            bağlam,
            move |t| t.içerik_türünü_seç(tür),
        ))
    });

    şerit_satırı()
        .child(eksen_etiketi_yüzü("Metin İçerik Türü"))
        .child(kuşak)
        .child(türetilmiş_rozet("MetinTanımı::içerik_türü · koda yazılır"))
}

/// `§7` değer türü · **dört kamusal aile**.
///
/// Sözleşme `§7` dört aile tanımlıyor. Para ve yüzde beşinci bir tür değil,
/// `Ondalık` ailesinin biçim profilleri (`§8`); tarih, saat ve tarih/saat de
/// `TarihZaman`ın kipleri. Sekiz düğme çizmek — ki bir süre öyleydi — dört
/// aileyi sekiz tür gibi sunuyordu.
///
/// Kanonik eksen dört ailedir; dokuz kip tezgâhın kendi modelidir ve
/// aile seçimi onu ailenin varsayılanına kurar, alt kip ise `TarihZaman`
/// içinde hangi varyantın seçildiğini söyler.
pub(crate) fn tür_satırı(
    tercih: &crate::TezgahTercihleri,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Div {
    let seçili_aile = crate::tür_ailesi(tercih.değer_türü);

    // Eşit genişlikli ızgara yerine içerik genişliği: dört kısa etiketi
    // kartın enine dağıtmak aralarında okumayı zorlaştıran boşluklar
    // bırakıyordu. Düğmeler sola toplanır, artan yer kartın sağında kalır.
    let aileler = şerit_satırı()
        .mt(px(ölçü::BLOK_ARASI))
        .gap(px(ölçü::ARALIK))
        .children(crate::TezgahAilesi::TÜMÜ.map(|aile| {
            geniş_seçenek(
                format!("tür-{}", aile.adı()),
                aile.adı(),
                seçili_aile == aile,
                bağlam,
                move |t| {
                    // Aile değişince o ailenin varsayılan varyantına
                    // dönülür: `Ondalık`a geçerken para/yüzde profili
                    // taşınsaydı ekranda `Ondalık` yazıp değerde para
                    // kurulmuş olurdu.
                    t.değer_türü = aile.varsayılan_tür();
                },
            )
            // Değer türü satırı diğer tercih satırlarından bir tık
            // yüksek: `height: 30px`, `padding: 0 8px`.
            .h(px(ölçü::TÜR_DÜĞMESİ))
            .px(px(ölçü::TÜR_GENİŞ_DOLGU))
        }));

    let mut kök = div().child(aileler);

    // Kip satırı: aile dört düğme taşıyor, tezgâh kip modeli dokuz
    // varyant. Kipsiz kalan varyantlar ekranda hiç seçilemiyordu.
    let kipler: Option<(&str, &[(&str, crate::TezgahDeğerKipi)])> = match seçili_aile {
        crate::TezgahAilesi::Ondalık => Some(("OndalıkKipi", &crate::ONDALIK_KİPLERİ)),
        crate::TezgahAilesi::TarihZaman => Some(("TarihZamanKipi", &crate::TARİH_KİPLERİ)),
        _ => None,
    };
    if let Some((etiket, liste)) = kipler {
        kök = kök.child(
            div()
                .mt(px(ölçü::ARALIK))
                .child(eksen_etiketi_yüzü(etiket))
                .child(div().mt_1().child(liste.iter().fold(
                    segment_şeridi("değer-kipi", etiket),
                    |kuşak, (ad, tür)| {
                        let tür = *tür;
                        kuşak.child(
                            tercih_düğmesi(
                                format!("kip-{ad}"),
                                ad,
                                tercih.değer_türü == tür,
                                bağlam,
                                move |t| t.değer_türü = tür,
                            )
                            .flex_1()
                            .justify_center(),
                        )
                    },
                ))),
        );
    }

    kök
}

/// `ORT-004 §20.1` imleç hızı ve kalınlığı.
///
/// Tipografi gibi imleç de temanın malıdır; alan kendi hızını tanımlamaz.
/// Tezgâhta olmasının nedeni tam da bu: tema tanımının gerçekten uygulandığı
/// ve `Platform` seçildiğinde işletim sistemi/tarayıcı bildiriminin devreye
/// girdiği görülebilsin.
pub(crate) fn imleç_satırı(
    tercih: &crate::TezgahTercihleri,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Div {
    use crate::İmleçHızı;

    let hız = tercih.tema.imleç_hızı;
    let kalınlık = tercih.tema.imleç_kalınlığı;
    let etiket = format!("İmleç · {} · {kalınlık:.1} px", hız.adı());

    let içerik =
        div()
            .child(kutu_başlığı("İmleç hızı", hız != İmleçHızı::Platform))
            .children(İmleçHızı::TÜMÜ.map(|aday| {
                div().mt_1().child(tercih_düğmesi(
                    format!("imleç-hız-{}", aday.adı()),
                    aday.adı(),
                    hız == aday,
                    bağlam,
                    move |t| t.tema.imleç_hızı = aday,
                ))
            }))
            .child(div().mt_2().child(kutu_başlığı("Kalınlık", true)).child(
                ızgara_dörtlü().children([1.0f32, 1.5, 2.0, 3.0].map(|aday| {
                    hücre(tercih_düğmesi(
                        format!("imleç-kalınlık-{aday}"),
                        &format!("{aday:.1}"),
                        (kalınlık - aday).abs() < f32::EPSILON,
                        bağlam,
                        move |t| t.tema.imleç_kalınlığı = aday,
                    ))
                })),
            ));

    div()
        .mt(px(ölçü::ARALIK))
        .child(şerit_seçicisi("imleç", &etiket, içerik, bağlam))
}

/// `§9.6` sayısal adım ve `§15` sınırı.
///
/// Satır yalnız sayısal türde çizilir: metin ya da tarih alanında adım
/// hiçbir şey yapmaz ve çalışmayan bir tercih gibi okunur. Sarma da yalnız
/// sınır açıkken görünür — sözleşme sarmayı sonlu sınır çiftine bağlıyor,
/// sınırsız sarma geçersiz yapılandırmadır.
pub(crate) fn adım_satırı(
    tercih: &crate::TezgahTercihleri,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Div {
    use crate::AdımÖlçeği;
    use crate::TezgahDeğerKipi;

    let açıkmı = tercih.sayısal_adım;
    let etiket = if açıkmı {
        format!("Adım · {}", tercih.adım_ölçeği.adı())
    } else {
        "Adım · kapalı".to_owned()
    };
    // Tamsayı alanda kesirli adım `§29` hatasıdır; seçenek listesi türe göre
    // daralır ki geçersiz bir yapılandırma hiç kurulamasın.
    let tamsayı = tercih.değer_türü == TezgahDeğerKipi::Tamsayı;
    let ölçekler: Vec<AdımÖlçeği> = AdımÖlçeği::TÜMÜ
        .into_iter()
        .filter(|ölçek| !(tamsayı && ölçek.kesirli_mi()))
        .collect();

    let içerik = div()
        .child(kutu_başlığı("Sayısal adım", açıkmı))
        .child(tercih_düğmesi(
            "adim-etkin",
            "Yön ve sayfa tuşları",
            açıkmı,
            bağlam,
            |t| t.sayısal_adım = !t.sayısal_adım,
        ))
        .when(açıkmı, |k| {
            k.child(
                div()
                    .mt_2()
                    .child(kutu_başlığı("Küçük · büyük", true))
                    .child(ızgara_dörtlü().children(ölçekler.into_iter().map(|ölçek| {
                        hücre(tercih_düğmesi(
                            format!("adim-olcek-{}", ölçek.adı()),
                            ölçek.adı(),
                            tercih.adım_ölçeği == ölçek,
                            bağlam,
                            move |t| t.adım_ölçeği = ölçek,
                        ))
                    }))),
            )
            .child(div().mt_2().child(tercih_düğmesi(
                "adim-hizala",
                "Katına hizala",
                tercih.adım_hizala,
                bağlam,
                |t| t.adım_hizala = !t.adım_hizala,
            )))
            .child(div().mt_1().child(tercih_düğmesi(
                "adim-sinir",
                "0…100 sınırı",
                tercih.adım_sınırı,
                bağlam,
                |t| t.adım_sınırı = !t.adım_sınırı,
            )))
            // Sarma sonlu alt ve üst sınır çiftini ister; sınır kapalıyken
            // bu düğme çalışmayan bir tercih olurdu.
            .when(tercih.adım_sınırı, |k| {
                k.child(div().mt_1().child(tercih_düğmesi(
                    "adim-sarma",
                    "Uçtan uca sar",
                    tercih.adım_sarma,
                    bağlam,
                    |t| t.adım_sarma = !t.adım_sarma,
                )))
            })
            // `AÇK-015`: GPUI `ScrollWheelEvent` yalnız `position`, `delta`,
            // `modifiers` ve `touch_phase` taşır; cihaz/kaynak alanı yoktur.
            // `SayısalTekerlekDavranışı` kanonik API'de var ama tetiklenmesi
            // için gereken kaynak kanıtı çalışma zamanında yok. Düğme
            // silinmiyor: silmek ekseni hiç yokmuş gibi gösterirdi.
            .child(div().mt_1().child(devre_dışı_düğme(
                "Tekerlekle adım",
                "GPUI tekerlek olayı cihaz/kaynak alanı taşımıyor (AÇK-015)",
            )))
        });

    div()
        .mt(px(ölçü::ARALIK))
        .child(şerit_seçicisi("adım", &etiket, içerik, bağlam))
}

/// Dışlayan bir eksenin kuşağı: etiket üstte, seçenekler altında.
///
/// `ızgara_dörtlü` yerine kuşak çünkü bu eksenlerin değerleri birbirini
/// dışlar; dört bağımsız hap, dördünün birden seçili olabileceğini ima
/// ederdi.
fn eksen_kuşağı<D: PartialEq + Copy + 'static>(
    kimlik: &'static str,
    etiket: &'static str,
    seçenekler: &[(&'static str, D)],
    seçili: D,
    bağlam: &mut Context<GaleriUygulaması>,
    yaz: impl Fn(&mut crate::TezgahTercihleri, D) + Copy + 'static,
) -> Div {
    let kuşak = seçenekler
        .iter()
        .fold(segment_şeridi(kimlik, etiket), |kuşak, (ad, değer)| {
            let değer = *değer;
            // Taslakta bu gruplar `width: max-content`: düğmeler içerik
            // genişliğinde durur. `flex_1` ile yaymak dört kısa etiketi
            // kartın enine dağıtıyor ve aralarında okunmayı zorlaştıran
            // boşluklar bırakıyordu.
            kuşak.child(tercih_düğmesi(
                format!("{kimlik}-{ad}"),
                ad,
                seçili == değer,
                bağlam,
                move |t| yaz(t, değer),
            ))
        });
    div()
        .child(eksen_etiketi_yüzü(etiket))
        .child(div().mt_1().child(kuşak))
}

/// `§6` harf dönüşümü, kırpma ve boş metin · `§10` yapıştırma.
///
/// Harf dönüşümü metne özgüdür: sayısal ayrıştırma büyük/küçük harf
/// tanımaz. Sayısal türde eksen **kapanır** — kurulamaz değil, çünkü tür
/// değişince yeniden açılır.
pub(crate) fn metin_isleme_satırı(
    tercih: &crate::TezgahTercihleri,
    sayısal: bool,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Div {
    use gpui_bilesenleri::{
        BoşMetinPolitikası as B, HarfDönüşümü as H, KırpmaPolitikası as K
    };

    let harf: Div = if sayısal {
        div()
            .child(eksen_etiketi_yüzü("Harf dönüşümü"))
            .child(
                div()
                    .mt_1()
                    .child(ızgara_dörtlü().child(hücre(devre_dışı_düğme(
                        "Harf dönüşümü",
                        "harf dönüşümü metne özgüdür; sayısal ayrıştırma harf tanımaz",
                    )))),
            )
    } else {
        eksen_kuşağı(
            "harf-donusumu",
            "Harf dönüşümü",
            &[
                ("Yok", H::Yok),
                ("Büyük", H::Büyük),
                ("Küçük", H::Küçük),
                ("Sözcük başı", H::SözcükBaşı),
            ],
            tercih.harf_dönüşümü,
            bağlam,
            |t, değer| t.harf_dönüşümü = değer,
        )
    };

    div()
        .child(harf)
        .child(div().mt(px(ölçü::ARALIK)).child(eksen_kuşağı(
            "kirpma",
            "Kırpma",
            &[
                ("Yok", K::Yok),
                ("Kabulde", K::KabuldeKırp),
                ("Her zaman", K::HerZamanKırp),
            ],
            tercih.kırpma,
            bağlam,
            |t, değer| t.kırpma = değer,
        )))
        .child(div().mt(px(ölçü::ARALIK)).child(eksen_kuşağı(
            "bos-metin",
            "Boş metin",
            &[
                ("Boş değer", B::BoşDeğer),
                ("Metni koru", B::BoşMetinKoru),
                ("Reddet", B::Reddet),
            ],
            tercih.boş_metin,
            bağlam,
            |t, değer| t.boş_metin = değer,
        )))
}

/// `§10` yapıştırma dönüşümü ve dil etiketi listesi.
///
/// Taslakta ayrı bir kart ve `akis-b`de: dört seçenek alt alta durduğu için
/// tam genişlik bir bölümde dikey olarak çok yer kaplıyordu.
pub(crate) fn yapistirma_satırı(
    tercih: &crate::TezgahTercihleri,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Div {
    use crate::TezgahYapıştırması as Y;

    div()
        .child(
            // Dört seçenek **alt alta**: adlar ("Tanımlı yerel ayarları
            // dene") tek satıra yan yana sığmaz.
            div()
                .flex()
                .flex_col()
                .gap(px(ölçü::ARALIK))
                .children(Y::TÜMÜ.map(|değer| {
                    geniş_seçenek(
                        format!("yapıştırma-{}", değer.adı()),
                        değer.adı(),
                        tercih.yapıştırma == değer,
                        bağlam,
                        move |t| t.yapıştırma = değer,
                    )
                })),
        )
        // Dil etiketleri serbest yazılmaz: geçersiz bir etiket `ORT-002`
        // doğrulamasından döner ve eksen çalışmıyormuş gibi görünürdü.
        // Denenen küme sabittir ve sırası anlamlıdır.
        .when(
            tercih.yapıştırma == Y::TanımlıYerelAyarlarıDene,
            |k| {
                k.child(
                    şerit_satırı()
                        .mt(px(ölçü::ARALIK))
                        .child(eksen_etiketi_yüzü("DilEtiketi listesi · sırayla"))
                        .child(türetilmiş_rozet("tr-TR · en-US")),
                )
            },
        )
}

/// `§29` yapılandırma doğrulaması.
///
/// Tablo **canlıdır**: kanonik `doğrula()` raporundan gelir. Tasarımın
/// `§8.15` tablosu 27 satır sayıyor; fiziksel `GirişYapılandırmaHatası` 16,
/// `GirişYapılandırmaUyarısı` 4 varyant taşıyor. Aradaki fark statik bir
/// liste olarak yazılmaz — üretilemeyen bir çelişkiyi "kural var" diye
/// göstermek, olmayan bir denetimi varmış gibi satmak olurdu.
///
/// Hata ile uyarı ayrı çizilir: hata yapılandırmanın kurulmasını engeller,
/// uyarı kurulur ama bir davranış sürprizi taşır.
pub(crate) fn dogrulama_satırı(rapor: &gpui_bilesenleri::GirişYapılandırmaRaporu) -> Div {
    let t = crate::TezgahTokenları::paletten(crate::palet());
    let g = crate::görünüm();

    let satır = |metin: SharedString, renk: gpui::Hsla| {
        crate::stili_uygula(div(), &g.gövde)
            .text_color(renk)
            .child(metin)
    };

    let mut kök = div().child(
        şerit_satırı()
            .justify_between()
            .child(eksen_etiketi_yüzü("Çelişki"))
            .child(türetilmiş_rozet(if rapor.geçerli_mi() {
                "kurulabilir"
            } else {
                "kurulamaz"
            })),
    );

    if rapor.hatalar.is_empty() && rapor.uyarılar.is_empty() {
        return kök.child(
            div()
                .mt(px(ölçü::ARALIK))
                .child(satır("Çelişki yok.".into(), t.soluk)),
        );
    }

    for hata in &rapor.hatalar {
        kök = kök.child(div().mt_1().child(satır(
            SharedString::new_static(crate::çelişki_metni(hata)),
            t.tehlike,
        )));
    }
    for uyarı in &rapor.uyarılar {
        kök = kök.child(div().mt_1().child(satır(
            SharedString::new_static(crate::uyarı_metni(uyarı)),
            t.uyarı,
        )));
    }
    kök
}

/// `C` bölümü · **önizleme senaryosu**.
///
/// `GirişÖzelDurumu` **türetilmiştir**: tek yazarı `sorunları_uygula` ve
/// kaynağı `§16` sorun kümesidir (`§29.0`). Kart onu seçtirmez, okur —
/// bir süre seçilebiliyordu ve seçilen değer ilk tuş vuruşunda sessizce
/// siliniyordu; `§16.2` göstergesi de onu hiç görmüyordu. Durumu değiştirmek
/// için Doğrulama kartından bir kural kurulur.
///
/// `ORT-004` erişim durumu ayrı bir kanaldır: `salt_okunur`/`etkin`
/// yapılandırmadan gelir, senaryo yalnız ikisi de açıkken konuşur ve
/// doğrulama onu yazmaz.
///
/// `Üzerine` ve `Odaklı` senaryo olarak kurulamaz: ikisi de gerçek
/// etkileşimden gelir ve `EtkileşimDurumu` alanı `GirişKutusu`'nda
/// tutulmuyor. Pasif ve gerekçeli dururlar.
pub(crate) fn turetilmis_durum_satırı(
    tercih: &crate::TezgahTercihleri,
    alan: &Entity<GirişKutusu>,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Div {
    use gpui_bilesenleri::{ErişimDurumu as E, GirişÖzelDurumu as D};

    let (
        yerleşim,
        gösterge_sorunu,
        sorun_sayısı,
        yürürlükteki_durum,
        yürürlükteki_önem,
        yürürlükteki_erişim,
    ) = alan.read_with(bağlam, |alan, _| {
        let durum = alan.durum_göstergesi_durumu();
        (
            durum.yerleşim(),
            durum.birincil_sorun().is_some(),
            alan.sorunlar().len(),
            alan.görsel_durum(),
            alan.önem(),
            alan.erişim(),
        )
    });

    let özel_durumlar = [
        ("Olağan", D::Olağan),
        ("Girdi reddedildi", D::GirdiReddedildi),
        (
            "Düzenlenemez · kaynak bütçesi",
            D::DüzenlenemezKaynakBütçesi,
        ),
        ("Yerel geçersiz", D::YerelGeçersiz),
        ("Dış hata", D::DışHata),
        ("Eşleşme yok uyarısı", D::EşleşmeYokUyarısı),
    ];
    let erişimler = [
        ("Normal", E::Etkin),
        ("Salt okunur", E::SaltOkunur),
        ("Devre dışı", E::DevreDışı),
    ];

    let satır = |etiket: &'static str, değer: SharedString| {
        şerit_satırı()
            .justify_between()
            .child(eksen_etiketi_yüzü(etiket))
            .child(türetilmiş_rozet(değer))
    };
    let yerleşim_adı = match yerleşim {
        gpui_bilesenleri::DurumGöstergesiYerleşimi::Yok => "Yok",
        gpui_bilesenleri::DurumGöstergesiYerleşimi::SatırSonu => "Satır sonu",
        gpui_bilesenleri::DurumGöstergesiYerleşimi::ÜstKöşe => "Üst köşe",
    };

    let durum_adı = özel_durumlar
        .iter()
        .find(|(_, durum)| *durum == yürürlükteki_durum)
        .map_or("—", |(ad, _)| *ad);
    let erişim_adı = erişimler
        .iter()
        .find(|(_, erişim)| *erişim == yürürlükteki_erişim)
        .map_or("—", |(ad, _)| *ad);
    let önem_adı = {
        use gpui_bilesenleri::SemantikÖnem as Ö;
        match yürürlükteki_önem {
            Ö::Hata => "Hata",
            Ö::Uyarı => "Uyarı",
            Ö::Bilgi => "Bilgi",
            Ö::Başarı => "Başarı",
            Ö::Olağan => "Olağan",
        }
    };

    div()
        // `§28` durum okunur, seçilmez: kaynağı sorun kümesidir. Doğrulama
        // kartından bir kural kurup kutuyu ihlal ettirin, buradaki değer
        // değişsin.
        .child(satır("Özel durum", durum_adı.into()))
        .child(
            div()
                .mt(px(ölçü::ARALIK))
                // Taslak burada "ORT-004 ortak etkileşim durumu" yazıyor ama
                // `YÖN-006` kullanıcı metninde sözleşme numarasını yasaklıyor
                // ve bir test bunu koruyor. Anlam sözleşme kimliğinde değil
                // "ortak" sözcüğünde: durum bu bileşene özgü değil, bütün
                // bileşenlerin paylaştığı kanaldan geliyor.
                .child(eksen_etiketi_yüzü("Ortak etkileşim durumu"))
                .child(
                    div()
                        .mt_1()
                        .flex()
                        .flex_wrap()
                        .gap(px(ölçü::ARALIK))
                        // `ORT-004` erişim durumu **türetilmiştir**: kaynağı
                        // `§20`nin `salt_okunur`/`etkin` yapılandırmasıdır.
                        // Bir süre burada da seçilebiliyordu; aynı sonuca
                        // iki yol vardı ve hangisinin geçerli olduğu
                        // yapılandırmadan okunamıyordu (`§29.0`).
                        .child(türetilmiş_rozet(SharedString::new_static(erişim_adı)))
                        // `§23` `üzerinde` de türetilmiştir: tek yazarı
                        // `on_hover`dır. Senaryo olarak kurulabildiği
                        // dönemde ilk fare hareketinde siliniyordu.
                        .child(pasif_gösterge(
                            "senaryo-üzerinde",
                            "Üzerinde",
                            "gerçek işaretçi konumundan gelir, senaryo kurulamaz",
                        ))
                        .child(pasif_gösterge(
                            "senaryo-odaklı",
                            "Odaklı",
                            "gerçek odak durumundan gelir, senaryo kurulamaz",
                        )),
                ),
        )
        // `§28` önem de türetilmiştir: sorunun kendi `GeçerlilikÖnemi`nden
        // çözülür ve ürün onu **kuralın** üzerinde bildirir. Doğrulama
        // kartındaki `Önem` ekseni o kaynaktır; burada sonuç okunur.
        //
        // Zemin uygulaması ayrı bir sunum tercihi: doğrulama onu yazmaz,
        // bu yüzden seçilebilir kalır.
        .child(
            div()
                .mt(px(ölçü::ARALIK))
                .child(satır("Önem düzeyi", önem_adı.into()))
                .child(
                    div()
                        .mt_1()
                        .child(ızgara_dörtlü().child(hücre(tercih_düğmesi(
                            "önem-zemin",
                            "Zemine de uygula",
                            tercih.önem_zemini,
                            bağlam,
                            |t| t.önem_zemini = !t.önem_zemini,
                        )))),
                ),
        )
        .child(
            div()
                .mt(px(ölçü::ARALIK))
                .child(satır("Gösterge yerleşimi", yerleşim_adı.into()))
                .child(div().mt_1().child(satır(
                    "gösterge sorunu",
                    if gösterge_sorunu { "var" } else { "yok" }.into(),
                )))
                .child(div().mt_1().child(satır(
                    "geçerlilik sorunu",
                    SharedString::new(sorun_sayısı.to_string()),
                ))),
        )
}

/// `B` bölümü · platform port kapıları.
///
/// `ACC-005`: port yoksa kontrol pasif ve gerekçeli kalır. Kart her zaman
/// çizilir — kapalı bir portu gizlemek, o yolun hiç olmadığı izlenimini
/// verirdi. Rozetler **seçilemez**: port varlığı bir tercih değil,
/// platformun bildirimidir.
///
/// Tercih tek başına yetmez: bağlı port **ve** verilmiş izin gerekir
/// (`ORT-019 GizlilikKapılıYetenek`). Port yoksa alan olağan manuel girişe
/// döner ve sahte bir doldurma düğmesi gösterilmez.
pub(crate) fn port_satırı(
    portlar: crate::PortDurumu,
    _bağlam: &mut Context<GaleriUygulaması>,
) -> Div {
    let satır = |etiket: &'static str, bağlı: bool, açıklama: &'static str| {
        div()
            .child(
                şerit_satırı()
                    .justify_between()
                    .child(eksen_etiketi_yüzü(etiket))
                    .child(türetilmiş_rozet(if bağlı {
                        "bağlı"
                    } else {
                        "bağlı değil"
                    })),
            )
            .child(div().mt_1().child(eksen_etiketi_yüzü(açıklama)))
    };

    div()
        .child(satır(
            "Otomatik doldurma portu",
            portlar.otomatik_doldurma,
            "Port yoksa alan manuel girişe döner; sahte doldurma düğmesi çizilmez.",
        ))
        .child(div().mt(px(ölçü::ARALIK)).child(satır(
            "Saat dilimi portu",
            portlar.saat_dilimi,
            "Port yoksa saat dilimi tezgâhın kendi çözümünde kalır.",
        )))
        .child(div().mt(px(ölçü::ARALIK)).child(satır(
            "İmleç portu",
            portlar.imleç,
            "Port yoksa imleç hızı ve kalınlığı temanın değerinde kalır.",
        )))
        .child(div().mt(px(ölçü::ARALIK)).child(satır(
            "Uzak doğrulama portu",
            portlar.uzak_doğrulama,
            // Kullanıcı kararıyla tek istisna: Dış doğrulama kartı gösterim
            // beslemesi bağlar. Benzersizlik ve iş kuralı yine üründedir.
            "Dış doğrulama kartının gösterim beslemesi; gerçek sunucu değildir.",
        )))
}

/// `§16` dış sorunlar ve temizleme politikası.
///
/// Politika ancak bir dış bildirimle gözlemlenebilir; "galeri sahte sunucu
/// taklit etmez" duruşu bu eksen için **kullanıcı kararıyla** esnetildi.
/// Besleme sabittir: tek bir `Sunucu` kaynaklı hata ya da boş (temiz)
/// bildirim — benzersizlik/iş kuralı taklidi yok.
pub(crate) fn dis_dogrulama_satırı(
    tercih: &crate::TezgahTercihleri,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Div {
    use gpui_bilesenleri::DışHataTemizleme as D;

    let g = crate::görünüm();
    let t = crate::TezgahTokenları::paletten(crate::palet());
    let hata_düğmesi = crate::seçenek("dış-hata-bildir", &g, &t, "Dış hata bildir", false)
        .on_click(bağlam.listener(|bu, _, pencere, bağlam| {
            bu.tezgah_dış_bildirimi(true, pencere, bağlam);
        }));
    let temiz_düğmesi =
        crate::seçenek("dış-temiz-bildirim", &g, &t, "Temiz bildirim gönder", false).on_click(
            bağlam.listener(|bu, _, pencere, bağlam| {
                bu.tezgah_dış_bildirimi(false, pencere, bağlam);
            }),
        );

    let politikalar = [
        ("Yerel düzenlemede temizle", D::YerelDüzenlemedeTemizle),
        ("Yeniden bildirime kadar koru", D::YenidenBildirimeKadarKoru),
    ];

    div()
        .child(eksen_etiketi_yüzü("Dış hata temizleme"))
        .child(
            div()
                .mt_1()
                .flex()
                .flex_col()
                .gap(px(ölçü::ARALIK))
                .children(politikalar.map(|(ad, değer)| {
                    geniş_seçenek(
                        format!("dış-temizleme-{ad}"),
                        ad,
                        tercih.dış_hata_temizleme == değer,
                        bağlam,
                        move |t| t.dış_hata_temizleme = değer,
                    )
                })),
        )
        .child(
            div()
                .mt(px(ölçü::ARALIK))
                .child(eksen_etiketi_yüzü(
                    "Gösterim beslemesi · gerçek sunucu değil",
                ))
                .child(
                    div()
                        .mt_1()
                        .flex()
                        .flex_col()
                        .gap(px(ölçü::ARALIK))
                        .child(hata_düğmesi)
                        .child(temiz_düğmesi),
                ),
        )
        // Sonuç Türetilmiş Durumlar kartından okunur: `Dış hata` özel
        // durumu, önem ve sorun sayısı oradadır.
        .child(
            şerit_satırı()
                .mt(px(ölçü::ARALIK))
                .child(eksen_etiketi_yüzü("Bildirimin kaynağı"))
                .child(türetilmiş_rozet("Sunucu · Hata")),
        )
}

/// `§24` seçici uyarlaması · `§15` doğrulama kuralı.
///
/// Seçici görünürlüğü yuvaya bağlıdır: yuva kapalıyken bir görünürlük
/// politikası ulaşılamayan bir hattı tarif ederdi.
///
/// Yüzey geometrisi (`AçılırYüzeyYapılandırması`) burada açılmaz: o
/// `ORT-006` alanıdır ve tezgâh ikinci bir yüzey modeli kurmaz.
pub(crate) fn secici_ve_dogrulama_satırı(
    tercih: &crate::TezgahTercihleri,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Div {
    use gpui_bilesenleri::{
        GeçerlilikTetikleyicisi as T, GeçerlilikÖnemi as Ö, SeçiciGörünürlüğü as S,
    };

    let seçici: Div = if tercih.seçici {
        eksen_kuşağı(
            "secici-gorunurluk",
            "Seçici görünürlüğü",
            &[
                ("Uyumlu türde", S::UyumluTürdeGöster),
                ("Her zaman", S::HerZamanGöster),
                ("Gizli", S::Gizli),
            ],
            tercih.seçici_görünürlüğü,
            bağlam,
            |t, değer| t.seçici_görünürlüğü = değer,
        )
    } else {
        div()
            .child(eksen_etiketi_yüzü("Seçici görünürlüğü"))
            .child(
                div()
                    .mt_1()
                    .child(ızgara_dörtlü().child(hücre(devre_dışı_düğme(
                        "Seçici görünürlüğü",
                        "seçici yuvası kapalıyken görünürlük politikası ulaşılamayan bir \
                     hattı tarif eder",
                    )))),
            )
    };

    div()
        .child(seçici)
        // `ORT-009` erişilebilir ad. İkisi ayrı uyarı üretir ve ayrı
        // sorulur: alanın adı varken yuvalar adsız kalabilir. Tezgâh
        // ikisini de sabit kuruyordu, yani `§29`'un iki uyarısı ekranda
        // yazılı olduğu hâlde hiç görülemiyordu.
        .child(
            div()
                .mt(px(ölçü::ARALIK))
                .child(eksen_etiketi_yüzü("Erişilebilir ad"))
                .child(
                    div().mt_1().child(
                        ızgara_dörtlü()
                            .child(hücre(tercih_düğmesi(
                                "erisim-alan-adı",
                                "Alan adlı",
                                tercih.erişilebilir_ad,
                                bağlam,
                                |t| t.erişilebilir_ad = !t.erişilebilir_ad,
                            )))
                            .child(hücre(tercih_düğmesi(
                                "erisim-yuva-adı",
                                "Yuvalar adlı",
                                tercih.yuva_adları,
                                bağlam,
                                |t| t.yuva_adları = !t.yuva_adları,
                            ))),
                    ),
                ),
        )
        .child(
            div()
                .mt(px(ölçü::ARALIK))
                .child(eksen_etiketi_yüzü("Zorunluluk"))
                .child(
                    div()
                        .mt_1()
                        .child(ızgara_dörtlü().child(hücre(tercih_düğmesi(
                            "dogrulama-zorunlu",
                            "Zorunlu alan",
                            tercih.zorunlu,
                            bağlam,
                            |t| t.zorunlu = !t.zorunlu,
                        )))),
                )
                // Tetikleyici ve önem yalnız bir kural varken anlamlı:
                // kuralsız bir tetikleyici hiçbir zaman çalışmaz.
                .when(tercih.zorunlu, |k| {
                    k.child(div().mt(px(ölçü::ARALIK)).child(eksen_kuşağı(
                        "dogrulama-tetikleyici",
                        "Tetikleyici",
                        &[
                            ("Kabulde", T::Kabulde),
                            ("Değişimde", T::Değişimde),
                            ("Odak kaybında", T::OdakKaybında),
                            ("Açık istekte", T::Açıkİstekte),
                        ],
                        tercih.doğrulama_tetikleyicisi,
                        bağlam,
                        |t, değer| t.doğrulama_tetikleyicisi = değer,
                    )))
                    .child(div().mt(px(ölçü::ARALIK)).child(eksen_kuşağı(
                        "dogrulama-onem",
                        "Önem",
                        &[("Hata", Ö::Hata), ("Uyarı", Ö::Uyarı), ("Bilgi", Ö::Bilgi)],
                        tercih.doğrulama_önemi,
                        bağlam,
                        |t, değer| t.doğrulama_önemi = değer,
                    )))
                    // `§29` alan açıkken ilk başarısız kuraldan sonra
                    // kalanları koşturmaz. Kanonik alan iş yapıyordu ama
                    // ekranda karşılığı yoktu: tezgâh yalnız "hepsini
                    // koştur" dalını gösteriyordu.
                    .child(
                        div()
                            .mt(px(ölçü::ARALIK))
                            .child(ızgara_dörtlü().child(hücre(tercih_düğmesi(
                                "dogrulama-ilk-hata",
                                "İlk hatada dur",
                                tercih.ilk_hatada_dur,
                                bağlam,
                                |t| t.ilk_hatada_dur = !t.ilk_hatada_dur,
                            )))),
                    )
                }),
        )
}

/// `§23` bitişik bölüt kuşağı ve arama gönderimi.
///
/// Kuşak alan kabuğunun **dışındadır** ve mantıksal satır sırasına girmez;
/// bu yüzden yardımcı eylem yuvasıyla çakışmaz.
///
/// Arama gönderimi yuvaya bağlıdır: alanı arama alanı yapan şey
/// `AramayıBaşlat` yuvasıdır, gönderim yapılandırması değil. Yuva
/// kapalıyken `arama_gönderimi = Some` olmak `UyumsuzAramaGönderimi`
/// yapılandırma hatasıdır — bu yüzden eksen pasif ve gerekçeli kalır.
pub(crate) fn bolut_satırı(
    tercih: &crate::TezgahTercihleri,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Div {
    use crate::TezgahBölütü as B;
    use gpui_bilesenleri::ÇalışırkenEnterPolitikası as Ç;

    let bölüt = |kimlik: &'static str,
                 etiket: &'static str,
                 seçili: Option<B>,
                 bağlam: &mut Context<GaleriUygulaması>,
                 yaz: fn(&mut crate::TezgahTercihleri, Option<B>)| {
        // "Yok" ayrı bir düğme: bölüt yokluğu burada gerçekten üçüncü bir
        // seçenektir — `§16.2` ankrajının aksine, aynı düğmeye ikinci basış
        // sabit metin ile eylem arasında ayrım yapamazdı.
        let kuşak = [None, Some(B::SabitMetin), Some(B::Eylem)]
            .into_iter()
            .fold(segment_şeridi(kimlik, etiket), |kuşak, değer| {
                let ad = değer.map_or("Yok", |bölüt| bölüt.adı());
                kuşak.child(
                    tercih_düğmesi(
                        format!("{kimlik}-{ad}"),
                        ad,
                        seçili == değer,
                        bağlam,
                        move |t| yaz(t, değer),
                    )
                    .flex_1()
                    .justify_center(),
                )
            });
        div()
            .child(eksen_etiketi_yüzü(etiket))
            .child(div().mt_1().child(kuşak))
    };

    let arama_gönderimi: Div = if tercih.arama {
        div()
            .child(eksen_etiketi_yüzü("Arama gönderimi"))
            .child(div().mt_1().child(ızgara_dörtlü().children([
                hücre(tercih_düğmesi(
                    "arama-enter",
                    "Enter gönderir",
                    tercih.arama_enter_gönderir,
                    bağlam,
                    |t| t.arama_enter_gönderir = !t.arama_enter_gönderir,
                )),
                hücre(tercih_düğmesi(
                    "arama-temizleme",
                    "Temizleme gönderir",
                    tercih.arama_temizleme_gönderir,
                    bağlam,
                    |t| t.arama_temizleme_gönderir = !t.arama_temizleme_gönderir,
                )),
                // `§23` yuvanın `çalışma` alanı. Yalnız arama yuvasında
                // anlamlı: temizleme ya da parola yuvası gönderim üretmez,
                // bu yüzden diğer yuvalar `Yok` kalır.
                hücre(tercih_düğmesi(
                    "arama-gönderim-bağı",
                    "Yuva gönderime bağlı",
                    tercih.arama_gönderime_bağlı,
                    bağlam,
                    |t| t.arama_gönderime_bağlı = !t.arama_gönderime_bağlı,
                )),
            ])))
            .child(div().mt(px(ölçü::ARALIK)).child(eksen_kuşağı(
                "calisirken-enter",
                "Çalışırken Enter",
                &[
                    ("Yoksay", Ç::Yoksay),
                    ("ORT-007'ye bırak", Ç::ORT007PolitikasınaBırak),
                ],
                tercih.çalışırken_enter,
                bağlam,
                |t, değer| t.çalışırken_enter = değer,
            )))
    } else {
        div()
            .child(eksen_etiketi_yüzü("Arama gönderimi"))
            .child(
                div()
                    .mt_1()
                    .child(ızgara_dörtlü().child(hücre(devre_dışı_düğme(
                        "Arama gönderimi",
                        "alanı arama alanı yapan AramayıBaşlat yuvasıdır; yuva kapalıyken \
                     gönderim UyumsuzAramaGönderimi olur",
                    )))),
            )
    };

    div()
        .child(bölüt(
            "baslangic-bolutu",
            "Başlangıç bölütü",
            tercih.başlangıç_bölütü,
            bağlam,
            |t, değer| t.başlangıç_bölütü = değer,
        ))
        .child(div().mt(px(ölçü::ARALIK)).child(bölüt(
            "bitis-bolutu",
            "Bitiş bölütü",
            tercih.bitiş_bölütü,
            bağlam,
            |t, değer| t.bitiş_bölütü = değer,
        )))
        .child(
            div()
                .mt(px(ölçü::ARALIK))
                .child(eksen_etiketi_yüzü("Bölüt sunumu"))
                .child(
                    div().mt_1().child(
                        ızgara_dörtlü()
                            .child(hücre(tercih_düğmesi(
                                "bolut-kademeli",
                                "Kademeli",
                                tercih.bölüt_kademeli,
                                bağlam,
                                |t| t.bölüt_kademeli = !t.bölüt_kademeli,
                            )))
                            // `ORT-003 §3.1` kuşakta yalnız dış köşeler
                            // yuvarlanır, iç kenar alanla paylaşılır.
                            // Bölütün kendi sınırı o paylaşımı görünür
                            // kılar; tezgâh onu `true` sabitliyordu.
                            .child(hücre(tercih_düğmesi(
                                "bolut-sinir",
                                "Kendi sınırı",
                                tercih.bölüt_sınırı,
                                bağlam,
                                |t| t.bölüt_sınırı = !t.bölüt_sınırı,
                            ))),
                    ),
                )
                // Bölütün **içeriği** kanonik olarak ayrı bir
                // `BitişikEylemBölütü` tipinde ve `GirişYapılandırması`'na
                // bağlı değil; ekranda örnek olarak yazılır.
                .when(
                    tercih.başlangıç_bölütü == Some(B::SabitMetin)
                        || tercih.bitiş_bölütü == Some(B::SabitMetin),
                    |k| {
                        k.child(
                            şerit_satırı()
                                .mt_1()
                                .child(eksen_etiketi_yüzü("Sabit metin örneği"))
                                .child(türetilmiş_rozet(B::SABİT_METİN)),
                        )
                    },
                ),
        )
        .child(div().mt(px(ölçü::ARALIK)).child(arama_gönderimi))
}

/// `§16.2` birincil sorun göstergesi · taslağın `durum_göstergesi` satırı.
///
/// Ankraj seçimi üç değerlidir ama üçüncü değer bir düğme değil: seçili
/// ankraja **yeniden basmak** yapılandırmayı `None`'a indirir (`§16.2.4`).
/// Ayrı bir "Kapalı" düğmesi, kapalılığı ankrajla eşdeğer üçüncü bir kademe
/// gibi gösterirdi; oysa kapalılık `durum_göstergesi` alanının yokluğudur.
///
/// `yüzey bağı` ve `aday uygun` taslakta seçilebilir düğmelerdir ama ikisi
/// de **pasif ve gerekçeli** durur: `GirişYüzeyBağı` fiziksel API'de yok ve
/// kabuk kayıtlı üst-köşe geometri adayı beslemiyor. Çizmemek o eksenlerin
/// hiç olmadığını, çalışır çizmek ise olmayan bir yeteneği satardı.
pub(crate) fn gösterge_satırı(
    tercih: &crate::TezgahTercihleri,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Div {
    use gpui_bilesenleri::{
        DurumGöstergesiAçıklamaTercihi as A, DurumGöstergesiYerleşimTercihi as Y,
    };

    let ankraj = [
        ("Satır sonu", Y::SatırSonu),
        ("Üst köşe", Y::UygunsaÜstKöşe),
    ]
    .into_iter()
    .fold(şerit(), |kuşak, (ad, değer)| {
        kuşak.child(gösterge_düğmesi(
            format!("dg-{ad}"),
            ad,
            tercih.gösterge_ankrajı == Some(değer),
            bağlam,
            move |t| t.gösterge_ankrajına_bas(değer),
        ))
    });

    let açıklama = şerit()
        .child(gösterge_düğmesi(
            "dg-aciklama",
            "Sağlayıcı varsayılanı",
            tercih.gösterge_açıklaması == A::SağlayıcıVarsayılanı,
            bağlam,
            |t| {
                t.gösterge_açıklaması = if t.gösterge_açıklaması == A::SağlayıcıVarsayılanı
                {
                    A::Yok
                } else {
                    A::SağlayıcıVarsayılanı
                };
            },
        ))
        .child(pasif_gösterge(
            "yuzey-bagi",
            "Yüzey bağı",
            "GirişYüzeyBağı fiziksel API'de yok, yüzey açılmaz",
        ));

    div()
        // `§16.2.1` gösterge artık kanonik `GirişKutusu::render`ın mantıksal
        // sırasında: yardımcı eylem grubundan sonra, koşullu parça olarak.
        //
        // Görmek için **gerçek** bir sorun gerekir: `§16` birincil sorunu
        // sorun kümesinden seçer. Alttaki "Özel durum" ekseni yalnız görsel
        // yüzeyi taklit eder, sorun kümesine dokunmaz — o yüzden gösterge
        // ona tepki vermez.
        .child(
            div().mb_1().child(
                crate::stili_uygula(div(), &crate::görünüm().gövde)
                    .text_color(crate::TezgahTokenları::paletten(crate::palet()).soluk)
                    .child(
                        "Göstergeyi görmek için gerçek bir sorun gerekir: Doğrulama \
                         kartından `Zorunlu alan`ı açın, kutuyu boş bırakıp Enter'a \
                         basın. Alttaki `Özel durum` ekseni yalnız görsel yüzeyi \
                         taklit eder; sorun kümesine girmediği için gösterge çizilmez. \
                         Üst köşe içinse kabuk henüz kayıtlı geometri adayı \
                         beslemiyor, çözüm satır sonuna düşer.",
                    ),
            ),
        )
        .child(
            şerit_satırı()
                .justify_between()
                .child(eksen_etiketi_yüzü("Durum göstergesi"))
                .child(
                    şerit_satırı().child(ankraj).child(
                        pasif_simge_düğmesi(
                            "aday-uygun",
                            "aday uygun · kabuk kayıtlı üst-köşe geometri adayı beslemiyor; \
                                 çözüm fail-closed ÜstKöşeAdayıYok kalır",
                        )
                        .w_auto()
                        .px(crate::görünüm().hap.yatay_dolgu)
                        .rounded(crate::görünüm().kart.yarıçap)
                        .child("aday uygun"),
                    ),
                ),
        )
        .child(
            şerit_satırı()
                .mt_1()
                .justify_between()
                .child(eksen_etiketi_yüzü("Durum açıklaması"))
                .child(açıklama),
        )
}

/// `§14` varsayılan değer ve sıfırlama davranışı.
///
/// Satır yalnız varsayılanın uygulanabildiği türlerde çizilir: `§14`
/// tarih, saat ve süre türünde varsayılanı uygulamıyor, uygulanmayan bir
/// tercih gösterilmez.
pub(crate) fn varsayılan_satırı(
    tercih: &crate::TezgahTercihleri,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Div {
    use gpui_bilesenleri::SıfırlamaDavranışı as S;

    let etiket = if tercih.varsayılan_değer {
        "Varsayılan · açık".to_owned()
    } else {
        "Varsayılan · kapalı".to_owned()
    };
    let içerik = div()
        .child(kutu_başlığı(
            "Varsayılan değer",
            tercih.varsayılan_değer,
        ))
        .child(tercih_düğmesi(
            "varsayilan-etkin",
            "Türe göre varsayılan",
            tercih.varsayılan_değer,
            bağlam,
            |t| t.varsayılan_değer = !t.varsayılan_değer,
        ))
        .child(div().mt_2().child(kutu_başlığı("Sıfırlama", true)))
        .children(
            [
                ("Boşa dön", S::BoşaDön),
                ("Varsayılana dön", S::VarsayılanaDön),
                ("Üst bileşene bırak", S::ÜstBileşeneBırak),
            ]
            .into_iter()
            .map(|(ad, davranış)| {
                div().mt_1().child(tercih_düğmesi(
                    format!("sifirlama-{ad}"),
                    ad,
                    tercih.sıfırlama == davranış,
                    bağlam,
                    move |t| t.sıfırlama = davranış,
                ))
            }),
        );

    div()
        .mt(px(ölçü::ARALIK))
        .child(şerit_seçicisi("varsayılan", &etiket, içerik, bağlam))
}

/// `§9.5` bölüm gezinimi.
///
/// Satır yalnız bölümlü maskede çizilir: bölüm kavramı tarih ve sayısal
/// maskenin alanıdır, maskesiz alanda hiçbir şey yapmaz.
pub(crate) fn bölüm_satırı(
    tercih: &crate::TezgahTercihleri,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Div {
    let açıkmı = tercih.bölüm_gezinimi;
    let etiket = if açıkmı {
        "Bölüm gezinimi · açık".to_owned()
    } else {
        "Bölüm gezinimi · kapalı".to_owned()
    };
    let içerik = div()
        .child(kutu_başlığı("Bölüm gezinimi", açıkmı))
        .child(tercih_düğmesi(
            "bolum-etkin",
            "Bölümlere ayır",
            açıkmı,
            bağlam,
            |t| t.bölüm_gezinimi = !t.bölüm_gezinimi,
        ))
        .when(açıkmı, |k| {
            k.child(div().mt_2().child(tercih_düğmesi(
                "bolum-atla",
                "Yön tuşu bölüm atlar",
                tercih.bölüm_atla,
                bağlam,
                |t| t.bölüm_atla = !t.bölüm_atla,
            )))
            .child(div().mt_1().child(tercih_düğmesi(
                "bolum-dolunca",
                "Dolunca ilerle",
                tercih.bölüm_dolunca_ilerle,
                bağlam,
                |t| t.bölüm_dolunca_ilerle = !t.bölüm_dolunca_ilerle,
            )))
            .child(div().mt_1().child(tercih_düğmesi(
                "bolum-artir",
                "Yön tuşu artırır",
                tercih.bölüm_artır,
                bağlam,
                |t| t.bölüm_artır = !t.bölüm_artır,
            )))
            // Taşma yalnız artırma açıkken iş yapar.
            .when(tercih.bölüm_artır, |k| {
                k.child(div().mt_1().child(tercih_düğmesi(
                    "bolum-tasar",
                    "Artırma taşar",
                    tercih.bölüm_taşar,
                    bağlam,
                    |t| t.bölüm_taşar = !t.bölüm_taşar,
                )))
            })
            .child(div().mt_1().child(tercih_düğmesi(
                "bolum-ayrac",
                "Ayraç yazımı ilerletir",
                tercih.bölüm_ayraç,
                bağlam,
                |t| t.bölüm_ayraç = !t.bölüm_ayraç,
            )))
        });

    div()
        .mt(px(ölçü::ARALIK))
        .child(şerit_seçicisi("bölüm", &etiket, içerik, bağlam))
}

/// `§25` otomatik doldurma amacı.
///
/// Satır yalnız platform yeteneği açıkken çizilir. Masaüstünde `GPUI` yerel
/// metin alanı açmadığı için yetenek kapalı; orada bu tercih gösterilse
/// açılıp hiçbir şey yapmazdı.
pub(crate) fn doldurma_satırı(
    tercih: &crate::TezgahTercihleri,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Div {
    use gpui_bilesenleri::OtomatikDoldurmaAmacı as A;

    let açıkmı = tercih.otomatik_doldurma;
    let etiket = if açıkmı {
        format!(
            "Otomatik doldurma · {}",
            doldurma_amacı_adı(tercih.doldurma_amacı)
        )
    } else {
        "Otomatik doldurma · kapalı".to_owned()
    };
    let içerik = div()
        .child(kutu_başlığı("Otomatik doldurma", açıkmı))
        .child(tercih_düğmesi(
            "doldurma-etkin",
            "Platforma ipucu ver",
            açıkmı,
            bağlam,
            |t| t.otomatik_doldurma = !t.otomatik_doldurma,
        ))
        .when(açıkmı, |k| {
            k.child(div().mt_2().child(kutu_başlığı("Amaç", true)))
                .children(
                    [
                        A::KullanıcıAdı,
                        A::GeçerliParola,
                        A::EPosta,
                        A::TekKullanımlıkKod,
                    ]
                    .into_iter()
                    .map(|amaç| {
                        div().mt_1().child(tercih_düğmesi(
                            format!("doldurma-{}", doldurma_amacı_adı(amaç)),
                            doldurma_amacı_adı(amaç),
                            tercih.doldurma_amacı == amaç,
                            bağlam,
                            move |t| t.doldurma_amacı = amaç,
                        ))
                    }),
                )
        });

    div()
        .mt(px(ölçü::ARALIK))
        .child(şerit_seçicisi("doldurma", &etiket, içerik, bağlam))
}

fn doldurma_amacı_adı(amaç: gpui_bilesenleri::OtomatikDoldurmaAmacı) -> &'static str {
    use gpui_bilesenleri::OtomatikDoldurmaAmacı as A;
    match amaç {
        A::Ad => "Ad",
        A::KullanıcıAdı => "Kullanıcı adı",
        A::YeniParola => "Yeni parola",
        A::GeçerliParola => "Geçerli parola",
        A::TekKullanımlıkKod => "Tek kullanımlık kod",
        A::EPosta => "E-posta",
        A::Telefon => "Telefon",
        A::AdresSatırı => "Adres",
        A::Kuruluş => "Kuruluş",
    }
}

/// `ORT-002 §5.2` saat dilimi kaynağı.
///
/// Dört kaynağın da tezgâhta olması gerekiyor: platformun bildirdiği dilim,
/// kullanıcının canlı seçimi, ürünün sabitlediği dilim ve hiçbiri
/// çözülemediğinde düşülen yedek. Programcı üçünü aynı alanda karşılaştırıp
/// önceliğin gerçekten uygulandığını görebilmeli.
pub(crate) fn saat_dilimi_satırı(
    tercih: &crate::TezgahTercihleri,
    dilim: &crate::ÇözülmüşSaatDilimi,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Div {
    use crate::{SaatDilimiKaynağı, SaatDilimiKimliği, SaatDilimiTercihi};

    let kaynak_adı = match dilim.kaynak {
        SaatDilimiKaynağı::Platform => "platform",
        SaatDilimiKaynağı::Kullanıcı => "kullanıcı",
        SaatDilimiKaynağı::Ürün => "ürün",
        SaatDilimiKaynağı::Yedek => "yedek",
    };
    let etiket = format!(
        "{} · {} · {kaynak_adı}",
        dilim
            .kimlik
            .as_ref()
            .map_or("—".to_owned(), |k| k.0.to_string()),
        dilim.gmt_farkı.gösterim()
    );

    let satır = |ad: &'static str,
                 seçili: bool,
                 hedef: SaatDilimiTercihi,
                 bağlam: &mut Context<GaleriUygulaması>| {
        div().mt_1().child(tercih_düğmesi(
            format!("dilim-{ad}"),
            ad,
            seçili,
            bağlam,
            move |t| t.saat_dilimi_tercihi = hedef.clone(),
        ))
    };
    let şimdiki = tercih.saat_dilimi_tercihi.clone();
    let içerik = div()
        .child(kutu_başlığı("Saat dilimi kaynağı", true))
        .child(satır(
            "Platform",
            şimdiki == SaatDilimiTercihi::Platform,
            SaatDilimiTercihi::Platform,
            bağlam,
        ))
        .children(
            ["Europe/Istanbul", "Europe/London", "America/New_York"].map(|ad| {
                let kimlik = SaatDilimiKimliği(ad.into());
                let seçili = şimdiki == SaatDilimiTercihi::Kullanıcı(kimlik.clone());
                div().mt_1().child(tercih_düğmesi(
                    format!("dilim-kullanıcı-{ad}"),
                    ad,
                    seçili,
                    bağlam,
                    move |t| t.saat_dilimi_tercihi = SaatDilimiTercihi::Kullanıcı(kimlik.clone()),
                ))
            }),
        )
        .child(
            div()
                .mt_2()
                .child(kutu_başlığı("Ürün sabiti", false))
                .child(
                    crate::stili_uygula(div(), &crate::görünüm().eksen_etiketi)
                        .text_color(crate::TezgahTokenları::paletten(crate::palet()).soluk)
                        .child("Kullanıcı seçimini de ezer."),
                ),
        )
        .child(satır(
            "UTC · ürün sabiti",
            matches!(şimdiki, SaatDilimiTercihi::Ürün(_)),
            SaatDilimiTercihi::Ürün(SaatDilimiKimliği("UTC".into())),
            bağlam,
        ));

    div()
        .mt(px(ölçü::ARALIK))
        .child(şerit_seçicisi("saat-dilimi", &etiket, içerik, bağlam))
}

/// `§12`/`§17`/`§18`/`§20` odak, kabul ve erişim tercihleri.
pub(crate) fn odak_satırı(
    tercih: &crate::TezgahTercihleri,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Div {
    use gpui_bilesenleri::{
        EnterDavranışı as E, EscapeDavranışı as E2, GeçersizOdakDavranışı as G, KabulSeçimi,
        OdakSeçimi,
    };

    // `§18` Enter davranışları yüzer kutuda durur: uzun düğmeler satırda
    // yer kaplarsa ızgara taşar ve hizalar bozulur.
    //
    // Liste yalnız **sahiplik ve odak** ekseni taşır: değeri kim işler,
    // sonra odak nerede kalır. Kabulden sonraki caret yerleşimi ayrı bir
    // eksendir (`kabul_seçimi`) ve buradan seçilmez — iki tercih birbirini
    // ezmesin diye ayrıldılar.
    let enter_içeriği = div().child(kutu_başlığı("Enter", true)).children(
        [
            ("Değeri işle ve kal", E::DeğeriİşleVeKal),
            ("Değeri işle, sonrakine geç", E::DeğeriİşleVeSonrakineGeç),
            ("Üst bileşene bırak", E::ÜstBileşeneBırak),
        ]
        .into_iter()
        .map(|(ad, davranış)| {
            div().mt_1().child(tercih_düğmesi(
                format!("kutu-enter-{ad}"),
                ad,
                tercih.enter == davranış,
                bağlam,
                move |t| t.enter = davranış,
            ))
        }),
    );

    // `§18` kabulden sonraki caret yerleşimi. Enter listesinden ayrı
    // durur: hangi tuşun değeri işlediği ile işlendikten sonra caret'in
    // nereye gittiği farklı sorulardır ve tek listede toplandıklarında
    // biri diğerini eziyordu.
    let kabul_içeriği = div().child(kutu_başlığı("Kabulde", true)).children(
        [
            ("Tümünü seç", KabulSeçimi::TümünüSeç),
            ("Sona git", KabulSeçimi::SonaGit),
            ("İmleci koru", KabulSeçimi::İmleciKoru),
        ]
        .into_iter()
        .map(|(ad, seçim)| {
            div().mt_1().child(tercih_düğmesi(
                format!("kutu-kabul-{ad}"),
                ad,
                tercih.kabul_seçimi == seçim,
                bağlam,
                move |t| t.kabul_seçimi = seçim,
            ))
        }),
    );

    div()
        // Tasarımın üçüncü satırı: Enter · Kabulde · Odakta seç · Salt okunur.
        .child(
            ızgara_dörtlü()
                .mt(px(ölçü::ARALIK))
                .child(eksen_bloğu(
                    "Enter",
                    tercih.enter != E::DeğeriİşleVeKal,
                    enter_içeriği,
                ))
                .child(eksen_bloğu(
                    "Kabulde",
                    tercih.kabul_seçimi != KabulSeçimi::TümünüSeç,
                    kabul_içeriği,
                ))
                .child(hücre(tercih_düğmesi(
                    "odak-tümünü-seç",
                    "Odakta seç",
                    tercih.odak_seçimi == OdakSeçimi::TümünüSeç,
                    bağlam,
                    |t| {
                        t.odak_seçimi = match t.odak_seçimi {
                            OdakSeçimi::TümünüSeç => OdakSeçimi::SonaGit,
                            OdakSeçimi::SonaGit => OdakSeçimi::TümünüSeç,
                        }
                    },
                )))
                .child(hücre(tercih_düğmesi(
                    "erişim-salt-okunur",
                    "Salt okunur",
                    tercih.salt_okunur,
                    bağlam,
                    |t| t.salt_okunur = !t.salt_okunur,
                ))),
        )
        // `§17` dış tıklamada odağı bırakma ve `§12.1` üzerine yazma kipi
        // tasarımda yok; sonradan istenen tercihler oldukları için kendi
        // satırlarında duruyorlar.
        .child(
            ızgara_dörtlü()
                .mt(px(ölçü::ARALIK))
                // Tasarımın `ORT-004` çalışma anahtarı: kapalıyken "Devre
                // dışı", açıkken "Çalışıyor". Tek düğmedir — iki ayrı hap
                // olsaydı ikisinin birden seçili olduğu bir durum düşünmek
                // gerekirdi ki böyle bir durum yok.
                .child(hücre(çalışma_anahtarı(tercih.etkin, bağlam)))
                .child(hücre(tercih_düğmesi(
                    "odak-sekme-durağı",
                    "Sekme durağı",
                    tercih.sekme_durağı,
                    bağlam,
                    |t| t.sekme_durağı = !t.sekme_durağı,
                )))
                .child(hücre(tercih_düğmesi(
                    "odak-dış-tıklama",
                    "Dış tıklama bırakır",
                    tercih.dış_tıklamada_odağı_bırak,
                    bağlam,
                    |t| t.dış_tıklamada_odağı_bırak = !t.dış_tıklamada_odağı_bırak,
                )))
                // `§12.1` alanın açılış kipi. Alan `GirişYapılandırması`na
                // geçiyordu ama ekranda hiçbir karşılığı yoktu: model
                // düzeyinde sınanan, kullanıcının göremediği bir eksendi.
                .child(hücre(tercih_düğmesi(
                    "kip-üzerine-yazma",
                    "Üzerine yazma",
                    tercih.üzerine_yazma,
                    bağlam,
                    |t| t.üzerine_yazma = !t.üzerine_yazma,
                ))),
        )
        // `§17` Escape ve odağın geçersiz değerle bırakılması. İkisi de üç
        // değerli dışlayan eksendir; hap ızgarası dördünün birden seçili
        // olabileceğini ima ederdi.
        .child(div().mt(px(ölçü::ARALIK)).child(eksen_kuşağı(
            "escape",
            "Escape",
            &[
                ("Eski değere dön", E2::EskiDeğereDön),
                ("Değişiklikleri koru", E2::DeğişiklikleriKoru),
                ("Üste bırak", E2::ÜstBileşeneBırak),
            ],
            tercih.escape,
            bağlam,
            |t, değer| t.escape = değer,
        )))
        .child(div().mt(px(ölçü::ARALIK)).child(eksen_kuşağı(
            "gecersiz-odak",
            "Geçersiz değerle odak kaybı",
            &[
                ("İzin ver", G::OdakKaybınaİzinVer),
                ("Odağı koru", G::OdağıKoru),
                ("Eski değere dön", G::EskiDeğereDönVeİzinVer),
            ],
            tercih.geçersiz_odak,
            bağlam,
            |t, değer| t.geçersiz_odak = değer,
        )))
}

/// `§9` maske özeti: düzenleyici kapalıyken o an ne kurulu.
///
/// Şablon düzenleyici yalnız `Özel…` biçiminde açılır. Kapalıyken bölüm
/// boş kalmamalı: maskenin biçim seçiminden **türediğini** ve o an ne
/// olduğunu söyler. Sayısal türde maske hiç kurulamaz ve bunu da yazar —
/// eskiden düzenleyici koşulsuz çiziliyor, Tamsayı alanında hazır desen
/// düğmeleri aktif görünüyor ama basınca maske kurulmuyordu.
pub(crate) fn maske_özeti(
    tercih: &crate::TezgahTercihleri,
    _bağlam: &mut Context<GaleriUygulaması>,
) -> Div {
    let g = crate::görünüm();
    let t = crate::TezgahTokenları::paletten(crate::palet());
    // Mesaj türe bağlı: bu türde hangi maskenin kurulabildiğini
    // `maske_seçenekleri` söylüyor. Tek bir metin, tarih alanında
    // kurulamayan `Özel…` deseni öneriyordu.
    let desen_kurulur = tercih
        .maske_seçenekleri()
        .contains(&crate::TezgahMaskesi::Desen);
    let açıklama = if tercih.sayısal_mı() {
        "Sayısal türde maske kurulmaz: düzenleme yapısı biçim planından gelir."
    } else if tercih.maske != crate::TezgahMaskesi::Yok {
        if desen_kurulur {
            "Maske biçim seçiminden türedi. Şablonu düzenlemek için biçim \
             listesinden `Özel…` seçin."
        } else {
            "Maske biçim seçiminden türedi. Bu türde şablon düzenlenmez: \
             bölümleri takvim tanımı verir."
        }
    } else if desen_kurulur {
        "Maske yok. Biçim listesinden bir giriş maskesi seçin ya da `Özel…` \
         ile kendi şablonunuzu yazın."
    } else {
        "Maske yok. Bu türde yalnız bölümlü tarih maskesi kurulur; biçim \
         listesinden bölümlü bir tarih seçin."
    };
    div()
        .child(
            şerit_satırı()
                .justify_between()
                .child(eksen_etiketi_yüzü("Giriş maskesi"))
                .child(türetilmiş_rozet(tercih.maske.adı())),
        )
        .child(
            div().mt_1().child(
                crate::stili_uygula(div(), &g.gövde)
                    .text_color(t.soluk)
                    .child(açıklama),
            ),
        )
}

/// Maske deseni: şablonu yaz, hazır desenle doldur, kod kümesini gör.
pub(crate) fn maske_tanımı(
    alanlar: &crate::MetinGirişiAlanları,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Div {
    let hazırlar = crate::HAZIR_DESENLER
        .iter()
        .enumerate()
        .map(|(sıra, (ad, desen))| {
            let desen = *desen;
            let d = tercih_düğmesi(format!("desen-{sıra}"), ad, false, bağlam, |_| {});
            hücre(d.on_click(bağlam.listener(move |bu, _, pencere, bağlam| {
                bu.desen_şablonunu_uygula(desen, pencere, bağlam);
            })))
        })
        .collect::<Vec<_>>();

    div()
        .mt_2()
        .child(etiketli_alan("Desen", alanlar.desen.clone()))
        .child(ızgara_üçlü().mt_2().children(hazırlar))
        .child(
            crate::stili_uygula(div().mt_2(), &crate::görünüm().eksen_etiketi)
                .text_color(crate::TezgahTokenları::paletten(crate::palet()).soluk)
                .child(
                    "0 zorunlu rakam · 9 isteğe bağlı · L harf · ? isteğe bağlı harf · \
                     A harf/rakam · & herhangi · > büyüt · < küçült · \\ kaçış · \"…\" sabit",
                ),
        )
}

/// Seçimlerin karşılığı olan Rust kodu.
///
/// Panel yalnız **A bölümünü** — kamusal `GirişYapılandırması` alanlarını —
/// sunar. B (platform yetenekleri ve portlar), C (türetilmiş durumlar) ve D
/// (tema ve önizleme bağlamı) buraya yazılmaz: port ve izin isteyen bir
/// tercihi yapılandırma satırı gibi göstermek, kopyalayan kişiye çalışacağı
/// sözünü verirdi. `salt_okunur` ve `etkin` C'de görünür ama yapılandırılabilir
/// alanlardır ve bu yüzden yazılır.
pub(crate) fn kod_paneli(tercih: &crate::TezgahTercihleri) -> Div {
    let g = crate::görünüm();
    let t = crate::TezgahTokenları::paletten(crate::palet());
    div()
        .child(
            şerit_satırı()
                .justify_between()
                .child(crate::bölüm_başlığı(&g, &t, "Karşılığı olan kod"))
                .child(türetilmiş_rozet("yalnız yapılandırma eksenleri")),
        )
        .child(
            crate::stili_uygula(div(), &g.kod_metni)
                .id("bil-010-tezgah-kod")
                .mt_2()
                .rounded(g.kart.yarıçap)
                // Taslakta kod bloğu koyu zeminli: tema açık olsa da kod
                // alanı kendi kontrastını taşır.
                .bg(t.kod_zemin)
                .p(g.kart.yatay_dolgu)
                .min_h(px(ölçü::KOD_YÜKSEKLİĞİ))
                .text_color(t.kod_metin)
                .child(tercih.kod()),
        )
}

// ------------------------------------------------------------- yapı taşları

/// Simge gruplarını saran ince çerçeve.
/// Yan yana duran denetimleri tek çerçevede toplayan şerit.
fn şerit() -> Div {
    crate::kuşak(
        &crate::görünüm(),
        &crate::TezgahTokenları::paletten(crate::palet()),
    )
}

fn şerit_satırı() -> Div {
    div()
        .flex()
        .items_center()
        .gap(px(ölçü::ARALIK))
        .flex_wrap()
}

/// Tasarımın dört eşit sütunlu ızgarası.
///
/// GPUI'de `grid` yok; eşit genişlik `flex_1` ile `w_0` birlikte verilir.
/// `w_0` olmadan sütunlar içeriklerine göre farklı genişler ve düğmeler
/// hizasını kaybeder — tasarımın dört sütunu birebir aynı genişliktedir.
fn ızgara_dörtlü() -> Div {
    // Sarmalı: hücreler içerik genişliğinde olduğu için dar kartta alt
    // satıra iner, taşmaz.
    div().flex().flex_wrap().gap(px(ölçü::ARALIK))
}

/// Izgara hücresi: sütunu eşit genişliğe zorlar.
fn hücre(içerik: impl IntoElement) -> Div {
    // `items_start`: flex varsayılanı `stretch`tir ve hücre satırın en uzun
    // öğesi kadar uzar. Yanında iki satırlık bir blok varsa tek satırlık hap
    // kapsüle dönüşüyordu — hapın yüksekliği içeriğinden gelmeli.
    //
    // `flex_1` **yok**: hücre içeriği kadar yer kaplar. Eşit genişliğe
    // zorlamak dört kısa düğmeyi kartın enine dağıtıyor ve aralarında
    // okumayı zorlaştıran boşluklar bırakıyordu. Eşit genişlik gerçekten
    // gerektiğinde `eksen_bloğu` kendi `flex_1`ini taşır.
    div().flex().items_start().flex_shrink_0().child(içerik)
}

fn ızgara_üçlü() -> Div {
    div().flex().gap(px(ölçü::ARALIK))
}

fn ayırıcı() -> Div {
    div().w(px(1.)).h(px(16.)).bg(rgb(crate::tezgah_kenarlık()))
}

/// Tezgâh şeridindeki SVG simge.
///
/// Metin karakteri kullanılmaz: kullanılabilir bir glif her yazı tipinde
/// bulunmaz ve eksikse kutu olarak çizilir. Simgeler gömülü varlıklardır.
fn tezgah_simgesi(dosya: &'static str) -> gpui::Svg {
    svg()
        .size(px(ölçü::SİMGE))
        .path(dosya)
        .text_color(rgb(crate::tezgah_ikincil_metin()))
}

/// Yüzer tercih kutusu: tetikleyici düğme ve üste binen panel.
///
/// Panel `deferred` ile kardeşlerinin üstünde çizilir ve `occlude` ile
/// altındaki tıklamaları yutar. Yerleşimde yer kaplamaz: bir tercihi açmak
/// sayfayı aşağı itip kullanıcının baktığı yeri kaybettirmez.
fn eksen_bloğu(etiket: &str, seçili: bool, içerik: impl IntoElement + 'static) -> Div {
    let tetikleyici = blok_etiketi(etiket, seçili)
        .justify_center()
        .child(etiket.to_owned());
    eksen_gövdesi(tetikleyici, içerik).flex_1().w_0()
}

/// Açık/kapalı durumu dışarıdan gelen seçici.
///
/// Tasarımın `<select>` karşılığı: kapalıyken yalnız seçili değer görünür.
/// Liste akışın içinde açılır — bu bir `ORT-006` yüzer yüzeyi değildir ve o
/// konağa bağlı değildir.
fn eksen_seçimi_açık(
    etiket: &str,
    çerçeveli: bool,
    açık: bool,
    içerik: impl IntoElement + 'static,
) -> Div {
    let g = crate::görünüm();
    let t = crate::TezgahTokenları::paletten(crate::palet());
    let mut tetikleyici = div()
        .id(SharedString::new(format!("kutu-{etiket}")))
        .flex()
        .items_center()
        .justify_between()
        .gap_1()
        .cursor_pointer()
        .text_size(g.eksen_etiketi.font_size)
        .font_family(g.eksen_etiketi.font_family.clone())
        .text_color(t.ana_metin)
        .child(etiket.to_owned())
        .child(tezgah_simgesi("acilir.svg").size(px(10.)));
    tetikleyici = if çerçeveli {
        // Tasarımdaki biçim listesi: kendi çerçevesi ve zemini var.
        tetikleyici
            .h(g.anahtar_yüksekliği)
            .px(g.hap.yatay_dolgu)
            .rounded(g.segment.yarıçap)
            .border_1()
            .border_color(t.kenarlık)
            .bg(t.yüzey)
    } else {
        // Yazı tipi ve punto listeleri bir şeridin içinde durur; çerçeveyi
        // şerit taşır.
        tetikleyici
            .h(g.simge_düğmesi)
            .px(g.segment.yatay_dolgu)
            .rounded(g.segment.yarıçap)
    };
    let gövde = eksen_gövdesi(tetikleyici, div());
    if açık {
        gövde.child(içerik)
    } else {
        gövde
    }
}

/// Eksen bloğunun etiketi. Tıklanmaz: açılır panel yoktur.
///
/// Ölçü ve renk `ORT-017` profilinden gelir; hap görünümündedir ama
/// [`crate::durum_hapı`] olduğu için buton rolü taşımaz.
fn blok_etiketi(etiket: &str, seçili: bool) -> Stateful<Div> {
    let g = crate::görünüm();
    let t = crate::TezgahTokenları::paletten(crate::palet());
    crate::durum_hapı(SharedString::new(format!("kutu-{etiket}")), &g, &t, seçili)
}

/// Tetikleyici ile üste binen paneli birleştirir.
/// Eksen bloğu: etiket ve içerik alt alta.
///
/// Eskiden `deferred` bir panel açardı; yeni düzende kartlar normal belge
/// akışındadır ve açılır panel yoktur (harita §5).
fn eksen_gövdesi(etiket: Stateful<Div>, içerik: impl IntoElement + 'static) -> Div {
    div()
        .flex()
        .flex_col()
        .gap_1p5()
        .child(etiket)
        .child(içerik)
}

/// Anahtarlı kutu başlığı: durum etiketinin kendisi tıklanır.
///
/// Ayrı bir `Kapat`/`Aç` düğmesi, aynı bilgiyi iki kez gösteriyordu —
/// başlıkta "açık" yazarken altında "Kapat" düğmesi duruyordu. Durum
/// etiketi tıklanabilir olunca ikisi tek denetime iniyor.
fn kutu_başlığı_anahtarlı(
    kimlik: &'static str,
    ad: &str,
    açık: bool,
    bağlam: &mut Context<GaleriUygulaması>,
    değiştir: impl Fn(&mut crate::TezgahTercihleri) + 'static,
) -> Div {
    let g = crate::görünüm();
    let t = crate::TezgahTokenları::paletten(crate::palet());
    crate::eksen_etiketi(&g, &t, ad.to_owned())
        .flex()
        .items_baseline()
        .justify_between()
        .gap_2()
        .mb_2()
        .text_color(t.ikincil_metin)
        .child(
            crate::stili_uygula(div(), &g.rozet_metni)
                .id(SharedString::new_static(kimlik))
                .role(gpui::Role::Switch)
                .aria_label(SharedString::new(format!(
                    "{ad} · {}",
                    if açık { "açık" } else { "kapalı" }
                )))
                .aria_toggled(if açık {
                    gpui::Toggled::True
                } else {
                    gpui::Toggled::False
                })
                .cursor_pointer()
                // Sabit genişlik: "açık" ile "kapalı" farklı uzunlukta ve
                // tıklamada blok — dolayısıyla altındaki metin kutusu — bir
                // oynayıp bir daralıyordu.
                .w(px(ölçü::DURUM_ETİKETİ))
                .flex()
                .justify_end()
                .px(g.segment.yatay_dolgu)
                .rounded(g.kart.yarıçap)
                .text_color(if açık { t.vurgu } else { t.soluk })
                .hover(|el| el.bg(t.yüzey))
                .on_click(bağlam.listener(move |bu, _, _, bağlam| {
                    bu.tezgahı_değiştir(&değiştir, bağlam);
                }))
                .child(if açık { "açık" } else { "kapalı" }),
        )
}

/// Yüzer kutunun başlık satırı: ad ve açık/kapalı durumu.
fn kutu_başlığı(ad: &str, açık: bool) -> Div {
    let g = crate::görünüm();
    let t = crate::TezgahTokenları::paletten(crate::palet());
    crate::eksen_etiketi(&g, &t, ad.to_owned())
        .flex()
        .items_baseline()
        .justify_between()
        .gap_2()
        .mb_2()
        .text_color(t.ikincil_metin)
        .child(
            div()
                .text_color(if açık { t.vurgu } else { t.soluk })
                .child(if açık { "açık" } else { "kapalı" }),
        )
}

/// `ORT-003` özel köşe yarıçapı kaydırma çubuğu.
///
/// Üst sınır önizleme kutusu yüksekliğinin yarısıdır: `§2` yarıçap kısa
/// kenarın yarısını aşamaz ve tek satırlı alanda kısıtlayan kenar
/// yüksekliktir. İzin ekrandaki yeri `canvas` ile yakalanır; tıklama konumu
/// o iz üzerinde oranlanarak değere çevrilir.
fn köşe_kaydırma_çubuğu(
    tercih: &crate::TezgahTercihleri,
    en_fazla: f32,
    iz: std::rc::Rc<std::cell::Cell<gpui::Bounds<gpui::Pixels>>>,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Div {
    let değer = tercih.köşe_pikseli.unwrap_or(0.);
    let oran = (değer / en_fazla).clamp(0., 1.);

    let yakalama = iz.clone();

    div()
        .child(kutu_başlığı(
            "Özel yarıçap",
            tercih.köşe_pikseli.is_some(),
        ))
        .child(
            crate::stili_uygula(div(), &crate::görünüm().eksen_etiketi)
                .flex()
                .items_baseline()
                .justify_between()
                .text_color(crate::TezgahTokenları::paletten(crate::palet()).ikincil_metin)
                .child("Yarıçap")
                .child(
                    div()
                        .text_color(rgb(crate::tezgah_vurgu()))
                        .child(format!("{değer:.0} px")),
                ),
        )
        .child(
            div()
                .id("köşe-izi")
                .relative()
                .mt_2()
                .h(px(18.))
                .cursor_pointer()
                .child(
                    // İzin ekrandaki yeri yerleşimde yakalanır; tıklama
                    // konumunu değere çevirmenin başka yolu yok.
                    gpui::canvas(
                        move |sınırlar, _, _| yakalama.set(sınırlar),
                        |_, _, _, _| {},
                    )
                    .absolute()
                    .size_full(),
                )
                .child(
                    div()
                        .absolute()
                        .top(px(8.))
                        .left(px(0.))
                        .right(px(0.))
                        .h(px(2.))
                        .bg(rgb(crate::tezgah_kenarlık())),
                )
                .child(
                    div()
                        .absolute()
                        .top(px(8.))
                        .left(px(0.))
                        .w(gpui::relative(oran))
                        .h(px(2.))
                        .bg(rgb(crate::tezgah_vurgu())),
                )
                .child(
                    div()
                        .absolute()
                        .top(px(4.))
                        .left(gpui::relative(oran))
                        .size(px(10.))
                        .rounded_full()
                        .bg(rgb(crate::tezgah_vurgu())),
                )
                .on_mouse_down(
                    gpui::MouseButton::Left,
                    bağlam.listener(move |bu, olay: &gpui::MouseDownEvent, _, bağlam| {
                        bu.köşe_sürüklemesini_ayarla(true);
                        bu.köşe_yarıçapını_konumdan_ayarla(olay.position.x, en_fazla, bağlam);
                    }),
                )
                // Tutamağı basılı sürüklemek değeri sürekli günceller: tek
                // tek tıklamak yerine çubuk ileri sarılabilir.
                .on_mouse_move(bağlam.listener(
                    move |bu, olay: &gpui::MouseMoveEvent, _, bağlam| {
                        if bu.köşe_sürükleniyor_mu() {
                            bu.köşe_yarıçapını_konumdan_ayarla(
                                olay.position.x,
                                en_fazla,
                                bağlam,
                            );
                        }
                    },
                ))
                .on_mouse_up(
                    gpui::MouseButton::Left,
                    bağlam.listener(|bu, _: &gpui::MouseUpEvent, _, _| {
                        bu.köşe_sürüklemesini_ayarla(false)
                    }),
                ),
        )
        .child(
            crate::stili_uygula(div(), &crate::görünüm().eksen_etiketi)
                .flex()
                .justify_between()
                .mt_1()
                .text_color(crate::TezgahTokenları::paletten(crate::palet()).soluk)
                .child("0")
                .child(format!("{en_fazla:.0} · kutu yüksekliğinin yarısı")),
        )
}

/// Şerit içindeki kare simge düğmesi.
fn simge_düğmesi(
    kimlik: impl Into<String>,
    başlık: &'static str,
    seçili: bool,
    bağlam: &mut Context<GaleriUygulaması>,
    değiştir: impl Fn(&mut crate::TezgahTercihleri) + 'static,
) -> Stateful<Div> {
    // Başlık eskiden `let _ =` ile düşürülüyordu: simge düğmeleri
    // erişilebilir ad taşımıyordu. Yüz onu `aria_label`a indirir.
    let g = crate::görünüm();
    let t = crate::TezgahTokenları::paletten(crate::palet());
    crate::simge_düğmesi(SharedString::new(kimlik.into()), &g, &t, başlık, seçili)
        .cursor_pointer()
        .text_color(if seçili { t.vurgu } else { t.ikincil_metin })
        .on_click(bağlam.listener(move |bu, _, _, bağlam| {
            bu.tezgahı_değiştir(&değiştir, bağlam);
        }))
}

/// Yüzer bir listenin dışlayan satırı.
fn liste_öğesi(
    kimlik: impl Into<String>,
    etiket: impl Into<SharedString>,
    seçili: bool,
) -> Stateful<Div> {
    let g = crate::görünüm();
    let t = crate::TezgahTokenları::paletten(crate::palet());
    crate::liste_öğesi(SharedString::new(kimlik.into()), &g, &t, etiket, seçili).h(g.simge_düğmesi)
}

/// Birbirini dışlayan bir eksenin simge öğesi.
///
/// [`simge_düğmesi`]den tek farkı rolüdür: bağımsız bir eylem değil, bir
/// seçim kümesinin üyesidir ve seçili olduğunu `aria_selected` ile bildirir.
fn segment_simgesi(
    kimlik: impl Into<String>,
    başlık: &'static str,
    seçili: bool,
    bağlam: &mut Context<GaleriUygulaması>,
    değiştir: impl Fn(&mut crate::TezgahTercihleri) + 'static,
) -> Stateful<Div> {
    let g = crate::görünüm();
    let t = crate::TezgahTokenları::paletten(crate::palet());
    crate::segment_simgesi(SharedString::new(kimlik.into()), &g, &t, başlık, seçili)
        .cursor_pointer()
        .text_color(if seçili { t.vurgu } else { t.ikincil_metin })
        .on_click(bağlam.listener(move |bu, _, _, bağlam| {
            bu.tezgahı_değiştir(&değiştir, bağlam);
        }))
}

/// Dışlayan eksenin kuşağı: rolsüz [`şerit`]ten farkı `RadioGroup` rolü ve
/// grubu adlandıran `aria_label`dır.
fn segment_şeridi(kimlik: &'static str, ad: &'static str) -> Stateful<Div> {
    crate::segment_kuşağı(
        kimlik,
        &crate::görünüm(),
        &crate::TezgahTokenları::paletten(crate::palet()),
        ad,
    )
}

/// `ORT-004` çalışma anahtarı.
///
/// Tasarımın kuralı: devre dışı ile çalışıyor **tek** düğmedir ve etiket
/// duruma göre değişir. Anahtar `§20`nin `etkin` alanını yazar: kapalıyken
/// erişim `DevreDışı`ya türer, alan erişilebilir ağaçta devre dışı ilan
/// edilir ve odak/Tab dışına çıkar. Tasarımdaki "bekleme bir yasak
/// değildir" notu `ORT-004`ün `çalışıyor` (meşgul) eksenine aittir; o
/// eksen bu anahtarın yazdığı kanal değildir.
fn çalışma_anahtarı(etkin: bool, bağlam: &mut Context<GaleriUygulaması>) -> Stateful<Div> {
    let g = crate::görünüm();
    let t = crate::TezgahTokenları::paletten(crate::palet());
    crate::küçük_anahtar(
        "erişim-etkin",
        &g,
        &t,
        if etkin {
            "Çalışıyor"
        } else {
            "Devre dışı"
        },
        etkin,
    )
    .cursor_pointer()
    .on_click(bağlam.listener(|bu, _, _, bağlam| {
        bu.tezgahı_değiştir(|t| t.etkin = !t.etkin, bağlam);
    }))
}

/// Bir denetim grubunun üstünde duran eksen etiketi.
fn eksen_etiketi_yüzü(metin: impl Into<SharedString>) -> Div {
    crate::eksen_etiketi(
        &crate::görünüm(),
        &crate::TezgahTokenları::paletten(crate::palet()),
        metin,
    )
}

/// Türetilmiş değer rozeti: seçilemez, noktalı çerçevelidir.
///
/// Tasarımın kuralı (`§4.3`): seçilebilen ile türetilen aynı yüzü
/// taşımaz. Rozet tıklanmaz — tıklanabilir görünmesi, kullanıcıya
/// değiştirilebilir olduğu sözünü verir.
fn türetilmiş_rozet(metin: impl Into<SharedString>) -> Div {
    crate::rozet(
        &crate::görünüm(),
        &crate::TezgahTokenları::paletten(crate::palet()),
        metin,
    )
}

/// Yeri korunan ama şu an uygulanamayan simge düğmesi.
///
/// Düğmeyi gizlemek yerine soluklaştırıyoruz: gizlenen düğme hem yerleşimi
/// oynatır hem de o tercihin var olduğunu saklar.
fn pasif_simge_düğmesi(kimlik: &'static str, gerekçe: impl Into<SharedString>) -> Stateful<Div> {
    // Gerekçe eskiden `let _ = başlık;` ile düşürülüyordu: pasif düğme ne
    // erişilebilir ad ne de neden taşıyordu — ekran okuyucuya adsız bir
    // kutu, göreni içinse sebepsiz bir soluk simge kalıyordu.
    let g = crate::görünüm();
    let t = crate::TezgahTokenları::paletten(crate::palet());
    crate::simge_düğmesi(kimlik, &g, &t, gerekçe.into(), false)
        .tab_stop(false)
        .text_color(g.devre_dışı.ön_plan)
}

/// Kurulamayan kısa seçenek · gösterge düğmesiyle aynı yüz.
fn pasif_gösterge(
    kimlik: impl Into<String>,
    etiket: &str,
    gerekçe: &'static str,
) -> Stateful<Div> {
    crate::pasif_gösterge_düğmesi(
        SharedString::new(kimlik.into()),
        &crate::görünüm(),
        etiket.to_owned(),
        gerekçe,
    )
}

/// Kısa, köşeli gösterge düğmesi (`gos-etiket`).
fn gösterge_düğmesi(
    kimlik: impl Into<String>,
    ad: &str,
    seçili: bool,
    bağlam: &mut Context<GaleriUygulaması>,
    değiştir: impl Fn(&mut crate::TezgahTercihleri) + 'static,
) -> Stateful<Div> {
    let g = crate::görünüm();
    let t = crate::TezgahTokenları::paletten(crate::palet());
    crate::gösterge_düğmesi(
        SharedString::new(kimlik.into()),
        &g,
        &t,
        ad.to_owned(),
        seçili,
    )
    .on_click(bağlam.listener(move |bu, _, _, bağlam| {
        bu.tezgahı_değiştir(&değiştir, bağlam);
    }))
}

/// Satırı dolduran geniş seçenek düğmesi · köşeli.
///
/// `tercih_düğmesi`nin hap yüzü kısa seçenekler içindir. Bir kartın
/// genişliğini kaplayan düğmede hap kenarları, tasarımın köşeli
/// kademesinden görünür biçimde ayrılır.
fn geniş_seçenek(
    kimlik: impl Into<String>,
    ad: &str,
    seçili: bool,
    bağlam: &mut Context<GaleriUygulaması>,
    değiştir: impl Fn(&mut crate::TezgahTercihleri) + 'static,
) -> Stateful<Div> {
    let g = crate::görünüm();
    let t = crate::TezgahTokenları::paletten(crate::palet());
    crate::seçenek(
        SharedString::new(kimlik.into()),
        &g,
        &t,
        ad.to_owned(),
        seçili,
    )
    .on_click(bağlam.listener(move |bu, _, _, bağlam| {
        bu.tezgahı_değiştir(&değiştir, bağlam);
    }))
}

/// Adı görünen tercih düğmesi.
fn tercih_düğmesi(
    kimlik: impl Into<String>,
    ad: &str,
    seçili: bool,
    bağlam: &mut Context<GaleriUygulaması>,
    değiştir: impl Fn(&mut crate::TezgahTercihleri) + 'static,
) -> Stateful<Div> {
    // Yüz `ORT-017` profilinden gelir: ölçü, yarıçap ve tipografi burada
    // hesaplanmaz.
    let g = crate::görünüm();
    let t = crate::TezgahTokenları::paletten(crate::palet());
    crate::hap(
        SharedString::new(kimlik.into()),
        &g,
        &t,
        ad.to_owned(),
        seçili,
    )
    .justify_center()
    .on_click(bağlam.listener(move |bu, _, _, bağlam| {
        bu.tezgahı_değiştir(&değiştir, bağlam);
    }))
}

/// Bu türde kurulamayan tercih: görünür ama tıklanamaz.
///
/// Gizlemek yerine soluk göstermek, programcıya yüzeyin tamamını anlatır ve
/// tür değişince düğmenin nereden geleceğini öğretir.
/// Kapanan eksenin pasif yüzü.
///
/// `§9` tür süzgecinde iki ayrı mekanizma vardır: o türde **kurulamayan**
/// eksen hiç çizilmez, **kapanan** eksen ise pasif ve gerekçeli kalır.
/// Kapanan ekseni gizlemek "bu eksen yok" der ve programcıyı yanıltır.
pub(crate) fn kapalı_eksen(ad: &'static str, gerekçe: &'static str) -> Stateful<Div> {
    div()
        .id(ad)
        .role(gpui::Role::Group)
        .aria_label(SharedString::from(format!("{ad} — {gerekçe}")))
        .flex()
        .flex_col()
        .gap_1()
        .child(
            crate::stili_uygula(div(), &crate::görünüm().eksen_etiketi)
                .text_color(crate::TezgahTokenları::paletten(crate::palet()).soluk)
                .child(ad),
        )
        .child(
            crate::stili_uygula(div(), &crate::görünüm().eksen_etiketi)
                .text_color(crate::TezgahTokenları::paletten(crate::palet()).soluk)
                .child(gerekçe),
        )
}

/// Pasif düğme · gerekçe **zorunlu**.
///
/// Tasarımın `§4.3` kuralı: pasif bir denetim asla sessiz olmaz, neden kapalı
/// olduğunu söyler. Gerekçe erişilebilir ada iner; renk tek kanal değildir.
fn devre_dışı_düğme(ad: &'static str, gerekçe: &'static str) -> Stateful<Div> {
    crate::hap_pasif(ad, &crate::görünüm(), ad, gerekçe).justify_center()
}

/// Etiketli metin tercihi: ön ek, son ek ve desen alanları.
fn etiketli_alan(etiket: &'static str, alan: Entity<GirişKutusu>) -> Div {
    div()
        .flex()
        .flex_col()
        .gap_1()
        .child(
            crate::stili_uygula(div(), &crate::görünüm().eksen_etiketi)
                .text_color(crate::TezgahTokenları::paletten(crate::palet()).ikincil_metin)
                .child(etiket),
        )
        .child(
            div()
                .id(SharedString::new(format!("tezgah-alan-{etiket}")))
                .w(px(200.))
                .child(alan),
        )
}

fn seçim_sergisi(seçili: u8, bağlam: &mut Context<GaleriUygulaması>) -> Stateful<Div> {
    let seçim_kipi = SeçimKipi::Tekli;
    let seçenekler = ["Temel", "Rahat", "Yoğun"]
        .into_iter()
        .enumerate()
        .map(|(sıra, etiket)| {
            div()
                .id(format!("bil-020-seçenek-{sıra}"))
                .cursor_pointer()
                .rounded_md()
                .border_1()
                .border_color(if seçili == sıra as u8 {
                    rgb(kabuk_vurgusu())
                } else {
                    rgb(kenarlık())
                })
                .bg(if seçili == sıra as u8 {
                    rgb(crate::palet().kabuk_seçili_zemin)
                } else {
                    rgb(crate::palet().kabuk_kart)
                })
                .px_3()
                .py_2()
                .text_sm()
                .child(etiket)
                .on_click(bağlam.listener(move |bu, _, _, bağlam| {
                    bu.sergi_seçimi = sıra as u8;
                    bağlam.notify();
                }))
        });
    sergi_kartı(
        "bil-020-canlı-sergi",
        "Seçim ve Liste",
        "Tek seçim ve görünür seçili durum",
    )
    .child(div().mt_5().flex().flex_wrap().gap_2().children(seçenekler))
    .child(
        div()
            .id("bil-020-devre-dışı-seçenek")
            .mt_2()
            .rounded_md()
            .border_1()
            .border_color(rgb(crate::palet().kabuk_kenarlık))
            .bg(rgb(crate::palet().kabuk_zemin))
            .px_3()
            .py_2()
            .text_xs()
            .text_color(rgb(crate::palet().soluk))
            .child("Kilitli seçenek · Devre dışı"),
    )
    .child(
        div()
            .mt_4()
            .text_xs()
            .text_color(rgb(ikincil_metin()))
            .child(format!(
                "Kip: {} · Seçili: {}",
                match seçim_kipi {
                    SeçimKipi::Tekli => "Tekli",
                    SeçimKipi::ÇokluBasit => "Çoklu",
                    SeçimKipi::ÇokluGenişletilmiş => "Genişletilmiş",
                },
                ["Temel", "Rahat", "Yoğun"][seçili as usize]
            )),
    )
}

fn mantıksal_giriş_sergisi(
    onaylı: bool,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Stateful<Div> {
    let değer = if onaylı {
        MantıksalDeğer::Açık
    } else {
        MantıksalDeğer::Kapalı
    };
    sergi_kartı(
        "bil-030-canlı-sergi",
        "Onay Kutusu",
        "İkili değer ve metinden bağımsız durum işareti",
    )
    .child(
        div()
            .mt_3()
            .flex()
            .items_center()
            .gap_2()
            .text_xs()
            .text_color(rgb(ikincil_metin()))
            .child("▣ Karma")
            .child("·")
            .child(
                div()
                    .rounded_sm()
                    .bg(rgb(crate::palet().kabuk_zemin))
                    .px_2()
                    .py_1()
                    .text_color(rgb(crate::palet().soluk))
                    .child("Devre dışı"),
            ),
    )
    .child(
        div()
            .id("bil-030-onay-kutusu")
            .mt_5()
            .flex()
            .items_center()
            .gap_3()
            .cursor_pointer()
            .on_click(bağlam.listener(|bu, _, _, bağlam| {
                bu.sergi_onaylı = !bu.sergi_onaylı;
                bağlam.notify();
            }))
            .child(
                div()
                    .size(px(22.))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_sm()
                    .border_1()
                    .border_color(if onaylı {
                        rgb(kabuk_vurgusu())
                    } else {
                        rgb(kenarlık())
                    })
                    .bg(if onaylı {
                        rgb(kabuk_vurgusu())
                    } else {
                        rgb(crate::palet().kabuk_kart)
                    })
                    .text_sm()
                    .text_color(rgb(crate::palet().kabuk_kart))
                    .child(if onaylı { "✓" } else { "" }),
            )
            .child(div().text_sm().child("Değişiklikleri kabul ediyorum")),
    )
    .child(
        div()
            .mt_4()
            .text_xs()
            .text_color(rgb(ikincil_metin()))
            .child(match değer {
                MantıksalDeğer::Açık => "Kanonik değer: Açık",
                MantıksalDeğer::Kapalı => "Kanonik değer: Kapalı",
                MantıksalDeğer::Karma => "Kanonik değer: Karma",
            }),
    )
}

fn sekme_sergisi(seçili: u8, bağlam: &mut Context<GaleriUygulaması>) -> Stateful<Div> {
    let kipler = [
        ("Genel", SekmeKipi::Olağan),
        ("Önizleme", SekmeKipi::Önizleme),
        ("Sabit", SekmeKipi::Sabitlenmiş),
    ];
    let sekmeler = kipler.into_iter().enumerate().map(|(sıra, (etiket, kip))| {
        div()
            .id(format!("bil-050-sekme-{sıra}"))
            .cursor_pointer()
            .border_b_2()
            .border_color(if seçili == sıra as u8 {
                rgb(kabuk_vurgusu())
            } else {
                rgb(crate::palet().kabuk_kart)
            })
            .px_3()
            .py_2()
            .text_sm()
            .text_color(if seçili == sıra as u8 {
                rgb(kabuk_vurgusu())
            } else {
                rgb(ikincil_metin())
            })
            .child(match kip {
                SekmeKipi::Olağan => etiket.to_owned(),
                SekmeKipi::Önizleme => etiket.to_owned(),
                SekmeKipi::Sabitlenmiş => etiket.to_owned(),
            })
            .on_click(bağlam.listener(move |bu, _, _, bağlam| {
                bu.sergi_sekmesi = sıra as u8;
                bağlam.notify();
            }))
    });
    sergi_kartı(
        "bil-050-canlı-sergi",
        "Sekmeler",
        "Olağan, önizleme ve sabitlenmiş sekme kipleri",
    )
    .child(
        div()
            .id("bil-050-sekme-çubuğu")
            .mt_4()
            .flex()
            .border_b_1()
            .border_color(rgb(kenarlık()))
            .children(sekmeler),
    )
    .child(
        div()
            .mt_4()
            .rounded_md()
            .bg(rgb(crate::palet().kabuk_zemin))
            .p_3()
            .text_sm()
            .child(format!(
                "Etkin içerik: {}",
                ["Genel ayarlar", "Geçici önizleme", "Sabit çalışma alanı"][seçili as usize]
            )),
    )
}

fn panel_sergisi(açık: bool, bağlam: &mut Context<GaleriUygulaması>) -> Stateful<Div> {
    let konum = PanelKonumu::Sağ;
    sergi_kartı(
        "bil-060-canlı-sergi",
        "Panel",
        "Görünürlük, konum ve yaşam döngüsü",
    )
    .child(
        div()
            .mt_4()
            .flex()
            .items_center()
            .justify_between()
            .rounded_t_md()
            .bg(rgb(crate::palet().kabuk_zemin))
            .px_3()
            .py_2()
            .child(div().text_sm().child("Özellikler"))
            .child(
                div()
                    .id("bil-060-panel-geçişi")
                    .cursor_pointer()
                    .text_xs()
                    .text_color(rgb(kabuk_vurgusu()))
                    .child(if açık { "Gizle" } else { "Göster" })
                    .on_click(bağlam.listener(|bu, _, _, bağlam| {
                        bu.sergi_paneli_açık = !bu.sergi_paneli_açık;
                        bağlam.notify();
                    })),
            ),
    )
    .when(açık, |kart| {
        kart.child(
            div()
                .id("bil-060-panel-içeriği")
                .border_1()
                .border_color(rgb(kenarlık()))
                .rounded_b_md()
                .p_3()
                .text_sm()
                .child("Seçili nesnenin özellikleri")
                .child(
                    div()
                        .mt_2()
                        .text_xs()
                        .text_color(rgb(ikincil_metin()))
                        .child("Yeniden boyutlandırılabilir · Zoomlanabilir"),
                ),
        )
    })
    .child(
        div()
            .mt_3()
            .text_xs()
            .text_color(rgb(ikincil_metin()))
            .child(format!(
                "Kanonik konum: {} · Görünür: {açık}",
                match konum {
                    PanelKonumu::Sol => "Sol",
                    PanelKonumu::Sağ => "Sağ",
                    PanelKonumu::Alt => "Alt",
                }
            )),
    )
}

fn araç_çubuğu_sergisi(
    taşma_açık: bool,
    bağlam: &mut Context<GaleriUygulaması>,
) -> Stateful<Div> {
    let bölge = AraçBölgesi::BirincilBaşlangıç;
    sergi_kartı(
        "bil-070-canlı-sergi",
        "Araç Çubuğu",
        "Bölge, sıra, daralma ve taşma",
    )
    .child(
        div()
            .id("bil-070-araç-çubuğu")
            .mt_4()
            .flex()
            .items_center()
            .gap_2()
            .rounded_md()
            .border_1()
            .border_color(rgb(kenarlık()))
            .p_2()
            .child(araç_düğmesi("Yeni", true))
            .child(araç_düğmesi("Kaydet", true))
            .child(araç_düğmesi("Geri al", false))
            .child(
                div()
                    .id("bil-070-taşma")
                    .cursor_pointer()
                    .rounded_md()
                    .bg(rgb(crate::palet().kabuk_zemin))
                    .px_3()
                    .py_2()
                    .text_sm()
                    .child("Daha fazla")
                    .on_click(bağlam.listener(|bu, _, _, bağlam| {
                        bu.sergi_araç_taşması_açık = !bu.sergi_araç_taşması_açık;
                        bağlam.notify();
                    })),
            ),
    )
    .when(taşma_açık, |kart| {
        kart.child(
            div()
                .id("bil-070-taşma-menüsü")
                .mt_2()
                .rounded_md()
                .border_1()
                .border_color(rgb(kenarlık()))
                .bg(rgb(crate::palet().kabuk_kart))
                .p_2()
                .text_sm()
                .child("Yazdır")
                .child(div().mt_2().child("Dışa aktar")),
        )
    })
    .child(
        div()
            .mt_3()
            .text_xs()
            .text_color(rgb(ikincil_metin()))
            .child(match bölge {
                AraçBölgesi::BirincilBaşlangıç => "Bölge: Birincil başlangıç",
                AraçBölgesi::BirincilBitiş => "Bölge: Birincil bitiş",
                AraçBölgesi::İkincil => "Bölge: İkincil",
            }),
    )
}

fn araç_düğmesi(etiket: &'static str, etkin: bool) -> impl IntoElement {
    div()
        .rounded_md()
        .px_2()
        .py_2()
        .text_xs()
        .bg(rgb(if etkin { 0xf8fafc } else { 0xf3f4f6 }))
        .text_color(rgb(if etkin { 0x111827 } else { 0x9ca3af }))
        .child(etiket)
}

fn modal_sergisi(açık: bool, bağlam: &mut Context<GaleriUygulaması>) -> Stateful<Div> {
    let tür = ModalTürü::OnayDialogu;
    sergi_kartı(
        "bil-080-canlı-sergi",
        "Modal ve Dialog",
        "Onay kararı, dismissal ve görünür modal durumu",
    )
    .child(
        div()
            .id("bil-080-modal-aç")
            .mt_4()
            .w(px(116.))
            .cursor_pointer()
            .rounded_md()
            .bg(rgb(kabuk_vurgusu()))
            .px_3()
            .py_2()
            .text_sm()
            .text_color(rgb(crate::palet().kabuk_kart))
            .child("Dialog aç")
            .on_click(bağlam.listener(|bu, _, _, bağlam| {
                bu.sergi_modali_açık = true;
                bağlam.notify();
            })),
    )
    .when(açık, |kart| {
        kart.child(
            div()
                .id("bil-080-dialog")
                .mt_3()
                .rounded_lg()
                .border_1()
                .border_color(rgb(kenarlık()))
                .bg(rgb(crate::palet().kabuk_kart))
                .p_3()
                .child(div().text_sm().child("Değişiklikler kaydedilsin mi?"))
                .child(
                    div()
                        .mt_3()
                        .flex()
                        .gap_2()
                        .child(
                            div()
                                .id("bil-080-modal-onay")
                                .cursor_pointer()
                                .rounded_md()
                                .bg(rgb(kabuk_vurgusu()))
                                .px_3()
                                .py_2()
                                .text_xs()
                                .text_color(rgb(crate::palet().kabuk_kart))
                                .child("Kaydet")
                                .on_click(bağlam.listener(|bu, _, _, bağlam| {
                                    bu.sergi_modali_açık = false;
                                    bağlam.notify();
                                })),
                        )
                        .child(
                            div()
                                .id("bil-080-modal-iptal")
                                .cursor_pointer()
                                .rounded_md()
                                .border_1()
                                .border_color(rgb(kenarlık()))
                                .px_3()
                                .py_2()
                                .text_xs()
                                .child("İptal")
                                .on_click(bağlam.listener(|bu, _, _, bağlam| {
                                    bu.sergi_modali_açık = false;
                                    bağlam.notify();
                                })),
                        ),
                ),
        )
    })
    .child(
        div()
            .mt_3()
            .text_xs()
            .text_color(rgb(ikincil_metin()))
            .child(format!(
                "Tür: {} · Açık: {açık}",
                match tür {
                    ModalTürü::OnayDialogu => "Onay dialogu",
                    _ => "Diğer",
                }
            )),
    )
}

fn seçici_sergisi(seçili: u8, bağlam: &mut Context<GaleriUygulaması>) -> Stateful<Div> {
    let sunum = SeçiciSunumu::Gömülü;
    let sonuçlar = ["Dosya aç", "Ayarları göster", "Yeni pencere"];
    let satırlar = sonuçlar.into_iter().enumerate().map(|(sıra, etiket)| {
        div()
            .id(format!("bil-090-sonuç-{sıra}"))
            .cursor_pointer()
            .rounded_md()
            .px_3()
            .py_2()
            .text_sm()
            .bg(rgb(if seçili == sıra as u8 {
                0xeef1ff
            } else {
                0xffffff
            }))
            .text_color(rgb(if seçili == sıra as u8 {
                kabuk_vurgusu()
            } else {
                0x111827
            }))
            .child(etiket)
            .on_click(bağlam.listener(move |bu, _, _, bağlam| {
                bu.sergi_seçici_sonucu = sıra as u8;
                bağlam.notify();
            }))
    });
    sergi_kartı(
        "bil-090-canlı-sergi",
        "Seçici",
        "Sorgu sonucu, vurgu, önizleme ve kabul",
    )
    .child(
        div()
            .mt_4()
            .rounded_md()
            .border_1()
            .border_color(rgb(kenarlık()))
            .bg(rgb(crate::palet().kabuk_zemin))
            .px_3()
            .py_2()
            .text_sm()
            .text_color(rgb(ikincil_metin()))
            .child("Komut ara…"),
    )
    .child(div().id("bil-090-sonuç-listesi").mt_2().children(satırlar))
    .child(
        div()
            .mt_2()
            .text_xs()
            .text_color(rgb(ikincil_metin()))
            .child(format!(
                "Sunum: {} · Vurgu: {}",
                match sunum {
                    SeçiciSunumu::Gömülü => "Gömülü",
                    SeçiciSunumu::Modal => "Modal",
                    SeçiciSunumu::Popover => "Popover",
                },
                sonuçlar[seçili as usize]
            )),
    )
}

fn veri_tablosu_sergisi(azalan: bool, bağlam: &mut Context<GaleriUygulaması>) -> Stateful<Div> {
    let yön = if azalan {
        SıralamaYönü::Azalan
    } else {
        SıralamaYönü::Artan
    };
    let satırlar = if azalan {
        [("Zeynep", "Aktif"), ("Mert", "Bekliyor"), ("Ayşe", "Aktif")]
    } else {
        [("Ayşe", "Aktif"), ("Mert", "Bekliyor"), ("Zeynep", "Aktif")]
    };
    sergi_kartı(
        "bil-100-canlı-sergi",
        "Veri Tablosu",
        "Sütun, satır, seçim ve sıralama",
    )
    .child(
        div()
            .id("bil-100-tablo")
            .mt_4()
            .rounded_md()
            .border_1()
            .border_color(rgb(kenarlık()))
            .overflow_hidden()
            .child(
                div()
                    .flex()
                    .bg(rgb(crate::palet().kabuk_zemin))
                    .text_xs()
                    .child(
                        div()
                            .id("bil-100-sıralama")
                            .w(px(130.))
                            .cursor_pointer()
                            .px_3()
                            .py_2()
                            .text_color(rgb(kabuk_vurgusu()))
                            .child(if azalan { "Ad ↓" } else { "Ad ↑" })
                            .on_click(bağlam.listener(|bu, _, _, bağlam| {
                                bu.sergi_tablo_azalan = !bu.sergi_tablo_azalan;
                                bağlam.notify();
                            })),
                    )
                    .child(div().px_3().py_2().child("Durum")),
            )
            .children(satırlar.into_iter().map(|(ad, durum)| {
                div()
                    .flex()
                    .border_t_1()
                    .border_color(rgb(kenarlık()))
                    .text_xs()
                    .child(div().w(px(130.)).px_3().py_2().child(ad))
                    .child(div().px_3().py_2().child(durum))
            })),
    )
    .child(
        div()
            .mt_3()
            .text_xs()
            .text_color(rgb(ikincil_metin()))
            .child(match yön {
                SıralamaYönü::Artan => "Kanonik sıralama: Artan",
                SıralamaYönü::Azalan => "Kanonik sıralama: Azalan",
            }),
    )
}

fn bildirim_sergisi(açık: bool, bağlam: &mut Context<GaleriUygulaması>) -> Stateful<Div> {
    let tür = BildirimTürü::Toast;
    sergi_kartı(
        "bil-110-canlı-sergi",
        "Bildirim",
        "Toast kuyruğu, önem, süre ve kapatma",
    )
    .child(
        div()
            .id("bil-110-bildirim-geçişi")
            .mt_4()
            .w(px(132.))
            .cursor_pointer()
            .rounded_md()
            .bg(rgb(kabuk_vurgusu()))
            .px_3()
            .py_2()
            .text_sm()
            .text_color(rgb(crate::palet().kabuk_kart))
            .child(if açık {
                "Bildirimi kapat"
            } else {
                "Bildirim göster"
            })
            .on_click(bağlam.listener(|bu, _, _, bağlam| {
                bu.sergi_bildirimi_açık = !bu.sergi_bildirimi_açık;
                bağlam.notify();
            })),
    )
    .when(açık, |kart| {
        kart.child(
            div()
                .id("bil-110-toast")
                .mt_3()
                .rounded_md()
                .border_l_4()
                .border_color(rgb(0x16a34a))
                .bg(rgb(0xf0fdf4))
                .p_3()
                .child(div().text_sm().child("Değişiklikler kaydedildi"))
                .child(
                    div()
                        .mt_1()
                        .text_xs()
                        .text_color(rgb(ikincil_metin()))
                        .child("Başarılı · Otomatik kapanır"),
                ),
        )
    })
    .child(
        div()
            .mt_3()
            .text_xs()
            .text_color(rgb(ikincil_metin()))
            .child(match tür {
                BildirimTürü::Toast => "Kanonik tür: Toast",
                _ => "Kanonik tür: Diğer",
            }),
    )
}

fn form_sergisi(gönderildi: bool, bağlam: &mut Context<GaleriUygulaması>) -> Stateful<Div> {
    let gönderim = if gönderildi {
        FormGönderimDurumu::Başarılı
    } else {
        FormGönderimDurumu::Boşta
    };
    sergi_kartı(
        "bil-120-canlı-sergi",
        "Form",
        "Alan kaydı, doğrulama ve gönderim durumu",
    )
    .child(
        div()
            .id("bil-120-form")
            .mt_4()
            .child(div().text_xs().child("E-posta"))
            .child(
                div()
                    .mt_1()
                    .rounded_md()
                    .border_1()
                    .border_color(rgb(kenarlık()))
                    .px_3()
                    .py_2()
                    .text_sm()
                    .child("ornek@uygulama.dev"),
            )
            .child(
                div()
                    .id("bil-120-gönder")
                    .mt_3()
                    .w(px(94.))
                    .cursor_pointer()
                    .rounded_md()
                    .bg(rgb(kabuk_vurgusu()))
                    .px_3()
                    .py_2()
                    .text_sm()
                    .text_color(rgb(crate::palet().kabuk_kart))
                    .child(if gönderildi {
                        "Gönderildi"
                    } else {
                        "Gönder"
                    })
                    .on_click(bağlam.listener(|bu, _, _, bağlam| {
                        bu.sergi_form_gönderildi = true;
                        bağlam.notify();
                    })),
            ),
    )
    .child(
        div()
            .mt_3()
            .text_xs()
            .text_color(rgb(if gönderildi {
                0x15803d
            } else {
                ikincil_metin()
            }))
            .child(match gönderim {
                FormGönderimDurumu::Boşta => "Gönderim: Boşta · Alan geçerli",
                FormGönderimDurumu::Başarılı => "Gönderim: Başarılı",
                _ => "Gönderim: İşleniyor",
            }),
    )
}

fn sürekli_değer_sergisi(değer: u8, bağlam: &mut Context<GaleriUygulaması>) -> Stateful<Div> {
    let kanonik = SürekliDeğer::Tek(f64::from(değer));
    let seçenekler = [20_u8, 50, 80].into_iter().map(|aday| {
        div()
            .id(format!("bil-130-değer-{aday}"))
            .cursor_pointer()
            .rounded_md()
            .border_1()
            .border_color(rgb(if değer == aday {
                kabuk_vurgusu()
            } else {
                kenarlık()
            }))
            .px_3()
            .py_2()
            .text_xs()
            .child(format!("%{aday}"))
            .on_click(bağlam.listener(move |bu, _, _, bağlam| {
                bu.sergi_sürekli_değer = aday;
                bağlam.notify();
            }))
    });
    sergi_kartı(
        "bil-130-canlı-sergi",
        "Slider ve Aralık",
        "Adım, tutamaç, önizleme ve kabul",
    )
    .child(
        div()
            .id("bil-130-iz")
            .mt_5()
            .h(px(8.))
            .w(px(220.))
            .rounded_full()
            .bg(rgb(crate::palet().kabuk_kenarlık))
            .child(
                div()
                    .h_full()
                    .w(px(f32::from(değer) * 2.2))
                    .rounded_full()
                    .bg(rgb(kabuk_vurgusu())),
            ),
    )
    .child(div().mt_4().flex().gap_2().children(seçenekler))
    .child(
        div()
            .mt_3()
            .text_xs()
            .text_color(rgb(ikincil_metin()))
            .child(match kanonik {
                SürekliDeğer::Tek(v) => format!("Kanonik tek değer: {v:.0}"),
                SürekliDeğer::Aralık { .. } => "Kanonik aralık".to_owned(),
            }),
    )
}

fn ilerleme_sergisi(ilerleme: u8, bağlam: &mut Context<GaleriUygulaması>) -> Stateful<Div> {
    let değer = İlerlemeDeğeri::Belirli {
        tamamlanan: f64::from(ilerleme),
        toplam: 100.0,
    };
    let oran = değer.oran().unwrap_or_default();
    sergi_kartı(
        "bil-140-canlı-sergi",
        "Durum ve İlerleme",
        "Banner, belirli ilerleme ve tamamlanma",
    )
    .child(
        div()
            .id("bil-140-banner")
            .mt_4()
            .rounded_md()
            .bg(rgb(crate::palet().kabuk_seçili_zemin))
            .p_3()
            .text_sm()
            .text_color(rgb(crate::palet().kabuk_vurgu))
            .child(if ilerleme == 100 {
                "İşlem tamamlandı"
            } else {
                "Dosyalar işleniyor"
            }),
    )
    .child(
        div()
            .mt_3()
            .h(px(8.))
            .w(px(220.))
            .rounded_full()
            .bg(rgb(crate::palet().kabuk_kenarlık))
            .child(
                div()
                    .h_full()
                    .w(px((oran as f32) * 220.0))
                    .rounded_full()
                    .bg(rgb(if ilerleme == 100 {
                        0x16a34a
                    } else {
                        kabuk_vurgusu()
                    })),
            ),
    )
    .child(
        div()
            .id("bil-140-ilerlet")
            .mt_3()
            .w(px(116.))
            .cursor_pointer()
            .rounded_md()
            .border_1()
            .border_color(rgb(kenarlık()))
            .px_3()
            .py_2()
            .text_sm()
            .child(if ilerleme == 100 {
                "Başa dön"
            } else {
                "İlerlet"
            })
            .on_click(bağlam.listener(|bu, _, _, bağlam| {
                bu.sergi_ilerleme = if bu.sergi_ilerleme >= 100 {
                    0
                } else {
                    bu.sergi_ilerleme.saturating_add(20).min(100)
                };
                bağlam.notify();
            })),
    )
    .child(
        div()
            .mt_3()
            .text_xs()
            .text_color(rgb(ikincil_metin()))
            .child(format!("Belirli ilerleme: %{ilerleme}")),
    )
}

fn takvim_sergisi(gün: u8, bağlam: &mut Context<GaleriUygulaması>) -> Stateful<Div> {
    let kaynak = TakvimEtkileşimKaynağı::İşaretçi;
    let günler = (10_u8..=16).map(|aday| {
        div()
            .id(format!("bil-150-gün-{aday}"))
            .size(px(30.))
            .flex()
            .items_center()
            .justify_center()
            .cursor_pointer()
            .rounded_md()
            .bg(rgb(if gün == aday {
                kabuk_vurgusu()
            } else {
                0xffffff
            }))
            .text_xs()
            .text_color(rgb(if gün == aday { 0xffffff } else { 0x111827 }))
            .child(aday.to_string())
            .on_click(bağlam.listener(move |bu, _, _, bağlam| {
                bu.sergi_takvim_günü = aday;
                bağlam.notify();
            }))
    });
    sergi_kartı(
        "bil-150-canlı-sergi",
        "Takvim ve Tarih Seçimi",
        "Ay ızgarası, tarih seçimi ve klavye modeli",
    )
    .child(div().mt_4().text_sm().child("Ağustos 2026"))
    .child(
        div()
            .id("bil-150-takvim")
            .mt_3()
            .flex()
            .gap_1()
            .children(günler),
    )
    .child(
        div()
            .mt_3()
            .text_xs()
            .text_color(rgb(ikincil_metin()))
            .child(format!(
                "Seçili: 2026-08-{gün:02} · {}",
                match kaynak {
                    TakvimEtkileşimKaynağı::İşaretçi => "İşaretçi",
                    TakvimEtkileşimKaynağı::Klavye => "Klavye",
                    TakvimEtkileşimKaynağı::ErişilebilirEylem => "Erişilebilir eylem",
                }
            )),
    )
}

fn disclosure_sergisi(açık: bool, bağlam: &mut Context<GaleriUygulaması>) -> Stateful<Div> {
    let tetikleyici = DisclosureTetikleyicisi::TümBaşlık;
    sergi_kartı(
        "bil-160-canlı-sergi",
        "Yapısal Sunum",
        "Kart, ayraç, bölüm ve disclosure",
    )
    .child(
        div()
            .id("bil-160-disclosure")
            .mt_4()
            .cursor_pointer()
            .rounded_md()
            .border_1()
            .border_color(rgb(kenarlık()))
            .child(
                div()
                    .flex()
                    .justify_between()
                    .bg(rgb(crate::palet().kabuk_zemin))
                    .px_3()
                    .py_2()
                    .text_sm()
                    .child("Gelişmiş seçenekler")
                    .child(if açık { "Kapat" } else { "Aç" }),
            )
            .when(açık, |bölüm| {
                bölüm.child(
                    div()
                        .border_t_1()
                        .border_color(rgb(kenarlık()))
                        .p_3()
                        .text_xs()
                        .child("Önbellek · Günlükleme · Deneysel API"),
                )
            })
            .on_click(bağlam.listener(|bu, _, _, bağlam| {
                bu.sergi_disclosure_açık = !bu.sergi_disclosure_açık;
                bağlam.notify();
            })),
    )
    .child(
        div()
            .mt_3()
            .text_xs()
            .text_color(rgb(ikincil_metin()))
            .child(match tetikleyici {
                DisclosureTetikleyicisi::TümBaşlık => "Tetikleyici: Tüm başlık",
                _ => "Tetikleyici: Gösterge",
            }),
    )
}

fn renk_sergisi(seçili: u8, bağlam: &mut Context<GaleriUygulaması>) -> Stateful<Div> {
    let yüzey = RenkYüzeyi::Kütüphaneİçi;
    let renkler = [
        ("İndigo", Rgba8([48, 70, 184, 255]), 0x3046b8),
        ("Yeşil", Rgba8([22, 163, 74, 255]), 0x16a34a),
        ("Turuncu", Rgba8([234, 88, 12, 255]), 0xea580c),
    ];
    let palet = renkler
        .into_iter()
        .enumerate()
        .map(|(sıra, (ad, _, renk))| {
            div()
                .id(format!("bil-170-renk-{sıra}"))
                .size(px(54.))
                .flex()
                .items_center()
                .justify_center()
                .cursor_pointer()
                .rounded_md()
                .border_2()
                .border_color(rgb(if seçili == sıra as u8 {
                    0x111827
                } else {
                    0xffffff
                }))
                .bg(rgb(renk))
                .text_xs()
                .text_color(rgb(crate::palet().kabuk_kart))
                .child(ad)
                .on_click(bağlam.listener(move |bu, _, _, bağlam| {
                    bu.sergi_renk_seçimi = sıra as u8;
                    bağlam.notify();
                }))
        });
    let (ad, rgba, _) = renkler[seçili as usize];
    sergi_kartı(
        "bil-170-canlı-sergi",
        "Renk Seçici",
        "Palet, renk önizleme ve kabul",
    )
    .child(
        div()
            .id("bil-170-palet")
            .mt_4()
            .flex()
            .gap_3()
            .children(palet),
    )
    .child(
        div()
            .mt_3()
            .text_xs()
            .text_color(rgb(ikincil_metin()))
            .child(format!(
                "{} · rgba({}, {}, {}, {}) · {}",
                ad,
                rgba.0[0],
                rgba.0[1],
                rgba.0[2],
                rgba.0[3],
                match yüzey {
                    RenkYüzeyi::Kütüphaneİçi => "Kütüphane içi",
                    RenkYüzeyi::PlatformPaneli => "Platform paneli",
                }
            )),
    )
}

fn aktarım_sergisi(ilerleme: u8, bağlam: &mut Context<GaleriUygulaması>) -> Stateful<Div> {
    let yön = DosyaAktarımYönü::İçe;
    let durum = Aktarımİlerlemesi::Belirli {
        aktarılan: u64::from(ilerleme),
        toplam: 100,
    };
    sergi_kartı(
        "bil-180-canlı-sergi",
        "Dosya Aktarımı",
        "Kuyruk, ilerleme, iptal ve yeniden deneme",
    )
    .child(
        div()
            .id("bil-180-aktarım")
            .mt_4()
            .rounded_md()
            .border_1()
            .border_color(rgb(kenarlık()))
            .p_3()
            .child(div().text_sm().child("rapor.pdf"))
            .child(
                div()
                    .mt_2()
                    .h(px(8.))
                    .w(px(220.))
                    .rounded_full()
                    .bg(rgb(crate::palet().kabuk_kenarlık))
                    .child(
                        div()
                            .h_full()
                            .w(px(f32::from(ilerleme) * 2.2))
                            .rounded_full()
                            .bg(rgb(kabuk_vurgusu())),
                    ),
            )
            .child(
                div()
                    .id("bil-180-ilerlet")
                    .mt_3()
                    .cursor_pointer()
                    .text_xs()
                    .text_color(rgb(kabuk_vurgusu()))
                    .child(if ilerleme == 100 {
                        "Yeniden başlat"
                    } else {
                        "İlerlet"
                    })
                    .on_click(bağlam.listener(|bu, _, _, bağlam| {
                        bu.sergi_aktarım = if bu.sergi_aktarım >= 100 {
                            0
                        } else {
                            bu.sergi_aktarım.saturating_add(25).min(100)
                        };
                        bağlam.notify();
                    })),
            ),
    )
    .child(
        div()
            .mt_3()
            .text_xs()
            .text_color(rgb(ikincil_metin()))
            .child(format!(
                "Yön: {} · Durum: {}%",
                match yön {
                    DosyaAktarımYönü::İçe => "İçe",
                    DosyaAktarımYönü::Dışa => "Dışa",
                },
                match durum {
                    Aktarımİlerlemesi::Belirli { aktarılan, .. } => aktarılan,
                    Aktarımİlerlemesi::Belirsiz => 0,
                }
            )),
    )
}

fn arama_sergisi(etkin: u8, bağlam: &mut Context<GaleriUygulaması>) -> Stateful<Div> {
    let kaynak = VurguKaynağı::AramaOturumuBil190;
    sergi_kartı(
        "bil-190-canlı-sergi",
        "Belge İçi Arama",
        "Arama oturumu, eşleşme ve gezinme",
    )
    .child(
        div()
            .id("bil-190-arama")
            .mt_4()
            .rounded_md()
            .border_1()
            .border_color(rgb(kenarlık()))
            .px_3()
            .py_2()
            .text_sm()
            .child("bileşen")
            .child(
                div()
                    .mt_2()
                    .text_xs()
                    .text_color(rgb(ikincil_metin()))
                    .child(format!("{} / 3", etkin + 1)),
            ),
    )
    .child(
        div()
            .id("bil-190-sonraki")
            .mt_3()
            .w(px(132.))
            .cursor_pointer()
            .rounded_md()
            .border_1()
            .border_color(rgb(kenarlık()))
            .px_3()
            .py_2()
            .text_sm()
            .child("Sonraki eşleşme")
            .on_click(bağlam.listener(|bu, _, _, bağlam| {
                bu.sergi_arama_eşleşmesi = (bu.sergi_arama_eşleşmesi + 1) % 3;
                bağlam.notify();
            })),
    )
    .child(
        div()
            .mt_3()
            .text_xs()
            .text_color(rgb(ikincil_metin()))
            .child(match kaynak {
                VurguKaynağı::AramaOturumuBil190 => "Vurgu sahibi: Arama oturumu",
                VurguKaynağı::SüzmeAçıklamasıBil020 => "Vurgu sahibi: Süzme",
            }),
    )
}

fn kısayol_sergisi(
    değiştirildi: bool, bağlam: &mut Context<GaleriUygulaması>
) -> Stateful<Div> {
    let yakalanabilir = tuşu_yakala(YakalamaBağlamı {
        odaklı: true,
        görünür: true,
        yaşayan: true,
        ime: false,
        modal_kapsam_içi: true,
    });
    let çakışma_çözüldü = çakışmayı_çöz(
        KısayolÇakışması {
            ayrılmış_sistem: false,
            erişilebilirlik: false,
        },
        Some(KısayolÇakışmaKararı::Değiştir),
    )
    .is_ok();
    sergi_kartı(
        "bil-200-canlı-sergi",
        "Kısayol Düzenleyici",
        "Odaklı yakalama, çakışma kararı ve kabul",
    )
    .child(
        div()
            .id("bil-200-kısayol")
            .mt_4()
            .flex()
            .items_center()
            .justify_between()
            .rounded_md()
            .border_1()
            .border_color(rgb(kenarlık()))
            .px_3()
            .py_3()
            .child(div().text_sm().child("Hızlı aç"))
            .child(
                div()
                    .rounded_md()
                    .bg(rgb(crate::palet().kabuk_zemin))
                    .px_3()
                    .py_1()
                    .text_sm()
                    .child(if değiştirildi {
                        "Ctrl + Shift + K"
                    } else {
                        "Ctrl + K"
                    }),
            ),
    )
    .child(
        div()
            .id("bil-200-yakala")
            .mt_3()
            .cursor_pointer()
            .rounded_md()
            .border_1()
            .border_color(rgb(kabuk_vurgusu()))
            .px_3()
            .py_2()
            .text_sm()
            .text_color(rgb(kabuk_vurgusu()))
            .child(if değiştirildi {
                "Varsayılana dön"
            } else {
                "Yeni kısayolu yakala"
            })
            .on_click(bağlam.listener(|bu, _, _, bağlam| {
                bu.sergi_kısayol_değiştirildi = !bu.sergi_kısayol_değiştirildi;
                bağlam.notify();
            })),
    )
    .child(
        div()
            .mt_3()
            .text_xs()
            .text_color(rgb(ikincil_metin()))
            .child(format!(
                "Yakalama: {} · Çakışma kararı: {}",
                if yakalanabilir { "Hazır" } else { "Kapalı" },
                if çakışma_çözüldü {
                    "Geçerli"
                } else {
                    "Gerekli"
                }
            )),
    )
}

fn ayar_sergisi(koyu: bool, bağlam: &mut Context<GaleriUygulaması>) -> Stateful<Div> {
    let düzenlenebilir = yönetilen_ayar_sunumu(false);
    let yönetilen = yönetilen_ayar_sunumu(true);
    sergi_kartı(
        "bil-210-canlı-sergi",
        "Ayar Kataloğu",
        "Arama, düzenleyici türü ve yönetilen durum",
    )
    .child(
        div()
            .id("bil-210-arama")
            .mt_4()
            .rounded_md()
            .border_1()
            .border_color(rgb(kenarlık()))
            .px_3()
            .py_2()
            .text_sm()
            .text_color(rgb(ikincil_metin()))
            .child("Ayarlarda ara: tema"),
    )
    .child(
        div()
            .id("bil-210-tema")
            .mt_3()
            .flex()
            .items_center()
            .justify_between()
            .cursor_pointer()
            .rounded_md()
            .border_1()
            .border_color(rgb(kenarlık()))
            .p_3()
            .child(div().text_sm().child("Görünüm teması"))
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(kabuk_vurgusu()))
                    .child(if koyu { "Koyu" } else { "Açık" }),
            )
            .on_click(bağlam.listener(|bu, _, _, bağlam| {
                bu.sergi_ayar_koyu = !bu.sergi_ayar_koyu;
                bağlam.notify();
            })),
    )
    .child(
        div()
            .mt_2()
            .rounded_md()
            .bg(rgb(crate::palet().kabuk_zemin))
            .p_3()
            .text_xs()
            .text_color(rgb(ikincil_metin()))
            .child(format!(
                "Düzenleyici: {} · Kurumsal ayar: {}",
                if düzenlenebilir.etkin_düzenleyici {
                    "Etkin"
                } else {
                    "Salt okunur"
                },
                if yönetilen.salt_okunur {
                    "Salt okunur"
                } else {
                    "Etkin"
                }
            )),
    )
}

fn bağlantı_sergisi(
    başarılı: bool, bağlam: &mut Context<GaleriUygulaması>
) -> Stateful<Div> {
    let eylemler = bağlantı_eylemleri(BağlantıYetenekleri {
        sürücü: true,
        kalıcı_kasa: false,
    });
    sergi_kartı(
        "bil-220-canlı-sergi",
        "Veri Kaynağı Bağlantısı",
        "Profil, capability ve güvenli bağlantı testi",
    )
    .child(
        div()
            .id("bil-220-profil")
            .mt_4()
            .rounded_md()
            .border_1()
            .border_color(rgb(kenarlık()))
            .p_3()
            .child(div().text_sm().child("Yerel PostgreSQL"))
            .child(
                div()
                    .mt_1()
                    .text_xs()
                    .text_color(rgb(ikincil_metin()))
                    .child("localhost:5432 · Gizli değer: ••••••••"),
            ),
    )
    .child(
        div()
            .id("bil-220-test")
            .mt_3()
            .cursor_pointer()
            .rounded_md()
            .bg(rgb(if başarılı {
                0x15803d
            } else {
                kabuk_vurgusu()
            }))
            .px_3()
            .py_2()
            .text_sm()
            .text_color(rgb(crate::palet().kabuk_kart))
            .child(if başarılı {
                "Bağlantı başarılı"
            } else {
                "Bağlantıyı test et"
            })
            .on_click(bağlam.listener(|bu, _, _, bağlam| {
                bu.sergi_bağlantı_başarılı = true;
                bağlam.notify();
            })),
    )
    .child(
        div()
            .mt_3()
            .text_xs()
            .text_color(rgb(ikincil_metin()))
            .child(format!(
                "Test capability: {} · Kalıcı kasa: {}",
                if eylemler.test { "Var" } else { "Yok" },
                if eylemler.gizli_kayıt { "Var" } else { "Yok" }
            )),
    )
}

fn kod_sergisi(satır: u8, bağlam: &mut Context<GaleriUygulaması>) -> Stateful<Div> {
    let vurgu = sözdizimi_çöz("keyword", &["keyword", "string"]);
    let satırlar = [
        (1_u8, "fn ana() {"),
        (2_u8, "    let durum = \"hazır\";"),
        (3_u8, "}"),
    ]
    .into_iter()
    .map(|(numara, metin)| {
        div()
            .id(format!("bil-230-satır-{numara}"))
            .flex()
            .gap_3()
            .cursor_pointer()
            .rounded_md()
            .bg(rgb(if satır == numara { 0xeef1ff } else { 0x111827 }))
            .px_2()
            .py_1()
            .text_xs()
            .text_color(rgb(if satır == numara { 0x1d4ed8 } else { 0xe5e7eb }))
            .child(div().w(px(18.)).child(numara.to_string()))
            .child(metin)
            .on_click(bağlam.listener(move |bu, _, _, bağlam| {
                bu.sergi_kod_satırı = numara;
                bağlam.notify();
            }))
    });
    sergi_kartı(
        "bil-230-canlı-sergi",
        "Kod ve Sözdizimi Görünümü",
        "Salt okunur satırlar, seçim ve tema rolü",
    )
    .child(
        div()
            .id("bil-230-kod")
            .mt_4()
            .rounded_md()
            .bg(rgb(crate::palet().kabuk_ana_metin))
            .p_2()
            .children(satırlar),
    )
    .child(
        div()
            .mt_3()
            .text_xs()
            .text_color(rgb(ikincil_metin()))
            .child(format!(
                "Seçili satır: {satır} · Sözdizimi: {}",
                match vurgu {
                    SözdizimiVurgusu::TemaRolü(_) => "Tema rolü",
                    SözdizimiVurgusu::GüvenliÖnPlan => "Güvenli ön plan",
                }
            )),
    )
}

fn yüzen_eylem_sergisi(açık: bool, bağlam: &mut Context<GaleriUygulaması>) -> Stateful<Div> {
    let grup = YüzenGrupDurumu::yeni(açık);
    sergi_kartı(
        "bil-250-canlı-sergi",
        "Yüzen Eylem Düğmesi",
        "Mantıksal köşe, görünürlük ve açılır eylem grubu",
    )
    .child(
        div()
            .id("bil-250-yüzey")
            .mt_4()
            .h(px(118.))
            .flex()
            .items_end()
            .justify_end()
            .gap_2()
            .rounded_md()
            .bg(rgb(crate::palet().kabuk_zemin))
            .p_3()
            .when(açık, |yüzey| {
                yüzey
                    .child(
                        div()
                            .size(px(40.))
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded_full()
                            .bg(rgb(crate::palet().kabuk_kart))
                            .text_sm()
                            .text_color(rgb(kabuk_vurgusu()))
                            .child("Yükle"),
                    )
                    .child(
                        div()
                            .size(px(40.))
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded_full()
                            .bg(rgb(crate::palet().kabuk_kart))
                            .text_sm()
                            .text_color(rgb(kabuk_vurgusu()))
                            .child("Yeni"),
                    )
            })
            .child(
                div()
                    .id("bil-250-geçiş")
                    .size(px(48.))
                    .flex()
                    .items_center()
                    .justify_center()
                    .cursor_pointer()
                    .rounded_full()
                    .bg(rgb(kabuk_vurgusu()))
                    .text_lg()
                    .text_color(rgb(crate::palet().kabuk_kart))
                    .child(if açık { "×" } else { "+" })
                    .on_click(bağlam.listener(|bu, _, _, bağlam| {
                        bu.sergi_yüzen_grup_açık = !bu.sergi_yüzen_grup_açık;
                        bağlam.notify();
                    })),
            ),
    )
    .child(
        div()
            .mt_3()
            .text_xs()
            .text_color(rgb(ikincil_metin()))
            .child(format!(
                "Grup: {} · Üyeler hedef: {}",
                if grup.açık { "Açık" } else { "Kapalı" },
                if grup.üyeler_hedef { "Evet" } else { "Hayır" }
            )),
    )
}

fn gezinme_sergisi(etkin: u8, bağlam: &mut Context<GaleriUygulaması>) -> Stateful<Div> {
    let etiketler = ["Genel", "Ekip", "Güvenlik"];
    let öğeler: Vec<_> = etiketler
        .iter()
        .enumerate()
        .map(|(sıra, etiket)| GezinmeÖğesi {
            kimlik: GezinmeHedefiKimliği::yeni(format!("hedef-{sıra}"))
                .expect("galeri gezinme kimliği geçerlidir"),
            etiket: (*etiket).into(),
            simge: None,
            etkin: sıra == etkin as usize,
            görünür: true,
        })
        .collect();
    let sunum = gezinme_sunumu(&öğeler, Some(&öğeler[etkin as usize].kimlik))
        .expect("galeri gezinme hedefleri benzersizdir");
    let yönelim = GezinmeYönelimi::Yatay;
    let hedefler = etiketler.into_iter().enumerate().map(|(sıra, etiket)| {
        div()
            .id(format!("bil-260-hedef-{sıra}"))
            .cursor_pointer()
            .border_b_2()
            .border_color(rgb(if etkin as usize == sıra {
                kabuk_vurgusu()
            } else {
                0xffffff
            }))
            .px_3()
            .py_2()
            .text_sm()
            .text_color(rgb(if etkin as usize == sıra {
                kabuk_vurgusu()
            } else {
                ikincil_metin()
            }))
            .child(etiket)
            .on_click(bağlam.listener(move |bu, _, _, bağlam| {
                bu.sergi_gezinme_hedefi = sıra as u8;
                bağlam.notify();
            }))
    });
    sergi_kartı(
        "bil-260-canlı-sergi",
        "Gezinme",
        "Etkin hedef, yönelim ve kararlı rota niyeti",
    )
    .child(
        div()
            .id("bil-260-gezinme")
            .mt_4()
            .flex()
            .border_b_1()
            .border_color(rgb(kenarlık()))
            .children(hedefler),
    )
    .child(
        div()
            .mt_4()
            .rounded_md()
            .bg(rgb(crate::palet().kabuk_zemin))
            .p_3()
            .text_sm()
            .child(format!("{} ayarları", etiketler[etkin as usize])),
    )
    .child(
        div()
            .mt_3()
            .text_xs()
            .text_color(rgb(ikincil_metin()))
            .child(format!(
                "Yönelim: {} · Etkin hedef: {}",
                match yönelim {
                    GezinmeYönelimi::Yatay => "Yatay",
                    GezinmeYönelimi::Dikey => "Dikey",
                },
                sunum
                    .etkin
                    .as_ref()
                    .map_or("Yok", GezinmeHedefiKimliği::metin)
            )),
    )
}

fn görsel_sergisi(konum: u8, bağlam: &mut Context<GaleriUygulaması>) -> Stateful<Div> {
    let renkler = [0x4f46e5, 0x0891b2, 0xea580c];
    let adlar = ["Dağ manzarası", "Deniz kıyısı", "Gün batımı"];
    let (gösterge, değişmezler) = görsel_konum_göstergesi(konum as usize + 1, 3);
    sergi_kartı(
        "bil-270-canlı-sergi",
        "Görsel Sunum",
        "Anlamlı görsel, önizleme ve dizi konumu",
    )
    .child(
        div()
            .id("bil-270-görsel")
            .mt_4()
            .h(px(118.))
            .flex()
            .items_center()
            .justify_center()
            .rounded_lg()
            .bg(rgb(renkler[konum as usize]))
            .text_lg()
            .text_color(rgb(crate::palet().kabuk_kart))
            .child(adlar[konum as usize]),
    )
    .child(
        div()
            .mt_3()
            .flex()
            .items_center()
            .justify_between()
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(ikincil_metin()))
                    .child(format!("{} / {}", gösterge.konum, gösterge.toplam)),
            )
            .child(
                div()
                    .id("bil-270-sonraki")
                    .cursor_pointer()
                    .rounded_md()
                    .border_1()
                    .border_color(rgb(kenarlık()))
                    .px_3()
                    .py_2()
                    .text_sm()
                    .child("Sonraki")
                    .on_click(bağlam.listener(|bu, _, _, bağlam| {
                        bu.sergi_görsel_konumu = (bu.sergi_görsel_konumu + 1) % 3;
                        bağlam.notify();
                    })),
            ),
    )
    .child(
        div()
            .mt_2()
            .text_xs()
            .text_color(rgb(ikincil_metin()))
            .child(format!(
                "Erişilebilir hedef: {} · Klavye eşlemesi: {}",
                gösterge.hedef, değişmezler.klavye_eşlemesi
            )),
    )
}

fn kod_sembolü_sergisi(qr: bool, bağlam: &mut Context<GaleriUygulaması>) -> Stateful<Div> {
    let veri = if qr { "GPUI" } else { "12345678" };
    let doğrulandı = kodu_doğrula(veri, 32, true).is_ok();
    let modüller = (0_usize..49).map(|sıra| {
        let dolu = if qr {
            sıra % 3 == 0 || sıra % 7 == 0 || sıra / 7 == 6
        } else {
            sıra % 2 == 0
        };
        div()
            .size(px(14.))
            .bg(rgb(if dolu { 0x111827 } else { 0xffffff }))
    });
    sergi_kartı(
        "bil-280-canlı-sergi",
        "Kod Sembolü",
        "Ön doğrulama, sessiz alan ve yüksek karşıtlık",
    )
    .child(
        div()
            .id("bil-280-sembol")
            .mt_4()
            .w(px(114.))
            .flex()
            .flex_wrap()
            .gap(px(2.))
            .border_1()
            .border_color(rgb(kenarlık()))
            .bg(rgb(crate::palet().kabuk_kart))
            .p_2()
            .children(modüller),
    )
    .child(
        div()
            .id("bil-280-tür")
            .mt_3()
            .cursor_pointer()
            .rounded_md()
            .border_1()
            .border_color(rgb(kenarlık()))
            .px_3()
            .py_2()
            .text_sm()
            .child(if qr {
                "Çubuk koda geç"
            } else {
                "QR koda geç"
            })
            .on_click(bağlam.listener(|bu, _, _, bağlam| {
                bu.sergi_kod_sembolü_qr = !bu.sergi_kod_sembolü_qr;
                bağlam.notify();
            })),
    )
    .child(
        div()
            .mt_3()
            .text_xs()
            .text_color(rgb(ikincil_metin()))
            .child(format!(
                "Tür: {} · Ön doğrulama: {}",
                if qr { "QR" } else { "Çubuk" },
                if doğrulandı { "Geçti" } else { "Reddedildi" }
            )),
    )
}

fn medya_sergisi(niyet: bool, bağlam: &mut Context<GaleriUygulaması>) -> Stateful<Div> {
    let teslim = oynatma_niyetini_teslim_et(MedyaAnlıkGörüntüsü {
        durum: MedyaDurumu::Hazır,
        bağlam_sürümü: 1,
    });
    let denetimler = medya_denetim_bağdaştırıcıları();
    sergi_kartı(
        "bil-290-canlı-sergi",
        "Medya Oynatma",
        "Poster, oynatma niyeti ve dürüst capability fallback'i",
    )
    .child(
        div()
            .id("bil-290-poster")
            .mt_4()
            .h(px(116.))
            .flex()
            .items_center()
            .justify_center()
            .rounded_lg()
            .bg(rgb(crate::palet().kabuk_ana_metin))
            .text_color(rgb(crate::palet().kabuk_kart))
            .child("GPUI medya posteri"),
    )
    .child(
        div()
            .id("bil-290-oynat")
            .mt_3()
            .cursor_pointer()
            .rounded_md()
            .bg(rgb(kabuk_vurgusu()))
            .px_3()
            .py_2()
            .text_sm()
            .text_color(rgb(crate::palet().kabuk_kart))
            .child(if niyet {
                "Port sonucu bekleniyor"
            } else {
                "Oynatmayı iste"
            })
            .on_click(bağlam.listener(|bu, _, _, bağlam| {
                bu.sergi_medya_niyeti = !bu.sergi_medya_niyeti;
                bağlam.notify();
            })),
    )
    .child(
        div()
            .mt_3()
            .text_xs()
            .text_color(rgb(ikincil_metin()))
            .child(format!(
                "Port bekleniyor: {} · Oynat denetimi: BİL-040={} · Yerel oynatma iddiası: Yok",
                teslim.port_sonucu_bekleniyor, denetimler.oynat_bil040
            )),
    )
}

#[cfg(test)]
mod testler {
    /// Açılır seçicilerin kimlikleri benzersiz olmalı.
    ///
    /// `açık_seçici` tek bir kimlik tutar ve her seçici `seçici_açık_mı`
    /// ile kendi kimliğini sorar: aynı kimliği paylaşan seçiciler birlikte
    /// açılır. Beş satır (`imleç`, `varsayılan`, `bölüm`, `doldurma`,
    /// `saat dilimi`) kopyala-yapıştır yüzünden `"imleç"` kimliğini
    /// paylaşıyordu. Dördü, sıfır yüksekliğe sıkışan akış bloklarında
    /// olduğu için çakışma ekranda hiç görünmedi — bölümler çizilir
    /// çizilmez iki panel birden açıldı.
    #[test]
    fn şerit_seçicisi_kimlikleri_benzersizdir() {
        let kaynak = include_str!("sergiler.rs");
        let mut görülen = std::collections::BTreeMap::<&str, usize>::new();
        for parça in kaynak.split("şerit_seçicisi(\"").skip(1) {
            let kimlik = parça.split('"').next().expect("kimlik kapanır");
            *görülen.entry(kimlik).or_default() += 1;
        }
        let yinelenen: Vec<_> = görülen
            .iter()
            .filter(|(_, sayı)| **sayı > 1)
            .map(|(kimlik, sayı)| format!("{kimlik} × {sayı}"))
            .collect();
        assert!(
            yinelenen.is_empty(),
            "aynı kimliği paylaşan seçiciler birlikte açılır: {}",
            yinelenen.join(", ")
        );
    }
}

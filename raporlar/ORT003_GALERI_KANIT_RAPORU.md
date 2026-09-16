# ORT-003 galeri kanıt raporu

> Tarih: 16 Eylül 2026
>
> Galeri kaynak commit'i: `4746e3708f4c2d343eb96410eb8b98e82ebabee6` üzerine
> bu turda eklenen `Tab` dolaşımı ve iki çalışma zamanı testi; ölçülen ağaç
> bu belgeyi taşıyan commit'tir.
>
> Exact çekirdek bağı: `48aa5cc10f4aca1701db049f883844035fb8f0c9`
>
> (Önceki kayıtlar: ilk koşum `ee4357d`, sonra `e0beaab`, sonra `d73add3`.
> Bu kesit çekirdeğin `BitişikBölüt` kanonikleşmesini, bitişik eylem
> bölütünün gönderim hattına bağlanmasını ve ORT-003'ün K11 makbuz
> zincirini tüketir.)
>
> Galeri başlangıç tabanı: `13e350931f7e9985d9140fc23961718071c8c7f2`
>
> Ortam: Apple Silicon `arm64`, macOS `26.6.2`, `rustc 1.97.1`
>
> **Yeniden koşum kaydı.** Bütün galeri koşumu, pushlanmış çekirdek
> `48aa5cc` üzerinde çalıştırıldı. Bağımlılık yolunun ölçülen çekirdeğe
> çözüldüğü ayrıca doğrulandı: `../gpui_bilesenleri` →
> `/Volumes/Taslaklar/tmp/ort003-20260915/core-teslim`, `HEAD = 48aa5cc`.
> Bu kesitte bitişik eylemin **yaşam döngüsü** galeri penceresinde kapandı:
> basış sırasında devre dışı bırakma/yeniden etkinleştirme, bölütü kaldırıp
> aynı içerikle geri ekleme, odaklı bölütün devre dışı bırakılması ve
> kaldırılması, gerçek Tab/Shift-Tab ile Enter, eski etkileşimin gönderim
> üretmemesi ve yeni tam etkileşimin çalışması.

Bu rapor `ORT-003 1.6.0` kutu şekli sağlayıcısının galeri tüketicisini ve o
tüketicinin koşum kanıtlarını kaydeder. Sözleşmelerin sahibi
`gpui_bilesenleri` deposudur; galeri yukarıdaki exact çekirdek commit'ini
tüketir. Karar onayı runtime kanıtı sayılmaz.

## 1. Fiziksel tüketici zinciri

### Şekil tercihi ve köşe metrikleri

- Tezgâh görünüm profili köşe metriklerini `KöşeMetrikleri::denetimli`
  kapısından kurar. Ters (`köşeli > yuvarlatılmış`), negatif veya sonlu
  olmayan tema çifti artık sessizce geçmez.
- Tercih çözümü `ÇözülmüşKutuŞekli`den okunur: galeri kademe başına ikinci
  bir çözüm kuralı tutmaz. `Hap` kademesinde tezgâh **kendi kromunu** GPUI
  `rounded()` ile çizdiği için kısa kenara kırpılacak büyük bir değer
  kullanıldığı kodda açıkça yazılıdır; bileşenlerin etkin yarıçapı gerçek
  sınırlar ve ölçek yakalandıktan sonra sağlayıcıda çözülür.
- Tezgâhın köşe pikseli girdisi `KutuYarıçapı::denetimli` kapısından geçer.
  Geçersiz girdi tercihe hiç girmez; yapılandırma `GörünümProfilinden`
  kademesine düşer.

### Görünür geometri gözlemi

- Alan durum panelinde üç satır vardır: kabuk geometrisi (dış/iç ölçü, dış
  yarıçap, yakalanmış ölçek), kuşak geometrisi (mantıksal bölüt sınırları ve
  paylaşılan ayırıcıların tek çizicileri) ve geometri hatası.
- Panel metni doğrulanmış geometriden türer; ayrı bir model tutulmaz.
  `YÖN-006` gereği etiketlerde sözleşme numarası geçmez.

## 2. Koşum kanıtları

| Koşum | Sonuç |
|---|---|
| `cargo test --workspace` (galeri) | 270 geçti, 0 kaldı |
| `rustfmt --edition 2024 --check` | fark yok |
| `cargo clippy --workspace --all-targets` | değişen dosyalarda yeni bulgu yok |

### Çalışma zamanı kanıtı

`crates/gpui-bilesenleri-galeri/tests/render_kosumu.rs::`
`ort003_tezgah_cizimde_dogrulanmis_geometri_ve_kusak_uretir` gerçek GPUI
penceresinde çizim yaptırır ve şunları exact doğrular:

1. Tek bölütte sahiplik **tek kabuktadır**: kuşak tutamağı boştur, iç kabuk
   sınır şeridini dışarıda bırakır, hata yoktur.
2. Bitişik bölüt açılınca sahiplik **kuşağa geçer**: tek kabuk tutamağı
   boşalır — iki sahip aynı anda bulunmaz.
3. Üç bölütlü kuşak dış kabuğu boşluksuz döşer; her bölütün içerik bölgesi
   çizilen kromun dışında ve çökmemiş kalır.
4. İki paylaşılan sınırın çizicileri ayrıdır: aynı kenarı iki bölüt çizmez.
5. Panelde görünen özet metni aynı geometriden üretilir.

## 3. `BİL-010 §23.2` bitişik eylem bölütü tüketicisi

- Tezgâhın bölüt ekseni kanonik `BitişikBölüt` enumunu üretir: sabit bölüt
  `Sabitİçerik`, eylem bölütü `BitişikEylemBölütü` (`AramayıBaşlat`
  niyeti). Kod paneli de bu kanonik şekli yazar.
- Sözleşmede karşılığı olmayan iki tezgâh ekseni kaldırıldı: bölüt
  **kademeli görünürlüğe girmez** (her zaman tam opak) ve iç ayırıcıyı
  `ORT-003 §13` koşulsuz çizer.
- Olay akışı paneli `AramaGönderildi` olayını kaynak ve değer sürümüyle
  gösterir.

### Çalışma zamanı kanıtı

`render_kosumu.rs::bil010_acc031_bitisik_bolut_gercek_etkilesimde_tek_gonderim_uretir`
gerçek GPUI penceresinde:

1. Sorgu gerçek klavye yolundan girilir; yazmak gönderim üretmez.
2. Bölütün görünür merkezi **kuşak geometrisinden** türetilir; sabit
   koordinat kullanılmaz.
3. Tıklama tam **bir** `AramaGönderildi` üretir — sayı ekrandaki olay
   akışının kendisinden okunur, ayrı sayaç tutulmaz.
4. Gönderim anındaki sorgu alanda durur (`"kedi"`).
5. Bölüte basmak odağı bırakmaz.
6. Uçuştaki gönderim varken ikinci tıklama yeni gönderim üretmez.

## 4. Kapsam sınırı

- Bu rapor yalnız galeri tüketicisinin kanıtıdır. Sağlayıcının kendi
  ölçüt zinciri ve sandık dışı public yüzey kanıtı çekirdek deposundadır.
- Galeri hâlâ kendi kromunu GPUI `rounded()` ile çizer; tezgâh kromu
  `ORT-003` şekil-duyarlı boya yolunu tüketmez. Bu bilinçli sınırdır:
  sağlayıcının boya yolu bileşen kabuğu içindir, tezgâh çerçevesi değil.
- `BİL-040` düğme bileşeninin görünür Rust uygulaması henüz yoktur; bu
  yüzden düğme kuşağı galeride görünür tüketici olarak koşmaz. Eylem
  bölütünün `BitişikBölütVurgusu::Açık(DüğmeGörünümü)` dalı da bu yüzden
  üretimde yoktur; tezgâh yalnız `AlandanÇöz` sunar.
- **Klavye ekseni kapandı** (önceki kesitte "bölütün odak tutamacı ve Tab
  durağı henüz yok" yazıyordu; bu kayıt bayattı). Eylem bölütü kendi odak
  tutamacını taşır, `tab_stop` olarak kaydedilir ve §23.2'nin "klavyeyle
  ulaşıldığında odak olağan Tab sırasıyla bölüte geçer" hükmü galeride
  gerçek tuş olaylarıyla ölçülür: `tab` bölüte ulaşır, `shift-tab` çıkar,
  `enter` mevcut gönderim hattına iner. Galeri kökü `ORT-005 §2` odak
  dolaşımını yürütür (`tab`/`shift-tab` → `focus_next`/`focus_prev`).

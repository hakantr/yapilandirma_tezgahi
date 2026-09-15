# ORT-003 galeri kanıt raporu

> Tarih: 15 Eylül 2026
>
> Galeri kaynak commit'i: `d2249398fd7a22fb0ec8796391360cd02fd095e4`
>
> Exact çekirdek bağı: `ee4357db2316e741c3cca0055ce41693de8d146b`
>
> Galeri başlangıç tabanı: `13e350931f7e9985d9140fc23961718071c8c7f2`
>
> Ortam: Apple Silicon `arm64`, macOS `26.6.2`, `rustc 1.97.1`

Bu rapor `ORT-003 1.5.9` kutu şekli sağlayıcısının galeri tüketicisini ve o
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
| `cargo test --workspace` (galeri) | 269 geçti, 0 kaldı |
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

## 3. Kapsam sınırı

- Bu rapor yalnız galeri tüketicisinin kanıtıdır. Sağlayıcının kendi
  ölçüt zinciri ve sandık dışı public yüzey kanıtı çekirdek deposundadır.
- Galeri hâlâ kendi kromunu GPUI `rounded()` ile çizer; tezgâh kromu
  `ORT-003` şekil-duyarlı boya yolunu tüketmez. Bu bilinçli sınırdır:
  sağlayıcının boya yolu bileşen kabuğu içindir, tezgâh çerçevesi değil.
- `BİL-040` düğme bileşeninin görünür Rust uygulaması henüz yoktur; bu
  yüzden düğme kuşağı galeride görünür tüketici olarak koşmaz.

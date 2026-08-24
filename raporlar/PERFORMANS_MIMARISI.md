# Tezgâh performans mimarisi

> Tarih: 24 Ağustos 2026
> Kapsam: `gpui-bilesenleri-galeri` çizim mimarisi; kullanıcı onaylı
> performans planının birinci turu.
> Not: Önceki devirde anılan "TEZGAH_DEVIR_NOTU sonundaki performans
> raporu" hiçbir dosyaya yazılmamıştı; bu belge o boşluğu doldurur ve
> planın kabul hedeflerini yeniden, bu kez ölçülebilir biçimde tanımlar.

## 1. GPUI'nin gerçek çizim modeli (eş kaynak `../gpui` üzerinde doğrulandı)

Planın doğru okunması bu mekaniğe dayanır; varsayım değil, kaynak
okumasıdır (`gpui/src/window.rs`, `gpui/src/view.rs`):

- **Her çizim kökten başlar.** `Window::draw → draw_roots` kök view'ın
  `render`'ını her karede çağırır (`window.rs:3117`). Herhangi bir
  entity'nin `notify`'ı pencereyi kirletir ve bir sonraki kare **bütün**
  önbelleklenmemiş ağacı yeniden kurar. Bir alt entity'yi `.child(entity)`
  ile gömmek tek başına çizim maliyetini değiştirmez.
- **Çizim atlamanın tek yolu `Entity::cached(style)`.** Önbellekli bir
  view, sınırları/içerik maskesi/metin stili değişmedikçe ve
  `dirty_views`e girmedikçe önceki prepaint/paint aralığını yeniden
  kullanır (`view.rs:386-401`). Önbellek, view **kendisi** bildirdiğinde
  ya da bir **alt view'ı** bildirdiğinde patlar: `mark_view_dirty`
  bildirilen view ile birlikte ata view'ları da kirletir
  (`window.rs:1941-1953`).
- **Önbellekli view'ın okuduğu entity'ler izlenir ama view'ı kirletmez.**
  `detect_accessed_entities` erişimi pencere düzeyinde kaydeder; entity
  değişince pencere yeniden çizilir, fakat önbellekli view `dirty_views`te
  olmadığı için **eski içeriğiyle** kalır. Sonuç: *önbelleğe alınacak bir
  bölge, durumunu okuduğu entity'yi gözlemiyorsa bayatlar.*

Bu üç gerçek, planın sırasını belirler: alan durumunu okuyan kartlar önce
kendi gözleyen entity'lerine taşınmalı ki sonraki turda geri kalan bölgeler
güvenle `cached` yapılabilsin.

## 2. Bu turda yapılanlar (birinci tur)

### 2.1 Kök köprüler kaldırıldı → `AlanDurumPaneli` + `OlayAkışıPaneli`

Eski durum: kök `GaleriUygulaması`, alana `observe(&alan) → notify()` ve
`subscribe(&alan) → tezgah_olayını_kaydet → notify()` iki köprüyle
bağlıydı. Her tuş vuruşu kökü bildiriyordu; kök çizimi de raporu, kod
metnini ve olay klonunu **kare başına** yeniden kuruyordu.

Yeni durum (`src/paneller.rs`):

- **`AlanDurumPaneli`** — `C` türetilmiş durumlar kartı + `§13/§19` değer
  üçlüsü kartı. Alanı `observe` eder, yalnız kendini bildirir. Tercihi
  zayıf kök tutamacından okur; `önem_zemini` düğmesi mutasyonu
  `tezgahı_değiştir`e iletir (`panel_tercih_düğmesi`).
- **`OlayAkışıPaneli`** — `§26` akışının sahibi. Olay aboneliği ve
  yinelenen-olay sayacı panelde; "Temizle" paneli günceller, kökü değil.
- Paneller alanla birlikte kurulur; tür değişiminde alan yeniden
  kurulduğunda `alanı_bağla` ile yeni alana bağlanır, entity kimlikleri
  kararlı kalır (akış geçmişi de korunur).
- Kart gövdeleri (`turetilmis_durum_satırı`, `değer_durumu`, `olay_akışı`)
  `sergiler.rs`'te kaldı — `§16.2` yapısal kanıt testleri o dosyayı
  `include_str!` ile okuyor; yalnız `Context` tipleri panellere döndü.

### 2.2 Rapor ve kod paneli tercih sürümüne bağlandı

`tercih_sürümü: u64` her `tezgahı_değiştir`de artar;
`yapılandırma(...).doğrula()` (§29 raporu) ve `tercih.kod()` (kod paneli
metni) yalnız sürüm değişince yeniden kurulur (`tezgah_raporu`,
`tezgah_kodu`). Eski kod raporu kare başına kuruyordu ve her kuruluş
`YardımcıKimlikleri::yeni` üzerinden **kimlik fabrikasından beş örnek
kimliği** tüketiyordu; ikisi de durdu.

### 2.3 Yol birleştirme ve açık bildirim

- `tezgah_ekranını_çiz` artık test erişim noktası `tezgah_profil_içeriği`
  ile aynı gövdeyi kullanır; iki kopya sessizce ayrışamaz.
- `tezgah_dış_bildirimi` köke açık `notify()` verir: port kapıları kartı
  `doğrulama_portu.is_some()` okur ve kök çizimindedir — kök alanı artık
  gözlemediği için bu değişim kendi bildirimini taşır.
- Ulaşılamayan galeri kataloğu yolu (`tezgah_ekranı` hep `true`; borç 22)
  derlenmeye devam eder: `SergiDurumu` tezgâh içeriğini hazır alır.

### 2.4 Bekçiler

- `paneller.rs` test modülü: kökte `observe` köprüsü geri gelemez; rapor
  yalnız önbellek ıskasında kurulur; gözlem kanalları panellerdedir.
- `tezgah_gosterge.rs · panel_sonucu_saklamaz` kapısı `paneller.rs`'i de
  tarar (opak `DurumGöstergesiDurumu` hiçbir yerde saklanmaz).
- `bölüm_sözlüğünde_ölü_kayıt_yok` panel dosyasındaki başlık kullanımını
  tanır.

### 2.5 `akici-dev` profili

Kök `Cargo.toml`: `inherits = "dev"`, çalışma ağacı `opt-level = 1`,
bağımlılıklar (GPUI dâhil) `opt-level = 3`. Elle etkileşim denemeleri
için: `cargo run --profile akici-dev -p gpui-bilesenleri-galeri-masaustu`.

### 2.6 Doğrulama

- `cargo test -p gpui-bilesenleri-galeri`: 206 test yeşil (203 + 3 bekçi).
- WASM'de elle: yazarken değer durumu/türetilmiş durum/olay akışı canlı;
  "Zemine de uygula" panel→kök yolu; tür değişiminde panel yeniden
  bağlanması; "Temizle"; dış hata bildiriminde kenarlık + gösterge + port
  rozeti. (`python3 tools/wasm_galeri_hazirla.py` + sunucu.)

## 3. Bu turun kazancı — dürüst değerlendirme

GPUI kökten çizdiği için tuş vuruşu başına ağaç kuruluşu **henüz**
küçülmedi; kazançlar şunlardır:

1. Tuş vuruşu başına `doğrula()` + kod üretimi + olay vektörü klonu ve
   kimlik fabrikası tüketimi kalktı (ölçülebilir, kesin).
2. Çift köprünün gereksiz gözlemci çağrıları kalktı.
3. Alan okuyan her kart artık gözleyen bir entity'de — ikinci turun
   (`cached` bölgeler) doğruluk ön koşulu hazır.

## 4. İkinci tur: sağ kolon bölüm entity'leri (sıradaki iş)

Hedef: tuş vuruşunda yalnız alan + iki panel çizilsin; tercih değişiminde
yalnız etkilenen bölgeler kurulsun.

- Sağ kolon bölümlerini (ve üst şeridi) `Entity::cached(style)` sınırına
  almak. Kısıt: önbellekli view `style`tan yerleşir, içerikten ölçülmez —
  kaydıran kolon (`overflow_y_scroll`, `flex_1`) doğal aday; sarmalayıcı
  stiller `KolonMetriği`nden türetilmeli.
- Bölüm kartlarındaki 16 `tezgahı_değiştir` listener'ı köke zayıf
  tutamaçla bağlanmalı (bu turdaki `panel_tercih_düğmesi` deseni).
- **`yuva_görünürlük_notu` taşınmalı**: kök çiziminde kalan tek alan
  okuması budur (`sergiler.rs`, `alan.read` — kutu boş mu?). Bugün
  doğru çalışır çünkü kök her karede çizilir; sol kolon `cached`
  olduğu gün bayatlar. Not ya `AlanDurumPaneli`ne ya kendi küçük gözleyen
  entity'sine gitmeli.
- `profil.rs` thread-local görünüm/palet kanalı kökte her karede kurulur;
  önbellekli bölgeler çizim atladığında bu kanal onlara işlemez — tema
  değişimi zaten kökten `notify` ile geldiğinden bugün sorun değil, ama
  `cached` bölgeler tema sürümünü kendi girdisi olarak taşımalı
  (`TezgahTeması::sürüm` alanları hazır).

## 5. Kabul hedefleri ve ölçüm

Önceki turun sayısal hedefleri devirde kaybolduğu için hedefler yeniden,
ölçüm yoluyla birlikte konur:

1. **Tuş vuruşu gecikmesi (uçtan uca):** `akici-dev` profilli masaüstü
   koşumunda GPUI profiler'ı (`gpui` `profiler` özelliği /
   `debug_frame_overlay`) ile kare süresi; hedef ikinci tur sonunda
   tuş vuruşu karesinde **sağ kolonun hiç render edilmemesi**
   (`dirty_views` yalnız alan + paneller).
2. **`ORT-018 bil-010.input.commit`:** tezgâhtaki yerleşik ölçüm
   (`ölçüm_toplu_ms`) iki hedefte de koşturulabilir; kabul motoru bu
   turdan etkilenmedi, sayı gerilememeli.
3. **Linux ölçümü:** kullanıcının Linux düğümünde `akici-dev` ile
   masaüstü koşumu (birinci turda yalnız macOS + WASM'de doğrulandı;
   Linux koşumu bekliyor).

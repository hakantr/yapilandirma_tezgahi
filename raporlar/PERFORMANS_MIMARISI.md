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

## 3. Birinci turun kazancı — dürüst değerlendirme

GPUI kökten çizdiği için tuş vuruşu başına ağaç kuruluşu birinci turda
**henüz** küçülmedi; kazançlar şunlardı:

1. Tuş vuruşu başına `doğrula()` + kod üretimi + olay vektörü klonu ve
   kimlik fabrikası tüketimi kalktı (ölçülebilir, kesin).
2. Çift köprünün gereksiz gözlemci çağrıları kalktı.
3. Alan okuyan her kart artık gözleyen bir entity'de — ikinci turun
   (`cached` bölgeler) doğruluk ön koşulu hazırlandı. Bu ön koşul işe
   yaradı: sağ kolon önbelleğe alındığında alan okumadığı için bayatlama
   riski taşımadı (§6.5).

## 4. İkinci tur: sağ kolon bölüm paneli — HEDEF TUTTU (düzeltmeyle)

> **Not (ölçüm turu, 24 Ağu 2026).** Bu turun hedefi — tuş vuruşu karesinde
> sağ kolonun kurulmaması — **gerçekleşti ve ölçüldü** (§6.2), ama o günkü
> hâliyle değil. Önbellek sınırı doğruydu; eksik olan **geçersizleme
> yoluydu**: GPUI'de `notify` bir `cached` sınırını patlatmaz, bu yüzden
> kolon açılıştan sonra hiç yeniden kurulmuyor, yani bayat kalıyordu.
> Ölçüm bunu yakaladı, çalışan yol (`refresh`) bulundu ve kolon doğru
> geçersizlemeyle önbelleğe geri alındı. Kanıt ve gerekçe: §6.3–6.5.
> Aşağıdaki alt bölümlerde önbellekle ilgili cümleler o günkü hâli anlatır;
> geçerli mekanik §6.5'tedir.

### 4.1 `BölümlerPaneli` (`src/paneller.rs`)

- Sağ kolonun tamamı tek önbellekli entity:
  `Entity::cached(StyleRefinement …flex_1())` sınırında gövdeye girer
  (`Tezgahİçeriği.yapılandırma: AnyElement`). Kolon kaydıran bir kap
  olduğu için boyutu içerikten bağımsızdır — `cached`ın "stilden yerleşir,
  içerikten ölçülmez" kısıtına uyar. Bölüm **kartları** içerik yükseklikli
  olduğundan kart başına önbellek kurulamaz; sınır kolon düzeyindedir.
- Panel alanı **gözlemez** — tuş vuruşları kolona işlemez. *(O gün kökü
  `observe` ediyordu ve geçersizlemenin bundan geleceği sanılıyordu;
  §6.3 bunun çalışmadığını gösterdi, abonelik kaldırıldı ve yerini kökün
  `kolonu_geçersizle` çağrısı aldı.)*
- **Listener'lar yeniden yazılmadı.** Panelin çizimi kökü `update` ile
  açar ve bölümleri kökün kendi bağlamında üretir
  (`GaleriUygulaması::tezgah_bölümleri` → profilin `bölümler()`i);
  karttaki 16 `tezgahı_değiştir` dinleyicisi köke bağlı kalır. Çizim
  sırasında kök kiralı değildir (GPUI, kök render'ı bitip element ağacı
  yerleşirken alt view'ları çizer), bu yüzden `update` güvenlidir.
- Bölüm listesinin tek kaynağı `tezgah_bölümleri`: ekrandaki panel de
  `tezgah_profil.rs` tür süzgeci testleri de oradan okur.

### 4.2 Kaynak okumasıyla "doğrulanan" üç mekanik — ve dersi

Şu üç mekanik kaynaktan doğru okundu ve hâlâ doğrudur:

- **Kaydırma:** GPUI, kaydırma ofsetini değiştirirken kaydıran öğenin
  view'ını bildirir (`div.rs paint_scroll_listener → cx.notify(current_view)`).
- **İç entity'ler:** bildirilen view'ın ataları da kirletilir
  (`mark_view_dirty`).
- **Açık listeler:** `deferred` çizimler ve fare dinleyicileri
  `reuse_prepaint`/`reuse_paint` ile taşınır.

**Ama hiçbiri "önbellek gerektiğinde patlar" sonucunu vermiyordu.** Üçü de
"bildirim şu yola girer" der; girdiği yolun sonunda `dirty_views`e ulaşıp
ulaşmadığını söylemez. Ölçüm gösterdi ki ulaşmıyor (§6.3). Ders: bir
mekanizmanın parçalarını kaynaktan okumak, bileşiminin çalıştığını
kanıtlamaz — çalıştığını yalnız çalışırken saymak kanıtlar.

### 4.3 `YuvaNotuPaneli` (ayakta)

Kökün çizim yolundaki **son alan okuması** (`yuva_görünürlük_notu`,
"kutu boş mu?") kendi gözleyen entity'sine taşındı; kabuk yuvaları kartı
notu panel olarak gömer. Kökün çizim yolunda artık alan okuması yoktur.
Bu kazanç önbellekten bağımsızdır ve durmaktadır.

### 4.4 Kabuk sınırındaki değişiklik

`Tezgahİçeriği.bölümler: Vec<TezgahBölümü>` kalktı; yerinde
`yapılandırma: AnyElement` var. Akış ayrıştırma `arayuz.rs`'te serbest
fonksiyona indi; kolon gövdesi `govde.rs`'te
(`yapılandırma_kolonu_gövdesi`, kabuk tarafı) durur ve panel onu çağırır.
Kabuk yine hiçbir bileşen tipini tanımaz.

### 4.5 O günkü doğrulama — ve neden yetmedi

O tur şöyle doğrulanmıştı: 208 test yeşil; WASM'de elle yazma, tür/biçim
değişimi, açık listenin karelerde ayakta kalması, kaydırma, koyu kip.
Hepsi geçti ve **hiçbiri önbelleğin geçersizleştiğini ölçmüyordu**:

- Yapısal bekçiler `.cached(` ve `observe(kök` dizgelerinin **varlığını**
  sınadı; davranışı değil.
- WASM'deki gözle denemeler "kolon güncel görünüyor" dedi — ama kolon o
  sırada da donmuş olabilirdi ve bakılan değişikliklerin çoğu kolonun
  dışındaki bir yolu tetikliyordu. Gözle bakmak, "bu kare kolonu yeniden
  kurdu mu" sorusunu yanıtlayamaz.

Eksik olan tek şey bir sayaçtı; ölçüm turunda eklenince bulgu ilk
koşumda çıktı.

## 5. Üçüncü tur: üst şerit ve sol tercih şeritleri — TAMAMLANDI (24 Ağu 2026)

Bu bölgeler `cached` sınırına **alınmadı** ve bu bir tercihtir, eksik
değil:

- `cached` view stilden yerleşir, içerikten ölçülmez. Üst şerit
  (`flex_wrap`) ve sol kolonun şerit/kartları içerik yüksekliklidir;
  onlara sabit yükseklik yazmak tipografi/yoğunluk eksenlerine karşı
  kırılgandır. Kaydıran sol blok ise alan gözleyen panellerle (türetilmiş
  durum, değer üçlüsü, olay akışı, yuva notu) serpiştirilmiştir — o
  panellerin bildirimi ataları kirlettiği için bloğu saran bir önbellek
  her tuş vuruşunda patlar, kazanç sıfırlanır.

Bunun yerine bu bölgelerin kare-başı maliyetinin asıl kaynağı öldürüldü:

### 5.1 Açılır liste içerikleri tembel (`şerit_seçicisi`)

İçerik parametresi `impl FnOnce(&mut Context<…>) -> AnyElement` oldu ve
yalnız seçici **açıkken** çağrılır. Eski imza hazır element alıyordu:
kapalı 15 seçicinin listeleri — üst şeritteki yazı ailesi listesi
(masaüstünde yüzlerce sistem ailesi), tema/ölçek/yoğunluk/iç boşluk/
hareket, sol kolondaki parça ailesi ve imleç listeleri, sağ kolondaki
biçim/adım/varsayılan/bölüm/doldurma/saat dilimi listeleri — dinleyicileri
ve öğeleriyle her karede kuruluyor ve hiç çizilmeden atılıyordu. Artık
kapalı seçicinin kare maliyeti tek tetikleyici satırıdır. Bekçi:
`şerit_seçicisi_içeriği_tembel_alır` (sergiler.rs test modülü).

### 5.2 Kare dikişi önbellekleri (tema sürümüne bağlı)

- **Çözülmüş görünüm** (`tasarım_görünümünü_çöz` + onun kurduğu tam tema
  anlık görüntüsü) artık kök `kare_dikişlerini_kur` içinde tema sürümüne
  bağlı önbellekten gelir; `görünümü_paylaşımlı_kur` aynı `Arc`'ı kare
  kare paylaşır. `temayı_tazele` sürümü dikişten **önce** artırır ki
  önbellek bayat çözüm döndürmesin. Bekçi:
  `görünüm_çözümü_tek_yerden_koşar`.
- **`ORT-003` yarıçap tavanı** (`en_fazla_yarıçap`): profil içeriği her
  karede tek ölçü için tam `tezgah_teması(...)` anlık görüntüsü
  kuruyordu; değer tema sürümüne bağlandı.
- Render'daki ve `temayı_tazele`deki kopya dikiş blokları tek
  `kare_dikişlerini_kur` metodunda birleşti.

### 5.3 Doğrulama

- 210 test yeşil (208 + 2 bekçi).
- WASM'de elle: yazı ailesi listesi tembel yoldan açılıyor ve seçim tüm
  ekranın tipografisini değiştiriyor (görünüm önbelleği geçersizlemesi);
  imleç seçicisi açılıp kalınlık uyguluyor; koyu kip önbellekli sağ kolon
  dâhil her yere işliyor; alana yazma ve yuva notu canlılığı bozulmadı.

Bu turdan sonra tuş vuruşu karesinde kalan iş: kök kabuğu + sol kolonun
şerit/kart elementleri (listeleri kırpılmış hâlde) + alan gözleyen küçük
paneller. Sağ kolon önbellekten gelir (§6.5), hiçbir kapalı liste
kurulmaz, hiçbir çözüm/rapor/kod yeniden hesaplanmaz. Ölçülen karşılık:
~1,16 ms (§6.2).

## 6. Ölçüm turu (24 Ağu 2026) — hakem konuştu

Üç tur boyunca hakem yoktu: kararlar kaynak okumasına, yapısal bekçilere
ve gözle yapılan WASM denemelerine dayanıyordu. Bu tur ölçüm koşumunu
kurdu ve ilk işi bir mimari kararı çürütmek oldu.

### 6.1 Koşum (`tests/kare_olcumu.rs`)

`TestApp` üzerinde gerçek pencere açar, gerçek metin sistemiyle
(`CosmicTextSystem` — saf Rust, macOS ve Linux'ta aynı shaping) koşar.
Ölçüm penceresi **girdiden ekrana kadar geçen bütün CPU işidir**:
mutasyon, efekt döngüsü (bildirimler, `refresh`, abonelik zincirleri) ve
o döngünün kirli pencere için yaptığı çizim(ler). Temiz kare senaryosunda
mutasyon olmadığı için çizim açıkça istenir.

Bu pencere bir düzeltmenin sonucudur: ölçüm bir ara mutasyonun hemen
ardındaki `draw`'ı zamanlıyordu ve o, `refresh` efekt kuyruğuna girdiği
için kolonun **kurulmadığı** kareydi — kolon sonraki, ölçülmeyen çizimde
kuruluyordu. S/T süreleri bu yüzden olduğundan ucuz çıkıyordu. Her
senaryo artık kolon kurulum sayısını da raporlar (`kolon N/N`) ve sayılar
ancak o kapı tutarsa yorumlanır. Varsayılan `cargo test` koşumunda
atlanır:

```bash
KARE_OLCUM=1 cargo test --profile akici-dev -p gpui-bilesenleri-galeri \
    --test kare_olcumu -- --nocapture
```

Ölçülen şey CPU kare maliyetidir (ağaç kurulumu + yerleşim + prepaint +
paint). GPU sunumu headless koşumda yoktur; gerçek input-to-present bunun
üstüne platform sunum süresini ekler.

Koşumun kurulumunda iki hata yapıldı ve ikisini de sayaç yakaladı:
mutasyonla çizimin sırası (efekt döngüsü araya girince ölçüm mutasyondan
**sonraki ikinci** kareyi ölçüyordu) ve efekt döngüsünün kendiliğinden
yaptığı ek çizimin sayıma karışması. İkisi de düzeltildi; ölçüm artık
kendi geçerlilik kapısını taşıyor (`kolon N/N`).

### 6.2 Sayılar (macOS, `akici-dev`, 1600×1000, 200 tekrar)

Önbellekli (bugünkü hâl):

| Senaryo | ort | p50 | p95 | en az | kolon kurulumu |
|---|---|---|---|---|---|
| D · temiz kare | 1,13 ms | 1,11 ms | 1,33 ms | 1,03 ms | 0/200 |
| K · tuş vuruşu | 1,24 ms | 1,22 ms | 1,41 ms | 1,15 ms | 0/200 |
| S · seçici | 2,86 ms | 2,83 ms | 3,27 ms | 2,57 ms | 200/200 |
| T · tercih | 2,77 ms | 2,69 ms | 3,10 ms | 2,54 ms | 200/200 |

Aynı koşum, kolon önbelleği kapalıyken (karşılaştırma tabanı):

| Senaryo | önbelleksiz | önbellekli | kazanç |
|---|---|---|---|
| D · temiz kare | 3,43 ms | 1,13 ms | %67 |
| K · tuş vuruşu | 3,50 ms | 1,24 ms | **%65** |
| S · seçici | 3,47 ms | 2,86 ms | %18 |
| T · tercih | 3,36 ms | 2,77 ms | %18 |

Okuma:

- **Tuş vuruşu ~1,24 ms** ve o işin içinde kolon hiç kurulmuyor (0/200).
  Kazanç burada: %65.
- S ve T'de kolon her tekrarda kuruluyor (200/200) — tazelik korunuyor ve
  ölçülen iş kurulumu **içeriyor**. Oradaki %18, kolon dışındaki
  önbelleklerden (rapor, kod, çözülmüş görünüm) gelir.
- 120 Hz CPU bütçesi (8,33 ms) karşısında tuş vuruşu ~%15, 60 Hz
  (16,7 ms) karşısında ~%7.
- Bu sayılar **headless CPU'dur ve gerçek gecikme değildir**; sunum
  (present/vsync), giriş kuyruğu ve fiziksel girdi eklenmemiştir. Tek
  makine, tek pencere boyutu.

### 6.3 Bulgu: `Entity::cached` sessizce donuyordu

Ölçüme eklenen çizim sayacı (`bölüm_çizim_sayısı`) sağ kolonun
**açılıştaki ilk çizimden sonra hiç yeniden kurulmadığını** gösterdi.
Denenen üç geçersizleme yolunun hiçbiri işe yaramadı:

1. Kökün bildirimi (`observe(kök) → notify`),
2. Panele **doğrudan** `notify`,
3. `refresh_windows()`.

Kolon önbellekten çıkarılınca aynı ölçüm her karede kurulum gösterdi —
yani altyapının geri kalanı sağlamdı, sorun `cached` sınırının kendisiydi.
İkinci turun "tuş vuruşunda kolon kurulmuyor" iddiası bu yüzden bir hız
kazancı değil, **bayat yapılandırma yüzeyi** anlamına geliyordu: tercih
değişse de kolon açılıştaki hâlinde kalırdı. Önbellek geri alındı.

Neden gözle fark edilmedi: WASM'de yapılan denemelerin çoğu tür/biçim
değişimiydi ve ekranda değişen şeyler kolon dışındaki yollardan da
geliyordu; "kolon güncel görünüyor" izlenimi, "bu kare kolonu yeniden
kurdu" ile aynı şey değil. Bu ayrımı yalnız sayaç yapabiliyor.

**Kaybın büyüklüğü ölçüldü.** Donmuş kolonla kare ~1,1 ms, önbelleksiz
canlı kolonla ~3,0 ms. Yani sağ kolonun kurulumu kare maliyetinin
**~%60'ı** — kazanç gerçek, eksik olan tek şey geçersizlemeydi.

### 6.4 Ölçümün kendi iki hatası

Sayaç yalnız mimariyi değil, ölçümü de denetledi:

- **Yanlış kare.** Mutasyon ayrı bir `update` bloğuna konduğunda, test
  kipindeki efekt döngüsü kirli pencereyi kendiliğinden çiziyor
  (`app.rs`, `flush_effects`) ve ölçülen `draw` mutasyondan **sonraki
  ikinci** kare oluyordu. Bu, bir ara "`refresh_windows` de işe yaramıyor"
  sonucunu üretti — oysa işe yarıyordu, etkisi ölçülmeyen karede kalıyordu.
- **Paylaşılan sayaç.** Sayaç süreç genelinde `static`ti; aynı süreçte
  paralel koşan testler birbirinin çizimlerini sayıyor ve sahte
  başarısızlık üretiyordu. `thread_local` yapıldı — bir GPUI uygulaması
  zaten kendi iş parçacığında çizer.
- **Dar ölçüm penceresi.** Düzeltmenin ardından ölçüm mutasyonun hemen
  ardındaki `draw`'ı zamanlıyordu; `refresh` efekt kuyruğuna girdiği için
  o kare kolonun kurulmadığı kareydi ve S/T süreleri olduğundan ucuz
  çıkıyordu (dış inceleme bulgusu). Pencere, girdiden ekrana kadar geçen
  bütün CPU işini kapsayacak biçimde genişletildi ve kapı `kolon N/N`
  olarak sıkılaştırıldı: S/T artık tekrar başına en az bir kurulum
  içermek zorunda.

Ders: ölçüm aracı da ölçülen sistem kadar şüpheyle karşılanmalı. Üç hata
da "beklenmedik sayı" olarak göründü ve kovalandığında araçta çıktı;
üçüncüsünü bağımsız bir inceleme yakaladı.

### 6.5 Çözüm: geçersizleme `refresh` ile yapılır

Üç yol denendi ve sayaçla ayırt edildi:

| Yol | Kolon yeniden kuruldu mu? |
|---|---|
| Kökün bildirimi (`observe(kök) → notify`) | hayır |
| Panele **doğrudan** `notify` | hayır |
| `refresh_windows()` / `Window::refresh()` | **evet** |

Nedeni GPUI'nin bildirim yolundadır: `App::notify` bir entity'nin
bildirimini yalnız o entity pencerenin `tracked_entities` kümesindeyken
`invalidate_view`e çevirir; önbellekten dönen bir view render edilmediği
için o kümeye kendi kimliğiyle girmez ve `dirty_views`e hiç ulaşmaz.
`refresh` ise ayrı bir kanaldır: `refreshing` bayrağı prepaint'teki cache
koşulunu doğrudan düşürür (`view.rs` · `!window.refreshing`).

Uygulama: kök `GaleriUygulaması::kolonu_geçersizle` ile **yalnız kendi
penceresini** yeniler (`defer` + `Window::refresh`; tutamaç ilk çizimde
yakalanır). `refresh_windows()` yalnız tutamaç henüz yokken, yani ilk
çizimden önce yedek yoldur — bütün pencereleri yenilemek hedefli değildir
ve ileride açılacak başka pencerelerin önbelleklerini de kırardı.
`Window::refresh` doğrudan çağrılamaz çünkü buraya bir listener içinden
gelinir ve pencere o sırada kiralıdır (`App::update_window` pencereyi
`take` eder). Geçersizleme kolonu ilgilendiren **her** kök değişiminde
yapılır — tercih (`tezgahı_değiştir`), tema (`temayı_tazele`),
açık seçici (`seçiciyi_değiştir`), `§16` dış bildirim
(`tezgah_dış_bildirimi`). Efekt üzerinden çalışması `Window` erişimi
gerektirmemesini sağlar ve gerçek akışa uyar: listener bildirir, efekt
döngüsü bayrağı kurar, sıradaki kare kolonu yeniden kurar, sonraki temiz
kareler yine önbellekten gelir.

Bir yan gözlem: alana ilk yazma da bütün önbellekleri atlar, çünkü
`Window::focus` `refresh()` çağırır (odak halkası ve tuş yönlendirmesi
pencere geneli meselelerdir). Sonraki vuruşlar odağı değiştirmediği için
önbellek isabet eder — ölçümdeki 0/200 bu evrenindir.

### 6.6 Kalıcı kapı (`tests/kolon_tazeligi.rs`)

Ölçümden bağımsız, her `cargo test` koşumunda koşar ve **iki yönü birden**
sabitler; biri olmadan diğeri değersizdir:

- **Tazelik:** tercih, tür, tema, açık seçici ve dış bildirim kolonu
  yeniden kurdurur. Kurdurmazsa ekranda bayat yapılandırma kalır.
- **Kazanç:** temiz kare ve (odak kurulduktan sonraki) tuş vuruşları
  kolonu kurdurmaz. Kurdururlarsa önbellek hiçbir işe yaramıyordur.

Yapısal bekçi de buna bağlandı: `paneller.rs` içinde `.cached(` varsa,
`lib.rs` içinde `refresh_windows` çağrısı ve en az dört
`kolonu_geçersizle` kullanımı bulunmalıdır.

## 7. Sol kolon ve üst şerit: desen neden uygulanmıyor (ölçüldü)

Sağ kolonda işe yarayan ikili — `Entity::cached` + `refresh` ile
geçersizleme — sol kolona ve üst şeride de uygulanmak istendi. Önce
payları ölçüldü, sonra uygulanabilirlikleri sınandı. Sonuç: **ikisi de
uygulanmıyor**, gerekçeler aşağıda. Bu, üçüncü turdaki kararın (§5)
ölçümle ve yeni mekanik bilgisiyle doğrulanmasıdır.

### 7.1 Payları küçük (ablation, aynı koşum)

Bölgeler geçici olarak boşaltılıp kare süresi yeniden alındı:

| Çıkarılan | D · temiz | K · tuş vuruşu | pay (yaklaşık) |
|---|---|---|---|
| — (tam ekran) | 1,13 ms | 1,24 ms | — |
| Üst şerit | 0,98 ms | 1,21 ms | ~0,15 ms (D'de %13) |
| Alan gözleyen iki panel | 1,00 ms | 1,01 ms | ~0,24 ms (K'de %19) |

Ölçüm gürültüsü bu ölçekte belirgin (p95–p50 farkı ~0,2 ms), yani
paylar "birkaç yüzde onda milisaniye" düzeyinde okunmalı.

### 7.2 Üst şerit: `cached` boyut kısıtına takılıyor

`Entity::cached` view'ı **stilden yerleştirir, içerikten ölçmez**
(`view.rs`). Sağ kolon bu kısıta uyuyordu çünkü `flex_1` bir kaptı. Üst
şerit ise içerik yüksekliklidir (iki satır, `flex_wrap`), yani `cached`
sınırına girmesi için sabit yükseklik yazmak gerekir. O sayı tipografiye,
yoğunluğa ve metin ölçeğine bağlıdır; sabitlemek `%200 ölçekte kabuk
okunur kalır` kabul ölçütünü (`YÖN-006.ACC-011`) riske atar. ~0,15 ms
için alınacak risk değil.

### 7.3 Sol kolon: canlı paneller sınırın **içinde**

Sol kolonun kayan bloğu boyut olarak uygundur (`flex_1` + `min_h(0)`,
sağ kolonla aynı profil). Engel başka: içinde alan gözleyen üç panel
yaşıyor (`YuvaNotuPaneli`, `AlanDurumPaneli`, `OlayAkışıPaneli`) ve
tasarımın kart sırası onları tercih kartlarının arasına yerleştiriyor.

Buradaki ayrım sağ kolonun neden işe yaradığını da açıklıyor:

- **Sağ kolon panellerin kardeşidir.** Panel bildirdiğinde
  `mark_view_dirty` panelin **atalarını** kirletir; kardeş kolon
  etkilenmez ve önbellekte kalır.
- **Sol kayan blok panellerin atasıdır.** Aynı mekanizma onu her panel
  bildiriminde kirletir — yani her tuş vuruşunda. Önbellek doğru çalışır
  ama hiç isabet etmez: kazanç sıfır.

Panelleri sınırın dışına almak sırayı bozar (kart dizilişi tasarımın
`§5` şemasıdır); ara grupları tek tek önbelleğe almak ise §7.2'deki boyut
kısıtına düşer. Bu yüzden sol kolon bugünkü hâlinde kalır.

### 7.4 Kayıt

Kalan ~1,24 ms'lik tuş vuruşu maliyeti şunlardan oluşur: kök kabuğu ve
üst şerit, sol kolonun şerit/kartları, yaşayan alanın kendi çizimi
(kanonik `MetinGirişiÖğesi`) ve alan gözleyen üç panel. Bunların hiçbiri
bugünkü mekaniklerle güvenle daraltılamıyor. Daha ileri gitmek için
gereken şey yeni bir önbellek katmanı değil, GPUI tarafında `cached`
sınırının içerik yüksekliğiyle çalışabilmesi ya da bildirim
taneciliğinin kardeş/ata ayrımından bağımsızlaşmasıdır — ikisi de bu
deponun işi değil.

## 8. İnceleme düzeltmeleri (24 Ağu 2026, bağımsız salt-okunur inceleme)

Üç turun ardından yapılan dış kaynak incelemesi dört noktayı düzeltti ya
da kesinleştirdi; kayıt buraya işlenir ki rapor tek başına doğru resmi
versin:

1. **Sıcak metin yolu zaten kanonik katmanda özel `Element`tir.**
   `GirişKutusu` dış kabuğunu `div` ile kurar; metin, seçim ve imleç
   çizimini `MetinGirişiÖğesi` yapar
   (`../gpui_bilesenleri/…/metin_girisi/bileşen.rs` — struct `2773`,
   `impl Element` `2824`, kabuğa gömülme `2475`, shaping `3004`'te
   `shape_line`). Zed'in "editör kanvası imperatif, chrome deklaratif"
   hibrit deseninin tek satırlık karşılığı bu yığında kuruludur. Bu
   raporun turları chrome tarafını daralttı; sıcak yola dokunmadı ve
   dokunması da gerekmiyordu — o yol bileşenin kendi turlarının malıdır.
2. **Shaping, glyph atlası ve batching GPUI tarafından sağlanır;
   "bedava" değildir.** Satır yerleşimi önbelleği ve `reuse_layouts`
   eşleşen metin/font için asıl shaping'i amorti eder; anahtar araması,
   run kuruluşu, önbellek/atlas ıskası ve rasterizasyon maliyetleri
   kalır. Atlas tek doku değildir (mono/subpixel/polychrome) ve sahne
   tek draw call'a inmez.
3. **İzolasyonun doğru tarifi yarıçaptır, sıklık değil.** Alan gözleyen
   paneller her alan bildirimini alır ve gerçekleşen her çizimde yeniden
   çizilir; kazanç, tuş vuruşunun kabuğun tamamına değil üç küçük panele
   (ve alanın kendi öğesine) değmesidir.
4. **ORT-007 gösterim beslemesi gecikme kanıtı değildir.** Tezgâhın
   portu anında çözülen bir future döndürür; kanıtladığı şey sürüm
   damgalı commit kapısının mekanizmasıdır. Gerçek arka plan yükü ve
   gecikme davranışı ürün portlarının işidir.

`§1`'deki çizim modeli cümlesi de şu kesinlikle okunmalı: kök render
"her ekran yenilemesinde" değil, **gerçekleşen her GPUI çiziminde**
koşar; hiçbir entity kirli değilse çizim de yoktur.

## 9. Sıradaki işler

1. **Sol kolon ve üst şerit — kapandı (§7).** Desen denendi, payları
   ölçüldü ve uygulanmadı: üst şerit `cached`in boyut kısıtına takılıyor,
   sol kolonun kayan bloğu ise canlı panelleri **içerdiği** için her tuş
   vuruşunda kirlenir. Yeniden açmak için GPUI tarafında bir değişiklik
   gerekir.
2. **Linux ölçümü:** aynı koşum kullanıcının Linux düğümünde
   (`CosmicTextSystem` iki hedefte de aynı olduğu için sayılar
   karşılaştırılabilir).
3. **Gerçek input-to-present:** headless CPU ölçümü sunum ve vsync
   içermez; uçtan uca gecikme ancak gerçek pencerede, platform
   profiler'ıyla (`gpui` `profiler` özelliği / `debug_frame_overlay`)
   ölçülür.
4. **`ORT-018 bil-010.input.commit`:** tezgâhtaki yerleşik ölçüm
   (`ölçüm_toplu_ms`) kabul motorunu ölçer; bu turlardan etkilenmedi,
   sayı gerilememeli.

Kayıtlı sınır: bu depo için **120 FPS ya da "sıfıra yakın gecikme" iddiası
yoktur**. Ölçülen tek şey headless CPU işidir — odak sonrası tuş vuruşu
için ~1,24 ms (120 Hz bütçesinin ~%15'i) — ve o da tek makinede, tek
pencere boyutunda, sunum/vsync ve fiziksel girdi dışarıda bırakılarak
alınmıştır. Yapılabilecek en geniş iddia şudur: *1600×1000 headless macOS
ölçümünde, odak sonrası bir tuş vuruşunun CPU çizim aşaması 120 Hz kare
bütçesine sığıyor.*

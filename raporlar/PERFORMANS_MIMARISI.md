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

### 6.2 Sayılar (macOS, `akici-dev`, 1600×1000, 200 tekrar × 5 koşum)

**Tek koşum güvenilir değil.** İlk ölçüm tek koşumdan alınmıştı ve
belirgin biçimde iyimserdi; beş ardışık koşum aynı sayıyı ~%50 yukarı
taşıdı (K: 1,24 → ~1,75 ms p50). Makine yükü ve ısınma bu ölçekte
görünür. Aşağıdaki taban **beş bağımsız koşumun** p50 medyanıdır;
köşeli parantez koşumlar arası aralıktır.

Önbellekli (bugünkü hâl):

| Senaryo | p50 medyan | koşum aralığı (p50) | ort medyan | p95 medyan | kolon |
|---|---|---|---|---|---|
| D · temiz kare | 1,55 ms | [1,54 – 1,61] | 1,76 ms | 3,16 ms | 0/200 |
| K · tuş vuruşu | 1,75 ms | [1,72 – 1,79] | 1,96 ms | 3,44 ms | 0/200 |
| S · seçici | 4,38 ms | [4,10 – 4,58] | 4,55 ms | 6,13 ms | 200/200 |
| T · tercih | 4,36 ms | [4,26 – 5,20] | 4,47 ms | 6,15 ms | 200/200 |

Taban — aynı makine, aynı koşum, `--features olcum-onbelleksiz`:

| Senaryo | p50 medyan | koşum aralığı (p50) | kolon |
|---|---|---|---|
| D · temiz kare | 3,55 ms | [3,23 – 4,40] | 200/200 |
| K · tuş vuruşu | 3,70 ms | [3,32 – 4,59] | 200/200 |

Kazanç (p50 medyanları): **D %56, K %53.**

Okuma:

- **Tuş vuruşu p50 ~1,75 ms**, o işin içinde kolon hiç kurulmuyor
  (0/200). 120 Hz CPU bütçesine (8,33 ms) göre ~%21; p95 (3,44 ms) ile
  ~%41. 60 Hz'e göre ~%10 ve ~%21.
- Önbellekli koşumun p50'si koşumdan koşuma çok kararlı (±0,04 ms),
  tabanınki değil (±0,6 ms). Karşılaştırma bu yüzden medyan üzerinden
  yapılır; tek koşum sayısı raporlanmaz.
- Önbellekli koşumda p95, p50'nin ~2 katı — uzun bir kuyruk var ve
  kaynağı henüz ayrıştırılmadı (aynı süreçte koşan S/T senaryolarının
  ısınma etkisi olabilir).
- S ve T'de kolon her tekrarda kuruluyor (200/200): tazelik korunuyor ve
  ölçülen iş kurulumu içeriyor. Oradaki fark kolon dışı önbelleklerden
  (rapor, kod, çözülmüş görünüm) gelir.
- Bu sayılar **headless CPU'dur ve gerçek gecikme değildir**; sunum
  (present/vsync), giriş kuyruğu ve fiziksel girdi dışarıdadır. Tek
  makine, tek pencere boyutu.

Taban koşumu elle düzenleme istemez: `olcum-onbelleksiz` bayrağı aynı
kodu önbelleksiz derler (`Cargo.toml`), böylece iki koşum yeniden
üretilebilir ve karşılaştırılabilir kalır.

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

Bu ablation tek koşumdan alındı ve §6.2'deki mühürlü tabandan **önce**
yapıldı; mutlak sayıları değil **büyüklük sırasını** okuyun. Beş koşumlu
taban, tek koşumun ~%50 iyimser olduğunu gösterdi — buradaki paylar da
aynı yönde kayıyor olabilir. Kararı değiştirmiyor: her iki bölge de
kare maliyetinin küçük bir dilimi.

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

## 8. Gerçek pencere ölçümü (24 Ağu 2026) — iddia kapanmadı, tersine döndü

Ölçüm modu kullanıcı tarafından gerçek klavyeyle koşuldu. Sonuç, headless
tablonun ürün gerçekliğini **temsil etmediğini** gösteriyor.

### 8.1 Sayılar (macOS, **release**, 1600×1000, 35 sn gerçek yazma)

Düzeltilmiş ölçüm penceresiyle (§8.2.1) ve **optimize** derlemeyle alınan
ilk geçerli koşum:

```
ortam             1600×1000 px · ölçek 1,0× · a11y kapalı · derleme optimize
girdi→draw sonu   n=478  p50 17,22 ms · p95 26,77 ms · p99 32,65 ms · ort 18,68 ms
çizim (draw)      n=482  p50 16,89 ms · p95 19,37 ms · p99 27,77 ms · ort 16,84 ms
sunum aralığı     örnek yok
kare başına olay  n=478  p50 1,0 · p95 2,0 · p99 3,0 · ort 1,09
çizim ortasında düşen olay: 0
aşama (ortalama)  render gövdeleri 2,21 ms · render sonrası draw aşamaları
                  14,63 ms · toplam draw 16,84 ms
```

Okuma:

- **Girdi→draw sonu p50 17,2 ms.** 60 Hz ekranda ~1 kare. Önceki
  `debug_assertions` açık ölçüm 29–32 ms diyordu; **o rakamın yaklaşık
  yarısı doğrulama maliyetiymiş** (§8.2.2). 120 Hz bütçesine (8,33 ms)
  göre hâlâ ~2 kat, 60 Hz bütçesine (16,7 ms) göre sınırda.
- `mid_draw_events_dropped: 0` ve kare başına ortalama 1,09 olay: sayılar
  eksik değil, yalnız yoğun yazmada birkaç olay aynı karede birleşmiş.
- **Sunum aralığı yine örneksiz.** GPUI bu histogramı yalnız pencere
  animasyon yaparken doldurur; tezgâh girdi başına çizer, boşta çizmez.
  "FPS" bu uygulamada ölçülebilir bir büyüklük değil — doğru metrik
  girdi→kare gecikmesidir.
- Örneklem güçlü (n≈478), yani bu tablo tek koşumluk gürültüden ibaret
  değil. Yine de tek makine, tek koşum: A/B–B/A tekrarları §8.3'te.

### 8.2 Farkın kaynağı: elenenler ve kalan

Teşhis satırı ölçüm moduna eklendi ve iki olağan şüpheli **elendi**:

| Hipotez | Sonuç |
|---|---|
| Retina (2× ölçek → 4× piksel) | **Elendi** — ölçek 1,0× |
| Erişilebilirlik ağacı her karede | **Elendi** — a11y kapalı |
| Derleme profili (hata ayıklama) | **Elendi** — `release` koşumu da p50 ~20,1 ms verdi |
| `draw` içinde sunum/vsync beklemesi | **Elenmedi ama zayıf** — `Window::draw` present çağırmıyor (`needs_present` işaretliyor); yine de 20,8 ms, 60 Hz aralığına (16,7 ms) şüpheli yakın |
| Platform metin ve GPU katmanı | **Kalan ana aday** — headless koşumda CoreText de Metal atlası da hiç çalışmıyor |

### 8.2.1 Ayrıştırma — GERİ ÇEKİLDİ (ölçüm kusurlu)

Bu bölüm bir ayrıştırma sayısı ("tezgâh %11 / GPUI+platform %89") ve ondan
türetilen bir çıkarım ("üç tur yalnız %11'e dokundu, toplam kazanç ~%5")
taşıyordu. **İkisi de geri çekildi.** Bağımsız inceleme dört kusur
buldu ve hepsi geçerli:

1. **Payda düzeltildi, pay düzeltilmedi.** Ölçüm penceresi kare
   *sayısından* açılış karelerini düşüyor ama histogramın *toplam
   süresinden* düşmüyor. Kodun kendisi bunu biliyor ve `çizim ≤…`,
   `pay ≥…` diye basıyor; rapor o işaretleri yok sayıp kesin `21,7 ms` ve
   `%11/%89` yazmıştı. Bu bir okuma hatasıydı.
2. **Kapı yanlış olayı bekliyor.** "İlk tuş vuruşu" sandığım şey ilk
   *geçersizleştiren platform girdisi*; metin alanına yapılan bir fare
   tıklaması da sayacı başlatır. Ayrıca başlangıçta histogramlar
   sıfırlanmadığı için sonuçlar pencere açılışından veri taşıyor.
3. **Kalan pay "GPUI'nin malı" değil.** O dilimin içinde **tezgâhın
   ürettiği ağacın** yerleşim, prepaint, paint ve shaping maliyeti var.
   Sağ kolon önbelleği tam da o aşamaları — `reuse_prepaint` /
   `reuse_paint` — yeniden kullanıyor. Yani "üç tur yalnız kendi %11'ine
   dokundu" cümlesi yanlış: önbellek `render` gövdesinin dışındaki işi de
   kesiyor.
4. **`girdi→kare` ekran pikseli değil.** Ölçüm `present_end`e, yani
   platformun `draw` çağrısının tamamlanmasına kadar. "Ekrana yansıma"
   ifadesi bundan daha kesin bir şey iddia ediyordu.

**Ölçüm aracı bu dört maddeye göre düzeltildi** (kod tarafı hazır, sayı
bekliyor):

- Pencere başında dört histogramın da anlık görüntüsü alınıyor ve sonda
  `Histogram::subtract` ile fark hesaplanıyor; çıkarma başarısız olursa
  rapor bunu **yazıyor** (sessiz yanlış sayı üretmiyor).
- Kapı artık gerçek **metin düzenlemesine** bağlı
  (`düzenleme_sayısı`, `DüzenlemeMetniDeğişti` olaylarını sayar); fare
  tıklaması ölçümü başlatmıyor.
- Aşama ayrımı ortalamalar üzerinden yapılıyor ve satır adı
  **"render gövdeleri / render sonrası draw aşamaları"** — sahiplik
  iddiası taşımıyor.
- Gecikme satırının adı `girdi→draw sonu` oldu; "ekrana yansıma"
  ifadesi kaldırıldı.

Deney tasarımı da düzeltildi: iki binary **A/B ve B/A sırasıyla**, birkaç
çift koşumda çalıştırılmalı (§8.3).

### 8.2.2 Ölçüm profili: sayılar `debug_assertions` **açıkken** alındı

Gerçek pencere ölçümleri `akici-dev` profilinde koşuldu ve o profil
`inherits = "dev"`, yani **`debug_assertions` açık** — ölçüm modunun
teşhis satırı da bunu "derleme hata ayıklama" diye yazıyor. GPUI bu
bayrakla element arenası, dispatch ağacı ve çizim fazı için ek
doğrulamalar koşar.

Headless koşum aynı kodu iki profille ölçtü (aynı makine, 200 tekrar):

| Profil | K · tuş vuruşu p50 | Fark |
|---|---|---|
| `akici-dev` (`debug_assertions` açık) | 1,75 ms | — |
| `release` (kapalı) | **1,04 ms** | **%40 daha ucuz** |

Yani §8.1'in 21,7 ms'lik `draw` rakamı ürün derlemesini temsil etmiyor;
üstündeki payın ne kadarının doğrulama olduğu **henüz ölçülmedi**, çünkü
gerçek pencerede ısınmış bir `release` koşumu alınmadı (girdisiz koşum
~20 ms verdi ama o yalnız açılış kareleriydi).

Bu, dışarıyla karşılaştırma yaparken de belirleyici: bir başka GPUI
uygulamasının yayımlanmış "8,4 ms" gibi bir kare süresi büyük olasılıkla
`release`tir ve bizim 21,7 ms'lik `akici-dev` sayımızla **doğrudan
kıyaslanamaz**.

### 8.3 Üç turluk optimizasyonun gerçek penceredeki etkisi

Bu bölüm önce bir tahmin ("toplam gecikmeye yansıması ~%5") taşıyordu;
§8.2.1 geri çekilince dayanağı kalmadı ve geri çekildi. Yerine iki ayrı
binary'nin A/B–B/A koşumu kondu. **O deney de sonuç vermedi** — ama
verdiği bilgi kendi başına değerli, o yüzden kaydı duruyor.

#### 8.3.1 Ayrı binary'lerle A/B–B/A — GEÇERSİZ (gürültü etkiden büyük)

Dört koşum, release, 1600×1000, her biri 25 sn gerçek yazma. Sıra ABBA;
koşumlar arası 10 sn ara.

| Koşum | Hâl | n (girdi) | n (kare) | çizim p50 | çizim ort | render gövdeleri | render sonrası |
|---|---|---|---|---|---|---|---|
| 1 | A önbellekli | 51 | 92 | 18,4 ms | 18,8 ms | 1,35 ms | 17,5 ms |
| 2 | B taban | 354 | 369 | 17,6 ms | 18,4 ms | 3,25 ms | 15,1 ms |
| 3 | B taban | 52 | 56 | 23,6 ms | 22,8 ms | 4,96 ms | 17,8 ms |
| 4 | A önbellekli | **1** | 142 | 18,5 ms | 17,6 ms | 0,35 ms | 17,3 ms |

Kare sayısıyla ağırlıklandırılmış hâli: A 18,10 ms, B 18,96 ms.

Karşılaştırma bu veriyle **kurulamaz**, üç nedenle:

1. **Gürültü tabanı, aranan etkiden büyük.** Aynı binary'nin (B) iki
   koşumu arasındaki `çizim` ortalaması farkı **4,42 ms**. A ile B
   arasındaki fark ise **0,86 ms**. Baskın değişken derleme değil,
   koşumun kendisi — termal durum, arka plan yükü, pencere konumu.
2. **Örneklem sayıları uyuşmuyor.** Girdi sayısı 51 / 354 / 52 / 1. Elle
   yazma temposu koşumdan koşuma beş kat değişmiş; bu, önbelleğin ıska
   veren kareleriyle isabet eden kareleri arasındaki karışımı da
   değiştirir. 4. koşumun gecikme satırı (n=1) hiçbir şey söylemez.
3. **Rapor edilen `n`ler zaten uyarıyordu.** Araç bunları basıyor; sayıyı
   yok sayıp ortalamaları yarıştırmak §8.2.1'deki hatanın aynısı olurdu.

Yine de veride **tutarlı tek bir sinyal** var: `render gövdeleri` dilimi
dört koşumda hiç örtüşmüyor.

| Hâl | render gövdeleri (koşum başına) |
|---|---|
| A önbellekli | 1,35 ms · 0,35 ms |
| B taban | 3,25 ms · 4,96 ms |

Bu, önbelleğin **beklendiği yerde iş kestiğini** gösteriyor: kolon
render edilmediğinde `render` gövdelerinin kare başına maliyeti ~2,7 ms
düşüyor. Aynı tasarrufun `toplam draw`da görünmemesinin nedeni etkinin
yok olması değil, gürültünün beş kat büyük olması.

`render sonrası draw aşamaları` için ise veri **hiçbir yön göstermiyor**:
A 17,3–17,5 ms, B 15,1–17,8 ms — aralıklar iç içe. §9'daki 3. maddenin
(önbellek `reuse_prepaint`/`reuse_paint` ile render dışındaki işi de
keser) bu ölçümde ne doğrulaması ne de yalanlaması var.

#### 8.3.2 Doğru düzenek: tek süreç içinde dönüşümlü fazlar

Ayrı koşumlar baskın gürültüyü ölçüme sokuyorsa, çözüm koşum sayısını
artırmak değil, değişkeni ortadan kaldırmaktır. Önbellek anahtarı
derleme zamanı `cfg`'den **çalışma zamanı bayrağına** taşındı
(`önbelleği_ayarla`, `paneller.rs`); `olcum-onbelleksiz` özelliği artık
yalnız bu bayrağın **varsayılanını** ters çeviriyor, yani durağan taban
derlemesi ve mevcut bekçiler olduğu gibi duruyor.

Bunun üstüne ikinci bir ölçüm kipi kuruldu: `--olcum-ab <saniye>`.

- Koşum beşer saniyelik fazlara bölünür, sıra **ABBA** (doğrusal kayma —
  ısınma, termal kısma — iki kovaya eşit dağılsın diye).
- Her faz başında bayrak çevrilir, `refresh` edilir ve **300 ms
  oturulur**: geçiş karesi zorunlu ıskadır, hiçbir kovaya girmez.
- Sonra `draw` histogramının ve `render` toplamının anlık görüntüsü
  alınır; faz sonunda fark hesaplanıp ilgili kovaya eklenir.
- Termal durum, arka plan yükü ve yazma temposu artık iki kovaya da eşit
  dağılır — çünkü aynı dakikanın içindedirler.

```bash
cargo run --release --features olcum \
    -p gpui-bilesenleri-galeri-masaustu -- --geniş --olcum-ab 80
```

Mutlak sayılar (gecikme, kare maliyeti, aşama ayrımı) için `--olcum <sn>`
kipi olduğu gibi duruyor; iki kip iki ayrı soruya bakar ve birbirinin
yerine geçmez.

### 8.4 Kayıtlı sonuç

Bu ölçümden sonra depo için geçerli tek cümle şudur:

> 1600×1000 pencerede, 60 Hz ekranlı bir macOS makinesinde, **optimize**
> derlemede, bir metin düzenlemesinden o düzenlemenin yol açtığı `draw`
> çağrısının tamamlanmasına kadar geçen süre **p50 17,2 ms** ölçüldü
> (n=478, düzeltilmiş ölçüm penceresi). Bu, ekrandaki pikselin değiştiği
> an değildir; sunum kuyruğu ve panel gecikmesi bunun dışındadır.
> Sürenin `draw` içi **aşama** dağılımı ortalamada 2,21 ms render
> gövdeleri / 14,63 ms render sonrası aşamalar — ama bu bir **sahiplik**
> ayrımı değildir (§8.2.1, madde 3).

`debug_assertions` açıkken ölçülen eski 29–32 ms'lik rakam ürünü temsil
etmiyordu; farkın yaklaşık yarısı doğrulama maliyetiymiş (§8.2.2).

120 FPS iddiası kapanmadı; ölçüm onu **çürüttü**. 60 Hz bütçesine
(16,7 ms) göre p50 sınırda, 120 Hz bütçesine (8,33 ms) göre ~2 kat.

## 9. İnceleme düzeltmeleri (24 Ağu 2026, bağımsız salt-okunur inceleme)

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

## 10. Sıradaki işler (öncelik sırasıyla)

Gerçek pencere ölçümü (§8) sırayı değiştirdi: artık en değerli iş, 20,8
ms'lik `draw`ın içini ayrıştırmak. Platformlar arası tekrar ondan sonra
gelir — yanlış katmanı optimize etmemek için.

1. **Optimizasyonun gerçek penceredeki etkisi** (§8.3): ayrı binary'lerle
   A/B–B/A denendi ve **geçersiz çıktı** — koşumlar arası gürültü (4,42 ms)
   aranan etkiden (0,86 ms) beş kat büyüktü. Düzenek değiştirildi: önbellek
   artık çalışma zamanı bayrağı ve `--olcum-ab` kipi iki hâli tek süreç
   içinde dönüşümlü ölçüyor (§8.3.2). Koşum bekliyor.
2. **`draw`ın render sonrası diliminin içi.** Ağırlığın orada olduğu
   ölçüldü (§8.1: ort 14,63 ms / 16,84 ms) ama o dilimin içi
   (yerleşim / prepaint / paint / shaping / rasterizasyon / sahne kodlama)
   ayrılmadı. Dilim **sahiplik değil aşamadır**: içinde hem GPUI'nin hem
   tezgâhın ürettiği ağacın işi var. Tezgâhın oradaki payına dokunan tek
   kaldıraç önbellektir (`reuse_prepaint`/`reuse_paint`) ve etkisi
   1. maddede ölçülüyor. Geri kalanı kardeş depoların işi (`gpui`,
   `gpui_apple`); araç hazır: GPUI'nin `profiler` izleri ve Instruments.
3. **Linux'ta aynı ölçüm** — hem headless hem gerçek pencere. İki kural
   yürürlükte: mutlak süreler platformlar arası yarıştırılmaz;
   karşılaştırma aynı makinede çift koşumla yapılır
   (`olcum-onbelleksiz`).
4. **`ORT-018 bil-010.input.commit`:** kabul motorunu ölçer; bu turlardan
   etkilenmedi, sayı gerilememeli.
5. **Headless koşumdaki p95 kuyruğu** (§6.2): p50'nin ~2 katı, kaynağı
   ayrıştırılmadı. Gerçek pencere ölçümünde `çizim süresi` p95'i (33,1 ms)
   de benzer bir kuyruk gösteriyor — ikisi aynı olgu olabilir.

Kayıtlı sınır: bu depo için **120 FPS ya da "sıfıra yakın gecikme" iddiası
yoktur ve ölçüm bu iddiayı çürütmüştür** (§8). Elde olan iki ayrı sayıdır:
headless CPU işi (odak sonrası tuş vuruşu p50 ~1,75 ms — yalnız element
ağacı katmanı) ve gerçek pencerede girdi→kare gecikmesi (p50 ~32 ms, 60 Hz
ekran). İkisi aynı şeyi ölçmez ve biri diğerinin yerine kullanılamaz.

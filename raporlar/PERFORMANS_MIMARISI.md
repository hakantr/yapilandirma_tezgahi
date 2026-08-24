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
   (`cached` bölgeler) doğruluk ön koşulu hazırlandı.

## 4. İkinci tur: sağ kolon bölüm paneli — TAMAMLANDI (24 Ağu 2026)

Hedef gerçekleşti: tuş vuruşu karesinde sağ kolonun element ağacı **hiç
kurulmuyor** — prepaint/paint aralıkları önceki kareden yeniden
kullanılıyor.

### 4.1 `BölümlerPaneli` (`src/paneller.rs`)

- Sağ kolonun tamamı tek önbellekli entity:
  `Entity::cached(StyleRefinement …flex_1().min_w(0).min_h(0))` sınırında
  gövdeye girer (`Tezgahİçeriği.yapılandırma: AnyElement`). Kolon kaydıran
  bir kap olduğu için boyutu içerikten bağımsızdır — `cached`ın "stilden
  yerleşir, içerikten ölçülmez" kısıtına doğal uyar. Bölüm **kartları**
  içerik yükseklikli olduğundan kart başına önbellek kurulamaz; sınır
  kolon düzeyindedir.
- Panel **kökü gözler** (`observe(kök) → notify`): tercih, tema, açık
  seçici ve dış bildirim kökten `notify` ile aktığı için önbellek tam
  gerektiğinde patlar. Alanı gözlemez — tuş vuruşları kolona işlemez.
- **Listener'lar yeniden yazılmadı.** Panelin çizimi kökü `update` ile
  açar ve bölümleri kökün kendi bağlamında üretir
  (`GaleriUygulaması::tezgah_bölümleri` → profilin `bölümler()`i);
  karttaki 16 `tezgahı_değiştir` dinleyicisi köke bağlı kalır. Çizim
  sırasında kök kiralı değildir (GPUI, kök render'ı bitip element ağacı
  yerleşirken alt view'ları çizer), bu yüzden `update` güvenlidir.
- Bölüm listesinin tek kaynağı `tezgah_bölümleri`: ekrandaki panel de
  `tezgah_profil.rs` tür süzgeci testleri de oradan okur.

### 4.2 Kaynak okumasıyla doğrulanan üç önbellek mekaniği

- **Kaydırma:** GPUI, kaydırma ofsetini değiştirirken kaydıran öğenin
  view'ını bildirir (`div.rs paint_scroll_listener → cx.notify(current_view)`)
  — kolonu kaydırmak önbelleği kendiliğinden patlatır.
- **İç entity'ler:** kolona gömülü tercih kutuları (desen, ön ek, son ek)
  bildirim yayımlayınca GPUI ataları da kirletir (`mark_view_dirty`) —
  onlara yazmak kolonu tazeler.
- **Açık listeler:** `deferred` çizimler ve fare dinleyicileri
  `reuse_prepaint`/`reuse_paint` ile taşınır — kolondaki açık bir seçici,
  önbellekten gelen karelerde de ekranda kalır ve tıklanabilir.

### 4.3 `YuvaNotuPaneli`

Kökün çizim yolundaki **son alan okuması** (`yuva_görünürlük_notu`,
"kutu boş mu?") kendi gözleyen entity'sine taşındı; kabuk yuvaları kartı
notu panel olarak gömer. Kökün çizim yolunda artık alan okuması yoktur —
sol kolon bir gün önbelleğe alındığında da bayatlayacak bir şey kalmadı.

### 4.4 Kabuk sınırındaki değişiklik

`Tezgahİçeriği.bölümler: Vec<TezgahBölümü>` kalktı; yerinde
`yapılandırma: AnyElement` var. Akış ayrıştırma `arayuz.rs`'te serbest
fonksiyona indi; kolon gövdesi `govde.rs`'te
(`yapılandırma_kolonu_gövdesi`, kabuk tarafı) durur ve panel onu çağırır.
Kabuk yine hiçbir bileşen tipini tanımaz.

### 4.5 Doğrulama

- 208 test yeşil (206 + 2 yeni bekçi: kolon önbellekli **ve** kökü gözler;
  yuva notu panel bağlamında).
- WASM'de elle: yazarken paneller canlı, kolon dokunulmadan duruyor; tür
  ve biçim değişimi kolonu tazeliyor; **açık biçim listesi, alana
  yazılan karelerde açık kaldı ve öğesi tıklanabilir kaldı** (deferred +
  listener taşınması); kolon içi ön ek kutusuna yazma çalışıyor; kaydırma
  akıcı; koyu kip bütün kolona işliyor.

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
paneller. Sağ kolon kurulmaz, hiçbir liste kurulmaz, hiçbir çözüm/rapor/
kod yeniden hesaplanmaz.

## 6. Kabul hedefleri ve ölçüm

Önceki turun sayısal hedefleri devirde kaybolduğu için hedefler yeniden,
ölçüm yoluyla birlikte konur:

1. **Tuş vuruşu gecikmesi (uçtan uca):** `akici-dev` profilli masaüstü
   koşumunda GPUI profiler'ı (`gpui` `profiler` özelliği /
   `debug_frame_overlay`) ile kare süresi. İkinci turun yapısal hedefi —
   tuş vuruşu karesinde sağ kolonun render edilmemesi (`dirty_views`
   yalnız alan + paneller) — koda bağlandı; sayısal karşılığı henüz
   ölçülmedi.
2. **`ORT-018 bil-010.input.commit`:** tezgâhtaki yerleşik ölçüm
   (`ölçüm_toplu_ms`) iki hedefte de koşturulabilir; kabul motoru bu
   turdan etkilenmedi, sayı gerilememeli.
3. **Linux ölçümü:** kullanıcının Linux düğümünde `akici-dev` ile
   masaüstü koşumu (birinci turda yalnız macOS + WASM'de doğrulandı;
   Linux koşumu bekliyor).

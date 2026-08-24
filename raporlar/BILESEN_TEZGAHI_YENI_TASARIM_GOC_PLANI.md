# Bileşen Tezgahı Yeni Tasarımı — Göç Planı

> Nitelik: Normatif olmayan çalışma planı
> Sürüm: **8** · 20 Ağustos 2026 (I.–VII. denetim turu bulgularıyla revize edildi)
> Belge yolu: `raporlar/BILESEN_TEZGAHI_YENI_TASARIM_GOC_PLANI.md`
> Girdi tasarımı: `Tezgah_yeni_tasarimi/TEZGAH_YAPIM_PLANI.md` (Bölüm I + Bölüm II),
> `Yerlesim Raporu.dc.html`, `Sozlesme Uyum Listesi.dc.html`, `screenshots/`
> Bağlayıcı kaynak: `sozlesmeler/` altındaki yaşayan kanonik gövdeler.
> Başlıca doğrudan sahipler: `BİL-010 13.0.0`, `YÖN-002/005`,
> `YÖN-006 1.1.7`, `ORT-002/003/004/006/008/016/017/019/021/023`.

---

## 0. Kullanıcı kararları (bu planın çerçevesi)

| Karar | İçerik |
|---|---|
| **Kapsam** | Yalnız `BİL-010` yeni tezgâha geçer. Bütün aileler zaten yeniden tasarlanacak; toplu göç yapılmaz. Her aile **yeniden yazılırken** bu tezgâha taşınır. |
| **Yol** | Yerinde yeniden yazım. Mevcut tezgâh sergisi paralel yaşatılmaz. |
| **Eksen kümesi** | Fazlı: önce yüz ve düzen, sonra `§16.2`, sonra kalan eksenler. |
| **F2 sınırı** | Tezgâh yalnız salt-okunur **geliştirici gözlem paneli** sunar; ORT-019 tanı zarfı üretmez. Canlı `SatırSonu` göstergesini **K1 tek başına açar**; K2 üst-köşe yolunu, K3 açıklama yüzeyini ekler. Üçü de bu göçün kapsamı dışındadır (F2.6). |
| **Bayat raporlar** | `Yerlesim Raporu` ve `Sozlesme Uyum Listesi` **yeniden üretildi** (III. turda tamamlandı). |

Mimari sonucu: **tezgâh `BİL-010`'a ait bir ekran değil, bileşen-bağımsız bir
kabuktur.** `BİL-010` onun ilk profilidir (§2).

### 0.1 I. denetim turu — bulgu kapanışı

| # | Bulgu | Doğrulama | Kapanış |
|---|---|---|---|
| 1 | [P0] F2 galeride ikinci çözücü kuruyor | **Doğru.** §16.2 tablosunun altında: "`durum_göstergesi_durumu`, bu opak ve ödünç güncel sonucu gözlemlemenin tek kamusal yoludur; ikinci bir sonuç fabrikası veya serbest kurucu yoktur." | F2 yeniden yazıldı; ikinci çözücü kalktı. *(Bu satırdaki "kanonik API zaten var, kapılı faz gerekmiyor" değerlendirmesi II. turda geri alındı — bkz. §0.2.)* |
| 2 | [P0] F2 kabul matrisi 5 yol, kanonik tablo 7 gerekçe | **Doğru, fakat eksik.** Kanonik *uygulama* bugün 7'den yalnız **4'ünü** üretiyor (`api.rs:2287–2308`, üst-köşe adayı fail-closed). | Matris 7 satıra çıkarıldı; üretilebilen 4 canlı sınanır, kalan 3 "aday beslemesi yok" gerekçesiyle pasif (§6 F2.3). |
| 3 | [P1] ORT-017 metrik sahipliği kurulmamış | **Doğru.** `ORT-004.ACC-001`: "bileşene özgü metrik yalnız `ORT-017` tipli profilinde tek sahipli olabilir." | `TezgahGörünümProfili` eklendi (§3.3); `yuzler.rs` sabit taşımaz. |
| 4 | [P1] Rota kararı çelişkili | **Doğru.** §1 "gömülü kalır" derken §8 "ayrı rota" öneriyordu. `GaleriSayfası` bugün yalnız `GenelBakış`/`Aile`. | Tek karara bağlandı: **gömülü kalır**, `min_w` kalkar (§1.3). |
| 5 | [P1] Dar yerleşim tarifi kendi içinde çalışmıyor | **Doğru.** `min_w(1216)` ile `<1216` sorgusu aynı kapta çelişir. | §4 yeniden yazıldı: sorgu dış kapta, `min_w` yok. |
| 6 | [P2] Envanter bayat | **Doğru.** 49 alan (45 değil), tezgâh kodu `sergiler.rs:800–2990`, `close-circle` zaten kayıtlı (`simgeler.rs:27`). | §1.1 ve §7 düzeltildi. |
| 7 | Açık nokta 2 kapatılabilir | **Doğru.** `pub use accesskit::{Orientation, Role, Toggled}` (`gpui.rs:93`); `Role::SpinButton` GPUI içinde kullanımda. | Açık nokta listesinden çıkarıldı. |

> **Bu paragraf II. turda geri alınmıştır — §0.2/1'e bakınız.**
>
> ~~`DurumGöstergesiDurumu` ve yedi varyantlı gerekçe kanonik kodda fiziksel;
> eksik olan yalnız `GirişYüzeyBağı`, bu yüzden F2 bütünüyle kapılı bir faz
> değil.~~ API'nin fiziksel olması doğruydu, ama `GirişKutusu::render`
> göstergeyi çizmediği için tezgâhta **canlı** gösterge yine de kurulamaz.

### 0.2 II. denetim turu — bulgu kapanışı

| # | Bulgu | Doğrulama | Kapanış |
|---|---|---|---|
| 1 | [P0] "yalnız `GirişYüzeyBağı` eksik" sonucu fiziksel kodla uyuşmuyor: `GirişKutusu::render` göstergeyi **hiç çizmiyor** | **Doğru.** `bileşen.rs:2146` `Render` bloğunda mantıksal sıra `ön ek → içerik → son ek → sayaç → yardımcı eylemler` ile bitiyor; `render` gövdesinde `gösterge` geçmiyor. | F2 tümüyle yeniden yazıldı (§6 F2). Canlı gösterge **kanonik atom borcu** olarak ayrıldı; galeri yalnız salt-okunur geliştirici gözlemi çizer. *(VI. turda “tanı paneli” adı ORT-019 zarfıyla karışmaması için değiştirildi.)* |
| 2 | [P0] `GirişYüzeyBağıEksik` fiziksel `GirişYapılandırmaHatası`nda yok | **Doğru.** `api.rs` içinde varyant bulunmuyor. | "Kanonikte beklenen, fiziksel API'de bulunmayan sonuç" etiketi kondu (§6 F2.4). |
| 3 | [P1] ORT-017 kaydı yüzeysel: gerçek kimlik/anatomi/kayıt/çözüm gösterilmemiş | **Doğru.** `görünüm.rs`'de `GörünümProfiliKimliği`, `BileşenGörünümTanımı`, `GörünümKayıtDefteri::anatomi_kaydet`, `ÇözülmüşGörünümProfili` fiziksel olarak var. | §3.3 gerçek yüzeye bağlandı; kabul koşumu eklendi. |
| 4 | [P1] `min_w(1216)` ve balon simülasyonu göç planında düzeltildi ama girdi kaynakları hâlâ eskisini tarif ediyor | **Doğru.** | §0.4 geçersiz-kılma tablosu eklendi. |
| 5 | [P2] Girdi klasörünün dosya statüleri sınıflandırılmamış | **Doğru.** | §0.3 girdi sicili eklendi. |
| 6 | [P2] `screenshots/genel.png` ve `s23.png` bayt düzeyinde aynı, ikisi de `.png` uzantılı JPEG | **Doğru.** Aynı md5 (`f16d64d2…`), `JPEG 924x540`. | Sicilde işaretlendi; `s23` bağımsız kanıt sayılmıyor. |
| 7 | [P2] "Çalışma ağacı temiz" ifadesi yanlış | **Doğru.** `git status`: `?? Tezgah_yeni_tasarimi/`, `?? raporlar/BILESEN_TEZGAHI_YENI_TASARIM_GOC_PLANI.md`. | §12 düzeltildi. |
| 8 | [P2] `Tezgah.dc.html` künyesi tutarsız | **Doğru.** Aynı dosyada `13.0.0` (2 kez) ve `4.0.0` (1 kez) geçiyordu. | Önce yalnız sicilde işaretlendi; **VII. turda kaynak yorumundaki `4.0.0` → `13.0.0` yapılarak doğrudan kapatıldı** (§0.9/8). |
| 9 | [P2] Girdi klasörü adı özel karakter ve boşluk taşıyor | **Doğru.** Eski ad dosya yolu ve araç çağrılarında Unicode normalleştirmesine bağımlıydı. | Klasör `Tezgah_yeni_tasarimi/` olarak yeniden adlandırıldı; plan içindeki etkin yol atıfları güncellendi. |
| 10 | [P2] Plan dosyası adı yalnız `BİL-010` tezgahına aitmiş izlenimi veriyor ve özel karakter taşıyor | **Doğru.** Kabuk bileşen-bağımsızdır; `BİL-010` yalnız ilk profildir. | Dosya `BILESEN_TEZGAHI_YENI_TASARIM_GOC_PLANI.md` olarak yeniden adlandırıldı; belge başlığı genelleştirildi. |

**I. turun "denetimin kaçırdığı nokta" tespiti geri alınmıştır.** API'nin fiziksel
olması doğruydu, ama yeterli değil: kanonik `render` göstergeyi çizmediği için
"galeri sonucu okur ve çizer" yaklaşımı galeriye ait **ikinci bir görsel
uygulama** olurdu. Bu, planın kendi §1.2 hükmüyle (kanonik davranış değişmez) ve
`YÖN-006.ACC-006` ile bağdaşmaz. II. turun ayrımı doğrudur ve benimsenmiştir.

---

### 0.3 Girdi sicili — `Tezgah_yeni_tasarimi/` klasörü

> **Yol düzeltmesi:** Eski `Tezgâh yeni tasarımı/` adı özel karakterlerden
> arındırılıp boşlukları alt çizgiye çevrilerek `Tezgah_yeni_tasarimi/` yapıldı.
> Bu rapordaki etkin dosya yollarında yeni ad kullanılır.

Her dosyanın bu plandaki statüsü. **Hiçbiri bağlayıcı kaynak değildir**;
bağlayıcı olan yalnız `sozlesmeler/` altındaki yaşayan kanonik gövdelerdir.

| Dosya | Statü | Not |
|---|---|---|
| `TEZGAH_YAPIM_PLANI.md` | **Uygulama girdisi** | Bölüm I tasarım kararları, Bölüm II GPUI eşlemesi. Geçersiz kılınan bölümleri §0.4'te. |
| `Tezgah.dc.html` | **Görsel referans** (asıl teslim) | Künye VII. turda `BİL-010 13.0.0` olarak tekleştirildi. Yine de yalnız görsel dil örneğidir; dosyadaki çizilmiş canlı gösterge/açıklama yüzeyi bugünkü fiziksel Rust davranışının veya kabul kanıtının temsili değildir (§0.4). |
| `Yerlesim Raporu.dc.html` | **Görsel referans · 2. sürüm, güncel** | Ölçü dökümü §4.1'e alındı. §3/§5/§6/§7 yeniden üretildi: balon simülasyonu kalktı, `GirişYüzeyBağıEksik` kanonik adı kondu, üç gerekçenin bugün ölçülemediği ve render'da gösterge olmadığı yazıldı. |
| `Sozlesme Uyum Listesi.dc.html` | **Tarihli snapshot · 3. sürüm, güncel** | Üç temsil boşluğu da kapandı (`dış_tıklamada_odağı_bırak`, `üzerine_yazma`, `SayısalTekerlekDavranışı` fiziksel). `§16.2` satırı koşullu · kanonik render borcudur; yüzey ORT-019 tanısı üretmeyen geliştirici gözlemi olarak künyelendi. Sayım `26+3 → 29+1`. |
| `sozlesme-taslagi-gosterge-rezervi-kaldirma.md` | **Tarihsel taslak** | Rezervin kaldırılması kanoniğe aktarılmıştır. Güncel hüküm kaynağı **değildir**: taslak `None ↔ Some` değişimini `AnatomiKaydı` sayıyor, kanonik metin bunun `AnatomiKaydı` olmadığını, en fazla `YerleşimVeBoya` geçersizleştirmesi olduğunu söylüyor (`metin_girişi…md:1847`). |
| `sozlesme-taslagi-durum-gostergesi.md` | **Tarihsel taslak · yerine yenisi geçti** | `BİL-010 ^1.0`'a öneri; dört gerekçeli eski model taşıyor. Yürürlükteki metin yedi gerekçeli `§16.2–16.2.5`'tir. |
| `github.md` | **Kaynak künyesi · exact** | Bölüm II'nin GPUI tabanı: yerel commit `1995423061…`, kaynak Zed commit'i `cef06d351b…`; tüketilen `crates/gpui/` kapsamı temiz. Yerel deponun kapsam-dışı üç kirli yolu ayrıca yazılıdır. |
| `screenshots/genel.png` | **Görsel referans** | `.png` uzantılı JPEG, 924×540. |
| `screenshots/s23.png` | **Bağımsız kanıt değil** | `genel.png` ile **bayt düzeyinde aynı** (md5 `f16d64d2…`). Yeniden alınmalı (§10/2). |
| `support.js`, `doc-page.js` | **Üretilmiş destek dosyası** | Design Component çalışma zamanı; uygulama girdisi değil. |
| `Canvas.dc.html` | **Kapsam dışı** | Boş `x-dc` kabuğu. |
| `.thumbnail`, `.DS_Store` | **Kapsam dışı** | Üretilmiş önizleme / işletim sistemi metadatası. |

---

### 0.4 Geçersiz-kılma tablosu — girdi tasarımı vs. bu plan

Girdi kaynakları aşağıdaki noktalarda **kanonik sözleşmeyle veya fiziksel kodla
çelişiyor**. Çelişkide bu plan geçerlidir; girdi kaynağı tarihsel kalır.

| Girdi kaynağı ve yeri | Ne diyor | Neden geçersiz | Yerine |
|---|---|---|---|
| `TEZGAH_YAPIM_PLANI` §II.7 `gosterge_cozumu()` | Tezgâh kendi çözücüsünü kurar | `§16.2`: `durum_göstergesi_durumu` tek kamusal yol, ikinci sonuç fabrikası yok | Kanonik sonuç okunur (§6 F2) |
| `TEZGAH_YAPIM_PLANI` §7.3, §II.8 · `Tezgah.dc.html` · `Yerlesim Raporu` §6 | Açıklama balonu tezgâhta simüle edilir | `§16.2.4`: "Yerel sahte baloncuk veya sessiz `Yok` fallback'i yoktur" | Balon kurulmaz (§6 F2.4) |
| `Tezgah.dc.html:680–689` | Canlı satır-sonu/üst-köşe gösterge ve kullanıcı iletisi boyalıdır | Fiziksel `GirişKutusu::render` göstergeyi çizmez; exact ORT-006/021 + `GirişYüzeyBağı` yoktur | Yalnız hedef görsel kompozisyon; uygulama/kabul kanıtı değildir (F2.4, F2.6) |
| `TEZGAH_YAPIM_PLANI` §4.3, §II.1 · `Tezgah.dc.html` `.aciklama` | Genel `?` yardımı yerel `<details>`/`deferred(anchored(..))` yüzeyi olarak açılır | ORT-006 aynı penceredeki geçici yüzeyin tek politika sahibidir; metin ORT-021'den çözülür. Exact fiziksel yüzey bugün yoktur | ORT-006/021 kapısı; yerel popup veya tıklama-tabanlı fallback yok (§3.4, F1) |
| `TEZGAH_YAPIM_PLANI` §5 · `Tezgah.dc.html` | Kök `min-width: 1216px` | Gömülü tezgâhta kabı dayatır; `YÖN-006 §3.4` kipi ölçülen alandan seçtirir | `min_w` yok, `container_query` dış kapta (§4) |
| `TEZGAH_YAPIM_PLANI` §6 üst şerit | Tezgâh kendi tema/hedef/ölçek şeridini taşır | `YÖN-006 §4` küresel eksenleri galeri çubuğuna verir; iki otorite olmaz | Şerit kurulmaz (§1.4) |
| `TEZGAH_YAPIM_PLANI` §4.1–4.3 · ham `px`/aile sabitleri | Yüzler sabit ölçü taşır | `ORT-004.ACC-001`: dağınık fiziksel ölçü sabiti yok | `TezgahGörünümProfili` (§3.3) |
| `TEZGAH_YAPIM_PLANI` §4.3 pasif düğme | "opaklık düşürülür" | `GörselOpaklıkKademesi` kanonik imza çelişkisi taşıyor ve yalnız kademeli görünürlük profilinde zorunlu | `ORT-004` `devre_dışı` kutu rolü (§5.5/1) |
| `Yerlesim Raporu` §6 (1. sürüm) | Balon tezgâhta simüle edilir, damga taşır | `§16.2.4` sahte baloncuğu yasaklar | **Rapor yeniden üretildi** (belge hazırlığı · tamam) |
| `Yerlesim Raporu` §7 (1. sürüm) | "Motor, tekerlek ve odak politikası tip düzeyinde kapalı" | Üçü de bugün fiziksel | **Rapor yeniden üretildi** (belge hazırlığı · tamam) |
| `Sozlesme Uyum Listesi` "temsil boşluğu" (1. sürüm) | Üç eksen temsil boşluğu | Üçü de bugün fiziksel | **Liste yeniden üretildi** (belge hazırlığı · tamam) |

---

### 0.5 III. denetim turu — bulgu kapanışı

| # | Bulgu | Doğrulama | Kapanış |
|---|---|---|---|
| 1 | [P0] Tanı paneli yasak veri gösteriyor: sorun kimliği ve değer sürümü | **Doğru.** `§16.2.5`: "Tanı zarfı kullanıcı iletisini, sorun kimliğini, değer sürümünü … taşımaz. Yerleşim gerekçesi kullanıcıya sunulan açıklamanın parçası değildir." | F2.2 yeniden yazıldı: kimlik yerine var/yok, sürümler panelden çıktı. *(Bu satırdaki "sürümler güncellik kontrolünde kullanılır" değerlendirmesi **IV. turda geri alındı**: üçüncü kök alanı private olduğu için karşılaştırma kurulmaz; güncellik ödünç okumayla sağlanır — güncel F2.5.)* |
| 2 | [P1] ORT-017 bağlantısı kanonik olarak kapanmamış | **Doğru.** Kanonik `4.3.0` trait'i `tipografi_parçaları` ve `tipografi_uygulama_grupları` istiyor; fiziksel `görünüm.rs` ikisini de taşımıyor, hata tipi ve alan adı da farklı. `GörünümKayıtDefteri`'nin somut uygulayıcısı **yok**. | §3.3.1 sapma tablosu eklendi; F0 kabulü daraltıldı, kayıt kapısı açıkça kapılı (§3.3.3/5). |
| 3 | [P1] `TextStyle` semantik rol değil | **Doğru.** `TextStyle` `color` + `font_family` + `font_size` taşır (`style.rs:438`). | §3.3.2: profil yalnız rol kimliği taşır; `TextStyle` `çöz()` katmanında üretilir. |
| 4 | [P1] Uyum Listesi tam güncellenmemiş; alttaki "üç temsil boşluğu" bölümü duruyor | **Doğru.** 313–318. satırlar hâlâ üç alanın struct'ta bulunmadığını söylüyor. | Bölüm yeniden yazıldı (bu turda). |
| 5 | [P2] Plan durum ifadeleri çelişkili | **Doğru.** §10 kapanış listesi "yeniden üretilecek" diyordu. | Düzeltildi. |
| 6 | Tanı kartının canlılık bağı tanımsız | **Doğru.** `cx.observe` mevcut (`app.rs:1081`). | Canlılık bağı eklendi. *(Bu satırdaki iki iddia **IV. turda geri alındı**: `lib.rs:1515` `observe` değil `subscribe` kullanır — ayrı kanal; "üç sürümlü güncellik kökü" kurulamaz çünkü `gösterge_girdisi_sürümü` private. Yerine ödünç okuma geldi — bkz. güncel F2.5.)* |
---

### 0.6 IV. denetim turu — bulgu kapanışı

| # | Bulgu | Doğrulama | Kapanış |
|---|---|---|---|
| 1 | [P0] Canlılık sürüm karşılaştırması uygulanamaz: `gösterge_girdisi_sürümü` private, getter yok | **Doğru.** Alan hem kanonik `§16.2` struct'ında hem `api.rs:2225`'te private; kamusal getterlar yalnız `değer_sürümü()` ve `sorun_sürümü()`. | Güncel F2.5 yeniden yazıldı: sürüm karşılaştırması tümüyle kalktı, yerine **ödünç okuma** geldi — panel sonucu saklamaz, bayatlayamaz. |
| 2 | [P0] Ham gerekçe enumunu göstermek sözleşmeye aykırı | **Doğru.** `§16.2.5` gerekçeyi yalnız kayıtlı ORT-019 koduna eşlenebilir kılıyor; `ORT-019`: "Kayıtlı kod kümesi/aralık/değer sınıfı dışındaki girdi `DeğerPolitikasıUyuşmuyor` olur." | Gerekçe satırı **panelden çıkarıldı**. Kod kümesi fizikselleşene kadar gösterilmez; "geliştirici tanısı" etiketi kayıt kapısının yerine geçmez. |
| 3 | [P1] §9 uyum tablosu ORT-017'yi kayıtlı ilan ediyor, §3.3 kapılı diyor | **Doğru.** | §9 satırı hedef/kapılı ayrımıyla yeniden yazıldı. |
| 4 | [P1] Rol tabakası tamamlanmamış; rol tipleri kodda yok | **Kısmen doğru.** Rol tiplerinin kodda olmaması doğruydu. | §3.3.2 ve §4.1 yeniden yazıldı. *(Bu satırın "`58px`'i profile yazmak yanlış" kısmı **V. turda geri alındı**: `ORT-017` yasak listesi `Pixels` içermez ve `ACC-001` metriği zaten profilde tek sahipli ister — sayılar profile geri döndü.)* |
| 5 | [P1] "Dört iş" sınırı fazla geniş | **Doğru.** `§16.2.4`: "`AçıklamaTercihi::Yok` yalnız görsel göstergeyi kullanır." | Güncel F2.6 üç ayrı kapıya ayrıldı: K1 temel render, K2 üst-köşe adayı, K3 açıklama yüzeyi. *("K1 bağımsızdır" değerlendirmesi **V. turda daraltıldı**: K1, K2/K3'ten bağımsızdır fakat ORT-017 temel anatomi/metrik göçüne bağlıdır.)* |
| 6 | Küçük artıklar: gelecek zaman, §10 atıf numaraları, açık karar sayısı, `subscribe`/`observe` karışıklığı | **Dördü de doğru.** | Hepsi düzeltildi; `observe` (bildirim) ile `subscribe` (olay) ayrımı güncel F2.5'te açıkça yazıldı. |
---

### 0.7 V. denetim turu — bulgu kapanışı

| # | Bulgu | Doğrulama | Kapanış |
|---|---|---|---|
| 1 | [P0] Gerekçe panelden bütünüyle çıkmamış: F2.3, F2 kabulü ve risk 12 hâlâ görünür metin iddia ediyor | **Doğru.** Üç yerde de duruyordu. | F2.3 "test kapsamı" olarak yeniden yazıldı; kabul metni negatif iddiaya çevrildi ("panelde gerekçe metni bulunmadığı"); risk 12 düzeltildi. |
| 2 | [P1] Metrik tek sahipliği kapanmamış: `çöz()` gövdesi profil değil | **Doğru.** `ORT-017` yasak listesi ham renk ve **fiziksel font** sayıyor; `Pixels` listede yok. `ORT-004.ACC-001` metriği **profilde** tek sahipli istiyor. | Metrikler profile **geri döndü** (§3.3.2). 4. sürümün aşırı düzeltmesi giderildi; `TextStyle` yasağı korundu. |
| 3 | [P1] K1 bütünüyle bağımsız değil | **Doğru.** `§16.2.1` kayıtlı `ORT-017 AnatomiParçasıSınıfı::Gösterge` parçası, `§16.2.2` aynı snapshot'tan iki bağlayıcı alt sınır istiyor. Fiziksel kodda yalnız eski `ParçaSınıfı::Gösterge`/`GörünümHatası::AnatomiUyumsuz` karşılıkları var; exact kanonik tipler ve iki alt sınır yok. | K1 satırına gerçek bağımlılık yazıldı; §0 karar satırı düzeltildi: canlı `SatırSonu` göstergesini K1 tek başına açar. |
| 4 | [P2] `TipografiRolü` sahibi yanlış | **Doğru.** Tip `ORT-004`'te tanımlı (beş varyant); `ORT-017` `TipografiUygulanabilirParçaKaydı` içinde tüketiyor. | §3.3.2'de sahiplik düzeltildi; tezgâhın beş metin yüzünün beş kanonik role eşlemesi eklendi. |
| 5 | [P2] Tarihsel kapanış kayıtları işaretlenmemiş | **Doğru.** | III. tur satırı 6 ve koşum kaydı "IV. turda geri alındı" künyesi aldı. |

### 0.8 VI. denetim turu — tam yeniden denetim ve bulgu kapanışı

| # | Bulgu | Doğrulama | Kapanış |
|---|---|---|---|
| 1 | [P0] F0 hâlâ bugün derlenebilir görünüyordu | **Doğru.** Fiziksel temel crate'te `TipografiRolü`, `GörselOpaklıkKademesi`, exact `AnatomiParçasıSınıfı`/`GörünümKayıtHatası` yok; ORT-017 kayıt uygulayıcısı da yok. | F0, bağımsız token/simge hazırlığı (**F0a**) ve ORT-004/017 kapılı profil/yüz işi (**F0b**) olarak ayrıldı (§3.3, F0). |
| 2 | [P1] Yerel `?` yardım balonu ikinci yüzen-yüzey uygulaması kuruyordu | **Doğru.** Girdi tasarımı `<details>`/`deferred(anchored(..))` kullanıyor; ORT-006 tek pencere-kapsamlı tooltip konağı, ORT-021 çözülmüş ileti ister. Exact tipler fiziksel değil. | `TezgahKutusu`/yerel popup fallback'i kaldırıldı; bölüm modeli yalnız `YerelleştirmeAnahtarı` taşır, `?` alt kabulü ORT-006/021'e kapılıdır (§2.2, §3.4, F1). |
| 3 | [P1] Profil ham opaklık ve yerel yarıçap sahipliği kuruyordu | **Doğru.** Opaklık ORT-017 exact tipinin, şekil/geometri ORT-003'ün tek sahipliğidir. | Hedef alan `GörselOpaklıkKademesi`; `KutuMetriği` yalnız boyut/boşluk taşır, `.rounded*` fallback'i yok. *(VII/1: exact opaklık imzası uygulanabilir olana kadar alan kapılıdır.)* |
| 4 | [P1] Tipografi eşlemesi bire bir değildi ve fiziksel durum belirtilmemişti | **Doğru.** Beş kanonik rolün sahibi ORT-004; fiziksel tip henüz yok. | Beş yüz beş role bire bir eşlendi; yeni rol icat edilmedi ve profil kapısı açıkça yazıldı (§3.3.1–2). |
| 5 | [P2] `.rounded_full()` yerel GPUI'de yok deniyordu | **Yanlış iddia.** Çağrı `style_helpers` makrosuyla var ve galeride on kez kullanılıyor. | API tablosu düzeltildi; var oluşu ORT-003 tek-sahiplik kapısını gevşetmiyor (§5). |
| 6 | [P2] `Tezgah.dc.html` canlı gösterge/açıklamayı bugünkü davranış gibi okutabiliyordu | **Doğru.** DOM'da altı boyalı gösterge var; fiziksel `GirişKutusu::render` bunları çizmiyor. | Sicil ve geçersiz-kılma tablosu bu parçaları yalnız hedef görsel kompozisyon, uygulama/kabul kanıtı değil diye işaretledi (§0.3–0.4). |
| 7 | [P2] F2, ORT-019 tanı zarfıyla karışıyor ve alt başlıklar 2.6→2.5 sırasındaydı | **Doğru.** Panel doğrudan kamusal opak sonucu okuyor, tanı göndermiyor. | Adı “geliştirici gözlem paneli” oldu; F2.5 canlılık, F2.6 atom kapıları olarak sıraya kondu. Uyum Listesi 3. sürüme çıkarıldı. |
| 8 | [P2] İlk VI düzeltmesindeki `Arc<GörselOpaklıkKademesi>` aşırı düzeltmeydi | **Doğru.** `düşük_taban(self)` tüketen getter alan-içi `Arc` üzerinden okunamaz. `sozlesme-api-klon-değil` listesinde bulunmaması ise tek başına `Clone` uygulaması kurmaz. | Alan-içi `Arc` aynı döngüde kaldırıldı. *(VII. turda doğrudan alanın da `Profil: Clone` sınırını kapatamadığı bulundu; iki biçim de çözüm değildir — §0.9/1.)* |
| 9 | [P2] `Tezgah.dc.html` gösterge kabukları geçersiz `span > div` iç içeliği kuruyordu | **Doğru.** Kaynakta bir `gosterge-serit` ve altı `durum-gosterge` kabuğu `span` iken blok `div` çocuk taşıyordu; tarayıcı bunları örtük olarak onarıyordu. | Yedi kabuk `div` yapıldı. `tidy`nin ilgili “missing/discarding” uyarıları bitti; önbelleksiz tarayıcı koşumunda 1 şerit + 6 göstergenin tamamı `DIV`, doğrudan `span > div` sayısı **0** ve önceki kontrol sayıları değişmedi. |

### 0.9 VII. denetim turu — ikinci tam döngü ve bulgu kapanışı

| # | Bulgu | Doğrulama | Kapanış |
|---|---|---|---|
| 1 | [P0] VI/8'in “doğrudan `GörselOpaklıkKademesi` yeterlidir” sonucu da uygulanabilir değildi | **Doğru.** Kanonik tip `Clone`/`Copy` türetmiyor ve tek getter `düşük_taban(self)`; aynı sözleşmede `BileşenGörünümTanımı::Profil: Clone` ve `çöz(&Profil)` isteniyor. Alanı `Arc` yapmak getter'ı, doğrudan taşımak profil `Clone` sınırını kapatamıyor. | F0b artık yalnız fiziksel göçe değil, bu **kanonik imza çelişkisinin** düzeltilmesine de kapılıdır. Plan ikinci sayı/ham `f32`/yerel kopyalama numarası önermiyor (§3.3.2–3, risk 15). |
| 2 | [P1] F1 yapısal olarak başlatılabilirken tamamlanma kapıları eksik yazılmıştı | **Doğru.** Dokuz yüz F0b'de ORT-003/004/017'ye kapılıdır; F1 yalnız ORT-006/021 yardım kapısını sayıyordu. | F1 “yapısal akış” ve “tam görsel kabul” olarak ayrıldı; tamamlanma F0b + ORT-006/021 kapılarını miras alır (§6 F1). |
| 3 | [P1] `TezgahBölümü.başlık` ham `SharedString` taşıyordu | **Doğru.** `YÖN-006.ACC-008`, sergi başlığını `ORT-021 İletiİsteği` ile güncel locale sürümünde çözdürür ve hazır dizeyi kaynak saymaz. | Bölüm başlığı da yardım açıklaması gibi `YerelleştirmeAnahtarı` oldu; render anında argümansız `İletiİsteği` ile çözülür (§2.2, §9). |
| 4 | [P1] F2.2 kanonikte olmayan bir `TanıKodu` tipini bekliyordu | **Doğru.** Exact model `TanıDeğeri::Kod(TanımKimliği)` + mühürlü `TanıZarfıFabrikası` + private sicildir; fiziksel model hâlâ serbest `GüvenliKod`/`TanıOlayı` yüzeyindedir. | Hayalî tip atfı kaldırıldı. Gerekçe ancak exact fabrika/private sicil eşlemesiyle kayıtlı `TanımKimliği`ne çevrilebildiğinde açılabilir; bugün panelde yoktur (F2.2). |
| 5 | [P2] ORT-021 fiziksel envanteri en yakın eski karşılığı atlıyordu | **Doğru.** Fiziksel temel crate'te `İletiİsteği`, `İletiÇözümleyicisi`, `YerelleştirmeAnahtarı` ve `Çözülmüşİleti` var; eksik olan exact `ÇözülmüşKullanıcıİletisi` sonucu/imzasıdır. | §3.4 tablosu exact/eski ayrımını doğru yüzeyle yeniden yazdı. |
| 6 | [P2] Uygulama yolu ve iki kaynak künyesi yeterince kesin değildi | **Doğru.** `src/...` yolları crate-tabanlıydı fakat taban söylenmiyordu; accesskit re-export satırı `gpui.rs:93`, 91 değil; üst künye ORT-021/023 ve YÖN-002'yi anmıyordu. | Crate yolu tabanı eklendi, satır atıfları düzeltildi ve bağlayıcı kaynak künyesi tamamlandı (§1.1, §5). |
| 7 | [P2] `github.md` exact kaynak pini taşımıyordu | **Doğru.** `../gpui` HEAD `1995423061bfe65b27266a80d9d4200e457a29e1`, kayıtlı Zed kaynağı `cef06d351bec10d0fb6176018ce8624e97baeb40`; planın tükettiği `crates/gpui/` kapsamı temizdir. | İki commit, yerel yol, denetim zamanı ve kapsam-dışı kirli yollar `github.md`ye yazıldı; açık karar kapandı (§0.3, §10). |
| 8 | [P2] `Tezgah.dc.html`deki bilinen `4.0.0` künyesi yalnız “güvenilmez” diye bırakılmıştı | **Doğru.** Yaşayan BİL-010 sürümü `13.0.0`; eski sayı yalnız kaynak yorumundaydı ve görsel/çalışma zamanı davranışı etkilemiyordu. | Yorum `13.0.0` yapıldı; dosyada artık yalnız güncel sürüm geçiyor. Sicil “görsel referans” statüsünü koruyor ama sürüm tutarsızlığı kapandı. |

---

## 1. Sınır — bu plan neyi değiştirir, neyi değiştirmez

### 1.1 Değişir

> Bu bölümdeki `src/...` ve `tests/...` yolları
> `crates/gpui-bilesenleri-galeri/` köküne göredir.

| Dosya | Aralık / durum | Değişim |
|---|---|---|
| `src/sergiler.rs` | `tezgah_sergisi` ve yardımcıları ≈**800–2990** | Şerit tabanlı çizim kalkar; iki kolonlu yeni düzen gelir. `ölçü` modülü `TezgahGörünümProfili` rollerine taşınır. |
| `src/lib.rs` | `TezgahKutusu`, `tezgah_kutusunu_değiştir/kapat/açık_mı`, `köşe_izi` (≈546–630) | Yerel yüzer kutu durum makinesi kalkar. Kartlar kalıcı akışta; `?` yardım yüzeyi yalnız ORT-021 çözümü + ORT-006 `Araçİpucu` konağı üzerinden açılır ve fiziksel göç tamamlanana kadar kapılıdır (§3.4). |
| `src/bil010_tezgah.rs` | `TezgahTercihleri` (**49 alan**) | Fazlara göre genişler; A/B/C/D alt yapılarına ayrılır (§7). |
| `src/palet.rs` | `galeri_paleti` | Dört kip elle kaydedilir; YK indirgemesi kalkar; `olumlu/tehlike/uyarı/bilgi/gölge` eklenir. |
| **yeni** `src/tezgah/` | — | Kabuk: tokenlar, görünüm profili, yüzler, yerleşim, `TezgahProfili`. |
| `src/simgeler.rs` | 24–56 | `warning` ve `info-circle` eklenir. `close-circle`, `search`, `eye`, `calendar` **zaten kayıtlı**. |

### 1.2 Değişmez

- Galeri kabuğu: üst araç çubuğu, sol bileşen gezintisi, katalog kartları, aile
  ayrıntı sayfası, dar yerleşim (`YÖN-006 §3`, `ACC-009`–`ACC-011`).
- Diğer ~30 aile sergisi.
- Kanonik sandıklar. Galeriden kanoniğe ters bağımlılık açılmaz (`ACC-006`).
- `GirişKutusu`'nun davranışı ve `§16.2` çözümü. Tezgâh **tüketicidir**.

### 1.3 Rota ve yerleşim kararı (bulgu 4 · kapandı)

**Karar: tezgâh aile ayrıntı sayfasının içindeki sergi olarak kalır.**
`GaleriSayfası::Sergi` rotası bu planda **açılmaz**.

Gerekçe:

- `YÖN-006 §3.4` orta belge sırasının değişmemesini ve hedefe özgü ikinci bir
  galeri tasarımı olmamasını istiyor; tam-pencere tezgâh üç bölgeli düzenden
  çıkar ve klavye turunu (`üst → sol → orta → sağ`) bozar.
- Sözleşme `Sergi { kimlik }` rotasını tanımlıyor ama kod taşımıyor
  (`galeri.rs:217`); rota eklemek `ORT-023` snapshot/niyet akışını da açar. Bu,
  tezgâh tasarımının değil ayrı bir atomun işidir.
- Bunun bedeli: tasarımın `404px + minmax(460px, 1fr)` iki kolonu orta bölgede
  her zaman sığmaz. Karşılığı §4'teki uyarlanabilir kolon kuralıdır — sığmayan
  genişlikte **tek kolona iner**, kırılmaz.

### 1.4 Kapsam dışı (bilinçli)

Tasarımın Bölüm I §6 üst şeridi galeri kabuğunun küresel eksen çubuğuyla
çakışır. Tezgâh **kendi üst şeridini kurmaz**; tema/kip/hedef/ölçek denetimleri
galerinin çubuğunda kalır (`YÖN-006 §4`). Tezgâha yalnız D bölümünün tezgâha
özgü kalemleri (aile seçimi, parça tipografisi, fallback rozeti) iner.

---

## 2. Mimari — iki katman

### 2.1 Katman A · Tezgâh kabuğu (bileşen-bağımsız)

```
src/tezgah/
├── tokenlar.rs   · dört kip → ORT-004 TemaAnlıkGörüntüsü
├── profil.rs     · TezgahGörünümProfili (ORT-017 tipli metrik sahibi)
├── yuzler.rs     · dokuz yüz; ölçü/tipografi profilden gelir, sabit taşımaz
├── yerlesim.rs   · kök, gövde, iki kaydıran kolon, uyarlanabilir akış
├── arayuz.rs     · TezgahProfili trait'i + TezgahBölümü
└── mod.rs
```

### 2.2 Katman B · Bileşen profili — **trait değil veri yapısı** (IX. tur)

Planın önceki sürümleri bir `TezgahProfili` trait'i öneriyordu. Uygulamada
trait'in her metodu `&mut Context<T>` ister (listener kurmak için) ve bu, kabuğu
galeri uygulamasına bağlar — "bileşen-bağımsız kabuk" iddiası kalmaz. Sınır bu
yüzden **veri yapısıdır**: profil kendi bağlamıyla çizer, kabuğa hazır
`AnyElement` verir.

```rust
pub struct Tezgahİçeriği {
    pub başlık: YerelleştirmeAnahtarı,           // erişilebilir ad
    pub önizleme_başlığı: YerelleştirmeAnahtarı, // sol bölge adı
    pub yapılandırma_başlığı: YerelleştirmeAnahtarı,
    pub önizleme: Vec<AnyElement>,   // kabuk denetimleri + yaşayan alan
    pub sol_ek: Vec<AnyElement>,     // türetilmiş durumlar, gözlem paneli
    pub kod: Option<AnyElement>,     // sol kolonun en altı
    pub bölümler: Vec<TezgahBölümü>, // profil tarafından SÜZÜLMÜŞ
}

pub struct TezgahBölümü {
    pub kimlik: &'static str,             // "s7", "s9" … çapa gezintisi
    pub başlık: YerelleştirmeAnahtarı,    // ham dize değil (YÖN-006.ACC-008)
    pub yardım: Option<YerelleştirmeAnahtarı>, // `?` yüzeyi ORT-006'ya kapılı
    pub akış: Akış,                       // TamGenişlik | A | B | C
    pub içerik: AnyElement,
}
```

**Tür süzgeci profilin işidir.** Kabuk "bu bölüm bu türde kurulabilir mi"
sorusunu sormaz; profil `bölümler`i zaten süzülmüş verir. Planın önceki
sürümündeki `kapsam: &'static [TürKimliği]` alanı kaldırıldı — `TürKimliği`
kabukta tanımlanamaz, tanımlansaydı kabuk `BİL-010`'un tür eksenini bilirdi.

Trait, ikinci bir profil geldiğinde ve ortak bir kayıt defteri gerektiğinde
açılabilir; bugün tek uygulayıcıyla soyutlama borcu yaratırdı.

---

## 3. Tasarım dili katmanı

### 3.1 Renk tokenları (Bölüm I §4.1)

| Konu | Karar |
|---|---|
| Kip sayısı | Dört: `Açık`, `Koyu`, `YüksekKarşıtlıkAçık`, `YüksekKarşıtlıkKoyu`. Hiçbiri türetilmez (`ORT-004 §5.7`, `ACC-011`: sahte destek ilan edilmez). |
| Tema ailesi | `Kâğıt` yeni tasarımın değerlerini alır; `Mürekkep` dört kipe tamamlanır. |
| Yeni tokenlar | `olumlu`, `tehlike`, `uyarı`, `bilgi`, `gölge`. |
| Bileşene iniş | Ham `u32` bileşene gitmez; `TemaAnlıkGörüntüsü` çevirisi tek yerde (`ACC-001`). |

### 3.2 Yüzler (Bölüm I §4.3)

Dokuz yüz, `yuzler.rs`'de tek tanım: hap · pasif hap · segment kuşağı ·
türetilmiş rozet · küçük anahtar · kart · bölüm başlığı · eksen etiketi ·
açıklama balonu.

**Normatif ayrım (§4.3):** birbirini dışlayan eksen → segment kuşağı; bağımsız
bool → hap düğme. Bütün bölümlerde aynıdır.

**Yasaklar (§4.4):** gradyan, emoji, sol kenar vurgu şeridi, kart içinde gölgeli
kart, dekoratif çizim yok; yeni renk uydurulmaz.

### 3.3 `TezgahGörünümProfili` — ORT-017 kaydı (I/3 · II/3 · III/2 · III/3)

`ORT-004.ACC-001` ham rengi **ve dağınık fiziksel ölçü sabitini** birlikte
yasaklıyor; bileşene özgü metrik yalnız `ORT-017` tipli profilinde tek sahipli
olabilir. Renk tarafı §3.1'de kapanıyor; ölçü/tipografi tarafı buraya bağlanır.

#### 3.3.1 Kanonik görünüm yüzeyi ile fiziksel kodun farkı (III/2 · VI)

Yürürlükteki `ORT-004/017` yüzeyi, fiziksel
`gpui-bilesenleri-temel/src/görünüm.rs` ve tema kodundan **daha geniştir**.
Plan kanonik metni esas alır:

| Kanonik `ORT-017 4.3.0` | Fiziksel `görünüm.rs` | Sonuç |
|---|---|---|
| `tipografi_parçaları() -> Arc<[TipografiUygulanabilirParçaKaydı]>` | **yok** | Fiziksel kod geride |
| `tipografi_uygulama_grupları() -> Arc<[TipografiUygulamaGrubuTanımı]>` | **yok** | Fiziksel kod geride |
| `Result<…, GörünümKayıtHatası>` | `GörünümHatası` | Ad farkı |
| `ÇözülmüşGörünümProfili.seçimler` | `.bileşenler` | Ad farkı |
| `GörünümKayıtDefteri` | trait var, **somut uygulayıcı yok** | Kayıt kapısı çalışmıyor |
| `GörselOpaklık` / `GörselOpaklıkKademesi` | **yok** | Profilin tipli pasif-opaklık alanı fiziksel ORT-017 göçüne bağlı |
| `AnatomiParçasıSınıfı` | eski `ParçaSınıfı` var | Exact kanonik ad/yüzey fiziksel değil; yerel eşadlı tip kurulmaz |
| `GörünümKayıtHatası::AnatomiUyumsuz` | eski `GörünümHatası::AnatomiUyumsuz` var | Exact hata yüzeyi fiziksel değil |
| `ORT-004 TipografiRolü` | **yok** | Beş kanonik rol sözleşmede var, fiziksel temel crate'te henüz yok |

Bu, tezgâh planının kapatabileceği bir açık değildir — `ORT-017` fiziksel
göçünün borcudur. **F0'ın kapsamı buna göre daralır** (§3.3.3).

#### 3.3.2 Profil neyi taşır, neyi taşımaz (III/3 · V/2 · V/4)

`ORT-017` profilin yasak listesini sayıyor: *"Profil ham renk, fiziksel font,
dosya yolu, OpenType etiketi, callback, `Any`, iş verisi veya `Entity`
taşımaz."* **`Pixels` bu listede yoktur.** `ORT-004.ACC-001` ise bileşene özgü
metriğin `ORT-017` tipli profilinde **tek sahipli** olmasını istiyor.

İkisi birlikte okununca sonuç nettir:

| Ne | Profilde | Gerekçe |
|---|---|---|
| Sayısal metrik (`Pixels`) | **taşınır** | Yasak listesinde yok; ACC-001 tek sahipliği zaten profilde istiyor |
| Ham renk (`Hsla`, `u32`) | taşınmaz | ORT-017 yasak listesi + ACC-001 |
| Fiziksel font (aile adı, `TextStyle`) | taşınmaz | ORT-017 "fiziksel font" yasağı |
| Tipografi | **`TipografiRolü` olarak** taşınır | Rol semantiktir; aile ve boyut temadan çözülür |

> **4. sürümün hatası düzeltildi.** O sürüm metrikleri profilden çıkarıp
> `çöz()` gövdesine koyuyordu. **Fonksiyon gövdesi profil değildir** —
> ACC-001'in istediği tek sahiplik böyle kurulmaz. Metrikler profile geri döndü.

**Tip sahipliği (V/4):** `TipografiRolü` **`ORT-004`**'te tanımlıdır
(`Gövde`, `KüçükGövde`, `Etiket`, `Başlık`, `TekAralıklı` — beş varyant);
`ORT-017` onu `TipografiUygulanabilirParçaKaydı.semantik_rol` içinde tüketir.
Planın 5. sürümü tipi ORT-017'ye ait göstermişti; düzeltildi.

Tezgâhın beş metin yüzü beş kanonik role **bire bir eşlenir**, yeni rol icat
edilmez:

| Tezgâh yüzü | `ORT-004 TipografiRolü` |
|---|---|
| gövde | `Gövde` |
| bölüm başlığı | `Başlık` |
| eksen etiketi | `Etiket` |
| rozet metni | `KüçükGövde` |
| kod paneli | `TekAralıklı` |

```rust
/// Tezgâh kabuğunun metrik ve tipografi rolü sahibi.
///
/// Sayısal metrik burada tek sahiplidir (ORT-004.ACC-001). Ham renk ve
/// fiziksel font taşınmaz (ORT-017 yasak listesi).
pub struct TezgahGörünümProfili {
    pub kimlik: GörünümProfiliKimliği,

    // --- metrikler: tek sahipli, ham px başka hiçbir yerde yok ---
    pub hap: KutuMetriği,              // yalnız boyut/boşluk; yarıçap ORT-003
    pub kart: KutuMetriği,
    pub segment: KutuMetriği,
    pub rozet: KutuMetriği,
    pub anahtar_yüksekliği: Pixels,
    pub simge_düğmesi: Pixels,
    pub pasif_opaklık: GörselOpaklıkKademesi, // hedef; VII/1 imza blokeri kapılı
    pub önizleme_kabuğu: KabukMetriği,  // §4.1
    pub kolonlar: KolonMetriği,         // aralık, önizleme kolonu, iki_kolon_eşiği

    // --- tipografi: ORT-004 rolü; aile/boyut temadan çözülür ---
    pub gövde: TipografiRolü,
    pub bölüm_başlığı: TipografiRolü,
    pub eksen_etiketi: TipografiRolü,
    pub rozet_metni: TipografiRolü,
    pub kod_metni: TipografiRolü,
}

/// Çözülmüş katman: profil + tema. `TextStyle` YALNIZ burada üretilir.
pub struct ÇözülmüşTezgahGörünümü {
    pub hap: KutuMetriği,               // profilden türetilmiş değişmez render snapshot'ı
    pub gövde: gpui::TextStyle,         // TipografiRolü + tema → burada
    …
}
```

`KutuMetriği`, `KabukMetriği` ve `KolonMetriği` kanonikte **yoktur**; tezgâhın
kendi tipleridir ve `src/tezgah/profil.rs`'de tanımlanır. `çöz()` metriği
değiştirmez, yalnız tipografi rolünü temaya uygulayarak `TextStyle` üretir.
`KutuMetriği` yalnız boyut ve boşluk taşır; şekil veya yarıçap taşımaz.
`KutuŞekliTercihi` ve exact `KutuŞekliGeometrisi` tek sahibi `ORT-003` yolundan
gelir. Pasif görünüm ham `f32` değil, doğrudan ORT-017
`GörselOpaklıkKademesi` taşır; GPUI alpha dönüşümü yalnız private boya
bağdaştırıcısındadır. Çözülmüş görünümdeki metrik kopyası yeni bir kaynak veya
hesap değildir; `BileşenGörünümTanımı::render` yalnız çözülmüş snapshot aldığı
için profil sahibinden değişmeden türetilen render girdisidir.

**Açık kanonik imza blokeri (VII/1):** `GörselOpaklıkKademesi` sözleşme API
bloğunda `Clone`/`Copy` türetmez ve tek getter'ı `düşük_taban(self)` değer
tüketir. Aynı API `type Profil: Clone` ile `çöz(&Profil)` ister. Bu nedenle:

- alanı doğrudan taşımak `TezgahGörünümProfili: Clone` kuramaz;
- alan-içi `Arc<GörselOpaklıkKademesi>` profilin klonlanmasını sağlar ama
  ödünç çözüm sırasında tüketen getter'ı okunabilir yapmaz;
- ikinci `u16`, ham `f32`, katsayı veya yeniden-kurma fallback'i
  `ORT-017.ACC-034/035/038` tek sahipliğini bozar.

Plan bu üç yoldan hiçbirini çözüm saymaz. F0b, ORT-017 kanonik API'sinde
kademenin güvenli ödünç okunmasını ve `Profil: Clone` sınırını birlikte mümkün
kılan bir düzeltme (örneğin sözleşme sahibinin belirleyeceği trait/getter
imzası) **ve ardından** fiziksel göç gelene kadar kapılıdır. Bu plan kanonik
sözleşmeyi kendiliğinden değiştirme yetkisi varsaymaz.

#### 3.3.3 F0 kabul koşumu (daraltıldı)

`GörünümKayıtDefteri`'nin somut uygulayıcısı olmadığı için "kayıt kapısından
geçmiş profil" bu turda **kanıtlanamaz**. F0 şunu hedefler:

1. `TezgahGörünümProfili` sayısal metriğin **tek sahibi**; `TextStyle`, `Hsla`,
   ham `f32` opaklık ve font aile adı profilde **yok**. Opaklık yalnız
   `GörselOpaklıkKademesi`, şekil yalnız `KutuŞekliTercihi`/ORT-003 yolundadır
   (tip düzeyinde kanıt).
2. `yuzler.rs` ve `yerlesim.rs` yalnız profilden/çözülmüşten okuyor — `px(`
   literali ve `font_family("…")` bu iki dosyada yok (grep kanıtı).
   `çöz()` metriği değiştirmiyor, yalnız `TipografiRolü` + tema → `TextStyle`.
3. `doğrula()` reddi tanıyla dönüyor; sessiz fallback yok.
4. Profil değişimi sürüm artırıyor; aynı sürüm aynı değeri veriyor.
5. **Kapılı:** Profilin derlenebilir kanonik biçimi (`ORT-004 TipografiRolü`,
   `ORT-017 GörselOpaklıkKademesi`), `BileşenGörünümTanımı` uygulaması ve
   `anatomi_kaydet` çağrısı; önce VII/1'deki opaklık `Clone`/ödünç-okuma
   çelişkisinin kanonikte kapanması, sonra ORT-004/017 fiziksel göçü
   (`tipografi_parçaları`, `tipografi_uygulama_grupları`, exact hata/parça
   sınıfları ve `GörünümKayıtDefteri` uygulayıcısı) tamamlandığında eklenir.
   Yerel eşadlı rol/çokluk tipi veya ham opaklık fallback'i kurulmaz. O güne
   kadar yalnız bağımsız token/simge hazırlığı yapılabilir; profil kayıt
   defterine bağlıymış ya da derlenebilir hedefmiş gibi sunulmaz.

### 3.4 ORT-003/006/021 fiziksel kapıları — yerel fallback yok

Planın üç başka kanonik tüketimi de fiziksel kaynakla aynı yüzeyde değildir:

| Kanonik hedef | Fiziksel kaynak | Plan sonucu |
|---|---|---|
| `ORT-003 KutuŞekliGeometrisi` | eski `KutuŞekliSonucu`/`KutuYolu` var; exact tip yok | Şekil kaynağı yine `KutuŞekliTercihi`dir; exact ortak boya/hit-test yolu gelmeden yeni tezgâh yüzü kendi geometri sonucunu üretmez |
| `ORT-006 Araçİpucuİsteği` + `YüzeyKonağı` | eski `YüzenYüzeyKonağı` var; exact istek/konak yok | `?` yardım yüzeyleri kapılı; doğrudan `deferred(anchored(..))` veya `TezgahKutusu` fallback'i yok |
| `ORT-021 ÇözülmüşKullanıcıİletisi` | `YerelleştirmeAnahtarı`, `İletiİsteği`, `İletiÇözümleyicisi` ve eski `Çözülmüşİleti` var; exact sonuç tipi ve çözüm imzası yok | `TezgahBölümü` yalnız başlık/açıklama anahtarı taşır; exact çözülmüş sonuç gelmeden yerel eşadlı tip veya hazır-dize fallback'i kurulmaz |

Bu kapılar F1'in **yapısal** kart/akış göçünü engellemez; fakat ORT-003 exact
ortak boya yolu olmadan yeni hap/kart yüzleri, ORT-021 exact çözüm sonucu
olmadan bölüm başlıkları ve ORT-006/021 exact yüzeyi olmadan `?` yardım
balonları tamamlanmış sayılmaz. Kapı açılmadan `?` tetikleyicisi çizilmez;
pasif neden gerekiyorsa kartın normal akışında görünür metin olarak sunulur.

---

## 4. Yerleşim (Bölüm I §5 → GPUI) — bulgu 5 · kapandı

Tezgâh gömülü olduğu için (§1.3) genişliği **kendi kabından** gelir; `min_w`
kurmaz. Kip seçimi ölçülen kap genişliğiyle yapılır (`YÖN-006 §3.4`: kip hedef
adına göre değil kullanılabilir alana, metin ölçeğine ve yön snapshot'ına göre
seçilir).

> **Sonradan değişti.** Aşağıdaki şema tezgâh galeriye gömülüyken
> geçerliydi. Tezgâh kendi ekranına taşınınca genişlik `956px`e sabitlendi,
> altına yatay kaydırma geldi ve kip seçimi ortadan kalktı; `container_query`,
> `iki_kolon_eşiği` ve `YerleşimKipi` çağrısız kalıp silindi (§8 borç 9).

```
tezgâh kökü  .flex().flex_col()          ← kendi genişliğini dayatmaz
└─ container_query(|kap| …)              ← ÖLÇÜLEN KAP, min_w yok
   ├─ kap.width >= profil.iki_kolon_eşiği → GENİŞ KİP
   │   gövde .flex().gap(kolon_aralığı)
   │   ├─ SOL .w(profil.önizleme_kolonu).flex_shrink_0()
   │   │       .id("önizleme").overflow_y_scroll().min_h(px(0.))
   │   └─ SAĞ .flex_1().min_w(px(0.))
   │           .id("yapılandırma").overflow_y_scroll().min_h(px(0.))
   └─ kap.width <  eşik → DAR KİP
       tek kolon: önizleme → yapılandırma → C → kod paneli
       (mantıksal sıra korunur, akışlar tek kolona iner)
```

- Eşik `profil.iki_kolon_eşiği`; **metin ölçeğiyle birlikte** değerlendirilir —
  `%200` ölçekte geniş kip erken terk edilir (`YÖN-006 §4`, kabulde `%200`).
- CSS `columns` karşılığı yok: akış kartları iki `flex_col` kolona elle
  dağıtılır; kart sayısı < 2 ise tek kolona düşer.
- Kaydıran her `div`'in `.id()`si ve `min_h(px(0.))`ı olur.
- Dar kipte kapalı bölge odağa giremez (`YÖN-006 §3.4`).

**Kabul koşumu (F1):** geniş/dar kip × Masaüstü/WASM × `%100`/`%200` metin
ölçeği — aynı genişlikte iki hedef aynı kipi üretir (`ACC-004`, `ACC-009`).

### 4.1 Önizleme kabuğunun ölçüleri (V/2)

`Yerlesim Raporu`'ndaki sayılar `TezgahGörünümProfili.önizleme_kabuğu`
alanına — yani **profile** — yazılır; başka hiçbir yerde ham px olarak
bulunmazlar:

```rust
pub struct KabukMetriği {
    pub yükseklik: Pixels,        // 58
    pub parça_aralığı: Pixels,    // 5  · tek gap
    pub simge_kutusu: Pixels,     // 15 · kare
    pub köşe_ankrajı: Point<Pixels>, // üst 5 · sağ 5, akış dışı
    pub iç_boşluk: Edges<Pixels>, // Açık nokta 1
}
```

Kapalı parça, kendi genişliğiyle **birlikte** kendisinden önceki `parça_aralığı`
kadar boşluğu da değere geri döndürür.

> **Açık nokta 1 (duruyor):** iç boşluk — Bölüm I §7.2 `8/5/6/6` mi, Yerleşim
> Raporu `5/5` mi? Önerim: Bölüm I normatif, rapor dipnot.

---

## 5. Bölüm II'nin yerel `../gpui`'ye uyarlanması

Bölüm II *Zed deposundaki güncel gpui*'ye göre yazılmış ve git bağımlılığı
öneriyor; bu depo `YÖN-002` uyarınca **yerel `../gpui` ağacını** kullanır, git
bağımlılığı alınmaz.

Doğrulanan API'ler: `.grid()`, `.grid_cols()`, `.col_span()`, `container_query`,
`.role()`, `.aria_label()`, `.aria_toggled()`, `.aria_numeric_value()`,
`.on_a11y_action()`, `.tab_stop()`, `deferred()`, `anchored()`,
`.snap_to_window_with_margin()`, `.border_dashed()`, `.opacity()`,
`.font_family()`, `text!`, `Role` / `Toggled` / `AccessibleAction` (accesskit
üzerinden, `gpui.rs:93`).

Karşılığı olmayan üç çağrı:

| Bölüm II'de | Yerel `../gpui` | Uyarlama |
|---|---|---|
| `.aria_hidden(true)` | yok | Glife `role`/`aria_label` **hiç verilmez**; ileti alanın var olan açıklama kanalından gelir (`§16.2.5`). |
| `.occlude_none()` | yok (`occlude()` var, tersi) | `occlude()` **çağrılmaz**; varsayılan hitbox kurmaz (`§16.2.4`: gösterge ayrı hitbox değildir). |
| `.rounded_full()` | **var** (`style_helpers` makrosu; galeride bugün kullanılıyor) | API eksikliği değildir. Buna rağmen yeni tezgâh yüzünde doğrudan çağrılmaz; `KutuŞekliTercihi::Açık(DüğmeŞekli::Hap)` kanonik ORT-003 `KutuŞekliGeometrisi` yolunda çözülür. Exact fiziksel yol gelene kadar yerel `.rounded_full()`/`.rounded(…)` fallback'i kurulmaz (§3.4). |

---

## 5.5 Onaylanan uygulama kararları (VIII. tur · kullanıcı yetkisi)

| # | Karar | Gerekçe |
|---|---|---|
| 1 | **Pasif yüz opaklıkla değil, `ORT-004` `devre_dışı` kutu roluyle çizilir.** | `GörselOpaklıkKademesi` alanı yalnız *kademeli görünürlük* veya *Sade görünüm* kullanan profilde zorunlu. Tezgâhın tek opaklık kullanımı pasif düğme yüzüydü; bu bir erişim durumudur ve `ortak_kutu_rolleri.devre_dışı` zaten dolu. Böylece kanonik imza çelişkisi (§0.9/1) **tezgâhı bağlamaz**; F0b'nin kapısı yalnız ORT-003/004/017 fiziksel göçüne kalır. Bedeli: Bölüm I §4.3'ün "opaklık düşürülür" ifadesinden sapmak (§0.4'e işlendi). |
| 2 | **Önizleme kabuğu iç boşluğu Bölüm I §7.2'dir: `8/5/6/6`.** | Yerleşim Raporu'nun `5/5`'i yatay simetriyi doğru veriyor ama dikey asimetri optik taban çizgisi hizası içindir. Açık nokta 1 **kapandı**. |
| 3 | **`iki_kolon_eşiği` sabit sayı değil, profilden türer:** `önizleme_kolonu + kolon_aralığı + (yapılandırma_asgarisi × metin_ölçeği)` | `%100` → `404 + 28 + 460 = 892px`; `%200` → `404 + 28 + 920 = 1352px`. Metin ölçeği yalnız yapılandırma kolonunu büyütür; önizleme kolonu sabit ölçülü kabuk taşır. Tek sihirli sayı yerine üç metrikten türeme, `ACC-001` tek sahipliğiyle tutarlı. Açık nokta 4 **kapandı**. |
| 4 | **`screenshots/s23.png` F1'e ertelendi.** | Bugün alınacak görüntü hâlâ eski şerit tabanlı tezgâhı gösterir; F1 sonrası alınırsa yeni düzenin gerçek kanıtı olur. Açık nokta 3 → F1 kanıt görevi. |

---

## 5.6 Yerel `../gpui` yenilik envanteri (VIII. tur)

Yerel ağaç `1995423061` (Zed `cef06d35`) ile eşit ve `SAPMALAR.md`'deki kayıtlı
sapmalar dışında upstream ile aynı. Bugün **hiçbir `#[deprecated]` public API
yok**, yani mevcut kod için ani kırılma riski taşımıyor. Asıl bulgu tersi
yönde: yeni gpui'nin getirdiği kolaylıklar **neredeyse hiç kullanılmıyor**.

| API | Galeri | Kanonik | Sonuç |
|---|---|---|---|
| `container_query` | 0 | 0 | Tezgâh sabit genişlikte (`956px`) ve yatay kaydırmalı; kip seçimi gömülülükle birlikte kalktı |
| `.grid()` / `grid_cols` / `col_span` | 0 | 0 | Izgara elle flex ile kuruluyor |
| `.role()` | 20 | 1 | **Kuruldu** (F1 görsel kabulü) |
| `aria_*` | 36 | 1 | **Kuruldu** |
| `tab_stop` / `tab_group` | 12 | 3 | **Kuruldu**; `tab_index` yalnız kanonikte |
| `text!` | 0 | 0 | Dize çocukları ham `&str` |
| `deferred` / `anchored` | 6 / 0 | 0 / 0 | Açılır listeler `deferred`; `anchored` `ORT-006` konağını bekliyor |
| `.focusable()` | 0 | 0 | — |
| `.observe()` | 1 | 0 | F2.5 gözlem paneli bağlandı |

*(Sayımlar VIII. turda sıfırdı; F1/F2 ile kuruldu. Tablo o turun envanteri
değil, bugünkü durumdur.)*

**F1 sonucu:** düzen `role`/`aria_*` ve `tab_stop`/`tab_group` üzerine
kuruldu; `viewport_size` deseni tezgâh kolonlarında tekrarlanmadı.
`container_query` gerekmedi — tezgâh kendi ekranına taşınınca genişlik
sabitlendi ve kip seçimi ortadan kalktı. Bu, `YÖN-006.ACC-011` (dar yerleşim erişilebilir
eşdeğeri) ve `ACC-009` (aynı klavye turu) için zaten gerekli.

**Ayrı bulgu — bu planın kapsamı dışı:** galeri kabuğunun kendisi de
`role`/`aria_*`/`tab_*` taşımıyor. Tezgâh sergisi bunları kazanınca kabuk
geride kalır; kabuk erişilebilirlik yüzeyi ayrı bir atom olarak açılmalıdır.

**Modül düzeni:** `ORT-001.ACC-001` ve `ORT-001 §2/7` `mod.rs`'i yasaklar ve
`#[path]` ile aşılmasını da yasaklar. Tezgâh modülü `tezgah.rs` + `tezgah/*.rs`
düzenindedir. Galeri sandığındaki dört gereksiz `#[path]` kaldırıldı; **depo
genelinde 60 tane daha var** — hiçbiri `mod.rs` yasağını aşmıyor, ama edition
2024'te gereksizler. Ayrı temizlik atomu.

---

## 6. Fazlar

### F0 · İskelet ve görünüm tabanı — bağımsız/kapılı ayrımı

**F0a · TAMAMLANDI** (`b48047a`, `8bf3ab6`, `adc8425`): `src/tezgah.rs` +
`tezgah/tokenlar.rs` açıldı; `palet.rs` dört kipe genişledi, YK indirgemesi
kalktı, dört semantik renk + gölge tokenı eklendi; `TemaAnlıkGörüntüsü::kip`
tahminden paletin kendi alanına taşındı; `simgeler.rs`e `warning` ve
`info-circle` varlık olarak eklendi (katalog kaydı K1'e bırakıldı).
Koşum: **98 test yeşil** (öncesi 93; beş yeni token testi).

**F0b · TAMAMLANDI** (`ORT-003/004/017` göçü açtı): `profil.rs` kanonik
`GörünümProfiliBaşlığı`, `KutuŞekliTercihi` ve `TipografiRolü` taşıyor;
`yuzler.rs` sekiz yüzü çiziyor. Pasif yüz `ORT-004` devre dışı kutu rolünü
kullanıyor, ham opaklık yok. `?` yardım yüzeyi hâlâ `ORT-006`ya kapılı ve
çizilmiyor. Kayıt kapısı (`anatomi_kaydet`) da kapalı: `GörünümKayıtDefteri`nin
somut uygulayıcısı yok, profil kendi içinde tek sahipli.

**F0b · eski hâli — fiziksel göçlere kapılı:** `profil.rs`in kanonik tipli gövdesi
ORT-004/017'ye; `yuzler.rs`deki dokuz yüz ORT-003 exact geometri + ORT-004/017
profile; `?` yardım yüzeyi ORT-006/021'e bağlıdır (§3.3.3/5, §3.4). Bu tiplerin
yerel eşadlı kopyaları, ham opaklık veya doğrudan köşe-yarıçapı fallback'i
kurulmaz.
- ~~Bayat raporlar yeniden üretilir~~ — **tamamlandı** (bu turda). `Yerlesim Raporu`
  §3/§5/§6/§7 ve `Sozlesme Uyum Listesi`'nin temsil boşluğu bölümü güncel fiziksel
  duruma göre yeniden yazıldı; listeye `§16.2` satırı eklendi.

*Kabul (F0a):* `cargo test -p gpui-bilesenleri-galeri` yeşil; dört kip ve beş
yeni palette rolü için snapshot/test; ekran değişmedi.

*Kabul (F0b, kapı açıldıktan sonra):* `yuzler.rs` + `yerlesim.rs` içinde `px(`
literali yok (grep kanıtı); `TezgahGörünümProfili`de ham renk/fiziksel font/ham
opaklık yok; exact ORT-003/004/017 tipleri kullanılıyor.

### F1 · Yüz ve düzen

**Yapısal akış çekirdeği TAMAMLANDI** (`9656ab2`): `tezgah/arayuz.rs`,
`tezgah/yerlesim.rs`, `tezgah/govde.rs`. Kip `container_query` ile ölçülen
kaptan seçiliyordu — **bu sonradan kalktı**, tezgâh kendi ekranına taşınıp
sabit genişliğe geçince mekanizma çağrısız kaldı (§8 borç 9). Kolonlar
`Role::Region` + `aria_label`, kartlar `Role::Group`
+ `tab_group().tab_stop(false)` taşıyor. Koşum: **106 test yeşil**.
Kalan yapısal iş: mevcut 49 eksenin bölümlere dağıtılması ve `TezgahKutusu`
yüzer panel mekanizmasının kaldırılması.

**Görsel kabul TAMAMLANDI.** F0b'nin yüz kümesi artık gerçekten çiziliyor:
`sergiler.rs` ve `govde.rs` ölçüyü `ÇözülmüşTezgahGörünümü`den, rengi
`TezgahTokenları`ndan okuyor. Kare başında `görünümü_kur(tasarım_görünümünü_çöz())`
palet kurulumunun hemen ardından çalışır; fallback yolu önbelleğe alınmaz, yoksa
tema değişince bayat ölçü dönerdi. Beş ham ölçü sabiti (`SÜTUN`, `GRUP_İÇİ`,
`DÜĞME`, `DÜĞME_DOLGUSU`, `ŞERİT_ARASI`) profile taşındığı için ölü kaldı ve
silindi. İki değişmez teste bağlandı: `çizim_katmanı_ham_renk_ve_ölçü_taşımaz`
(yüzler ve gövde `rgb(..)`/`px(<sayı>)` içermez; `px(0.)` bir ölçü değil, flex
taşma sıfırlamasıdır) ve `ölü_yüz_yoktur` (her `pub fn` yüzünün çizimde bir
kullanımı var — `pub` olduğu için derleyicinin uyarmayacağı sessiz ölüm).

Görsel kabul üç eksende **tasarımı fiilen uyguladı**, çünkü karşılığı olmayan
yüz kullanılamazdı:

- **Çalışma anahtarı** — "Etkin" hapı tek anahtara dönüştü; kapalıyken "Devre
  dışı", açıkken "Çalışıyor" (tasarım §362). İki ayrı hap olsaydı ikisinin
  birden seçili olduğu bir durum düşünmek gerekirdi.
- **Hizalama eksenleri** — yatay/dikey hizalama rolsüz şeritten `segment_kuşağı`
  (`Role::RadioGroup` + `aria_label`) içine alındı, öğeleri `segment_simgesi`
  (`RadioButton` + `aria_selected`) oldu. `RadioGroup` içinde `Button` çocuk
  bırakmak seçim semantiğini kaybettirirdi.
- **Türetilmiş maske rozeti** — maske biçim seçiminden türer, kullanıcı doğrudan
  seçmez. Seçilebilir bir hap olarak çizmek değiştirilebilir olduğu sözünü
  verirdi; noktalı çerçeveli rozet bunun tersini söyler.

Yüz kümesi dokuzdan **on ikiye** çıktı: `kuşak` (rolsüz şerit — `segment_kuşağı`
onun rollü hâli), `durum_hapı` (hap görünümünde ama tıklanmaz, bu yüzden
`Button` rolü taşımaz) ve `segment_simgesi` eklendi.

**Yapısal maddeler TAMAMLANDI.** Dördü de kapandı ve kabul listesi
(Bölüm I `§13/1–7, 18–24`; Bölüm II `§II.12/1–2, 8–9, 12`) madde madde
ölçüldü:

- ✅ `TezgahKutusu` yüzer panel mekanizması kalktı (`f64f5be`); kartlar kalıcı
  akışta. `?` yüzeyi hâlâ ORT-006 `Araçİpucu` konağına kapılı ve çizilmiyor.
- ✅ 49 eksen bölümlere dağıtıldı; tür süzgeci iki mekanizmaya ayrıldı
  (`2747126`).
- ✅ Kod paneli sol kolonun en altında ve artık **"yalnız A bölümü"** rozetini
  taşıyor.
- ✅ `§13/3` kod paneli **gerçekten** yalnız A'yı yazıyor. İki eksik kapandı:
  `biçim` ve `üzerine_yazma` panelde hiç yoktu — kullanıcı ekseni oynatıyor,
  kod sabit kalıyordu; panel sessizce yalan söylüyordu. B (otomatik doldurma)
  ve D (tema) sızıntısı yok, test bunu koruyor.
- ✅ `§13/19` Tamsayı ailesinde ondalık derinliği artık **hiç çizilmiyor**.
  Eskiden pasif duruyordu; pasiflik "kapanan eksen" der, oysa `BiçimTanımı::Tamsayı`
  kesir taşımaz ve tür Tamsayı kaldıkça derinlik kurulamaz.
- ✅ `§13/20` ondalık derinliği `0–12` (kod `6`da duruyordu).
- ✅ `§13/22` "Tekerlekle adım" **pasif ve `AÇK-015` gerekçeli** olarak eklendi;
  eksen hiç yoktu. GPUI `ScrollWheelEvent` cihaz/kaynak alanı taşımıyor.
- ✅ `§13/23` `ParolayıGöster` yuvası yalnız `Gizli`/`GeçiciGöster`de. Bunun
  için `§22` ekseni bool'dan **dört duruma** çıkarıldı (`Açık · Gizli · Geçici
  göster · Opak`): `GeçiciGöster` ve `Opak` ekranda hiç yoktu. `Opak` bir
  "daha gizli"lik kademesi değil — değer hiç alınmamıştır, reveal yoktur.
- ✅ `§13/6` kart yüzü `min_w(px(0.))` taşıyor. GPUI'de `overflow-wrap` yok;
  uzun ve boşluksuz bir tanımlayıcı kartı genişletip kolonu taşırırdı.
- ✅ `§II.12/8` tezgâhtaki her tıklanabilir öğe rol ve ad taşıyor. Punto, yazı
  ailesi ve biçim listeleri rolsüz `div`di; `liste_öğesi` yüzüne
  (`RadioButton` + `aria_selected`) bağlandı. Biçim listesinde kurulamama
  gerekçesi artık `aria_label`da — soluk bir metin ekran okuyucuya "neden
  basılamıyor"u söylemez.
- ✅ `§II.12/9` `pasif_simge_düğmesi` gerekçeyi `let _ = başlık;` ile
  düşürüyordu: ekran okuyucuya adsız kutu, görene sebepsiz soluk simge
  kalıyordu.
- ✅ `§13/5` (`min-width: 1216px` altında yatay kaydırma) **bilinçli sapma**:
  §0.4 geçersiz-kılma tablosu `min_w`i kaldırdı. Kip bir süre
  `container_query` ile ölçülen kaptan seçildi; tezgâh kendi ekranına
  taşınınca genişlik `956px`e sabitlendi ve yatay kaydırma tasarımın
  kendi çözümü oldu (§8 borç 9).

**Kapanmayan tek madde `§13/18` — kod göç borcu (§8/16).** Tasarım "kamusal
`GirişTürü` tam olarak dört ailedir; para, yüzde ve bilimsel beşinci bir tür
DEĞİL, `Ondalık` ailesinin biçim profilleridir" diyor ve sözleşme `§7` bunu
birebir tanımlıyor: `GirişTürü::{Metin(MetinTanımı), Tamsayı(TamsayıTanımı),
Ondalık(OndalıkTanımı), TarihZaman(TarihZamanTanımı)}`.

Bu tür **kodda zaten var** (`api.rs:1393`) — eksik olan kanonik tür değil,
onu taşıması gereken alan. Sözleşme `§6` `GirişYapılandırması`'nın ilk
alanını `giriş_türü: GirişTürü` olarak yazıyor; fiziksel yapı ise eski
`değer_türü: GirişDeğerTürü`yü taşıyor ve o enum tanım taşımayan **dokuz** düz
varyanttır (`Metin, Tamsayı, Ondalık, ParaBirimi, Yüzde, Tarih, Saat,
TarihSaat, Süre`). Kanonik `GirişTürü` bugün yalnız
`ÇözülmüşGirişKısıtları.giriş_türü`nde tüketiliyor.

Tezgâh yapılandırmayı bu yapıdan kurduğu için dokuz varyantı tür düğmesi
olarak çizmek zorunda. Tek başına kapatamaz: varyantları ekrandan gizlemek
fiziksel olarak var olan bir yeteneği saklamak olurdu. Kapanış
`GirişYapılandırması`'nın kanonik şekle göçüyle gelir; şekil faz-3 kapısında
`// şekil struct GirişYapılandırması ac9343af…` olarak zaten kayıtlıdır ve
kapı **147 hatayla** açıktır. Borç §8'e yazıldı.

*Kabul:* Yapısal alt-kabul Bölüm I §13/1–7, 18–24; Bölüm II §II.12/1–2,
8–9, 12 ve §4'teki geniş/dar × hedef × ölçek koşumudur. **F1 tamamlanma
kapısı**, F0b'nin ORT-003/004/017 görsel kapılarını da geçirir — bu kapı
görsel kabulle **geçildi** (127 galeri testi yeşil). `?` yüzeyi için
ayrıca ORT-006.ACC-013, 041, 055 ve ORT-021 güncellik kanıtı gerekir; fiziksel
kapı kapalıyken bu alt kabul **kapılıdır** ve tetikleyici çizilmez.

### F2 · §16.2 durum göstergesi — **geliştirici gözlem paneli**

**TAMAMLANDI.** Bölüm `durum_gostergesi` (`Akış::A`) eksen ile gözlemi aynı
kartta taşıyor: kullanıcı ankrajı değiştirdiğinde sonucun hemen altında
değiştiğini görür.

- **F2.1** `gösterge_ankrajı: Option<DurumGöstergesiYerleşimTercihi>` +
  `gösterge_açıklaması` tercihleri kanonik `durum_göstergesi` alanına
  çevriliyor ve kod paneline yazılıyor. Ankraj üç durumlu ama iki düğme:
  `gösterge_ankrajına_bas` seçiliye ikinci basışta `None`'a indiriyor. Üçüncü
  bir "Kapalı" düğmesi kapalılığı ankrajla eşdeğer bir kademe gibi
  gösterirdi; oysa kapalılık alanın yokluğudur (`§16.2.4`).
- **F2.2** Panel `yerleşim` ile `birincil sorun var/yok` yazıyor; gerekçe,
  sorun kimliği, sürümler ve ileti metni yok. Yasak düzyazıda değil testte:
  `panel_yasak_alanlari_okumaz` panel gövdesinde `.gerekçe()`,
  `.değer_sürümü()`, `.sorun_sürümü()` ve `gösterge_girdisi_sürümü` arıyor —
  bir sonraki düzenleme sessizce ekleyemesin.
- **F2.4** doğrulandı: `GirişYüzeyBağı` fiziksel API'de **yok** ve
  `GirişYapılandırmaHatası` (16 varyant) `GirişYüzeyBağıEksik` taşımıyor.
  Açıklama tercihi seçilebilir kalıyor — yapılandırma alanı gerçek — ama
  `SağlayıcıVarsayılanı` seçilince "yüzey açılmaz · GirişYüzeyBağı fiziksel
  değil" rozeti çıkıyor. Sahte balon kurulmadı.
- **F2.5** `bağlam.observe(&alan, …)` tezgâh alanı kurulurken bağlanıyor.
  Panel sonucu saklamıyor; `panel_sonucu_saklamaz` galeri kaynağında
  (yorumlar elenerek) `DurumGöstergesiDurumu` arıyor.

*Koşum:* `tests/tezgah_gosterge.rs` — 9 test; galeri toplamı **144 yeşil**.

**Fiziksel sınır.** `GirişKutusu::render` göstergeyi **çizmiyor**: mantıksal sıra
`ön ek → içerik → son ek → sayaç → yardımcı eylemler` ile bitiyor
(`bileşen.rs:2146–2255`). `§16.2.1`'in koşullu `Gösterge` parçası fiziksel
render'da yok.

Bundan iki ayrı iş çıkar ve **karıştırılmamalıdır**:

| | İş | Sahibi | Bu planda |
|---|---|---|---|
| **A** | Gösterge sonucunu salt-okunur **gözlem olarak** sunmak | Galeri (tüketici) | **F2 kapsamı** |
| **B** | Göstergeyi canlı `GirişKutusu` anatomisinin parçası olarak **çizmek** | Kanonik `BİL-010` | **Kapsam dışı · atom borcu** |

Galerinin B'yi yapması, tüketicide **ikinci bir görsel uygulama** kurmak olurdu:
planın kendi §1.2 hükmünü (kanonik davranış değişmez) ve `YÖN-006.ACC-006`yı
ihlal eder. Teknik olarak da mümkün değil — `Entity<GirişKutusu>` opaktır,
galeri onun kabuğunun içine çocuk ekleyemez.

#### F2.1 · Yapılandırma ekseni (A bölümü · canlı)

`GirişYapılandırması.durum_göstergesi: Option<DurumGöstergesiYapılandırması>` —
`None` / `SatırSonu` / `UygunsaÜstKöşe` + açıklama tercihi. Seçili ankraja
yeniden basmak `None`'a indirir. Bu eksen bugün fiziksel ve koda yazılır.

#### F2.2 · Gözlem paneli — **ne gösterilebilir, ne gösterilemez** (III/1 · VI)

`§16.2.5` tanı zarfını dar tutuyor: *"Tanı zarfı kullanıcı iletisini, sorun
kimliğini, değer sürümünü, fiziksel sınırları veya ham profil değerlerini
taşımaz; yalnız gerekçe sınıfı ile ilgili anatomi/geometri sürümünün izinli
kayıtlı kodunu taşır. Yerleşim gerekçesi kullanıcıya sunulan açıklamanın parçası
değildir."*

Bu panel bir ORT-019 `TanıZarfı` üretmez veya taşımaz; galerinin kamusal opak
sonuç için salt-okunur geliştirici gözlemidir. Yine de kullanıcı iletisi,
sorun kimliği, sürüm ve ham profil değeri yüzeye çıkarılmaz. Ayrım:

| Okunan | Panelde | Gerekçe |
|---|---|---|
| `yerleşim()` | **gösterilir** | Yapılandırma sonucudur, tanı zarfı değil |
| `gerekçe()` | **gösterilmez** (IV/2) | `§16.2.5` gerekçeyi *yalnız kayıtlı ORT-019 koduna* eşlenebilir kılıyor; `ORT-019` kayıtlı küme dışındaki girdiyi `DeğerPolitikasıUyuşmuyor` sayar ve serbest kod fallback'ini reddeder. Kod kümesi fiziksel olmadığı için ham enum adı gösterilemez — "geliştirici tanısı" etiketi kayıt kapısının yerine geçmez |
| `birincil_sorun()` | **yalnız var/yok** (`is_some()`) | Sorun kimliği yasak |
| `değer_sürümü()`, `sorun_sürümü()` | **gösterilmez ve karşılaştırılmaz** | Değer sürümü açıkça yasak; güncellik ödünç okumayla sağlanır (F2.5) |
| `gösterge_girdisi_sürümü` | **okunamaz** (IV/1) | Alan hem kanonikte hem fizikselde private; getter'ı yok |
| İleti metni | **gösterilmez** | Tanı zarfı kullanıcı iletisi taşımaz |

Panel iskeleti:

```
§16.2 gösterge çözümü
  yerleşim        : Yok | SatırSonu | ÜstKöşe
  birincil sorun  : var | yok                   ← kimlik yazılmaz
```

Gerekçe satırı, `ORT-019` kayıtlı tanı kodu kümesi fizikselleşene kadar
**panelde yer almaz**. O gün geldiğinde satır kodun kendisiyle açılır, enum
adıyla değil. (Gerekçe testlerde okunabilir — test tanı zarfı değildir.)

Kart künyesi: "kanonik opak sonucun salt-okunur geliştirici gözlemi; ORT-019
tanı zarfı veya kullanıcıya sunulan açıklama değildir". Panel opak sonucu
yeniden yorumlamaz, ikinci çözücü kurmaz ve tanı göndermez.

> **Not:** Kanonik ORT-019'da ayrı bir `TanıKodu` tipi yoktur. Exact yol
> `TanıDeğeri::Kod(TanımKimliği)` ile mühürlü `TanıZarfıFabrikası`nın private
> alan/kod sicilidir. Fiziksel temel crate hâlâ serbest `GüvenliKod`/
> `TanıOlayı` modelini taşır; exact fabrika ve private eşleme fiziksel değildir.
> Bu, gerekçeyi göstermemenin **nedenidir**, geçici gevşetmenin gerekçesi değil:
> kayıtlı küme dışındaki girdi `DeğerPolitikasıUyuşmuyor` sayılır.

#### F2.3 · Gerekçe matrisi — **test kapsamı**, panel içeriği değil (V/1)

Bu tablo `tests/tezgah_gosterge.rs`'in kapsamıdır. **Hiçbir satırı ekranda
görünmez** — gerekçe panelde gösterilmez (F2.2). Test tanı zarfı değildir,
bu yüzden gerekçeyi okuyabilir.

| # | Gerekçe | Bugün üretilebilir mi | Testte |
|---|---|---|---|
| 1 | `YapılandırmaylaKapalı` | ✅ | sınanır |
| 2 | `BirincilSorunYok` | ✅ | sınanır |
| 3 | `SatırSonuTercihEdildi` | ✅ | sınanır |
| 4 | `ÜstKöşeAdayıYok` | ✅ | sınanır |
| 5 | `ÜstKöşeGeometrisiUygunDeğil` | ❌ | sınanamaz — aday beslemesi yok (K2) |
| 6 | `ÜstKöşeAnatomiyleÇakışıyor` | ❌ | sınanamaz — aday beslemesi yok (K2) |
| 7 | `ÜstKöşeUygun` | ❌ | sınanamaz — aday beslemesi yok (K2) |

5–7 üretilemez çünkü fiziksel kabuk kayıtlı üst-köşe geometri adayı
sağlamıyor; kanonik uygulama fail-closed `ÜstKöşeAdayıYok` yayımlıyor
(`api.rs:2302–2307`). Panelde bunun görünen tek sonucu, `UygunsaÜstKöşe`
seçiliyken bile `yerleşim`in `SatırSonu` kalmasıdır.

#### F2.4 · Açıklama yüzeyi — kurulmaz

- `GirişYüzeyBağı` fiziksel API'de yok (yalnız `sozlesme_api_faz3.rs:366`
  bekliyor; kapı 146 hatayla kırmızı).
- `GirişYüzeyBağıEksik` **fiziksel `GirişYapılandırmaHatası`nda da yok**.
  Bu yüzden ekranda gösterilecek şey "sözleşmenin ürettiği kuruluş reddi" değil,
  **"kanonikte beklenen, fiziksel API'de henüz bulunmayan sonuç"** etiketidir.
  Plan 2. sürümünün "taklit değildir, zaten sözleşmenin ürettiği sonuçtur"
  ifadesi bugün doğru değildi; düzeltilmiştir.
- Yerel sahte baloncuk ve sessiz `Yok` fallback'i `§16.2.4` ile yasaktır.
  Girdi tasarımının "simülasyon balonu" tarifi geçersizdir (§0.4).

#### F2.5 · Canlılık bağı — ödünç okuma (III/6 · IV/1)

Panel sonucu **saklamaz**. Her çizimde `durum_göstergesi_durumu()` ödünç okunur;
`DurumGöstergesiDurumu` zaten `Clone` taşımayan opak bir ödünçtür ve sözleşme
onu "ödünç güncel sonuç" diye tanımlar. Saklanmayan sonuç bayatlayamaz.

```rust
// Tezgâh kurulurken bir kez: alan bildirdiğinde tezgâh yeniden çizilir.
cx.observe(&self.alan, |_bu, _alan, cx| cx.notify()).detach();
```

`observe`, `Entity`'nin `notify` bildirimini dinler; galerideki var olan
`subscribe` deseni (`lib.rs:1515`) **olay** tabanlıdır ve bunun yerine geçmez —
ikisi ayrı kanallardır.

**Sürüm karşılaştırması yapılmaz.** Üç sebeple:

1. `gösterge_girdisi_sürümü` hem kanonikte hem fizikselde **private**, getter'ı
   yok; kamusal olarak yalnız `değer_sürümü()` ve `sorun_sürümü()` okunabiliyor.
   Eksik parçayla yapılan karşılaştırma "kökün her parçası güncel" iddiasını
   kuramaz.
2. Sürümleri panelde göstermek zaten yasak (F2.2).
3. Ödünç okuma karşılaştırmayı gereksiz kılar.

*Kanıt:* `tests/tezgah_gosterge.rs` — yapılandırma değiştiğinde panelin yeni
sonucu gösterdiği; galeri tarafında `DurumGöstergesiDurumu`'nun hiçbir alanının
kopyalanıp saklanmadığı (yapısal kanıt).

#### F2.6 · Kanonik atom borcu — **üç ayrı kapı** (IV/5)

Planın 4. sürümü dört işi tek blok sayıyordu. `§16.2.4` bunu ayırıyor:
*"`DurumGöstergesiAçıklamaTercihi::Yok` yalnız görsel göstergeyi kullanır."*
Yani görsel gösterge, yüzey bağı olmadan çalışır. Üç ayrı kapı:

| Kapı | İçerik | Neyi açar | Bağımlılığı |
|---|---|---|---|
| **K1 · Temel render** | `GirişKutusu::render`'a `§16.2.1` koşullu `Gösterge` parçası; kayıtlı `ORT-017 AnatomiParçasıSınıfı::Gösterge` anatomi girdisi; aynı snapshot'tan iki bağlayıcı alt sınır: `göstergesiz_asgari_mantıksal_genişlik` ve `satır_sonu_göstergeli_asgari_mantıksal_genişlik` (`§16.2.2`) | `SatırSonu` ankrajında **canlı görsel gösterge** (`AçıklamaTercihi::Yok` ile tam çalışır) | **ORT-017 temel anatomi/metrik göçü.** K2/K3'ten bağımsız, ama sıfır bağımlı değil: fiziksel kodda yalnız eski `ParçaSınıfı::Gösterge` ve `GörünümHatası::AnatomiUyumsuz` karşılıkları var; exact `AnatomiParçasıSınıfı`, `GörünümKayıtHatası` ve iki alt sınır metriği **yok** |
| **K2 · Üst-köşe adayı** | `ORT-017` kayıtlı geometri adayı beslemesi | `UygunsaÜstKöşe` yolu ve gerekçe 5–7 | K1 |
| **K3 · Açıklama yüzeyi** | `GirişYüzeyBağı` + `GirişYapılandırmaHatası::{GirişYüzeyBağıEksik, GeçersizDurumAçıklamaProfili}` + exact ORT-006/021 yüzeyi | `SağlayıcıVarsayılanı` açıklaması; §13/14–17 | K1; K2'den bağımsız; ORT-006/021 fiziksel göçü |

K1 tek başına en büyük kazancı verir: tezgâhta gösterge **gerçekten görünür**.
K2 ve K3 ona eklenir, sırası serbesttir. K1'in kendisi `ORT-017` temel
anatomi/metrik göçüne bağlıdır — 5. sürümün "bağımlılık yok" ifadesi yanlıştı.

**Karar:** üç kapı da göç planının kapsamı dışındadır; ayrı `BİL-010`
atom(lar)ı olarak yürütülür. Bu borç yeni bir `AÇK-*` kaydı değildir; sözleşme
sicilini genişletmek ayrı kullanıcı yetkisi ister.

*Kabul (F2 · bu planda):* §13/8, 12, 13 (rezervsizlik yerine: gözlem panelinin
`Yok` çözümünde hiçbir görsel iddia üretmemesi; `None`'a iniş; yalnız birincil
sorun). §13/9–11, 14–17 **kanonik atom borcuna** taşındı.
*Kanıt:* `tests/tezgah_gosterge.rs` — dört canlı gerekçe, `Yok` çözümünde panel
metni, `UygunsaÜstKöşe` seçiliyken yerleşimin `SatırSonu` kaldığı, panelde
gerekçe metni bulunmadığı (negatif iddia) ve galeri sandığında
`DurumGöstergesiYerleşimGerekçesi` üreten kod olmadığı.

### F3 · Kalan A bölümü eksenleri

**On üç eksen TAMAMLANDI, dördü kapılı.**

| Eksen | Durum |
|---|---|
| `s6` harf dönüşümü, kırpma, boş giriş | ✅ `harf_dönüşümü` sayısal türde kapanır |
| `s10` yapıştırma + `dil_etiketleri` | ✅ sabit deneme kümesi (`tr-TR` → `en-US`) |
| `s17` escape, geçersiz odak | ✅ |
| `s23` başlangıç/bitiş bölütü, arama gönderimi | ✅ gönderim `AramayıBaşlat` yuvasına bağlı |
| `s24` seçici uyarlaması, erişilebilir ad | ✅ uyarlama yuvaya bağlı; ad kod paneline girdi |
| `s15` doğrulama (zorunluluk, tetikleyici, önem) | ✅ kural kimliği `2` |
| `s9` tür alt tanımları (4 eksen) | ⛔ **§8/16 borcuna kapılı** |

`s9`'un dört ekseni — `metin_içerik_türü`, `tamsayı_tanımı`, `ondalık_tanımı`,
`tarih_zaman_tanımı` — kanonik `GirişTürü` varyantlarının **payload**'ıdır.
`GirişYapılandırması` `giriş_türü: GirişTürü` taşımadığı için (borç 16) bu
tanımların yazılacağı bir alan yok; tezgâh onları kuramaz.

Üç kanonik sınır bu fazda kayda geçti:

- **Bölüt içeriği** ayrı bir `BitişikEylemBölütü` tipinde ve
  `GirişYapılandırması`'na bağlı değil; kuşak yalnız `başlangıç`/`bitiş`
  taşıyor. Tasarımın `https://` örneği ekranda örnek olarak durur,
  yapılandırmaya girmez.
- **`BölütKonumu`** (tasarımın türetilmiş `Tek · Orta · Bitiş` rozeti)
  fiziksel olarak **yok**; rozet çizilmedi.
- **`AçılırYüzeyYapılandırması`** `gpui_bilesenleri`'den dışa açılmıyordu.
  `SeçiciUyarlaması.yüzey` kamusal bir alan olduğu hâlde türü adlandırılamıyor,
  yani tüketici o yapıyı kuramıyordu. Re-export eklendi.

*Koşum:* `tests/tezgah_f3.rs` — 11 test.

### F4 · B, C, D bölümleri

**TAMAMLANDI.** Üç kart da eklendi. Ortak özellikleri: hiçbiri
`GirişYapılandırması`'na yazılmaz ve kod paneline girmez — bunu bir test
koruyor.

- **B · Port kapıları** (`Akış::B`) — dört port ayrı ayrı raporlanır:
  `§6.1` otomatik doldurma, `ORT-002 §5.2` saat dilimi, `ORT-004 §20.1`
  imleç, `§15` uzak doğrulama. "Portlar hazır" diye toplu bir bayrak hangi
  yolun kapalı olduğunu gizlerdi. Rozetler **seçilemez**: port varlığı bir
  tercih değil, platformun bildirimidir. Kart kapalı portta da çizilir —
  gizlenen kapı o yolun hiç olmadığı izlenimini verir (`ACC-005`).
  Uzak doğrulama galeride hiç kurulmuyor ve `false` sabit: galeri sahte
  sunucu taklit etmez, benzersizlik ve iş kuralı ürünün bilgisidir.
- **C · Türetilmiş durumlar** (`Akış::C`) — `görsel_durum()`,
  `sorunlar().len()`, `metin_mutasyonuna_izin_var()`, `odaklanabilir()`
  canlı alandan **ödünç** okunur. Seçilebilir eksen yok: bunlar modelden ve
  etkileşimden türer. Yapılandırılabilir olan yalnız `salt_okunur` ve
  `etkin`, ikisi de `§17–20` bölümünde kalır — aynı ekseni iki yerde çizmek
  hangisinin gerçek olduğunu belirsizleştirirdi.
- **D · Aile kataloğu** (`Akış::C`) — aile kaynağı (kitaplık kataloğu /
  işletim sistemi / çözülemedi), kitaplık ve sistem aile sayısı. Ayrı
  tipografi **yapılandırma** bölümü açılmadı (`AÇK-009`); kart önizleme
  bağlamını raporlar.

**Fiziksel sınır:** `ACC-034`'ün `MerkeziFallback` rolü kanonik kodda
**yok**. Rozet bu yüzden rolün adını değil ailenin kaynağını yazar ve rolün
fizikselleşmediğini söyler; sessiz fallback de yapılmaz. Rol adını rozet
değeri olarak yazmak, olmayan bir kademeyi varmış gibi göstermek olurdu.

*Koşum:* `tests/tezgah_f4.rs` — 4 test.

### F5 · §29 tablosu ve kabul turu

**§29 tablosu TAMAMLANDI · kabul turu koşuldu.**

`§29` kartı `akis-c`nin son kartı olarak eklendi ve **canlıdır**: kanonik
`doğrula()` raporundan gelir, hata ile uyarıyı ayrı çizer. Rapor bir kez
kurulur ve karta hazır verilir — kartın yapılandırmayı ikinci kez kurması
ekranda gösterilenle uygulanan arasında sessiz bir fark açardı.

Ham enum adı yazılmaz: `ÇakışanİçerikYuvası` programcıya bir şey söylemez,
"İki içerik aynı yuvayı istiyor" söyler. `çelişki_metni`/`uyarı_metni`
tasarımın `§8.15` çelişki/sonuç sütunlarını izler.

**Fiziksel sınır:** tasarımın tablosu 27 satır sayıyor; fiziksel
`GirişYapılandırmaHatası` **16**, `GirişYapılandırmaUyarısı` **4** varyant
taşıyor. Aradaki fark statik bir liste olarak yazılmadı — üretilemeyen bir
çelişkiyi "kural var" diye göstermek, olmayan bir denetimi varmış gibi
satmak olurdu.

**Yerleşim düzeltmesi.** Kabul turu iki sapma yakaladı ve ikisi de
düzeltildi: `§29` önce `TamGenişlik` çizilmişti, oysa tasarımın `§5`
yerleşiminde tam genişlik yalnız `§7` ve `§9`'a ayrılmış; `C` türetilmiş
durumlar ile `D` aile kataloğu ise sağ kolonun eksenleri arasındaydı, oysa
`§5` şeması sol kolona `önizleme → C türetilmiş durumlar → kod paneli`
sırasını veriyor. İkisi de bağlam kartıdır: sağ kolonda eksenlerle aynı
görsel ağırlığı taşır ve seçilebilir sanılırlardı.

*Kabul turu:* `tests/tezgah_kabul.rs` — 8 test. Maddelerin çoğu kendi
fazının dosyasında kanıtlı; burada oralarda karşılığı olmayanlar ile **kapı
bekçileri** var. Kapılı madde atlanmadı: neden kapılı olduğu yazıldı ve
kapının hâlâ kapalı olduğu sınandı — kapı açıldığı gün test düşer.

| Madde | Durum |
|---|---|
| `§13/1–2` | HTML'e özgü; Rust karşılığı `F1` görsel kabulünde |
| `§13/3–4, 6–7` | ✅ |
| `§13/5` | bilinçli sapma (`§0.4`); `min_w` dayatılmadığı sınanıyor |
| `§13/8–17` | ⛔ kanonik render göstergeyi çizmiyor (`F2.6` K1) — bekçi test |
| `§13/18` | ⛔ `§8/16` kod göç borcu |
| `§13/19–24` | ✅ |
| `§13/25` | ✅ "rezervli şerit" ifadesi hiçbir kaynakta yok |
| `§13/26–28` | belge ve çalışma zamanı maddeleri; kaynak denetimine girmez |
| `§II.12/1–2, 8–12` | ✅ |
| `§II.12/3–7` | ⛔ aynı kapı; galerinin sağladığı, çizmediğini iddia etmemek |

**Kalan:** ekran görüntüsü ve `YÖN-005 UyumBulgusu` kaydı. Ekran görüntüsü
GUI koşumu ister ve ayrı bir adımdır.

---

## 7. Model genişlemesi

`TezgahTercihleri` bugün **49 alan** taşıyor. Fazlara göre:

| Faz | Eklenen |
|---|---|
| F1 | Yapısal ayrım: `TezgahTercihleri { a, b, c, d }` — A bölümü `GirişYapılandırması`na yazılır, B/C/D kod üretimine **girmez**. |
| F2 | `a.durum_göstergesi: Option<DurumGöstergesiYapılandırması>` — **yalnız bu**. `GöstergeÇözümü`/`YerleşimGerekçesi`/`ÜstKöşeAdayı` galeride **tanımlanmaz**; geliştirici gözlem paneli kanonik opak sonucu okur ve ORT-019 tanısı üretmez. |
| F3 | `metin_içerik_türü`, `tamsayı_tanımı`, `ondalık_tanımı`, `tarih_zaman_tanımı`, `harf_dönüşümü`, `kırpma`, `boş_giriş`, `yapıştırma`, `dil_etiketleri`, `doğrulama_kuralları`, `escape`, `geçersiz_odak`, `başlangıç_bölütü`, `bitiş_bölütü`, `arama_gönderimi`, `seçici_uyarlaması`, `erişilebilir_ad` |
| F4 | `c.özel_durum`, `c.etkileşim_durumu`, `c.sorun_senaryosu`, `d.hedef`, `d.aile`, `d.parça_tipografisi` |

---

## 8. Riskler ve açık borçlar

| # | Konu | Etki | Karşılık |
|---|---|---|---|
| 1 | `GirişKutusu::render` göstergeyi çizmiyor (`bileşen.rs:2146–2255`) | Tezgâhta gösterge **canlı görünemez** | F2 geliştirici gözlem paneline indirgendi; canlı çizim kanonik atom borcu (F2.6) |
| 2 | Üst-köşe adayı beslemesi yok | 7 gerekçeden 3'ü üretilemez | F2.3 pasif + gerekçeli; besleme gelince kendiliğinden canlanır |
| 3 | `GirişYüzeyBağı` fiziksel değil (faz-3 kapısı **146 hata**) | Açıklama yüzeyi kurulamaz | F2.4; sahte balon yok |
| 4 | `GirişYüzeyBağıEksik` varyantı fiziksel değil | "Kuruluş reddi" gösterilemez | "Kanonikte beklenen, fizikselde yok" etiketi (F2.4) |
| 5 | `AÇK-015` — tekerlek olayında cihaz/kaynak yok | "Tekerlekle adım" çalışamaz | Pasif + gerekçeli; `on_scroll_wheel` kurulmaz |
| 6 | `AÇK-009` — tipografi sahipliği kapalı | Tipografi bölümü açılamaz | D'de yalnız otorite simülasyonu |
| 7 | Kod göçü borcu (pin `78b6d15`) | Kanonik yüzey eski sözleşmeye göre | Tezgâh yapılandırma yüzeyini çizer |
| 8 | 1106 ölçüt kanıtsız | Yeşil denetim ≠ uygulanmış sözleşme | Tezgâh kanıtı yalnız kendi ekseninde iddia eder |
| 9 | ~~Gömülü tezgâh dar orta bölgede~~ **KAPANDI** | Tezgâh galeri içine gömülü değil; kendi ekranı ve sabit `956px` genişlikte, altında yatay kaydırma var | Uyarlanabilir kolon kaldırıldı: `YerleşimKipi`, `iki_kolon_eşiği`, `kip()` ve `AkışDağılımı` çağrısız kalmıştı ve testleri yeşildi — ölçtükleri kod ekranda kullanılmıyordu. Dördü de silindi |
| 10 | Girdi kaynakları kısmen bayat | Yanlış hükmü yürürlükte sanma riski | §0.3 sicili + §0.4 geçersiz-kılma tablosu |
| 11 | ORT-004/017 fiziksel kodu kanonik yüzeyin gerisinde; `TipografiRolü`, `GörselOpaklıkKademesi`, exact parça/hata tipleri ve `GörünümKayıtDefteri` uygulayıcısı yok | Tezgâh profilinin kanonik biçimi derlenemiyor ve kayıt kapısından geçemiyor | §3.3.1/5 kapılı; yerel eşadlı tip/ham opaklık fallback'i yok |
| 12 | ORT-019 exact `TanıDeğeri::Kod` + mühürlü `TanıZarfıFabrikası`/private sicil fiziksel değil; eski serbest `GüvenliKod` modeli var | Gerekçe kayıtlı `TanımKimliği`ne güvenle eşlenemiyor | **Gerekçe panelde gösterilmez**; exact fabrika/sicil yolu gelirse satır yalnız kayıtlı kimlikle açılır (F2.2) |
| 13 | ORT-003 exact `KutuŞekliGeometrisi` fiziksel değil | Yeni hap/kart yüzünün ortak boya/hit-test tek sahipliği kanıtlanamaz | F0b kapılı; `.rounded_full()` veya eski `KutuŞekliSonucu` yeni yüz için fallback değildir (§3.4) |
| 14 | ORT-006/021 exact tooltip ve çözülmüş ileti yüzeyi fiziksel değil | `?` yardım balonları kanonik konaktan açılamaz | F1 alt kabulü kapılı; yerel `deferred(anchored(..))`/`TezgahKutusu` fallback'i ve ölü tetikleyici yok (§3.4) |
| 16 | ~~`değer_türü` düz dokuz varyant~~ **KAPANDI** (`287bc7f`, `392f9e1`) | `GirişYapılandırması` artık sözleşme `§6`'daki gibi `giriş_türü: GirişTürü` taşıyor; para/yüzde tür değil `Ondalık` + ORT-008 biçim profili (birim `ParaBiçimi`nin içinde, fiziksel-yalnız `para_birimi` alanı emekli), `giriş_türünü_çöz` silindi ve içerik türü/bit genişliği çözümde kaybolmuyor | Dokuz ekran kipi `TezgahDeğerKipi` UI modeli olarak korunur; kod paneli gerçek `GirişTürü` kuruluşunu yazar. `ORT-008 2.2.0` kullanıcı yetkili revizyonuyla `İşaretKonumu` ekseni eklendi (`38b91d1`): ₺ ve % işareti biçimde önde ya da sonda temsil edilir, tezgâh seçicisiyle canlı (`₺7.890,12` ↔ `7.890,12 ₺`, `%50` ↔ `50%`) |
| 15 | Kanonik `GörselOpaklıkKademesi` `Clone`/`Copy` değil ve `düşük_taban(self)` tüketiyor; `Profil: Clone` + `çöz(&Profil)` ile birlikte kullanılamıyor | F0b, exact sözleşme fiziksel olarak kopyalansa bile derlenebilir profil/çözüm kuramaz | ORT-017 sahibinde imza kapanmadan uygulama yok; alan-içi `Arc`, ikinci sayı ve ham opaklık fallback'i yasak (§3.3.2–3) |
| 17 | Tezgâhın açılır listeleri (`şerit_seçicisi`) yerel bir `deferred(...)` + `occlude()` katmanıdır; `ORT-006` yüzer yüzey konağına bağlanmaz | Liste **dış tıklamayla kapanmıyor**: pencereyi kaplayan bir kapatıcı katman GPUI hit testinde listenin kendi tıklamalarını da yutuyor. Beş düzen denendi (kök katman, `deferred` öncelikleri, kardeş sıralaması, `on_mouse_down_out`, `on_mouse_down`) — hepsinde ya liste kapanmıyor ya içindeki seçim çalışmıyor. Kapanış tetikleyiciye ikinci tıklama, başka bir seçicinin açılması ve **seçim yapılması** ile olur — seçim sonrası kapanma `tezgahı_değiştir` içinde kuruldu; dış tıklama borcun kalan kısmıdır | Kapanış `ORT-006` konağı fiziksel olduğunda oradan gelir: dış tıklama, `Escape` ve odak kaybı yüzeyin kendi sözleşmesindedir ve hit-test sırası konağın işidir. Yerel kapatıcı katman **kurulmaz** — çalışmıyor ve ikinci bir kapanış otoritesi olurdu. Liste `BİL-020` seçim listesi sözleşmesi geldiğinde o konağa taşınır (bkz. borç 14, aynı `ORT-006` kapısı) |
| 18 | ~~`dış_hata_temizleme` ve `geçici_gösterim` okunmuyor~~ **KAPANDI** | `geçici_gösterim` (`112e976`): üç politika fiziksel, `GeçiciGöster` politikasız kurulamaz, tezgâh "Geri dönüş politikası" satırını çizer. `dış_hata_temizleme` (`c735eb5`): sorun deposu kaynakla bölümlendi, `§28` yüzeyi kaynaktan türetilir (`YerelGeçersiz > DışHata`), politika `metni_değiştirdi`ye bağlı ve `§19` Escape oradan geçer. Tezgâh sunumu (`5e4206c`): **kullanıcı kararıyla** "galeri sahte sunucu taklit etmez" duruşu bu eksen için esnetildi — Dış doğrulama kartı sabit bir `Sunucu` sonucunu gerçek port yolundan geçiren **gösterim beslemesi** taşır ve gerçek sunucu olmadığını açıkça yazar | Davranış başsız beş testle, tezgâh akışı WASM'da uçtan uca ölçüldü (bildir → `DışHata`; varsayılanda yazı temizler, Koru'da yeni bildirim kapatır). Benzersizlik ve iş kuralı yine ürünündür |
| 25 | `§22.1` programatik köken kapısı fiziksel değil: `DeğerKökeni` hiçbir yerde tüketilmiyor ve kurucu gizli alana da başlangıç metni alıyor (`§28.2` derleme kapısı kod göçü borcunda) | Geçici gösterim politikası programatik kökenli gizli değeri de açabilir; sözleşme bunu yasaklıyor (`GeçiciGöster` o değeri açamaz, `ParolayıGöster` sunulmaz) | Kanonik atom borcu; geçici gösterim ekseni açılırken bilinçli kapsam dışı bırakıldı (`112e976`). Köken takibi eklendiğinde kapı `yardımcı_yuvalar` görünürlüğüne ve politika çözümüne bağlanır |
| 19 | Sözleşmenin `§40` (ORT-013 yerel geçmiş dikişi) ve `§41` (ORT-004 işaretçi imleci) bölümleri fiziksel değil: `YerelGeçmişPortu` mühürlü bir trait ve **hiçbir tip onu uygulamıyor**, `İşaretçiİmleciHedefKimliği` ile `İşaretçiİmleciCapabilitySnapshotı` yalnız faz-1 kapı testinde şekil mührü olarak var, kaynakta yok | Tezgâh geri alma yığınını ve I-beam niyetini gösteremez; ikisi de `§31` test matrisinde ölçülüyor ama ekranda karşılığı kurulamaz | Sahibi kanonik atom; port uygulanınca tezgâh geri alma adımını değer durumu kartından okur, imleç niyeti `ORT-004` konağı geldiğinde açılır. Yerel bir taklit **kurulmaz** — mühürlü trait zaten buna izin vermiyor ve ikinci bir geçmiş sahibi olurdu |
| 20 | Pasif denetimlerin gerekçesi yalnız `aria_label`'da: `hap_pasif` ve `pasif_simge_düğmesi` gerekçeyi erişilebilir ada koyar, ekrana yazmaz | Gören kullanıcı bir düğmenin **neden** pasif olduğunu okuyamıyor ve bozuk sanıyor — göz simgesi, tekerlekle adım, üst-köşe adayı ve on kadar eksen aynı durumda | Üçü için kart altına görünür not yazıldı (yuva görünürlüğü, göz simgesi, gösterge çizimi). Genel çözüm `ORT-006` araç ipucu konağıdır (borç 14 ile aynı kapı): pasif öğe üzerinde gerekçe balonu. O konak gelene kadar her gerekçeyi kart altına yazmak kolonu doldurur; yalnız kullanıcıyı fiilen yanıltanlar yazılır |
| 22 | Galeri kataloğu **ulaşılamaz**: `GaleriUygulaması.tezgah_ekranı` hep `true` ve `render` erken dönüyor; hiçbir yol onu `false` yapmıyor | `aile_sergisi` ile altındaki 31 sergi fonksiyonu (ORT laboratuvarı, kabuk simülasyonu, on üç bileşen sergisi) çizilmiyor ve testlerden de çağrılmıyor — `YÖN-006 §3` bilgi mimarisi ekranda yok | Kullanıcı kararı: "Uygulama doğrudan tezgâh olsun." Kod **silinmiyor** çünkü `YÖN-006 §3` tamamlanmış bir sözleşme ve kataloğun akıbeti ayrı bir revizyon atomudur (`YÖN-001 §7.1`). Bayrağın kendisi de kaldırılmıyor: kaldırmak kararı koda gömer. Borç 4 (rota çelişkisi) ile aynı hattadır |
| 21 | ~~Tezgâh yüzlerinde ham ölçü~~ **KAPANDI** | Ham ölçüler `ORT-004` metin ölçeğinden etkilenmiyor: `%200`de o satırlar küçük kalıp ekranı yarı ölçekli bir karmaya çeviriyordu | İlk sayım (97) yanlıştı: `sergiler.rs` hem eski galeri sergilerini hem tezgâhı taşıyor ve 88'i sergilere aitti. Tezgâh yüzlerindeki **12** nokta profil rollerine bağlandı; `tezgâh_yüzleri_ham_ölçü_taşımaz` testi yüzleri adla sayarak sınıfı kapalı tutuyor. Eski galeri sergileri kapsam dışı — onlar `YÖN-006 §3` bilgi mimarisiyle birlikte ayrı bir atomda ele alınır |
| 23 | ~~Sağ kolonun on bölümü hiç çizilmiyordu~~ **KAPANDI** | `overflow_y_scroll` kabındaki üç akış bloğundan yalnız ilki yer alıyor, sonraki ikisi flex sıkışmasıyla sıfır yüksekliğe iniyordu. Kaydırma sınırı da ilk akışın sonunda hesaplandığı için ekranda hiçbir ipucu yoktu: kolon bitmiş gibi duruyordu. Görünmeyenler yapıştırma, port kapıları, hacim ve sayaç, içerik görünürlüğü, otomatik doldurma, yapılandırma doğrulaması, seçici ve erişilebilirlik, sayısal adım, odak/kabul/erişim, saat dilimi | `akış_bloğu` sıkışmayı kapatıyor. Yerleşim ölçüsü test API'sinden okunamadığı için bildirim kaynakta doğrulanıyor (`akış_blokları_sıkışmaz`). Bu bölümler hiç çizilmediği için hiç denenmemişti; gizli içerik, göz yuvası ve `ParolayıGöster` rozeti WASM'da tek tek doğrulandı |
| 24 | ~~Yoğunluk ve hareket tercihleri hiçbir şey yapmıyordu~~ **KAPANDI** | Üst şeritteki altı düğme (üç yoğunluk, üç hareket) seçili görünüyor ama tema kurulumuna hiç girmiyordu: `TezgahTeması.yoğunluk` ve `.hareket` yalnız kendi seçicilerinde okunuyordu | `ORT-004 §25` anlık görüntüsü ikisini de taşıyor. Hareket `imleç_çözümü` üzerinden metin imlecine iniyor; yoğunluk tezgâhın kendi görünüm profilinde `DolguÖlçeği`ne çözülüyor (`§43`: sayısal karşılık profilde, çizim koduna dağılmış ham `px` farkı değil). Etkileşim hedefi ve simge kutusu sabit — `§1240` `ORT-009` asgarisinin altına inmeyi yasaklıyor |

---

## 9. Sözleşme uyumu

| Sözleşme | Etki |
|---|---|
| `YÖN-002` | GPUI kaynağı yerel `../gpui` exact çalışma ağacıdır; Bölüm II'nin git bağımlılığı alınmaz (§5). |
| `YÖN-005` | F5 kanıtı yalnız kayıtlı `UyumBulgusu` ve gerçek koşum çıktısıyla kapanır; görsel taslak tek başına kanıt değildir. |
| `YÖN-006 §3` | Bilgi mimarisi değişmez; tezgâh aile sayfasında kalır (§1.3). |
| `YÖN-006 §3.4` | Görsel dil `ORT-004` + `ORT-017`'den çözülür; kip ölçülen alandan seçilir (§4). |
| `YÖN-006 §4` | Küresel eksenler galeri çubuğunda; tezgâh ikinci otorite kurmaz. |
| `YÖN-006.ACC-005` | Desteklenmeyen capability görünür ve dürüst — pasif + gerekçe kuralının kaynağı. |
| `YÖN-006.ACC-008` | Tezgâh/bölüm başlığı hazır dize değil `YerelleştirmeAnahtarı` → ORT-021 `İletiİsteği` sonucudur (§2.2). |
| `YÖN-006.ACC-006` | Galeri kanonik render'ı taklit eden ikinci görsel uygulama kurmaz (F2). |
| `BİL-010 §16.2` | Gösterge sonucu **yalnız** `durum_göstergesi_durumu()` ile gözlemlenir. |
| `BİL-010 §16.2.4` | Yerel sahte baloncuk ve sessiz `Yok` fallback'i yok. |
| `BİL-010 §16.2.5` | Glif rol/ad/tabindex üretmez; ikinci canlı bölge yok. |
| `ORT-002` | Harf dönüşümü, locale/betik doğruluğu ve `YerelleştirmeAnahtarı` sahipliği ikinci yerel metin modeli kurulmadan tüketilir (F3). |
| `ORT-004.ACC-001` | Ham renk **ve** dağınık ölçü sabiti yok → `TezgahGörünümProfili` (§3.3). |
| `ORT-004.ACC-011` | Sağlanmayan yetenek için sahte destek ilan edilmez. |
| `ORT-017` | **Hedef:** tezgâh metrikleri `BileşenGörünümTanımı` + `anatomi_kaydet` yoluyla kayıtlı profilde tek sahipli olur. **Bugün kapılı:** opaklık kademesinin kanonik `Clone`/ödünç-okuma imza çelişkisi açık; ayrıca exact tipler ve `GörünümKayıtDefteri` uygulayıcısı fiziksel değil (§3.3.1–3). |
| `ORT-003` | Köşe yarıçapı `KutuŞekliTercihi`nden çözülür; exact `KutuŞekliGeometrisi` fiziksel olana kadar yeni yüz kapılıdır (§3.4). |
| `ORT-006` | Bütün `?` yardım yüzeyleri tek pencere-kapsamlı `Araçİpucu` konağından açılır; exact fiziksel yüzey yokken yerel popup kurulmaz. |
| `ORT-008` | `s8` biçim/giriş basamağı ve `BiçimYetenekleri` kayıtlı çözümden gelir; ikinci sayısal/biçim modeli kurulmaz (F3). |
| `ORT-016` | Yeni `warning` ve `info-circle` varlıkları kayıtlı simge kimliği/varlık anahtarı yolundan çözülür; gömülü ad-hoc SVG tüketimi açılmaz (F0a). |
| `ORT-019` | Gözlem paneli tanı zarfı üretmez; gerekçe ham enum adıyla gösterilmez. Exact kayıt yolu `TanıDeğeri::Kod(TanımKimliği)` + mühürlü fabrika/private sicildir (F2.2). |
| `ORT-021` | Başlık ve yardım metni ham `SharedString` değil, `YerelleştirmeAnahtarı`ndan güncel katalog/yerel snapshot'ıyla çözülmüş iletidir. |
| `ORT-023` | Yeni `Sergi` rotası bu atomda açılmaz; galeri snapshot/niyet akışı genişletilmez (§1.3). |

---

## 10. Açık kalan kararlar

1. Önizleme kabuğu iç boşluğu: Bölüm I §7.2 (`8/5/6/6`) mi, Yerleşim Raporu
   (`5/5`) mi? *(önerim: Bölüm I)*
2. `screenshots/s23.png` yeniden alınmalı — bugün `genel.png` ile bayt düzeyinde
   aynı ve `.png` uzantısına rağmen JPEG. İki görüntü de doğru formatta olmalı.
3. `iki_kolon_eşiği` değeri ve `%200` metin ölçeğinde nasıl kayacağı — F0'da
   profile yazılacak sayı.
*(Kapanan açık noktalar: `Role` varyantları mevcut · rota gömülü olarak bağlandı ·
balon simülasyonu iptal edildi · **F2 sınırı geliştirici gözlem paneli olarak bağlandı** ·
**iki bayat rapor yeniden üretildi** · **`github.md` exact GPUI/Zed commit'lerine
bağlandı**.)*

---

## 11. Sıra ve tahmin

| Faz | İçerik | Büyüklük |
|---|---|---|
| F0a | İskelet, dört kip/tokenlar, simgeler | küçük |
| F0b | Kanonik görünüm profili ve dokuz yüz (ORT-017 opaklık imza düzeltmesi + ORT-003/004/017 fiziksel göçü kapılı) | orta |
| F1 | Yeni düzen, mevcut 49 eksen, uyarlanabilir kolon; yapısal alt iş başlatılabilir, tam kabul F0b + ORT-006/021 kapılı | **büyük** |
| F2 | §16.2 geliştirici gözlem paneli (canlı çizim kanonik atom borcu) | küçük |
| F3 | Kalan A eksenleri | büyük |
| F4 | B/C/D bölümleri | orta |
| F5 | §29 tablosu + kabul turu | küçük |

F2 iki kez küçüldü: önce ikinci çözücü kalktı, sonra canlı çizim kanonik atom
borcuna ayrıldı. Geriye opak sonucu okuyan, ORT-019 tanısı üretmeyen bir
geliştirici gözlem kartı kaldı.

---

## 12. Koşum kaydı

**I. tur**

- Kanonik yapısal denetim: 68 belge, 2247 tür, 1648 ölçüt; **1106 ölçüt kanıtsız**.
- `cargo test -p gpui-bilesenleri-galeri`: **93 test yeşil** — F0/F1 taban çizgisi.
- Faz-3 fiziksel API kapısı: **kırmızı, 146 derleme hatası**; `GirişYüzeyBağı` dâhil.

**II. tur**

- `Tezgah.dc.html`, Yerleşim Raporu ve Uyum Listesi tarayıcıda açıldı: ana yüzeyde
  254 kontrol, konsol hatası yok. Bu yalnız paketleme/render açılışını doğrular,
  **sözleşme uyumunu doğrulamaz**.
- Çalışma ağacı **temiz değil**: `?? Tezgah_yeni_tasarimi/` ve
  `?? raporlar/BILESEN_TEZGAHI_YENI_TASARIM_GOC_PLANI.md` izlenmiyor. (1. sürümdeki
  "çalışma ağacı temiz" ifadesi yanlıştı; izlenen dosyalar değişmemişti ama
  izlenmeyen yollar var.)
- Fiziksel doğrulamalar: `bileşen.rs` render'ında gösterge yok · `api.rs`'de
  `GirişYüzeyBağıEksik` yok · `görünüm.rs`'de ORT-017 kayıt yüzeyi var ·
  `genel.png` ≡ `s23.png` (md5 `f16d64d2…`) · `Tezgah.dc.html` künyesinde hem
  `13.0.0` hem `4.0.0` *(VII. turda kaynak yorumundaki eski sürüm düzeltildi)*.

**III. tur**

- Adlandırma göçü doğrulandı: `Tezgah_yeni_tasarimi/` ve
  `raporlar/BILESEN_TEZGAHI_YENI_TASARIM_GOC_PLANI.md` yerinde; eski ada etkin
  atıf kalmadı.
- Fiziksel doğrulamalar: `§16.2.5` tanı zarfı sorun kimliğini ve değer sürümünü
  yasaklıyor · kanonik `ORT-017 4.3.0` trait'i `tipografi_parçaları` +
  `tipografi_uygulama_grupları` istiyor, fiziksel `görünüm.rs` taşımıyor ·
  `GörünümKayıtDefteri`'nin somut uygulayıcısı yok · `TextStyle` `color` +
  `font_family` + `font_size` taşıyor · `cx.observe` mevcut ~~ve galeri deseni
  `lib.rs:1515`'te kullanımda~~ *(IV. turda geri alındı: orası `subscribe`)* ·
  Uyum Listesi'nin 313–318. satırları bayattı.
- Çalışma ağacı: `?? Tezgah_yeni_tasarimi/` ve `?? raporlar/BILESEN_TEZGAHI_…md`
  hâlâ izlenmeyen iki yol. Rust kaynakları değiştirilmedi.

**IV. tur**

- Fiziksel doğrulamalar: `gösterge_girdisi_sürümü` private, getter yok
  (`api.rs:2225`, kamusal getterlar `api.rs:2324` civarı) · `ORT-019` kayıtlı
  kod kümesi dışındaki girdiyi `DeğerPolitikasıUyuşmuyor` sayıyor ·
  `§16.2.4` `AçıklamaTercihi::Yok`u yüzey bağından bağımsız kılıyor ·
  kanonik tip `TipografiRolü`, metrik rolleri hiçbir yerde yok ·
  `lib.rs:1515` `subscribe` (olay), `observe` (bildirim) değil.
- Uyum Listesi'nin "3 → 0" bölümü doğrulandı; adlandırma göçleri yerinde;
  ~~HTML etiket dengesi korunuyor~~ *(VI. turda geri alındı: tarayıcının örtük
  onardığı `span > div` iç içeliği bulundu ve kaynakta düzeltildi)*.
- Rust kaynaklarına dokunulmadı; çalışma ağacında iki izlenmeyen yol duruyor.

**V. tur**

- Fiziksel doğrulamalar: `ORT-017` profil yasak listesi `Pixels` içermiyor,
  yalnız ham renk ve fiziksel font · `TipografiRolü` `ORT-004`'te tanımlı, beş
  varyant · `§16.2.1` kayıtlı `Gösterge` anatomi parçası, `§16.2.2` iki
  bağlayıcı alt sınır istiyor · ~~`AnatomiParçasıSınıfı` ve `AnatomiUyumsuz`
  fiziksel~~ *(VI. turda düzeltildi: yalnız eski `ParçaSınıfı` ve
  `GörünümHatası::AnatomiUyumsuz` karşılıkları fiziksel; exact kanonik tipler
  ve iki alt sınır metriği değil)*.
- Rust kaynaklarına dokunulmadı; çalışma ağacında iki izlenmeyen yol duruyor.

**VI. tur**

- Kanonik denetim yeniden çalıştı: **68 belge, 2247 genel tür, 517 kenar,
  1648 ölçüt; 542 kanıtlı, 1106 kanıtsız**. Sonuç yapısal başarılıdır;
  kanıtsız ölçütler uyum kanıtı değildir.
- Fiziksel API denetimi kırmızı: **1701** public öğe; faz-1 ORT eksiği **1184**,
  faz-2 ORT eksiği **1409**, BİL faz-3 eksiği **146**. Bu plandaki
  `KutuŞekliGeometrisi`/`TipografiRolü`/`ÇözülmüşKullanıcıİletisi` faz-1;
  `AnatomiParçasıSınıfı`/`GörselOpaklıkKademesi`/`Araçİpucuİsteği`/
  `YüzeyKonağı` faz-2; `GirişYüzeyBağı` faz-3 kapısındadır.
- `cargo test -p gpui-bilesenleri-galeri`: **93/93 geçti**. Dört mevcut uyarı
  (`TemaKipi` importu, crate-level olmayan `allow`, iki ölü sabit) ve
  `block 0.1.6` gelecek-uyumsuzluk uyarısı test sonucundan ayrı kaydedildi.
- Tarayıcı açılışı: üç HTML `readyState=complete`, konsol hatası/uyarısı yok.
  Ana tezgâhta **254 form kontrolü + 15 summary = 269 etkileşimli öğe** ve
  hedef görsel olarak altı durum göstergesi var. Yerleşim Raporu fiziksel
  render sınırını/simülasyon yasağını; Uyum Listesi 3. sürüm gözlem künyesini
  ve “3 → 0” başlığını görünür taşıyor.
- Kaynak HTML taramasında tarayıcının örttüğü bir gerçek yapı hatası bulundu:
  `gosterge-serit` ile altı `durum-gosterge` kabuğundaki `span > div` iç
  içeliği `div` kabuklarına çevrildi. Son `tidy` filtresinde ilgili etiket
  kapatma/atma uyarısı yok; önbelleksiz tarayıcı koşumunda doğrudan
  `span > div` sayısı **0**, 1 şerit + 6 göstergenin tamamı `DIV` ve görünüm
  bozulmamış durumda. `x-dc`, `helmet` ve şablon nitelikleri gibi tasarım
  bileşeni biçimine özgü `tidy` uyarıları W3C-HTML uygunluk iddiası sayılmaz.
- Girdi envanteri yeniden sayıldı; iki ekran görüntüsü hâlâ JPEG 924×540 ve
  aynı md5 (`f16d64d214db4e1b626ded18740ca5fb`). Eski klasör/plan adına etkin
  yol atfı yok; yalnız §0.3'te tarihsel eski klasör adı açıklanıyor.
- Rust kaynakları değiştirilmedi. Çalışma ağacı hâlâ iki izlenmeyen kök gösterir:
  `?? Tezgah_yeni_tasarimi/` ve
  `?? raporlar/BILESEN_TEZGAHI_YENI_TASARIM_GOC_PLANI.md`.

**VII. tur**

- Kanonik API imzaları satır satır yeniden karşılaştırıldı.
  `GörselOpaklıkKademesi`nin `Clone`/`Copy` taşımadığı ve
  `düşük_taban(self)` getter'ının `Profil: Clone` + `çöz(&Profil)` ile birlikte
  uygulanamadığı doğrulandı. Bu fiziksel göçten önce kapanması gereken exact
  ORT-017 sözleşme blokeri olarak ayrıldı; plan ikinci opaklık modeli önermiyor.
- ORT-019 yüzeyi düzeltildi: kanonik yol hayalî `TanıKodu` değil,
  `TanıDeğeri::Kod(TanımKimliği)` + mühürlü `TanıZarfıFabrikası` + private
  sicildir. Fiziksel eski `GüvenliKod` modeli bu kapının yerine geçmez.
- ORT-021 eski/exact ayrımı yeniden doğrulandı: fiziksel `YerelleştirmeAnahtarı`,
  `İletiİsteği`, `İletiÇözümleyicisi` ve `Çözülmüşİleti` vardır; exact
  `ÇözülmüşKullanıcıİletisi` ve çözüm imzası yoktur. Bölüm başlıkları da hazır
  `SharedString` yerine anahtar taşır.
- F1'in yapısal işi F0a üzerinde başlatılabilir; tam görsel kabulü F0b'nin
  ORT-003/004/017 kapılarını ve yardım yüzeyi için ORT-006/021 kapısını miras
  alır. Crate-tabanlı yol künyesi ve `gpui.rs:93` atfı düzeltildi.
- `github.md`, temiz `crates/gpui/` kapsamı için yerel
  `1995423061bfe65b27266a80d9d4200e457a29e1` ve kaynak Zed
  `cef06d351bec10d0fb6176018ce8624e97baeb40` commit'lerine bağlandı; yerel
  deponun kapsam-dışı kirli yolları saklanmadı.
- `Tezgah.dc.html` kaynak yorumundaki eski `BİL-010 4.0.0` künyesi yaşayan
  `13.0.0` ile değiştirildi; dosyada çelişkili sürüm kalmadı.
- Son mekanik koşum VII değerlerini korudu: kanonik yapısal denetim başarılı
  fakat 1106 ölçüt kanıtsız; fiziksel API kapısı kırmızı; galeri testleri
  **93/93 yeşil**. Markdown'da 14 kod çiti dengeli ve tablolar sütun-tutarlı;
  kritik HTML kapatma/atma filtresi boş. Rust kaynaklarına dokunulmadı.
- Önbelleksiz son tarayıcı koşumunda üç belge de `readyState=complete`:
  tezgâh 254 form kontrolü + 15 `summary`, 1 `DIV` gösterge şeridi, 6 `DIV`
  gösterge ve **0** doğrudan `span > div` taşıyor; Yerleşim Raporu fiziksel
  sınır/simülasyon reddini, Uyum Listesi gözlem künyesini ve
  “Temsil boşlukları · kapandı (3 → 0)” kaynak başlığını görünür taşıyor.

# BİL-010 tezgâhı · devir notu

| Alan | Değer |
|---|---|
| Dal | `tezgah-yeni-tasarim` |
| Son commit | `055ef53` |
| `BİL-010` sürümü | `17.0.0` |
| `BİL-120` (form) sürümü | `6.0.13`, zarf `BİL-010 ^17.0` |
| Denetim | `python3 tools/sozlesme_denetimi.py` → **YAPISAL BAŞARILI** (1106 ölçüt kanıtsız, taban durum) |
| Testler | BİL-010 tarafı 268 geçer + 8 atlanır (`bil010*`), galeri 202, uyum 260 — hepsi geçiyor |

Bu belge sohbet devri içindir: neyin bittiğini, neyin açık kaldığını ve
sıradaki işin nereden başlayacağını tek yerde tutar.

---

## 1. Kapanan bulgular

Tezgâh gezintisinde bulunup düzeltilenler (commit sırasına göre):

| # | Bulgu | Kapanış |
|---|---|---|
| 1 | Tercih kutuları tema değişiminde tazelenmiyordu (koyu kipte ön ek/son ek açık kalıyordu) | `91f2709`, `f7f13f7` |
| 2 | Yoğunluk ve hareket düğmeleri hiçbir şey yapmıyordu — tema kurulumuna hiç girmiyorlardı | `b71e45f` |
| 3 | Seçim yapılınca açılır liste kapanmıyordu | `b71e45f` |
| 4 | Pasif önem göstergesinin etiketi eksen adını tekrarlıyordu ("Önem düzeyi" alt alta iki kez) | `96cbd7b` |
| 5 | **Sağ kolonun on bölümü hiç çizilmiyordu** — `overflow_y_scroll` kabında ikinci ve üçüncü akış sıfır yüksekliğe iniyordu | `0ffa894` |
| 6 | Beş açılır seçici aynı `"imleç"` kimliğini paylaşıyordu; biri açılınca beşi birden açılıyordu | `7a72d0d` |
| 7 | `§18` ile `ACC-048` çelişiyordu: `DeğeriİşleVeKalTümünüSeç` tercih kapalıyken `DeğeriİşleVeKal` ile aynı davranıyordu | `c28948a`, `f50a83c` |
| 8 | **Durum göstergesi (`§16.2`) hiç çizilmiyordu** — yerleşim çözülüyor, parça satıra eklenmiyordu | `155f92d` |
| 9 | Enter/odak eksenleri tek alana bağlıydı; `YeniSatır` çalışma zamanı hatası üretiyordu | `3583793` |
| 10 | `§12`'deki "bastırabilir" bağlayıcı değildi | `2c4fff7` |
| 11 | Tezgâhtaki gösterge notu yanlış yol tarif ediyordu | `2c4fff7` |
| 12 | `özel_durum` ve `önem` dışarıdan yazılabiliyordu; yazılan değer ilk tuş vuruşunda siliniyordu | `fa42d24` |
| 13 | Sorun kalkınca `önem` bayat kalıyordu; kurucu varsayılanı `Hata`ydı | `8552d97` |
| 14 | `erişim` ve `üzerinde` dışarıdan yazılabiliyordu | `597c844` |
| 15 | **Devre dışı alan tıklamayla odak halkası alıyordu** — GPUI izlenen tutamacı kendiliğinden odaklıyor, Tab durağında da kalıyordu (`ORT-005 §2` ihlali) | `d27fdd7` |
| 16 | **Web'de yapıştırma `§10` politikalarını atlıyordu** — DOM paste olayı `EntityInputHandler::paste` varsayılanına düşüyor, ham metin yazma yoluna gidiyordu | `a070e98` |
| 17 | **`§22` geçici gösterim politikası fiziksel değildi** (borç 18'in yarısı) — üç geri dönüş kipi de artık çalışıyor, `GeçiciGöster` politikasız kurulamıyor, tezgâh "Geri dönüş politikası" satırını çiziyor | `112e976` |
| 18 | **`§16` dış sorun kanalı fiziksel değildi** (borç 18'in kalan kanonik yarısı) — `DışHata` hiç üretilmiyordu, her tuş vuruşu dış sonuçları eziyordu; sorun deposu kaynakla bölümlendi, `dış_hata_temizleme` politikası yerel düzenlemeye bağlandı | `c735eb5` |
| 19 | **Dış doğrulama kartı + gösterim beslemesi** — kullanıcı kararıyla "sahte sunucu taklit edilmez" duruşu bu eksen için esnetildi; borç 18 tümüyle kapandı | `5e4206c` |
| 20 | **Köşe dili tek kademe** (kullanıcı kararı) — hap (999px) düğme yüzleri ve kesikli türetilmiş rozetler kart kademesine çekildi; `hap*` adları tarihsel, dolgular ve profil değişmedi | `30512de`, `6919674` |
| 21 | **Borç 16 kapandı** — `giriş_türü` kanonik aileye göçtü, para/yüzde `Ondalık` + biçim profiline indi (kullanıcı kararı); iki borç bekçisi testi tetiklenip kanonik yönde yeniden yazıldı | `287bc7f` |
| 22 | **`ORT-008 2.2.0` — `İşaretKonumu`** (kullanıcı yetkili revizyon): ₺ ve % işareti biçimde önde ya da sonda; motor, tezgâh seçicisi ve uyum testi | `38b91d1`, `71a1b2e`, `392f9e1` |
| 23 | **Telefon deseninde baştaki `0` sabit** (kullanıcı kararı) — kaçışsız `0` zorunlu rakam yuvasıydı; `\0(000) 000 00 00` tek `Telefon` satırında birleşti (`HAZIR_DESENLER`, biçim listesi, sergi). WASM'de görsel + davranış doğrulandı | `7bd2251`, `98b2ec8` |
| 24 | **Kabul reddi yüzeyi `§28` türetimini eziyordu** (kullanıcı bulgusu: eksik maskede kenarlık kırmızıydı, Uyarı sarı olmalı) — `değeri_işle`/`EskiDeğereDön` içindeki ikinci durum yazarları kaldırıldı; `Hatalı` ayrıştırma artık kümeye yerel `Hata` sorunu olarak girer, eksik maske kendi `Uyarı` sorunuyla türer. Enter, Tab ve dış tıklama üç yolda da WASM'de doğrulandı | `752d5a3`, `297562e` |
| 25 | `§18` Enter/Tab asimetrisi bekçilendi: `DeğeriİşleVeSonrakineGeç` yalnız başarılı kabulde ilerler, Tab başarısızlıkta `§17` politikasına düşer — sözleşme gereği, kusur değil | `7bd2251` |
| 26 | `ort006` odak köprüsü testi ORT-001 kimlik göçünden beri kırmızıydı (her `bileşen(...)` çağrısı yeni örnek kimliği üretir); kimlik bir kez bağlanarak düzeltildi — tezgâh ekseninden bağımsız | `752d5a3` |
| 27 | Masaüstü galeriye Linux pencere arka uçları (`wayland`, `x11`) eklendi — onlarsız Linux'ta açılmıyor; macOS/Windows'ta hedef kapılı, etkisiz. Linux'ta eşitlenmiş `../gpui` ile doğrulandı (taffy düşürmesi eski gpui klonundandı, depo işi değil) | `055ef53` |

## 2. Kullanıcının koyduğu ilke — `§29.0` eksen ayrıklığı

> Hiçbir yapılandırma alanı bir başkasının değerini ezmez, gizlemez veya
> ondan türetilmez. Çakışma **tiple** engellenir, çalışma zamanı hatasıyla
> değil. Kural bileşenin bütün tanımları için geçerlidir.

Sözleşmeye `§29.0` olarak yazıldı. Bu ilke uyarınca kapatılan kanallar:

| Alan | Eski durum | Yeni durum |
|---|---|---|
| `özel_durum` | `pub`, iki otorite | `pub(crate)`, `görsel_durum()` ile okunur |
| `önem` | `pub`, iki otorite | `pub(crate)`, `önem()` ile okunur; ürün onu `GeçerlilikKuralı::önem`de bildirir |
| `erişim` | `pub` alan + `erişimi_değiştir` | `erişim()` — `etkin`/`salt_okunur`tan türer |
| `üzerinde` | `pub` | `pub(crate)`, `üzerinde()`; `§23` çözümü onu parametre alır |

`§29.0` kapsamında **açık kanal kalmadı**.

## 3. Açık işler

### 3.1 Denenmemiş iki eksen — KAPANDI

- **Yapıştırma politikaları** WASM'da gerçek DOM `paste` olayıyla elle
  denendi (sentetik `ClipboardEvent` + `DataTransfer`, gizli `input`
  öğesine): Katı geçerliyi alır, karışığı kırmızı kenarlıkla reddeder;
  süzme `₺1.234,56 TL → 1.234,56`; ayıklama `₺1.234,56 → 1234,56`;
  yerel denemesi `1.234`ü belirsiz diye reddeder, `12,5`i alır. Deneme
  bulgu 16'yı çıkardı ve kapattı.
- **`Devre dışı` görsel sunumu** gözle doğrulandı: zemin `pencere`
  rengine düşer, kenarlık gri kalır, tıklama odak halkası üretmez
  (bulgu 15 bu denemeden çıktı ve kapatıldı).

### 3.2 `§8` borç tablosundaki açık kayıtlar

`raporlar/BILESEN_TEZGAHI_YENI_TASARIM_GOC_PLANI.md` §8:

- **16** — **tümüyle kapandı** (`287bc7f`): alan kanonik `GirişTürü`,
  para/yüzde biçime indi, `para_birimi` emekli. Beraberinde `ORT-008
  2.2.0` (`38b91d1`): `İşaretKonumu` ekseni — ₺/% önde ya da sonda,
  tezgâh seçicisi canlı (`392f9e1`)
- **17** — **tezgâh borcu değil (kullanıcı kapsam kararı, 24 Ağu):**
  açılır listenin dış tıklamayla kapanışı açılır liste / yüzer yüzey
  sözleşmesinin (`BİL-020`/`ORT-006`) işidir ve o turda halledilir.
  Tezgâhın işi yalnız BİL-010'un kendi sözleşmesini doğru sağlaması ve
  sözleşmelerin doğru aktarılmış olmasıdır; ödünç alınan diğer
  bileşenlerin doğru çalışması ileriki konulardır
- **18** — **tümüyle kapandı** (`112e976`, `c735eb5`, `5e4206c`): iki
  kanonik yarı fiziksel, tezgâh sunumu kullanıcı kararıyla gösterim
  beslemesine bağlandı
- **25** — `§22.1` programatik köken kapısı fiziksel değil (`DeğerKökeni`
  tüketilmiyor); geçici gösterim açılırken bilinçli kapsam dışı kaldı
- **19** — `§40` yerel geçmiş ve `§41` işaretçi imleci fiziksel değil
- **20** — pasif denetimlerin gerekçesi yalnız `aria_label`'da
- **22** — galeri kataloğu ulaşılamaz (`tezgah_ekranı` hep `true`)

### 3.3 Kullanıcı kararı bekleyen

Yok. Son iki karar verildi ve uygulandı (`5e4206c`, `30512de`):

- **Dış hata ekseninin tezgâh sunumu**: gösterim beslemesi seçildi —
  "galeri sahte sunucu taklit etmez" duruşu yalnız bu eksen için
  esnetildi, kart gerçek sunucu olmadığını açıkça yazar.
- **Köşe dili**: tezgâh düğmelerinde hap yüzü bırakıldı, tek köşeli
  (kart kademesi) dil; `hap*` yüz adları tarihsel.

## 4. Çalışma notları

- **WASM doğrulaması:** `python3 tools/wasm_galeri_hazirla.py`, sonra
  `python3 tools/wasm_galeri_sunucu.py --port 8000`. Sunucu takılırsa
  `lsof -nP -iTCP:8000 -sTCP:LISTEN` ile PID bulup öldür.
- **Tarayıcı:** son oturumda pane sık takıldı (30 s timeout). Pencere
  yeniden boyutlandırma WASM çizim alanını bozuyor; sekmeyi kapatıp
  yeniden açmak düzeltiyor. `800×900` viewport 1:1 ekran görüntüsü verir,
  tıklama hedeflemesi orada güvenilir.
- **Klavye:** `computer type` (çoklu karakter) WASM'a **geçmiyor** —
  `gpui_web` `input` olayını dinlemiyor. `computer key` (tek tuş) geçiyor.
  Gerçek kullanıcı yolu (keydown, paste, composition) etkilenmiyor.
- **Yapıştırma denemesi panosuz yapılabiliyor:** `gpui_web` gizli `input`
  öğesinde DOM `paste` dinliyor; tarayıcı konsolunda `new ClipboardEvent`
  + `DataTransfer` kurup `input.dispatchEvent(ev)` demek gerçek dinleyici
  yolundan geçiyor (`isTrusted` denetlenmiyor).
- **Ekran görüntüsü bir kare geride:** etkileşimden sonra ilk görüntü eski
  durumu gösterebilir; ikinci kez al.
- **`sozlesme_api_faz*` test hedefleri `HEAD`'de derlenmiyor** — kod göçü
  borcu, bu çalışmanın kapsamı dışında. `cargo test --workspace` bu yüzden
  kırmızı; hedefli crate testleri kullanılmalı.
- Sözleşme sürümü değişince sıra: kanonik belge → tüketici zarfı →
  `--manifest-yaz` → `--tamamlanmis-yuzey-yaz <KİMLİK>` → denetim.

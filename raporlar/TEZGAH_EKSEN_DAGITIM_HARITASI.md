# Tezgâh Eksen Dağıtım Haritası

> Nitelik: Normatif olmayan çalışma haritası
> Tarih: 20 Ağustos 2026
> Üst plan: `raporlar/BILESEN_TEZGAHI_YENI_TASARIM_GOC_PLANI.md` (F1 · yapısal akış)
> Kaynak: `TezgahTercihleri` **49 alan** · `sergiler.rs` **24 tezgâh çizim fonksiyonu**

Bu harita, mevcut şerit tabanlı tezgâhtaki her ekseni yeni düzenin hangi
bölümüne taşıyacağımızı ve hangi çizim fonksiyonunun akıbetini gösterir.
Uygulamadan önce onaylanır; onaysız taşıma yapılmaz.

---

## 1. Sol kolon — kabuk denetimleri, önizleme ve kod

Yeni düzende sol kolon üç bloktan oluşur (`Tezgahİçeriği.önizleme` +
`sol_ek` + `kod`).

| Blok | Eksenler | Mevcut çizim | Akıbet |
|---|---|---|---|
| **Kabuk şekli** | `şekil`, `köşe_pikseli` | `köşe_şeridi`, `köşe_kaydırma_çubuğu` | Taşınır. Yüzer kutu tetikleyicisi kalkar, yarıçap `details` yerine kart içi satır olur. **Yarıçap değeri ORT-003 kapısına bağlıdır** (F0b). |
| **Yardımcı yuvalar** | `temizle`, `arama`, `parola_düğmesi`, `seçici` | `yardımcı_eylem_şeridi` | Taşınır. `§23`ün üç yuva sınırı ve "bulunmayan yuva yer kaplamaz" kuralı korunur. |
| **Hizalama** | `hizalama`, `dikey` | `yatay_hizalama_şeridi`, `dikey_hizalama_şeridi` | Taşınır (segment kuşağı yüzü F0b'de). |
| **Parça tipografisi (D)** | `tema` | `yazı_şeridi`, `yazı_biçimi_şeridi`, `punto_listesi`, `aile_listesi` | Taşınır. Ayrı tipografi **bölümü açılmaz** (`AÇK-009`); bunlar önizleme bağlamıdır, `GirişYapılandırması`na yazılmaz. |
| **İmleç (ORT-004)** | `tema.imleç_hızı` | `imleç_satırı` | Taşınır; D bölümünün parçası. |
| **Yaşayan önizleme** | — | `tezgah_sergisi` içindeki `.child(alan)` | Aynen kalır: gerçek `GirişKutusu` çizilir, taklit kurulmaz. |
| **Kod paneli** | türetilmiş | `kod_paneli` | Sol kolonun **en altına** iner, "yalnız A bölümü" notuyla. |

---

## 2. Sağ kolon — bölüm eşlemesi

`AK` = akış. Tam genişlik bölümleri akış bölünmesine girmez.

| Bölüm | Başlık | AK | Tür kapsamı | Eksenler | Mevcut çizim | Faz |
|---|---|---|---|---|---|---|
| `s7` | §7 Değer türü | Tam | hepsi | `değer_türü` | `tür_satırı` | **F1** |
| `s9` | §7 Tür tanımı · §9 Giriş maskesi | Tam | hepsi | `maske`, `desen`, `bölüm_gezinimi`, `bölüm_atla`, `bölüm_dolunca_ilerle`, `bölüm_artır`, `bölüm_taşar`, `bölüm_ayraç` | `maske_tanımı`, `bölüm_satırı` | **F1** |
| `s8` | §8 Biçim profili · ORT-008 | A | tamsayı·ondalık·tarih | `biçim_seçeneği`, `ondalık_basamak`, `binler_ayracı` | `biçim_satırı`, `biçim_listesi`, `sayı_biçimi_şeridi` | **F1** |
| `s6ek` | §6 Ön ek ve son ek · Sabitİçerik | A | hepsi | `ön_ek`, `ön_ek_metni`, `son_ek`, `son_ek_metni`, `ek_sunum_rolü` | `içerik_satırı` (bölünür) | **F1** |
| `s97` | §9.7–9.8 Hacim ve sayaç | B | hepsi (sayısalda **pasif**) | `uzunluk_sınırı`, `uzunluk_davranışı`, `sayaç`, `sayaç_birimi`, `sayaç_sınırı_göster` | `içerik_satırı` (bölünür) | **F1** |
| `s96` | §9.6 Sayısal adım | C | tamsayı·ondalık | `sayısal_adım`, `adım_ölçeği`, `adım_hizala`, `adım_sınırı`, `adım_sarma` | `adım_satırı` | **F1** |
| `s22` | §22 İçerik görünürlüğü | B | hepsi (sayısalda **pasif**) | `gizli_içerik` | `içerik_satırı` (bölünür) | **F1** |
| `s17` | §17–20 Odak, kabul ve erişim | C | hepsi | `enter`, `odaklanınca_tümünü_seç`, `dış_tıklamada_odağı_bırak`, `üzerine_yazma`, `salt_okunur`, `etkin`, `varsayılan_değer`, `sıfırlama` | `odak_satırı`, `varsayılan_satırı` | **F1** |
| `s24` | §24 Seçici · §20.1 Erişilebilirlik | C | hepsi | `yer_tutucu` | `içerik_satırı` (bölünür) | **F1** kısmi |
| `s61` | §6.1 Otomatik doldurma (**B bölümü**) | B | metin | `otomatik_doldurma`, `doldurma_amacı` | `doldurma_satırı` | **F1** |
| `sz` | ORT-002 saat dilimi | C | tarih | `saat_dilimi_tercihi` | `saat_dilimi_satırı` | **F1** |
| `s6` | §6 Metin işleme | A | metin | *(yeni)* harf dönüşümü, kırpma, boş giriş | — | F3 |
| `s10` | §10 Yapıştırma | B | hepsi | *(yeni)* politika, dil etiketleri, seçimsiz kopyala | — | F3 |
| `s15` | §15–16 Doğrulama | C | hepsi | *(yeni)* kural türü, tetikleyici, önem, uzak zorunluluk | — | F3 |
| `s23` | §23 Eylem ve bölüt | C | hepsi | *(yeni)* bitişik bölüt, yuva kademesi, arama gönderimi | — | F3 |
| `s29` | §29 Yapılandırma doğrulaması | Tam | hepsi | *(türetilmiş tablo)* | — | F5 |
| `port` | Port kapıları | C | hepsi | *(B bölümü kapanış kartı)* | — | F4 |

**F1 kapsamı: 11 bölüm, 49 eksenin tamamı.** F3/F4/F5'e kalan altı bölüm bugün
modelde karşılığı olmayan eksenlerdir; F1 mevcut olanı taşır, yeni eksen açmaz.

---

## 3. `içerik_satırı` dörde bölünür

Tek fonksiyon bugün dört ayrı sözleşme bölümünün eksenlerini taşıyor. Yeni
düzende her biri kendi kartına gider:

```
içerik_satırı
├── ön_ek, ön_ek_metni, son_ek, son_ek_metni, ek_sunum_rolü  → s6ek
├── gizli_içerik                                             → s22
├── uzunluk_sınırı, uzunluk_davranışı, sayaç,
│   sayaç_birimi, sayaç_sınırı_göster                         → s97
└── yer_tutucu                                               → s24
```

Bu bölünme yalnız düzen değil, sözleşme okunabilirliği meselesidir: `§6`, `§22`,
`§9.7` ve `§24` ayrı maddelerdir ve tek satırda toplanınca hangi eksenin hangi
maddeden geldiği görünmez.

---

## 4. Tür süzgeci — iki mekanizmanın ayrılması

Bugün beş eksen `.when(...)` ile **tümüyle gizleniyor**. `§9` bunu ikiye
ayırıyor; taşırken her biri doğru mekanizmaya bağlanır:

| Bugünkü koşul | Bölüm | Doğru mekanizma | Neden |
|---|---|---|---|
| `.when(sayısal, adım_satırı)` | `s96` | **Kapsam süzgeci** — hiç `child` üretme | Sayısal adım metin türünde *kurulamaz*; eksen o türde yoktur |
| `.when(tarih_türü_mü(), saat_dilimi_satırı)` | `sz` | **Kapsam süzgeci** | Saat dilimi yalnız tarih/saat biçimlerinde etkilidir |
| `.when(varsayılan_uygulanabilir_mi(), varsayılan_satırı)` | `s17` | **Kapsam süzgeci** | `§14` tarih/saat/süre türünde uygulanmaz |
| `.when(bölüm_gezinimi_anlamlı_mı(), bölüm_satırı)` | `s9` | **Pasif + gerekçeli** | Bölüm gezinimi *vardır*, maskesiz alanda **kapanır**; gizlemek "eksen yok" der |
| `.when(doldurma_var, doldurma_satırı)` | `s61` | **Pasif + gerekçeli** | Port yokluğu yeteneğin yokluğu değildir; `YÖN-006.ACC-005` desteklenmeyen capability'yi *görünür ve dürüst* ister |
| *(bugün yok)* `sayaç` sayısal türde | `s97` | **Pasif + gerekçeli** | `§9.8` sayısal türde sayaç uygulanmaz — ama eksen vardır |

Son iki satır bugünün davranışını **değiştirir**: gizlenen eksenler pasif ve
gerekçeli hâle gelir.

---

## 5. Kaldırılacak mekanizma — `TezgahKutusu`

Yüzer panel durum makinesi yeni düzende karşılıksızdır: kartlar normal belge
akışındadır, açılır panel yoktur.

| Öğe | Yer | Akıbet |
|---|---|---|
| `TezgahKutusu` enum + `tezgah_kutusunu_değiştir/kapat/açık_mı` | `lib.rs` ≈546–590 | Kaldırılır |
| `köşe_izi`, `köşe_sürükleniyor_mu`, `köşe_sürüklemesini_ayarla`, `köşe_yarıçapını_konumdan_ayarla` | `lib.rs` ≈591–625 | **Korunur** — düzeltme: bunlar yüzer panel değil, yarıçap kaydırıcısının sürükleme mekanizması. Kaydırıcı kart içine indi, izi hâlâ gerekli. |
| `yüzer_kutu`, `yüzer_seçim`, `yüzer_kutu_simgeli`, `kutu_tetikleyicisi`, `yüzer_gövde`, `kutu_başlığı` | `sergiler.rs` ≈1786–1965 | Kaldırılır |
| `köşe_kaydırma_çubuğu` | `sergiler.rs` ≈1965 | Kart içi satıra dönüşür |
| Kök `on_mouse_down` (dışa tıklayınca kapat) | `tezgah_sergisi` başı | Kaldırılır |

`?` yardım yüzeyi **yerine geçmez**: o yüzey `ORT-006` `Araçİpucu` konağı
fiziksel olana kadar çizilmez (`§3.4`). Yüzer kutuların taşıdığı içerik kart
içine iner, açılır panele değil.

---

## 6. Taşırken çıkan iki bulgu — **ikisi de kapandı**

**`üzerine_yazma` ekranda hiç yok.** Alan `TezgahTercihleri`nde var,
`tests/tezgah.rs` model düzeyinde sınıyor, ama `sergiler.rs` ve `lib.rs`
içinde tek bir çizim yok. `Sozlesme Uyum Listesi` §12/12.1 satırı "İki
anahtar" diyor — bu **yanlış**; bugün tek anahtar var. F1'de ikinci anahtar
`s17`ye eklenir ve uyum listesi düzeltilir. **Kapandı** (adım 3): anahtar
§17–20 bölümünde, uyum listesi düzeltildi.

**`doldurma_var` port kapısı ekseni gizliyor.** `YÖN-006.ACC-005` desteklenmeyen
capability'nin görünür ve dürüst olmasını ister; gizleme bunu karşılamaz.
**Kapandı** (adım 3): bölüm her zaman üretiliyor, port yokken içeriği pasif ve
gerekçeli çiziliyor. Bölüm gezinimi de aynı yolu izliyor.

---

## 6.1 Üçüncü bulgu — `GirişDeğerTürü` dokuz varyant taşıyor

Adım 1 uygulanırken çıktı. Kanonik tip bugün `Metin`, `Tamsayı`, `Ondalık`,
`ParaBirimi`, `Yüzde`, `Tarih`, `Saat`, `TarihSaat`, `Süre` — **dokuz**
varyant. Tasarımın `§8.1`'i ise "kamusal `GirişTürü` tam olarak bu dört
ailedir" diyor: `Metin`, `Tamsayı`, `Ondalık`, `TarihZaman`; para, yüzde,
bilimsel ve kesir beşinci tür değil `Ondalık` biçim profilidir.

Bu bir **kod göçü borcudur**, tezgâhın kapatabileceği bir açık değil: tür
ailesini daraltmak `BİL-010`'un kamusal yüzeyini değiştirir. Tezgâh bugün
dokuz varyantı da doğru süzer; `tests/tezgah_profil.rs` dokuzunu birden
tarar. Tür ailesi daraldığında profilin süzgeci sadeleşir, bölüm listesi
değişmez.

---

## 7. Uygulama sırası (F1 kalan iş)

| Adım | İçerik | Dokunulan |
|---|---|---|
| ~~**1**~~ | ✅ **TAMAM** — `BİL-010` profili: 7 bölüm + sol kolon blokları + köprü. `s6ek`/`s97`/`s22`/`s24` adım 2'yi bekliyor. | `bil010_profil.rs`, `lib.rs`, `tests/tezgah_profil.rs` |
| ~~**2**~~ | ✅ **TAMAM** — `içerik_satırı` dörde bölündü: `ön_ek_satırı`, `hacim_satırı`, `görünürlük_satırı`, `yer_tutucu_satırı`. On bir bölümün tamamı profilde. | `sergiler.rs`, `bil010_profil.rs`, `tests/tezgah_profil.rs` |
| ~~**3**~~ | ✅ **TAMAM** — kapanan eksenler artık pasif ve gerekçeli (`kapalı_eksen`); `devre_dışı_düğme` gerekçe almadan çağrılamıyor; `üzerine_yazma` anahtarı eklendi. | `sergiler.rs`, `metin_girisi_profili.rs`, `tests/tezgah_profil.rs` |
| ~~**4**~~ | ✅ **TAMAM** — yüzer panel mekanizması söküldü: enum, dört metot, altı yardımcı, üç kapatma çağrısı ve kök `on_mouse_down` kalktı. Kaydırıcı izi (`köşe_izi`) korundu: o panel değil, denetim mekanizması. | `lib.rs`, `sergiler.rs`, `metin_girisi_profili.rs` |
| ~~**5**~~ | ✅ **TAMAM** — sergi artık düzeni kendisi kurmuyor: profil `Tezgahİçeriği` üretiyor, kabuk iki kolonlu düzende çiziyor. Başlık anahtarları `tezgah_bölüm_adı` ile tek yerden çözülüyor. | `sergiler.rs`, `galeri.rs`, `tests/render_kosumu.rs` |

**Beş adım da tamamlandı.** Adım 4 en riskliydi — `lib.rs`teki durum makinesi
ve `sergiler.rs`in yüzer panel yardımcıları birlikte düştü — ve adım 5 zinciri
kapattı. F1'in **yapısal akışı** bitti: kart sırası, iki kaydıran kolon, akış
dağıtımı, tür süzgecinin iki mekanizması ve erişilebilir bölge/klavye turu
yerinde. F1'in **tamamlanmış görsel kabulü** hâlâ F0b'ye kapılıdır: köşe
yarıçapı `ORT-003`, tipografi ve opaklık `ORT-004/017` fiziksel göçünü bekler.

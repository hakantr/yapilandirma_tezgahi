# RA-AF K08/K09 galeri artçı kanıt raporu

> Tarih: 14 Eylül 2026
>
> Nitelik: Normatif olmayan kaynak, TestApp scene ve tüketici kanıtı
>
> Galeri kaynak/test revizyonu:
> `2dc1389a5ae98388d08e34ea861d989b8ecdfe9e`
>
> Galeri parent'ı: `4784c0d496e0dddecdafed3e8f9b0d955e37de30`
>
> Exact çekirdek revizyonu:
> `ac09785822284e361d76d869a853382c77f5fbc1`
>
> Exact çekirdek Git ağacı:
> `1ff5725db0cc011c4c5fa8f992a84e8963479d1a`
>
> Galeri kaynak Git ağacı:
> `6f8c7fafbd0808f451fb09e61d20e93a4c23ed3d`

Bu artçı teslim, ilk Paket D raporunda yalnız plan nesli ve sayaçla
gösterilmiş K08 sonucunu gerçek sunum içeriğiyle; eski çıplak K09 çözüm
sonucunu da çekirdeğin sahipli public tutamacıyla değiştirir. Galerinin
`../gpui_bilesenleri` bağı immutable bir Cargo SHA pini değildir. Bu nedenle
koşumda çözülen dizinin gerçek hedefi
`/Volumes/Taslaklar/tmp/gpui-k05-k06-20260910/repo`, commit'i ve Git ağaç
özeti yukarıda ayrı ayrı kaydedilmiştir. Çekirdek ve galeri ağaçları koşum
başında ve sonunda temizdir.

## 1. K08 gerçek yayın ve anlamlı sahne

Galeri artık aynı statik yayından farklı adlarla iki plan üretmez. Tek pencere
kökü aşağıdaki üretim zincirini her kullanıcı geçişinde tamamlar:

1. `DurumPlanPencereHizmeti::kanonik_yayını_tazele` current
   ORT-017 görünüm+bileşim çiftini aynı pencere için ilerletir.
2. `BileşimKökü::durum_plan_yayınını_hazırla` eski current yetkiyle yeni
   pair kaydını hazırlar; `KanonikDurumPlanDeposu::yaşayanı_ilerlet` yaşam
   yetkisini ve bileşim damgasını ilerletir.
3. Galeri uygulama kökünün gerçek ORT-021 çözücüsü başlık, açıklama ve
   ilerleme etiketlerini tam `ÇözülmüşKullanıcıİletisi` zarfları olarak
   verir. Ham dize K08 hazır komut zincirine sokulmaz.
4. State-specific tek yüzey ve plan `%25`, `%75`, `Sonuç` sırasıyla
   hazırlanır; mevcut `DurumPlanGörünümTutamacı` yerinde güncellenir.

Galerideki sibling yüzde şeridi kanıt değildir. Scene testi galerinin kurduğu
ilk opak K08 tutamacını yakalayıp tek başına kök olarak çizer. Aynı yakalanmış
entity üzerinde:

- yaşam ve plan nesilleri `1 -> 2 -> 3` ilerler;
- gerçek bilgi rengi dolgusunun `%75` genişliği `%25` genişliğinin exact
  üç katıdır;
- terminal geçişte bilgi dolgusu tamamen kalkar ve başarı ailesine özgü kare
  geometri görünür;
- açık/koyu pencere görünümünün provider ışıklılıkları ayrı doğrulanır;
- kart, relative K08 öğesine fiziksel `88px` alan ayırır; auto-height
  ebeveynde sıfır alanlı, üretilip çizilmeyen plan kabul edilmez.

Bu sonuç çekirdekteki sabit şerit, eski içerik ve boş element mutasyon
kanıtlarının gerçek galeri tüketicisindeki karşılığıdır. Test yalnız üretim
nesli veya özel etki sayacı okumaz; `Window::painted_quads()` çıktısını
ölçer.

## 2. K09 sahipli çözümün gerçek galeri tüketimi

Exact çekirdek `SimgeÇizimHizmeti::çöz` çağrısı artık çıplak
`SimgeÇözümAkıbeti` veya çözülmüş simge `Arc`ı döndürmez. Galeri:

- public `SimgeÇözümTutamacı`nı çözümden paint sonuna kadar scope içinde
  tutar;
- çizimi aynı sahipli tutamacı `SimgeÇizimHizmeti::çiz`e vererek yapar;
- iki public çözüm tutamacının aynı gerçek sayılan allocationı paylaştığını
  `test_aynı_allocation` ile doğrular;
- mantıksal ve geometri cache'lerinde ilk miss, ikinci hit ve pozitif gerçek
  payload gözlemlerini korur.

Böylece galeri tüketicisi, cache lease'i düşünce dış-sahip muhasebesinden
kopan eski çıplak `Arc` davranışına bağlı değildir. Gerçek payloadın
allocation öncesi rezervasyonu, kapasite/eviction/drop/hot-reload ve kontrollü
mutasyon kanıtları exact çekirdek tesliminin sorumluluğundadır; bu rapor
onları galeri sayaçlarıyla yeniden icat etmez.

## 3. Fresh doğrulama

Bütün geçici, Cargo target, log ve kurtarma yolları yalnız
`/Volumes/Taslaklar/tmp/gpui-k08-gallery-artci-20260913` altındadır.
`TMPDIR`, `TMP`, `TEMP`, `CARGO_TARGET_DIR` ve
`PYTHONPYCACHEPREFIX` bu köke açıkça bağlandı.

| Kapı | Sonuç |
|---|---|
| `cargo fmt --all -- --check` | geçti |
| `cargo check --workspace --all-targets --locked` | geçti |
| `cargo test -p gpui-bilesenleri-galeri --test render_kosumu --locked` | `12/12` geçti |
| `cargo test -p gpui-bilesenleri-galeri --test simgeler --locked` | `4/4` geçti |
| `cargo test --workspace --all-targets --locked` | `264` geçti, `0` kaldı, `1` önceden tanımlı ignored |
| `cargo test --workspace --doc --locked` | geçti; `0` doctest bulundu |
| `git diff --check` | geçti |

Hedef davranış testi:
`k08_k09_gercek_plan_gecisi_ve_sicak_soguk_gpui_cizimi`.
Doğrudan K09 testi:
`k09_ayni_mantiksal_istek_once_kacirma_sonra_vurus_uretir`.

## 4. Kaynak hash sicili

| Dosya | SHA-256 |
|---|---|
| `src/durum_plani.rs` | `d01f97ec15ba0abcc526cad2fa4ac66346e1e07e452e523b941e3498ccd963dd` |
| `src/lib.rs` | `3e2d2876b293d154d7be15766c8b25df3fe1fe71b7fa32190d3f3a3f7503f35d` |
| `src/metin_hizmetleri.rs` | `3e1072c748c738a6143220babc103fa87ec3b349742bfc48da65869c9c70f0d6` |
| `src/sergiler.rs` | `9cb4aef28e46f8c90bb33f17ac1a780bf483a8507733e0f19e1ee55ce9b74b67` |
| `src/simgeler.rs` | `b6d0b7c856d08c52d9250b50d5ad6cd531ddd3f7674b90a1b60edf0c709c0c16` |
| `tests/render_kosumu.rs` | `a172e8d1f609cdbb0108641fe422194a6386e95ff49a8c8b5f84e344d1ca4b08` |
| `tests/simgeler.rs` | `852adfcd7cfde899c33b363cd473cf501aade528d7a706a9a741c6b5840d5997` |

## 5. Kapanış sınırı

| Eksen | Sonuç | Sınır |
|---|---|---|
| K08 plan yaşamı | **Kapalı** | Provider pair, yaşam yetkisi, bileşim damgası, plan ve aynı entity birlikte ilerler. |
| K08 anlamlı TestApp sahnesi | **Kapalı** | Exact kesir geometrisi ve sonuç ailesi gerçek layout/prepaint/paint çıktısında ayrılır. |
| K09 galeri sahipliği | **Kapalı** | Public sahipli tutamaç gerçek çözümden paint sonuna kadar taşınır. |
| K09 çekirdek bellek/eviction | **Exact çekirdek kanıtına bağlı** | Bu repo çekirdeği yeniden ölçmez; exact SHA ve ağaç özeti kaydedilir. |
| Gerçek OS/GPU present | **Açık** | TestApp sahnesi compositor presentı, vsync veya sealed runtime attestation değildir. |

Bu teslim K02 no-store reveal, K10, K06 `ACC-050`, kalan ORT-008 hücreleri,
platform matrisi, gerçek pencere/GPU present, provenance veya runtime
attestation kapılarını kapatmaz. Son kaydedilmiş genel yayın tabanı
`12` engel, `53` Rust yüzey ihlali ve `5` derlenmeyen API hedefidir;
final yeniden ölçüm yapılana kadar bunlar kırmızı kalır. Model sayaçları ya da
headless sahne sonucu yayın onayı sayılmaz.

## 6. Kaynak push ve kurtarma

Kaynak push öncesinde canlı uzak, `origin/main` ve `FETCH_HEAD`
`4784c0d496e0dddecdafed3e8f9b0d955e37de30`; ahead/behind `1/0`,
commit sayısı `1`, merge commit sayısı `0`, diff yedi galeri dosyası ve
ağaç temizdi. Normal exact-SHA push sonrasında canlı uzak, `HEAD`,
`origin/main` ve `FETCH_HEAD`
`2dc1389a5ae98388d08e34ea861d989b8ecdfe9e`; ahead/behind `0/0` ve
ağaç temizdir.

Kurtarma kökü:
`/Volumes/Taslaklar/tmp/gpui-k08-gallery-artci-20260913/recovery/gallery-k08-source-2dc1389`.
Pre-push tam bundle, `4784c0d...` prerequisite'li artımlı bundle ve
post-push tam bundle doğrulandı. Kurtarma `SHA256SUMS` manifestinin SHA-256
değeri
`638a296812876870039371a8ae861fc2dbead4f98aa29ef19cc1fe552f87e605`dir.

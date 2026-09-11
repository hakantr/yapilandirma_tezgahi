# RA-AF Paket D gerçek galeri kanıt raporu

> Tarih: 12 Eylül 2026
>
> Galeri kaynak commit'i: `585212dc3d381e90ed971bde041c464f7b7287b5`
>
> Exact çekirdek bağı: `e5b436ed7a3d8f9c1ef6a844cd86c8a944011845`
>
> Galeri başlangıç tabanı: `8a97452f3195188a7a66a797b94850c8304d703c`
>
> Ortam: Apple Silicon `arm64`, macOS `26.6.2`, `rustc 1.97.1`

Bu rapor Paket D'nin gerçek galeri tüketicisini ve o tüketicinin fresh
kanıtlarını kaydeder. K01-K11 karar onayını runtime kanıtı saymaz; çekirdek
sözleşmelerinin sahibi `gpui_bilesenleri` deposudur. Galeri, yukarıdaki exact
çekirdek commit'ini tüketir.

## 1. Fiziksel tüketici zinciri

### K07 — ürün yardımcı simgesi

- `App` kökünde tek `Arc<SimgeÇizimHizmeti>` kurulur. Ürün manifestindeki
  `input.product-action` kimliği `product.svg` varlığına bağlanır; kimlik eylem
  adından türetilmez ve kayıt yokluğunda yeni kimlik uydurulmaz.
- BİL-010 ürün yuvası kurulurken aynı root-yetkili `SimgeKimliği`
  `.simgeyle(...)` üzerinden taşınır. Model-only yol App/yetki yoksa fail-closed
  kalır.
- Görünür metin girişi tezgâhı aynı ürün yuvasını gerçek render tüketicisinde
  kullanır; erişilebilir ad ürün eyleminde korunur.

### K08 — gerçek plan yaşamı

- Uygulama hizmeti App'te, pencere plan deposu pencere bağlamında yaşar.
- İki soğuk plan kanonik depo ve ORT-018 profil kaydıyla hazırlanır.
- Görünür kart aynı `DurumPlanGörünümTutamacı` entity'sini kullanır. Geçiş
  statik yüzde değişimi değil, aynı yaşayan entity üzerinde exact plan nesli
  `1 → 2` ilerlemesidir.
- K08 kartı erişilemeyen eski katalog dalında bırakılmadı; yaşayan BİL-010
  tezgâhına eklendi.

### K09 — iki cache ve çizim tüketicisi

- Mantıksal çözüm ve geometri/platform hazırlığı aynı root hizmetinin ayrı
  sonlu cache'lerini tüketir.
- Görünür karttaki iki GPUI tuvali aynı snapshot, ürün kimliği, mantıksal istek
  ve geometri anahtarını kullanır: birinci yol soğuk, ikinci yol sıcaktır.
- Gözlem yüzeyi mantıksal/geometri hit, miss, atım, giriş ve resident payload
  değerlerini; ayrıca dışarıda yaşayan ve geçici payload değerlerini ayrı
  taşır. Bu sayaçlar runtime attestation yerine kullanılmaz.

Galeri capability aralıkları exact çekirdek sürümlerine çekildi:
`BİL-010 ^33.2`, `BİL-140 ^3.1`, `ORT-016 ^4.0`, `ORT-018 ^6.2`.

## 2. Fresh doğrulama

Bütün geçici, Cargo target, ölçüm ve log yolları yalnız
`/Volumes/Taslaklar/tmp/gpui-package-d-20260911` altındadır. Koşumlarda
`TMPDIR`, `TMP` ve `TEMP` açıkça
`/Volumes/Taslaklar/tmp/gpui-package-d-20260911/runtime-tmp` değerini aldı.

| Kapı | Sonuç |
|---|---|
| `cargo fmt --all -- --check` | exit `0` |
| `cargo check --workspace --all-targets --offline --locked` | exit `0` |
| `cargo test --workspace --all-targets --offline --locked` | exit `0`; `264` geçti, `0` kaldı, dış canlı YON-005 raporu isteyen `1` test ignored |
| `cargo test --workspace --doc --offline --locked` | exit `0`; `0` doctest bulundu |
| `cargo clippy --workspace --all-targets --offline --locked --message-format=json` | exit `0` |
| `git diff --check` | exit `0` |

Doğrudan davranış eşlemeleri:

| Kesit | Gerçek kanıt |
|---|---|
| K07 kurulum + render | `k07_urun_yuvasi_root_yetkili_simgeyi_kurulusta_ve_renderda_tasir` |
| K08 aynı entity + nesil | `k08_k09_gercek_plan_gecisi_ve_sicak_soguk_gpui_cizimi` içinde `1 → 2` |
| K09 soğuk/sıcak çözüm + paint | aynı testte mantıksal ve geometri miss/hit ile pozitif payload |
| K09 doğrudan hizmet | `k09_ayni_mantiksal_istek_once_kacirma_sonra_vurus_uretir` |
| K07 model sınırı | ürün yolu root hizmeti olmadan simge uydurmaz; canlı yol exact kimliği enjekte eder |
| Gerçek GPUI render paketi | `render_kosumu` finalde `12/12` |
| Host yaşamı | `host_entegrasyonu` finalde `10/10` |

İlk tam test koşumunda sekiz host testi, eski kurucuların yeni zorunlu kök
hizmetini enjekte etmediğini açığa çıkardı. Bütün yaşayan host kurucuları aynı
gerçek hizmete geçirildi ve tam workspace testi yeniden, sıfır hatayla koşuldu;
ilk başarısız koşum final kanıt yerine kullanılmadı.

## 3. Exact API tanı farkı

Anahtar `(code, whitespace-normalized message)` ve çokluk sayısıdır. API göç
kesimi, eski galeri `8a97452...` kodunu aday çekirdek `e5b436ed...` ile derler;
bu nedenle eski tüketicinin yeni fiziksel API karşısındaki kırılmasını ölçer.
Aday galeri aynı çekirdekle `0` error üretir.

- Taban: toplam `56`, benzersiz `20` error.
- Aday: toplam `0`, benzersiz `0` error.
- Yeni: **yok**.
- Kapanan: toplam `56`, aşağıdaki `20` exact anahtar.

| Kod | Çokluk | Normalize ileti |
|---|---:|---|
| `<none>` | 2 | ``cannot construct `SimgeCacheKanıtBağı` with struct literal syntax due to private fields`` |
| `E0004` | 2 | ``non-exhaustive patterns: `&GirişYapılandırmaHatası::SimgeÇizimHizmetiEksik` not covered`` |
| `E0061` | 2 | `this function takes 2 arguments but 1 argument was supplied` |
| `E0308` | 8 | `mismatched types` |
| `E0432` | 2 | ``unresolved imports `gpui_bilesenleri::GpuiSimgeÇizimBağdaştırıcısı`, `gpui_bilesenleri::SimgeÇizimBağlamı``` |
| `E0432` | 2 | ``unresolved imports `gpui_bilesenleri_temel::DerlemeProfili`, `gpui_bilesenleri_temel::PerformansPlatformProfili`, `gpui_bilesenleri_temel::PerformansPlatformu``` |
| `E0560` | 2 | ``struct `PerformansBütçesi` has no field named `birim``` |
| `E0560` | 2 | ``struct `PerformansBütçesi` has no field named `platform_profili``` |
| `E0560` | 2 | ``struct `SimgeCacheBütçesi` has no field named `geometri_bayt_tavanı``` |
| `E0560` | 2 | ``struct `SimgeCacheBütçesi` has no field named `mantıksal_plan_bayt_tavanı``` |
| `E0560` | 2 | ``struct `SimgeCacheBütçesi` has no field named `mantıksal_plan_girdi_tavanı``` |
| `E0560` | 2 | ``struct `SimgeSnapshotYapılandırması` has no field named `etkin_küme``` |
| `E0560` | 2 | ``struct `SimgeSnapshotYapılandırması` has no field named `nötr_eksik_simge``` |
| `E0560` | 2 | ``struct `SimgeSnapshotYapılandırması` has no field named `taban_küme``` |
| `E0599` | 2 | ``no associated function or constant named `kur` found for struct `BellekSimgeSnapshotDeposu` in the current scope`` |
| `E0599` | 2 | ``no associated function or constant named `kökten` found for struct `SimgeAdAlanıYetkisi` in the current scope`` |
| `E0599` | 2 | ``no associated function or constant named `yeni` found for struct `ÖlçümKimliği` in the current scope`` |
| `E0599` | 8 | ``no method named `simge_çizim_bağlamını_değiştir` found for mutable reference `&mut GirişKutusu` in the current scope`` |
| `E0599` | 4 | ``no variant, associated function, or constant named `Tahsis` found for enum `BütçeSınıfı` in the current scope`` |
| `E0599` | 4 | ``no variant, associated function, or constant named `ÇizimKaresi` found for enum `BütçeSınıfı` in the current scope`` |

## 4. Exact Clippy tanı farkı

Clippy karşılaştırması geçerli iki eşlenmiş grafiği kullanır: tarihsel galeri
`8a97452...` kendi kayıtlı çekirdeği `e2fc5e72...` ile; aday galeri
`585212d...` exact çekirdeği `e5b436ed...` ile.

- Taban: toplam `158`, benzersiz `41` warning.
- Aday: toplam `161`, benzersiz `49` warning.
- Değişmeden kalan çokluk: `151`.
- Yeni `clippy::*`: **yok**.

Kapanan exact tanılar:

| Kod | Çokluk | Normalize ileti |
|---|---:|---|
| `clippy::redundant_closure` | 6 | `redundant closure` |
| `dead_code` | 1 | ``methods `durum_göstergesi_durumu`, `durum`, `etkin_ime`, `doğrulama_işleri`, and `metin` are never used`` |

Yeni exact tanılar, Paket A-C çekirdek farkının galeri Clippy grafiğine giren
`dead_code` kayıtlarıdır; Paket D bunları kapandı diye yeniden sınıflandırmaz:

| Kod | Çokluk | Normalize ileti |
|---|---:|---|
| `dead_code` | 1 | ``field `0` is never read`` |
| `dead_code` | 1 | ``field `iç` is never read`` |
| `dead_code` | 1 | ``field `örnek` is never read`` |
| `dead_code` | 1 | ``fields `bütçe`, `veri_kümesi_kökü`, `fiziksel_kaynak_kökü`, and `değerler` are never read`` |
| `dead_code` | 1 | ``fields `form` and `form_sürümü` are never read`` |
| `dead_code` | 1 | ``fields `işlem` and `hedef` are never read`` |
| `dead_code` | 1 | ``method `aynı_sağlayıcı` is never used`` |
| `dead_code` | 1 | ``method `kapı` is never used`` |
| `dead_code` | 1 | ``methods `durum_göstergesi_durumu`, `etkin_ime`, `doğrulama_işleri`, and `metin` are never used`` |
| `dead_code` | 1 | ``variant `DurumPlanBelleği` is never constructed`` |

Eski galeriyi yanlışlıkla aday çekirdeğe bağlayan ilk Clippy taban denemesi
`28 previous errors` ile derlenmedi. O koşum Clippy farkına katılmadı; ham
çıktısı yanlış eşlemenin izini kaybetmemek için kurtarma kaydında tutuldu.

## 5. Etkilenen `akici-dev` ölçümü

Her koşum `200` tekrar, `1600×1000`, headless CPU'dur.

| Senaryo | Cache açık ort./p50/p95/min | Cache kapalı ort./p50/p95/min | Ort. azalma | Kolon açık/kapalı |
|---|---:|---:|---:|---:|
| D temiz | 1.381 / 1.360 / 1.497 / 1.353 ms | 3.356 / 3.331 / 3.510 / 3.283 ms | 58.850% | 0/200 · 200/200 |
| K tuş | 1.503 / 1.481 / 1.582 / 1.467 ms | 3.503 / 3.496 / 3.636 / 3.413 ms | 57.094% | 0/200 · 200/200 |
| S seçici | 3.028 / 3.032 / 3.282 / 2.904 ms | 3.634 / 3.622 / 3.795 / 3.479 ms | 16.676% | 200/200 · 200/200 |
| T tercih | 2.960 / 2.956 / 3.120 / 2.899 ms | 3.554 / 3.545 / 3.681 / 3.479 ms | 16.714% | 200/200 · 200/200 |

Bu sonuç yalnız düzenek içi CPU A/B farkıdır. GPU present, vsync, gerçek pencere
frame pacing veya ORT-018 runtime attestation kanıtı değildir.

## 6. Kapanış eksenleri

| Eksen | Paket D K07-K09 galeri kesiti | Sınır |
|---|---|---|
| Sözleşme | **Bağlı** | Kanonik anlam çekirdek `e5b436ed...` içindedir; bu commit yeni sözleşme onaylamaz. |
| Fiziksel API | **Kapalı** | Exact çekirdek, gerçek dış-sandık galeri tarafından `--locked --offline` derlenir; API adayı `0` error. |
| Üretim | **Kapalı** | Kök hizmetleri, yaşayan K08 planı ve K09 tuvalleri galeri üretim kaynaklarında kuruludur. |
| Tüketici | **Kapalı** | K07 ürün yuvası ile K08/K09 kartı gerçek, görünür BİL-010 tezgâh yolundadır. |
| Davranış | **Kapalı (bu kesit)** | Tam workspace, gerçek GPUI render/host testleri ve cache/nesil gözlemleri yeşildir. |
| Runtime | **Kısmi / açık kapı** | GPUI test penceresi ve headless CPU ölçümü vardır; GPU present/vsync, elle masaüstü ve platform attestation yoktur. |

## 7. Açık kalan kapsam

Bu teslim aşağıdakileri kapatmaz ve yayın kapanışı bunlar yüzünden ayrı izlenir:

1. K02 no-store reveal ve buna bağlı gerçek platform yolları.
2. K10 odak/erişilebilirlik zincirinin kalan uçtan uca kanıtı.
3. K06 `ACC-050` ve diğer açık ORT-008 hücreleri.
4. Linux/Windows ve gerekli diğer platform matrisleri.
5. GPU present/vsync, gerçek pencere frame pacing ve runtime attestation.
6. Dış canlı YON-005 rapor girdisi gerektiren ignored test.

Model sayaçları veya bu rapordaki CPU ölçümleri gerçek GPUI yayın attestation'ı
sayılmaz; karşılanmayan yayın kapıları kırmızı kalmalıdır.

## 8. Kurtarma ve push kaydı

Kaynak commit normal push öncesinde canlı `fetch` ve bağımsız `ls-remote` ile
doğrulandı: uzak taban, `origin/main` ve merge-base
`8a97452f3195188a7a66a797b94850c8304d703c`; ahead/behind `0/1`; gönderilecek
tek commit `585212dc3d381e90ed971bde041c464f7b7287b5`; ağaç temizdi.

Normal push sonrasında canlı uzak, `origin/main`, `FETCH_HEAD` ve `HEAD`
`585212dc3d381e90ed971bde041c464f7b7287b5`; ahead/behind `0/0`; ağaç temizdi.

Kurtarma kökü:
`/Volumes/Taslaklar/tmp/gpui-package-d-20260911/recovery/gallery-source-585212d`.
Tam bundle, taban prerequisite'li artımlı bundle ve patch hedef dizinden
doğrulandı. `SHA256SUMS` `26/26` geçti; manifestin SHA-256 değeri
`f806f2dcd22ebad9aff32fa11e2eebd014e8f24f115ca5c62b1a42caf0a4d1de`.

İlk artımlı bundle çağrısı salt SHA aralığıyla adlandırılmış uç üretmedi ve
exit `128` döndü; başarısız çıktı kullanılmadı. Bundle `HEAD ^8a97452...`
biçimiyle yeniden üretildi ve doğrulandı.

# BİL-010 23.0.0 host/runtime kanıt makbuzu

> Tarih: 1 Eylül 2026
> Host deposu: `yapilandirma_tezgahi`, bu makbuzun commitlendiği atomun
> ebeveyni `bcbc300` (zincir: `e683674 → 791b9d3 → b076c6c → bb87848 →
> 1623664 → bce72cc → 6ed5718 → bcbc300`)
> Ürün deposu: `gpui_bilesenleri` `708e39edc7ab61e80709f272c87f7ed4fd6e4d95`
> Kardeş HEAD'ler: gpui `dca2db8d46bf`, wgpu `d4359d7494`
> Ortam: Apple M4 Pro, macOS 26.6.2, rustc 1.97.1 (repo pinli)

Bu makbuz host tarafının kanıt sınırını kaydeder; sözleşme anlamının
sahibi ürün deposudur. "Derleme" ve "runtime" satırları bilinçli ayrıdır.

## 1. Koşulan kapılar (canlı, bu turda)

| Kapı | Komut | Sonuç |
|---|---|---|
| Biçim | `cargo fmt --all -- --check` | temiz |
| Derleme | `cargo check --workspace --all-targets --locked` | 0 hata |
| Host entegrasyonu | `cargo test -p gpui-bilesenleri-galeri --test host_entegrasyonu --locked` | 10/10 |
| Workspace testleri | `cargo test --workspace --all-targets --locked` | 252/0 |
| WASM hedefli check | `cargo check --locked --target wasm32-unknown-unknown -p gpui-bilesenleri-galeri-wasm` | 0 hata |
| WASM paketi | `python3 tools/wasm_galeri_hazirla.py` (wasm-bindgen 0.2.127) | üretildi |
| Kare ölçümü A/B | `KARE_OLCUM=1 cargo test --profile akici-dev -p gpui-bilesenleri-galeri --test kare_olcumu --locked -- --nocapture` ×8 | `olcumler/kare_olcumu_0006_ab/` |

**Kapı sapması (ölçülen):** `cargo check --workspace --all-targets
--target wasm32-unknown-unknown` bu bağımlılık grafiğinde geçemez —
test hedefleri unix-only `wait-timeout` dev bağımlılığını çeker ve
masaüstü feature birleşimi nightly-only `wasm_thread`'i açar (E0554).
WASM'ın kanonik kapısı hedefli check + paket build'idir; ikisi de sıfır
hata.

## 2. BİL-010 23.0.0 kabul matrisi ↔ test eşlemesi

Testler gerçek `ORT-002` kök hizmetleri ve gerçek GPUI test penceresiyle
koşar; K4–K7 kompozisyonu gerçek `replace_and_mark_text_in_range` /
`replace_text_in_range` platform giriş noktalarıyla sürer.

| # | Kabul maddesi | Kanıt (test fonksiyonu · dosya) |
|---|---|---|
| K1 | Kuruluş çağıranın verdiği yerel bağlamı kullanır | `kurulus_enjekte_yerel_baglami_gercekten_kullanir` · `tests/host_entegrasyonu.rs` (en/el maske çıktısı gerçekten ayrışır) |
| K2 | Aynı yerel kimliğe geçiş gerçek no-op | `ayni_yerel_baglama_gecis_gercek_noop` · aynı dosya (Ok + 0 bildirim + eksenler sabit) |
| K3 | Kompozisyonsuz farklı yerel geçişi başarılı | `kompozisyonsuz_farkli_yerel_gecisi_basarili` · aynı dosya (plan yenilenir, tek bildirim, sürüm +1) |
| K4 | Etkin IME'de farklı yerel exact `CompositionEtkin` | `etkin_kompozisyonda_ret_kurtarma_ve_asili_eksen_yoklugu` · aynı dosya |
| K5 | Ret kolunda eksen korunumu | aynı test: metin, ham metin, seçim, kompozisyon değeri, IME aralığı, `değer_sürümü`, etkin yerel; bildirim 0. Geri-al yığını ayrı probe edilmedi — `değer_sürümü` sabitliği + ürün deposunun yapısal/davranış nöbetleri (`bil010_yerel_baglam_enjeksiyonu`, `kutu_testleri`) bu ekseni taşır |
| K6 | Commit/unmark sonrası yeniden deneme başarılı | aynı test (insertText commit + retry `Ok`); unmark kolu `metin_hizmetleri.rs` bekleyen-eşitleme testi |
| K7 | `insertText` sonrası sahte/asılı kompozisyon yok | aynı test (`composition.is_none()`) + `inserttext_commiti_kompozisyonu_dusurur_asili_eksen_olusmaz` · `src/metin_hizmetleri.rs` (galeri kökü düzeyi: bekleyen hedef uygulanır, kalıcı ret kaydı ve tanı satırı oluşmaz) |
| K8 | Bildirim/yeniden çizim döngüsü yok | K2 (no-op 0 bildirim), K4 (ret 0 bildirim), K3 (başarı tek bildirim); kök `yerel_uygulama_hatası` yuvası yalnız geçişte `notify` üretir (`lib.rs`) |
| K9 | Açık maskeli başlangıç `"ab" → "AB_"` tabanı | `acik_maskeli_baslangic_tabani_ve_escape` · `tests/host_entegrasyonu.rs` |
| K10 | Düzenleme sonrası `Escape` `"AB_"` tabanına döner | aynı test (gerçek `DuzenlemeyiIptalEt` eylemi, çizilen konak + odak) |
| K11 | Hata gösterimi typed'ı string sezgisine çevirmez | `tani_satirlari_exact_varyant_adini_tasir` · `src/metin_hizmetleri.rs`; yuvalar `Option<GirişHatası>`/`TercihEşitlemeKaydı` tipli, sunum satırı exact varyant taşır |
| K12 | Galeri dil seçenekleri yalnız sergi örneği | Belge kaydı: `DEVIR_NOTU.md §5` (1 Eyl 2026 girişi, ORT-002 sayım notuyla) |

"Asılı/işaretsiz `CompositionEtkin`" ayrımı korunmadı: ürün `1e7fce9`
sonrasında bu duruma platform giriş noktalarından erişilemediği canlı
ölçüldü (eski nöbet güncel ürünle düşüyordu) ve dal + açıklamalar
`bce72cc` ile kaldırıldı.

## 3. Masaüstü runtime (ayrı satır)

- **Derleme:** `akici-dev` profili, 0 hata.
- **Smoke runtime:** `gpui-bilesenleri-galeri-masaustu` gerçek pencerede
  açıldı, 10 sn ayakta kaldı, tezgâh ekranı gerçekten çizildi (yerel
  ekran yakalamasıyla doğrulandı; görüntü üçüncü-taraf ekran içeriği
  taşıdığı için depoya alınmadı), süreç temiz sonlandırıldı.
- **Yapılmadı:** senaryolu etkileşimli masaüstü koşumu (gerçek platform
  IME'siyle yerel değişimi/ret/kurtarma ve maskeli Escape'in elle
  gözlemi). Bu eksenlerin davranış kanıtı yukarıdaki gerçek-GPUI
  entegrasyon testlerindedir; masaüstü penceresinde elle koşum ayrı bir
  oturum ister.

## 4. WASM runtime (ayrı satır)

- **Derleme kanıtı:** hedefli check 0 hata; release paketi üretildi.
- **Runtime kanıtı (tarayıcı, bu turda sürüldü):** galeri boot + WebGPU
  kuruluşu + tam çizim; gerçek klavye olaylarıyla metin yazımı (`ab`),
  Telefon maskesi seçimi ve rakamların yuvalara işlenmesi
  (`+90 0(532) …`), `Escape` ile düzenleme tabanına dönüş (maske
  yapılandırması korunarak).
- **Bu turda yakalanan ve kapatılan wasm kusuru:** ölçüm sarmalayıcısı
  `render_ölç` wasm'da `Instant::now` paniciyle ilk karede düşürüyordu;
  `bcbc300` cfg ayrımıyla kapattı ve düzeltme tarayıcıda doğrulandı.
- **Yapılmadı:** tarayıcıda IME kompozisyon koşumu (sentetik kompozisyon
  olayları güvenilir sürülemiyor).

## 5. Performans (ayrı satır)

Tahsis temizliği atomunun A/B ölçümü: hedef senaryo K·tuş vuruşu ort
medyanı −%4,3 (14/16 ikili karşılaştırmada önde), D −%1,2..1,7; S/T
gürültü bandında, kazanç iddiası yok. Düzenek + ham çıktılar:
`olcumler/kare_olcumu_0006_ab/`, özet `raporlar/PERFORMANS_MIMARISI.md
§8.9`. Headless CPU sayısıdır; GPU/present ile eşitlenmez.

## 6. Açık kalan kanıtlar

1. Masaüstünde elle, gerçek platform IME'siyle senaryolu koşum (§3).
2. Tarayıcıda IME kompozisyon koşumu (§4).
3. Linux/Windows hedefleri: bu makbuz yalnız macOS + wasm32'yi kapsar.

Bu makbuz ürün deposundaki `ACC-158` test kanıtlarının yerine geçmez;
onların üstüne host bütünleşme ve runtime katmanını kaydeder.

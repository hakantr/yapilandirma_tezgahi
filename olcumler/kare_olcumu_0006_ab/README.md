# 0006 performans atomunun A/B kare ölçümü (1 Eyl 2026)

Tahsis temizliği atomunun (tuş-vuruşu tercih klonları, kök-kare tercih
klonları, `tezgah_etkileşim_hedefi` tek kaynağı, tema üreticisi memoları)
kare maliyetine etkisi; **eski milisaniye değerleri kullanılmadı**, iki taraf
da bu turda aynı makinede yeniden ölçüldü.

## Düzenek

- A = `bce72cc` (göç-uyum commit'i; 0006 içeriği YOK) — ayrı worktree.
- B = 0006 içerikli çalışma ağacı (bu dizinin commitlendiği atom).
- Komut (iki tarafta birebir aynı):
  `KARE_OLCUM=1 cargo test --profile akici-dev -p gpui-bilesenleri-galeri --test kare_olcumu --locked -- --nocapture`
- Hakem: `tests/kare_olcumu.rs` — 1600×1000 viewport, ISINMA=30,
  TEKRAR=200, headless CPU (element kurulumu + yerleşim + prepaint +
  paint sahne yazımı; GPU/present YOK — bu sayı gerçek pencere
  girdi→present süresiyle eşitlenemez).
- Sıra: A1 B1 B2 A2 · B3 A3 A4 B4 (ABBA + BAAB; ısı/sürüklenme dengesi).
- Ortam: Apple M4 Pro, macOS 26.6.2, ürün `gpui_bilesenleri` `708e39e`.

## Sonuç (p50 / ort medyanları, ms; n=4 koşum/taraf)

| Senaryo | A p50 | B p50 | Δp50 | A ort | B ort | Δort |
|---|---|---|---|---|---|---|
| D · temiz | 1.099 | 1.080 | −1.7% | 1.132 | 1.119 | −1.2% |
| K · tuş vuruşu | 1.234 | 1.196 | −3.0% | 1.290 | 1.234 | −4.3% |
| S · seçici | 2.870 | 2.878 | +0.3% | 2.922 | 2.908 | −0.5% |
| T · tercih | 2.835 | 2.866 | +1.1% | 2.864 | 2.905 | +1.4% |

- Hedef senaryo **K**'de kazanım gözlemlenebilir ve tutarlıdır: ort
  bazında B'nin dört koşumu da A'nın üçünün altında; 16 ikili
  karşılaştırmanın 14'ünde B önde.
- **D** küçük ama tutarlı (15/16 ikili karşılaştırmada B önde).
- **S** ve **T** koşumlar-arası gürültü bandının (±%2–4) içindedir; bu iki
  senaryoda kazanç iddiası YOKTUR.
- Karar: hedef senaryodaki gözlemlenebilir kazanım nedeniyle atom bütün
  olarak korunur; geri çekilen parça yok.

Ham çıktılar `kosum_{A,B}{1..4}.txt` dosyalarındadır ve tablo yalnız
bu çıktılardan türetilmiştir.

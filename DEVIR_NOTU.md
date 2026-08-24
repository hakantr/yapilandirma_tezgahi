# Yapılandırma Tezgâhı · taşınma devir notu

> Tarih: 24 Ağustos 2026
> Kaynak: `gpui_bilesenleri` deposu, `main` @ `d1b48a6`
> Nitelik: Normatif olmayan devir notu; bağlayıcı kaynaklar
> `../gpui_bilesenleri/sozlesmeler/` altındaki yaşayan kanonik gövdelerdir.

Bu depo, Yapılandırma Tezgâhı uygulamasını (galeri + masaüstü + WASM
koşucuları) taşır. Tezgâh `gpui_bilesenleri` deposunda çok büyüdüğü için
kullanıcı kararıyla (24 Ağu 2026) buraya taşındı. Tezgâhın işi değişmedi:
bileşen ailelerinin kendi sözleşmelerine uyumunu ölçen sınama yüzeyidir;
sözleşmelerin ve bileşen kodunun sahibi `gpui_bilesenleri`dir.

## 1. Kardeş depo düzeni (zorunlu)

Path bağımlılıkları ve kanıt okumaları şu düzeni varsayar:

```text
~/github/
├── gpui/                 # eş kaynak GPUI (YÖN-002)
├── gpui_bilesenleri/     # sözleşmeler + bileşen sandıkları
└── yapilandirma_tezgahi/ # bu depo
```

- `Cargo.toml` → `../gpui/crates/{gpui,gpui_platform,gpui_web}` ve
  `../gpui_bilesenleri/crates/{gpui-bilesenleri,gpui-bilesenleri-kabuk}`.
- Bazı kanıt testleri kardeş depodan `include_str!` ile okur (aşağıda).
- Linux'ta kardeş `gpui` klonunun bu depoyla aynı eşitlemede olması
  gerekir; eski klon `taffy` sürüm düşürmesi gibi sahte farklar üretir.

## 2. Taşınanlar ve uyarlamalar

| Parça | Not |
|---|---|
| `crates/gpui-bilesenleri-galeri{,-masaustu,-wasm}` | Adlar korundu; yeniden adlandırma (ör. `yapilandirma-tezgahi`) ayrı kullanıcı kararı |
| `tools/wasm_galeri_hazirla.py`, `wasm_galeri_sunucu.py` | Yol düzeni aynı, değişiklik gerekmedi |
| `varliklar/` | **Kopya**: kaynak depoda da duruyor (simge çözümleme sözleşmesi ve `yayin_kapisi.py` oradan okuyor); galeri buradakini gömer |
| `raporlar/TEZGAH_DEVIR_NOTU.md` | Önceki turların sicili (bulgular 1-27, kullanıcı kararları) — tarihçe olarak taşındı |
| `raporlar/BILESEN_TEZGAHI_YENI_TASARIM_GOC_PLANI.md`, `TEZGAH_EKSEN_DAGITIM_HARITASI.md` | Göç planı ve eksen haritası |
| `Tezgah_yeni_tasarimi/` | Tasarım referansları; git dışı (kaynak depodaki gelenek korundu) |
| `rust-toolchain.toml` | `1.97.1` |

Taşınma uyarlamaları:

- Testlerdeki depolar-arası `include_str!` yolları kardeş depoya çevrildi
  (`tezgah_gosterge.rs`, `tezgah_kabul.rs`, `yon006.rs`):
  `../../gpui-bilesenleri/…` → `../../../../gpui_bilesenleri/crates/…`,
  `../../../sozlesmeler/…` → `../../../../gpui_bilesenleri/sozlesmeler/…`,
  `../../../raporlar/wasm_runtime.md` → kardeş depo karşılığı.
- `ort001` kanıt testi `boş_galeri_render_yolu_io_çağrısı_içermez`
  kaynak deponun uyum paketinden **buraya** taşındı
  (`tests/ort001_kaniti.rs`): kanıt, kanıtladığı kodun yanında yaşar.

## 3. Kaynak depoda yapılan temizlik

- Üç galeri sandığı, iki wasm betiği ve üç tezgâh raporu kaldırıldı;
  workspace üyeleri, `gpui_platform`/`gpui_web`/`wasm-bindgen` bağımlılık
  girdileri, `.gitignore` ve CI'daki WASM adımları temizlendi;
  `katman_siniri_denetimi.py` dört sandığa indirildi.
- Kanıt manifesti yeniden üretildi: kanıtsız ölçüt **1106 → 1118**.
  Galeriyle giden 12 ölçütün kanıtı (YÖN-006 galeri sözleşmesi kanıtları
  başta) artık bu depodaki testlerde yaşıyor; kaynak deponun denetimi
  onları görmüyor. Bu bilinçli ve kayıtlı bir sonuçtur — kalıcı çözüm
  (çapraz depo kanıt köprüsü ya da kabul) ayrı karar ister.

## 4. Doğrulama durumu

- Bu depo: `cargo test` — 15 hedef, tüm testler yeşil (galeri 202 +
  taşınan ort001 kanıtı). WASM paketi: `python3 tools/wasm_galeri_hazirla.py`.
- Kaynak depo: `cargo check --workspace` temiz; bil010/uyum paketleri
  yeşil; sözleşme denetimi `YAPISAL BAŞARILI — 1118 ÖLÇÜT KANITSIZ`.
- Bilinen borç (kaynak depo): `sozlesme_api_faz*` test hedefleri HEAD'de
  derlenmiyor; taşınmadan bağımsız, kayıtlı.

## 5. Sıradaki işler (yeni sohbetin gündemi)

1. **Performans mimarisi turu** — kullanıcı onaylı plan. **Birinci tur
   tamamlandı (24 Ağu 2026):** kök `observe(&alan)`/`subscribe` köprüleri
   kaldırılıp gözlem `AlanDurumPaneli` + `OlayAkışıPaneli` entity'lerine
   taşındı; `§29` raporu ve kod paneli tercih sürümüne bağlandı;
   `akici-dev` profili eklendi. Mekanik, kazançlar, ikinci turun işleri
   (sağ kolon bölüm entity'leri + `cached` bölgeler + `yuva_görünürlük_notu`
   taşınması) ve ölçüm hedefleri: `raporlar/PERFORMANS_MIMARISI.md`.
   (Önceki devirde anılan "TEZGAH_DEVIR_NOTU sonundaki performans raporu"
   hiç yazılmamıştı; o boşluğu bu belge doldurur.) Linux ölçümü bekliyor.
2. Crate/paket adlarının yeniden adlandırılması (kullanıcı kararı).
3. Bu depoya CI kurmak (kaynak depodaki `uygulama-iskeleti` işinin
   uyarlaması; kardeş `gpui` **ve** `gpui_bilesenleri` checkout'ları gerekir).
4. 12 kanıtsız ölçüt için kalıcı karar (madde 3'teki köprü/kabul).
5. Tezgâh gündemi kaldığı yerden: açık borçlar 17, 19, 20, 22, 25
   (hepsi başka atomlara kapılı; `raporlar/TEZGAH_DEVIR_NOTU.md` §3.2).

Yürürlükteki kullanıcı kararları ve tezgâh kapsam ilkesi ("tezgâh yalnız
bileşenin kendi sözleşme uyumunu ölçer; ödünç bileşen kusurları kendi
turlarına") `raporlar/TEZGAH_DEVIR_NOTU.md`'dedir ve geçerliliğini korur.

## 6. Çalıştırma

```bash
cargo run -p gpui-bilesenleri-galeri-masaustu          # masaüstü
python3 tools/wasm_galeri_hazirla.py                   # WASM paketi
python3 tools/wasm_galeri_sunucu.py --port 8000        # tarayıcı
```

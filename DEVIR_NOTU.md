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

## 5. Kardeş eşitleme sicili

### 24 Ağu 2026 — gpui `bbc4a9b1da` / wgpu `d4359d749`

Kardeş `gpui`, Zed `1b86941c` + kardeş wgpu `d4359d74` senkronunu ve
`cargo-shear` bağımlılık temizliğini aldı (kendi etki analizi:
`../gpui/ZED_WGPU_2026_08_24_ETKI_ANALIZI.md`). Public API değişikliği
yok; bu depoda uygulananlar:

- Kilit yenilendi (`ba58bfe`): temizlikle düşen paketler silindi, 71
  satır silme.
- wasm-bindgen `=0.2.127` pinine hizalandı (`b6d37c2`): taşınmadan beri
  betik CLI'dan 0.2.126 bekliyordu; CLI 0.2.127'ye yükseltildi,
  `tools/wasm_galeri_hazirla.py` beklentileri güncellendi, paket yeniden
  üretildi.
- Tarayıcı önizlemesi için `.claude/launch.json` eklendi (`ee8dbd7`).

Doğrulama: `cargo test` 16 hedef / 211 test yeşil; wasm32 cross-check
temiz; masaüstü smoke (8 sn ayakta) geçti; tarayıcıda yeni paket
(`build.json` sha eşleşmesi) hatasız render edildi. Bilinen uyarılar
(`block 0.1.6` future-incompat, wgpu-core `expect(unused)`) gpui/upstream
sahipliğinde — bu depoda iş yok.

## 6. Sıradaki işler (yeni sohbetin gündemi)

1. **Performans mimarisi turu** — kullanıcı onaylı plan. **Üç tur + ölçüm
   turu tamamlandı (24 Ağu 2026):** (1) kök `observe(&alan)`/`subscribe`
   köprüleri `AlanDurumPaneli` + `OlayAkışıPaneli` entity'lerine taşındı,
   `§29` raporu ve kod paneli tercih sürümüne bağlandı, `akici-dev`
   profili eklendi; (2) sağ kolon `BölümlerPaneli`ne çıkarıldı ve
   `yuva_görünürlük_notu` kendi gözleyen paneline taşındı; (3) açılır
   liste içerikleri tembelleşti (yalnız açıkken kurulur), çözülmüş görünüm
   ve yarıçap tavanı tema sürümüne bağlandı; (4) **ölçüm turu** (`tests/kare_olcumu.rs`,
   `KARE_OLCUM=1` + `akici-dev`) önce ikinci turun `Entity::cached`
   kararını çürüttü — sayaç, kolonun açılıştan sonra hiç yeniden
   kurulmadığını, yani bayat yüzey ürettiğini gösterdi — sonra kök nedeni
   buldu: GPUI'de `notify` bir `cached` sınırını patlatmaz (`App::notify`
   yalnız pencerenin `tracked_entities` kümesindeki entity'ler için
   `invalidate_view` çağırır; önbellekten dönen view o kümeye girmez),
   **`refresh` patlatır**. Kök artık `kolonu_geçersizle` →
   `refresh_windows` ile tercih/tema/seçici/dış bildirim değişimlerinde
   kolonu tazeliyor. Bir dış inceleme ölçüm penceresinin
   yanlış kareyi ölçtüğünü yakaladı (S/T'de kolon kurulumu ölçüm dışında
   kalıyordu); pencere girdiden ekrana kadar geçen bütün CPU işini
   kapsayacak biçimde düzeltildi ve geçersizleme hedefli hâle getirildi
   (`defer` + `Window::refresh`, yalnız kendi penceresi). Düzeltilmiş
   kazanç: tuş vuruşu **3,50 → 1,24 ms (%65)**, temiz kare 3,43 → 1,13 ms,
   seçici/tercih %18; kolon tuş vuruşunda 0/200, seçici ve tercihte
   200/200. Kapı: `tests/kolon_tazeligi.rs` (tazelik **ve** kazanç).
   **Sol kolon ve üst şerit için aynı desen denendi, ölçüldü ve
   uygulanmadı** (raporun §7'si): üst şerit `cached`in boyut kısıtına
   takılıyor (payı ~0,15 ms), sol kolonun kayan bloğu ise alan gözleyen
   panelleri *içerdiği* için her tuş vuruşunda kirlenir — sağ kolon
   çalışıyor çünkü o panellerin kardeşi, atası değil. Mekanik, sayılar ve
   sıradaki işler: `raporlar/PERFORMANS_MIMARISI.md`. (Önceki devirde anılan
   "TEZGAH_DEVIR_NOTU sonundaki performans raporu" hiç yazılmamıştı; o
   boşluğu bu belge doldurur.) **Açık:** Linux ölçümü ve gerçek
   input-to-present (sunum/vsync/fiziksel girdi). **Bu depo için
   120 FPS / "sıfıra yakın gecikme" iddiası yoktur.**
2. Crate/paket adlarının yeniden adlandırılması (kullanıcı kararı).
3. Bu depoya CI kurmak (kaynak depodaki `uygulama-iskeleti` işinin
   uyarlaması; kardeş `gpui` **ve** `gpui_bilesenleri` checkout'ları gerekir).
4. 12 kanıtsız ölçüt için kalıcı karar (madde 3'teki köprü/kabul).
5. Tezgâh gündemi kaldığı yerden: açık borçlar 17, 19, 20, 22, 25
   (hepsi başka atomlara kapılı; `raporlar/TEZGAH_DEVIR_NOTU.md` §3.2).

Yürürlükteki kullanıcı kararları ve tezgâh kapsam ilkesi ("tezgâh yalnız
bileşenin kendi sözleşme uyumunu ölçer; ödünç bileşen kusurları kendi
turlarına") `raporlar/TEZGAH_DEVIR_NOTU.md`'dedir ve geçerliliğini korur.

## 7. Çalıştırma

```bash
cargo run -p gpui-bilesenleri-galeri-masaustu          # masaüstü
python3 tools/wasm_galeri_hazirla.py                   # WASM paketi
python3 tools/wasm_galeri_sunucu.py --port 8000        # tarayıcı
```

# Ant Design Icons — vendor kaydı

| Alan | Değer |
|---|---|
| Kaynak | `github.com/ant-design/ant-design-icons` |
| Commit | `6c18c63fbcfcf71dae09cd6bd6d63a48f8b688f1` |
| Alınma tarihi | `2026-07-30` |
| Alınan yol | `packages/icons-svg/svg/` |
| Lisans | MIT — `LICENSE` dosyası bu dizinde birebir korunur |
| Telif | Copyright (c) 2018-present Ant UED, https://xtech.antfin.com/ |

## Normalleştirme

Dosyalar birebir kopyalanmadı. `ORT-016` varlık güvenliği kuralları gereği
şunlar sökülerek alındı:

- `<?xml ...?>` bildirimi ve `<!DOCTYPE ...>` — dış DTD URL’i taşıyordu
  (51 dosya); `ORT-016` SVG içinde dış URL’i reddeder.
- İhracatçı artıkları: `t`, `p-id`, `class`, `width`, `height`, `version`,
  `standalone`, `xmlns:xlink` öznitelikleri.
- Gömülü sabit renkler (`fill`/`stroke` hex değerleri, 156 dosya);
  `ORT-016` SVG içinde sabit ürün rengini reddeder ve renk `ORT-004`
  temasından gelir.

Geometri (`viewBox="0 0 1024 1024"`, yol verisi) değiştirilmedi.

## İki tonlu ikonlar

`twotone` ailesinin tamamı kaynakta iki renkli tasarlanmıştır. ORT-016
`2.0.0`, bunu gömülü boya olarak taşımaz: `katalog.json` içindeki
`iki_tonlu_ikincil_yollar` ayrımı aynı viewport ve dönüşümü kullanan iki
monokrom katman manifestinin provenance girdisidir. Private GPUI
bağdaştırıcısı ikincil ve birincil katmanı iki ayrı `paint_svg` geçişinde,
scene'e eklemeden önce birlikte hazırlar. Eksik/uyuşmaz katman veya
kanıtlanamayan yüksek-karşıtlık güvenliği aynı semantik kimliğin tek-ton
siluetine düşer; yarım katman çizilmez.

Alan adı vendor tema adını değil, kaynak geometrisinde sabit renkler
sökülmeden önce gözlenen ikincil katman sınırını anlatır. Bu nedenle tek
istisna olan `filled/x.svg` kaydı da bu alandadır: exact upstream dosyada
ana zemin yolu `#000`, 1, 2 ve 3 numaralı ayrıntı yolları `#FFF` taşır.
Normalleştirme bu renkleri sökerken geometriyi değiştirmemiş ve
`katalog.json` içinde `filled: [1, 2, 3]` kaydıyla katman ayrımını korumuştur.
Bu kayıt silinemez veya tek-ton kanıtı sayılamaz. `filled` vendor etiketi de
kendiliğinden public `SimgeGörselBiçimi::Dolu` eşlemesi kurmaz; snapshot
hazırlığı varlığı ancak doğrulanmış iki katman ve güvenli tek-ton yedeğiyle
`İkiTonlu` biçime eşleyebilir ya da adaydan çıkarmalıdır.

Bu vendor kaydı tek başına dağıtıma hazır `SimgeSnapshotı` kanıtı değildir.
Snapshot ayrıca ORT-016'nın sıralı semantik eşleme, normalize dosya/geometri
SHA-256 manifesti ve lisans kapılarının tamamını geçer. Kaynak asset adları
public `SimgeKimliği` garantisi değildir.

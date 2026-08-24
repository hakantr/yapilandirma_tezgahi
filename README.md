# Yapılandırma Tezgâhı

GPUI bileşen ailelerinin sözleşme uyumunu ölçen sınama yüzeyi: galeri,
masaüstü ve WASM koşucuları. `gpui_bilesenleri` deposundan taşındı;
sözleşmelerin ve bileşen kodunun sahibi orasıdır.

Kardeş depo düzeni zorunludur: `../gpui` ve `../gpui_bilesenleri`
bu deponun yanında durur (ayrıntı: `DEVIR_NOTU.md`).

```bash
cargo run -p gpui-bilesenleri-galeri-masaustu   # masaüstü galeri
python3 tools/wasm_galeri_hazirla.py            # WASM paketi
python3 tools/wasm_galeri_sunucu.py --port 8000 # tarayıcıda çalıştır
```

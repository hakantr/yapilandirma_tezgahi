# Minimal Zed `InputField` ölçümü

Bu bağımsız hedef, Zed kaynağını değiştirmeden gerçek
`ui_input::InputField` bileşenini açar. Alanın içindeki
`Editor::single_line` entity'si 50 ms arayla 400 kez, gerçek
`EntityInputHandler::replace_text_in_range` yolundan değiştirilir.

Depo kökünden çalıştırma:

```sh
CARGO_TARGET_DIR=../zed/target \
  cargo run \
  --manifest-path olcumler/zed_input_minimal/Cargo.toml \
  --release \
  --locked
```

Ölçüm geçerli sayılmak için 400/400 düzenleme ve en az 360 `draw`
örneği ister. Sonuç CPU tarafındaki GPUI `draw` süresidir; GPU süresini,
sunumu veya input-to-present gecikmesini ölçmez.

`Cargo.lock`, Zed revizyonunun bağımlılık çözümünü bu hedef için sabitler.
Zed'in kök workspace yamaları ayrı workspace tarafından miras alınmadığı
için `Cargo.toml` içinde ayrıca kayıtlıdır.

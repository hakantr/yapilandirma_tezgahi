//! `ORT-001` kanıtı: galeri render yolu G/Ç çağrısı içermez.
//!
//! Test, galeri sandığıyla birlikte `gpui_bilesenleri` deposundan taşındı;
//! kanıt taşınan kodun yanında yaşar.

#[test]
fn boş_galeri_render_yolu_io_çağrısı_içermez() {
    let kaynak = include_str!("../src/lib.rs");
    for yasak in ["std::fs", "std::net", "read_to_string", "File::open"] {
        assert!(!kaynak.contains(yasak), "render yolunda yasak I/O: {yasak}");
    }
}

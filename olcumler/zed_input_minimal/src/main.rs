//! Zed `ui_input::InputField` için minimal gerçek-pencere ölçümü.
//!
//! Zed kaynağını değiştirmez. Tek `InputField` oluşturur; alanın içindeki
//! gerçek `Editor::single_line` entity'sini 50 ms arayla 400 kez
//! `EntityInputHandler::replace_text_in_range` yolundan değiştirir.

use std::{sync::Arc, time::Duration};

use assets::Assets;
use editor::Editor;
use gpui::{
    App, AppContext as _, Bounds, Context, Entity, EntityInputHandler, IntoElement, Render, Window,
    WindowBounds, WindowOptions, div, prelude::*, px, size,
};
use ui_input::{ErasedEditor, InputField};

const DÜZENLEME_MS: u64 = 50;
const DÜZENLEME_SAYISI: u64 = 400;

struct MinimalZedGirişi {
    alan: Entity<InputField>,
    silinmiş_editör: Arc<dyn ErasedEditor>,
    editör: Entity<Editor>,
}

impl MinimalZedGirişi {
    fn yeni(pencere: &mut Window, bağlam: &mut Context<Self>) -> Self {
        let alan = bağlam.new(|bağlam| InputField::new(pencere, bağlam, "Metin girin…"));
        let silinmiş_editör = alan.read(bağlam).editor().clone();
        let editör = silinmiş_editör
            .as_any()
            .downcast_ref::<Entity<Editor>>()
            .expect("Zed InputField, Editor::single_line taşımalı")
            .clone();
        editör.update(bağlam, |editör, bağlam| {
            editör.set_text("izleyici ölçümü alfa", pencere, bağlam)
        });
        Self {
            alan,
            silinmiş_editör,
            editör,
        }
    }

    fn yaz(&mut self, metin: &str, pencere: &mut Window, bağlam: &mut Context<Self>) {
        self.silinmiş_editör.select_all(pencere, bağlam);
        self.editör.update(bağlam, |editör, bağlam| {
            EntityInputHandler::replace_text_in_range(editör, None, metin, pencere, bağlam);
        });
    }
}

impl Render for MinimalZedGirişi {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .child(div().w(px(640.)).child(self.alan.clone()))
    }
}

fn main() {
    gpui_platform::application()
        .with_assets(Assets)
        .run(|bağlam: &mut App| {
            release_channel::init(semver::Version::new(0, 0, 0), bağlam);
            Assets
                .load_fonts(bağlam)
                .expect("Zed gömülü yazı tipleri yüklenmeli");
            settings::init(bağlam);
            theme_settings::init(theme::LoadThemes::JustBase, bağlam);
            ui_input::ERASED_EDITOR_FACTORY
                .set(|pencere, bağlam| {
                    bağlam
                        .new(|bağlam| Editor::single_line(pencere, bağlam))
                        .update(bağlam, |editör, bağlam| editör.erased(bağlam))
                })
                .expect("Zed silinmiş editör fabrikası bir kez kurulmalı");

            bağlam.activate(true);
            let sınırlar = Bounds::centered(None, size(px(1600.), px(1000.)), bağlam);
            let pencere = bağlam
                .open_window(
                    WindowOptions {
                        window_bounds: Some(WindowBounds::Windowed(sınırlar)),
                        ..Default::default()
                    },
                    |pencere, bağlam| {
                        theme_settings::setup_ui_font(pencere, bağlam);
                        bağlam.new(|bağlam| MinimalZedGirişi::yeni(pencere, bağlam))
                    },
                )
                .expect("minimal Zed input penceresi açılmalı");
            pencere
                .update(bağlam, |_, pencere, _| pencere.activate_window())
                .expect("minimal Zed input penceresi etkinleşmeli");
            ölçümü_planla(pencere, bağlam);
        });
}

fn ölçümü_planla(pencere: gpui::WindowHandle<MinimalZedGirişi>, bağlam: &mut App) {
    eprintln!(
        "minimal Zed InputField: {DÜZENLEME_SAYISI} düzenleme × \
         {DÜZENLEME_MS} ms; fiziksel yazma gerekmez"
    );
    bağlam
        .spawn(async move |bağlam| {
            bağlam
                .background_executor()
                .timer(Duration::from_secs(1))
                .await;
            pencere
                .update(bağlam, |kök, pencere, bağlam| {
                    kök.yaz("izleyici ölçümü ısınma", pencere, bağlam)
                })
                .ok();
            bağlam
                .background_executor()
                .timer(Duration::from_millis(250))
                .await;
            let başlangıç = pencere
                .update(bağlam, |_, pencere, _| pencere.frame_duration_snapshot())
                .ok();
            let mut uygulanan = 0;
            for sıra in 0..DÜZENLEME_SAYISI {
                let metin = if sıra % 2 == 0 {
                    "izleyici ölçümü alfa"
                } else {
                    "izleyici ölçümü beta"
                };
                if pencere
                    .update(bağlam, |kök, pencere, bağlam| {
                        kök.yaz(metin, pencere, bağlam)
                    })
                    .is_err()
                {
                    break;
                }
                uygulanan += 1;
                bağlam
                    .background_executor()
                    .timer(Duration::from_millis(DÜZENLEME_MS))
                    .await;
            }
            pencere
                .update(bağlam, |_, pencere, _| {
                    raporla(pencere, uygulanan, başlangıç.as_ref())
                })
                .ok();
            bağlam.update(|bağlam| bağlam.quit());
        })
        .detach();
}

fn raporla(
    pencere: &Window,
    uygulanan: u64,
    başlangıç: Option<&gpui::profiler::FrameDurationSnapshot>,
) {
    const MS: f64 = 1_000_000.;
    let mut çizim = pencere.frame_duration_snapshot().draw_duration_histogram;
    let tam = başlangıç
        .is_some_and(|başlangıç| çizim.subtract(&başlangıç.draw_duration_histogram).is_ok());
    println!("\n— minimal Zed ui_input::InputField —");
    println!(
        "ortam                    {:.0}×{:.0} px · ölçek {:.1}× · etkin {} · derleme release",
        f32::from(pencere.viewport_size().width),
        f32::from(pencere.viewport_size().height),
        pencere.scale_factor(),
        if pencere.is_window_active() {
            "evet"
        } else {
            "hayır"
        },
    );
    if çizim.is_empty() {
        println!("çizim (draw)             örnek yok");
    } else {
        println!(
            "çizim (draw)             n={:<5} p50 {:7.3} ms · p95 {:7.3} ms · \
             p99 {:7.3} ms · ort {:7.3} ms",
            çizim.len(),
            çizim.value_at_quantile(0.50) as f64 / MS,
            çizim.value_at_quantile(0.95) as f64 / MS,
            çizim.value_at_quantile(0.99) as f64 / MS,
            çizim.mean() / MS,
        );
    }
    println!("düzenleme                {uygulanan}/{DÜZENLEME_SAYISI}");
    if !tam || uygulanan != DÜZENLEME_SAYISI || çizim.len() < DÜZENLEME_SAYISI * 9 / 10 {
        eprintln!("GEÇERSİZ: histogram farkı, düzenleme veya draw kapısı geçilmedi");
    }
}

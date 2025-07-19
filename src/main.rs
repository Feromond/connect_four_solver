use eframe::egui;
use crate::egui::IconData;
use std::path::Path;
use connect_four_solver::ConnectFourApp;

fn main() -> Result<(), eframe::Error> {

    let icon_path = Path::new("icon.ico");

    let icon_data = if icon_path.exists() {
        let image = image::open(icon_path)
            .expect("Failed to open icon.ico")
            .to_rgba8();
        let (width, height) = image.dimensions();
        Some(IconData {
            rgba: image.into_raw(),
            width,
            height,
        })
    } else {
        // Fallback: use a transparent 32x32 icon.
        Some(IconData {
            rgba: vec![0; 32 * 32 * 4],
            width: 32,
            height: 32,
        })
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 500.0])
            .with_icon(icon_data.unwrap()),
        ..Default::default()
    };

    eframe::run_native(
        "Connect Four Solver",
        options,
        Box::new(|_cc| Ok(Box::<ConnectFourApp>::default())),
    )
}

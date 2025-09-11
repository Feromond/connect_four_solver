#![cfg_attr(all(target_os = "windows", not(debug_assertions)), windows_subsystem = "windows")]
use connect_four_solver::ConnectFourApp;
use eframe::egui;
use egui::IconData;
use log::info;
#[cfg(not(target_os = "macos"))]
use std::path::Path;

fn main() -> Result<(), eframe::Error> {
    // Initialize logger; ignore errors if already initialized
    let _ = env_logger::builder().format_timestamp(None).try_init();
    info!("Starting Connect Four Solver");
    #[cfg(not(target_os = "macos"))]
    let icon_path = Path::new("icon.ico");
    #[cfg(not(target_os = "macos"))]
    let icon_data = if icon_path.exists() {
        match image::open(icon_path) {
            Ok(img) => {
                let image = img.to_rgba8();
                let (width, height) = image.dimensions();
                info!("Loaded window icon from icon.ico ({}x{})", width, height);
                Some(IconData {
                    rgba: image.into_raw(),
                    width,
                    height,
                })
            }
            Err(_) => {
                // Fallback: use a transparent 32x32 icon if icon.ico cannot be read
                info!("icon.ico could not be read; using transparent placeholder icon");
                Some(IconData {
                    rgba: vec![0; 32 * 32 * 4],
                    width: 32,
                    height: 32,
                })
            }
        }
    } else {
        // Fallback: use a transparent 32x32 icon.
        info!("icon.ico missing; using transparent placeholder icon");
        Some(IconData {
            rgba: vec![0; 32 * 32 * 4],
            width: 32,
            height: 32,
        })
    };

    #[cfg(not(target_os = "macos"))]
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([700.0, 740.0])
            .with_icon(icon_data.unwrap()),
        ..Default::default()
    };

    #[cfg(target_os = "macos")]
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([700.0, 740.0])
            .with_icon(IconData::default()), // This allows for default os icon, which will be set in cargo bundle
        ..Default::default()
    };

    eframe::run_native(
        "Connect Four Solver",
        options,
        Box::new(|_cc| Ok(Box::<ConnectFourApp>::default())),
    )
}

use eframe::egui;
use connect_four_solver::ConnectFourApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 500.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Connect Four Solver",
        options,
        Box::new(|_cc| Ok(Box::<ConnectFourApp>::default())),
    )
}

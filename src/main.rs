#![windows_subsystem = "windows"]

mod app;
mod pcileech;

use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 400.0]),
        ..Default::default()
    };
    eframe::run_native(
        "PCILeech GUI",
        options,
        Box::new(|_cc| Ok(Box::new(app::PciLeechApp::default()))),
    )
}

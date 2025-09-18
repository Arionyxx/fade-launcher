mod app;
mod ui;
mod search;
mod config;
mod theme;

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([600.0, 400.0])
            .with_decorations(false) // Remove window decorations for sleek look
            .with_transparent(true)
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "Fade Launcher",
        options,
        Box::new(|cc| {
            // Setup custom fonts and style
            setup_custom_fonts(&cc.egui_ctx);
            setup_style(&cc.egui_ctx);
            
            Box::new(app::FadeLauncher::new(cc))
        }),
    )
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    
    // You can add custom fonts here later
    // fonts.font_data.insert(
    //     "custom_font".to_owned(),
    //     egui::FontData::from_static(include_bytes!("../assets/font.ttf")),
    // );
    
    ctx.set_fonts(fonts);
}

fn setup_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    
    // Set the overall theme to dark to complement our gradient
    ctx.set_visuals(egui::Visuals::dark());
    
    // Customize spacing and rounding
    style.spacing.item_spacing = egui::vec2(12.0, 8.0);
    style.spacing.button_padding = egui::vec2(16.0, 8.0);
    style.visuals.window_rounding = egui::Rounding::same(15.0);
    style.visuals.menu_rounding = egui::Rounding::same(10.0);
    
    ctx.set_style(style);
}
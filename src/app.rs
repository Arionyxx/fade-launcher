use eframe::egui;
use crate::{theme::FadeTheme, search::AppSearcher, ui::LauncherUI};
use std::time::{Duration, Instant};

pub struct FadeLauncher {
    search_query: String,
    search_results: Vec<AppInfo>,
    searcher: AppSearcher,
    ui: LauncherUI,
    last_search_time: Instant,
    animation_time: f32,
    show_settings: bool,
}

#[derive(Clone, Debug)]
pub struct AppInfo {
    pub name: String,
    pub path: String,
    pub icon_path: Option<String>,
    pub description: Option<String>,
    pub score: f32, // Relevance score for search results
}

impl FadeLauncher {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut searcher = AppSearcher::new();
        
        // Start initial app scanning in background
        searcher.start_initial_scan();
        
        Self {
            search_query: String::new(),
            search_results: Vec::new(),
            searcher,
            ui: LauncherUI::new(),
            last_search_time: Instant::now(),
            animation_time: 0.0,
            show_settings: false,
        }
    }
    
    fn update_search(&mut self) {
        if self.search_query.is_empty() {
            self.search_results = self.searcher.get_recent_apps(10);
        } else {
            self.search_results = self.searcher.search(&self.search_query, 10);
        }
        self.last_search_time = Instant::now();
    }
    
    fn launch_app(&mut self, app: &AppInfo) {
        if let Err(e) = self.searcher.launch_app(&app.path) {
            eprintln!("Failed to launch app {}: {}", app.name, e);
        } else {
            // Add to recent apps
            self.searcher.add_to_recent(app.clone());
            
            // Optionally minimize or close the launcher after launching
            // This would require additional egui context methods
        }
    }
}

impl eframe::App for FadeLauncher {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update animation time
        self.animation_time += ctx.input(|i| i.unstable_dt);
        
        // Check if we need to update search results
        if self.last_search_time.elapsed() > Duration::from_millis(300) {
            if !self.search_query.is_empty() || self.search_results.is_empty() {
                self.update_search();
            }
        }
        
        // Handle global shortcuts
        ctx.input(|i| {
            if i.key_pressed(egui::Key::Escape) {
                if self.show_settings {
                    self.show_settings = false;
                } else if !self.search_query.is_empty() {
                    self.search_query.clear();
                    self.update_search();
                } else {
                    // Exit the application
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            }
            
            if i.modifiers.ctrl && i.key_pressed(egui::Key::Comma) {
                self.show_settings = true;
            }
        });
        
        // Create the main window with custom styling
        egui::CentralPanel::default()
            .frame(egui::Frame::none())
            .show(ctx, |ui| {
                self.ui.draw_main_window(
                    ui,
                    &mut self.search_query,
                    &self.search_results,
                    self.animation_time,
                    |app| self.launch_app(app),
                );
            });
        
        // Show settings window if needed
        if self.show_settings {
            egui::Window::new("Settings")
                .frame(egui::Frame::window(&ctx.style()).fill(FadeTheme::BACKGROUND_DARK))
                .show(ctx, |ui| {
                    ui.colored_label(FadeTheme::TEXT_PRIMARY, "Settings");
                    ui.separator();
                    
                    ui.colored_label(FadeTheme::TEXT_SECONDARY, "Scan directories:");
                    ui.label("Configure which directories to scan for applications");
                    
                    ui.separator();
                    
                    if ui.button("Close").clicked() {
                        self.show_settings = false;
                    }
                });
        }
        
        // Request repaint for smooth animations
        ctx.request_repaint_after(Duration::from_millis(16)); // ~60 FPS
    }
}
use eframe::egui::{self, *};
use crate::{theme::FadeTheme, app::AppInfo};

pub struct LauncherUI {
    search_focused: bool,
    selected_index: usize,
}

impl LauncherUI {
    pub fn new() -> Self {
        Self {
            search_focused: false,
            selected_index: 0,
        }
    }
    
    pub fn draw_main_window(
        &mut self,
        ui: &mut Ui,
        search_query: &mut String,
        results: &[AppInfo],
        animation_time: f32,
        mut on_launch: impl FnMut(&AppInfo),
    ) {
        let rect = ui.max_rect();
        
        // Draw animated gradient background
        self.draw_gradient_background(ui, rect, animation_time);
        
        // Center the main content
        let content_width = 600.0f32.min(rect.width() * 0.8);
        let content_height = 400.0f32.min(rect.height() * 0.8);
        
        let content_rect = Rect::from_center_size(
            rect.center(),
            Vec2::new(content_width, content_height),
        );
        
        // Main content area with semi-transparent background
        let main_frame = Frame::none()
            .fill(FadeTheme::BACKGROUND_DARK)
            .rounding(Rounding::same(20.0))
            .stroke(Stroke::new(2.0, FadeTheme::BORDER_COLOR))
            .inner_margin(Margin::same(30.0));
        
        ui.allocate_ui_at_rect(content_rect, |ui| {
            main_frame.show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    
                    // App title with gradient text
                    self.draw_title(ui, animation_time);
                    
                    ui.add_space(30.0);
                    
                    // Search box
                    let search_response = self.draw_search_box(ui, search_query);
                    
                    ui.add_space(20.0);
                    
                    // Results list
                    self.draw_results_list(ui, results, &mut on_launch);
                    
                    // Handle keyboard navigation
                    self.handle_keyboard_navigation(ui, search_response, results, &mut on_launch);
                });
            });
        });
    }
    
    fn draw_gradient_background(&self, ui: &mut Ui, rect: Rect, animation_time: f32) {
        let painter = ui.painter();
        
        // Create animated gradient mesh
        let mesh = self.create_animated_gradient_mesh(rect, animation_time);
        painter.add(Shape::mesh(mesh));
        
        // Add some subtle animated particles/dots for extra flair
        self.draw_animated_particles(ui, rect, animation_time);
    }
    
    fn create_animated_gradient_mesh(&self, rect: Rect, time: f32) -> Mesh {
        let mut mesh = Mesh::default();
        
        // Create a grid of vertices for smooth gradient
        let grid_size = 20;
        let width = rect.width();
        let height = rect.height();
        
        for i in 0..=grid_size {
            for j in 0..=grid_size {
                let x = rect.left() + (i as f32 / grid_size as f32) * width;
                let y = rect.top() + (j as f32 / grid_size as f32) * height;
                
                // Create gradient position with animation
                let gradient_pos = (i as f32 / grid_size as f32) + 
                    0.3 * (time * 0.5 + j as f32 * 0.1).sin();
                
                let color = FadeTheme::gradient_color(gradient_pos);
                let vertex = Vertex { pos: pos2(x, y), uv: pos2(0.0, 0.0), color };
                mesh.vertices.push(vertex);
            }
        }
        
        // Create triangles
        for i in 0..grid_size {
            for j in 0..grid_size {
                let idx = |x: usize, y: usize| (x * (grid_size + 1) + y) as u32;
                
                // Two triangles per quad
                mesh.indices.extend_from_slice(&[
                    idx(i, j), idx(i + 1, j), idx(i, j + 1),
                    idx(i + 1, j), idx(i + 1, j + 1), idx(i, j + 1),
                ]);
            }
        }
        
        mesh
    }
    
    fn draw_animated_particles(&self, ui: &mut Ui, rect: Rect, time: f32) {
        let painter = ui.painter();
        
        // Draw some floating particles
        for i in 0..15 {
            let phase = time + i as f32 * 0.4;
            let x = rect.left() + (phase * 0.3).sin() * rect.width() * 0.3 + rect.width() * 0.5;
            let y = rect.top() + (phase * 0.2).cos() * rect.height() * 0.3 + rect.height() * 0.5;
            
            let size = 2.0 + (phase * 2.0).sin().abs() * 3.0;
            let alpha = (0.3 + 0.2 * (phase * 1.5).sin()) * 255.0;
            
            let color = if i % 2 == 0 {
                FadeTheme::with_alpha(FadeTheme::PINK_SECONDARY, alpha as u8)
            } else {
                FadeTheme::with_alpha(FadeTheme::AQUA_SECONDARY, alpha as u8)
            };
            
            painter.circle_filled(pos2(x, y), size, color);
        }
    }
    
    fn draw_title(&self, ui: &mut Ui, animation_time: f32) {
        ui.vertical_centered(|ui| {
            let title_color = FadeTheme::gradient_color(0.3 + 0.2 * (animation_time * 2.0).sin());
            
            ui.colored_label(title_color, RichText::new("Fade Launcher")
                .size(36.0)
                .strong());
            
            ui.colored_label(FadeTheme::TEXT_MUTED, 
                RichText::new("Type to search for applications...")
                    .size(14.0));
        });
    }
    
    fn draw_search_box(&mut self, ui: &mut Ui, search_query: &mut String) -> Response {
        let search_style = ui.style_mut();
        search_style.visuals.extreme_bg_color = FadeTheme::SEARCH_BACKGROUND;
        search_style.visuals.selection.bg_fill = FadeTheme::SEARCH_FOCUS;
        
        ui.horizontal(|ui| {
            ui.add_space(20.0);
            
            let search_response = ui.add_sized(
                [ui.available_width() - 40.0, 40.0],
                TextEdit::singleline(search_query)
                    .hint_text("Search applications...")
                    .font(TextStyle::Heading)
                    .desired_width(f32::INFINITY)
            );
            
            // Auto-focus search box
            if !self.search_focused {
                search_response.request_focus();
                self.search_focused = true;
            }
            
            search_response
        }).inner
    }
    
    fn draw_results_list(&mut self, ui: &mut Ui, results: &[AppInfo], on_launch: &mut impl FnMut(&AppInfo)) {
        if results.is_empty() {
            ui.vertical_centered(|ui| {
                ui.add_space(40.0);
                ui.colored_label(FadeTheme::TEXT_MUTED, "No applications found");
            });
            return;
        }
        
        ScrollArea::vertical()
            .max_height(250.0)
            .show(ui, |ui| {
                for (index, app) in results.iter().enumerate() {
                    let is_selected = index == self.selected_index;
                    self.draw_app_item(ui, app, is_selected, on_launch);
                }
            });
    }
    
    fn draw_app_item(&self, ui: &mut Ui, app: &AppInfo, is_selected: bool, on_launch: &mut impl FnMut(&AppInfo)) {
        let item_height = 50.0;
        let (rect, response) = ui.allocate_exact_size(
            Vec2::new(ui.available_width(), item_height),
            Sense::click()
        );
        
        // Background for selected item
        if is_selected || response.hovered() {
            let bg_color = if is_selected {
                FadeTheme::ACTIVE_COLOR
            } else {
                FadeTheme::HOVER_COLOR
            };
            
            ui.painter().rect_filled(
                rect,
                Rounding::same(8.0),
                bg_color
            );
        }
        
        // App content
        ui.allocate_ui_at_rect(rect.shrink(10.0), |ui| {
            ui.horizontal(|ui| {
                // TODO: Add app icon here
                ui.add_space(5.0);
                
                ui.vertical(|ui| {
                    ui.colored_label(FadeTheme::TEXT_PRIMARY, 
                        RichText::new(&app.name).size(16.0).strong());
                    
                    if let Some(desc) = &app.description {
                        ui.colored_label(FadeTheme::TEXT_SECONDARY,
                            RichText::new(desc).size(12.0));
                    } else {
                        ui.colored_label(FadeTheme::TEXT_MUTED,
                            RichText::new(&app.path).size(11.0));
                    }
                });
            });
        });
        
        if response.clicked() {
            on_launch(app);
        }
    }
    
    fn handle_keyboard_navigation(
        &mut self,
        ui: &mut Ui,
        _search_response: Response,
        results: &[AppInfo],
        on_launch: &mut impl FnMut(&AppInfo),
    ) {
        ui.input(|i| {
            if !results.is_empty() {
                if i.key_pressed(Key::ArrowDown) {
                    self.selected_index = (self.selected_index + 1).min(results.len() - 1);
                } else if i.key_pressed(Key::ArrowUp) {
                    self.selected_index = self.selected_index.saturating_sub(1);
                } else if i.key_pressed(Key::Enter) {
                    if let Some(app) = results.get(self.selected_index) {
                        on_launch(app);
                    }
                }
            }
        });
    }
}
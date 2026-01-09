mod asset_manager;
mod editor;
mod level;
mod ui;

use editor::EditorState;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_title("Éditeur de Niveaux - ARM64"),
        ..Default::default()
    };

    eframe::run_native(
        "Level Editor",
        options,
        Box::new(|_cc| Ok(Box::new(EditorApp::default()))),
    )
}

struct EditorApp {
    state: EditorState,
}

impl Default for EditorApp {
    fn default() -> Self {
        Self {
            state: EditorState::new(),
        }
    }
}

impl eframe::App for EditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ui::draw_top_panel(ctx, &mut self.state);
        ui::draw_side_panel(ctx, &mut self.state);
        ui::draw_bottom_panel(ctx, &mut self.state);
        ui::draw_central_panel(ctx, &mut self.state);
    }
}


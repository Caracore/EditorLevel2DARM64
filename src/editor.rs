use crate::asset_manager::AssetManager;
use crate::level::{Level, TileData};
use egui::{Color32, Pos2, Rect, Sense, Vec2};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tool {
    Paint,
    Erase,
    Select,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PaintMode {
    ColorTile([u8; 3]), // RGB
    TextureTile { tileset_id: usize, tile_index: u32 },
}

pub struct EditorState {
    pub level: Level,
    pub current_tool: Tool,
    pub paint_mode: PaintMode,
    pub current_layer: usize,
    pub zoom: f32,
    pub offset: Vec2,
    pub show_grid: bool,
    pub is_dragging: bool,
    pub last_painted: Option<(i32, i32)>,
    pub asset_manager: AssetManager,
    pub selected_tileset: Option<usize>,
    pub tileset_scroll: f32,
    pub selected_color: [u8; 3],
    pub color_hex_input: String,
}

impl EditorState {
    pub fn new() -> Self {
        Self {
            level: Level::new("Nouveau Niveau".to_string(), 64, 48, 16),
            current_tool: Tool::Paint,
            paint_mode: PaintMode::ColorTile([139, 69, 19]), // Marron par défaut
            current_layer: 1,
            zoom: 1.0,
            offset: Vec2::ZERO,
            show_grid: true,
            is_dragging: false,
            last_painted: None,
            asset_manager: AssetManager::new(),
            selected_tileset: None,
            tileset_scroll: 0.0,
            selected_color: [139, 69, 19],
            color_hex_input: "#8B4513".to_string(),
        }
    }

    pub fn draw_canvas(&mut self, ui: &mut egui::Ui) {
        let available_size = ui.available_size();
        let (response, painter) = ui.allocate_painter(available_size, Sense::click_and_drag());

        // Gestion du zoom avec molette
        if response.hovered() {
            ui.input(|i| {
                let scroll = i.smooth_scroll_delta.y;
                if scroll != 0.0 {
                    let zoom_factor = 1.0 + scroll * 0.001;
                    self.zoom = (self.zoom * zoom_factor).clamp(0.2, 5.0);
                }
            });
        }

        // Gestion du déplacement
        if response.dragged_by(egui::PointerButton::Middle) {
            self.offset += response.drag_delta();
        }

        let tile_size = self.level.tile_size as f32 * self.zoom;
        let canvas_rect = response.rect;
        let canvas_center = canvas_rect.center();

        // Dessiner l'arrière-plan
        painter.rect_filled(canvas_rect, 0.0, Color32::from_rgb(30, 30, 30));

        // Dessiner les tiles
        for layer_idx in 0..self.level.layers.len() {
            let layer = &self.level.layers[layer_idx];
            if !layer.visible {
                continue;
            }

            for (&(tx, ty), &tile_data) in &layer.tiles {
                let screen_pos = canvas_center
                    + self.offset
                    + Vec2::new(tx as f32 * tile_size, ty as f32 * tile_size);

                let tile_rect = Rect::from_min_size(
                    Pos2::new(screen_pos.x, screen_pos.y),
                    Vec2::new(tile_size, tile_size),
                );

                if canvas_rect.intersects(tile_rect) {
                    let alpha = if layer_idx == self.current_layer { 255 } else { 100 };
                    
                    match tile_data {
                        TileData::Color(rgb) => {
                            painter.rect_filled(
                                tile_rect,
                                0.0,
                                Color32::from_rgba_unmultiplied(rgb[0], rgb[1], rgb[2], alpha),
                            );
                        }
                        TileData::Texture { tileset_id, tile_index } => {
                            if let Some(tileset) = self.asset_manager.get_tileset(tileset_id) {
                                // Calculer les coordonnées UV du tile
                                let tile_x = tile_index % tileset.columns;
                                let tile_y = tile_index / tileset.columns;
                                
                                let uv = Rect::from_min_max(
                                    Pos2::new(
                                        tile_x as f32 / tileset.columns as f32,
                                        tile_y as f32 / tileset.rows as f32,
                                    ),
                                    Pos2::new(
                                        (tile_x + 1) as f32 / tileset.columns as f32,
                                        (tile_y + 1) as f32 / tileset.rows as f32,
                                    ),
                                );
                                
                                let tint = Color32::from_rgba_unmultiplied(255, 255, 255, alpha);
                                painter.image(tileset.texture.id(), tile_rect, uv, tint);
                            }
                        }
                    }
                }
            }
        }

        // Dessiner la grille
        if self.show_grid {
            let grid_color = Color32::from_rgba_unmultiplied(100, 100, 100, 50);
            for gx in -20..40 {
                for gy in -20..40 {
                    let screen_pos = canvas_center
                        + self.offset
                        + Vec2::new(gx as f32 * tile_size, gy as f32 * tile_size);

                    let tile_rect = Rect::from_min_size(
                        Pos2::new(screen_pos.x, screen_pos.y),
                        Vec2::new(tile_size, tile_size),
                    );

                    if canvas_rect.intersects(tile_rect) {
                        painter.rect_stroke(tile_rect, 0.0, (1.0, grid_color));
                    }
                }
            }
        }

        // Gestion des outils
        if response.hovered() {
            if let Some(pointer_pos) = response.hover_pos() {
                let rel_pos = pointer_pos.to_vec2() - canvas_center.to_vec2() - self.offset;
                let tile_x = (rel_pos.x / tile_size).floor() as i32;
                let tile_y = (rel_pos.y / tile_size).floor() as i32;

                // Prévisualisation
                let preview_rect = Rect::from_min_size(
                    canvas_center
                        + self.offset
                        + Vec2::new(tile_x as f32 * tile_size, tile_y as f32 * tile_size),
                    Vec2::new(tile_size, tile_size),
                );

                let preview_color = match self.current_tool {
                    Tool::Paint => {
                        match self.paint_mode {
                            PaintMode::ColorTile(rgb) => {
                                Color32::from_rgba_unmultiplied(rgb[0], rgb[1], rgb[2], 150)
                            }
                            PaintMode::TextureTile { .. } => {
                                Color32::from_rgba_unmultiplied(255, 255, 255, 150)
                            }
                        }
                    }
                    Tool::Erase => Color32::from_rgba_unmultiplied(255, 0, 0, 100),
                    Tool::Select => Color32::from_rgba_unmultiplied(255, 255, 0, 100),
                };

                painter.rect_stroke(preview_rect, 0.0, (2.0, preview_color));
                
                // Prévisualisation de la texture si mode texture
                if self.current_tool == Tool::Paint {
                    if let PaintMode::TextureTile { tileset_id, tile_index } = self.paint_mode {
                        if let Some(tileset) = self.asset_manager.get_tileset(tileset_id) {
                            let tile_x = tile_index % tileset.columns;
                            let tile_y = tile_index / tileset.columns;
                            
                            let uv = Rect::from_min_max(
                                Pos2::new(
                                    tile_x as f32 / tileset.columns as f32,
                                    tile_y as f32 / tileset.rows as f32,
                                ),
                                Pos2::new(
                                    (tile_x + 1) as f32 / tileset.columns as f32,
                                    (tile_y + 1) as f32 / tileset.rows as f32,
                                ),
                            );
                            
                            painter.image(
                                tileset.texture.id(),
                                preview_rect,
                                uv,
                                Color32::from_rgba_unmultiplied(255, 255, 255, 200),
                            );
                        }
                    }
                }

                // Application de l'outil avec clic gauche/droit inversé pour gomme
                let primary_click = response.dragged_by(egui::PointerButton::Primary)
                    || response.clicked_by(egui::PointerButton::Primary);
                let secondary_click = response.dragged_by(egui::PointerButton::Secondary)
                    || response.clicked_by(egui::PointerButton::Secondary);
                
                if primary_click || secondary_click {
                    if self.last_painted != Some((tile_x, tile_y)) {
                        if let Some(layer) = self.level.layers.get_mut(self.current_layer) {
                            // Logique inversée : clic droit = gomme par défaut
                            // Si outil Gomme sélectionné : clic gauche = gomme, clic droit = paint
                            let should_erase = if self.current_tool == Tool::Erase {
                                primary_click  // Avec gomme active : clic gauche efface
                            } else {
                                secondary_click  // Sinon : clic droit efface
                            };
                            
                            let should_paint = if self.current_tool == Tool::Erase {
                                secondary_click  // Avec gomme active : clic droit peint
                            } else {
                                primary_click  // Sinon : clic gauche peint
                            };
                            
                            if should_erase {
                                layer.set_tile(tile_x, tile_y, TileData::empty());
                            } else if should_paint && self.current_tool != Tool::Select {
                                let tile_data = match self.paint_mode {
                                    PaintMode::ColorTile(rgb) => {
                                        TileData::Color(rgb)
                                    }
                                    PaintMode::TextureTile { tileset_id, tile_index } => {
                                        TileData::Texture { tileset_id, tile_index }
                                    }
                                };
                                layer.set_tile(tile_x, tile_y, tile_data);
                            }
                        }
                        self.last_painted = Some((tile_x, tile_y));
                    }
                }
            }
        }

        if response.drag_stopped() {
            self.last_painted = None;
        }
    }
}

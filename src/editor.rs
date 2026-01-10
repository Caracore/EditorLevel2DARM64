use crate::asset_manager::AssetManager;
use crate::level::{Level, TileData, Layer};
use egui::{Color32, Pos2, Rect, Sense, Vec2};
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tool {
    Paint,
    Erase,
    Select,
    LineFill,  // Remplissage en ligne (horizontal ou vertical)
    RectFill,  // Remplissage en carré/rectangle
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SelectionData {
    pub start: Option<(i32, i32)>,
    pub end: Option<(i32, i32)>,
    pub is_active: bool,  // Indique si une sélection est active
}

#[derive(Debug, Clone)]
pub struct SelectionContent {
    pub tiles: Vec<((i32, i32), TileData)>,
    pub origin: (i32, i32),  // Position d'origine de la sélection
    pub width: i32,
    pub height: i32,
}

/// Structure pour l'historique (Undo/Redo)
#[derive(Debug, Clone)]
struct HistoryState {
    layers: Vec<Layer>,
    current_layer: usize,
}

struct History {
    undo_stack: VecDeque<HistoryState>,
    redo_stack: VecDeque<HistoryState>,
    max_size: usize,
}

impl History {
    fn new(max_size: usize) -> Self {
        Self {
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
            max_size,
        }
    }

    fn push(&mut self, state: HistoryState) {
        if self.undo_stack.len() >= self.max_size {
            self.undo_stack.pop_front();
        }
        self.undo_stack.push_back(state);
        // Clear redo stack when a new action is performed
        self.redo_stack.clear();
    }

    fn undo(&mut self) -> Option<HistoryState> {
        self.undo_stack.pop_back()
    }

    fn redo(&mut self) -> Option<HistoryState> {
        self.redo_stack.pop_back()
    }

    fn push_redo(&mut self, state: HistoryState) {
        if self.redo_stack.len() >= self.max_size {
            self.redo_stack.pop_front();
        }
        self.redo_stack.push_back(state);
    }

    fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }
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
    pub notification: Option<(String, f32)>, // (message, temps restant)
    pub last_loaded_file: Option<String>,
    pub show_layer_config: bool,
    pub selection: SelectionData,  // Pour les outils de sélection
    history: History,  // Historique pour Undo/Redo
    pub clipboard: Option<SelectionContent>,  // Presse-papier pour copier/coller
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
            notification: None,
            last_loaded_file: None,
            show_layer_config: false,
            selection: SelectionData { start: None, end: None, is_active: false },
            history: History::new(50),  // Max 50 étapes d'historique
            clipboard: None,
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

        // Gestion des raccourcis clavier
        ui.input(|i| {
            // Ctrl+Z : Annuler
            if i.modifiers.command && i.key_pressed(egui::Key::Z) {
                self.undo();
            }
            // Ctrl+Y : Refaire
            if i.modifiers.command && i.key_pressed(egui::Key::Y) {
                self.redo();
            }
            // Ctrl+C : Copier la sélection
            if i.modifiers.command && i.key_pressed(egui::Key::C) {
                self.copy_selection();
            }
            // Ctrl+V : Coller (sera géré au clic)
            // Delete : Supprimer la sélection
            if i.key_pressed(egui::Key::Delete) {
                self.delete_selection();
            }
            // Escape : Annuler la sélection en cours
            if i.key_pressed(egui::Key::Escape) {
                self.selection.start = None;
                self.selection.end = None;
                self.selection.is_active = false;
            }
        });

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

        // Dessiner la sélection active
        if self.selection.is_active {
            if let (Some(start), Some(end)) = (self.selection.start, self.selection.end) {
                let min_x = start.0.min(end.0);
                let max_x = start.0.max(end.0);
                let min_y = start.1.min(end.1);
                let max_y = start.1.max(end.1);
                
                for tx in min_x..=max_x {
                    for ty in min_y..=max_y {
                        let tile_rect = Rect::from_min_size(
                            canvas_center
                                + self.offset
                                + Vec2::new(tx as f32 * tile_size, ty as f32 * tile_size),
                            Vec2::new(tile_size, tile_size),
                        );
                        painter.rect_stroke(tile_rect, 0.0, (2.0, Color32::from_rgb(255, 255, 0)));
                    }
                }
                
                // Bordure externe de la sélection
                let selection_rect = Rect::from_min_size(
                    canvas_center
                        + self.offset
                        + Vec2::new(min_x as f32 * tile_size, min_y as f32 * tile_size),
                    Vec2::new((max_x - min_x + 1) as f32 * tile_size, (max_y - min_y + 1) as f32 * tile_size),
                );
                painter.rect_stroke(selection_rect, 0.0, (3.0, Color32::from_rgb(255, 255, 0)));
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
                    Tool::LineFill => Color32::from_rgba_unmultiplied(0, 255, 255, 100),
                    Tool::RectFill => Color32::from_rgba_unmultiplied(255, 165, 0, 100),
                };

                painter.rect_stroke(preview_rect, 0.0, (2.0, preview_color));
                
                // Prévisualisation de la sélection pour LineFill et RectFill
                if self.current_tool == Tool::LineFill || self.current_tool == Tool::RectFill {
                    if let Some(start) = self.selection.start {
                        let (start_x, start_y) = start;
                        
                        // Calculer la zone de prévisualisation
                        let (min_x, max_x, min_y, max_y) = if self.current_tool == Tool::LineFill {
                            // Pour LineFill, choisir ligne ou colonne selon la distance
                            let dx = (tile_x - start_x).abs();
                            let dy = (tile_y - start_y).abs();
                            if dx > dy {
                                // Ligne horizontale
                                (start_x.min(tile_x), start_x.max(tile_x), start_y, start_y)
                            } else {
                                // Ligne verticale
                                (start_x, start_x, start_y.min(tile_y), start_y.max(tile_y))
                            }
                        } else {
                            // RectFill : rectangle complet
                            (start_x.min(tile_x), start_x.max(tile_x), 
                             start_y.min(tile_y), start_y.max(tile_y))
                        };
                        
                        // Dessiner la zone de sélection
                        for tx in min_x..=max_x {
                            for ty in min_y..=max_y {
                                let tile_rect = Rect::from_min_size(
                                    canvas_center
                                        + self.offset
                                        + Vec2::new(tx as f32 * tile_size, ty as f32 * tile_size),
                                    Vec2::new(tile_size, tile_size),
                                );
                                painter.rect_filled(tile_rect, 0.0, preview_color);
                            }
                        }
                    }
                }

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
                
                // Gestion du mode Sélection
                if self.current_tool == Tool::Select {
                    // Vérifier si on veut coller avec Ctrl+V
                    let want_paste = ui.input(|i| i.modifiers.command && i.key_pressed(egui::Key::V));
                    
                    if want_paste && self.clipboard.is_some() {
                        // Coller à la position de la souris
                        self.paste_selection(tile_x, tile_y);
                    } else if response.clicked_by(egui::PointerButton::Primary) {
                        if !self.selection.is_active {
                            // Premier clic : démarrer la sélection
                            self.selection.start = Some((tile_x, tile_y));
                            self.selection.end = Some((tile_x, tile_y));
                            self.selection.is_active = false;
                        } else {
                            // Clic en dehors : créer nouvelle sélection
                            self.selection.start = Some((tile_x, tile_y));
                            self.selection.end = Some((tile_x, tile_y));
                            self.selection.is_active = false;
                        }
                    } else if response.dragged() && self.selection.start.is_some() {
                        // Mise à jour en temps réel pendant le drag
                        self.selection.end = Some((tile_x, tile_y));
                    } else if response.drag_stopped() && self.selection.start.is_some() {
                        // Fin du drag : activer la sélection
                        self.selection.end = Some((tile_x, tile_y));
                        self.selection.is_active = true;
                        
                        if let (Some(start), Some(end)) = (self.selection.start, self.selection.end) {
                            let count = ((start.0 - end.0).abs() + 1) * ((start.1 - end.1).abs() + 1);
                            self.show_notification(format!("📦 {} tiles sélectionnés", count));
                        }
                    } else if response.clicked_by(egui::PointerButton::Secondary) {
                        // Clic droit : annuler la sélection
                        self.selection.start = None;
                        self.selection.end = None;
                        self.selection.is_active = false;
                    }
                } else if self.current_tool == Tool::LineFill || self.current_tool == Tool::RectFill {
                    // Gestion des outils de remplissage LineFill et RectFill
                    if response.clicked_by(egui::PointerButton::Primary) {
                        if self.selection.start.is_none() {
                            // Premier clic : définir le point de départ
                            self.selection.start = Some((tile_x, tile_y));
                        } else {
                            // Deuxième clic : remplir la zone
                            if let Some(start) = self.selection.start {
                                self.save_history();  // Sauvegarder avant modification
                                let (start_x, start_y) = start;
                                
                                // Calculer la zone à remplir
                                let (min_x, max_x, min_y, max_y) = if self.current_tool == Tool::LineFill {
                                    let dx = (tile_x - start_x).abs();
                                    let dy = (tile_y - start_y).abs();
                                    if dx > dy {
                                        (start_x.min(tile_x), start_x.max(tile_x), start_y, start_y)
                                    } else {
                                        (start_x, start_x, start_y.min(tile_y), start_y.max(tile_y))
                                    }
                                } else {
                                    (start_x.min(tile_x), start_x.max(tile_x), 
                                     start_y.min(tile_y), start_y.max(tile_y))
                                };
                                
                                // Remplir la zone
                                if let Some(layer) = self.level.layers.get_mut(self.current_layer) {
                                    let tile_data = match self.paint_mode {
                                        PaintMode::ColorTile(rgb) => TileData::Color(rgb),
                                        PaintMode::TextureTile { tileset_id, tile_index } => {
                                            TileData::Texture { tileset_id, tile_index }
                                        }
                                    };
                                    
                                    for tx in min_x..=max_x {
                                        for ty in min_y..=max_y {
                                            layer.set_tile(tx, ty, tile_data);
                                        }
                                    }
                                    
                                    let count = ((max_x - min_x + 1) * (max_y - min_y + 1)) as usize;
                                    let tool_name = if self.current_tool == Tool::LineFill { "ligne" } else { "rectangle" };
                                    self.show_notification(format!("✅ {} tiles remplis en {}", count, tool_name));
                                }
                                
                                // Réinitialiser la sélection
                                self.selection.start = None;
                                self.selection.end = None;
                            }
                        }
                    } else if response.clicked_by(egui::PointerButton::Secondary) {
                        // Clic droit : annuler la sélection
                        self.selection.start = None;
                        self.selection.end = None;
                    }
                } else if primary_click || secondary_click {
                    // Comportement normal pour les autres outils
                    if self.last_painted != Some((tile_x, tile_y)) {
                        // Sauvegarder l'historique au premier clic
                        if response.clicked() && self.last_painted.is_none() {
                            self.save_history();
                        }
                        
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

    pub fn show_notification(&mut self, message: String) {
        self.notification = Some((message, 3.0)); // 3 secondes
    }

    pub fn update_notification(&mut self, delta_time: f32) {
        if let Some((_, ref mut time)) = self.notification {
            *time -= delta_time;
            if *time <= 0.0 {
                self.notification = None;
            }
        }
    }

    /// Sauvegarde l'état actuel dans l'historique
    pub fn save_history(&mut self) {
        let state = HistoryState {
            layers: self.level.layers.clone(),
            current_layer: self.current_layer,
        };
        self.history.push(state);
    }

    /// Annule la dernière action (Ctrl+Z)
    pub fn undo(&mut self) {
        if let Some(previous_state) = self.history.undo() {
            // Sauvegarder l'état actuel dans la pile redo avant de le remplacer
            let current_state = HistoryState {
                layers: self.level.layers.clone(),
                current_layer: self.current_layer,
            };
            self.history.push_redo(current_state);
            
            // Restaurer l'état précédent
            self.level.layers = previous_state.layers;
            self.current_layer = previous_state.current_layer;
            self.show_notification("↶ Annulation".to_string());
        } else {
            self.show_notification("❌ Rien à annuler".to_string());
        }
    }

    /// Refait la dernière action annulée (Ctrl+Y)
    pub fn redo(&mut self) {
        if let Some(next_state) = self.history.redo() {
            // Sauvegarder l'état actuel dans la pile undo
            let current_state = HistoryState {
                layers: self.level.layers.clone(),
                current_layer: self.current_layer,
            };
            self.history.push(current_state);
            
            // Restaurer l'état suivant
            self.level.layers = next_state.layers;
            self.current_layer = next_state.current_layer;
            self.show_notification("↷ Rétablir".to_string());
        } else {
            self.show_notification("❌ Rien à rétablir".to_string());
        }
    }

    /// Copie la sélection dans le presse-papier (Ctrl+C)
    pub fn copy_selection(&mut self) {
        if self.selection.is_active {
            if let (Some(start), Some(end)) = (self.selection.start, self.selection.end) {
                let min_x = start.0.min(end.0);
                let max_x = start.0.max(end.0);
                let min_y = start.1.min(end.1);
                let max_y = start.1.max(end.1);
                
                let mut tiles = Vec::new();
                if let Some(layer) = self.level.layers.get(self.current_layer) {
                    for tx in min_x..=max_x {
                        for ty in min_y..=max_y {
                            if let Some(&tile_data) = layer.tiles.get(&(tx, ty)) {
                                if !tile_data.is_empty() {
                                    tiles.push(((tx - min_x, ty - min_y), tile_data));
                                }
                            }
                        }
                    }
                }
                
                self.clipboard = Some(SelectionContent {
                    tiles,
                    origin: (min_x, min_y),
                    width: max_x - min_x + 1,
                    height: max_y - min_y + 1,
                });
                
                let count = (max_x - min_x + 1) * (max_y - min_y + 1);
                self.show_notification(format!("📋 {} tiles copiés", count));
            }
        } else {
            self.show_notification("❌ Aucune sélection active".to_string());
        }
    }

    /// Colle le contenu du presse-papier (Ctrl+V)
    pub fn paste_selection(&mut self, paste_x: i32, paste_y: i32) {
        // Clone le clipboard pour éviter le conflit de borrowing
        if let Some(clipboard) = self.clipboard.clone() {
            self.save_history();
            
            if let Some(layer) = self.level.layers.get_mut(self.current_layer) {
                for ((rel_x, rel_y), tile_data) in &clipboard.tiles {
                    let abs_x = paste_x + rel_x;
                    let abs_y = paste_y + rel_y;
                    layer.set_tile(abs_x, abs_y, *tile_data);
                }
            }
            
            let count = clipboard.tiles.len();
            self.show_notification(format!("✅ {} tiles collés", count));
        } else {
            self.show_notification("❌ Presse-papier vide".to_string());
        }
    }

    /// Supprime les tiles dans la sélection (Delete)
    pub fn delete_selection(&mut self) {
        if self.selection.is_active {
            if let (Some(start), Some(end)) = (self.selection.start, self.selection.end) {
                self.save_history();
                
                let min_x = start.0.min(end.0);
                let max_x = start.0.max(end.0);
                let min_y = start.1.min(end.1);
                let max_y = start.1.max(end.1);
                
                if let Some(layer) = self.level.layers.get_mut(self.current_layer) {
                    let mut count = 0;
                    for tx in min_x..=max_x {
                        for ty in min_y..=max_y {
                            layer.set_tile(tx, ty, TileData::empty());
                            count += 1;
                        }
                    }
                    self.show_notification(format!("🗑️ {} tiles supprimés", count));
                }
                
                // Désactiver la sélection
                self.selection.is_active = false;
                self.selection.start = None;
                self.selection.end = None;
            }
        } else {
            self.show_notification("❌ Aucune sélection active".to_string());
        }
    }

    pub fn can_undo(&self) -> bool {
        self.history.can_undo()
    }

    pub fn can_redo(&self) -> bool {
        self.history.can_redo()
    }
}

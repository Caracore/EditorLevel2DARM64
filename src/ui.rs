use crate::editor::{EditorState, PaintMode, Tool};
use eframe::egui;

/// Convertit une couleur RGB en string hexadécimale
fn rgb_to_hex(rgb: [u8; 3]) -> String {
    format!("#{:02X}{:02X}{:02X}", rgb[0], rgb[1], rgb[2])
}

/// Parse une string hexadécimale en RGB
fn parse_hex_color(hex: &str) -> Option<[u8; 3]> {
    let hex = hex.trim().trim_start_matches('#');
    
    if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some([r, g, b])
    } else if hex.len() == 3 {
        // Format court #RGB -> #RRGGBB
        let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
        let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
        let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
        Some([r, g, b])
    } else {
        None
    }
}

/// Met à jour le champ hexadécimal depuis RGB
fn update_hex_from_rgb(state: &mut EditorState) {
    state.color_hex_input = rgb_to_hex(state.selected_color);
}

pub fn draw_top_panel(ctx: &egui::Context, state: &mut EditorState) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("Fichier", |ui| {
                if ui.button("📁 Nouveau").clicked() {
                    state.level =
                        crate::level::Level::new("Nouveau Niveau".to_string(), 64, 48, 16);
                    state.asset_manager = crate::asset_manager::AssetManager::new();
                    state.last_loaded_file = None;
                    ui.close_menu();
                }

                ui.separator();
                ui.label("💾 Sauvegarder");

                if ui.button("  📦 Projet Complet (.editorproj)").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("Projet Éditeur", &["editorproj"])
                        .set_file_name("mon_niveau.editorproj")
                        .save_file()
                    {
                        let path_str = path.to_str().unwrap();
                        let mut project = crate::project::Project::new(state.level.clone());
                        project.tilesets = state.asset_manager.get_metadata();
                        
                        match project.save_to_file(path_str) {
                            Ok(_) => {
                                state.show_notification(format!("✅ Projet sauvegardé : {} (avec {} tilesets)", 
                                    path.file_name().unwrap().to_str().unwrap(),
                                    project.tilesets.len()));
                            }
                            Err(e) => {
                                state.show_notification(format!("❌ Erreur : {}", e));
                                eprintln!("Erreur de sauvegarde: {}", e);
                            }
                        }
                    }
                    ui.close_menu();
                }

                if ui.button("  📄 Niveau seul (.json)").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("JSON", &["json"])
                        .set_file_name("niveau.json")
                        .save_file()
                    {
                        let path_str = path.to_str().unwrap();
                        match state.level.save_to_file(path_str) {
                            Ok(_) => {
                                state.show_notification(format!("✅ Niveau sauvegardé : {}", 
                                    path.file_name().unwrap().to_str().unwrap()));
                            }
                            Err(e) => {
                                state.show_notification(format!("❌ Erreur : {}", e));
                                eprintln!("Erreur de sauvegarde: {}", e);
                            }
                        }
                    }
                    ui.close_menu();
                }

                ui.separator();
                ui.label("📂 Charger");

                if ui.button("  📦 Projet Complet (.editorproj)").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("Projet Éditeur", &["editorproj"])
                        .pick_file()
                    {
                        let path_str = path.to_str().unwrap();
                        match crate::project::Project::load_from_file(path_str) {
                            Ok(project) => {
                                let filename = path.file_name().unwrap().to_str().unwrap();
                                state.level = project.level;
                                
                                // Recharger tous les tilesets
                                state.asset_manager = crate::asset_manager::AssetManager::new();
                                let mut loaded_count = 0;
                                let mut failed = Vec::new();
                                
                                for tileset_meta in &project.tilesets {
                                    let tileset_path = std::path::PathBuf::from(&tileset_meta.path);
                                    match state.asset_manager.load_tileset(
                                        ctx,
                                        tileset_path.clone(),
                                        tileset_meta.tile_width,
                                        tileset_meta.tile_height,
                                    ) {
                                        Ok(_) => loaded_count += 1,
                                        Err(e) => {
                                            failed.push(format!("{}: {}", tileset_meta.name, e));
                                        }
                                    }
                                }
                                
                                state.last_loaded_file = Some(filename.to_string());
                                state.current_layer = 1.min(state.level.layers.len() - 1);
                                state.zoom = 1.0;
                                state.offset = egui::Vec2::ZERO;
                                
                                if failed.is_empty() {
                                    state.show_notification(format!("✅ Projet chargé : {} ({} calques, {} tilesets)", 
                                        filename,
                                        state.level.layers.len(),
                                        loaded_count));
                                } else {
                                    state.show_notification(format!("⚠️ Projet chargé : {} ({}/{} tilesets)", 
                                        filename, loaded_count, project.tilesets.len()));
                                    for error in &failed {
                                        eprintln!("⚠️ Tileset non chargé: {}", error);
                                    }
                                }
                            }
                            Err(e) => {
                                state.show_notification(format!("❌ Erreur de chargement : {}", e));
                                eprintln!("Erreur de chargement: {}", e);
                            }
                        }
                    }
                    ui.close_menu();
                }

                if ui.button("  📄 Niveau seul (.json)").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("JSON", &["json"])
                        .pick_file()
                    {
                        let path_str = path.to_str().unwrap();
                        match crate::level::Level::load_from_file(path_str) {
                            Ok(level) => {
                                let filename = path.file_name().unwrap().to_str().unwrap();
                                state.level = level;
                                state.last_loaded_file = Some(filename.to_string());
                                state.current_layer = 1.min(state.level.layers.len() - 1);
                                state.zoom = 1.0;
                                state.offset = egui::Vec2::ZERO;
                                state.show_notification(format!("✅ Niveau chargé : {} ({} calques, {} tiles)\n⚠️ Tilesets non chargés (utilisez .editorproj)", 
                                    filename,
                                    state.level.layers.len(),
                                    state.level.layers.iter().map(|l| l.tiles.len()).sum::<usize>()));
                            }
                            Err(e) => {
                                state.show_notification(format!("❌ Erreur de chargement : {}", e));
                                eprintln!("Erreur de chargement: {}", e);
                            }
                        }
                    }
                    ui.close_menu();
                }

                ui.separator();

                if ui.button("❌ Quitter").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });

            ui.menu_button("Édition", |ui| {
                if ui.button("🗑️ Effacer le calque actuel").clicked() {
                    if let Some(layer) = state.level.layers.get_mut(state.current_layer) {
                        layer.clear();
                    }
                    ui.close_menu();
                }

                ui.separator();

                if ui.button("➕ Ajouter un calque").clicked() {
                    let layer_count = state.level.layers.len();
                    state.level.add_layer(format!("Layer {}", layer_count + 1));
                    state.current_layer = layer_count;
                    ui.close_menu();
                }

                if ui.button("➖ Supprimer le calque actuel").clicked() {
                    if state.level.remove_layer(state.current_layer) {
                        if state.current_layer >= state.level.layers.len() {
                            state.current_layer = state.level.layers.len().saturating_sub(1);
                        }
                    }
                    ui.close_menu();
                }
                
                ui.separator();
                
                if ui.button("⚙️ Configuration des calques...").clicked() {
                    state.show_layer_config = true;
                    ui.close_menu();
                }
            });

            ui.menu_button("Affichage", |ui| {
                ui.checkbox(&mut state.show_grid, "Afficher la grille");

                ui.separator();
                ui.label(format!("Zoom: {:.0}%", state.zoom * 100.0));
                if ui.button("Réinitialiser zoom").clicked() {
                    state.zoom = 1.0;
                    state.offset = egui::Vec2::ZERO;
                    ui.close_menu();
                }
                
                ui.separator();
                ui.label("⚙️ Configuration du Canvas");
                
                ui.horizontal(|ui| {
                    ui.label("Largeur:");
                    ui.add(egui::DragValue::new(&mut state.level.width)
                        .speed(1.0)
                        .clamp_range(10..=1000));
                });
                
                ui.horizontal(|ui| {
                    ui.label("Hauteur:");
                    ui.add(egui::DragValue::new(&mut state.level.height)
                        .speed(1.0)
                        .clamp_range(10..=1000));
                });
                
                if ui.button("🔲 Preset Petit (32x24)").clicked() {
                    state.level.width = 32;
                    state.level.height = 24;
                }
                
                if ui.button("🔳 Preset Moyen (64x48)").clicked() {
                    state.level.width = 64;
                    state.level.height = 48;
                }
                
                if ui.button("🔴 Preset Grand (128x96)").clicked() {
                    state.level.width = 128;
                    state.level.height = 96;
                }
                
                if ui.button("♾️ Preset Énorme (256x256)").clicked() {
                    state.level.width = 256;
                    state.level.height = 256;
                }
            });
        });
    });
}

pub fn draw_side_panel(ctx: &egui::Context, state: &mut EditorState) {
    egui::SidePanel::left("left_panel")
        .default_width(250.0)
        .show(ctx, |ui| {
            ui.heading("🛠️ Outils");
            ui.separator();

            ui.horizontal(|ui| {
                if ui
                    .selectable_label(state.current_tool == Tool::Paint, "✏️ Pinceau")
                    .clicked()
                {
                    state.current_tool = Tool::Paint;
                    state.selection.start = None;
                    state.selection.end = None;
                }
                if ui
                    .selectable_label(state.current_tool == Tool::Erase, "🧹 Gomme")
                    .clicked()
                {
                    state.current_tool = Tool::Erase;
                    state.selection.start = None;
                    state.selection.end = None;
                }
                if ui
                    .selectable_label(state.current_tool == Tool::Select, "📦 Sélection")
                    .clicked()
                {
                    state.current_tool = Tool::Select;
                    state.selection.start = None;
                    state.selection.end = None;
                }
            });
            
            ui.horizontal(|ui| {
                if ui
                    .selectable_label(state.current_tool == Tool::LineFill, "📏 Ligne")
                    .on_hover_text("Remplir en ligne (horizontal ou vertical)")
                    .clicked()
                {
                    state.current_tool = Tool::LineFill;
                    state.selection.start = None;
                    state.selection.end = None;
                }
                if ui
                    .selectable_label(state.current_tool == Tool::RectFill, "⬛ Carré")
                    .on_hover_text("Remplir en rectangle")
                    .clicked()
                {
                    state.current_tool = Tool::RectFill;
                    state.selection.start = None;
                    state.selection.end = None;
                }
            });
            
            // Afficher l'instruction pour les outils de sélection
            if state.current_tool == Tool::LineFill || state.current_tool == Tool::RectFill {
                ui.add_space(5.0);
                if state.selection.start.is_none() {
                    ui.label("👉 Cliquez pour le point de départ");
                } else {
                    ui.label("👉 Cliquez pour le point d'arrivée");
                    ui.label("   (clic droit pour annuler)");
                }
            }
            
            // Instructions pour le mode Sélection
            if state.current_tool == Tool::Select {
                ui.add_space(5.0);
                ui.label("📦 Mode Sélection");
                ui.label("• Glissez pour sélectionner");
                ui.label("• Ctrl+C : Copier");
                ui.label("• Ctrl+V : Coller");
                ui.label("• Delete : Supprimer");
                ui.label("• Echap : Annuler");
                
                if state.selection.is_active {
                    ui.label("✅ Sélection active");
                }
                if state.clipboard.is_some() {
                    ui.label("📋 Presse-papier plein");
                }
            }
            
            ui.add_space(10.0);
            
            // Raccourcis globaux
            ui.heading("⌨️ Raccourcis");
            ui.separator();
            ui.label("Ctrl+Z : Annuler");
            ui.label("Ctrl+Y : Rétablir");
            
            // Indicateurs d'état
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                let undo_color = if state.can_undo() {
                    egui::Color32::GREEN
                } else {
                    egui::Color32::GRAY
                };
                let redo_color = if state.can_redo() {
                    egui::Color32::GREEN
                } else {
                    egui::Color32::GRAY
                };
                ui.colored_label(undo_color, "↶");
                ui.colored_label(redo_color, "↷");
            });

            ui.add_space(10.0);
            
            // Section Tilesets
            ui.heading("🖼️ Tilesets");
            ui.separator();
            
            if ui.button("➕ Charger Tileset").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Images", &["png", "jpg", "jpeg"])
                    .pick_file()
                {
                    // Demander la taille des tiles (16x16 par défaut pour RPG 2D)
                    match state.asset_manager.load_tileset(ctx, path, 16, 16) {
                        Ok(id) => {
                            state.selected_tileset = Some(id);
                        }
                        Err(e) => eprintln!("Erreur de chargement du tileset: {}", e),
                    }
                }
            }
            
            ui.add_space(5.0);
            
            // Afficher les tilesets chargés
            let tilesets = state.asset_manager.get_all_tilesets();
            if !tilesets.is_empty() {
                egui::ScrollArea::vertical()
                    .id_source("tilesets_list_scroll")
                    .max_height(200.0)
                    .show(ui, |ui| {
                        for (idx, tileset) in tilesets.iter().enumerate() {
                            let is_selected = state.selected_tileset == Some(idx);
                            
                            ui.horizontal(|ui| {
                                if ui.selectable_label(is_selected, &tileset.name).clicked() {
                                    state.selected_tileset = Some(idx);
                                }
                                
                                ui.label(format!("({}x{})", tileset.columns, tileset.rows));
                            });
                            
                            // Si ce tileset est sélectionné, afficher la grille de tiles
                            if is_selected {
                                ui.indent("tileset_tiles", |ui| {
                                    let tile_count = tileset.columns * tileset.rows;
                                    let tiles_per_row = 4;
                                    let tile_display_size = 48.0;
                                    
                                    egui::ScrollArea::vertical()
                                        .id_source(format!("tileset_tiles_scroll_{}", idx))
                                        .max_height(300.0)
                                        .show(ui, |ui| {
                                            for row in 0..(tile_count + tiles_per_row - 1) / tiles_per_row {
                                                ui.horizontal(|ui| {
                                                    for col in 0..tiles_per_row {
                                                        let tile_index = row * tiles_per_row + col;
                                                        if tile_index >= tile_count {
                                                            break;
                                                        }
                                                        
                                                        let tile_x = tile_index % tileset.columns;
                                                        let tile_y = tile_index / tileset.columns;
                                                        
                                                        let uv = egui::Rect::from_min_max(
                                                            egui::pos2(
                                                                tile_x as f32 / tileset.columns as f32,
                                                                tile_y as f32 / tileset.rows as f32,
                                                            ),
                                                            egui::pos2(
                                                                (tile_x + 1) as f32 / tileset.columns as f32,
                                                                (tile_y + 1) as f32 / tileset.rows as f32,
                                                            ),
                                                        );
                                                        
                                                        let is_selected_tile = matches!(
                                                            state.paint_mode,
                                                            PaintMode::TextureTile { tileset_id, tile_index: ti }
                                                            if tileset_id == idx && ti == tile_index
                                                        );
                                                        
                                                        let image_button = egui::ImageButton::new(
                                                            egui::load::SizedTexture::new(
                                                                tileset.texture.id(),
                                                                egui::vec2(tile_display_size, tile_display_size),
                                                            )
                                                        ).uv(uv);
                                                        
                                                        let response = ui.add(image_button);
                                                        
                                                        if is_selected_tile {
                                                            ui.painter().rect_stroke(
                                                                response.rect,
                                                                2.0,
                                                                (2.0, egui::Color32::YELLOW),
                                                            );
                                                        }
                                                        
                                                        if response.clicked() {
                                                            state.paint_mode = PaintMode::TextureTile {
                                                                tileset_id: idx,
                                                                tile_index,
                                                            };
                                                            state.current_tool = Tool::Paint;
                                                        }
                                                    }
                                                });
                                            }
                                        });
                                });
                            }
                        }
                    });
            }

            ui.add_space(10.0);
            ui.heading("🎨 Sélecteur de Couleur");
            ui.separator();

            // Affichage de la couleur actuelle
            ui.horizontal(|ui| {
                ui.label("Couleur actuelle:");
                let color_preview_size = egui::vec2(40.0, 40.0);
                let (rect, _response) = ui.allocate_exact_size(color_preview_size, egui::Sense::hover());
                ui.painter().rect_filled(
                    rect,
                    4.0,
                    egui::Color32::from_rgb(state.selected_color[0], state.selected_color[1], state.selected_color[2]),
                );
                ui.painter().rect_stroke(rect, 4.0, (2.0, egui::Color32::WHITE));
            });
            
            ui.add_space(5.0);
            
            // Sliders RGB
            ui.label("Rouge:");
            if ui.add(egui::Slider::new(&mut state.selected_color[0], 0..=255)).changed() {
                update_hex_from_rgb(state);
            }
            
            ui.label("Vert:");
            if ui.add(egui::Slider::new(&mut state.selected_color[1], 0..=255)).changed() {
                update_hex_from_rgb(state);
            }
            
            ui.label("Bleu:");
            if ui.add(egui::Slider::new(&mut state.selected_color[2], 0..=255)).changed() {
                update_hex_from_rgb(state);
            }
            
            ui.add_space(5.0);
            
            // Input hexadécimal
            ui.horizontal(|ui| {
                ui.label("Code HEX:");
                let text_edit = egui::TextEdit::singleline(&mut state.color_hex_input)
                    .desired_width(80.0);
                if ui.add(text_edit).lost_focus() {
                    if let Some(rgb) = parse_hex_color(&state.color_hex_input) {
                        state.selected_color = rgb;
                    } else {
                        // Restaurer la valeur correcte si invalide
                        update_hex_from_rgb(state);
                    }
                }
            });
            
            ui.add_space(5.0);
            
            // Bouton pour appliquer la couleur
            if ui.button("✏️ Utiliser cette couleur").clicked() {
                state.paint_mode = PaintMode::ColorTile(state.selected_color);
                state.current_tool = Tool::Paint;
            }
            
            ui.add_space(10.0);
            
            // Palette de couleurs prédéfinies
            ui.label("Couleurs prédéfinies:");
            egui::ScrollArea::vertical()
                .id_source("preset_colors_scroll")
                .max_height(200.0)
                .show(ui, |ui| {
                let preset_colors = vec![
                    ("Marron (Sol)", [139, 69, 19]),
                    ("Gris (Mur)", [128, 128, 128]),
                    ("Beige (Plateforme)", [205, 133, 63]),
                    ("Rouge (Piège)", [255, 0, 0]),
                    ("Jaune (Pièce)", [255, 215, 0]),
                    ("Vert (Départ)", [0, 255, 0]),
                    ("Bleu ciel (Sortie)", [0, 191, 255]),
                    ("Noir", [0, 0, 0]),
                    ("Blanc", [255, 255, 255]),
                    ("Rose", [255, 192, 203]),
                    ("Violet", [128, 0, 128]),
                    ("Orange", [255, 165, 0]),
                ];
                
                for (name, color) in preset_colors {
                    ui.horizontal(|ui| {
                        if ui.button(name).clicked() {
                            state.selected_color = color;
                            update_hex_from_rgb(state);
                            state.paint_mode = PaintMode::ColorTile(color);
                            state.current_tool = Tool::Paint;
                        }
                        
                        // Aperçu de la couleur
                        let rect = egui::Rect::from_min_size(
                            ui.cursor().left_top(),
                            egui::vec2(25.0, ui.spacing().interact_size.y),
                        );
                        ui.painter().rect_filled(
                            rect,
                            2.0,
                            egui::Color32::from_rgb(color[0], color[1], color[2]),
                        );
                        ui.allocate_space(egui::vec2(30.0, 0.0));
                    });
                }
            });
        });
}

pub fn draw_bottom_panel(ctx: &egui::Context, state: &mut EditorState) {
    egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label(format!("Niveau: {}", state.level.name));
            
            // Afficher le fichier chargé si disponible
            if let Some(ref filename) = state.last_loaded_file {
                ui.separator();
                ui.label(format!("📂 {}", filename));
            }
            
            ui.separator();
            ui.label(format!(
                "Taille: {}x{} (Tile: {}px)",
                state.level.width, state.level.height, state.level.tile_size
            ));
            ui.separator();
            ui.label(format!("Zoom: {:.0}%", state.zoom * 100.0));
            ui.separator();
            ui.label("⇌: Zoom | 🖋️: Déplacer | Clic droit: Gomme");
        });
    });
}

pub fn draw_central_panel(ctx: &egui::Context, state: &mut EditorState) {
    // Mettre à jour les notifications
    state.update_notification(ctx.input(|i| i.stable_dt));
    
    // Fenêtre de configuration des calques
    if state.show_layer_config {
        egui::Window::new("⚙️ Configuration des Calques")
            .collapsible(false)
            .resizable(true)
            .default_width(400.0)
            .show(ctx, |ui| {
                ui.heading("Gestion des calques");
                ui.separator();
                
                ui.label(format!("Nombre de calques actuels : {}", state.level.layers.len()));
                ui.add_space(10.0);
                
                // Liste des calques avec renommage
                egui::ScrollArea::vertical()
                    .id_source("layer_config_scroll")
                    .max_height(200.0)
                    .show(ui, |ui| {
                        for (idx, layer) in state.level.layers.iter_mut().enumerate() {
                            ui.horizontal(|ui| {
                                ui.label(format!("{}.", idx + 1));
                                
                                let mut name = layer.name.clone();
                                if ui.text_edit_singleline(&mut name).changed() {
                                    layer.name = name;
                                }
                                
                                ui.checkbox(&mut layer.visible, "👁");
                                
                                ui.label(format!("({} tiles)", layer.tiles.len()));
                            });
                        }
                    });
                
                ui.add_space(10.0);
                ui.separator();
                ui.heading("Actions rapides");
                
                ui.horizontal(|ui| {
                    if ui.button("➕ Ajouter calque").clicked() {
                        let count = state.level.layers.len();
                        state.level.add_layer(format!("Layer {}", count + 1));
                    }
                    
                    if ui.button("🗑️ Tout effacer").clicked() {
                        for layer in &mut state.level.layers {
                            layer.clear();
                        }
                    }
                });
                
                ui.add_space(10.0);
                ui.separator();
                ui.heading("Presets de calques");
                
                if ui.button("🎨 Setup RPG Standard (5 calques)").clicked() {
                    state.level.layers.clear();
                    state.level.add_layer("Fond lointain".to_string());
                    state.level.add_layer("Arrière-plan".to_string());
                    state.level.add_layer("Gameplay".to_string());
                    state.level.add_layer("Décorations".to_string());
                    state.level.add_layer("Premier plan".to_string());
                    state.current_layer = 2; // Gameplay par défaut
                }
                
                if ui.button("🏗️ Setup Parallax (7 calques)").clicked() {
                    state.level.layers.clear();
                    state.level.add_layer("Ciel".to_string());
                    state.level.add_layer("Montagnes".to_string());
                    state.level.add_layer("Arbres lointains".to_string());
                    state.level.add_layer("Terrain".to_string());
                    state.level.add_layer("Objets".to_string());
                    state.level.add_layer("Arbres proches".to_string());
                    state.level.add_layer("UI/Overlay".to_string());
                    state.current_layer = 3; // Terrain par défaut
                }
                
                if ui.button("🎮 Setup Minimal (3 calques)").clicked() {
                    state.level.layers.clear();
                    state.level.add_layer("Background".to_string());
                    state.level.add_layer("Main".to_string());
                    state.level.add_layer("Foreground".to_string());
                    state.current_layer = 1;
                }
                
                ui.add_space(10.0);
                ui.separator();
                
                if ui.button("✅ Fermer").clicked() {
                    state.show_layer_config = false;
                }
            });
    }
    
    // Afficher la notification si elle existe
    if let Some((ref message, time)) = state.notification {
        egui::Window::new("📢 Notification")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_TOP, [0.0, 10.0])
            .show(ctx, |ui| {
                ui.label(message);
                ui.add(egui::ProgressBar::new(time / 3.0).show_percentage());
            });
    }
    
    egui::CentralPanel::default().show(ctx, |ui| {
        // Panneau des calques en haut avec contrôles
        ui.horizontal(|ui| {
            ui.label("📑 Calques:");
            
            let current_layer = state.current_layer;
            let layer_count = state.level.layers.len();
            
            // Variables pour stocker les actions à effectuer après l'itération
            let mut new_current_layer = None;
            let mut move_up = None;
            let mut move_down = None;
            let mut toggle_visibility = Vec::new();
            
            for (idx, layer) in state.level.layers.iter().enumerate() {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        let is_current = idx == current_layer;
                        
                        if ui
                            .selectable_label(is_current, &layer.name)
                            .on_hover_text("Cliquer pour sélectionner ce calque")
                            .clicked()
                        {
                            new_current_layer = Some(idx);
                        }

                        let mut visible = layer.visible;
                        if ui.checkbox(&mut visible, "👁")
                            .on_hover_text("Visibilité du calque")
                            .changed()
                        {
                            toggle_visibility.push((idx, visible));
                        }
                        
                        // Boutons de réorganisation
                        if is_current {
                            if ui.small_button("⬆").on_hover_text("Déplacer vers le haut").clicked() {
                                move_up = Some(idx);
                            }
                            
                            if ui.small_button("⬇").on_hover_text("Déplacer vers le bas").clicked() {
                                move_down = Some(idx);
                            }
                        }
                    });
                });
            }
            
            // Bouton pour ajouter un calque rapidement
            if ui.button("➕").on_hover_text("Ajouter un nouveau calque").clicked() {
                state.level.add_layer(format!("Layer {}", layer_count + 1));
                state.current_layer = layer_count;
            }
            
            // Appliquer les actions après l'itération
            if let Some(idx) = new_current_layer {
                state.current_layer = idx;
            }
            
            for (idx, visible) in toggle_visibility {
                if let Some(layer) = state.level.layers.get_mut(idx) {
                    layer.visible = visible;
                }
            }
            
            if let Some(idx) = move_up {
                if state.level.move_layer_up(idx) {
                    state.current_layer = idx.saturating_sub(1);
                }
            }
            
            if let Some(idx) = move_down {
                if state.level.move_layer_down(idx) {
                    state.current_layer = (idx + 1).min(state.level.layers.len() - 1);
                }
            }
        });

        ui.separator();

        // Zone de dessin
        state.draw_canvas(ui);
    });
}

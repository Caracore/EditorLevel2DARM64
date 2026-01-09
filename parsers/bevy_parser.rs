//! EditorLevel2D Parser pour Bevy
//! 
//! Module Rust pour charger et afficher les niveaux créés avec EditorLevel2D
//! dans vos jeux Bevy.
//! 
//! # Installation
//! 
//! Ajoutez dans votre `Cargo.toml`:
//! ```toml
//! [dependencies]
//! bevy = "0.14"
//! serde = { version = "1.0", features = ["derive"] }
//! serde_json = "1.0"
//! ```
//! 
//! # Utilisation
//! 
//! ```rust
//! use bevy::prelude::*;
//! use parsers::bevy_parser::*;
//! 
//! fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
//!     // Charger le niveau
//!     commands.spawn(EditorLevelBundle::from_file(
//!         "mon_niveau.editorproj",
//!         &asset_server,
//!     ));
//! }
//! ```

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Référence vers un tile (couleur ou texture)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TileData {
    Color([u8; 3]),
    Texture { tileset_id: usize, tile_index: u32 },
}

/// Représente un calque du niveau
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    pub name: String,
    pub visible: bool,
    #[serde(
        serialize_with = "serialize_tiles",
        deserialize_with = "deserialize_tiles"
    )]
    pub tiles: HashMap<(i32, i32), TileData>,
}

/// Structure du niveau
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Level {
    pub name: String,
    pub width: i32,
    pub height: i32,
    pub tile_size: i32,
    pub layers: Vec<Layer>,
}

/// Information sur un tileset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TilesetInfo {
    pub name: String,
    pub path: String,
    pub tiles_per_row: u32,
    pub total_tiles: u32,
}

/// Projet complet avec niveau et tilesets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorProject {
    pub level: Level,
    #[serde(default)]
    pub tilesets: HashMap<String, TilesetInfo>,
}

// Fonctions de sérialisation pour HashMap<(i32, i32), TileData>
fn serialize_tiles<S>(
    tiles: &HashMap<(i32, i32), TileData>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::ser::SerializeMap;
    let mut map = serializer.serialize_map(Some(tiles.len()))?;
    for ((x, y), tile) in tiles {
        let key = format!("{},{}", x, y);
        map.serialize_entry(&key, tile)?;
    }
    map.end()
}

fn deserialize_tiles<'de, D>(
    deserializer: D,
) -> Result<HashMap<(i32, i32), TileData>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let string_map: HashMap<String, TileData> = HashMap::deserialize(deserializer)?;
    let mut tiles = HashMap::new();
    
    for (key, value) in string_map {
        let parts: Vec<&str> = key.split(',').collect();
        if parts.len() == 2 {
            if let (Ok(x), Ok(y)) = (parts[0].parse(), parts[1].parse()) {
                tiles.insert((x, y), value);
            }
        }
    }
    
    Ok(tiles)
}

/// Component pour identifier un niveau chargé
#[derive(Component)]
pub struct EditorLevel {
    pub project: EditorProject,
}

/// Component pour identifier un tile du niveau
#[derive(Component)]
pub struct LevelTile {
    pub layer_index: usize,
    pub position: (i32, i32),
}

/// Bundle pour spawner un niveau complet
#[derive(Bundle)]
pub struct EditorLevelBundle {
    pub level: EditorLevel,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

impl EditorLevelBundle {
    /// Charge un niveau depuis un fichier .editorproj ou .json
    pub fn from_file(path: impl AsRef<Path>, asset_server: &AssetServer) -> Self {
        let project = EditorProject::load_from_file(path).expect("Impossible de charger le niveau");
        
        Self {
            level: EditorLevel { project },
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
        }
    }
}

impl EditorProject {
    /// Charge un projet depuis un fichier
    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let project: EditorProject = serde_json::from_str(&content)?;
        Ok(project)
    }
    
    /// Retourne tous les tiles d'un calque
    pub fn get_layer_tiles(&self, layer_name: &str) -> Option<&HashMap<(i32, i32), TileData>> {
        self.level.layers
            .iter()
            .find(|l| l.name == layer_name)
            .map(|l| &l.tiles)
    }
    
    /// Trouve toutes les positions des tiles d'une couleur spécifique
    pub fn find_tiles_by_color(&self, color: [u8; 3]) -> Vec<(i32, i32)> {
        let mut positions = Vec::new();
        
        for layer in &self.level.layers {
            for (&pos, &tile_data) in &layer.tiles {
                if let TileData::Color(tile_color) = tile_data {
                    if tile_color == color {
                        positions.push(pos);
                    }
                }
            }
        }
        
        positions
    }
}

/// Système pour spawner les tiles du niveau
pub fn spawn_level_tiles(
    mut commands: Commands,
    query: Query<(Entity, &EditorLevel), Added<EditorLevel>>,
    asset_server: Res<AssetServer>,
) {
    for (entity, editor_level) in query.iter() {
        let project = &editor_level.project;
        let level = &project.level;
        let tile_size = level.tile_size as f32;
        
        // Charger les textures des tilesets
        let mut tileset_handles: HashMap<String, Handle<Image>> = HashMap::new();
        let project_dir = PathBuf::from("assets"); // Adapter selon votre structure
        
        for (tileset_id, tileset_info) in &project.tilesets {
            let texture_path = project_dir.join(&tileset_info.path);
            let handle = asset_server.load(texture_path.to_str().unwrap());
            tileset_handles.insert(tileset_id.clone(), handle);
        }
        
        // Spawner les tiles de chaque calque
        for (layer_index, layer) in level.layers.iter().enumerate() {
            if !layer.visible {
                continue;
            }
            
            for (&(x, y), &tile_data) in &layer.tiles {
                let world_x = x as f32 * tile_size;
                let world_y = -(y as f32 * tile_size); // Y inversé pour Bevy
                
                match tile_data {
                    TileData::Color(color) => {
                        // Spawner un sprite coloré
                        commands.entity(entity).with_children(|parent| {
                            parent.spawn((
                                SpriteBundle {
                                    sprite: Sprite {
                                        color: Color::srgb_u8(color[0], color[1], color[2]),
                                        custom_size: Some(Vec2::new(tile_size, tile_size)),
                                        ..default()
                                    },
                                    transform: Transform::from_xyz(world_x, world_y, layer_index as f32),
                                    ..default()
                                },
                                LevelTile {
                                    layer_index,
                                    position: (x, y),
                                },
                            ));
                        });
                    }
                    TileData::Texture { tileset_id, tile_index } => {
                        // Spawner un sprite avec texture
                        if let Some(texture_handle) = tileset_handles.get(&tileset_id.to_string()) {
                            let tileset_info = &project.tilesets[&tileset_id.to_string()];
                            let tiles_per_row = tileset_info.tiles_per_row;
                            
                            // Calculer les coordonnées UV
                            let tile_x = (tile_index % tiles_per_row) as f32 * 16.0;
                            let tile_y = (tile_index / tiles_per_row) as f32 * 16.0;
                            
                            commands.entity(entity).with_children(|parent| {
                                parent.spawn((
                                    SpriteBundle {
                                        texture: texture_handle.clone(),
                                        sprite: Sprite {
                                            custom_size: Some(Vec2::new(tile_size, tile_size)),
                                            rect: Some(Rect::new(tile_x, tile_y, tile_x + 16.0, tile_y + 16.0)),
                                            ..default()
                                        },
                                        transform: Transform::from_xyz(world_x, world_y, layer_index as f32),
                                        ..default()
                                    },
                                    LevelTile {
                                        layer_index,
                                        position: (x, y),
                                    },
                                ));
                            });
                        }
                    }
                }
            }
        }
    }
}

/// Plugin pour faciliter l'intégration dans Bevy
pub struct EditorLevelPlugin;

impl Plugin for EditorLevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_level_tiles);
    }
}

// Exemple d'utilisation complet
#[cfg(test)]
mod example {
    use super::*;

    fn main() {
        App::new()
            .add_plugins(DefaultPlugins)
            .add_plugins(EditorLevelPlugin)
            .add_systems(Startup, setup)
            .run();
    }

    fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        // Caméra
        commands.spawn(Camera2dBundle::default());
        
        // Charger le niveau
        commands.spawn(EditorLevelBundle::from_file(
            "assets/levels/mon_niveau.editorproj",
            &asset_server,
        ));
    }
}

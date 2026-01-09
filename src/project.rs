use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Métadonnées d'un tileset dans le projet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TilesetMetadata {
    pub id: usize,
    pub name: String,
    pub path: String,  // Chemin relatif ou absolu
    pub tile_width: u32,
    pub tile_height: u32,
    pub columns: u32,
    pub rows: u32,
}

/// Projet complet avec niveau et tilesets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub version: String,
    pub level: crate::level::Level,
    pub tilesets: Vec<TilesetMetadata>,
}

impl Project {
    pub fn new(level: crate::level::Level) -> Self {
        Self {
            version: "1.0".to_string(),
            level,
            tilesets: Vec::new(),
        }
    }

    pub fn add_tileset(&mut self, metadata: TilesetMetadata) {
        self.tilesets.push(metadata);
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(path)?;
        let project = serde_json::from_str(&json)?;
        Ok(project)
    }

    /// Sauvegarde en format ancien (seulement le niveau, pour compatibilité)
    pub fn save_level_only(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.level.save_to_file(path)
    }
}

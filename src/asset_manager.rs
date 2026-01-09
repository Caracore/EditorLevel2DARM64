use egui::{ColorImage, Context, TextureHandle, TextureOptions};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Représente un tileset avec son image et ses métadonnées
#[derive(Clone)]
pub struct Tileset {
    pub name: String,
    pub path: PathBuf,
    pub texture: TextureHandle,
    pub tile_width: u32,
    pub tile_height: u32,
    pub columns: u32,
    pub rows: u32,
    pub image: ColorImage,
}

/// Référence vers un tile dans un tileset
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TileRef {
    pub tileset_id: usize,
    pub tile_index: u32,
}

/// Gestionnaire d'assets pour les tilesets
pub struct AssetManager {
    tilesets: Vec<Tileset>,
    tileset_counter: usize,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            tilesets: Vec::new(),
            tileset_counter: 0,
        }
    }

    /// Charge un nouveau tileset depuis un fichier image
    pub fn load_tileset(
        &mut self,
        ctx: &Context,
        path: PathBuf,
        tile_width: u32,
        tile_height: u32,
    ) -> Result<usize, String> {
        // Charger l'image
        let img = image::open(&path).map_err(|e| format!("Erreur de chargement: {}", e))?;

        let size = [img.width() as usize, img.height() as usize];
        let img_buffer = img.to_rgba8();
        let pixels = img_buffer.as_flat_samples();

        let color_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());

        // Créer la texture
        let texture = ctx.load_texture(
            format!("tileset_{}", self.tileset_counter),
            color_image.clone(),
            TextureOptions::NEAREST,
        );

        let columns = img.width() / tile_width;
        let rows = img.height() / tile_height;

        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown")
            .to_string();

        let tileset = Tileset {
            name,
            path,
            texture,
            tile_width,
            tile_height,
            columns,
            rows,
            image: color_image,
        };

        let id = self.tileset_counter;
        self.tilesets.push(tileset);
        self.tileset_counter += 1;

        Ok(id)
    }

    /// Récupère un tileset par son ID
    pub fn get_tileset(&self, id: usize) -> Option<&Tileset> {
        self.tilesets.get(id)
    }

    /// Récupère tous les tilesets
    pub fn get_all_tilesets(&self) -> &[Tileset] {
        &self.tilesets
    }

    /// Supprime un tileset
    pub fn remove_tileset(&mut self, id: usize) {
        if id < self.tilesets.len() {
            self.tilesets.remove(id);
        }
    }

    /// Calcule les coordonnées UV d'un tile dans un tileset
    pub fn get_tile_uv(&self, tile_ref: TileRef) -> Option<(f32, f32, f32, f32)> {
        let tileset = self.get_tileset(tile_ref.tileset_id)?;

        let tile_x = (tile_ref.tile_index % tileset.columns) as f32;
        let tile_y = (tile_ref.tile_index / tileset.columns) as f32;

        let u1 = tile_x / tileset.columns as f32;
        let v1 = tile_y / tileset.rows as f32;
        let u2 = (tile_x + 1.0) / tileset.columns as f32;
        let v2 = (tile_y + 1.0) / tileset.rows as f32;

        Some((u1, v1, u2, v2))
    }

    /// Retourne le nombre total de tiles dans un tileset
    pub fn get_tileset_tile_count(&self, tileset_id: usize) -> Option<u32> {
        let tileset = self.get_tileset(tileset_id)?;
        Some(tileset.columns * tileset.rows)
    }
}

impl Default for AssetManager {
    fn default() -> Self {
        Self::new()
    }
}

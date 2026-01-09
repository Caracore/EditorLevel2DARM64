use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Référence vers un tile (peut être une couleur ou une texture)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TileData {
    Color([u8; 3]), // RGB direct
    Texture { tileset_id: usize, tile_index: u32 },
}

impl TileData {
    pub fn empty() -> Self {
        TileData::Color([40, 40, 40])
    }
    
    pub fn is_empty(&self) -> bool {
        matches!(self, TileData::Color([40, 40, 40]))
    }
}

/// Représente un type de tile prédéfini (pour compatibilité)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TileType {
    Empty,
    Ground,
    Wall,
    Platform,
    Spike,
    Coin,
    Spawn,
    Exit,
}

impl TileType {
    pub fn color(&self) -> [u8; 3] {
        match self {
            TileType::Empty => [40, 40, 40],
            TileType::Ground => [139, 69, 19],
            TileType::Wall => [128, 128, 128],
            TileType::Platform => [205, 133, 63],
            TileType::Spike => [255, 0, 0],
            TileType::Coin => [255, 215, 0],
            TileType::Spawn => [0, 255, 0],
            TileType::Exit => [0, 191, 255],
        }
    }

    pub fn name(&self) -> &str {
        match self {
            TileType::Empty => "Vide",
            TileType::Ground => "Sol",
            TileType::Wall => "Mur",
            TileType::Platform => "Plateforme",
            TileType::Spike => "Piège",
            TileType::Coin => "Pièce",
            TileType::Spawn => "Départ",
            TileType::Exit => "Sortie",
        }
    }

    pub fn all() -> Vec<TileType> {
        vec![
            TileType::Empty,
            TileType::Ground,
            TileType::Wall,
            TileType::Platform,
            TileType::Spike,
            TileType::Coin,
            TileType::Spawn,
            TileType::Exit,
        ]
    }
}

/// Représente une couche (layer) du niveau
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    pub name: String,
    pub visible: bool,
    // Utiliser Vec pour la sérialisation JSON (HashMap ne supporte pas les tuples comme clés)
    #[serde(serialize_with = "serialize_tiles", deserialize_with = "deserialize_tiles")]
    pub tiles: HashMap<(i32, i32), TileData>,
}

// Fonctions de sérialisation personnalisées pour HashMap<(i32, i32), TileData>
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
    use serde::de::{MapAccess, Visitor};
    use std::fmt;

    struct TilesVisitor;

    impl<'de> Visitor<'de> for TilesVisitor {
        type Value = HashMap<(i32, i32), TileData>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map with string keys in 'x,y' format")
        }

        fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            let mut map = HashMap::new();
            while let Some((key, value)) = access.next_entry::<String, TileData>()? {
                let parts: Vec<&str> = key.split(',').collect();
                if parts.len() == 2 {
                    if let (Ok(x), Ok(y)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                        map.insert((x, y), value);
                    }
                }
            }
            Ok(map)
        }
    }

    deserializer.deserialize_map(TilesVisitor)
}

impl Layer {
    pub fn new(name: String) -> Self {
        Self {
            name,
            visible: true,
            tiles: HashMap::new(),
        }
    }

    pub fn set_tile(&mut self, x: i32, y: i32, tile_data: TileData) {
        if tile_data.is_empty() {
            self.tiles.remove(&(x, y));
        } else {
            self.tiles.insert((x, y), tile_data);
        }
    }

    pub fn get_tile(&self, x: i32, y: i32) -> TileData {
        self.tiles
            .get(&(x, y))
            .copied()
            .unwrap_or(TileData::empty())
    }

    pub fn clear(&mut self) {
        self.tiles.clear();
    }
}

/// Représente un niveau complet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Level {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub tile_size: u32,
    pub layers: Vec<Layer>,
}

impl Level {
    pub fn new(name: String, width: u32, height: u32, tile_size: u32) -> Self {
        let mut level = Self {
            name,
            width,
            height,
            tile_size,
            layers: Vec::new(),
        };
        level.layers.push(Layer::new("Background".to_string()));
        level.layers.push(Layer::new("Main".to_string()));
        level.layers.push(Layer::new("Foreground".to_string()));
        level
    }

    pub fn add_layer(&mut self, name: String) {
        self.layers.push(Layer::new(name));
    }

    pub fn remove_layer(&mut self, index: usize) -> bool {
        if self.layers.len() > 1 && index < self.layers.len() {
            self.layers.remove(index);
            true
        } else {
            false
        }
    }

    pub fn move_layer_up(&mut self, index: usize) -> bool {
        if index > 0 && index < self.layers.len() {
            self.layers.swap(index, index - 1);
            true
        } else {
            false
        }
    }

    pub fn move_layer_down(&mut self, index: usize) -> bool {
        if index < self.layers.len() - 1 {
            self.layers.swap(index, index + 1);
            true
        } else {
            false
        }
    }

    pub fn rename_layer(&mut self, index: usize, new_name: String) -> bool {
        if index < self.layers.len() {
            self.layers[index].name = new_name;
            true
        } else {
            false
        }
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(path)?;
        let level = serde_json::from_str(&json)?;
        Ok(level)
    }
}

# Éditeur de Niveaux RPG 2D - ARM64

Un éditeur de niveaux professionnel pour créer des maps de jeux RPG 2D, compatible ARM64, avec support complet des tilesets et des images.

## 🎮 Fonctionnalités

### 🖼️ Gestion des Tilesets (NOUVEAU!)
- **Charger des images** : Importez vos propres tilesets (PNG, JPG, JPEG)
- **Découpage automatique** : Les tilesets sont automatiquement découpés en tiles de 16x16 pixels
- **Sélection visuelle** : Choisissez les tiles directement dans une grille visuelle interactive
- **Prévisualisation** : Voir le tile sélectionné en temps réel avant de le placer
- **Support multi-tilesets** : Chargez plusieurs tilesets simultanément

### 🎨 Sélecteur de Couleur Personnalisé (NOUVEAU!)
- **Sliders RGB** : Ajustez Rouge, Vert, Bleu avec des sliders (0-255)
- **Code hexadécimal** : Entrez directement un code couleur HTML (#RRGGBB ou #RGB)
- **Palette prédéfinie** : 12+ couleurs courantes pour démarrer rapidement
- **Aperçu en temps réel** : Visualisez la couleur avant de l'appliquer
- **Format JSON propre** : Les couleurs sont sauvegardées en RGB [R, G, B]

### Interface graphique intuitive

- **Système de calques avancé** :
  - Calques multiples (Background, Main, Foreground par défaut)
  - Ajouter/Supprimer des calques dynamiquement
  - Réorganiser les calques avec ⬆⬇
  - Contrôle de visibilité individuel (👁)
  - Design en profondeur pour des maps de qualité professionnelle
  
- **Outils d'édition**:
  - ✏️ Pinceau pour placer des tiles (couleurs ou textures)
  - 🧹 Gomme pour effacer
  - 🖱️ Clic droit = gomme rapide
  - 📦 Sélection (à venir)
  
- **Types de tiles couleur** (compatibilité):
  - Sol, Mur, Plateforme
  - Pièges, Pièces
  - Point de départ et sortie
  
- **Navigation**:
  - Molette : Zoom (20% à 500%)
  - Clic molette + glisser : Déplacer la vue
  - Grille activable/désactivable

- **Sauvegarde/Chargement** en JSON

## 🚀 Compilation

```bash
# Build
cargo build --release

# Exécution
cargo run --release
```

## 🎨 Utilisation

### Barre de menu
- **Fichier** : Nouveau, Sauvegarder, Charger, Quitter
- **Édition** : 
  - Effacer le calque actuel
  - ➕ Ajouter un calque
  - ➖ Supprimer le calque actuel
- **Affichage** : Grille, Zoom

### Panneau latéral
- Section **🖼️ Tilesets** :
  - Bouton "➕ Charger Tileset" pour importer des images
  - Grille visuelle pour sélectionner les tiles
  - Support multi-tilesets avec sélection par clic
- Section **🎨 Sélecteur de Couleur** :
  - Aperçu de la couleur actuelle
  - Sliders RGB pour ajustement précis
  - Champ texte pour codes hexadécimaux (#RRGGBB)
  - Palette de 12 couleurs prédéfinies avec aperçu
  - Bouton "✏️ Utiliser cette couleur" pour appliquer

### Zone centrale (Canvas)
- **Gestion des calques** :
  - Cliquez sur un calque pour le sélectionner
  - Utilisez 👁 pour afficher/masquer
  - Boutons ⬆⬇ pour réorganiser l'ordre (calques supérieurs = premier plan)
  - Bouton ➕ pour ajouter rapidement un calque
- **Zone de dessin interactive** :
  - Clic gauche : Peindre avec le tile sélectionné
  - Clic droit : Gomme rapide
  - Molette : Zoom
  - Clic molette + glisser : Déplacer la vue

### Raccourcis
- **Clic gauche** : Peindre (ou gomme si outil Gomme actif)
- **Clic droit** : Gomme (ou peindre si outil Gomme actif)
- **Clic molette + glisser** : Déplacer la vue
- **Molette** : Zoom in/out

## 📁 Format de fichier

Les niveaux sont sauvegardés en JSON avec cette structure :

```json
{
  "name": "Mon Niveau",
  "width": 64,
  "height": 48,
  "tile_size": 16,
  "layers": [
    {
      "name": "Background",
      "visible": true,
      "tiles": {
        "0,0": {"Color": [139, 69, 19]},
        "1,0": {"Texture": {"tileset_id": 0, "tile_index": 5}},
        "2,0": {"Color": [255, 0, 0]}
      }
    },
    {
      "name": "Main",
      "visible": true,
      "tiles": {}
    }
  ]
}
```

**Format des clés** : Les positions sont au format `"x,y"` (string)
**Format des couleurs** : RGB en tableau `[R, G, B]` (0-255)

## 🔧 Architecture

```
src/
├── main.rs       # Point d'entrée et boucle principale
├── level.rs      # Structures de données (Level, Layer, TileType)
├── editor.rs     # État de l'éditeur et logique du canvas
└── ui.rs         # Interface utilisateur (panneaux, menus)
```

## 📦 Dépendances

- **eframe/egui** : Framework d'interface graphique
- **serde/serde_json** : Sérialisation des niveaux
- **rfd** : Dialogues de fichiers natifs

## 🎯 Utilisation dans un jeu

Le format JSON est **100% compatible** avec les moteurs de jeux populaires comme **Pygame**, **Bevy**, **Godot**, etc.

### Intégration Pygame

```python
import json

# Charger le niveau
with open("niveau.json", "r") as f:
    level = json.load(f)

# Parcourir les tiles
for layer in level["layers"]:
    if not layer["visible"]:
        continue
    
    for pos_str, tile_data in layer["tiles"].items():
        # Convertir "x,y" en tuple
        x, y = map(int, pos_str.split(','))
        
        # Tile avec couleur RGB
        if "Color" in tile_data:
            rgb = tile_data["Color"]  # [R, G, B]
            color = (rgb[0], rgb[1], rgb[2])
            # Dessiner un rectangle avec pygame
            pygame.draw.rect(screen, color, (x * 16, y * 16, 16, 16))
        
        # Tile avec texture
        elif "Texture" in tile_data:
            tileset_id = tile_data["Texture"]["tileset_id"]
            tile_index = tile_data["Texture"]["tile_index"]
            # Calculer la position dans le tileset
            tiles_per_row = tileset_width // 16  # 16 = tile_size
            tile_x = (tile_index % tiles_per_row) * 16
            tile_y = (tile_index // tiles_per_row) * 16
            # Dessiner depuis le tileset...
            screen.blit(tileset_image, 
                       (x * 16, y * 16), 
                       (tile_x, tile_y, 16, 16))
```

### Intégration Bevy (Rust)

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Utiliser les mêmes structures
#[derive(Deserialize)]
struct Level {
    name: String,
    width: u32,
    height: u32,
    tile_size: u32,
    layers: Vec<Layer>,
}

#[derive(Deserialize)]
struct Layer {
    name: String,
    visible: bool,
    tiles: HashMap<(i32, i32), TileData>,
}

#[derive(Deserialize)]
enum TileData {
    Color(TileType),
    Texture { tileset_id: usize, tile_index: u32 },
}

// Charger
let level_json = std::fs::read_to_string("niveau.json")?;
let level: Level = serde_json::from_str(&level_json)?;

// Utiliser avec Bevy
fn spawn_tiles(
    mut commands: Commands,
    level: Res<Level>,
    tilesets: Res<Tilesets>,
) {
    for layer in &level.layers {
        if !layer.visible { continue; }
        
        for (&(x, y), tile_data) in &layer.tiles {
            match tile_data {
                TileData::Texture { tileset_id, tile_index } => {
                    let tileset = &tilesets[*tileset_id];
                    // Spawner l'entité avec le sprite du tileset
                    commands.spawn(SpriteSheetBundle {
                        texture: tileset.texture.clone(),
                        atlas: TextureAtlas { index: *tile_index, .. },
                        transform: Transform::from_xyz(
                            x as f32 * level.tile_size as f32,
                            y as f32 * level.tile_size as f32,
                            0.0
                        ),
                        ..default()
                    });
                }
                _ => {}
            }
        }
    }
}
```

### Format des tilesets

Les tilesets sont référencés par ID dans le JSON. Dans votre jeu :
1. Chargez les mêmes images de tilesets
2. Utilisez le même découpage (16x16 par défaut)
3. Les `tile_index` correspondent à l'ordre : ligne par ligne, de gauche à droite

## 🚧 Améliorations futures

- [ ] Outil de sélection et copier-coller
- [ ] Configuration personnalisée de la taille des tiles via UI
- [ ] Support des animations de tiles
- [ ] Entités personnalisables (spawn points, NPCs, objets)
- [ ] Undo/Redo
- [ ] Minimap
- [ ] Export vers différents formats (Tiled TMX, Godot TileMap)
- [ ] Support des propriétés personnalisées
- [ ] Gestion de plusieurs niveaux dans un projet
- [ ] Drag & drop de fichiers tilesets
- [ ] Pipette pour copier une couleur existante sur la map

## 💡 Conseils pour vos tilesets

- **Taille recommandée** : 16x16 pixels (RPG classique) ou 32x32 (RPG HD)
- **Format** : PNG avec transparence pour les objets/personnages
- **Organisation** : Grille régulière sans espacement entre tiles
- **Ressources gratuites** : OpenGameArt.org, itch.io, Kenney.nl

## 🎨 Conseils pour les couleurs

- Utilisez **Coolors.co** ou **Adobe Color** pour créer des palettes harmonieuses
- Copiez les codes hex (#RRGGBB) directement dans l'éditeur
- Créez des dégradés en variant légèrement les valeurs RGB
- Gardez une palette cohérente pour chaque niveau (5-10 couleurs principales)

## 📝 Licence

MIT

## 🤝 Contribution

Les contributions sont les bienvenues ! N'hésitez pas à ouvrir une issue ou une pull request.

# Éditeur de Niveaux RPG 2D - ARM64

Un éditeur de niveaux professionnel pour créer des maps de jeux RPG 2D, compatible ARM64, avec support complet des tilesets et des images.

## 🎮 Fonctionnalités

### � Système de Projet Complet (NOUVEAU! ⭐)
- **Format `.editorproj`** : Sauvegarde le niveau ET les tilesets ensemble
- **Rechargement automatique** : Les tilesets se chargent automatiquement avec le niveau
- **Workflow professionnel** : Travaillez sur vos projets sans recharger manuellement
- **Compatibilité .json** : Les anciens fichiers JSON restent supportés
- **Voir le guide** : [GUIDE_PROJETS.md](GUIDE_PROJETS.md) pour tous les détails

### 🖼️ Gestion des Tilesets
- **Charger des images** : Importez vos propres tilesets (PNG, JPG, JPEG)
- **Découpage automatique** : Les tilesets sont automatiquement découpés en tiles de 16x16 pixels
- **Sélection visuelle** : Choisissez les tiles directement dans une grille visuelle interactive
- **Prévisualisation** : Voir le tile sélectionné en temps réel avant de le placer
- **Support multi-tilesets** : Chargez plusieurs tilesets simultanément
- **Persistance** : Les tilesets sont sauvegardés dans les fichiers `.editorproj`

### 🎨 Sélecteur de Couleur Personnalisé
- **Sliders RGB** : Ajustez Rouge, Vert, Bleu avec des sliders (0-255)
- **Code hexadécimal** : Entrez directement un code couleur HTML (#RRGGBB ou #RGB)
- **Palette prédéfinie** : 12+ couleurs courantes pour démarrer rapidement
- **Aperçu en temps réel** : Visualisez la couleur avant de l'appliquer
- **Format JSON propre** : Les couleurs sont sauvegardées en RGB [R, G, B]

### Interface graphique intuitive

- **Système de calques ultra-flexible** :
  - Nombre de calques ajustable (de 1 à autant que nécessaire)
  - Configuration dédiée : Menu Édition → ⚙️ Configuration des calques
  - Renommage en direct de chaque calque
  - Ajouter/Supprimer des calques dynamiquement
  - Réorganiser les calques avec ⬆⬇
  - Contrôle de visibilité individuel (👁)
  - Presets intégrés : Minimal (3), Standard (5), Parallax (7)
  - Design en profondeur pour des maps de qualité professionnelle

- **Configuration du canvas** :
  - Taille ajustable : 10x10 à 1000x1000 tiles
  - Presets : Petit (32x24), Moyen (64x48), Grand (128x96), Énorme (256x256)
  - Canvas quasi-infini possible pour mondes ouverts
  - Menu Affichage → ⚙️ Configuration du Canvas
  
- **Outils d'édition**:
  - ✏️ Pinceau pour placer des tiles (couleurs ou textures)
  - 🧹 Gomme pour effacer
  - � **Remplissage en Ligne** (NOUVEAU! ⭐) : remplir rapidement en horizontal/vertical
  - ⬛ **Remplissage en Rectangle** (NOUVEAU! ⭐) : remplir des zones rectangulaires
  - 📦 **Sélection** (NOUVEAU! ⭐) : copier, coller, supprimer des zones
  - 🖱️ Clic droit = gomme rapide
  
- **Système Undo/Redo** (NOUVEAU! ⭐):
  - ↶ Ctrl+Z : Annuler (jusqu'à 50 actions)
  - ↷ Ctrl+Y : Rétablir
  - Historique intelligent par session
  - Indicateurs visuels de disponibilité
  
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

**💡 Installation facile sur Raspberry Pi :**
```bash
./install.sh  # Installe tout automatiquement!
```

## 🎨 Utilisation

### 🔄 Workflow : Créer, Sauvegarder, Rééditer

#### Créer un nouveau niveau
1. **Fichier** → **📁 Nouveau** : Crée un niveau vierge (64x48, tiles 16px)
2. Dessinez votre map avec couleurs et/ou tilesets
3. Utilisez plusieurs calques pour la profondeur

#### Sauvegarder votre travail
1. **Fichier** → **💾 Sauvegarder**
2. Choisissez un nom (ex: `mon_niveau.json`)
3. ✅ Notification : "Niveau sauvegardé : mon_niveau.json"
4. Le fichier est prêt à être utilisé dans votre jeu !

#### Charger et rééditer une map existante

**💡 RECOMMANDÉ : Utilisez le format `.editorproj` !**

##### Option 1 : Projet Complet (.editorproj) ⭐
1. **Fichier** → **📂 Charger** → **📦 Projet Complet (.editorproj)**
2. Sélectionnez votre fichier `.editorproj`
3. ✅ **Tout se charge automatiquement** : niveau + tilesets + couleurs
4. Continuez l'édition immédiatement
5. Sauvegardez avec **💾 Sauvegarder** → **📦 Projet Complet (.editorproj)**

##### Option 2 : Niveau seul (.json)
1. **Fichier** → **📂 Charger** → **📄 Niveau seul (.json)**
2. Sélectionnez votre fichier JSON
3. ⚠️ Les couleurs se chargent, mais **pas les tilesets**
4. Rechargez manuellement vos tilesets via **Assets → Charger Tileset**
5. Pour éviter cela à l'avenir, sauvegardez en `.editorproj`

**Note :** Le nom du fichier chargé s'affiche dans la barre du bas 📂

📖 **Guide complet** : [GUIDE_PROJETS.md](GUIDE_PROJETS.md)
📖 **Détails techniques** : [GUIDE_CHARGER.md](GUIDE_CHARGER.md)

### 🖌️ Outils de Remplissage (NOUVEAU!)

#### 📏 Remplissage en Ligne
Remplissez rapidement des lignes horizontales ou verticales :
1. Sélectionnez l'outil **📏 Ligne**
2. Choisissez une couleur ou un tileset
3. **Clic 1** : Point de départ
4. **Clic 2** : Point d'arrivée
5. La ligne se remplit automatiquement !
   - Direction horizontale si vous bougez plus à gauche/droite
   - Direction verticale si vous bougez plus en haut/bas

**💡 Astuce** : Idéal pour créer des bordures, murs, ou chemins !

#### ⬛ Remplissage en Rectangle
Remplissez rapidement des zones rectangulaires entières :
1. Sélectionnez l'outil **⬛ Carré**
2. Choisissez une couleur ou un tileset
3. **Clic 1** : Un coin du rectangle
4. **Clic 2** : Le coin opposé
5. Tout le rectangle est rempli !

**💡 Astuce** : Parfait pour les sols, plateformes, et grandes zones uniformes !

#### Prévisualisation et Annulation
- **Prévisualisation** : Bougez la souris après le 1er clic pour voir la zone
  - Cyan pour l'outil Ligne
  - Orange pour l'outil Rectangle
- **Annuler** : Clic droit pour annuler la sélection en cours
- **Notification** : "✅ X tiles remplis" après chaque remplissage

📖 **Guide détaillé** : [GUIDE_REMPLISSAGE.md](GUIDE_REMPLISSAGE.md)

### 🔄 Undo/Redo et Sélection (NOUVEAU!)

#### Système d'Annulation
Faites des erreurs sans crainte ! Le système conserve les 50 dernières actions :
- **Ctrl+Z** : Annuler la dernière action
- **Ctrl+Y** : Rétablir une action annulée
- Indicateurs visuels ↶↷ dans le panneau latéral

#### Mode Sélection 📦
Gérez efficacement de grandes zones :
1. Sélectionnez l'outil **📦 Sélection**
2. Glissez pour sélectionner une zone (contour jaune)
3. **Ctrl+C** : Copier la sélection
4. **Ctrl+V** : Coller à la position de la souris
5. **Delete** : Supprimer la zone sélectionnée
6. **Échap** : Annuler la sélection

**💡 Cas d'usage** :
- Dupliquer des éléments répétitifs (arbres, bâtiments)
- Déplacer des zones complètes
- Créer des motifs répétitifs rapidement
- Nettoyer de grandes zones

📖 **Guide complet** : [GUIDE_UNDO_SELECTION.md](GUIDE_UNDO_SELECTION.md)

### Barre de menu
- **Fichier** : 
  - 📁 Nouveau : Créer un niveau vierge
  - 💾 Sauvegarder : 
    - **📦 Projet Complet (.editorproj)** ← Recommandé
    - 📄 Niveau seul (.json)
  - 📂 Charger : 
    - **📦 Projet Complet (.editorproj)** ← Recommandé
    - 📄 Niveau seul (.json)
  - ❌ Quitter
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

## 📁 Formats de fichier

### Format `.editorproj` (Projet Complet) ⭐

Le format **recommandé** pour travailler. Il contient le niveau ET les références aux tilesets :

```json
{
  "version": "1.0",
  "level": {
    "name": "Mon Niveau",
    "width": 64,
    "height": 48,
    "tile_size": 16,
    "layers": [
      {
        "name": "Background",
        "visible": true,
        "opacity": 1.0,
        "tiles": {
          "0,0": {"Color": [139, 69, 19]},
          "1,0": {"Texture": {"tileset_id": 0, "tile_index": 5}},
          "2,0": {"Color": [255, 0, 0]}
        }
      }
    ]
  },
  "tilesets": [
    {
      "id": 0,
      "name": "tileset_dungeon.png",
      "path": "/chemin/vers/tileset_dungeon.png",
      "tile_width": 16,
      "tile_height": 16,
      "columns": 16,
      "rows": 16
    }
  ]
}
```

**✅ Avantages :**
- Rechargement automatique des tilesets
- Un seul fichier pour tout le projet
- Format JSON lisible et éditable
- Parfait pour le développement

### Format `.json` (Niveau seul)

Format pour l'export vers votre moteur de jeu :

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
        "1,0": {"Texture": {"tileset_id": 0, "tile_index": 5}}
      }
    }
  ]
}
```

**Format des clés** : Les positions sont au format `"x,y"` (string)  
**Format des couleurs** : RGB en tableau `[R, G, B]` (0-255)  
**Textures** : Référencent un tileset par ID + index de tile

📖 **Guide détaillé** : [GUIDE_PROJETS.md](GUIDE_PROJETS.md)

## 🔧 Architecture

```
src/
├── main.rs          # Point d'entrée et boucle principale
├── level.rs         # Structures de données (Level, Layer, TileData)
├── editor.rs        # État de l'éditeur et logique du canvas
├── asset_manager.rs # Gestion des tilesets et textures
├── project.rs       # Format .editorproj avec métadonnées
└── ui.rs            # Interface utilisateur (panneaux, menus)
```

## 📦 Dépendances

- **eframe/egui** : Framework d'interface graphique
- **serde/serde_json** : Sérialisation des niveaux
- **rfd** : Dialogues de fichiers natifs

## 🛠️ Installation sur Raspberry Pi OS ARM64

Des scripts d'installation et de désinstallation sont fournis pour faciliter l'installation sur Raspberry Pi :

```bash
# Installation
./install.sh

# Lancer l'éditeur
editor_level

# Désinstallation
./uninstall.sh
```

Le script d'installation s'occupe de :
- Installer Rust si nécessaire
- Installer toutes les dépendances système
- Compiler le programme
- Créer un raccourci dans le menu
- Configurer le PATH

📖 **Plus de détails** : Les scripts incluent des messages détaillés à chaque étape.

## 🎯 Utilisation dans un jeu

Le format JSON est **100% compatible** avec les moteurs de jeux populaires comme **Pygame**, **Bevy**, **Godot**, etc.

### 🚀 Parsers Officiels Disponibles

Des parsers prêts à l'emploi sont fournis dans le dossier [`parsers/`](parsers/) :

#### 🐍 Pygame (Python)
```python
from parsers.pygame_parser import EditorLevelLoader

loader = EditorLevelLoader("mon_niveau.editorproj")
loader.render(screen, camera_x=0, camera_y=0)
```

#### 🦀 Bevy (Rust)
```rust
use parsers::bevy_parser::*;

commands.spawn(EditorLevelBundle::from_file(
    "assets/levels/mon_niveau.editorproj",
    &asset_server,
));
```

📖 **Documentation complète** : [GUIDE_PARSERS.md](GUIDE_PARSERS.md)  
📦 **Dossier des parsers** : [parsers/](parsers/)

### Intégration Manuelle

### Intégration Manuelle (si vous ne voulez pas utiliser les parsers)

#### Pygame (Python)

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

#### Bevy (Rust)

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

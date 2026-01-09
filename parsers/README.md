# Parsers EditorLevel2D

Ce dossier contient les parsers pour intégrer vos niveaux EditorLevel2D dans différents moteurs de jeu.

## 📦 Parsers Disponibles

### 🐍 Pygame (Python)

**Fichier**: `pygame_parser.py`

Module complet pour charger et afficher vos niveaux dans Pygame.

**Fonctionnalités:**
- Chargement automatique des niveaux et tilesets
- Rendu optimisé avec culling des tiles hors écran
- Support complet des couleurs et textures
- Système de cache pour les performances
- Méthodes utilitaires pour collisions et entités

**Installation:**
```bash
pip install pygame
```

**Exemple rapide:**
```python
from parsers.pygame_parser import EditorLevelLoader

loader = EditorLevelLoader("mon_niveau.editorproj")
loader.render(screen, camera_x=0, camera_y=0)
```

---

### 🦀 Bevy (Rust)

**Fichier**: `bevy_parser.rs`

Module Rust pour intégrer vos niveaux dans Bevy avec un plugin dédié.

**Fonctionnalités:**
- Plugin Bevy ready-to-use
- Spawning automatique des tiles
- Support des couleurs et textures
- Components pour identifier les tiles
- Méthodes pour placer des entités

**Installation:**
Copiez `bevy_parser.rs` dans votre projet Bevy et ajoutez:
```toml
[dependencies]
bevy = "0.14"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

**Exemple rapide:**
```rust
use parsers::bevy_parser::*;

App::new()
    .add_plugins(EditorLevelPlugin)
    .add_systems(Startup, setup)
    .run();
```

---

## 📖 Documentation Complète

Voir [GUIDE_PARSERS.md](../GUIDE_PARSERS.md) pour:
- Tutoriels détaillés
- Exemples complets
- Gestion des collisions
- Placement d'entités
- Système de caméra
- Optimisations

## 🎮 Workflow Typique

1. **Créer votre niveau** avec EditorLevel2D
2. **Sauvegarder** en `.editorproj`
3. **Copier** le parser dans votre projet de jeu
4. **Charger** le niveau avec le parser
5. **Profiter** de votre map dans votre jeu!

## 🤝 Contribuer

Vous avez créé un parser pour un autre moteur? (Unity, Godot, Phaser, etc.)

N'hésitez pas à contribuer en ajoutant votre parser dans ce dossier!

## 📝 Format des Données

Les parsers supportent le format `.editorproj`:

```json
{
  "level": {
    "name": "Mon Niveau",
    "width": 64,
    "height": 48,
    "tile_size": 32,
    "layers": [...]
  },
  "tilesets": {
    "0": {
      "name": "Tileset 1",
      "path": "tilesets/tiles.png",
      ...
    }
  }
}
```

## 🎨 Couleurs Prédéfinies

Utilisez ces couleurs pour marquer des emplacements spéciaux:

| Couleur | RGB | Usage |
|---------|-----|-------|
| Spawn | `(0, 255, 0)` | Point d'apparition du joueur |
| Exit | `(0, 191, 255)` | Sortie/Fin du niveau |
| Coin | `(255, 215, 0)` | Pièces/Collectibles |
| Enemy | `(255, 0, 0)` | Ennemis |
| Ground | `(139, 69, 19)` | Sol/Plateformes |
| Wall | `(128, 128, 128)` | Murs |

Puis dans votre jeu:

```python
# Pygame
spawn_positions = loader.get_tiles_by_color((0, 255, 0))

# Bevy
let spawn_positions = project.find_tiles_by_color([0, 255, 0]);
```

## 🚀 Prochaines Étapes

Moteurs de jeu à venir:
- [ ] Unity (C#)
- [ ] Godot (GDScript)
- [ ] Phaser (JavaScript)
- [ ] LibGDX (Java)
- [ ] Love2D (Lua)

Votre moteur préféré n'est pas dans la liste? Créez un issue ou contribuez!

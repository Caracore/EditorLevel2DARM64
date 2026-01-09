# Guide d'Installation et Démarrage Rapide

## 🚀 Installation Rapide sur Raspberry Pi OS ARM64

### Installation en Une Commande

```bash
./install.sh
```

Le script s'occupe automatiquement de:
- ✅ Installer Rust (si nécessaire)
- ✅ Installer toutes les dépendances système
- ✅ Compiler le programme
- ✅ Créer le raccourci dans le menu
- ✅ Configurer le PATH

### Après l'Installation

Lancez l'éditeur depuis:
- **Terminal**: `editor_level`
- **Menu**: Cherchez "EditorLevel2D" dans Applications

## 🎮 Utilisation des Parsers

### Parser Pygame (Python)

#### Installation
```bash
pip install pygame
```

#### Test Rapide
```bash
# Créez d'abord un niveau avec l'éditeur et sauvegardez-le en .editorproj
python3 demo_pygame.py mon_niveau.editorproj
```

#### Intégration dans Votre Jeu
```python
from parsers.pygame_parser import EditorLevelLoader

# Charger le niveau
loader = EditorLevelLoader("assets/levels/level1.editorproj")

# Dans votre boucle de jeu
loader.render(screen, camera_x, camera_y)

# Trouver les spawns, coins, etc.
spawns = loader.get_tiles_by_color((0, 255, 0))  # Vert = spawn
coins = loader.get_tiles_by_color((255, 215, 0))  # Or = coins
```

### Parser Bevy (Rust)

#### Intégration
Copiez `parsers/bevy_parser.rs` dans votre projet Bevy.

```rust
use parsers::bevy_parser::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EditorLevelPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(EditorLevelBundle::from_file(
        "assets/levels/level1.editorproj",
        &asset_server,
    ));
}
```

## 📚 Documentation

- **README.md** : Documentation principale
- **GUIDE_PARSERS.md** : Guide complet des parsers avec exemples
- **GUIDE_PROJETS.md** : Système de projets .editorproj
- **GUIDE_COULEURS.md** : Guide du sélecteur de couleurs
- **parsers/README.md** : Aperçu des parsers disponibles

## 🎨 Workflow Recommandé

### 1. Créer un Niveau
1. Lancez EditorLevel2D
2. **Fichier → Nouveau**
3. Chargez vos tilesets (**Assets → Charger Tileset**)
4. Dessinez votre niveau
5. **Fichier → Sauvegarder → Projet Complet (.editorproj)**

### 2. Intégrer dans Votre Jeu

#### Pygame
```bash
# Copier le parser dans votre projet
cp parsers/pygame_parser.py mon_jeu/

# Copier votre niveau
cp mon_niveau.editorproj mon_jeu/assets/levels/

# Dans votre jeu
python3 mon_jeu/main.py
```

#### Bevy
```bash
# Copier le parser
cp parsers/bevy_parser.rs mon_jeu/src/

# Copier le niveau
cp mon_niveau.editorproj mon_jeu/assets/levels/

# Compiler
cd mon_jeu && cargo run --release
```

## 🎯 Codes Couleur pour Gameplay

Utilisez ces couleurs dans l'éditeur pour marquer des emplacements spéciaux:

| Élément | Couleur RGB | Couleur Hex | Usage |
|---------|-------------|-------------|-------|
| Spawn | `(0, 255, 0)` | `#00FF00` | Point d'apparition |
| Exit | `(0, 191, 255)` | `#00BFFF` | Sortie/Fin |
| Coin | `(255, 215, 0)` | `#FFD700` | Pièces |
| Enemy | `(255, 0, 0)` | `#FF0000` | Ennemis |
| Ground | `(139, 69, 19)` | `#8B4513` | Sol |
| Wall | `(128, 128, 128)` | `#808080` | Murs |

### Exemple d'Utilisation

**Dans l'éditeur:**
1. Créez un calque "Entities"
2. Sélectionnez la couleur verte `#00FF00`
3. Placez un tile où le joueur doit apparaître

**Dans votre jeu:**
```python
# Pygame
spawn_positions = loader.get_tiles_by_color((0, 255, 0))
player.x, player.y = spawn_positions[0]

# Rust/Bevy
let spawn_positions = project.find_tiles_by_color([0, 255, 0]);
```

## 🔧 Désinstallation

Si vous souhaitez désinstaller EditorLevel2D:

```bash
./uninstall.sh
```

Le script vous demandera si vous souhaitez conserver vos données utilisateur.

## 🧪 Test de l'Installation

Pour vérifier que tout est correctement installé:

```bash
./test_installation.sh
```

## ❓ Dépannage

### L'éditeur ne se lance pas
```bash
# Vérifier l'installation
which editor_level

# Réinstaller
./uninstall.sh
./install.sh
```

### Le parser Pygame ne fonctionne pas
```bash
# Vérifier que Pygame est installé
python3 -c "import pygame; print(pygame.ver)"

# Installer Pygame
pip install pygame
```

### Erreur de compilation Rust
```bash
# Mettre à jour Rust
rustup update

# Nettoyer et recompiler
cargo clean
cargo build --release
```

## 📞 Support

Pour toute question ou problème:
1. Consultez la documentation dans les fichiers GUIDE_*.md
2. Vérifiez les exemples fournis
3. Testez avec les fichiers exemple_*.json

## 🎉 Prochaines Étapes

1. **Créez votre premier niveau**
   ```bash
   editor_level
   ```

2. **Testez le parser Pygame**
   ```bash
   python3 demo_pygame.py exemple_projet.editorproj
   ```

3. **Lisez la doc complète**
   ```bash
   cat GUIDE_PARSERS.md
   ```

4. **Intégrez dans votre jeu**
   - Copiez le parser approprié
   - Chargez vos niveaux
   - Profitez!

Bon développement! 🎮

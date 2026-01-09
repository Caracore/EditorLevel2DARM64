# Guide d'Utilisation des Parsers

Ce guide vous explique comment utiliser les parsers EditorLevel2D pour intégrer vos niveaux dans différents moteurs de jeu.

## 📦 Parsers Disponibles

- **Pygame** : Pour les jeux Python 2D
- **Bevy** : Pour les jeux Rust avec le moteur Bevy

## 🎮 Parser Pygame

### Installation

```bash
pip install pygame
```

### Utilisation Rapide

```python
from parsers.pygame_parser import EditorLevelLoader
import pygame

# Initialisation
pygame.init()
screen = pygame.display.set_mode((800, 600))

# Charger un niveau
loader = EditorLevelLoader("mon_niveau.editorproj")

# Dans votre boucle de jeu
camera_x, camera_y = 0, 0
loader.render(screen, camera_x, camera_y)
```

### Fonctionnalités Avancées

#### 1. Accéder aux Données du Niveau

```python
# Récupérer les informations du niveau
width = loader.level["width"]
height = loader.level["height"]
tile_size = loader.level["tile_size"]
name = loader.level["name"]
```

#### 2. Travailler avec les Calques

```python
# Obtenir tous les tiles d'un calque
main_tiles = loader.get_layer_tiles("Main")

# Parcourir les tiles
for (x, y), tile_data in main_tiles.items():
    print(f"Tile à ({x}, {y}): {tile_data}")
```

#### 3. Détection de Collisions

```python
# Obtenir toutes les positions des tiles de collision
collision_positions = loader.get_collision_tiles("Main")

# Vérifier si le joueur touche un tile
player_tile_x = player.x // tile_size
player_tile_y = player.y // tile_size

if (player_tile_x, player_tile_y) in collision_positions:
    print("Collision détectée!")
```

#### 4. Placer des Entités selon les Couleurs

```python
# Couleurs spéciales (définies dans EditorLevel2D)
SPAWN_COLOR = (0, 255, 0)    # Vert
EXIT_COLOR = (0, 191, 255)   # Bleu ciel
COIN_COLOR = (255, 215, 0)   # Or
ENEMY_COLOR = (255, 0, 0)    # Rouge

# Trouver toutes les positions de spawn
spawn_positions = loader.get_tiles_by_color(SPAWN_COLOR)
if spawn_positions:
    spawn_x, spawn_y = spawn_positions[0]
    player.x = spawn_x * tile_size
    player.y = spawn_y * tile_size

# Placer tous les coins
for x, y in loader.get_tiles_by_color(COIN_COLOR):
    coins.append(Coin(x * tile_size, y * tile_size))
```

#### 5. Caméra et Zoom

```python
# Caméra qui suit le joueur
camera_x = player.x - screen_width // 2
camera_y = player.y - screen_height // 2

# Limiter la caméra aux bords du niveau
max_camera_x = loader.level["width"] * tile_size - screen_width
max_camera_y = loader.level["height"] * tile_size - screen_height
camera_x = max(0, min(camera_x, max_camera_x))
camera_y = max(0, min(camera_y, max_camera_y))

# Afficher avec zoom
loader.render(screen, camera_x, camera_y, scale=2.0)  # 2x zoom
```

### Exemple Complet Pygame

```python
import pygame
from parsers.pygame_parser import EditorLevelLoader

pygame.init()
screen = pygame.display.set_mode((800, 600))
clock = pygame.time.Clock()

# Charger le niveau
loader = EditorLevelLoader("assets/levels/level1.editorproj")

# Créer un joueur simple
class Player:
    def __init__(self, x, y):
        self.x, self.y = x, y
        self.width, self.height = 32, 32
        self.speed = 5
        self.vel_y = 0
        self.jumping = False
    
    def update(self, collision_tiles, tile_size):
        keys = pygame.key.get_pressed()
        
        # Mouvement horizontal
        if keys[pygame.K_LEFT]:
            self.x -= self.speed
        if keys[pygame.K_RIGHT]:
            self.x += self.speed
        
        # Gravité
        self.vel_y += 0.5
        self.y += self.vel_y
        
        # Collisions
        player_tile_x = int(self.x // tile_size)
        player_tile_y = int(self.y // tile_size)
        
        if (player_tile_x, player_tile_y) in collision_tiles:
            self.y = player_tile_y * tile_size - self.height
            self.vel_y = 0
            self.jumping = False
        
        # Saut
        if keys[pygame.K_SPACE] and not self.jumping:
            self.vel_y = -10
            self.jumping = True
    
    def draw(self, screen, camera_x, camera_y):
        pygame.draw.rect(screen, (0, 255, 0), 
                        (self.x - camera_x, self.y - camera_y, 
                         self.width, self.height))

# Initialiser le joueur au spawn
spawn_positions = loader.get_tiles_by_color((0, 255, 0))
player = Player(spawn_positions[0][0] * 32, spawn_positions[0][1] * 32)

# Obtenir les tiles de collision
collision_tiles = loader.get_collision_tiles("Main")
tile_size = loader.level["tile_size"]

# Boucle principale
running = True
camera_x, camera_y = 0, 0

while running:
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            running = False
    
    # Mise à jour
    player.update(collision_tiles, tile_size)
    
    # Caméra suit le joueur
    camera_x = player.x - 400
    camera_y = player.y - 300
    
    # Affichage
    screen.fill((135, 206, 235))  # Bleu ciel
    loader.render(screen, camera_x, camera_y)
    player.draw(screen, camera_x, camera_y)
    
    pygame.display.flip()
    clock.tick(60)

pygame.quit()
```

## 🦀 Parser Bevy

### Installation

Ajoutez dans votre `Cargo.toml`:

```toml
[dependencies]
bevy = "0.14"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### Utilisation Rapide

```rust
use bevy::prelude::*;
use parsers::bevy_parser::*;

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
        "assets/levels/level1.editorproj",
        &asset_server,
    ));
}
```

### Fonctionnalités Avancées

#### 1. Accéder aux Données du Niveau

```rust
fn read_level_data(query: Query<&EditorLevel>) {
    for level in query.iter() {
        let project = &level.project;
        println!("Niveau: {}", project.level.name);
        println!("Taille: {}x{}", project.level.width, project.level.height);
        println!("Nombre de calques: {}", project.level.layers.len());
    }
}
```

#### 2. Placer des Entités selon les Couleurs

```rust
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Coin;

fn spawn_entities(
    mut commands: Commands,
    query: Query<&EditorLevel, Added<EditorLevel>>,
) {
    for level in query.iter() {
        let project = &level.project;
        
        // Placer le joueur au spawn (vert)
        let spawn_positions = project.find_tiles_by_color([0, 255, 0]);
        if let Some(&(x, y)) = spawn_positions.first() {
            let tile_size = project.level.tile_size as f32;
            commands.spawn((
                Player,
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.0, 1.0, 0.0),
                        custom_size: Some(Vec2::new(tile_size, tile_size)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        x as f32 * tile_size,
                        -(y as f32 * tile_size),
                        10.0
                    ),
                    ..default()
                },
            ));
        }
        
        // Placer toutes les pièces (or)
        let coin_positions = project.find_tiles_by_color([255, 215, 0]);
        let tile_size = project.level.tile_size as f32;
        
        for &(x, y) in &coin_positions {
            commands.spawn((
                Coin,
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(1.0, 0.84, 0.0),
                        custom_size: Some(Vec2::new(tile_size * 0.5, tile_size * 0.5)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        x as f32 * tile_size,
                        -(y as f32 * tile_size),
                        10.0
                    ),
                    ..default()
                },
            ));
        }
    }
}
```

#### 3. Système de Collision

```rust
#[derive(Component)]
struct CollisionTile {
    position: (i32, i32),
}

fn setup_collision(
    mut commands: Commands,
    query: Query<&EditorLevel, Added<EditorLevel>>,
) {
    for level in query.iter() {
        let project = &level.project;
        
        // Obtenir les tiles du calque "Main" pour les collisions
        if let Some(tiles) = project.get_layer_tiles("Main") {
            let tile_size = project.level.tile_size as f32;
            
            for (&(x, y), _) in tiles {
                commands.spawn((
                    CollisionTile { position: (x, y) },
                    TransformBundle::from(Transform::from_xyz(
                        x as f32 * tile_size,
                        -(y as f32 * tile_size),
                        0.0
                    )),
                ));
            }
        }
    }
}

fn check_player_collision(
    player_query: Query<&Transform, With<Player>>,
    collision_query: Query<(&Transform, &CollisionTile)>,
) {
    for player_transform in player_query.iter() {
        let player_pos = player_transform.translation;
        
        for (col_transform, _) in collision_query.iter() {
            let col_pos = col_transform.translation;
            let distance = player_pos.distance(col_pos);
            
            if distance < 32.0 {  // tile_size
                println!("Collision détectée!");
            }
        }
    }
}
```

### Exemple Complet Bevy

```rust
use bevy::prelude::*;
use parsers::bevy_parser::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EditorLevelPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (spawn_entities, player_movement, camera_follow))
        .run();
}

#[derive(Component)]
struct Player {
    speed: f32,
}

#[derive(Component)]
struct MainCamera;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Caméra
    commands.spawn((
        Camera2dBundle::default(),
        MainCamera,
    ));
    
    // Charger le niveau
    commands.spawn(EditorLevelBundle::from_file(
        "assets/levels/level1.editorproj",
        &asset_server,
    ));
}

fn spawn_entities(
    mut commands: Commands,
    query: Query<&EditorLevel, Added<EditorLevel>>,
) {
    for level in query.iter() {
        let project = &level.project;
        let spawn_positions = project.find_tiles_by_color([0, 255, 0]);
        
        if let Some(&(x, y)) = spawn_positions.first() {
            let tile_size = project.level.tile_size as f32;
            commands.spawn((
                Player { speed: 200.0 },
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.0, 1.0, 0.0),
                        custom_size: Some(Vec2::new(tile_size, tile_size)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        x as f32 * tile_size,
                        -(y as f32 * tile_size),
                        10.0
                    ),
                    ..default()
                },
            ));
        }
    }
}

fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    for (player, mut transform) in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        
        if keyboard.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if keyboard.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }
        if keyboard.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if keyboard.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }
        
        if direction.length() > 0.0 {
            direction = direction.normalize();
            transform.translation += direction * player.speed * time.delta_seconds();
        }
    }
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            camera_transform.translation.x = player_transform.translation.x;
            camera_transform.translation.y = player_transform.translation.y;
        }
    }
}
```

## 📝 Format des Fichiers

### Structure .editorproj

```json
{
  "level": {
    "name": "Mon Niveau",
    "width": 64,
    "height": 48,
    "tile_size": 32,
    "layers": [
      {
        "name": "Background",
        "visible": true,
        "tiles": {
          "0,0": {"Color": [100, 100, 200]},
          "1,0": {"Texture": {"tileset_id": 0, "tile_index": 5}}
        }
      }
    ]
  },
  "tilesets": {
    "0": {
      "name": "Mon Tileset",
      "path": "tilesets/tiles.png",
      "tiles_per_row": 16,
      "total_tiles": 256
    }
  }
}
```

## 🎨 Codes Couleur Prédéfinis

```python
# Utilisez ces couleurs dans EditorLevel2D pour marquer des emplacements spéciaux
EMPTY = (40, 40, 40)       # Gris foncé
GROUND = (139, 69, 19)     # Marron
WALL = (128, 128, 128)     # Gris
PLATFORM = (205, 133, 63)  # Peru
SPIKE = (255, 0, 0)        # Rouge
COIN = (255, 215, 0)       # Or
SPAWN = (0, 255, 0)        # Vert
EXIT = (0, 191, 255)       # Bleu ciel
```

## 🚀 Conseils d'Utilisation

1. **Organisation des Assets**
   - Placez vos fichiers `.editorproj` dans un dossier `assets/levels/`
   - Placez vos tilesets dans `assets/tilesets/`

2. **Performance**
   - Les parsers utilisent un cache pour les tiles
   - Seuls les tiles visibles à l'écran sont dessinés (Pygame)
   - Les tiles sont automatiquement regroupés en entités (Bevy)

3. **Workflow de Développement**
   - Créez vos niveaux avec EditorLevel2D
   - Sauvegardez en `.editorproj` pour garder les tilesets
   - Rechargez le fichier dans votre jeu pour voir les changements

4. **Debuggage**
   - Utilisez les calques de visibilité pour isoler les problèmes
   - Marquez les positions importantes avec des couleurs spéciales
   - Testez avec des niveaux simples avant de créer des maps complexes

## 📚 Ressources

- [Documentation EditorLevel2D](README.md)
- [Guide des Projets](GUIDE_PROJETS.md)
- [Guide des Couleurs](GUIDE_COULEURS.md)
- [Pygame Documentation](https://www.pygame.org/docs/)
- [Bevy Documentation](https://bevyengine.org/learn/)

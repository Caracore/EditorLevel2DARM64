"""
EditorLevel2D Parser pour Pygame
=================================

Module Python pour charger et afficher les niveaux créés avec EditorLevel2D
dans vos jeux Pygame.

Installation:
    pip install pygame

Utilisation:
    from parsers.pygame_parser import EditorLevelLoader
    
    # Charger un niveau
    loader = EditorLevelLoader("mon_niveau.editorproj")
    
    # Dans votre boucle de jeu
    loader.render(screen, camera_x=0, camera_y=0)
    
    # Accéder aux données
    tiles = loader.get_layer_tiles("Main")
    width = loader.level["width"]
    height = loader.level["height"]
"""

import json
import pygame
from pathlib import Path
from typing import Dict, List, Tuple, Optional


class EditorLevelLoader:
    """Charge et affiche un niveau EditorLevel2D dans Pygame."""
    
    def __init__(self, project_file: str):
        """
        Charge un fichier de projet .editorproj ou .json
        
        Args:
            project_file: Chemin vers le fichier .editorproj ou .json
        """
        self.project_file = Path(project_file)
        self.level = None
        self.tilesets = {}
        self.tileset_surfaces = {}
        self.tile_cache = {}  # Cache des surfaces de tiles
        
        self._load_project()
    
    def _load_project(self):
        """Charge le fichier de projet."""
        with open(self.project_file, 'r', encoding='utf-8') as f:
            data = json.load(f)
        
        # Si c'est un .editorproj, il contient "level" et "tilesets"
        if "level" in data:
            self.level = data["level"]
            self.tilesets = data.get("tilesets", {})
            self._load_tilesets()
        else:
            # C'est un ancien fichier .json (juste le niveau)
            self.level = data
    
    def _load_tilesets(self):
        """Charge les images des tilesets."""
        project_dir = self.project_file.parent
        
        # Gérer le cas où tilesets est une liste ou un dictionnaire
        tilesets_items = []
        if isinstance(self.tilesets, dict):
            tilesets_items = self.tilesets.items()
        elif isinstance(self.tilesets, list):
            tilesets_items = enumerate(self.tilesets)
        
        for tileset_id, tileset_info in tilesets_items:
            # Le chemin peut être relatif ou absolu
            tileset_path = tileset_info["path"]
            full_path = project_dir / tileset_path
            
            if not full_path.exists():
                # Essayer le chemin tel quel
                full_path = Path(tileset_path)
            
            if full_path.exists():
                try:
                    self.tileset_surfaces[tileset_id] = pygame.image.load(str(full_path))
                    print(f"✓ Tileset chargé: {tileset_info['name']}")
                except pygame.error as e:
                    print(f"✗ Erreur lors du chargement de {tileset_path}: {e}")
            else:
                print(f"✗ Tileset introuvable: {tileset_path}")
    
    def get_tile_surface(self, tile_data: Dict) -> Optional[pygame.Surface]:
        """
        Retourne une surface Pygame pour un tile.
        
        Args:
            tile_data: Dictionnaire contenant "Color" ou "Texture"
            
        Returns:
            Surface Pygame ou None
        """
        # Vérifier le cache
        cache_key = str(tile_data)
        if cache_key in self.tile_cache:
            return self.tile_cache[cache_key]
        
        surface = None
        tile_size = self.level.get("tile_size", 32)
        
        if "Color" in tile_data:
            # Créer une surface colorée
            color = tuple(tile_data["Color"])
            surface = pygame.Surface((tile_size, tile_size))
            surface.fill(color)
        
        elif "Texture" in tile_data:
            # Extraire le tile du tileset
            texture_info = tile_data["Texture"]
            tileset_id = str(texture_info["tileset_id"])
            tile_index = texture_info["tile_index"]
            
            if tileset_id in self.tileset_surfaces:
                tileset_surface = self.tileset_surfaces[tileset_id]
                
                # Récupérer les infos du tileset (gérer liste ou dict)
                if isinstance(self.tilesets, dict):
                    tileset_info = self.tilesets.get(tileset_id)
                elif isinstance(self.tilesets, list):
                    try:
                        tileset_info = self.tilesets[int(tileset_id)]
                    except (ValueError, IndexError):
                        tileset_info = None
                else:
                    tileset_info = None
                
                if tileset_info:
                    # Calculer la position du tile dans le tileset
                    tiles_per_row = tileset_info["tiles_per_row"]
                    tile_x = (tile_index % tiles_per_row) * 16  # Taille tile source = 16
                    tile_y = (tile_index // tiles_per_row) * 16
                    
                    # Extraire et redimensionner le tile
                    tile_rect = pygame.Rect(tile_x, tile_y, 16, 16)
                    tile_surface = tileset_surface.subsurface(tile_rect)
                    
                    # Redimensionner au tile_size du niveau
                    if tile_size != 16:
                        tile_surface = pygame.transform.scale(tile_surface, (tile_size, tile_size))
                    
                    surface = tile_surface.copy()
        
        # Mettre en cache
        if surface:
            self.tile_cache[cache_key] = surface
        
        return surface
    
    def render(self, screen: pygame.Surface, camera_x: int = 0, camera_y: int = 0, 
               scale: float = 1.0):
        """
        Affiche le niveau sur l'écran.
        
        Args:
            screen: Surface Pygame sur laquelle dessiner
            camera_x: Position X de la caméra (en pixels)
            camera_y: Position Y de la caméra (en pixels)
            scale: Facteur de zoom (1.0 = taille normale)
        """
        tile_size = int(self.level.get("tile_size", 32) * scale)
        
        # Calculer la zone visible
        screen_width, screen_height = screen.get_size()
        start_x = max(0, camera_x // tile_size)
        start_y = max(0, camera_y // tile_size)
        end_x = min(self.level["width"], (camera_x + screen_width) // tile_size + 1)
        end_y = min(self.level["height"], (camera_y + screen_height) // tile_size + 1)
        
        # Dessiner chaque calque
        for layer in self.level["layers"]:
            if not layer["visible"]:
                continue
            
            tiles = layer["tiles"]
            
            # Dessiner les tiles visibles
            for y in range(start_y, end_y):
                for x in range(start_x, end_x):
                    tile_key = f"{x},{y}"
                    
                    if tile_key in tiles:
                        tile_data = tiles[tile_key]
                        tile_surface = self.get_tile_surface(tile_data)
                        
                        if tile_surface:
                            screen_x = x * tile_size - camera_x
                            screen_y = y * tile_size - camera_y
                            
                            if scale != 1.0:
                                tile_surface = pygame.transform.scale(
                                    tile_surface, (tile_size, tile_size)
                                )
                            
                            screen.blit(tile_surface, (screen_x, screen_y))
    
    def get_layer_tiles(self, layer_name: str) -> Dict[Tuple[int, int], Dict]:
        """
        Retourne tous les tiles d'un calque spécifique.
        
        Args:
            layer_name: Nom du calque
            
        Returns:
            Dictionnaire {(x, y): tile_data}
        """
        for layer in self.level["layers"]:
            if layer["name"] == layer_name:
                tiles = {}
                for key, value in layer["tiles"].items():
                    x, y = map(int, key.split(','))
                    tiles[(x, y)] = value
                return tiles
        return {}
    
    def get_tiles_by_color(self, color: Tuple[int, int, int]) -> List[Tuple[int, int]]:
        """
        Trouve toutes les positions des tiles d'une couleur spécifique.
        Utile pour placer des entités (spawn, coins, etc.)
        
        Args:
            color: Tuple RGB (R, G, B)
            
        Returns:
            Liste de positions [(x, y), ...]
        """
        positions = []
        
        for layer in self.level["layers"]:
            for key, tile_data in layer["tiles"].items():
                if "Color" in tile_data and tuple(tile_data["Color"]) == color:
                    x, y = map(int, key.split(','))
                    positions.append((x, y))
        
        return positions
    
    def get_collision_tiles(self, layer_name: str = "Main") -> List[Tuple[int, int]]:
        """
        Retourne toutes les positions des tiles d'un calque (pour les collisions).
        
        Args:
            layer_name: Nom du calque de collision
            
        Returns:
            Liste de positions [(x, y), ...]
        """
        tiles = self.get_layer_tiles(layer_name)
        return list(tiles.keys())


# Exemple d'utilisation complet
if __name__ == "__main__":
    # Initialiser Pygame
    pygame.init()
    screen = pygame.display.set_mode((800, 600))
    pygame.display.set_caption("EditorLevel2D - Test Pygame")
    clock = pygame.time.Clock()
    
    # Charger un niveau
    try:
        loader = EditorLevelLoader("exemple_projet.editorproj")
        print(f"Niveau chargé: {loader.level['name']}")
        print(f"Taille: {loader.level['width']}x{loader.level['height']}")
        print(f"Calques: {len(loader.level['layers'])}")
    except FileNotFoundError:
        print("Erreur: Fichier exemple_projet.editorproj introuvable")
        print("Créez un niveau avec EditorLevel2D et sauvegardez-le en .editorproj")
        pygame.quit()
        exit(1)
    
    # Variables de caméra
    camera_x = 0
    camera_y = 0
    camera_speed = 5
    
    # Boucle principale
    running = True
    while running:
        # Événements
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = False
        
        # Contrôles de la caméra
        keys = pygame.key.get_pressed()
        if keys[pygame.K_LEFT]:
            camera_x -= camera_speed
        if keys[pygame.K_RIGHT]:
            camera_x += camera_speed
        if keys[pygame.K_UP]:
            camera_y -= camera_speed
        if keys[pygame.K_DOWN]:
            camera_y += camera_speed
        
        # Limiter la caméra
        tile_size = loader.level.get("tile_size", 32)
        max_camera_x = loader.level["width"] * tile_size - 800
        max_camera_y = loader.level["height"] * tile_size - 600
        camera_x = max(0, min(camera_x, max_camera_x))
        camera_y = max(0, min(camera_y, max_camera_y))
        
        # Affichage
        screen.fill((40, 40, 40))  # Fond gris
        loader.render(screen, camera_x, camera_y)
        
        # Texte d'aide
        font = pygame.font.Font(None, 24)
        help_text = "Flèches: Déplacer la caméra | ESC: Quitter"
        text_surface = font.render(help_text, True, (255, 255, 255))
        screen.blit(text_surface, (10, 10))
        
        pygame.display.flip()
        clock.tick(60)
    
    pygame.quit()

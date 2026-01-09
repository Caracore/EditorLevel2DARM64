#!/usr/bin/env python3
"""
Script de démonstration du parser Pygame pour EditorLevel2D

Ce script charge un niveau .editorproj et l'affiche avec Pygame.
Utilisez les flèches pour déplacer la caméra.

Usage:
    python3 demo_pygame.py <fichier.editorproj>
    
Exemple:
    python3 demo_pygame.py exemple_projet.editorproj
"""

import sys
import os

# Vérifier que Pygame est installé
try:
    import pygame
except ImportError:
    print("❌ Erreur: Pygame n'est pas installé")
    print("📦 Installation: pip install pygame")
    sys.exit(1)

# Ajouter le dossier parent au path pour importer le parser
sys.path.insert(0, os.path.dirname(__file__))

try:
    from parsers.pygame_parser import EditorLevelLoader
except ImportError as e:
    print(f"❌ Erreur d'import du parser: {e}")
    sys.exit(1)


def main():
    # Vérifier les arguments
    if len(sys.argv) < 2:
        print("Usage: python3 demo_pygame.py <fichier.editorproj>")
        print("\nFichiers disponibles:")
        for f in os.listdir('.'):
            if f.endswith('.editorproj') or f.endswith('.json'):
                print(f"  - {f}")
        sys.exit(1)
    
    level_file = sys.argv[1]
    
    # Vérifier que le fichier existe
    if not os.path.exists(level_file):
        print(f"❌ Erreur: Le fichier '{level_file}' n'existe pas")
        sys.exit(1)
    
    # Initialiser Pygame
    pygame.init()
    screen = pygame.display.set_mode((800, 600))
    pygame.display.set_caption(f"EditorLevel2D Demo - {os.path.basename(level_file)}")
    clock = pygame.time.Clock()
    
    # Charger le niveau
    print(f"📂 Chargement de {level_file}...")
    try:
        loader = EditorLevelLoader(level_file)
        print(f"✅ Niveau chargé: {loader.level['name']}")
        print(f"   Taille: {loader.level['width']}x{loader.level['height']}")
        print(f"   Tile size: {loader.level['tile_size']}px")
        print(f"   Calques: {len(loader.level['layers'])}")
        
        # Afficher les tilesets chargés
        if loader.tilesets:
            print(f"   Tilesets: {len(loader.tilesets)}")
            for ts_id, ts_info in loader.tilesets.items():
                print(f"     - {ts_info['name']}")
    except Exception as e:
        print(f"❌ Erreur lors du chargement: {e}")
        import traceback
        traceback.print_exc()
        pygame.quit()
        sys.exit(1)
    
    # Variables de caméra
    camera_x = 0
    camera_y = 0
    camera_speed = 10
    
    # Calculer les limites de la caméra
    tile_size = loader.level.get("tile_size", 32)
    max_camera_x = max(0, loader.level["width"] * tile_size - 800)
    max_camera_y = max(0, loader.level["height"] * tile_size - 600)
    
    # Police pour les infos
    font = pygame.font.Font(None, 24)
    
    print("\n🎮 Contrôles:")
    print("   ←→↑↓ : Déplacer la caméra")
    print("   ESC   : Quitter")
    print("\n✨ Démarrage de la démo...")
    
    # Boucle principale
    running = True
    while running:
        # Événements
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = False
            elif event.type == pygame.KEYDOWN:
                if event.key == pygame.K_ESCAPE:
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
        camera_x = max(0, min(camera_x, max_camera_x))
        camera_y = max(0, min(camera_y, max_camera_y))
        
        # Affichage
        screen.fill((40, 40, 40))  # Fond gris foncé
        
        # Dessiner le niveau
        loader.render(screen, camera_x, camera_y)
        
        # Afficher les infos
        info_texts = [
            f"Niveau: {loader.level['name']}",
            f"Caméra: ({camera_x}, {camera_y})",
            f"FPS: {int(clock.get_fps())}",
            "",
            "Contrôles: ←→↑↓ pour bouger | ESC pour quitter",
        ]
        
        y_offset = 10
        for text in info_texts:
            if text:  # Ne pas dessiner les lignes vides
                text_surface = font.render(text, True, (255, 255, 255))
                # Fond semi-transparent
                bg_rect = text_surface.get_rect()
                bg_rect.topleft = (10, y_offset)
                bg_rect.inflate_ip(10, 4)
                bg_surface = pygame.Surface(bg_rect.size)
                bg_surface.set_alpha(128)
                bg_surface.fill((0, 0, 0))
                screen.blit(bg_surface, bg_rect)
                screen.blit(text_surface, (10, y_offset))
            y_offset += 25
        
        pygame.display.flip()
        clock.tick(60)
    
    # Nettoyer
    pygame.quit()
    print("👋 Merci d'avoir testé EditorLevel2D!")


if __name__ == "__main__":
    main()

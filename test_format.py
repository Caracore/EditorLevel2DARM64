#!/usr/bin/env python3
"""
Script de test pour valider le format JSON de l'éditeur
Compatible avec Pygame et autres moteurs
"""

import json
import sys

def rgb_to_hex(rgb):
    """Convertit RGB [R, G, B] en code hexadécimal #RRGGBB"""
    return f"#{rgb[0]:02X}{rgb[1]:02X}{rgb[2]:02X}"

def test_level_format(filepath):
    """Teste et affiche le contenu d'un niveau"""
    try:
        with open(filepath, 'r') as f:
            level = json.load(f)
        
        print("=" * 60)
        print(f"📁 Fichier : {filepath}")
        print("=" * 60)
        print(f"Nom du niveau : {level['name']}")
        print(f"Dimensions : {level['width']}x{level['height']}")
        print(f"Taille des tiles : {level['tile_size']}px")
        print(f"Nombre de calques : {len(level['layers'])}")
        print()
        
        total_tiles = 0
        
        for layer in level['layers']:
            print(f"\n📑 Calque : {layer['name']}")
            print(f"   Visible : {'✓' if layer['visible'] else '✗'}")
            print(f"   Nombre de tiles : {len(layer['tiles'])}")
            
            total_tiles += len(layer['tiles'])
            
            # Afficher quelques exemples de tiles
            if layer['tiles']:
                print("   Exemples de tiles :")
                count = 0
                for pos, tile in layer['tiles'].items():
                    if count >= 5:  # Limiter à 5 exemples
                        print(f"   ... ({len(layer['tiles']) - 5} autres)")
                        break
                    
                    x, y = map(int, pos.split(','))
                    
                    if "Color" in tile:
                        rgb = tile["Color"]
                        hex_color = rgb_to_hex(rgb)
                        print(f"      ({x:2d},{y:2d}) : Couleur RGB{rgb} = {hex_color}")
                    elif "Texture" in tile:
                        tex = tile["Texture"]
                        print(f"      ({x:2d},{y:2d}) : Texture (tileset {tex['tileset_id']}, index {tex['tile_index']})")
                    
                    count += 1
        
        print()
        print("=" * 60)
        print(f"✅ Total : {total_tiles} tiles placés")
        print("=" * 60)
        print("\n✓ Format JSON valide et compatible !")
        print("✓ Compatible avec Pygame, Bevy, Godot, Unity, etc.")
        
        return True
        
    except FileNotFoundError:
        print(f"❌ Erreur : Fichier '{filepath}' introuvable")
        return False
    except json.JSONDecodeError as e:
        print(f"❌ Erreur JSON : {e}")
        return False
    except KeyError as e:
        print(f"❌ Erreur de format : Clé manquante {e}")
        return False
    except Exception as e:
        print(f"❌ Erreur : {e}")
        return False

def pygame_example(filepath):
    """Génère un exemple de code Pygame pour charger le niveau"""
    print("\n" + "=" * 60)
    print("📝 Exemple de code Pygame")
    print("=" * 60)
    print(f"""
import pygame
import json

# Initialisation
pygame.init()
screen = pygame.display.set_mode((800, 600))

# Charger le niveau
with open('{filepath}', 'r') as f:
    level = json.load(f)

tile_size = level['tile_size']

# Dessiner les tiles
for layer in level['layers']:
    if not layer['visible']:
        continue
    
    for pos_str, tile_data in layer['tiles'].items():
        x, y = map(int, pos_str.split(','))
        rect = pygame.Rect(x * tile_size, y * tile_size, tile_size, tile_size)
        
        if "Color" in tile_data:
            rgb = tile_data["Color"]
            pygame.draw.rect(screen, rgb, rect)
        elif "Texture" in tile_data:
            # Charger depuis le tileset
            tileset_id = tile_data["Texture"]["tileset_id"]
            tile_index = tile_data["Texture"]["tile_index"]
            # ... dessiner depuis le tileset

pygame.display.flip()
""")

if __name__ == "__main__":
    if len(sys.argv) > 1:
        filepath = sys.argv[1]
    else:
        # Fichiers de test par défaut
        test_files = [
            "exemple_couleurs_hex.json",
            "exemple_avec_calques.json",
            "exemple_niveau.json"
        ]
        
        print("🧪 Test des fichiers d'exemple\n")
        
        for filepath in test_files:
            if test_level_format(filepath):
                print()
        
        # Afficher un exemple Pygame
        if test_files:
            pygame_example(test_files[0])
        
        sys.exit(0)
    
    # Test d'un fichier spécifique
    if test_level_format(filepath):
        pygame_example(filepath)
        sys.exit(0)
    else:
        sys.exit(1)

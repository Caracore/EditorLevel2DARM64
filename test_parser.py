#!/usr/bin/env python3
"""
Test basique du parser Pygame (sans fenêtre graphique)

Ce script teste le chargement d'un niveau sans avoir besoin d'afficher
une fenêtre Pygame. Utile pour vérifier que le parser fonctionne.
"""

import sys
import os
import json

# Ajouter le dossier au path
sys.path.insert(0, os.path.dirname(__file__))


def test_json_loading():
    """Test de chargement d'un fichier JSON simple"""
    print("🧪 Test 1: Chargement JSON basique")
    
    # Chercher un fichier exemple
    test_files = [
        "exemple_projet.editorproj",
        "exemple_niveau.json",
        "test.json"
    ]
    
    test_file = None
    for f in test_files:
        if os.path.exists(f):
            test_file = f
            break
    
    if not test_file:
        print("❌ Aucun fichier de test trouvé")
        return False
    
    try:
        with open(test_file, 'r', encoding='utf-8') as f:
            data = json.load(f)
        
        print(f"✅ Fichier chargé: {test_file}")
        
        # Vérifier la structure
        if "level" in data:
            level = data["level"]
            print(f"   Type: Projet complet (.editorproj)")
            print(f"   Niveau: {level.get('name', 'Sans nom')}")
            print(f"   Taille: {level.get('width')}x{level.get('height')}")
            print(f"   Calques: {len(level.get('layers', []))}")
            if "tilesets" in data:
                print(f"   Tilesets: {len(data['tilesets'])}")
        else:
            print(f"   Type: Niveau seul (.json)")
            print(f"   Niveau: {data.get('name', 'Sans nom')}")
            print(f"   Taille: {data.get('width')}x{data.get('height')}")
            print(f"   Calques: {len(data.get('layers', []))}")
        
        return True
        
    except Exception as e:
        print(f"❌ Erreur: {e}")
        return False


def test_parser_import():
    """Test d'import du parser"""
    print("\n🧪 Test 2: Import du parser")
    
    try:
        from parsers.pygame_parser import EditorLevelLoader
        print("✅ Parser importé avec succès")
        
        # Vérifier que la classe a les bonnes méthodes
        required_methods = [
            'render',
            'get_layer_tiles',
            'get_tiles_by_color',
            'get_collision_tiles',
        ]
        
        for method in required_methods:
            if hasattr(EditorLevelLoader, method):
                print(f"   ✓ Méthode '{method}' présente")
            else:
                print(f"   ✗ Méthode '{method}' manquante")
                return False
        
        return True
        
    except ImportError as e:
        print(f"❌ Impossible d'importer le parser: {e}")
        return False


def test_parser_loading():
    """Test de chargement avec le parser (sans Pygame)"""
    print("\n🧪 Test 3: Chargement avec le parser")
    
    # Chercher un fichier exemple
    test_files = [
        "exemple_projet.editorproj",
        "exemple_niveau.json",
        "test.json"
    ]
    
    test_file = None
    for f in test_files:
        if os.path.exists(f):
            test_file = f
            break
    
    if not test_file:
        print("❌ Aucun fichier de test trouvé")
        return False
    
    try:
        # Mock de pygame pour le test sans l'installer
        import sys
        from unittest.mock import MagicMock
        
        # Créer un mock de pygame
        pygame_mock = MagicMock()
        pygame_mock.Surface = MagicMock
        pygame_mock.image.load = MagicMock(return_value=MagicMock())
        pygame_mock.transform.scale = MagicMock(return_value=MagicMock())
        pygame_mock.error = Exception
        
        sys.modules['pygame'] = pygame_mock
        
        from parsers.pygame_parser import EditorLevelLoader
        
        loader = EditorLevelLoader(test_file)
        
        print(f"✅ Niveau chargé: {loader.level['name']}")
        print(f"   Dimensions: {loader.level['width']}x{loader.level['height']}")
        print(f"   Tile size: {loader.level.get('tile_size', 32)}px")
        print(f"   Nombre de calques: {len(loader.level['layers'])}")
        
        # Tester les méthodes
        if loader.level['layers']:
            first_layer = loader.level['layers'][0]['name']
            tiles = loader.get_layer_tiles(first_layer)
            print(f"   Tiles dans '{first_layer}': {len(tiles)}")
        
        return True
        
    except Exception as e:
        print(f"❌ Erreur lors du chargement: {e}")
        import traceback
        traceback.print_exc()
        return False


def main():
    print("=" * 60)
    print("Test du Parser EditorLevel2D (sans interface)")
    print("=" * 60)
    print()
    
    results = []
    
    # Test 1: Chargement JSON
    results.append(("Chargement JSON", test_json_loading()))
    
    # Test 2: Import du parser
    results.append(("Import du parser", test_parser_import()))
    
    # Test 3: Chargement avec le parser
    results.append(("Chargement avec parser", test_parser_loading()))
    
    # Résumé
    print("\n" + "=" * 60)
    print("Résumé des Tests")
    print("=" * 60)
    
    passed = 0
    total = len(results)
    
    for name, success in results:
        status = "✅ PASS" if success else "❌ FAIL"
        print(f"{status} - {name}")
        if success:
            passed += 1
    
    print(f"\nRésultat: {passed}/{total} tests réussis")
    
    if passed == total:
        print("\n🎉 Tous les tests sont passés!")
        print("\n📝 Prochaines étapes:")
        print("   1. Installer Pygame: pip install pygame")
        print("   2. Tester avec la démo: python3 demo_pygame.py exemple_projet.editorproj")
        return 0
    else:
        print("\n⚠️ Certains tests ont échoué")
        print("   Vérifiez que tous les fichiers sont présents")
        return 1


if __name__ == "__main__":
    sys.exit(main())

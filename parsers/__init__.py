"""
EditorLevel2D Parsers Package

Ce package contient les parsers pour différents moteurs de jeu.

Modules disponibles:
- pygame_parser: Parser pour Pygame (Python)
- bevy_parser: Parser pour Bevy (Rust) - copier dans votre projet Rust

Usage:
    from parsers.pygame_parser import EditorLevelLoader
    
    loader = EditorLevelLoader("mon_niveau.editorproj")
    loader.render(screen)
"""

__version__ = "1.0.0"
__author__ = "EditorLevel2D Team"

# Exposer les classes principales pour un import facile
try:
    from .pygame_parser import EditorLevelLoader
    __all__ = ['EditorLevelLoader']
except ImportError:
    # Pygame n'est peut-être pas installé
    __all__ = []

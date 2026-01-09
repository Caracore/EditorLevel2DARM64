# Ajouter

## ✅ Complété

### Scripts d'Installation/Désinstallation
- ✅ Script d'installation pour Pi OS ARM64 (`install.sh`)
  - Installation automatique de Rust et dépendances
  - Compilation et installation du binaire
  - Création du raccourci menu
  - Configuration du PATH
  
- ✅ Script de désinstallation (`uninstall.sh`)
  - Suppression propre du binaire
  - Nettoyage des raccourcis
  - Option de conservation des données utilisateur

### Parsers pour Moteurs de Jeu
- ✅ Parser Pygame (Python) - `parsers/pygame_parser.py`
  - Chargement automatique des niveaux et tilesets
  - Rendu optimisé avec culling
  - Méthodes pour collisions et placement d'entités
  - Support caméra et zoom
  
- ✅ Parser Bevy (Rust) - `parsers/bevy_parser.rs`
  - Plugin Bevy ready-to-use
  - Spawning automatique des tiles
  - Components et systèmes intégrés
  - Support textures et couleurs

- ✅ Documentation complète - `GUIDE_PARSERS.md`
  - Tutoriels détaillés
  - Exemples complets Pygame et Bevy
  - Guide d'intégration

## 🚀 À Faire

### Parsers Supplémentaires
- [ ] Parser pour Unity (C#)
- [ ] Parser pour Godot (GDScript)
- [ ] Parser pour Phaser (JavaScript)
- [ ] Parser pour LibGDX (Java)
- [ ] Parser pour Love2D (Lua)

### Améliorations de l'Éditeur
- [ ] Outil de sélection et copier-coller
- [ ] Configuration personnalisée de la taille des tiles via UI
- [ ] Support des animations de tiles
- [ ] Undo/Redo
- [ ] Minimap
- [ ] Export vers TMX (Tiled)
- [ ] Drag & drop de tilesets

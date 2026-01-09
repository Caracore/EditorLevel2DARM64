# Version et Historique

## Version 0.2.0 - 9 janvier 2026

### 🎉 Nouveautés Majeures

#### Scripts d'Installation
- ✨ Script d'installation automatique pour Raspberry Pi OS ARM64
- ✨ Script de désinstallation avec options de nettoyage
- ✨ Configuration automatique du PATH et du menu
- ✨ Gestion intelligente des dépendances

#### Parsers pour Moteurs de Jeu
- ✨ Parser Pygame complet avec rendu optimisé
- ✨ Parser Bevy avec plugin ECS intégré
- ✨ Support des formats .editorproj et .json
- ✨ Méthodes utilitaires (collisions, entités, caméra)

#### Documentation
- ✨ GUIDE_PARSERS.md - Guide complet d'utilisation
- ✨ QUICKSTART.md - Démarrage rapide
- ✨ NOUVEAUTES.md - Liste des ajouts
- ✨ parsers/README.md - Documentation des parsers

#### Outils de Test
- ✨ test_installation.sh - Vérifie l'installation
- ✨ test_parser.py - Test du parser sans interface
- ✨ demo_pygame.py - Démonstration interactive

### 📦 Fichiers Ajoutés

**Scripts (4):**
- install.sh
- uninstall.sh
- test_installation.sh
- demo_pygame.py

**Parsers (3):**
- parsers/pygame_parser.py
- parsers/bevy_parser.rs
- parsers/__init__.py

**Documentation (4):**
- GUIDE_PARSERS.md
- QUICKSTART.md
- NOUVEAUTES.md
- parsers/README.md

**Tests (1):**
- test_parser.py

### 🔧 Améliorations

- Parser Pygame: Support des formats liste et dictionnaire pour tilesets
- Documentation: Plus de 70 exemples de code
- README.md: Sections ajoutées pour installation et parsers

### 🐛 Corrections

- Parser Pygame: Gestion correcte des tilesets en liste ou dictionnaire
- Compatibilité: Vérification de l'architecture ARM64

---

## Version 0.1.0 - Décembre 2025

### Fonctionnalités Initiales

- ✨ Éditeur de niveaux 2D avec interface graphique
- ✨ Système de calques avec visibilité
- ✨ Support des tilesets (PNG, JPG)
- ✨ Sélecteur de couleur RGB avec codes hex
- ✨ Format .editorproj avec métadonnées
- ✨ Export JSON pour intégration
- ✨ Zoom et navigation dans le canvas
- ✨ Outils pinceau et gomme

### Guides Créés

- README.md - Documentation principale
- GUIDE_PROJETS.md - Système de projets
- GUIDE_CHARGER.md - Chargement de niveaux
- GUIDE_COULEURS.md - Sélecteur de couleurs
- GUIDE_CONFIGURATION.md - Configuration du canvas
- GUIDE_MIGRATION.md - Migration des formats
- GUIDE_TEST.md - Tests et validation

---

## Roadmap Future

### Version 0.3.0 (À venir)

**Parsers Supplémentaires:**
- [ ] Unity (C#)
- [ ] Godot (GDScript)
- [ ] Phaser (JavaScript)
- [ ] LibGDX (Java)
- [ ] Love2D (Lua)

**Améliorations de l'Éditeur:**
- [ ] Outil de sélection et copier-coller
- [ ] Undo/Redo
- [ ] Minimap
- [ ] Animations de tiles
- [ ] Export TMX (Tiled)
- [ ] Drag & drop de tilesets

**Optimisations:**
- [ ] Cache de rendu amélioré
- [ ] Multithreading pour grandes maps
- [ ] Compression des fichiers

---

## Statistiques

### Version 0.2.0
- **Lignes de code ajoutées:** ~2,300
- **Nouveaux fichiers:** 12
- **Documentation:** +1,000 lignes
- **Exemples de code:** 70+
- **Guides:** 4 nouveaux

### Total du Projet
- **Fichiers sources Rust:** 6
- **Fichiers Python:** 4
- **Scripts Shell:** 5
- **Documentation:** 13 fichiers
- **Lignes de code totales:** ~5,000+

---

## Contribution

Merci à tous les utilisateurs qui ont testé et fourni des retours!

Pour contribuer:
1. Forkez le projet
2. Créez une branche feature
3. Committez vos changements
4. Ouvrez une Pull Request

---

## Licence

MIT License - Voir LICENSE pour plus de détails

---

## Contact

Pour questions, bugs ou suggestions:
- Ouvrez une issue sur GitHub
- Consultez la documentation
- Testez avec les fichiers d'exemple

**Bon développement! 🎮**

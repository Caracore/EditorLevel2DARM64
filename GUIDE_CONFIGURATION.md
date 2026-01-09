# Configuration du Canvas et des Calques

## 🐛 Corrections apportées

### Erreurs d'ID egui corrigées
Les erreurs que vous avez vues dans l'image ont été corrigées :
- ✅ "First/Second use of ScrollArea ID 92AA" → IDs uniques ajoutés
- ✅ "First/Second use of widget ID BC6B" → IDs uniques pour chaque ScrollArea

**Correction technique :**
```rust
// Avant (causait des erreurs)
egui::ScrollArea::vertical().show(ui, |ui| { ... });

// Après (IDs uniques)
egui::ScrollArea::vertical()
    .id_source("tilesets_list_scroll")
    .show(ui, |ui| { ... });
```

## ⚙️ Configuration du Canvas (NOUVEAU!)

### Accès : Menu **Affichage** → Section "⚙️ Configuration du Canvas"

### Réglage de la taille du canvas

Vous pouvez maintenant ajuster la taille de votre carte dynamiquement :

- **Largeur** : 10 à 1000 tiles (utilisez les flèches ou tapez un nombre)
- **Hauteur** : 10 à 1000 tiles

### Presets rapides disponibles

| Preset | Taille | Usage |
|--------|--------|-------|
| 🔲 Petit | 32x24 | Petites salles, tutoriels |
| 🔳 Moyen | 64x48 | Niveau standard RPG 2D |
| 🔴 Grand | 128x96 | Grandes maps, exploration |
| ♾️ Énorme | 256x256 | Monde ouvert, presque infini |

**Note :** Le preset "Énorme" (256x256 = 65,536 tiles) simule un canvas quasi-infini pour les jeux d'exploration.

## 📑 Configuration des Calques (NOUVEAU!)

### Accès : Menu **Édition** → "⚙️ Configuration des calques..."

### Fenêtre de configuration

Une nouvelle fenêtre dédiée s'ouvre avec toutes les options :

#### 1. Liste des calques
- Voir tous vos calques
- **Renommer** : Cliquez dans le champ texte et modifiez le nom
- **Visibilité** : Toggle 👁 pour afficher/masquer
- **Compteur** : Nombre de tiles par calque

#### 2. Actions rapides
- **➕ Ajouter calque** : Crée un nouveau calque vierge
- **🗑️ Tout effacer** : Vide tous les calques (garde la structure)

#### 3. Presets de calques

##### 🎨 Setup RPG Standard (5 calques)
```
1. Fond lointain    - Ciel, éléments très éloignés
2. Arrière-plan     - Décor, bâtiments, arbres
3. Gameplay         - Sol, murs, plateformes (actif par défaut)
4. Décorations      - Objets, détails
5. Premier plan     - Éléments au-dessus du joueur
```

##### 🏗️ Setup Parallax (7 calques)
Pour des effets de profondeur avancés :
```
1. Ciel             - Background fixe
2. Montagnes        - Défilement très lent
3. Arbres lointains - Défilement lent
4. Terrain          - Vitesse normale (actif par défaut)
5. Objets           - Joueur, ennemis, objets
6. Arbres proches   - Défilement rapide
7. UI/Overlay       - Interface, HUD
```

##### 🎮 Setup Minimal (3 calques)
Pour les projets simples :
```
1. Background       - Arrière-plan
2. Main             - Gameplay principal (actif par défaut)
3. Foreground       - Premier plan
```

## 🎯 Cas d'usage

### Monde ouvert quasi-infini

```
1. Affichage → Configuration du Canvas
2. Sélectionner "♾️ Énorme (256x256)"
3. Canvas de 256x256 tiles = 65,536 emplacements !
4. Avec tiles de 16px : 4096x4096 pixels au total
5. Parfait pour : Terraria-like, Metroidvania, exploration
```

### Niveau avec parallax professionnel

```
1. Édition → Configuration des calques
2. Cliquer "🏗️ Setup Parallax (7 calques)"
3. Dessiner sur chaque calque :
   - Calque 1 : Couleur de ciel unie
   - Calque 2 : Montagnes avec sélecteur couleur
   - Calque 3 : Arbres avec tileset
   - Calque 4-5 : Gameplay (sol, plateformes)
   - Calque 6 : Branches, feuilles au premier plan
   - Calque 7 : Vide (pour UI dans le jeu)
4. Dans votre jeu, déplacer chaque calque à des vitesses différentes
```

### Niveau simple mais professionnel

```
1. Édition → Configuration des calques
2. Cliquer "🎨 Setup RPG Standard (5 calques)"
3. Répartition suggérée :
   - Fond : Couleur unie (ciel/ambiance)
   - Arrière-plan : Décor statique
   - Gameplay : Sol, murs, plateformes
   - Décorations : Plantes, détails
   - Premier plan : Branches qui passent devant
```

## 💡 Conseils d'utilisation

### Taille du canvas

**Petit projet (32x24)** :
- Temps de dessin rapide
- Parfait pour prototypage
- Bon pour mini-jeux

**Moyen (64x48)** :
- Taille standard RPG 2D
- Bon équilibre détail/performance
- Recommandé pour débuter

**Grand (128x96)** :
- Grandes zones à explorer
- Demande plus de travail
- Bon pour niveaux complexes

**Énorme (256x256)** :
- Quasi-infini pour le joueur
- Permet scrolling dans toutes les directions
- Nécessite bonne organisation

### Nombre de calques

**3 calques (Minimal)** :
- Jeux simples, platformers basiques
- Facile à gérer
- Performance optimale

**5 calques (Standard)** :
- Équilibre idéal pour la plupart des jeux
- Permet bon niveau de détail
- Recommandé pour RPG 2D classiques

**7 calques (Parallax)** :
- Effets de profondeur professionnels
- Visuels impressionnants
- Demande plus de travail artistique

**Plus de 7 calques** :
- Créez autant que nécessaire avec ➕
- Exemple : 10+ pour jeux très détaillés
- Attention à la complexité

## 🔧 Intégration dans votre jeu

### Canvas énorme (256x256)

```python
# Pygame - Charger seulement la zone visible
level = load_level("huge_map.json")
camera_x, camera_y = player.x, player.y
viewport_w, viewport_h = 20, 15  # Tiles visibles

for layer in level["layers"]:
    for pos, tile in layer["tiles"].items():
        x, y = map(int, pos.split(','))
        # Ne dessiner que les tiles dans le viewport
        if (camera_x - viewport_w//2 <= x <= camera_x + viewport_w//2 and
            camera_y - viewport_h//2 <= y <= camera_y + viewport_h//2):
            draw_tile(tile, x, y)
```

### Parallax avec plusieurs calques

```python
# Vitesses de défilement par calque
parallax_speeds = {
    "Ciel": 0.0,           # Fixe
    "Montagnes": 0.2,      # Très lent
    "Arbres lointains": 0.4,
    "Terrain": 1.0,        # Normal
    "Objets": 1.0,         # Normal
    "Arbres proches": 1.5, # Rapide
    "UI/Overlay": 0.0      # Fixe
}

for layer in level["layers"]:
    speed = parallax_speeds.get(layer["name"], 1.0)
    offset_x = camera_x * speed
    # Dessiner avec l'offset calculé
```

## 📊 Limites et recommandations

### Performances

| Taille | Tiles max | Recommandation |
|--------|-----------|----------------|
| 32x24 | 768 | ✅ Excellent |
| 64x48 | 3,072 | ✅ Très bon |
| 128x96 | 12,288 | ✅ Bon (chargement sélectif) |
| 256x256 | 65,536 | ⚠️ Nécessite optimisation |

**Astuce** : Pour les grandes maps, ne dessinez que les tiles visibles à l'écran (culling).

### Nombre de calques

- ✅ 3-7 calques : Optimal pour la plupart des jeux
- ⚠️ 8-15 calques : Possible, mais organisez bien
- ❌ 15+ calques : Risque de confusion, sauf besoin spécifique

## ✅ Résumé des améliorations

### Erreurs corrigées
- ✅ Plus d'erreurs "ScrollArea ID" dupliquées
- ✅ Plus d'erreurs "widget ID" dupliquées
- ✅ Interface stable et sans warnings

### Nouvelles fonctionnalités
- ✅ Configuration dynamique du canvas (10x10 à 1000x1000)
- ✅ Presets de taille (Petit, Moyen, Grand, Énorme)
- ✅ Fenêtre dédiée de configuration des calques
- ✅ Renommage des calques en direct
- ✅ Presets de calques (Minimal, Standard, Parallax)
- ✅ Actions rapides (ajouter, tout effacer)

Votre éditeur est maintenant totalement flexible et peut créer des maps de toutes tailles avec autant de calques que nécessaire ! 🚀

# Guide Rapide - Test de l'Éditeur

## Test 1 : Sauvegarde/Chargement ✅

1. Lancer l'éditeur : `./target/release/editor_level`
2. Dessiner quelques tiles sur la map
3. Fichier → Sauvegarder → choisir un nom
4. Vérifier que le fichier JSON est créé
5. Fichier → Charger → charger le fichier
6. Vérifier que la map est restaurée correctement

**Résultat attendu** : Aucune erreur "key must be a string", le fichier JSON utilise des clés "x,y"

## Test 2 : Gestion des Calques ✅

1. Lancer l'éditeur
2. Dessiner sur le calque "Main"
3. Cliquer sur "Background" pour changer de calque
4. Dessiner différemment sur ce calque
5. Utiliser 👁 pour cacher/afficher les calques
6. Cliquer sur ➕ pour ajouter un nouveau calque "Layer 4"
7. Dessiner sur ce nouveau calque
8. Utiliser ⬆⬇ pour réorganiser les calques
9. Menu Édition → Supprimer le calque actuel

**Résultat attendu** : 
- Chaque calque garde ses tiles indépendamment
- La visibilité fonctionne
- L'ajout/suppression fonctionne
- La réorganisation change l'ordre d'affichage

## Test 3 : Tilesets (avec images) 🖼️

1. Trouver ou créer un tileset 16x16 (ex: terrain.png)
2. Cliquer sur "➕ Charger Tileset"
3. Sélectionner l'image
4. Vérifier que la grille de tiles s'affiche
5. Cliquer sur un tile pour le sélectionner
6. Dessiner sur la map avec ce tile
7. Sauvegarder
8. Vérifier dans le JSON : les tiles ont {"Texture": {"tileset_id": 0, "tile_index": X}}

**Résultat attendu** : Les textures s'affichent correctement, la sauvegarde contient les références

## Test 4 : Contrôles Souris 🖱️

1. Sélectionner un tile avec le Pinceau actif
2. **Clic gauche** → doit peindre
3. **Clic droit** → doit effacer (gomme rapide)
4. Activer l'outil Gomme
5. **Clic gauche** → doit effacer
6. **Clic droit** → doit peindre (inversé!)
7. **Molette** → zoom in/out
8. **Clic molette + glisser** → déplacer la vue

**Résultat attendu** : Le clic droit agit comme une gomme rapide

## Test 5 : Export vers Pygame 🐍

Après avoir créé une map avec tilesets :

```python
import json

with open("ma_map.json", "r") as f:
    level = json.load(f)

print(f"Niveau: {level['name']}")
print(f"Dimensions: {level['width']}x{level['height']}")
print(f"Taille tile: {level['tile_size']}")

for layer in level["layers"]:
    print(f"\nCalque: {layer['name']} (visible: {layer['visible']})")
    print(f"  Nombre de tiles: {len(layer['tiles'])}")
    
    for pos, tile in layer["tiles"].items():
        x, y = map(int, pos.split(','))
        if "Texture" in tile:
            print(f"  ({x},{y}): Tileset {tile['Texture']['tileset_id']}, Index {tile['Texture']['tile_index']}")
        else:
            print(f"  ({x},{y}): Couleur {tile['Color']}")
```

**Résultat attendu** : Le script Python lit correctement toutes les données

## Ressources Tilesets Gratuites

Pour tester avec de vrais tilesets :

- **Kenney.nl** : https://kenney.nl/assets (style moderne)
- **OpenGameArt.org** : https://opengameart.org/art-search?keys=tileset
- **itch.io** : https://itch.io/game-assets/free/tag-tileset (nombreux styles RPG)

Cherchez des tilesets avec :
- Format PNG avec transparence
- Taille 16x16 ou 32x32 pixels par tile
- Grille régulière sans espacement

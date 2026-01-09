# Guide : Charger et Rééditer une Map

## 📂 Comment ouvrir et rééditer une map existante

### Méthode 1 : Via le menu Fichier

1. Lancez l'éditeur : `./target/release/editor_level`
2. Cliquez sur **Fichier** → **📂 Charger**
3. Sélectionnez un fichier JSON (ex: `exemple_couleurs_hex.json`)
4. ✅ La map se charge avec :
   - Tous les calques restaurés
   - Toutes les tiles (couleurs et textures)
   - Vue réinitialisée (zoom 100%, centré)
   - Notification de succès affichée

### Méthode 2 : Tester avec les exemples fournis

L'éditeur inclut plusieurs fichiers d'exemple :

```bash
# Dans le dossier EditorLevel2DARM64/

exemple_niveau.json              # Map avec l'ancien format
exemple_avec_calques.json        # Map avec 4 calques
exemple_couleurs_hex.json        # Map avec couleurs RGB
test.json                        # Votre dernière sauvegarde
```

## 🎨 Workflow complet : Créer, Sauvegarder, Rééditer

### Étape 1 : Créer une nouvelle map

```
1. Fichier → Nouveau
2. Dessinez votre niveau avec :
   - Couleurs personnalisées (sliders RGB ou hex)
   - Tilesets (si vous en avez chargé)
   - Plusieurs calques pour la profondeur
```

### Étape 2 : Sauvegarder

```
1. Fichier → 💾 Sauvegarder
2. Choisissez un nom : "ma_map.json"
3. ✅ Notification : "Niveau sauvegardé : ma_map.json"
```

### Étape 3 : Fermer et rouvrir

```
1. Fichier → Quitter (ou fermez la fenêtre)
2. Relancez l'éditeur
3. Fichier → 📂 Charger
4. Sélectionnez "ma_map.json"
5. ✅ Tout est restauré à l'identique !
```

### Étape 4 : Continuer l'édition

```
- Tous vos calques sont là
- Toutes vos couleurs et textures
- Vous pouvez :
  ✓ Ajouter de nouveaux éléments
  ✓ Modifier des tiles existantes
  ✓ Ajouter/supprimer des calques
  ✓ Re-sauvegarder (même fichier ou nouveau)
```

## 🔍 Informations affichées au chargement

Quand vous chargez un niveau, vous verrez :

```
✅ Niveau chargé : ma_map.json (3 calques, 45 tiles)
```

Cette notification vous indique :
- ✓ Le nom du fichier chargé
- ✓ Le nombre de calques
- ✓ Le nombre total de tiles

Le nom du fichier reste affiché dans la barre du bas : `📂 ma_map.json`

## ⚠️ Gestion des erreurs

### Fichier corrompu ou invalide

```
❌ Erreur de chargement : expected value at line 5 column 10
```

→ Vérifiez que le JSON est valide avec `python3 test_format.py ma_map.json`

### Fichier avec ancien format

Les fichiers créés avec l'ancienne version (avant les couleurs RGB) peuvent avoir des problèmes.

**Solution :** Convertissez manuellement dans le JSON :
```json
Ancien : "0,0": "Ground"
Nouveau : "0,0": {"Color": [139, 69, 19]}
```

## 🎯 Cas d'usage pratiques

### Travail en sessions multiples

```
Jour 1 : Créer la base (terrain, obstacles)
        → Sauvegarder "niveau1_base.json"

Jour 2 : Charger "niveau1_base.json"
        → Ajouter décorations et objets
        → Sauvegarder "niveau1_v2.json"

Jour 3 : Charger "niveau1_v2.json"
        → Peaufiner, tester
        → Sauvegarder "niveau1_final.json"
```

### Variantes d'un même niveau

```
1. Créer "niveau1_jour.json" avec couleurs claires
2. Charger "niveau1_jour.json"
3. Modifier les couleurs (plus sombres)
4. Sauvegarder sous "niveau1_nuit.json"
```

### Templates réutilisables

```
1. Créer une structure de base (plateformes standard)
2. Sauvegarder "template_platformer.json"
3. Pour chaque nouveau niveau :
   - Charger "template_platformer.json"
   - Personnaliser
   - Sauvegarder sous un nouveau nom
```

## 💾 Format de sauvegarde

Les fichiers sont en **JSON pur** :

```json
{
  "name": "Mon Niveau",
  "width": 64,
  "height": 48,
  "tile_size": 16,
  "layers": [
    {
      "name": "Background",
      "visible": true,
      "tiles": {
        "0,0": {"Color": [139, 69, 19]},
        "1,0": {"Texture": {"tileset_id": 0, "tile_index": 5}}
      }
    }
  ]
}
```

**Avantages :**
- ✓ Lisible par l'humain
- ✓ Éditable manuellement si besoin
- ✓ Compatible avec Git/contrôle de version
- ✓ Portable entre systèmes

## 🔄 Sauvegarde automatique (à venir)

Fonctionnalité future :
- Auto-save toutes les 5 minutes
- Fichier temporaire ".autosave.json"
- Récupération en cas de crash

## ✅ Checklist avant de partager un niveau

Avant de partager votre niveau avec d'autres :

- [ ] Le fichier JSON se charge sans erreur
- [ ] Tous les calques sont visibles
- [ ] Les couleurs s'affichent correctement
- [ ] Les tilesets utilisés sont documentés
- [ ] Le fichier JSON est bien formaté (utilisez `python3 test_format.py`)
- [ ] Les métadonnées sont à jour (nom du niveau)

## 🎮 Export vers votre jeu

Une fois votre niveau terminé et sauvegardé :

1. Le fichier JSON peut être chargé directement dans votre jeu (Pygame, Bevy, etc.)
2. Aucune conversion nécessaire
3. Format standardisé et documenté

Consultez le README.md pour les exemples d'intégration !

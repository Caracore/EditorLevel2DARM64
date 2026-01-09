# Guide du Sélecteur de Couleur

## 🎨 Utilisation du Sélecteur de Couleur

### Méthode 1 : Sliders RGB

1. Dans le panneau latéral gauche, trouvez la section "🎨 Sélecteur de Couleur"
2. Ajustez les trois sliders :
   - **Rouge** : 0-255
   - **Vert** : 0-255
   - **Bleu** : 0-255
3. L'aperçu se met à jour en temps réel
4. Le code hexadécimal s'affiche automatiquement
5. Cliquez sur "✏️ Utiliser cette couleur" pour commencer à peindre

### Méthode 2 : Code Hexadécimal

1. Dans le champ "Code HEX", entrez directement :
   - Format complet : `#FF5733` (6 caractères)
   - Format court : `#F53` (3 caractères, équivalent à #FF5533)
2. Appuyez sur Entrée ou cliquez ailleurs
3. Les sliders RGB se mettent à jour automatiquement
4. Cliquez sur "✏️ Utiliser cette couleur"

**Exemples de codes hex :**
- `#FF0000` = Rouge pur
- `#00FF00` = Vert pur
- `#0000FF` = Bleu pur
- `#8B4513` = Marron (SaddleBrown)
- `#87CEEB` = Bleu ciel (SkyBlue)
- `#FFD700` = Or (Gold)
- `#FFC0CB` = Rose (Pink)

### Méthode 3 : Palette Prédéfinie

La section "Couleurs prédéfinies" propose 12 couleurs courantes :
- **Marron (Sol)** - #8B4513
- **Gris (Mur)** - #808080
- **Beige (Plateforme)** - #CD853F
- **Rouge (Piège)** - #FF0000
- **Jaune (Pièce)** - #FFD700
- **Vert (Départ)** - #00FF00
- **Bleu ciel (Sortie)** - #00BFFF
- **Noir** - #000000
- **Blanc** - #FFFFFF
- **Rose** - #FFC0CB
- **Violet** - #800080
- **Orange** - #FFA500

Cliquez simplement sur le nom de la couleur pour l'utiliser immédiatement !

## 🖌️ Workflow Recommandé

### Pour un niveau RPG classique :

1. **Définir la palette du niveau** :
   - Sol : #8B4513 (Marron)
   - Herbe : #228B22 (Vert forêt)
   - Eau : #1E90FF (Bleu)
   - Murs : #696969 (Gris foncé)

2. **Créer des calques thématiques** :
   - Calque "Terrain" : Couleurs de base (marron, vert)
   - Calque "Eau" : Bleu pour rivières/lacs
   - Calque "Décorations" : Couleurs variées
   - Calque "Gameplay" : Objets interactifs (pièces, etc.)

3. **Utiliser les textures pour les détails** :
   - Chargez un tileset pour les éléments complexes
   - Gardez les couleurs pour les zones larges

## 💡 Astuces

### Créer des dégradés
Variez légèrement les valeurs RGB pour créer des effets de profondeur :
- Sol foncé : [100, 50, 20]
- Sol normal : [139, 69, 19]
- Sol clair : [170, 100, 40]

### Palettes cohérentes
Utilisez des sites comme :
- **Coolors.co** : Générateur de palettes
- **Adobe Color** : Roue chromatique
- **Paletton.com** : Schémas de couleurs harmonieux

Copiez les codes hex générés et collez-les dans l'éditeur !

### Sauvegarder vos couleurs
Créez un fichier texte avec vos palettes favorites :
```
# Ma Palette RPG
Herbe : #228B22
Eau : #1E90FF
Sable : #F4A460
Pierre : #696969
Or : #FFD700
```

## 🔧 Format dans le JSON

Les couleurs sont sauvegardées en RGB :

```json
"tiles": {
  "0,0": {"Color": [139, 69, 19]},  // Marron
  "1,0": {"Color": [255, 0, 0]},    // Rouge
  "2,0": {"Color": [0, 255, 0]}     // Vert
}
```

**Conversion hex → RGB :**
- #8B4513 → [139, 69, 19]
- #FF0000 → [255, 0, 0]
- #00FF00 → [0, 255, 0]

## 🎮 Utilisation dans les jeux

### Pygame
```python
rgb = tile_data["Color"]  # [R, G, B]
pygame.draw.rect(screen, rgb, rect)
```

### Bevy
```rust
let rgb = tile_data.Color;
Color::rgb_u8(rgb[0], rgb[1], rgb[2])
```

### Godot
```gdscript
var rgb = tile_data["Color"]
var color = Color8(rgb[0], rgb[1], rgb[2])
```

Votre éditeur stocke maintenant les couleurs de manière universelle, compatible avec tous les moteurs de jeu ! 🚀

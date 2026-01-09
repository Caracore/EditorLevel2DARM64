# 📦 Guide : Projets avec Tilesets

## 🎯 Problème Résolu

**Avant** : Quand vous chargiez un fichier `.json`, les couleurs apparaissaient mais les tilesets (images) ne se rechargeaient pas automatiquement.

**Maintenant** : Utilisez le format `.editorproj` qui sauvegarde à la fois le niveau ET les références aux tilesets !

---

## 📂 Types de Fichiers

### 1️⃣ `.editorproj` - Projet Complet (RECOMMANDÉ)
- ✅ Sauvegarde le niveau
- ✅ Sauvegarde les chemins vers les tilesets
- ✅ Recharge automatiquement les tilesets
- ✅ Format JSON lisible

### 2️⃣ `.json` - Niveau seul
- ✅ Sauvegarde uniquement le niveau
- ❌ NE sauvegarde PAS les tilesets
- ⚠️ Les textures ne se rechargeront pas

---

## 💾 Sauvegarder un Projet

### Menu Fichier → Sauvegarder

**📦 Projet Complet (.editorproj)** ← Utilisez ceci !
- Sauvegarde tout : niveau + liste des tilesets utilisés
- Extension : `.editorproj`
- Exemple : `mon_donjon.editorproj`

**📄 Niveau seul (.json)**
- Pour compatibilité ou export uniquement
- Les tilesets devront être rechargés manuellement

---

## 📂 Charger un Projet

### Menu Fichier → Charger

**📦 Projet Complet (.editorproj)**
- ✅ Charge le niveau ET les tilesets automatiquement
- ✅ Vous pouvez continuer à éditer immédiatement
- ✅ Notification avec nombre de tilesets chargés

**📄 Niveau seul (.json)**
- ✅ Charge uniquement le niveau
- ⚠️ Avertissement : "Tilesets non chargés"
- 🔧 Vous devrez recharger les tilesets manuellement

---

## 📋 Format du Fichier .editorproj

```json
{
  "version": "1.0",
  "level": {
    "name": "Mon Niveau",
    "tile_size": 16,
    "width": 64,
    "height": 48,
    "layers": [
      {
        "name": "Sol",
        "visible": true,
        "opacity": 1.0,
        "tiles": {
          "10,5": {"Color": [100, 150, 200]},
          "11,5": {"Texture": {"tileset_id": 0, "tile_index": 42}}
        }
      }
    ]
  },
  "tilesets": [
    {
      "id": 0,
      "name": "tileset_dungeon.png",
      "path": "/chemin/vers/tileset_dungeon.png",
      "tile_width": 16,
      "tile_height": 16,
      "columns": 16,
      "rows": 16
    }
  ]
}
```

---

## 🔄 Workflow Recommandé

### 1️⃣ Démarrer un Nouveau Projet
1. Menu **Fichier → Nouveau**
2. Menu **Assets → Charger Tileset** (autant que nécessaire)
3. Dessinez votre niveau
4. **Sauvegardez en `.editorproj`** ✅

### 2️⃣ Continuer un Projet Existant
1. Menu **Fichier → Charger → Projet Complet (.editorproj)**
2. Tout se charge automatiquement ! 🎉
3. Continuez à éditer
4. Sauvegardez à nouveau en `.editorproj`

### 3️⃣ Exporter pour un Jeu
1. Ouvrez votre `.editorproj`
2. Menu **Fichier → Sauvegarder → Niveau seul (.json)**
3. Copiez le `.json` dans votre projet de jeu
4. Copiez aussi les fichiers de tilesets nécessaires

---

## ⚠️ Points Importants

### Chemins des Tilesets
- Les chemins des tilesets sont **absolus** (chemin complet)
- Si vous déplacez vos tilesets, rechargez-les manuellement
- Alternative : gardez vos tilesets dans un dossier fixe

### Compatibilité
- ✅ Vous pouvez ouvrir d'anciens `.json`
- ✅ Ils se chargeront (sans tilesets)
- ✅ Ajoutez vos tilesets manuellement
- ✅ Sauvegardez en `.editorproj` pour la prochaine fois

### Performance
- Le fichier `.editorproj` contient uniquement les **chemins** vers les images
- Les images ne sont pas embarquées (le fichier reste léger)
- Si une image n'existe plus, vous verrez un avertissement

---

## 💡 Exemples d'Utilisation

### Scénario 1 : Premier Projet
```
1. Nouveau
2. Charger tileset_terrain.png
3. Charger tileset_objets.png
4. Dessiner la carte
5. Sauvegarder → mon_niveau.editorproj ✅
6. Fermer l'éditeur
7. Rouvrir → Charger → mon_niveau.editorproj
   → Tout se recharge ! 🎉
```

### Scénario 2 : Export pour Pygame
```
1. Charger → mon_niveau.editorproj
2. Vérifier que tout est OK
3. Sauvegarder → niveau_export.json
4. Copier niveau_export.json dans le dossier du jeu
5. Copier tileset_terrain.png et tileset_objets.png
6. Votre jeu peut maintenant lire le JSON
```

### Scénario 3 : Migration depuis .json
```
1. Charger → ancien_niveau.json (sans tilesets)
2. Voir l'avertissement "Tilesets non chargés"
3. Menu Assets → Charger les tilesets nécessaires
4. Sauvegarder → ancien_niveau.editorproj ✅
5. Supprimer ancien_niveau.json (optionnel)
6. Désormais, utilisez toujours le .editorproj
```

---

## 🚀 Avantages du Format .editorproj

| Avantage | Description |
|----------|-------------|
| 🔄 **Reprise rapide** | Rechargez tout en un clic |
| 💾 **Aucune perte** | Toutes les textures et couleurs |
| 📝 **JSON lisible** | Format texte, facile à versionner |
| 🎨 **Multi-tilesets** | Support illimité de tilesets |
| ⚡ **Léger** | Seulement les chemins, pas les images |

---

## 📞 En Cas de Problème

### "Tileset non chargé : ..."
- Le fichier image a été déplacé ou supprimé
- Solution : Rechargez-le manuellement via Menu Assets

### "⚠️ Projet chargé : X/Y tilesets"
- Certains tilesets n'ont pas pu être rechargés
- Les tiles utilisant ces tilesets seront invisibles
- Vérifiez les chemins des fichiers manquants

### Mon ancien .json ne fonctionne plus
- Les `.json` fonctionnent toujours !
- Ils chargent uniquement les couleurs (c'est normal)
- Rechargez vos tilesets manuellement
- Sauvegardez en `.editorproj` pour la prochaine fois

---

**🎉 Astuce** : Toujours utiliser `.editorproj` pendant le développement, et exporter en `.json` uniquement pour votre moteur de jeu final !

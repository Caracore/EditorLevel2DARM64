# 🔄 Guide de Migration : .json → .editorproj

## 📌 Pourquoi Migrer ?

### Problème avec `.json`
Quand vous chargez un fichier `.json`, voici ce qui se passe :
- ✅ Les **couleurs** se chargent correctement
- ❌ Les **tilesets** (images) ne se chargent PAS
- 😞 Vous devez **recharger manuellement** tous vos tilesets à chaque fois

### Solution : `.editorproj`
Avec le nouveau format `.editorproj` :
- ✅ Les **couleurs** se chargent
- ✅ Les **tilesets** se chargent automatiquement
- 🎉 Vous pouvez **reprendre votre travail immédiatement**

---

## 🚀 Migration en 3 Étapes

### Étape 1 : Charger votre ancien .json

1. Ouvrir l'éditeur
2. **Fichier** → **Charger** → **📄 Niveau seul (.json)**
3. Sélectionner votre ancien fichier `ma_map.json`
4. ⚠️ Vous verrez le message : _"Tilesets non chargés (utilisez .editorproj)"_

👉 **C'est normal !** Les couleurs sont là, mais pas les tilesets.

---

### Étape 2 : Recharger vos Tilesets

1. Aller dans **Assets** (panneau latéral)
2. Cliquer sur **➕ Charger Tileset**
3. Sélectionner chaque tileset que vous aviez utilisé
4. Répéter pour tous les tilesets nécessaires

💡 **Astuce** : Regardez votre niveau pour voir quels tilesets manquent (les textures apparaîtront au fur et à mesure)

---

### Étape 3 : Sauvegarder en .editorproj

1. **Fichier** → **Sauvegarder** → **📦 Projet Complet (.editorproj)**
2. Donner un nom : `ma_map.editorproj`
3. Cliquer sur **Enregistrer**
4. ✅ Notification : _"Projet sauvegardé : ma_map.editorproj (avec X tilesets)"_

🎉 **C'est fait !** Vous n'aurez plus jamais à recharger ces tilesets manuellement.

---

## 📂 Avant / Après

### Avant (Workflow .json) 😓
```
1. Ouvrir l'éditeur
2. Charger niveau.json
3. ⚠️ Recharger tileset_sol.png
4. ⚠️ Recharger tileset_murs.png
5. ⚠️ Recharger tileset_objets.png
6. Enfin, éditer...
7. Sauvegarder niveau.json
8. Fermer
9. Recommencer les étapes 3-5 la prochaine fois ! 😩
```

### Après (Workflow .editorproj) 😎
```
1. Ouvrir l'éditeur
2. Charger mon_projet.editorproj
3. ✅ Tout est chargé automatiquement !
4. Éditer
5. Sauvegarder mon_projet.editorproj
6. Fermer
7. La prochaine fois : étapes 2-5, c'est tout ! 🎉
```

---

## 🔄 Migration en Masse

Si vous avez plusieurs fichiers `.json` :

### Option A : Migration Progressive
- Migrez au fur et à mesure
- Quand vous ouvrez un `.json`, rechargez les tilesets et sauvegardez en `.editorproj`
- Gardez les deux formats si besoin

### Option B : Migration Complète
1. Listez tous vos fichiers `.json`
2. Pour chaque fichier :
   - Ouvrez-le
   - Rechargez les tilesets
   - Sauvegardez en `.editorproj`
3. Organisez vos anciens `.json` dans un dossier `old_format/`

---

## 📊 Comparaison des Formats

| Critère | .json | .editorproj |
|---------|-------|-------------|
| **Sauvegarde niveau** | ✅ | ✅ |
| **Sauvegarde tilesets** | ❌ | ✅ |
| **Rechargement auto** | ❌ | ✅ |
| **Export pour jeu** | ✅ | ✅ (via JSON) |
| **Lisibilité** | ✅ | ✅ |
| **Taille fichier** | Petit | Petit+ |

---

## 🎯 Cas d'Usage

### Utilisez `.editorproj` pour :
- ✅ Développement actif de vos maps
- ✅ Projets avec plusieurs tilesets
- ✅ Travail quotidien sur un niveau
- ✅ Collaboration (versioning Git)

### Utilisez `.json` pour :
- ✅ Export final vers votre moteur de jeu
- ✅ Partage d'un niveau (sans les assets)
- ✅ Niveaux ne contenant que des couleurs
- ✅ Compatibilité avec outils externes

---

## ❓ FAQ

### Q : Puis-je garder mes anciens .json ?
**R :** Oui ! Les `.json` restent compatibles. Vous pouvez les ouvrir à tout moment et les migrer quand vous voulez.

### Q : Le .editorproj est-il plus lourd ?
**R :** Très légèrement. Il contient juste les chemins des tilesets (quelques lignes de texte en plus). Les images ne sont pas embarquées.

### Q : Et si je déplace mes tilesets ?
**R :** Si vous déplacez un fichier tileset, l'éditeur ne pourra pas le recharger automatiquement. Vous verrez un avertissement. Rechargez-le manuellement et re-sauvegardez le projet.

### Q : Puis-je éditer le .editorproj à la main ?
**R :** Oui ! C'est du JSON standard. Vous pouvez modifier les chemins des tilesets, la version, etc.

### Q : Mon jeu peut-il lire les .editorproj ?
**R :** Non, utilisez l'export `.json` pour votre jeu. Le `.editorproj` est uniquement pour l'éditeur. Workflow :
1. Développez avec `.editorproj`
2. Exportez en `.json` quand le niveau est terminé
3. Utilisez le `.json` dans votre jeu

### Q : Les deux formats peuvent coexister ?
**R :** Absolument ! Vous pouvez avoir `niveau_v1.json` et `niveau_v1.editorproj` dans le même dossier. Utilisez le `.editorproj` pour éditer et gardez le `.json` pour votre jeu.

---

## 📝 Exemple Pratique

### Fichier original : `donjon1.json`
```json
{
  "name": "Donjon 1",
  "layers": [
    {
      "name": "Sol",
      "tiles": {
        "5,5": {"Texture": {"tileset_id": 0, "tile_index": 10}},
        "6,5": {"Color": [139, 69, 19]}
      }
    }
  ]
}
```

### Après migration : `donjon1.editorproj`
```json
{
  "version": "1.0",
  "level": {
    "name": "Donjon 1",
    "layers": [
      {
        "name": "Sol",
        "tiles": {
          "5,5": {"Texture": {"tileset_id": 0, "tile_index": 10}},
          "6,5": {"Color": [139, 69, 19]}
        }
      }
    ]
  },
  "tilesets": [
    {
      "id": 0,
      "name": "tileset_donjon.png",
      "path": "/home/user/assets/tileset_donjon.png",
      "tile_width": 16,
      "tile_height": 16,
      "columns": 16,
      "rows": 16
    }
  ]
}
```

**Différence** : La section `tilesets` permet de recharger automatiquement `tileset_donjon.png` !

---

## ✅ Checklist de Migration

- [ ] J'ai ouvert mon ancien fichier `.json`
- [ ] J'ai rechargé tous les tilesets nécessaires
- [ ] J'ai vérifié que toutes les textures s'affichent
- [ ] J'ai sauvegardé en format `.editorproj`
- [ ] J'ai testé en rechargeant le `.editorproj`
- [ ] Tout fonctionne automatiquement ! 🎉

---

**🎊 Félicitations !** Vous utilisez maintenant le format moderne `.editorproj`. Votre productivité va exploser ! 🚀

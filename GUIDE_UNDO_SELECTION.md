# Guide Undo/Redo et Sélection

## 🔄 Système d'Annulation (Undo/Redo)

### Raccourcis Clavier
- **Ctrl+Z** : Annuler la dernière action
- **Ctrl+Y** : Rétablir une action annulée

### Fonctionnement
Le système conserve un historique des **50 dernières modifications** sur vos calques.

#### Actions qui créent un point d'historique :
- Dessiner avec le pinceau (premier clic d'une série)
- Effacer des tiles
- Remplir en ligne
- Remplir en rectangle
- Coller une sélection
- Supprimer une sélection

#### Indicateurs visuels
Dans le panneau latéral, deux symboles indiquent l'état :
- **↶** vert : Des actions peuvent être annulées
- **↶** gris : Rien à annuler
- **↷** vert : Des actions peuvent être rétablies
- **↷** gris : Rien à rétablir

### Exemples d'utilisation

#### Corriger une erreur rapide
1. Vous dessinez un mur au mauvais endroit
2. **Ctrl+Z** → Le mur disparaît
3. Dessinez au bon endroit

#### Tester plusieurs variations
1. Créez un motif
2. **Ctrl+Z** pour l'annuler
3. Créez un autre motif
4. **Ctrl+Z** pour le comparer
5. **Ctrl+Y** pour revenir au deuxième
6. **Ctrl+Y** encore pour revenir au troisième

#### Annuler plusieurs actions
Appuyez plusieurs fois sur **Ctrl+Z** pour remonter dans l'historique :
- 1× Ctrl+Z : Annule la dernière action
- 2× Ctrl+Z : Annule les 2 dernières actions
- 3× Ctrl+Z : Annule les 3 dernières actions
- etc.

### Important
⚠️ **Nouvelle action = Perte du Redo** : Si vous annulez des actions puis faites une nouvelle modification, vous ne pourrez plus rétablir les actions annulées.

---

## 📦 Mode Sélection

### Activation
Cliquez sur l'outil **📦 Sélection** dans le panneau latéral.

### Sélectionner une Zone

#### Méthode 1 : Glisser-Déposer
1. Cliquez et maintenez le bouton gauche sur une case
2. Glissez jusqu'à la case opposée
3. Relâchez : la zone est sélectionnée (contour jaune)

#### Méthode 2 : Deux Clics
1. Cliquez sur un coin
2. Cliquez sur le coin opposé
3. La zone est sélectionnée

### Actions sur la Sélection

#### 📋 Copier (Ctrl+C)
1. Sélectionnez une zone
2. **Ctrl+C** → La zone est copiée dans le presse-papier
3. Notification : "📋 X tiles copiés"

Le panneau latéral affiche : **📋 Presse-papier plein**

#### 📌 Coller (Ctrl+V)
1. Ayez du contenu dans le presse-papier (Ctrl+C)
2. Positionnez la souris où vous voulez coller
3. **Ctrl+V** → Le contenu est collé à cette position
4. Notification : "✅ X tiles collés"

💡 **Astuce** : Vous pouvez coller plusieurs fois !

#### 🗑️ Supprimer (Delete)
1. Sélectionnez une zone
2. Appuyez sur **Delete**
3. Tous les tiles de la zone deviennent vides
4. Notification : "🗑️ X tiles supprimés"

#### ❌ Annuler la Sélection
- **Clic droit** : Annule la sélection en cours
- **Échap** : Annule la sélection active

### Visualisation
- **Sélection en cours** : Contour jaune simple
- **Sélection active** : Contour jaune épais + grille
- Le nombre de tiles sélectionnés s'affiche en notification

### Cas d'Usage

#### Dupliquer un élément
```
1. Dessinez un arbre
2. Sélectionnez l'arbre (outil Sélection)
3. Ctrl+C (copier)
4. Déplacez la souris ailleurs
5. Ctrl+V (coller)
6. Ctrl+V encore pour un 3ème arbre
7. etc.
```

#### Déplacer une zone
```
1. Sélectionnez la zone à déplacer
2. Ctrl+C (copier)
3. Delete (supprimer l'original)
4. Positionnez la souris à la nouvelle position
5. Ctrl+V (coller)
```

#### Créer un motif répétitif
```
1. Créez une tuile de base (ex: dalles 2x2)
2. Sélectionnez ces 4 tiles
3. Ctrl+C
4. Collez (Ctrl+V) à côté → 2x répétition
5. Sélectionnez les 8 tiles
6. Ctrl+C
7. Collez plusieurs fois → 4x, 8x, 16x répétition !
```

#### Nettoyer une grande zone
```
1. Sélectionnez la zone à nettoyer
2. Delete
3. Toute la zone est vidée instantanément
```

---

## 🎯 Raccourcis Globaux Récapitulatif

| Raccourci | Action | Contexte |
|-----------|--------|----------|
| **Ctrl+Z** | Annuler | Toujours disponible |
| **Ctrl+Y** | Rétablir | Toujours disponible |
| **Ctrl+C** | Copier | Mode Sélection avec zone active |
| **Ctrl+V** | Coller | Mode Sélection avec presse-papier |
| **Delete** | Supprimer | Mode Sélection avec zone active |
| **Échap** | Annuler sélection | Mode Sélection |
| **Clic droit** | Annuler/Gomme | Selon l'outil |
| **Molette** | Zoom | Sur le canvas |
| **Clic molette** | Déplacer vue | Sur le canvas |

---

## 💡 Astuces et Bonnes Pratiques

### Workflow Efficace
1. **Sauvegardez régulièrement** : Ctrl+S n'annule pas l'historique
2. **Testez sans crainte** : Ctrl+Z est votre filet de sécurité
3. **Utilisez les calques** : Travaillez sur différents calques pour faciliter la sélection

### Combiner les Outils
- Utilisez **Remplissage Rectangle** pour les grandes zones uniformes
- Utilisez **Sélection + Copier/Coller** pour les motifs complexes
- Utilisez **Pinceau** pour les détails finaux

### Performance
- L'historique conserve 50 étapes maximum
- Au-delà, les anciennes modifications sont automatiquement effacées
- Cela n'affecte pas vos sauvegardes de fichiers

### Limitations
- L'historique est **par session** : fermer le programme réinitialise l'historique
- Le presse-papier contient **une seule sélection** à la fois
- Les sélections fonctionnent **sur un seul calque** à la fois

---

## 🐛 Dépannage

### "Rien à annuler"
- Vous êtes au début de l'historique
- Ou vous venez d'ouvrir le programme

### "Rien à rétablir"
- Vous êtes à la fin de l'historique
- Ou vous avez fait une nouvelle action après avoir annulé

### "Aucune sélection active"
- Vous devez sélectionner une zone avant de copier/supprimer
- Vérifiez que le contour jaune est bien visible

### "Presse-papier vide"
- Vous devez copier (Ctrl+C) avant de coller (Ctrl+V)
- Le presse-papier se vide si vous fermez le programme

### La sélection ne se voit pas bien
- Augmentez le zoom (molette)
- Vérifiez que vous êtes sur le bon calque
- Le contour jaune devrait être clairement visible

---

## 🎓 Tutoriel Pratique

### Exercice 1 : Maîtriser Undo/Redo
1. Dessinez 3 carrés de couleurs différentes
2. Ctrl+Z trois fois → Les carrés disparaissent
3. Ctrl+Y trois fois → Ils réapparaissent
4. Ctrl+Z deux fois → Le dernier carré disparaît
5. Dessinez un autre carré → Vous ne pouvez plus Ctrl+Y l'ancien

### Exercice 2 : Copier/Coller Basique
1. Créez un petit motif 3x3
2. Outil Sélection, sélectionnez-le
3. Ctrl+C
4. Déplacez la souris ailleurs
5. Ctrl+V
6. Recommencez Ctrl+V plusieurs fois

### Exercice 3 : Workflow Complet
1. Créez une tour 5x10 avec le pinceau
2. Oups, mauvaise couleur → Ctrl+Z
3. Changez de couleur
4. Remplissage Rectangle pour la base
5. Pinceau pour les détails
6. Sélectionnez toute la tour
7. Ctrl+C puis Ctrl+V → Dupliquez-la
8. Delete sur l'une pour ne garder que la meilleure

Vous êtes maintenant prêt à créer des maps efficacement ! 🎮

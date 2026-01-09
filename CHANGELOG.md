# 🎉 Changelog - Système de Projets avec Tilesets

## Version 2.0 - Janvier 2025

### 🆕 Nouvelles Fonctionnalités

#### 📦 Format de Projet `.editorproj` (MAJEUR)
- **Nouveau format de fichier** qui sauvegarde le niveau ET les tilesets ensemble
- **Rechargement automatique** : plus besoin de recharger les tilesets manuellement
- **Workflow professionnel** : reprenez votre travail là où vous l'avez laissé
- **Format JSON lisible** : facile à versionner avec Git
- **Rétrocompatible** : les anciens `.json` restent supportés

#### 💾 Système de Sauvegarde Amélioré
Menu **Fichier → Sauvegarder** propose maintenant deux options :
- **📦 Projet Complet (.editorproj)** ← Recommandé pour le développement
- **📄 Niveau seul (.json)** ← Pour export vers votre moteur de jeu

#### 📂 Système de Chargement Amélioré
Menu **Fichier → Charger** propose maintenant deux options :
- **📦 Projet Complet (.editorproj)** ← Charge tout automatiquement
- **📄 Niveau seul (.json)** ← Charge le niveau, avertit pour les tilesets

### 🔧 Améliorations Techniques

#### Nouveau Module : `project.rs`
- Structure `Project` contenant le niveau + métadonnées des tilesets
- Structure `TilesetMetadata` pour sauvegarder les infos des tilesets
- Méthodes `save_to_file()` et `load_from_file()` pour la persistance
- Versioning du format (actuellement "1.0")

#### Module `asset_manager.rs`
- Nouvelle méthode `get_metadata()` pour exporter les infos des tilesets
- Retourne un `Vec<TilesetMetadata>` avec tous les tilesets chargés
- Inclut : ID, nom, chemin absolu, dimensions, grille (colonnes/lignes)

#### Module `ui.rs`
- Menu **Fichier** restructuré avec sous-menus
- Section **Sauvegarder** avec deux options
- Section **Charger** avec deux options
- **Notifications enrichies** :
  - Nombre de tilesets sauvegardés
  - Nombre de tilesets rechargés
  - Avertissements si des tilesets manquent
  - Message clair pour les fichiers .json (tilesets non chargés)

### 📚 Nouvelle Documentation

#### GUIDE_PROJETS.md (NOUVEAU)
Guide complet du système de projets :
- Explication du problème résolu
- Différences entre `.editorproj` et `.json`
- Workflow recommandé
- Format détaillé des fichiers
- FAQ et exemples d'utilisation
- Points importants et bonnes pratiques

#### GUIDE_MIGRATION.md (NOUVEAU)
Guide de migration depuis l'ancien format :
- Pourquoi migrer
- Migration en 3 étapes faciles
- Comparaison avant/après
- Migration en masse
- Tableau comparatif des formats
- FAQ détaillée
- Exemples pratiques
- Checklist de migration

#### demo_projets.sh (NOUVEAU)
Script de démonstration interactif :
- Affichage du format `.editorproj`
- Explication des différences
- Workflow recommandé
- Avantages du nouveau système
- Compatibilité et documentation

#### exemple_projet.editorproj (NOUVEAU)
Fichier d'exemple pour tester le système :
- Projet simple avec couleurs
- Structure complète d'un `.editorproj`
- Peut être chargé directement dans l'éditeur

### 🐛 Corrections

#### Rechargement des Tilesets
- **RÉSOLU** : Les tilesets importés ne se rechargeaient pas avec le niveau
- **Solution** : Nouveau format `.editorproj` qui sauvegarde les références
- **Impact** : Les utilisateurs peuvent maintenant reprendre leur travail sans friction

#### Gestion des Erreurs
- Meilleurs messages d'erreur si un tileset est introuvable
- Affichage du nombre de tilesets chargés/échoués
- Warnings clairs dans les notifications

### 📋 Changements Non-Cassants

#### Compatibilité
- ✅ Les anciens fichiers `.json` fonctionnent toujours
- ✅ Vous pouvez ouvrir n'importe quel `.json`
- ✅ Conversion facile : ouvrir .json → recharger tilesets → sauver .editorproj
- ✅ Les deux formats peuvent coexister

#### Format JSON
- ✅ Le format `.json` est inchangé
- ✅ Toujours utilisable pour l'export
- ✅ Compatible avec tous les moteurs de jeu

### 🎯 Cas d'Usage

#### Pour le Développement
```
1. Créer un nouveau niveau
2. Charger des tilesets
3. Dessiner la map
4. Sauvegarder en .editorproj ✅
5. Fermer l'éditeur

Le lendemain :
6. Ouvrir le .editorproj
7. Tout se charge ! 🎉
8. Continuer l'édition
```

#### Pour l'Export
```
1. Ouvrir votre .editorproj
2. Finaliser le niveau
3. Sauvegarder en .json
4. Copier le .json dans votre jeu
5. Copier les tilesets nécessaires
6. Votre jeu charge le .json
```

### 📊 Statistiques

- **2 nouveaux modules** : `project.rs` (120 lignes)
- **1 nouveau format** : `.editorproj`
- **4 nouveaux guides** : GUIDE_PROJETS.md, GUIDE_MIGRATION.md, demo_projets.sh, exemple_projet.editorproj
- **1 méthode ajoutée** : `AssetManager::get_metadata()`
- **Menu restructuré** : Sous-menus pour Sauvegarder/Charger
- **Notifications améliorées** : Infos sur les tilesets
- **0 breaking changes** : Rétrocompatibilité totale

### 🚀 Performances

- **Taille fichier** : +~50-200 octets par tileset (seulement les métadonnées)
- **Temps de chargement** : Identique au chargement manuel
- **Utilisation mémoire** : Aucun changement
- **Format léger** : Les images ne sont pas embarquées

### 🔜 Améliorations Futures Possibles

Idées pour de futures versions :
- [ ] Chemins relatifs pour tilesets (portabilité)
- [ ] Compression ZIP du projet + assets (archive unique)
- [ ] Recherche automatique de tilesets déplacés
- [ ] Historique des projets récents
- [ ] Détection automatique des tilesets non utilisés
- [ ] Export vers différents formats (Tiled, Godot, etc.)

---

## 🎊 Remerciements

Merci à l'utilisateur pour avoir signalé le problème de persistance des tilesets ! Cette fonctionnalité améliore grandement l'expérience utilisateur et le workflow de développement.

---

**Date** : 9 janvier 2025  
**Version** : 2.0  
**Plateforme** : ARM64 Linux  
**Framework** : eframe/egui 0.29  
**Statut** : ✅ Stable et Testé

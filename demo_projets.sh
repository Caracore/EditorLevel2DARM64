#!/bin/bash

# 🎨 Démonstration du Système de Projets .editorproj
# Ce script montre comment utiliser le nouveau format de projet

echo "========================================"
echo "🎮 DÉMO : Système de Projets .editorproj"
echo "========================================"
echo ""

# Couleurs pour l'affichage
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 1. Afficher le fichier d'exemple
echo -e "${BLUE}📦 Contenu d'un fichier .editorproj :${NC}"
echo ""
cat exemple_projet.editorproj | head -30
echo "..."
echo ""
echo "==========================================="
echo ""

# 2. Différences avec .json
echo -e "${YELLOW}🔄 Différence avec le format .json :${NC}"
echo ""
echo "Format .json (ancien) :"
echo "  ❌ Sauvegarde uniquement le niveau"
echo "  ❌ Les tilesets se perdent au rechargement"
echo "  ⚠️  Il faut recharger les images manuellement"
echo ""
echo "Format .editorproj (nouveau) :"
echo "  ✅ Sauvegarde le niveau ET les tilesets"
echo "  ✅ Rechargement automatique complet"
echo "  ✅ Workflow professionnel sans friction"
echo ""
echo "==========================================="
echo ""

# 3. Structure du projet
echo -e "${GREEN}📂 Structure d'un fichier .editorproj :${NC}"
echo ""
echo "  {
    \"version\": \"1.0\",           ← Version du format
    \"level\": {                   ← Données du niveau
      \"name\": \"...\",
      \"layers\": [...]
    },
    \"tilesets\": [                ← NOUVEAU ! Liste des tilesets
      {
        \"id\": 0,
        \"name\": \"tileset.png\",
        \"path\": \"/chemin/absolu/tileset.png\",
        \"tile_width\": 16,
        \"tile_height\": 16,
        \"columns\": 16,
        \"rows\": 16
      }
    ]
  }"
echo ""
echo "==========================================="
echo ""

# 4. Workflow recommandé
echo -e "${BLUE}🚀 Workflow recommandé :${NC}"
echo ""
echo "1️⃣  Nouveau projet :"
echo "   - Fichier → Nouveau"
echo "   - Assets → Charger Tileset (autant que nécessaire)"
echo "   - Dessiner votre map"
echo "   - Sauvegarder → 📦 Projet Complet (.editorproj)"
echo ""
echo "2️⃣  Reprendre un projet :"
echo "   - Charger → 📦 Projet Complet (.editorproj)"
echo "   - Tout se charge automatiquement ! 🎉"
echo "   - Continuer l'édition"
echo ""
echo "3️⃣  Exporter pour votre jeu :"
echo "   - Ouvrir le .editorproj"
echo "   - Sauvegarder → 📄 Niveau seul (.json)"
echo "   - Copier le .json + les tilesets dans votre jeu"
echo ""
echo "==========================================="
echo ""

# 5. Avantages
echo -e "${GREEN}✨ Avantages du format .editorproj :${NC}"
echo ""
echo "  🔄 Reprise rapide du travail"
echo "  💾 Aucune perte de données"
echo "  📝 Format JSON lisible"
echo "  🎨 Support multi-tilesets illimité"
echo "  ⚡ Fichier léger (chemins uniquement)"
echo "  🎮 Compatible avec l'export .json"
echo ""
echo "==========================================="
echo ""

# 6. Compatibilité
echo -e "${YELLOW}📌 Compatibilité :${NC}"
echo ""
echo "  ✅ Vous pouvez toujours ouvrir des .json"
echo "  ✅ Les .json se chargent (sans tilesets)"
echo "  ✅ Ajoutez vos tilesets manuellement"
echo "  ✅ Puis sauvegardez en .editorproj"
echo ""
echo "  → Migration facile depuis l'ancien format !"
echo ""
echo "==========================================="
echo ""

# 7. Guides disponibles
echo -e "${BLUE}📚 Documentation disponible :${NC}"
echo ""
echo "  📖 GUIDE_PROJETS.md       ← Guide complet du système"
echo "  📖 GUIDE_CHARGER.md       ← Détails sur save/load"
echo "  📖 GUIDE_COULEURS.md      ← Utiliser le color picker"
echo "  📖 GUIDE_CONFIGURATION.md ← Canvas et calques"
echo "  📖 README.md              ← Documentation générale"
echo ""
echo "==========================================="
echo ""

echo -e "${GREEN}✅ Démo terminée !${NC}"
echo ""
echo "Lancez l'éditeur et testez le nouveau système :"
echo "  ./target/release/editor_level"
echo ""
echo "Créez une map, chargez des tilesets, et sauvegardez"
echo "en format .editorproj pour profiter du rechargement"
echo "automatique ! 🎉"
echo ""

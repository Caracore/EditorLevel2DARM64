#!/bin/bash
# Script de démonstration : Créer, Sauvegarder, Charger

echo "=========================================="
echo "🧪 Démonstration : Workflow complet"
echo "=========================================="
echo ""

# Vérifier que l'éditeur est compilé
if [ ! -f "target/release/editor_level" ]; then
    echo "❌ Erreur : L'éditeur n'est pas compilé"
    echo "Lancez : cargo build --release"
    exit 1
fi

echo "✓ Éditeur compilé trouvé"
echo ""

# Vérifier les fichiers d'exemple
echo "📁 Fichiers d'exemple disponibles :"
echo ""
for file in *.json; do
    if [ -f "$file" ]; then
        echo "  - $file"
        tiles=$(grep -o '"tiles"' "$file" | wc -l)
        layers=$(grep -o '"name":' "$file" | wc -l)
        echo "    ($layers calques détectés)"
    fi
done

echo ""
echo "=========================================="
echo "📋 Instructions de test"
echo "=========================================="
echo ""
echo "1️⃣  Lancer l'éditeur :"
echo "    ./target/release/editor_level"
echo ""
echo "2️⃣  Tester le chargement :"
echo "    - Fichier → 📂 Charger"
echo "    - Sélectionner 'exemple_couleurs_hex.json'"
echo "    - Vérifier que la map s'affiche"
echo "    - Observer la notification en haut"
echo ""
echo "3️⃣  Modifier la map :"
echo "    - Ajouter quelques tiles"
echo "    - Changer de calque"
echo "    - Modifier les couleurs"
echo ""
echo "4️⃣  Sauvegarder :"
echo "    - Fichier → 💾 Sauvegarder"
echo "    - Nom : 'test_edition.json'"
echo "    - Vérifier la notification"
echo ""
echo "5️⃣  Recharger pour vérifier :"
echo "    - Fichier → 📂 Charger"
echo "    - Sélectionner 'test_edition.json'"
echo "    - Tout doit être identique !"
echo ""
echo "=========================================="
echo "✅ Test réussi si :"
echo "=========================================="
echo ""
echo "  ✓ Le chargement affiche une notification"
echo "  ✓ Tous les calques sont présents"
echo "  ✓ Toutes les tiles sont restaurées"
echo "  ✓ Les couleurs RGB sont correctes"
echo "  ✓ Le nom du fichier apparaît en bas"
echo "  ✓ La vue est réinitialisée (zoom 100%)"
echo ""
echo "🚀 Prêt à tester ! Lancez l'éditeur maintenant."
echo ""

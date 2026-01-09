#!/bin/bash
# Script de test pour vérifier que l'installation a réussi

echo "🧪 Test de l'installation EditorLevel2D"
echo "========================================"
echo ""

# 1. Vérifier que les scripts existent et sont exécutables
echo "[1/5] Vérification des scripts..."
if [ -x "install.sh" ] && [ -x "uninstall.sh" ]; then
    echo "✓ Scripts d'installation présents et exécutables"
else
    echo "✗ Erreur: Les scripts ne sont pas exécutables"
    echo "  Exécutez: chmod +x install.sh uninstall.sh"
    exit 1
fi

# 2. Vérifier que les parsers existent
echo "[2/5] Vérification des parsers..."
if [ -f "parsers/pygame_parser.py" ] && [ -f "parsers/bevy_parser.rs" ]; then
    echo "✓ Parsers Pygame et Bevy présents"
else
    echo "✗ Erreur: Parsers manquants"
    exit 1
fi

# 3. Vérifier que la documentation existe
echo "[3/5] Vérification de la documentation..."
if [ -f "GUIDE_PARSERS.md" ] && [ -f "parsers/README.md" ]; then
    echo "✓ Documentation complète présente"
else
    echo "✗ Erreur: Documentation manquante"
    exit 1
fi

# 4. Vérifier que Cargo.toml existe
echo "[4/5] Vérification du projet Rust..."
if [ -f "Cargo.toml" ]; then
    echo "✓ Projet Rust configuré"
else
    echo "✗ Erreur: Cargo.toml manquant"
    exit 1
fi

# 5. Test de compilation (optionnel)
echo "[5/5] Test de compilation (optionnel)..."
if command -v cargo &> /dev/null; then
    echo "  Rust détecté. Voulez-vous tester la compilation? (o/N)"
    read -t 5 -n 1 -r REPLY || REPLY="n"
    echo
    if [[ $REPLY =~ ^[Oo]$ ]]; then
        echo "  Compilation en cours..."
        if cargo check --quiet; then
            echo "✓ Compilation réussie"
        else
            echo "✗ Erreur de compilation"
            exit 1
        fi
    else
        echo "⊘ Test de compilation ignoré"
    fi
else
    echo "⊘ Rust non installé, test de compilation ignoré"
fi

echo ""
echo "========================================"
echo "✅ Tous les tests sont passés!"
echo "========================================"
echo ""
echo "📖 Prochaines étapes:"
echo "  1. Installer avec: ./install.sh"
echo "  2. Lancer avec: editor_level"
echo "  3. Voir la doc: cat GUIDE_PARSERS.md"
echo ""

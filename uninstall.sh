#!/bin/bash
# Script de désinstallation pour EditorLevel2D sur Raspberry Pi OS ARM64

set -e  # Arrêter en cas d'erreur

# Couleurs pour les messages
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE} Désinstallation de EditorLevel2D ARM64${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

# Variables de chemins
INSTALL_DIR="$HOME/.local/bin"
DATA_DIR="$HOME/.local/share/editor_level"
DESKTOP_DIR="$HOME/.local/share/applications"
BINARY="$INSTALL_DIR/editor_level"
DESKTOP_FILE="$DESKTOP_DIR/editor_level.desktop"

# Fonction pour demander confirmation
confirm() {
    read -p "$1 (o/N) " -n 1 -r
    echo
    [[ $REPLY =~ ^[Oo]$ ]]
}

# Vérifier si le programme est installé
if [ ! -f "$BINARY" ]; then
    echo -e "${YELLOW}EditorLevel2D ne semble pas être installé dans $INSTALL_DIR${NC}"
    echo -e "${YELLOW}Vérification des autres emplacements...${NC}"
    
    # Chercher le binaire dans d'autres emplacements courants
    if command -v editor_level >/dev/null 2>&1; then
        BINARY=$(which editor_level)
        echo -e "${GREEN}Trouvé dans: $BINARY${NC}"
    else
        echo -e "${RED}Aucune installation trouvée.${NC}"
        exit 1
    fi
fi

echo -e "${BLUE}Installation détectée:${NC}"
echo -e "  Binaire: $BINARY"
[ -d "$DATA_DIR" ] && echo -e "  Données: $DATA_DIR"
[ -f "$DESKTOP_FILE" ] && echo -e "  Raccourci menu: $DESKTOP_FILE"
echo ""

# Confirmation de la désinstallation
if ! confirm "Voulez-vous désinstaller EditorLevel2D?"; then
    echo -e "${YELLOW}Désinstallation annulée.${NC}"
    exit 0
fi

echo ""
echo -e "${GREEN}[1/4] Suppression du binaire...${NC}"
if [ -f "$BINARY" ]; then
    rm -f "$BINARY"
    echo -e "${GREEN}Binaire supprimé.${NC}"
else
    echo -e "${YELLOW}Binaire non trouvé.${NC}"
fi

echo -e "${GREEN}[2/4] Suppression du raccourci menu...${NC}"
if [ -f "$DESKTOP_FILE" ]; then
    rm -f "$DESKTOP_FILE"
    
    # Mettre à jour la base de données des applications
    if command -v update-desktop-database >/dev/null 2>&1; then
        update-desktop-database "$DESKTOP_DIR" 2>/dev/null || true
    fi
    echo -e "${GREEN}Raccourci menu supprimé.${NC}"
else
    echo -e "${YELLOW}Raccourci menu non trouvé.${NC}"
fi

echo -e "${GREEN}[3/4] Gestion des données utilisateur...${NC}"
if [ -d "$DATA_DIR" ]; then
    echo -e "${YELLOW}Un dossier de données existe: $DATA_DIR${NC}"
    
    # Vérifier s'il y a des fichiers
    if [ "$(ls -A $DATA_DIR 2>/dev/null)" ]; then
        echo -e "${YELLOW}Le dossier contient des fichiers de données.${NC}"
        if confirm "Voulez-vous supprimer les données utilisateur?"; then
            rm -rf "$DATA_DIR"
            echo -e "${GREEN}Données supprimées.${NC}"
        else
            echo -e "${YELLOW}Données conservées dans: $DATA_DIR${NC}"
        fi
    else
        # Dossier vide, on peut le supprimer
        rm -rf "$DATA_DIR"
        echo -e "${GREEN}Dossier de données vide supprimé.${NC}"
    fi
else
    echo -e "${YELLOW}Aucune donnée utilisateur trouvée.${NC}"
fi

echo -e "${GREEN}[4/4] Nettoyage des fichiers de compilation (optionnel)...${NC}"
if [ -d "target" ] && [ -f "Cargo.toml" ]; then
    echo -e "${YELLOW}Des fichiers de compilation sont présents dans le dossier actuel.${NC}"
    if confirm "Voulez-vous supprimer le dossier 'target' (fichiers de compilation)?"; then
        rm -rf target
        echo -e "${GREEN}Dossier 'target' supprimé.${NC}"
    else
        echo -e "${YELLOW}Dossier 'target' conservé.${NC}"
    fi
else
    echo -e "${YELLOW}Aucun fichier de compilation dans le dossier actuel.${NC}"
fi

echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}  Désinstallation terminée!${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo -e "${YELLOW}Note:${NC}"
echo -e "  - Rust et les dépendances système n'ont pas été supprimés"
echo -e "  - Si vous souhaitez les supprimer également:"
echo -e "    ${BLUE}rustup self uninstall${NC} (pour Rust)"
echo -e "    ${BLUE}sudo apt remove libgtk-3-dev ...${NC} (pour les libs)"
echo ""
echo -e "${BLUE}Pour réinstaller EditorLevel2D, exécutez:${NC}"
echo -e "  ${YELLOW}./install.sh${NC}"
echo ""

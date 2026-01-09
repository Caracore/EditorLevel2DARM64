#!/bin/bash
# Script d'installation pour EditorLevel2D sur Raspberry Pi OS ARM64

set -e  # Arrêter en cas d'erreur

# Couleurs pour les messages
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}  Installation de EditorLevel2D ARM64${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

# Vérification que c'est bien ARM64
ARCH=$(uname -m)
if [[ "$ARCH" != "aarch64" ]] && [[ "$ARCH" != "arm64" ]]; then
    echo -e "${YELLOW}Attention: Ce script est conçu pour ARM64, mais l'architecture détectée est: $ARCH${NC}"
    read -p "Continuer quand même? (o/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Oo]$ ]]; then
        exit 1
    fi
fi

# Fonction pour vérifier si une commande existe
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# 1. Mise à jour du système
echo -e "${GREEN}[1/5] Mise à jour du système...${NC}"
sudo apt update

# 2. Installation des dépendances système
echo -e "${GREEN}[2/5] Installation des dépendances système...${NC}"
sudo apt install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    libgtk-3-dev \
    libxcb-render0-dev \
    libxcb-shape0-dev \
    libxcb-xfixes0-dev \
    libxkbcommon-dev \
    libfontconfig1-dev \
    mesa-vulkan-drivers \
    libwayland-dev \
    libxrandr-dev \
    libxi-dev \
    libxxf86vm-dev \
    libasound2-dev \
    libudev-dev

# 3. Installation de Rust si nécessaire
if ! command_exists rustc; then
    echo -e "${GREEN}[3/5] Installation de Rust...${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    echo -e "${YELLOW}Rust installé avec succès!${NC}"
else
    echo -e "${GREEN}[3/5] Rust est déjà installé ($(rustc --version))${NC}"
fi

# S'assurer que Rust est dans le PATH
if ! command_exists cargo; then
    export PATH="$HOME/.cargo/bin:$PATH"
    source "$HOME/.cargo/env" 2>/dev/null || true
fi

# 4. Compilation du programme
echo -e "${GREEN}[4/5] Compilation de EditorLevel2D...${NC}"
echo -e "${YELLOW}Cela peut prendre plusieurs minutes sur Raspberry Pi...${NC}"
cargo build --release

if [ ! -f "target/release/editor_level" ]; then
    echo -e "${RED}Erreur: La compilation a échoué!${NC}"
    exit 1
fi

echo -e "${GREEN}Compilation réussie!${NC}"

# 5. Installation du binaire
echo -e "${GREEN}[5/5] Installation du binaire...${NC}"

INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

# Copier le binaire
cp target/release/editor_level "$INSTALL_DIR/editor_level"
chmod +x "$INSTALL_DIR/editor_level"

# Créer un dossier pour les données du projet
DATA_DIR="$HOME/.local/share/editor_level"
mkdir -p "$DATA_DIR"

# Ajouter au PATH si nécessaire
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo ""
    echo -e "${YELLOW}Ajout de $INSTALL_DIR au PATH...${NC}"
    
    # Détecter le shell
    if [ -f "$HOME/.bashrc" ]; then
        if ! grep -q "export PATH=\"\$HOME/.local/bin:\$PATH\"" "$HOME/.bashrc"; then
            echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.bashrc"
            echo -e "${GREEN}PATH ajouté à ~/.bashrc${NC}"
        fi
    fi
    
    if [ -f "$HOME/.zshrc" ]; then
        if ! grep -q "export PATH=\"\$HOME/.local/bin:\$PATH\"" "$HOME/.zshrc"; then
            echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.zshrc"
            echo -e "${GREEN}PATH ajouté à ~/.zshrc${NC}"
        fi
    fi
    
    export PATH="$INSTALL_DIR:$PATH"
fi

# Créer un fichier desktop pour le menu (optionnel)
echo -e "${YELLOW}Création du raccourci dans le menu...${NC}"
DESKTOP_DIR="$HOME/.local/share/applications"
mkdir -p "$DESKTOP_DIR"

cat > "$DESKTOP_DIR/editor_level.desktop" << EOF
[Desktop Entry]
Version=1.0
Type=Application
Name=EditorLevel2D
Comment=Éditeur de niveaux 2D pour jeux
Exec=$INSTALL_DIR/editor_level
Icon=applications-games
Terminal=false
Categories=Graphics;2DGraphics;Development;
Keywords=level;editor;game;2d;tilemap;
EOF

# Mettre à jour la base de données des applications
if command_exists update-desktop-database; then
    update-desktop-database "$DESKTOP_DIR" 2>/dev/null || true
fi

echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}  Installation terminée avec succès!${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo -e "${BLUE}Pour lancer l'éditeur:${NC}"
echo -e "  - Depuis le terminal: ${YELLOW}editor_level${NC}"
echo -e "  - Depuis le menu: Cherchez '${YELLOW}EditorLevel2D${NC}'"
echo ""
echo -e "${BLUE}Emplacement du binaire:${NC} $INSTALL_DIR/editor_level"
echo -e "${BLUE}Dossier des données:${NC} $DATA_DIR"
echo ""
echo -e "${YELLOW}Note: Vous devrez peut-être redémarrer votre terminal ou exécuter:${NC}"
echo -e "  ${YELLOW}source ~/.bashrc${NC}"
echo ""

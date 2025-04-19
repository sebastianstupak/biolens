#!/bin/sh
set -e

REPO="sebastianstupak/biolens"
INSTALL_DIR="/usr/local/bin"
VERSION="${VERSION:-latest}"
VISUALIZATION="${VISUALIZATION:-}"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo "${BLUE}"
echo "┌───────────────────────────────────────┐"
echo "│            BioLens Installer          │"
echo "└───────────────────────────────────────┘${NC}"
echo ""

OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$ARCH" in
  x86_64) ARCH="x86_64" ;;
  amd64) ARCH="x86_64" ;;
  arm64) ARCH="aarch64" ;;
  aarch64) ARCH="aarch64" ;;
  *)
    echo "${RED}Unsupported architecture: $ARCH${NC}"
    exit 1
    ;;
esac

if [ "$OS" = "linux" ]; then
  TARGET="linux-${ARCH}"
  EXT="tar.gz"
elif [ "$OS" = "darwin" ]; then
  TARGET="macos-${ARCH}"
  EXT="tar.gz"
else
  echo "${RED}Unsupported operating system: $OS${NC}"
  exit 1
fi

echo "${BLUE}System detection:${NC}"
echo "- Operating System: ${YELLOW}$OS${NC}"
echo "- Architecture: ${YELLOW}$ARCH${NC}"
echo "- Target: ${YELLOW}$TARGET${NC}"
echo ""

if [ -z "$VISUALIZATION" ]; then
  echo "${BLUE}BioLens is available in two versions:${NC}"
  echo "1. Standard version (smaller, basic functionality)"
  echo "2. Visualization version (larger, includes visualization features)"
  echo ""
  printf "${YELLOW}Would you like to install the version with visualization features? [y/N] ${NC}"
  read -r VIZ_CONFIRM

  if [ "$VIZ_CONFIRM" = "y" ] || [ "$VIZ_CONFIRM" = "Y" ]; then
    VISUALIZATION="true"
    echo "${GREEN}Installing BioLens with visualization features.${NC}"
  else
    VISUALIZATION="false"
    echo "${GREEN}Installing standard BioLens version.${NC}"
  fi
  echo ""
fi

if [ "$VISUALIZATION" = "true" ]; then
  PACKAGE_SUFFIX="-viz"
else
  PACKAGE_SUFFIX=""
fi

if [ "$VERSION" = "latest" ]; then
  echo "${BLUE}Fetching latest release information...${NC}"
  RELEASE_URL=$(curl -s https://api.github.com/repos/$REPO/releases/latest | grep "browser_download_url.*biolens${PACKAGE_SUFFIX}-$TARGET.$EXT" | cut -d '"' -f 4)

  if [ -z "$RELEASE_URL" ]; then
    echo "${RED}Could not find a release for your platform: $TARGET${NC}"
    echo "${YELLOW}Trying alternative package name...${NC}"

    if [ "$VISUALIZATION" = "true" ]; then
      ALT_SUFFIX=""
      echo "${YELLOW}Falling back to standard version${NC}"
    else
      ALT_SUFFIX="-viz"
      echo "${YELLOW}Falling back to visualization version${NC}"
    fi

    RELEASE_URL=$(curl -s https://api.github.com/repos/$REPO/releases/latest | grep "browser_download_url.*biolens${ALT_SUFFIX}-$TARGET.$EXT" | cut -d '"' -f 4)

    if [ -z "$RELEASE_URL" ]; then
      echo "${RED}Could not find any suitable release for your platform: $TARGET${NC}"
      exit 1
    fi
  fi

  VERSION_FROM_URL=$(echo $RELEASE_URL | grep -o 'v[0-9]\+\.[0-9]\+\.[0-9]\+' || echo "latest")
  echo "Latest version: ${YELLOW}$VERSION_FROM_URL${NC}"
else
  RELEASE_URL="https://github.com/$REPO/releases/download/${VERSION}/biolens${PACKAGE_SUFFIX}-${TARGET}.${EXT}"
  echo "Installing version: ${YELLOW}$VERSION${NC}"
fi

echo ""
echo "${YELLOW}BioLens will be installed to: ${BLUE}$INSTALL_DIR/biolens${NC}"
echo "${YELLOW}This may require sudo access.${NC}"
echo ""
printf "${YELLOW}Do you want to continue? [y/N] ${NC}"
read -r CONFIRM

if [ "$CONFIRM" != "y" ] && [ "$CONFIRM" != "Y" ]; then
  echo "${RED}Installation cancelled.${NC}"
  exit 1
fi

echo ""
echo "${BLUE}Downloading BioLens from: ${NC}$RELEASE_URL"
TEMP_DIR=$(mktemp -d)
curl -L "$RELEASE_URL" -o "$TEMP_DIR/biolens.$EXT"

cd "$TEMP_DIR"
if [ "$EXT" = "tar.gz" ]; then
  tar -xzf "biolens.$EXT"
fi

chmod +x biolens

echo "${BLUE}Installing BioLens...${NC}"
if [ -w "$INSTALL_DIR" ]; then
  mv biolens "$INSTALL_DIR/"
  echo "${GREEN}Installed without requiring elevated permissions.${NC}"
elif command -v sudo > /dev/null; then
  echo "Requesting sudo access to install to $INSTALL_DIR"
  sudo mv biolens "$INSTALL_DIR/"
  echo "${GREEN}Installed with sudo.${NC}"
else
  echo "${YELLOW}Attempting to install without sudo...${NC}"
  mv biolens "$INSTALL_DIR/"
  if [ $? -ne 0 ]; then
    echo "${RED}Installation failed. Try running with sudo or specify a different install location.${NC}"
    exit 1
  fi
fi

echo ""
echo "${GREEN}BioLens has been successfully installed to $INSTALL_DIR/biolens${NC}"
echo "${BLUE}Run '${YELLOW}biolens --help${BLUE}' to get started${NC}"

cd - > /dev/null
rm -rf "$TEMP_DIR"

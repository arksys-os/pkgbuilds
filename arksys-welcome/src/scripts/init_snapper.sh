#!/usr/bin/bash
#set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

echo "###################################"
echo "        ArkSys Snapper Setup       "
echo "###################################"
echo
# Get current user's username
username=$(whoami)

# Welcome message
echo -e "${GREEN}Hi, $username! Checking for BTRFS and setting up Snapper if found.${NC}"
echo
sleep 3
# Check if the file system is BTRFS
if lsblk -f | grep -q "btrfs"; then
    echo -e "${GREEN}BTRFS partitions found. Installing and initializing Snapper.${NC}"
    echo
    sudo pacman -Sy btrfs-assistant btrfs-du snapper-gui-git snapper-support btrfsmaintenance
    echo
    sleep 3
    echo "Initializing snapper for root subvolume"
    echo
    sudo snapper -c root create-config /
    sudo snapper -c root create
    echo
    sleep 3
    echo "Initializing snapper for home subvolume"
    sudo snapper -c home create-config /home
    sudo snapper -c home create
    echo
    sleep 3
    echo -e "${BLUE}Snapper installed and initialized. Reboot required.${NC}"
    echo
else
    echo -e "${RED}No BTRFS Partitions found. Exiting...${NC}"
    exit 1
fi

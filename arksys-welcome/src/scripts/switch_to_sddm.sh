#!/bin/bash
#set -e
##################################################################################################################
# Written to be used on 64 bits computers
# Author 	: 	David7ce
# Website 	: 	https://github.com/arksys-os/
##################################################################################################################
echo
echo "Removing LIGHTDM & its Dependencies"
echo "################################"

sudo pacman -R --noconfirm lightdm lightdm-gtk-greeter lightdm-gtk-greeter-settings

echo
sleep 2
echo "Installing & Enabling SDDM"
echo "#############################"

sudo pacman -S sddm sddm-kcm --needed --noconfirm
sleep 2

sudo systemctl enable sddm.service -f

echo "#################################"
echo "SDDM is now active - rebooting..."
echo "#################################"
sleep 6
reboot

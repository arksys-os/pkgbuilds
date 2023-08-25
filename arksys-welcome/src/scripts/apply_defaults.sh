#!/bin/bash
#set -e
##################################################################################################################
# Written to be used on 64 bits computers
# Author 	: 	David7ce
# Website 	: 	http://github.com/arksys-os/
##################################################################################################################
tput setaf 1
echo "############################################################################"
echo "#                         !!! ArkSys Reset Tool !!!                        #"
echo "#                                                                          #"
echo "#           Having Issues With Messed Up Layout or Settings ?              #"
echo "#                                                                          #"
echo "#         This Will Restore Stock Defaults. Layout WILL BE RESET           #"
echo "############################################################################"
tput sgr0
echo
echo "Hello $USER, which Edition are you using ?"
echo
echo "1.  ArkSys KDE Plasma."
echo
#echo "4.  Exit"
echo
echo "Please Select an Option..."
echo

read CHOICE

case $CHOICE in

    1 )
      echo "Creating Backups of ~/.config folder"
      echo "#####################################"
      cp -Rf ~/.config ~/.config-backup-$(date +%Y.%m.%d-%H.%M.%S)
      rm -Rf ~/.local/share/plasma/
      sleep 2
      echo "###################################"
      echo "  Restoring/Applying KDE defaults  "
      echo "###################################"
      sleep 2
      sudo pacman -Rdd qt5-virtualkeyboard --noconfirm
      sudo pacman -S lightly-git latte-dock-git asian-fonts lightlyshaders-git catppuccin-cursors-git catppuccin-kde-theme-git catppuccin-gtk-theme-mocha tela-circle-icon-theme-dracula-git --noconfirm --needed
      cp -rf /etc/skel/. ~
      sudo sed -i "s/Current=.*/Current=catppuccin/g" /etc/sddm.conf.d/kde_settings.conf
      sleep 2
      echo "##################################"
      echo "  Done! Reboot to Apply Settings  "
      echo "##################################"
      sleep 6

      ;;

    * )
      echo "#################################"
      echo "Choose the correct number"
      echo "#################################"
      ;;
esac

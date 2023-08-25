#!/bin/bash
#set -e
##################################################################################################################
# Written to be used on 64 bits computers
# Author 	: 	David7ce
# Website 	: 	https://github.com/arksys-os/
##################################################################################################################
echo
tput setaf 3
echo "###############################################################################"
echo "#                             ArkSys System Updater                           #"
echo "#                                                                             #"
echo "#                            !!!! ATTENTION !!!                               #"
echo "#                                                                             #"
echo "#           Option 3) uses Topgrade which will Update *EVERYTHING*.           #"
echo "# Which includes any Git Repos, Docker Containers, Nix, Device Firmware etc.. #"
echo "#                                                                             #"
echo "#               Do *NOT* use unless you know what you are doing               #"
echo "###############################################################################"
tput sgr0
echo
echo "Hello $USER, Please Select Packages To Update."
echo
echo "########## Standard Users ##########"
echo
echo "1.  Arch & AUR Packages."
echo "2.  Flatpak Packages."
echo
echo "########## Advanced Users ##########"
echo
echo "3.  Everything, One Shot. (Nuke!)"
echo
echo "Type Your Selection. To Exit, just close Window."
echo

while :; do

read CHOICE

case $CHOICE in

    1 )
      echo
      echo "##########################################"
      echo "       Updating Arch & AUR Packages       "
      echo "##########################################"
			sleep 3
			if [[ -f "/usr/bin/paru" ]]; then
               paru -Syu
             else
               yay -Syu
            fi
			sleep 3
      echo "#######################################"
      echo "                 Done !                "
      echo "#######################################"
            clear && sh /usr/share/arksys-welcome/scripts/update_system.sh
      ;;

    2 )
      echo
      echo "##########################################"
      echo "        Updating Flatpak Packages         "
      echo "##########################################"
			sleep 3
			flatpak update
			sleep 3
      echo "#######################################"
      echo "                 Done !                "
      echo "#######################################"
            clear && sh /usr/share/arksys-welcome/scripts/update_system.sh
      ;;


    3 )
      echo
      echo "##########################################"
      echo "       !!! Updating Everything !!!        "
      echo "##########################################"
			sleep 3
			topgrade
			sleep 3
      echo "#######################################"
      echo "                 Done !                "
      echo "#######################################"
            exit

      ;;

    * )
      echo "#################################"
      echo "    Choose the correct number    "
      echo "#################################"
      ;;
esac
done

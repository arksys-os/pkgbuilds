#!/bin/bash
#set -e
##################################################################################################################
# Written to be used on 64 bits computers
# Author 	: 	David7ce
# Website : 	https://github.com/arksys-os/
##################################################################################################################
echo
tput setaf 3
echo "###############################################################################"
echo "#                  ArkSys nVidia (Proprietary) Driver Installer               #"
echo "#                                                                             #"
echo "#                            !!!! ATTENTION !!!                               #"
echo "#                                                                             #"
echo "#                     Carefully Select The right Drivers..                    #"
echo "#                                                                             #"
echo "# Only R525.x & R470.x Drivers will be Pre-Compiled. For Other use Option 4). #"
echo "###############################################################################"
tput sgr0
echo
echo "Hello $USER, Please Select An Option Below."
echo
echo "##################### GPU Checker ####################"
echo
echo "1.  Check Which nVidia GPU You Have."
echo
echo "############# nVidia Proprietary Drivers #############"
echo
echo "2.  nVidia R525.x Drivers (GTX 900 Seies & Newer incl. 745/750ti)."
echo "3.  nVidia R470.x Drivers (GTX 600 & 700 Series excl. 605/610/620/625/645)."
echo "4.  Other nVidia  Drivers (Building will be required, via TKG's Custom Script)."
echo
echo "Type Your Selection. To Exit, just close Window."
echo

while :; do

read CHOICE

case $CHOICE in

    1 )
      echo
      echo "##########################################"
      echo "          Checking Installed GPU          "
      echo "##########################################"
      echo
      echo "Your system has the following GPU(s)"
      echo
			sleep 3
			lspci -x | grep VGA
			sleep 3
      echo
            glxinfo | grep -E "OpenGL vendor|OpenGL renderer*"
            sleep 6
      echo
			echo "##################################################################"
			echo "                  Opening nVidia Drivers page...                  "
			echo " Check What Version Your GPU Needs Before Installing or Building. "
			echo "##################################################################"
			sleep 6
			xdg-open https://www.nvidia.com/download/index.aspx?lang=en-us &
			sleep 1
			clear && sh /usr/share/arksys-welcome/scripts/nVidia_drivers.sh
      echo
      echo "#######################################"
      echo "                 Done !                "
      echo "#######################################"

      ;;

    2 )
      echo
      echo "##########################################"
      echo "   Installing nVidia R525.x GPU Drivers   "
      echo "##########################################"
			sleep 3
			sudo pacman -S --noconfirm --needed nvidia-dkms-tkg nvidia-utils-tkg nvidia-settings-tkg nvidia-egl-wayland-tkg opencl-nvidia-tkg libxnvctrl lib32-libxnvctrl lib32-nvidia-utils-tkg lib32-opencl-nvidia-tkg dxvk-nvapi-mingw vulkan-icd-loader lib32-vulkan-icd-loader lib32-nvidia-libgl
			sleep 3
      echo "#######################################"
      echo "                 Done !                "
      echo "#######################################"
            exit
      ;;

    3 )
      echo
      echo "##########################################"
      echo "   Installing nVidia R470.x GPU Drivers   "
      echo "##########################################"
			sleep 3
			sudo pacman -S --noconfirm nvidia-470xx-dkms-tkg nvidia-470xx-utils-tkg nvidia-470xx-settings-tkg opencl-nvidia-470xx-tkg libxnvctrl-470xx lib32-nvidia-470xx-utils-tkg lib32-opencl-nvidia-470xx-tkg lib32-libxnvctrl-470xx dxvk-nvapi-mingw
			sleep 3
      echo "#######################################"
      echo "                 Done !                "
      echo "#######################################"
            exit
      ;;

    4 )
      echo
      echo "##########################################"
      echo "      Older/Unlisted nVidia Drivers.      "
      echo "##########################################"
			sleep 3
			cd ~ && git clone https://github.com/Frogging-Family/nvidia-all/
			cd ~/nvidia-all/ && makepkg -si
			rm -rf ~/nvidia-all/
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

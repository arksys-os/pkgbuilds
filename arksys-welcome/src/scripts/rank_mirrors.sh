#!/usr/bin/bash
echo
echo "##########################################"
echo "     Updating Mirrors To Fastest Ones     "
echo "##########################################"
echo
rate-mirrors --allow-root --protocol https arch  | sudo tee /etc/pacman.d/mirrorlist && sudo pacman -Sy
echo
echo "##################################"
echo " Done ! Updating should go faster "
echo "##################################"

#!/usr/bin/bash
#set -e

echo "###################################"
echo "#      Installing OhMyBash!       #"
echo "###################################"

bash -ci "$(curl -fsSL https://raw.githubusercontent.com/ohmybash/oh-my-bash/master/tools/install.sh)"

sleep 2
echo "##################################"
echo "#  Importing ArkSys Settings  #"
echo "##################################"
mv $HOME/.bashrc $HOME/.bashrc.bk
cd $HOME/ && wget -O .bashrc https://raw.githubusercontent.com/arksys-os/ark-fixes/main/conf/.ombrc
cd $HOME/.oh-my-bash/aliases/ && mv misc.aliases.sh misc.aliases.sh.bk
cd $HOME/.oh-my-bash/aliases/ && wget https://raw.githubusercontent.com/arksys-os/ark-fixes/main/conf/misc.aliases.sh

sleep 2
echo "#################################"
echo "#  Done ! Now Logout & back in  #"
echo "#################################"

#!/bin/bash

cd ./arksys-calamares-3.3.0

makepkg -s

read -p "Do you want to remove files generated in makepkg? [Y/n] " answer

if [[ ${answer,,} =~ ^(yes|y)$ ]]; then
    echo "Executing the command..."
    rm -rf ./arksys-calamares-3.3.0/ ./pkg/ ./src/
else
    echo "Command not executed."
fi

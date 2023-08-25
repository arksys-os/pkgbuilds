#!/bin/bash

makepkg -s

read -p "Do you want to remove files generated in makepkg? [Y/n] " answer

if [[ ${answer,,} =~ ^(yes|y)$ ]]; then
    echo "Executing the command..."
    rm -rf ./calamares/ ./pkg/ ./src/
else
    echo "Command not executed."
fi

#!/bin/bash

# List of available folders to choose from
folders=("arcolinux-calamares-3.3.0-alpha6-01" "arksys-calamares-config" "arksys-calamares-v3.2.62" "arksys-calamares-v3.3.0.alpha6" "arksys-mirrorlist" "arksys-welcome" "eaglemode")

# Display the available folders for selection
echo "Select a folder:"
for ((i=0; i<${#folders[@]}; i++)); do
    echo "$((i+1)). ${folders[i]}"
done

# Read user's choice
read -p "Enter the number corresponding to the folder: " choice

# Validate user's input
if [[ ! "$choice" =~ ^[0-9]+$ ]] || ((choice < 0)) || ((choice >= ${#folders[@]}+1)); then
    echo "Invalid choice. Exiting."
    exit 1
fi

# Adjust the choice to array index (subtracting 1)
selected_folder="${folders[choice - 1]}"

# Navigate to the selected folder
cd $selected_folder || exit 1

# Run the makepkg command
makepkg -s

# Ask whether to remove generated files
read -p "Do you want to remove files generated in makepkg? [Y/n] " answer

if [[ ${answer,,} =~ ^(yes|y)$ ]]; then
    echo "Executing the command..."
    rm -rf $selected_folder ./pkg/ ./src/
else
    echo "Command not executed."
fi

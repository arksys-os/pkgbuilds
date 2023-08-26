# PKGBUILDs of ArkSys

The PKGBUILD is the configuration to build arch packages "name-version.pkg.tar.gz" that use the package manager 'pacman' for installing the software in your Linux system.

To build the app you need to run `makepkg` inside the directory that contains the PKGBUILD. You can also run the script `build.sh` in the correspondent PKGBUILD directory.

> ![note] If you use source from local files the PKGBUILD need to be updated manually.

> ![note] You must use source from online repository to auto-update with pacman.

## Arksys Calamares v3.3.0
Calamares app for version 3.3.0, used for Arksys-ISO

Based on [ArcoLinux-pkgbuild-calamares version git](https://github.com/arcolinux/arcolinux-pkgbuild-calamares/tree/master/arco-calamares-git)

Source from online repo: https://github.com/erikdubois/calamares

## Arksys Calamares v3.2.62
Calamares app for version 3.2.62, not used.

Based on [DT-OS calamares](https://gitlab.com/dtos/dtos-pkgbuild/-/tree/main/x86_64/calamares) and [Xerolinux calamares](https://github.com/xerolinux/calamares-app/blob/main/PKGBUILD)

Source from online repo: https://github.com/calamares/calamares/archive/v3.2.62.tar.gz

## Arksys Calamares config
Calamares configuration for "arksys-calamares-v3.3.0".

Source from online repo: https://github.com/arksys-os/arksys-calamares-config/calamares/archive/v3.2.62.tar.gz

> Also you can copy "/etc/calamares/" as a config file and paste in "archiso/airootfs/etc/calamares".

## Arksys mirrorlist
Mirrolist of online servers tha contains the database packages of ArkSys, (not configured in Arksys-ISO).

Source from local files: ./etc/pacman.d/arksys-mirrorlist

## Arksys Welcome
Welcome app (not configured in Arksys-ISO).

Source from local files: ./



# PKGBUILDs of ArkSys

The PKGBUILD is the configuration to build arch packages "name-version.pkg.tar.gz" that use the package manager 'pacman' for installing the software in your Linux system.

To build the app you need to run `makepkg` inside the directory that contains the PKGBUILD. You can also run the script `build.sh` in the correspondent PKGBUILD directory.

> [!IMPORTANT] If you use source from local files the PKGBUILD need to be updated manually.
> You must use source from online repository to auto-update with pacman.

## Arksys Calamares v3.3.0.alpha6
Calamares app for version 3.3.0.alpha6 used for Arksys-ISO.

Based on [ArcoLinux-pkgbuild-calamares version git](https://github.com/arcolinux/arcolinux-pkgbuild-calamares/tree/master/arco-calamares-3.3.0-alpha6-01)

Source from online repo: https://github.com/erikdubois/calamares/archive/refs/tags/3.3.0.alpha6.tar.gz

## Arksys Calamares v3.2.62 (old)
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

## Arksys Welcome Python
Welcome app (not configured in Arksys-ISO).

Source from local files: ./

## Arksys Welcome Rust
Welcome app (not configured in Arksys-ISO).

Source from local files: ./

## Extra packages
Eaglemode: a zoomable user interface (ZUI) with file manager, file viewers, games, and more.

---

# How to develop Linux apps you can use:

With appropriate libraries and frameworks you can use any language as CLI or GUI. Anything that creates a binary file.

1. Native development:

- CLI (Command-line apps)
    - Bash
    - C
    - C++
    - Python
    - Java
    - Perl
    - Ruby
    - Rust
    - Lua
    - Go (Golang)
    - Nim
    - Haskell

- GUI (Graphical User Interface)
    - C/C++ (with GTK or Qt)
    - Python (with GTK or PyQt/PySide)
    - Java (with JavaFX or Swing)
    - Vala (with GTK)
    - Rust (with Druid or Gtk-rs)

2. Cross platform frameworks:
- Electron: A framework for building cross-platform desktop apps using web technologies (HTML, CSS, JavaScript).
- Flutter: A UI toolkit by Google for creating natively compiled applications for mobile, web, and desktop from a single codebase.
- JavaFX: A Java library for building rich graphical user interfaces (GUIs) for desktop applications.

3. Web applicattions: HTML, CSS, JS as frontend + PHP with SQL as backend

> The advantages of web-app is that runs on a web-browser and every OS can a run one. The main disadvantage is that is non natve.

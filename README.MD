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

With appropriate libraries and frameworks you can use any language as CLI or GUI.

1. Native development:

- CLI (Command-line apps)
    - Bash
    - C
    - C++
    - Python
    - Perl
    - Ruby
    - Rust
    - Lua
    - Go (Golang)
    - Nim
    - Haskell
    - D
    - Ada

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

3. Web applicattions:
- Traditional method.
    - Front-end (client side)
        - Markup language: HTML
        - Style sheet language: CSS
        - Client programming language: JavaScript
    - Back-end (server side)
        - Server programming language: PHP
        - Database langauge (SQL / NoSQL)
- Modern methods as [web framework](https://en.wikipedia.org/wiki/Web_framework)
    - Front-end frameworks
        - React: JavaScript library for building user interfaces
            - Next.js: for building server-rendered and client-rendered React apps
        - Angular: for building complex and feature-rich apps with Typescript
        - Vue.js: for building user interfaces and apps with JS
    - Back-end frameworks
        - NestJS: JS framework for building RESTful APIs, microservices and backend apps
        - Django: high-level Python framework
        - Express.js: web framework for Node.js
        - Ruby on rails: web framework for the Ruby
    - Full-stack frameworks
        - Laravel: PHP framework with Model-View-Controller
        - Meteor: full-stack JavaScript framework
        - Split boot: Java-based framework
    - Static-Site-Generators (SSG)
        - HUGO: SSG written in Go
        - Jekyll: SSG written in Ruby
        - Gatsby: SSG that uses React and GraphQL.
    - [Content Management System (CMS)](https://en.wikipedia.org/wiki/List_of_content_management_systems)
        - WordPress: Popular and user-friendly blogging platform.
        - Joomla: Flexible and balanced CMS.
        - Drupal: Powerful and versatile CMS for complex projects.

> The advantages of web-app is that runs on a web-browser and every OS can a run one. The main disadvantage is that is non natve.


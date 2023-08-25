use crate::utils;
use crate::utils::PacmanWrapper;

use gtk::prelude::*;

#[derive(Clone, Debug)]
#[repr(C)]
pub struct AlpmHelper {
    pub pkg_list_install: Vec<String>,
    pub pkg_list_removal: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AlpmHelperResult {
    Nothing,
    Remove,
    Add,
    Both,
}

impl AlpmHelper {
    pub fn new() -> Self {
        Self { pkg_list_install: Vec::new(), pkg_list_removal: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.pkg_list_install.clear();
        self.pkg_list_removal.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.pkg_list_install.is_empty() && self.pkg_list_removal.is_empty()
    }

    pub fn do_update(&self, is_flatpak: bool) -> AlpmHelperResult {
        let mut result = AlpmHelperResult::Nothing;
        if self.pkg_list_install.is_empty() && self.pkg_list_removal.is_empty() {
            return result;
        }

        if !self.pkg_list_removal.is_empty()
            && self.install_apps(&self.pkg_list_removal, false, is_flatpak)
        {
            if !is_flatpak {
                let infodialog = gtk::MessageDialog::builder()
                    .title("Removal completed successfully!")
                    .text(
                        format!("These pkgs '{:?}' has been removed!", self.pkg_list_removal)
                            .as_str(),
                    )
                    .modal(true)
                    .message_type(gtk::MessageType::Info)
                    .build();
                infodialog.show();
            }
            result = AlpmHelperResult::Remove;
        }
        if !self.pkg_list_install.is_empty()
            && self.install_apps(&self.pkg_list_install, true, is_flatpak)
        {
            if !is_flatpak {
                let infodialog = gtk::MessageDialog::builder()
                    .title("Installation completed successfully!")
                    .text(
                        format!("These pkgs '{:?}' has been installed!", self.pkg_list_install)
                            .as_str(),
                    )
                    .modal(true)
                    .message_type(gtk::MessageType::Info)
                    .build();
                infodialog.show();
            }
            if result == AlpmHelperResult::Nothing {
                result = AlpmHelperResult::Add;
            } else {
                result = AlpmHelperResult::Both;
            }
        }

        result
    }

    pub fn set_package(&mut self, pkg_name: &String, install: bool, installed: bool) {
        if self.to_remove(pkg_name) {
            let index = self.pkg_list_removal.iter().position(|x| *x == *pkg_name).unwrap();
            self.pkg_list_removal.remove(index);
        } else if !install && installed {
            self.pkg_list_removal.push(String::from(pkg_name));
        }

        if self.to_install(pkg_name) {
            let index = self.pkg_list_install.iter().position(|x| *x == *pkg_name).unwrap();
            self.pkg_list_install.remove(index);
        } else if install && !installed {
            self.pkg_list_install.push(String::from(pkg_name));
        }
    }

    pub fn to_install(&self, pkg_name: &String) -> bool {
        self.pkg_list_install.contains(pkg_name)
    }

    pub fn to_remove(&self, pkg_name: &String) -> bool {
        self.pkg_list_removal.contains(pkg_name)
    }

    fn install_apps(&self, pkg_list: &Vec<String>, install: bool, is_flatpak: bool) -> bool {
        if pkg_list.is_empty() {
            return false;
        }

        let packages_do = pkg_list.iter().map(|s| s.to_string() + " ").collect::<String>();

        if !is_flatpak {
            let (cmd, escalate) = match install {
                true => match utils::get_pacman_wrapper() {
                    PacmanWrapper::Pak => ("pak -Sy", false),
                    PacmanWrapper::Yay => ("yay -Sy", false),
                    PacmanWrapper::Paru => ("paru -Sy", false),
                    _ => ("pacman -Sy", true),
                },
                false => match utils::get_pacman_wrapper() {
                    PacmanWrapper::Pak => ("pak -R", false),
                    PacmanWrapper::Yay => ("yay -R", false),
                    PacmanWrapper::Paru => ("paru -R", false),
                    _ => ("pacman -R", true),
                },
            };
            let _ = utils::run_cmd_terminal(format!("{} {}", cmd, packages_do), escalate);
        } else {
            let arg = match install {
                false => "remove",
                _ => "install",
            };
            let _ = utils::run_cmd_terminal(format!("flatpak {} {}", arg, packages_do), false);
        }

        match install {
            true => self.app_installed(&pkg_list[0], is_flatpak),
            false => !self.app_installed(&pkg_list[0], is_flatpak),
        }
    }

    fn app_installed(&self, pkg_names: &str, is_flatpak: bool) -> bool {
        if is_flatpak {
            return true;
        }

        let pkg_name_vec = pkg_names.split(' ').map(String::from).collect::<Vec<String>>();
        let pkg_name = pkg_name_vec.first().unwrap();

        let pacman =
            pacmanconf::Config::with_opts(None, Some("/etc/pacman.conf"), Some("/")).unwrap();
        let alpm = alpm_utils::alpm_with_conf(&pacman).unwrap();
        matches!(alpm.localdb().pkg(pkg_name.as_bytes()), Ok(_))
    }
}

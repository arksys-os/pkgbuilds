use gtk::prelude::*;
use std::path::Path;
use std::{fs, str};

use subprocess::{Exec, Redirection};

#[derive(Debug)]
pub enum PacmanWrapper {
    Pak,
    Yay,
    Paru,
    Pacman,
}

#[inline]
pub fn fix_path(path: &str) -> String {
    if !path.starts_with('~') {
        return String::from(path);
    }
    path.replace('~', glib::home_dir().as_path().to_str().unwrap())
}

#[inline]
pub fn read_json(path: &str) -> serde_json::Value {
    let buf = fix_path(path);
    let data = fs::read_to_string(buf).expect("Unable to read file");
    serde_json::from_str(&data).expect("Unable to parse")
}

#[inline]
pub fn check_regular_file(path: &str) -> bool {
    let metadata = fs::metadata(path);
    if let Ok(meta) = metadata {
        meta.file_type().is_file()
    } else {
        false
    }
}

pub fn create_combo_with_model(group_store: &gtk::ListStore) -> gtk::ComboBox {
    let group_combo = gtk::ComboBox::with_model(group_store);
    let combo_renderer = gtk::CellRendererText::new();
    group_combo.pack_start(&combo_renderer, true);
    group_combo.add_attribute(&combo_renderer, "text", 0);
    group_combo.set_active(Some(0));

    group_combo
}

pub fn run_cmd_terminal(cmd: String, escalate: bool) -> bool {
    let cmd_formated = format!("{}; read -p 'Press enter to exit'", cmd);
    let mut args: Vec<&str> = vec![];
    if escalate {
        args.extend_from_slice(&["-s", "pkexec /usr/share/arksys-welcome/scripts/rootshell.sh"]);
    }
    args.push(cmd_formated.as_str());

    let exit_status = Exec::cmd("/usr/share/arksys-welcome/scripts/terminal-helper")
        .args(args.as_slice())
        .stdout(Redirection::Pipe)
        .join()
        .unwrap();
    exit_status.success()
}

pub fn run_cmd_root(cmd: String) -> bool {
    let exit_status = Exec::cmd("/sbin/pkexec").arg("bash").arg("-c").arg(cmd).join().unwrap();
    exit_status.success()
}

#[inline]
pub fn get_pacman_wrapper() -> PacmanWrapper {
    if Path::new("/sbin/pak").exists() {
        return PacmanWrapper::Pak;
    } else if Path::new("/sbin/yay").exists() {
        return PacmanWrapper::Yay;
    } else if Path::new("/sbin/paru").exists() {
        return PacmanWrapper::Paru;
    }

    PacmanWrapper::Pacman
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_file() {
        assert!(check_regular_file("/etc/fstab"));
        assert!(!check_regular_file("/etc"));
    }
}

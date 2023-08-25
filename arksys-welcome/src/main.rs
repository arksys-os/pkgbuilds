#![feature(const_str_from_utf8)]
#![feature(string_remove_matches)]
#![allow(non_upper_case_globals)]

mod alpm_helper;
mod application_browser;
mod config;
mod data_types;
mod pages;
mod utils;

use config::{APP_ID, GETTEXT_PACKAGE, LOCALEDIR, PKGDATADIR, VERSION};
use data_types::*;
use gettextrs::LocaleCategory;
use gtk::{gio, glib, Builder, HeaderBar, Window};
use std::path::Path;
use std::sync::Arc;
use utils::*;

use gio::prelude::*;
use gtk::prelude::*;

use gdk_pixbuf::Pixbuf;

use std::{fs, str};
use subprocess::Exec;

static mut g_hello_window: Option<Arc<HelloWindow>> = None;

fn launch_installer() {
    // Create the widgets
    let cmd = String::from("sudo -E calamares -D6");

    // Spawn child process in separate thread.
    std::thread::spawn(move || {
        Exec::shell(cmd).join().unwrap();
    });
}

fn show_about_dialog() {
    let main_window: Window;
    unsafe {
        main_window = g_hello_window.clone().unwrap().window.clone();
    }
    let logo_path = format!("/usr/share/icons/hicolor/scalable/apps/{}.png", APP_ID);
    let logo = Pixbuf::from_file(logo_path).unwrap();

    let dialog = gtk::AboutDialog::builder()
        .transient_for(&main_window)
        .modal(true)
        .program_name(&gettextrs::gettext("ArkSys Welcome"))
        .comments(&gettextrs::gettext("Welcome screen for ArkSys"))
        .version(VERSION)
        .logo(&logo)
        .authors(vec![
            "Vladislav Nepogodin".into(),
        ])
        // Translators: Replace "translator-credits" with your names. Put a comma between.
        .translator_credits(&gettextrs::gettext("translator-credits"))
        .copyright("2021-2023 CachyOS team")
        .license_type(gtk::License::Gpl30)
        .website("https://github.com/cachyos/cachyos-welcome")
        .website_label("GitHub")
        .build();

    dialog.run();
    dialog.hide();
}

fn main() {
    gettextrs::setlocale(LocaleCategory::LcAll, "");
    gettextrs::bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain.");
    gettextrs::textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain.");

    glib::set_application_name("ArkSysHello");

    gtk::init().expect("Unable to start GTK3.");

    let application = gtk::Application::new(
        Some(APP_ID),       // Application id
        Default::default(), // Using default flags
    );

    application.connect_activate(|application| {
        build_ui(application);
    });

    // Run the application and start the event loop
    application.run();
}

fn build_ui(application: &gtk::Application) {
    let data = fs::read_to_string(format!("{}/data/preferences.json", PKGDATADIR))
        .expect("Unable to read file");
    let preferences: serde_json::Value = serde_json::from_str(&data).expect("Unable to parse");

    // Import Css
    let provider = gtk::CssProvider::new();
    provider
        .load_from_path(preferences["style_path"].as_str().unwrap())
        .expect("Failed to load CSS");
    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::default().expect("Error initializing gtk css provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    // Init window
    let builder: Builder = Builder::from_file(preferences["ui_path"].as_str().unwrap());
    builder.connect_signals(|_builder, handler_name| {
        match handler_name {
            // handler_name as defined in the glade file => handler function as defined above
            "on_action_clicked" => Box::new(on_action_clicked),
            "on_btn_clicked" => Box::new(on_btn_clicked),
            "on_link_clicked" => Box::new(on_link_clicked),
            "on_link1_clicked" => Box::new(on_link1_clicked),
            "on_delete_window" => Box::new(on_delete_window),
            _ => Box::new(|_| None),
        }
    });

    let main_window: Window = builder.object("window").expect("Could not get the object window");
    main_window.set_application(Some(application));

    unsafe {
        g_hello_window = Some(Arc::new(HelloWindow {
            window: main_window.clone(),
            builder: builder.clone(),
            preferences: preferences.clone(),
        }));
    };

    // Subtitle of headerbar
    let header: HeaderBar = builder.object("headerbar").expect("Could not get the headerbar");
    header.set_subtitle(Some("ArkSys rolling"));

    // Load logo on first page
    {
        let logo_path = format!("{}/data/img/arka.png", PKGDATADIR);
        let logo = Pixbuf::from_file(logo_path).unwrap();
        let distribimage: gtk::Image =
            builder.object("distriblogo").expect("Could not get the distriblogo");
        distribimage.set_from_pixbuf(Some(&logo));
    }

    // Load images
    let logo_path = format!("{}/{}.png", preferences["logo_path"].as_str().unwrap(), APP_ID);
    if Path::new(&logo_path).exists() {
        let logo = Pixbuf::from_file(logo_path).unwrap();
        main_window.set_icon(Some(&logo));
    }

    let social_box: gtk::Box = builder.object("social").unwrap();
    for btn in social_box.children() {
        let name = btn.widget_name();
        let icon_path = format!("{}/data/img/{}.png", PKGDATADIR, name);
        let image: gtk::Image = builder.object(name.as_str()).unwrap();
        image.set_from_file(Some(&icon_path));
    }

    let homepage_grid: gtk::Grid = builder.object("homepage").unwrap();
    for widget in homepage_grid.children() {
        let casted_widget = widget.downcast::<gtk::Button>();
        if casted_widget.is_err() {
            continue;
        }

        let btn = casted_widget.unwrap();
        if btn.image_position() != gtk::PositionType::Right {
            continue;
        }
        let image_path = format!("{}/data/img/external-link.png", PKGDATADIR);
        let image = gtk::Image::new();
        image.set_from_file(Some(&image_path));
        image.set_margin_start(2);
        btn.set_image(Some(&image));
    }

    // Create pages
    let pages =
        format!("{}/data/pages/{}", PKGDATADIR, preferences["default_locale"].as_str().unwrap());

    for page in fs::read_dir(pages).unwrap() {
        let scrolled_window =
            gtk::ScrolledWindow::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);

        let viewport = gtk::Viewport::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);
        viewport.set_border_width(10);

        let label = gtk::Label::new(None);
        label.set_line_wrap(true);
        let image = gtk::Image::from_icon_name(Some("go-previous"), gtk::IconSize::Button);
        let back_btn = gtk::Button::new();
        back_btn.set_image(Some(&image));
        back_btn.set_widget_name("home");

        back_btn.connect_clicked(glib::clone!(@weak builder => move |button| {
            let name = button.widget_name();
            let stack: gtk::Stack = builder.object("stack").unwrap();
            stack.set_visible_child_name(&format!("{}page", name));
        }));

        let grid = gtk::Grid::new();
        grid.attach(&back_btn, 0, 1, 1, 1);
        grid.attach(&label, 1, 2, 1, 1);
        viewport.add(&grid);
        scrolled_window.add(&viewport);
        scrolled_window.show_all();

        let stack: gtk::Stack = builder.object("stack").unwrap();
        let child_name =
            format!("{}page", page.unwrap().path().file_name().unwrap().to_str().unwrap());
        stack.add_named(&scrolled_window, &child_name);
    }

    // Init translation
    gettextrs::bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR)
        .expect("Unable to switch to the text domain.");
    gettextrs::bind_textdomain_codeset(GETTEXT_PACKAGE, "UTF-8")
        .expect("Unable to set domain encoding.");
    gettextrs::textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain.");

    // Set autostart switcher state
    let autostart = Path::new(&fix_path(preferences["autostart_path"].as_str().unwrap())).exists();
    let autostart_switch: gtk::Switch = builder.object("autostart").unwrap();
    autostart_switch.set_active(autostart);

    pages::init_gpg_main_button(&builder);
    pages::init_update_sys_main_button(&builder);

    // Live systems
    if (Path::new(&preferences["live_path"].as_str().unwrap()).exists())
        && (check_regular_file(preferences["installer_path"].as_str().unwrap()))
    {
        // Hide everything that is not available on live system.
        let forum_btn: gtk::Button = builder.object("forum").unwrap();
        forum_btn.set_visible(false);
        let development_btn: gtk::Button = builder.object("development").unwrap();
        development_btn.set_visible(false);
        let update_system_btn: gtk::Button = builder.object("update-system").unwrap();
        update_system_btn.set_visible(false);
        let drivers_page_btn: gtk::Button = builder.object("driversBrowser").unwrap();
        drivers_page_btn.set_visible(false);
        let faq_page_btn: gtk::Button = builder.object("faqBrowser").unwrap();
        faq_page_btn.set_visible(false);
        let fix_gpg_keys_btn: gtk::Button = builder.object("fix-gpg-keys").unwrap();
        fix_gpg_keys_btn.set_visible(false);

        // Cleanup the grid
        homepage_grid.remove(&forum_btn);
        homepage_grid.remove(&development_btn);
        homepage_grid.remove(&update_system_btn);
        homepage_grid.remove(&drivers_page_btn);
        homepage_grid.remove(&faq_page_btn);
        homepage_grid.remove(&fix_gpg_keys_btn);

        // Add/Move needed buttons.
        let update_mirrorlist_btn: gtk::Button = builder.object("update-arch-mirrorlist").unwrap();
        let fix_vmware_res_btn = gtk::Button::with_label("VMWare Resolution Fix");
        fix_vmware_res_btn.set_visible(true);
        let fix_qemu_res_btn = gtk::Button::with_label("QEMU Resolution Fix");
        fix_qemu_res_btn.set_visible(true);
        let donate_btn = gtk::Button::with_label("Arksys on GitHub");
        donate_btn.set_visible(true);

        fix_vmware_res_btn.connect_clicked(move |_| {
            let _ = utils::run_cmd_root(String::from("systemctl enable --now vmtoolsd"));
        });
        donate_btn.connect_clicked(move |_| {
            let uri = "https://github.com/arksys-os/";
            let _ = gtk::show_uri_on_window(gtk::Window::NONE, uri, 0);
        });
        fix_qemu_res_btn.connect_clicked(move |_| {
            Exec::shell(String::from("xrandr -s 1920x1080 && xrandr --dpi 96")).join().unwrap();
        });

        homepage_grid.remove(&update_mirrorlist_btn);
        homepage_grid.attach(&fix_vmware_res_btn, 0, 5, 1, 1);
        homepage_grid.attach(&donate_btn, 1, 5, 1, 1);
        homepage_grid.attach(&fix_qemu_res_btn, 2, 5, 1, 1);

        let install_label: gtk::Label = builder.object("installlabel").unwrap();
        install_label.set_visible(true);

        let welcome_label: gtk::Label = builder.object("welcomelabel").unwrap();
        welcome_label.set_label(
            "This tool will help you install the system, it will self destruct upon successful \
             installation,\nand replaced with post-install version. We hope you enjoy your stay \
             on this cool Distro. \n
             To fix resolution right click on KDE desktop and select -Configure display settings-",
        );
        welcome_label.set_visible(true);

        let install: gtk::Button = builder.object("install").unwrap();
        install.set_visible(true);

        // Show the UI
        main_window.show();
        return;
    } else {
        let install_label: gtk::Label = builder.object("installlabel").unwrap();
        install_label.set_visible(false);

        let install: gtk::Button = builder.object("install").unwrap();
        install.set_visible(false);
    }
    pages::init_mirrorlist_main_button(&builder);
    pages::create_appbrowser_page(&builder);
    pages::create_postinstall_page(&builder);
    pages::create_drivers_page(&builder);
    pages::create_faq_page(&builder);

    // Show the UI
    main_window.show();
}

fn set_autostart(autostart: bool) {
    let autostart_path: String;
    let desktop_path: String;
    unsafe {
        autostart_path = fix_path(
            g_hello_window.clone().unwrap().preferences["autostart_path"].as_str().unwrap(),
        );
        desktop_path = g_hello_window.clone().unwrap().preferences["desktop_path"]
            .as_str()
            .unwrap()
            .to_string();
    }
    let config_dir = Path::new(&autostart_path).parent().unwrap();
    if !config_dir.exists() {
        fs::create_dir_all(config_dir).unwrap();
    }
    if autostart && !check_regular_file(autostart_path.as_str()) {
        std::os::unix::fs::symlink(desktop_path, autostart_path).unwrap();
    } else if !autostart && check_regular_file(autostart_path.as_str()) {
        std::fs::remove_file(autostart_path).unwrap();
    }
}

fn on_action_clicked(param: &[glib::Value]) -> Option<glib::Value> {
    let widget = param[0].get::<gtk::Widget>().unwrap();
    return match widget.widget_name().as_str() {
        "install" => {
            launch_installer();
            None
        },
        "autostart" => {
            let action = widget.downcast::<gtk::Switch>().unwrap();
            set_autostart(action.is_active());
            None
        },
        _ => {
            show_about_dialog();
            None
        },
    };
}

fn on_btn_clicked(param: &[glib::Value]) -> Option<glib::Value> {
    let widget = param[0].get::<gtk::Button>().unwrap();
    let name = widget.widget_name();

    unsafe {
        let stack: gtk::Stack = g_hello_window.clone().unwrap().builder.object("stack").unwrap();
        stack.set_visible_child_name(&format!("{}page", name));
    };

    None
}

fn on_link_clicked(param: &[glib::Value]) -> Option<glib::Value> {
    let widget = param[0].get::<gtk::Widget>().unwrap();
    let name = widget.widget_name();

    unsafe {
        let preferences = &g_hello_window.clone().unwrap().preferences["urls"];

        let uri = preferences[name.as_str()].as_str().unwrap();
        let _ = gtk::show_uri_on_window(gtk::Window::NONE, uri, 0);
    }

    Some(false.to_value())
}

fn on_link1_clicked(param: &[glib::Value]) -> Option<glib::Value> {
    let widget = param[0].get::<gtk::Widget>().unwrap();
    let name = widget.widget_name();

    unsafe {
        let preferences = &g_hello_window.clone().unwrap().preferences["urls"];

        let uri = preferences[name.as_str()].as_str().unwrap();
        let _ = gtk::show_uri_on_window(gtk::Window::NONE, uri, 0);
    }

    None
}

fn on_delete_window(_param: &[glib::Value]) -> Option<glib::Value> {
    Some(false.to_value())
}

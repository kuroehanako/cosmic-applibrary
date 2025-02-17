mod application;
#[rustfmt::skip]
mod config;
mod app_grid;
mod app_group;
mod desktop_entry_data;
mod app_item;
mod group_grid;
mod group_item;
mod localize;
mod utils;
mod window;
mod window_inner;

use gtk4::{gio, glib};

use self::application::CosmicAppLibraryApplication;

pub fn localize() {
    let localizer = crate::localize::localizer();
    let requested_languages = i18n_embed::DesktopLanguageRequester::requested_languages();

    if let Err(error) = localizer.select(&requested_languages) {
        eprintln!(
            "Error while loading language for pop-desktop-widget {}",
            error
        );
    }
}

fn main() {
    // Initialize logger
    pretty_env_logger::init();
    
    let _monitors = libcosmic::init();

    glib::set_application_name("Cosmic App Library");

    localize();

    gio::resources_register_include!("compiled.gresource").unwrap();

    let app = CosmicAppLibraryApplication::new();
    app.run();
}

mod imp;

use ::glib::clone;
use gio::Settings;
use gio::subclass::prelude::ObjectSubclassIsExt;
use gtk::glib;
use gtk::prelude::*;
use gtk::{Application, gio};

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

const APP_ID: &str = "org.gtk_rs.Settings";

impl Window {
    pub fn new(app: &Application) -> Self {
        let window: Self = glib::Object::builder()
            .property("application", app)
            .property("title", "Settings Example")
            .build();

        window.setup_settings();
        window.load_window_size();

        // Auto-save on close
        window.connect_close_request(clone!(@strong window => move |_| {
            let _ = window.save_window_size();
            glib::Propagation::Proceed
        }));

        window
    }

    fn setup_settings(&self) {
        let settings = Settings::new(APP_ID);
        self.imp()
            .settings
            .set(settings)
            .expect("`settings` should not be set before calling `setup_settings`.");
    }

    fn settings(&self) -> &Settings {
        self.imp()
            .settings
            .get()
            .expect("`settings` should be set in `setup_settings`.")
    }

    pub fn save_window_size(&self) -> Result<(), glib::BoolError> {
        let width = self.width();
        let height = self.height();

        if width > 0 && height > 0 {
            self.settings().set_int("window-width", width)?;
            self.settings().set_int("window-height", height)?;
        }

        self.settings()
            .set_boolean("is-maximized", self.is_maximized())?;

        println!(
            "Saved window state: {}x{}, maximized: {}",
            width,
            height,
            self.is_maximized()
        );
        Ok(())
    }

    fn load_window_size(&self) {
        let width = self.settings().int("window-width");
        let height = self.settings().int("window-height");
        let is_maximized = self.settings().boolean("is-maximized");

        let width = width.clamp(400, 2000);
        let height = height.clamp(300, 1500);

        // defer applying until shown (window is realized)
        self.connect_show(move |win| {
            win.set_default_size(width, height);
            if is_maximized {
                win.maximize();
            }
            println!(
                "Loaded window state: {}x{}, maximized: {}",
                width, height, is_maximized
            );
        });
    }
}

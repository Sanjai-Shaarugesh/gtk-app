use gio::Settings;
use gtk::glib;
use gtk::subclass::prelude::*;
use std::cell::OnceCell;

#[derive(Default)]
pub struct Window {
    pub settings: OnceCell<Settings>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "SettingsExampleWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;
}

impl ObjectImpl for Window {}
impl WidgetImpl for Window {}
impl WindowImpl for Window {}
impl ApplicationWindowImpl for Window {}

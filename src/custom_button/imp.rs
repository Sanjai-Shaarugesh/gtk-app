use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::cell::Cell;

//object holding state
#[derive(Default)]
pub struct CustomButton {
    number: Cell<i32>,
}

// Central trait for subclassing GObject
#[glib::object_subclass]
impl ObjectSubclass for CustomButton {
    const NAME: &'static str = "CButton";
    type Type = super::CustomButton;
    type ParentType = gtk::Button;
}

// Trait shared for all GObjects
impl ObjectImpl for CustomButton {
    fn constructed(&self) {
        self.parent_constructed();
        self.obj().set_label(&self.number.get().to_string());
    }
}

// Trait shared for all widgets
impl WidgetImpl for CustomButton {}

// Trait shared for all buttons
impl ButtonImpl for CustomButton {
    fn clicked(&self) {
        self.number.set(self.number.get() + 1);
        self.obj().set_label(&self.number.get().to_string());
    }
}

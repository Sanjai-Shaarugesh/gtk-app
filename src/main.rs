use crate::glib::clone;
use gio::Settings;
use gtk::Align;
use gtk::Orientation;
use gtk::Switch;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, glib};
use std::cell::Cell;
use std::rc::Rc;
// use std::thread;
// use std::time::Duration;

mod custom_button;
use custom_button::CustomButton;

// enum Value<T> {
//     bool(bool),
//     i8(i8),
//     i32(i32),
//     u32(u32),
//     i64(i64),
//     u64(u64),
//     f32(f32),
//     f64(f64),
//     //boxed types
//     String(Option<String>),
//     Object(Option<glib::Object>),
// }

const APP_ID: &str = "org.gtk_rs.Settings";

fn main() -> glib::ExitCode {
    let settings = Settings::new(APP_ID);
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(move |app| build_ui(app, &settings));
    app.run()
}

fn build_ui(app: &Application, settings: &Settings) {
    let number = Rc::new(Cell::new(0));

    let (sender, receiver) = async_channel::bounded(1);

    let button_inc = CustomButton::new();

    button_inc.set_margin_top(13);
    button_inc.set_margin_bottom(13);
    button_inc.set_margin_start(13);
    button_inc.set_margin_end(13);

    let button_dec = Button::builder()
        .label("-")
        .margin_top(13)
        .margin_bottom(13)
        .margin_start(13)
        .margin_end(13)
        .build();

    let button = Button::builder()
        .label("0")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let button_re = Button::builder()
        .label("reset")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let button_as = Button::builder()
        .label("async")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // button.connect_clicked(clone!(
    //     #[weak]
    //     number,
    //     #[weak]
    //     button_dec,
    //     #[weak]
    //     button_inc,
    //     move |_| {
    //         button_inc.set_label(&number.get().to_string());
    //     }
    // ));

    // let number_copy = number.clone();

    // button_inc.connect_clicked(move |_| number_copy.set(number_copy.get() + 1));
    // button_dec.clicked(move |_| {
    //     number.set(number.get() - 1);
    // });
    //

    // button_inc.connect_clicked(clone!(
    //     #[weak]
    //     number,
    //     move |_| {
    //         number.set(number.get() + 1);
    //         // button_dec.set_label(&number.get().to_string());
    //     }
    // ));

    // button_dec.connect_clicked(clone!(
    //     #[weak]
    //     number,
    //     move |_| {
    //         number.set(number.get() - 1);
    //         // button_inc.set_label(&number.get().to_string());
    //     }
    // ));
    //

    let switch_1 = Switch::builder()
        .margin_top(48)
        .margin_bottom(48)
        .margin_start(48)
        .margin_end(48)
        .valign(Align::Center)
        .halign(Align::Center)
        .build();

    let switch_2 = Switch::new();

    let is_switch_enabled = settings.boolean("is-switch-enabled");

    switch_1.set_active(true);

    switch_1
        .bind_property("active", &switch_2, "active")
        .bidirectional()
        .build();

    let switch = Switch::builder()
        .margin_top(48)
        .margin_bottom(48)
        .margin_start(48)
        .margin_end(48)
        .valign(Align::Center)
        .halign(Align::Center)
        .state(is_switch_enabled)
        .build();

    let switch_active = switch_1.is_active();
    println!("{}", switch_active);

    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(10)
        .margin_top(20)
        .margin_bottom(20)
        .margin_start(20)
        .margin_end(20)
        .homogeneous(false)
        .build();

    let button_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(5)
        .build();

    button_box.append(&button_inc);
    button_box.append(&button);
    button_box.append(&button_dec);
    button_box.append(&button_re);
    button_box.append(&button_as);

    let switch_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(10)
        .halign(gtk::Align::Center)
        .build();

    switch_box.append(&switch_1);
    switch_box.append(&switch_2);
    switch_box.append(&switch);

    gtk_box.append(&button_box);
    gtk_box.append(&switch_box);

    button_inc.connect_clicked(clone!(
        #[strong]
        number,
        #[weak]
        button,
        move |_| {
            number.set(number.get() + 1);
            button.set_label(&number.get().to_string());
        }
    ));

    button_dec.connect_clicked(clone!(
        #[strong]
        number,
        #[weak]
        button,
        move |_| {
            number.set(number.get() - 1);
            button.set_label(&number.get().to_string());
        }
    ));

    button_re.connect_clicked(clone!(
        #[strong]
        number,
        #[weak]
        button,
        move |_| {
            number.set(0);
            button.set_label(&number.get().to_string());
        }
    ));

    button_as.connect_clicked(move |_| {
        // The main loop executes the asynchronous block
        glib::spawn_future_local(clone!(
            #[strong]
            sender,
            async move {
                let response = reqwest::get("https://www.gtk-rs.org").await;
                sender
                    .send(response)
                    .await
                    .expect("The channel needs to be open.");
            }
        ));
    });

    // The main loop executes the asynchronous block
    glib::spawn_future_local(async move {
        while let Ok(response) = receiver.recv().await {
            if let Ok(response) = response {
                println!("Status: {}", response.status());
            } else {
                println!("Could not make a `GET` request.");
            }
        }
    });

    button.connect_clicked(clone!(
        #[strong]
        number,
        #[weak]
        button,
        move |_| {
            button.set_label(&number.get().to_string());
        }
    ));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("sanjai")
        .child(&gtk_box)
        .default_width(250)
        .default_height(150)
        .build();

    window.present();
}

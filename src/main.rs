use gtk::gdk::Display;
use crate::glib::clone;

use gio::Settings;
use gtk::Align;
use gtk::CssProvider;
use gtk::Label;
use gtk::ListItem;
use gtk::ListView;
use gtk::NoSelection;
use gtk::Orientation;
use gtk::PolicyType;
use gtk::ScrolledWindow;
use gtk::SignalListItemFactory;
use gtk::StringList;
use gtk::StringObject;
use gtk::Switch;
use gtk::Widget;
use gtk::prelude::*;
use gtk::{Button, glib};
use std::cell::Cell;
use std::rc::Rc;
use adw::HeaderBar;

mod custom_window;
mod integer_object;
mod task_object;
mod task_row;
mod window;

use custom_window::Window as CustomWindow;
use window::Window as TodoWindow;

mod custom_button;
use custom_button::CustomButton;

const APP_ID: &str = "org.gtk_rs.Settings";

fn main() -> glib::ExitCode {
    // Register resources for both applications
    gio::resources_register_include!("composite_templates_1.gresource")
        .expect("Failed to register resources.");
    gio::resources_register_include!("todo_1.gresource")
        .expect("Failed to register todo resources.");

    // Create a new application using adw::Application like in todo
    let app = adw::Application::builder().application_id(APP_ID).build();

    // Connect to signals
    app.connect_startup(setup_shortcuts_and_css);
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn setup_shortcuts_and_css(app: &adw::Application) {
    // Load CSS
    load_css();

    // Set up shortcuts (you can customize these as needed)
    app.set_accels_for_action("win.filter('All')", &["<Ctrl>a"]);
    app.set_accels_for_action("win.filter('Open')", &["<Ctrl>o"]);
    app.set_accels_for_action("win.filter('Done')", &["<Ctrl>d"]);
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("style.css"));

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &adw::Application) {
    let settings = Settings::new(APP_ID);

    // Create a new custom window
    let win = CustomWindow::new(&app.clone().upcast::<gtk::Application>());

    let number = Rc::new(Cell::new(0));
    let (sender, receiver) = async_channel::bounded(1);

    let button_inc = CustomButton::new();
    button_inc.set_margin_top(13);
    button_inc.add_css_class("destructive-action");
    button_inc.set_margin_bottom(13);
    button_inc.set_margin_start(13);
    button_inc.set_margin_end(13);
    button_inc.set_widget_name("button-1");

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

    // Add a button to open todo window
    let button_todo = Button::builder()
        .label("Open Todo")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

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

    let model: StringList = (0..=100_000).map(|number| number.to_string()).collect();

    let factory = SignalListItemFactory::new();
    factory.connect_setup(move |_, list_item| {
        // Create label
        let label = Label::new(None);
        let list_item = list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem");
        list_item.set_child(Some(&label));

        // Bind `list_item->item->string` to `label->label`
        list_item
            .property_expression("item")
            .chain_property::<StringObject>("string")
            .bind(&label, "label", Widget::NONE);
    });

    let selection_model = NoSelection::new(Some(model));
    let list_view = ListView::new(Some(selection_model), Some(factory));
    list_view.set_vexpand(true);

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .min_content_width(360)
        .child(&list_view)
        .build();
    scrolled_window.set_vexpand(true);
    scrolled_window.set_policy(PolicyType::Automatic, PolicyType::Automatic);

    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(10)
        .margin_top(20)
        .margin_bottom(20)
        .margin_start(20)
        .margin_end(20)
        .homogeneous(false)
        .build();

    gtk_box.set_vexpand(true);

    let button_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(5)
        .build();

    button_box.append(&button_inc);
    button_box.append(&button);
    button_box.append(&button_dec);
    button_box.append(&button_re);
    button_box.append(&button_as);
    button_box.append(&button_todo);

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

    // Button event handlers
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

    // Todo button handler
    button_todo.connect_clicked(clone!(
        #[weak]
        app,
        move |_| {
            let todo_window = TodoWindow::new(&app);
            todo_window.present();
        }
    ));

    button_as.connect_clicked(move |_| {
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



    // Create additional window for scrolled content
    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("sanjai")
        .content(&scrolled_window)
        .default_width(600)
        .default_height(300)
        .build();

    window.present();

    // Set up the main custom window
    win.set_child(Some(&gtk_box));
    win.set_default_size(300, 400);
    win.present();
}
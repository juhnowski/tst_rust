mod db;
use db::db_init;


use log::{error, info, warn, LevelFilter};
use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Root},
    encode::json::JsonEncoder,
};

#[macro_use]
extern crate glib;
extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;

use gtk::{
        Application,
        ButtonsType,
        DialogFlags,
        MessageType, 
        MessageDialog, 
        FileChooserDialog,
        FileChooserAction,
        ResponseType,
        Window,
        ListBox
    };

use std::env;

use std::env::args;

use row_data::RowData;

fn build_ui() {


if gtk::init().is_err() {
    error!("Failed to initialize GTK.");
    return;
}
let glade_src = include_str!("builder_basics.glade");
let builder = gtk::Builder::from_string(glade_src);

let window: gtk::Window = builder.get_object("window1").unwrap();
window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(320, 480);

let button: gtk::Button = builder.get_object("button1").unwrap();
let dialog: gtk::MessageDialog = builder.get_object("messagedialog1").unwrap();

let revert: gtk::Button = builder.get_object("button_revert").unwrap();
let udp_ok: gtk::Button = builder.get_object("button_udp_ok").unwrap();

let host: gtk::Entry  = builder.get_object("host").unwrap();
let port: gtk::Entry  = builder.get_object("port").unwrap();

let open: gtk::Button = builder.get_object("button_open").unwrap();

let app_open_dialog: gtk::MessageDialog = builder.get_object("app_open_dialog").unwrap();
let app_open_sender: gtk::Button = builder.get_object("app_open_sender").unwrap();
let app_open_receiver: gtk::Button = builder.get_object("app_open_receiver").unwrap();

let dlg = app_open_dialog.clone();
let dlg1 = app_open_dialog.clone();
let dialog_udp = dialog.clone();

app_open_sender.connect_clicked(move |_| {
    info!("sender");
    dlg.hide();   
});

app_open_receiver.connect_clicked(move |_| {
    info!("receiver");
    dlg1.hide();
});


app_open_dialog.run();
app_open_dialog.hide();

db_init();

button.connect_clicked(move |_| {
    dialog.run();
    dialog.hide();
});

revert.connect_clicked(move |_| {
    host.set_text("127.0.0.1");
    port.set_text("7001");
});

udp_ok.connect_clicked(move |_| {
    dialog_udp.hide();
});

open.connect_clicked(move |_| {
    let open_dialog = FileChooserDialog::with_buttons::<Window>(
        Some("Open File"),
        None,
        FileChooserAction::Open,
        &[("_Cancel", ResponseType::Cancel), ("_Open", ResponseType::Accept)]
    );
    open_dialog.run();
    open_dialog.hide();
});


let model = gio::ListStore::new(RowData::static_type());
let listbox: gtk::ListBox = builder.get_object("listbox").unwrap();

window.show_all();

}


fn main() {
    let stdout: ConsoleAppender = ConsoleAppender::builder()
    .encoder(Box::new(JsonEncoder::new()))
    .build();
let log_config = log4rs::config::Config::builder()
    .appender(Appender::builder().build("stdout", Box::new(stdout)))
    .build(Root::builder().appender("stdout").build(LevelFilter::Info))
    .unwrap();
log4rs::init_config(log_config).unwrap();

info!("App started");
/*    warn!("Warn log with value {}", "test");
error!("ERROR!");
*/

    build_ui();
    gtk::main();
}

mod row_data {
    use super::*;

    use glib::subclass;
    use glib::subclass::prelude::*;
    use glib::translate::*;

    // Implementation sub-module of the GObject
    mod imp {
        use super::*;
        use std::cell::RefCell;

        // The actual data structure that stores our values. This is not accessible
        // directly from the outside.
        pub struct RowData {
            name: RefCell<Option<String>>,
            count: RefCell<u32>,
        }

        // GObject property definitions for our two values
        static PROPERTIES: [subclass::Property; 2] = [
            subclass::Property("name", |name| {
                glib::ParamSpec::string(
                    name,
                    "Name",
                    "Name",
                    None, // Default value
                    glib::ParamFlags::READWRITE,
                )
            }),
            subclass::Property("count", |name| {
                glib::ParamSpec::uint(
                    name,
                    "Count",
                    "Count",
                    0,
                    100,
                    0, // Allowed range and default value
                    glib::ParamFlags::READWRITE,
                )
            }),
        ];

        // Basic declaration of our type for the GObject type system
        impl ObjectSubclass for RowData {
            const NAME: &'static str = "RowData";
            type ParentType = glib::Object;
            type Instance = subclass::simple::InstanceStruct<Self>;
            type Class = subclass::simple::ClassStruct<Self>;

            glib_object_subclass!();

            // Called exactly once before the first instantiation of an instance. This
            // sets up any type-specific things, in this specific case it installs the
            // properties so that GObject knows about their existence and they can be
            // used on instances of our type
            fn class_init(klass: &mut Self::Class) {
                klass.install_properties(&PROPERTIES);
            }

            // Called once at the very beginning of instantiation of each instance and
            // creates the data structure that contains all our state
            fn new() -> Self {
                Self {
                    name: RefCell::new(None),
                    count: RefCell::new(0),
                }
            }
        }

        // The ObjectImpl trait provides the setters/getters for GObject properties.
        // Here we need to provide the values that are internally stored back to the
        // caller, or store whatever new value the caller is providing.
        //
        // This maps between the GObject properties and our internal storage of the
        // corresponding values of the properties.
        impl ObjectImpl for RowData {
            glib_object_impl!();

            fn set_property(&self, _obj: &glib::Object, id: usize, value: &glib::Value) {
                let prop = &PROPERTIES[id];

                match *prop {
                    subclass::Property("name", ..) => {
                        let name = value
                            .get()
                            .expect("type conformity checked by `Object::set_property`");
                        self.name.replace(name);
                    }
                    subclass::Property("count", ..) => {
                        let count = value
                            .get_some()
                            .expect("type conformity checked by `Object::set_property`");
                        self.count.replace(count);
                    }
                    _ => unimplemented!(),
                }
            }

            fn get_property(&self, _obj: &glib::Object, id: usize) -> Result<glib::Value, ()> {
                let prop = &PROPERTIES[id];

                match *prop {
                    subclass::Property("name", ..) => Ok(self.name.borrow().to_value()),
                    subclass::Property("count", ..) => Ok(self.count.borrow().to_value()),
                    _ => unimplemented!(),
                }
            }
        }
    }

    // Public part of the RowData type. This behaves like a normal gtk-rs-style GObject
    // binding
    glib_wrapper! {
        pub struct RowData(Object<subclass::simple::InstanceStruct<imp::RowData>, subclass::simple::ClassStruct<imp::RowData>, RowDataClass>);

        match fn {
            get_type => || imp::RowData::get_type().to_glib(),
        }
    }

    // Constructor for new instances. This simply calls glib::Object::new() with
    // initial values for our two properties and then returns the new instance
    impl RowData {
        pub fn new(name: &str, count: u32) -> RowData {
            glib::Object::new(Self::static_type(), &[("name", &name), ("count", &count)])
                .expect("Failed to create row data")
                .downcast()
                .expect("Created row data is of wrong type")
        }
    }
}
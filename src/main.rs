
use log::{error, info, warn, LevelFilter};
use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Root},
    encode::json::JsonEncoder,
};

extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;

use gtk::{ButtonsType, DialogFlags, MessageType, MessageDialog, Window};

use std::env;

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

if gtk::init().is_err() {
    error!("Failed to initialize GTK.");
    return;
}
let glade_src = include_str!("builder_basics.glade");
let builder = gtk::Builder::from_string(glade_src);

let window: gtk::Window = builder.get_object("window1").unwrap();
let button: gtk::Button = builder.get_object("button1").unwrap();
let dialog: gtk::MessageDialog = builder.get_object("messagedialog1").unwrap();

let revert: gtk::Button = builder.get_object("button_revert").unwrap();
let host: gtk::Entry  = builder.get_object("host").unwrap();
let port: gtk::Entry  = builder.get_object("port").unwrap();

button.connect_clicked(move |_| {
    dialog.run();
    dialog.hide();
});

revert.connect_clicked(move |_| {
    host.set_text("127.0.0.1")
    port.set_text("7001")
});


window.show_all();

gtk::main();

}
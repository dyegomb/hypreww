#![doc = include_str ! ("../README.md")]


#[allow(unused, unused_imports)]

use hyprland::data::{Client, Clients};
use hyprland::prelude::*;
use hyprland::event_listener::EventListenerMutable as EventListener;
use hyprland::shared::HyprError;
use freedesktop_icon_lookup::{Cache, LookupParam};
use std::path::PathBuf;
//use gtk::{self, prelude::*};

fn main() -> hyprland::Result<()> {
    //let _ = gtk::init();
    //let teste = gtk::Image::from_icon_name("firefox");
    //println!("{teste:?}");
    //println!("{:?}", teste.icon_name());
    //println!("{:?}", teste.icon_size());
    let clients = Clients::get().unwrap();
    println!("{:?}", &clients);
    //println!("{:?}", &clients.count());

    clients
        .iter()
        .for_each(|c| {
            println!("{:?}", c.class);
            println!("{:?}", c.title);
        });
    
    // Create a event listener
    let mut event_listener = EventListener::new();

    //event_listener.add_window_open_handler(f);

    //event_listener.start_listener()


    let theme = "Adwaita";
    let mut cache = Cache::new().unwrap();
    cache.load(theme).unwrap_or(());
    let icon: Option<PathBuf> = cache.lookup("firefox", theme);

    println!("{:?}", icon);

    Ok(())

}


//use gtk::{glib, prelude::*};
//
//fn main() -> glib::ExitCode {
//    let application = gtk::Application::builder()
//        .application_id("com.github.gtk-rs.examples.basic")
//        .build();
//    application.connect_activate(build_ui);
//    application.run()
//}
//
//fn build_ui(application: &gtk::Application) {
//    let window = gtk::ApplicationWindow::new(application);
//
//    window.set_title(Some("First GTK Program"));
//    window.set_default_size(350, 70);
//
//    let button = gtk::Button::with_label("Click me!");
//    let img = gtk::Image::from_icon_name("firefox");
//
//    window.set_child(Some(&button));
//    window.set_child(Some(&img));
//
//    window.present();
//}

#![doc = include_str ! ("../README.md")]
#[allow(unused, unused_imports)]
//use freedesktop_icon_lookup::{Cache, LookupParam};
use clap::{arg, Arg, ArgGroup, Command, FromArgMatches as _};
use clap::{Args, Parser, Subcommand, ValueEnum};
use core::panic;
use freedesktop_icons::lookup;
use hyprland::data::{Client, Clients};
use hyprland::event_listener::{EventListenerMutable as EventListener, State, WindowOpenEvent};
use hyprland::prelude::*;
use hyprland::shared::HyprError;
use serde::de::value;
use std::path::PathBuf;
use std::str::FromStr;

mod workspaces;
use workspaces::prelude::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliArgs {
    #[command(subcommand)]
    cmd: Subcmds,
    //#[arg(value_enum)]
    //action: Action,
}

//https://docs.rs/clap/latest/clap/_derive/_cookbook/typed_derive/index.html
#[derive(Clone, Debug, Subcommand)]
enum Subcmds {
    #[command(alias("ws"))]
    Workspaces(WorkspacesActions),
    #[command(alias("w"))]
    Windows(WindowsActions),
}

#[derive(Debug, ValueEnum, Clone)]
enum Action {
    Listen,
    Show,
    Active,
}

// Derive attributes
// https://docs.rs/clap/latest/clap/_derive/index.html#attributes

#[derive(Debug, Clone, Args)]
struct WorkspacesActions {
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    listen: bool,
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    show: bool,
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    active: bool,
}

#[derive(Debug, Clone, Args)]
struct WindowsActions {
    //#[arg(short, long)]
    action: WindowsArgs,
}

#[derive(Debug, Clone, ValueEnum)]
enum WorkspacesArgs {
    #[value(alias("l"))]
    Listen,
    Show,
    Active,
}

#[derive(Debug, Clone, ValueEnum)]
enum WindowsArgs {
    Listen,
    Show,
    Active,
}

fn list_apps() -> Vec<Client> {
    let clients = Clients::get();

    match clients {
        Ok(clients) => clients
            .into_iter()
            .filter(|c| c.class != "")
            .collect::<Vec<Client>>(),
        Err(_) => Vec::new(),
    }
}

fn get_icon(client: &Client, theme: &str) -> PathBuf {
    let client_class = client.initial_class.to_lowercase();
    let default_icon = PathBuf::from_str("/usr/share/weston/wayland.svg");
    match lookup(&client_class).with_theme(theme).find() {
        Some(icon_path) => icon_path,
        None => match lookup(&client_class).find() {
            Some(icon_path) => icon_path,
            None => lookup(client.initial_title.to_lowercase().as_str())
                .with_theme(theme)
                .find()
                .unwrap_or(default_icon.unwrap()),
        },
    }

    //.unwrap_or(PathBuf::from_str("/usr/share/weston/wayland.svg").unwrap())
}

//fn windows_list(a: &mut State, b: WindowOpenEvent ) -> () {
fn windows_list() -> () {
    //println!("a: {:?}, b: {:?}", a, b);
    list_apps()
        .iter()
        .map(|c| {
            (
                c.address.to_string(),
                &c.class,
                get_icon(c, "suru-4all-dark"),
            )
        })
        .for_each(|i| println!("{:?}", i));
}

fn main() -> hyprland::Result<()> {
    //let _ = gtk::init();
    //let teste = gtk::Image::from_icon_name("firefox");
    //println!("{teste:?}");
    //println!("{:?}", teste.icon_name());
    //println!("{:?}", teste.icon_size());
    //let clients = Clients::get().unwrap();
    //println!("{:?}", &clients);
    //println!("{:?}", &clients.count());

    //let clients = list_apps();
    //clients.iter().for_each(|c| {
    //    println!("{:?}", c.class);
    //println!("{:?}", c.title);
    //});

    //    let window_list = |a: &_,b: &_| {
    //        println!("a: {:?}, b: {:?}", a, b);
    //        list_apps()
    //        .iter()
    //        .map(|c| {
    //            (
    //                c.address.to_string(),
    //                &c.class,
    //                get_icon(c, "suru-4all-dark"),
    //            )
    //        })
    //        .for_each(|i| println!("{:?}", i))};
    //
    //    window_list("","");
    //

    let cli = CliArgs::parse();

    println!("{:?}", cli);

    //let cli_matches = Command::new("Hypreww")
    //    //.group(ArgGroup::new("workspaces").conflicts_with("windows"))
    //    //.group(ArgGroup::new("windows").conflicts_with("workspaces"))
    //    //.arg(Arg::new("listen").short('l'))
    //    //.arg(Arg::new("get").short('g').conflicts_with("listen"))
    //    //.get_matches();
    //    .subcommand(
    //        Command::new("workspaces")
    //            .arg(Arg::new("show").short('s').exclusive(true))
    //            .arg(Arg::new("get-active").short('g').exclusive(true))
    //            .arg(Arg::new("listen").short('l').exclusive(true)),
    //    )
    //    .subcommand(
    //        Command::new("windows")
    //            .arg(Arg::new("listen").exclusive(true))
    //            .arg(Arg::new("show").exclusive(true)),
    //    )
    //    .get_matches();

    //println!("{:?}", cli_matches);
    //println!(
    //    "{:?}",
    //    cli_matches
    //        //.subcommand_matches("workspaces")
    //        .subcommand()
    //        .unwrap()
    //        //.get_occurrences::<String>("listen")
    //);

    //if let Some((subcmd, args)) = cli_matches.subcommand() {
    //    match subcmd {
    //        "workspaces" => println!("Workspaces: {:?}", args.ids()),
    //        "windows" => println!("Windows: {:?}", args),
    //        _ => unreachable!("Ooops!"),
    //    }
    //}

    Ok(())
    //workspaces::listen_workspaces(9)
    //workspaces::listen_active();

    // Create a event listener
    //let mut event_listener = EventListener::new();
    //event_listener.add_window_open_handler(|_, _| windows_list());
    //event_listener.start_listener()

    //let theme = "suru-4all-dark";
    //let mut cache = Cache::new().unwrap();
    //cache.load(theme).unwrap();
    //let icon: Option<PathBuf> = cache.lookup("firefox", theme);
    //let icon2: Option<PathBuf> = cache.lookup("kitty", theme);
    //let icon3 = lookup("firefox").find();
    //let icon4 = lookup("Wayland").find();

    //println!("{:?}", icon);
    //println!("{:?}", icon2);
    //println!("{:?}", icon3);
    //println!("{:?}", icon4);
    //
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

#![doc = include_str ! ("../README.md")]

//use freedesktop_icon_lookup::{Cache, LookupParam};
use clap::{arg, Args, Parser, Subcommand, ValueEnum};
//use core::panic;
//use std::any::Any;
//use freedesktop_icons::lookup;
//use hyprland::data::{Client, Clients};
//use hyprland::event_listener::{EventListenerMutable as EventListener, State, WindowOpenEvent};
//use hyprland::prelude::*;
//use hyprland::shared::HyprError;
//use serde::de::value;
//use std::path::PathBuf;
//use std::str::FromStr;

mod workspaces;
//use workspaces::prelude::*;
mod windows;

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

//#[derive(Debug, ValueEnum, Clone)]
//enum Action {
//    Listen,
//    Show,
//    Active,
//}

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
    #[arg(short, long, num_args(2))]
    change: Vec<String>,
}

#[derive(Debug, Clone, Args)]
struct WindowsActions {
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    listen: bool,
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    show: bool,
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    active: bool,
}

//#[derive(Debug, Clone, Args)]
//struct WindowsActions {
//    //#[arg(short, long)]
//    action: WindowsArgs,
//}

//#[derive(Debug, Clone, ValueEnum)]
//enum WorkspacesArgs {
//    #[value(alias("l"))]
//    Listen,
//    Show,
//    Active,
//}
//
//#[derive(Debug, Clone, ValueEnum)]
//enum WindowsArgs {
//    Listen,
//    Show,
//    Active,
//}

fn main() {
    let cli = CliArgs::parse();

    //println!("{:?}", cli);

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

    match cli.cmd {
        Subcmds::Windows(actions) => {
            if actions.show {
                windows::windows_list("suru-4all-dark");
            }
            if actions.active {}
            if actions.listen {}
        }
        Subcmds::Workspaces(actions) => {
            if actions.show {
                workspaces::get_workspaces(9);
            }
            if !actions.change.is_empty() {
                println!("{:?}", actions.change);
            }
            if actions.active {
                let _ = workspaces::listen_active();
            }
            if actions.listen {
                let _ = workspaces::listen_workspaces(9);
            }
        }
    };
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

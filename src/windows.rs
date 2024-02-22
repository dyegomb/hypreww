use freedesktop_icons::lookup;
use hyprland::data::{Client, Clients};
use hyprland::event_listener::{EventListenerMutable as EventListener, State, WindowOpenEvent};
use hyprland::prelude::*;
use hyprland::shared::HyprError;
use serde::de::value;
use std::path::PathBuf;
use std::str::FromStr;

pub fn list_apps() -> Vec<Client> {
    let clients = Clients::get();

    match clients {
        Ok(clients) => clients
            .into_iter()
            .filter(|c| c.class != "")
            .collect::<Vec<Client>>(),
        Err(_) => Vec::new(),
    }
}

pub fn get_icon(client: &Client, theme: &str) -> PathBuf {
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
pub fn windows_list() -> () {
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

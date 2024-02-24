use freedesktop_icons::lookup;
use hyprland::data::{Client, Clients};
use hyprland::prelude::*;
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
}

pub fn windows_list(theme: &str) -> () {
    list_apps()
        .iter()
        .map(|c| {
            (
                c.address.to_string(),
                &c.class,
                get_icon(c, theme),
            )
        })
        .for_each(|i| println!("{:?}", i));
}

use freedesktop_icons::lookup;
use hyprland::data::{Client, Clients};
use hyprland::dispatch::{Dispatch, DispatchType, WindowIdentifier};
use hyprland::prelude::*;
use hyprland::shared::Address;
use serde::Serialize;
use std::path::PathBuf;
use std::rc::Rc;
use std::str::FromStr;

#[derive(Serialize)]
struct Task {
    address: String,
    class: String,
    title: String,
    icon: PathBuf,
}

fn list_apps() -> Vec<Client> {
    let clients = Clients::get();

    match clients {
        Ok(clients) => clients
            .into_iter()
            .filter(|c| !c.class.is_empty())
            .collect::<Vec<Client>>(),
        Err(_) => Vec::new(),
    }
}

fn get_icon_c(client: &Client, theme: &str) -> PathBuf {
    let client_class = client.initial_class.to_lowercase();
    let default_icon = PathBuf::from_str("~/.local/share/icons/defaul.svg").unwrap();
    match lookup(&client_class).with_theme(theme).find() {
        Some(icon_path) => icon_path,
        None => match lookup(&client_class).find() {
            Some(icon_path) => icon_path,
            None => lookup(client.initial_title.to_lowercase().as_str())
                .with_theme(theme)
                .find()
                .unwrap_or(default_icon),
        },
    }
}

pub fn get_icon(client: &str, theme: &str) -> PathBuf {
    let client_class = client.to_lowercase();
    let default_icon = PathBuf::from_str("~/.local/share/icons/defaul.svg").unwrap();
    match lookup(&client_class).with_theme(theme).find() {
        Some(icon_path) => icon_path,
        None => match lookup(&client_class).find() {
            Some(icon_path) => icon_path,
            None => lookup(&client_class)
                .with_theme(theme)
                .find()
                .unwrap_or(default_icon),
        },
    }
}

pub fn windows_list(theme: &str) {
    let jsonfy = list_apps()
        .iter()
        .map(|c| Task {
            address: c.address.to_string(),
            class: c.class.to_owned(),
            title: c.title.to_owned(),
            icon: get_icon_c(c, theme),
        })
        .collect::<Vec<_>>();
    println!(
        "{}",
        serde_json::to_string(&jsonfy).unwrap_or("[]".to_string())
    );
}

pub fn listen(theme: &str) -> hyprland::Result<()> {
    let mut listener = hyprland::event_listener::EventListenerMutable::new();
    let theme_rc = Rc::new(theme.to_string());

    windows_list(theme);

    let theme = Rc::clone(&theme_rc);
    listener.add_window_moved_handler(move |_, _| windows_list(&theme));
    let theme = Rc::clone(&theme_rc);
    listener.add_window_open_handler(move |_, _| windows_list(&theme));
    let theme = Rc::clone(&theme_rc);
    listener.add_window_close_handler(move |_, _| windows_list(&theme));

    listener.start_listener()
}

pub fn window_change(address: &str) -> hyprland::Result<()> {
    Dispatch::call(DispatchType::FocusWindow(WindowIdentifier::Address(
        Address::new(address),
    )))
}

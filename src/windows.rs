use freedesktop_icons::lookup;
use hyprland::data::{Client, Clients};
use hyprland::prelude::*;
use serde::Serialize;
use std::path::PathBuf;
use std::str::FromStr;
//use std::sync::Arc;
use std::rc::Rc;

#[derive(Serialize)]
struct Task {
    address: String,
    class: String,
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
}

pub fn windows_list(theme: &str) {
    let jsonfy = list_apps()
        .iter()
        .map(|c| Task {
            address: c.address.to_string(),
            class: c.class.to_owned(),
            icon: get_icon(c, theme),
        })
        .collect::<Vec<_>>();
    println!(
        "{}",
        serde_json::to_string(&jsonfy).unwrap_or("[]".to_string())
    );
}

pub fn listen(theme: &str) -> Result<(), hyprland::shared::HyprError> {
    let mut listener = hyprland::event_listener::EventListenerMutable::new();
    let theme_rc = Rc::new(theme.to_string());

    let theme = Rc::clone(&theme_rc);
    listener.add_window_moved_handler(move |_, _| windows_list(&theme));
    let theme = Rc::clone(&theme_rc);
    listener.add_window_open_handler(move |_, _| windows_list(&theme));
    let theme = Rc::clone(&theme_rc);
    listener.add_window_close_handler(move |_, _| windows_list(&theme));

    listener.start_listener()
}

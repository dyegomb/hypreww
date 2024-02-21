// [{"id":"1","windows":2},{"id":"2","windows":1},{"id":"3","windows":0},{"id":"4","windows":0},{"id":"5","windows":0},{"id":"6","windows":0},{"id":"7","windows":0},{"id":"8","windows":0},{"id":"9","windows":0}]
//
use hyprland::data::{Client, Clients};
use hyprland::data::{Workspace, Workspaces};
//use hyprland::event_listener::{EventListenerMutable as EventListener, State, WindowOpenEvent};
use hyprland::prelude::*;
use hyprland::shared::Address;
use serde::Serialize;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::default;

pub mod prelude {}

#[derive(Serialize, Eq, PartialOrd, Clone)]
struct SimpleWindow {
    id: i32,
    windows: u16,
}

impl PartialEq for SimpleWindow {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Ord for SimpleWindow {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

fn get_workspaces(num: usize) {
    let mut windows = BTreeMap::new();
    let mut jsonfy = Vec::with_capacity(num);
    match Workspaces::get() {
        Ok(result) => {
            result
                .iter()
                .filter(|w| w.id > 0 && w.id <= num as i32)
                .for_each(|f| {
                    windows.insert(
                        f.id,
                        SimpleWindow {
                            id: f.id,
                            windows: f.windows,
                        },
                    );
                });

            (1..=num).for_each(|id| match windows.get(&(id as i32)) {
                Some(window) => jsonfy.push(window.to_owned()),
                None => jsonfy.push(SimpleWindow {
                    id: id as i32,
                    windows: 0,
                }),
            });

            println!(
                "{}",
                serde_json::to_string(&jsonfy).unwrap_or("[]".to_string())
            );
        }
        Err(_) => {
            println!("[{{\"id\": 1, \"windows\":0}}]");
        }
    }
}

pub fn listen_workspaces(num: usize) -> hyprland::Result<()> {
    let mut listener = hyprland::event_listener::EventListenerMutable::new();

    listener.add_window_moved_handler(move |_, _| get_workspaces(num));
    listener.add_window_open_handler(move |_, _| get_workspaces(num));
    listener.add_window_close_handler(move |_, _| get_workspaces(num));

    get_workspaces(num);
    listener.start_listener()
}

pub fn listen_active() -> hyprland::Result<()> {
    let mut listener = hyprland::event_listener::EventListenerMutable::new();

    listener.add_workspace_change_handler(|w, _| {
        println!("{}", w.to_string());
    });
    listener.start_listener()
}

// [{"id":"1","windows":2},{"id":"2","windows":1},{"id":"3","windows":0},{"id":"4","windows":0},{"id":"5","windows":0},{"id":"6","windows":0},{"id":"7","windows":0},{"id":"8","windows":0},{"id":"9","windows":0}]
//
use hyprland::data::{Client, Clients};
use hyprland::data::{Workspace, Workspaces};
//use hyprland::event_listener::{EventListenerMutable as EventListener, State, WindowOpenEvent};
use hyprland::prelude::*;
use serde::Serialize;
use std::collections::BTreeMap;

pub mod prelude {}

#[derive(Serialize, Ord, Eq, PartialEq, PartialOrd, Clone)]
struct SimpleWindow {
    id: i32,
    windows: u16,
}


fn list_workspaces(num: usize) {
    let mut windows = BTreeMap::new();
    let mut jsonfy = Vec::with_capacity(num);
    match Workspaces::get() {
        Ok(result) => {
            //println!("{:?}", result);
            result.iter()
                .filter(|w| w.id > 0 && w.id <= num as i32)
                .for_each(|f| {
                    windows.insert(f.id, SimpleWindow {
                        id: f.id,
                        windows: f.windows,
                    });
            });

            (0..=num).for_each(|id|{
                match windows.get(&(id as i32)) {
                   Some(window) => jsonfy.push(window.to_owned()),
                   None => jsonfy.push(SimpleWindow{id: id as i32, windows: 0})
                }
            });

            println!(
                "{}",
                serde_json::to_string(&jsonfy).unwrap_or("[]".to_string())
            );
        },
        Err(_) => {
            println!("{}", serde_json::json!([{"id":"1", "windows":0}]));
        }
    }

}

pub fn listen() {
    list_workspaces(9);
}

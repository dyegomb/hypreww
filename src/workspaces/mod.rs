// [{"id":"1","windows":2},{"id":"2","windows":1},{"id":"3","windows":0},{"id":"4","windows":0},{"id":"5","windows":0},{"id":"6","windows":0},{"id":"7","windows":0},{"id":"8","windows":0},{"id":"9","windows":0}]
//
use hyprland::data::{Client, Clients};
use hyprland::data::{Workspace, Workspaces};
//use hyprland::event_listener::{EventListenerMutable as EventListener, State, WindowOpenEvent};
use hyprland::prelude::*;
use serde::{Deserialize, Serialize};

pub mod prelude {}

#[derive(Serialize)]
struct SimpleWindow {
    id: i32,
    windows: u16,
}

fn list_workspaces() {
    let mut jsonfy = vec![];
    match Workspaces::get() {
        Ok(result) => {
            //println!("{:?}", result);
            result.iter().for_each(|f| {
                jsonfy.push(SimpleWindow {
                    id: f.id,
                    windows: f.windows,
                })
            });
            println!(
                "{}",
                serde_json::to_string(&jsonfy).unwrap_or("[]".to_string())
            );
        }
        Err(_) => {
            println!("{}", serde_json::json!([{"id":"1", "windows":0}]));
        }
    }
}

pub fn listen() {
    list_workspaces();
}

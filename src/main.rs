#![doc = include_str ! ("../README.md")]

use clap::{arg, Args, Parser, Subcommand};
use windows::{get_icon, window_change};

mod windows;
mod workspaces;

#[derive(Parser, Debug)]
#[command(version, about = "A helper to integrate Hyprland to Eww", long_about = None)]
struct CliArgs {
    #[command(subcommand)]
    cmd: Subcmds,
}

//https://docs.rs/clap/latest/clap/_derive/_cookbook/typed_derive/index.html
#[derive(Clone, Debug, Subcommand)]
enum Subcmds {
    #[command(alias("ws"))]
    Workspaces(WorkspacesActions),
    #[command(alias("w"))]
    Windows(WindowsActions),
}

// Derive attributes
// https://docs.rs/clap/latest/clap/_derive/index.html#attributes

#[derive(Debug, Clone, Args)]
struct WorkspacesActions {
    #[arg(short, long, action = clap::ArgAction::SetTrue, help="start listening to workspaces changes")]
    listen: bool,
    #[arg(short, long, action = clap::ArgAction::SetTrue, help="show workspaces states")]
    show: bool,
    #[arg(short, long, action = clap::ArgAction::SetTrue, help="start listen to active workspace")]
    active: bool,
    #[arg(
        short,
        long,
        num_args(2),
        help = "(up|down) (1-9)  change active workspace up or down based on current"
    )]
    change: Vec<String>,
}

#[derive(Debug, Clone, Args)]
struct WindowsActions {
    #[arg(short, long, action=clap::ArgAction::SetTrue, help="listen to change on window clients")]
    listen: bool,
    #[arg(short, long, action=clap::ArgAction::SetTrue, help="show current windows states")]
    show: bool,
    #[arg(short, long, action=clap::ArgAction::SetTrue, help="show current active window")]
    active: bool,
    #[arg(short, long, help="specify icon theme")]
    icon_theme: Option<String>,
    #[arg(short, long, help="get the icon file for a specific application")]
    which_icon: Option<String>,
    #[arg(short, long, help="change current active window to informed address")]
    change: Option<String>,
}

fn main() {
    let cli = CliArgs::parse();

    match cli.cmd {
        Subcmds::Windows(actions) => {
            if actions.show {
                if let Some(ref theme) = actions.icon_theme {
                    windows::windows_list(theme);
                } else {
                    windows::windows_list("hicolor");
                }
            }
            if let Some(address) = actions.change {
                let _ = window_change(&address);
            }
            if actions.active {
                unimplemented!()
            }
            if actions.which_icon.is_some() {
                if let (Some(theme), Some(ref app)) = (&actions.icon_theme, &actions.which_icon) {
                    println!("{}", get_icon(app, theme).to_string_lossy());
                } else {
                    println!(
                        "{}",
                        get_icon(&actions.which_icon.unwrap(), "hicolor").to_string_lossy()
                    );
                }
            }
            if actions.listen {
                if let Some(ref theme) = actions.icon_theme {
                    let _ = windows::listen(theme);
                } else {
                    let _ = windows::listen("hicolor");
                }
            }
        }
        Subcmds::Workspaces(actions) => {
            let workspaces_num = 9;
            if actions.show {
                workspaces::get_workspaces(workspaces_num);
            }
            if !actions.change.is_empty() {
                let _ = workspaces::change_active_workspace(
                    workspaces_num,
                    actions.change[1].parse().unwrap_or(1),
                    &actions.change[0],
                );
            }
            if actions.active {
                let _ = workspaces::listen_active();
            }
            if actions.listen {
                let _ = workspaces::listen_workspaces(workspaces_num);
            }
        }
    };
}

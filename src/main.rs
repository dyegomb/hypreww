#![doc = include_str ! ("../README.md")]

use clap::{arg, Args, Parser, Subcommand, ValueEnum};

mod workspaces;
mod windows;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
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


fn main() {
    let cli = CliArgs::parse();

    match cli.cmd {
        Subcmds::Windows(actions) => {
            if actions.show {
                windows::windows_list("suru-4all-dark");
            }
            if actions.active {}
            if actions.listen {}
        }
        Subcmds::Workspaces(actions) => {
            let workspaces_num = 9;
            if actions.show {
                workspaces::get_workspaces(workspaces_num);
            }
            if !actions.change.is_empty() {
                //println!("{:?}", actions.change);
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


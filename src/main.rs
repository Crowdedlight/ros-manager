
use slint::{Model, ComponentHandle};
use std::{cell::RefCell, rc};
use std::fs;
use std::ops::Deref;

use crate::terminalhandler::launch_sourced;
use crate::terminalhandler::launch_build;

mod ui;
mod workspacehandler;
mod terminalhandler;
mod config;

// The odd issue with not being able to borrow_mut and call functions comes when BorrowMut is in "use" - https://github.com/rust-lang/rust/issues/39232

fn main() {
    let ui = ui::MainWindow::new();
    let ui_weak: slint::Weak<ui::MainWindow> = ui.as_weak();

    // get collection from slint as starting base
    let ws_list_initial: Vec<ui::WorkspaceItem> = ui.get_workspace_list().iter().collect();

    // CONFIG - load values
    let mut settings = config::Cfg {ws_root_path: "".parse().unwrap() };
    settings.init();
    println!("{:?}", settings.ws_root_path.clone());

    let ws_state = rc::Rc::new(RefCell::new(workspacehandler::WorkspaceState {
        workspaces: Vec::<ui::WorkspaceItem>::from(ws_list_initial),
        main_window: ui_weak,
        ws_root_path: settings.ws_root_path.clone(),
        settings,
    }));

    // initial load
    ws_state.clone().borrow_mut().load_workspaces();

    // clone weak reference as we move it into the function and can't use it afterwards
    let state_build_weak = ws_state.clone(); 
    ui.on_ros_workspace_build(move |path| {
        // get workspace item so we can pass it to terminal handler
        let state = state_build_weak.borrow();
        let ws_item = state.get_ws_item_from_path(path);

        if ws_item.is_some() {
            launch_build(ws_item.unwrap());            
        }
    });

    // clone weak reference as we move it into the function and can't use it afterwards
    let state_source_weak = ws_state.clone();
    ui.on_ros_workspace_sourced(move |path| {
        // get workspace item so we can pass it to terminal handler
        let state = state_source_weak.borrow();
        let ws_item = state.get_ws_item_from_path(path);

        if ws_item.is_some() {
            launch_sourced(ws_item.unwrap());            
        }
    });

    let mut state_path_weak = ws_state.clone();
    ui.on_set_workspace_path(move || {
        let res = rfd::FileDialog::new()
        // .add_filter("text", &["txt", "rs"])
        // .set_directory(&path)
        .pick_folder();

        // debug
        println!("The user choose: {:#?}", res);

        // avoid panic if none is chosen and we just close dialog
        if res.is_none() {
            return;
        }

        let mut state = state_path_weak.borrow_mut();
        state.workspace_changed(res.unwrap().as_path().to_str().unwrap().to_string());
    });

    ui.run();
}
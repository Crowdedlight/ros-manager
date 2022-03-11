
use slint::{Model, ModelNotify, VecModel, ComponentHandle};
use std::borrow::{Borrow, BorrowMut};
use std::rc::Rc;
use std::cell::RefCell;


mod ui;
mod workspacehandler;

// pub struct WorkspaceData {
//     data: Rc<slint::VecModel<WorkspaceItem>>,
//     notify: ModelNotify,
// }

// impl WorkspaceData {
    // fn push_job(&self, title: slint::SharedString) {
    //     self.data.push(PrinterQueueItem {
    //         status: "WAITING...".into(),
    //         progress: 0,
    //         title,
    //         owner: env!("CARGO_PKG_AUTHORS").into(),
    //         pages: 1,
    //         size: "100kB".into(),
    //         submission_date: current_time(),
    //     })
    // }
// }

fn main() {
    let ui = ui::MainWindow::new();
    let ui_weak: slint::Weak<ui::MainWindow> = ui.as_weak();

    // get collection from slint as starting base
    let ws_list_initial: Vec<ui::WorkspaceItem> = ui.get_workspace_list().iter().collect();

    let ws_state = RefCell::new(workspacehandler::WorkspaceState {
        workspaces: Vec::<ui::WorkspaceItem>::new(),
        main_window: ui_weak,
        ws_root_path: "".to_string(),
    });

    //let ui_handle = ui.as_weak();   
    ui.on_ros_workspace_build(move |path| {
        println!("{path}");
    });

    ui.on_ros_workspace_sourced(move |path| {
        println!("{path}");
    });
    
    ui.on_set_workspace_path(move || {
        let res = rfd::FileDialog::new()
        // .add_filter("text", &["txt", "rs"])
        // .set_directory(&path)
        .pick_folder();

        // debug
        println!("The user choose: {:#?}", res);

        let mut state = ws_state.borrow_mut();
        state.workspace_changed(res.unwrap().as_path().to_str().unwrap().to_string());
    });

    ui.run();
}

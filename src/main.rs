
use slint::{Model, ModelNotify};
use std::rc::Rc;

slint::include_modules!();

mod WorkspaceHandler;

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
    let ui = MainWindow::new();

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

        println!("The user choose: {:#?}", res);
        WorkspaceHandler::workspace_changed(res.unwrap().as_path().to_str().unwrap().to_string());
    });

    ui.run();
}

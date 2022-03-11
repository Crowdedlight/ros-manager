

use slint::{Model, ModelNotify, VecModel, Weak};
use std::rc::Rc;

slint::include_modules!();

pub struct WorkspaceState {
    workspaces: Vec<WorkspaceItem>,
    main_window: slint::Weak<MainWindow>,
    ws_root_path: String,
}

impl WorkspaceState { 
    // consider making this return the updated model struct of the contents? And then have the main-rs ui thread update the ui so we don't pass ui ref around
    fn workspace_changed(&self, ws_path: String) {
        // if our workspace is changed, we reload everything
        self.ws_root_path = ws_path;
        println!("new workspace: {}", self.ws_root_path);

    }

    fn load_workspaces(&self) {
        //let mut workspaces: VecModel<WorkspaceItem> = self.main_window.unwrap().get_workspace_list().iter().collect();
    
        // load workspaces from root path

        // make new WorkspaceItem per item

        // save and push list
        let test = Rc::new(slint::VecModel::<WorkspaceItem>::from(self.workspaces));
        self.main_window.unwrap().set_workspace_list(test.into());
    }

    fn get_nodes_in_ws(path: String) -> u16 {
        // count number of folders in the ws_path
    
        return 5;
    }

    fn ros_version_select(path: String) -> String {
        // given the path to the workspace, figure out of its a ros1 or ros2 workspace, and if we are melodic or foxy? Any files that would tell us? 
        //  Like maybe read the first node in src, and see if we can figure it out from package.xml or something?
    
        return "".to_string();
    }
}
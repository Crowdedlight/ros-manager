

use slint::{SharedString};
use std::{rc::Rc, fs, path::PathBuf};
use crate::ui;

pub struct WorkspaceState {
    pub workspaces: Vec<ui::WorkspaceItem>,
    pub main_window: slint::Weak<ui::MainWindow>,
    pub ws_root_path: String,
}

impl WorkspaceState { 
    // consider making this return the updated model struct of the contents? And then have the main-rs ui thread update the ui so we don't pass ui ref around
    pub fn workspace_changed(&mut self, ws_path: String) {
        // if our workspace is changed, we reload everything
        self.ws_root_path = ws_path;

        // load workspaces
        self.load_workspaces();

        // update gui
        self.update_gui();
    }

    fn update_gui(&self) {
        // save and push list
        let test = Rc::new(slint::VecModel::<ui::WorkspaceItem>::from(self.workspaces.clone()));
        self.main_window.unwrap().set_workspace_list(test.into());
    }

    fn load_workspaces(&mut self) {
        //let mut workspaces: VecModel<WorkspaceItem> = self.main_window.unwrap().get_workspace_list().iter().collect();
        // new empty vector
        let mut list = Vec::<ui::WorkspaceItem>::new();
    
        // load workspaces from root path
        for file in fs::read_dir(self.ws_root_path.as_str()).unwrap() {

            let entry = file.unwrap();
            // if not dir, we skip, we are only looking at dirs here...
            if entry.metadata().unwrap().is_file() {
                continue;
            }

            // save name, path
            let ws_path = entry.path(); 
            let ws_name = ws_path.file_name();
            
            // try to detect ros
            let ros_version = self.get_ros_version(&ws_path);
            let ros_nodes = self.get_nodes_in_ws(&ws_path);

            // make new item and push to vector
            let mut new_ws = ui::WorkspaceItem::default();
            new_ws.path = SharedString::from(ws_path.to_str().unwrap());
            new_ws.name = SharedString::from(ws_name.unwrap().to_str().unwrap());
            new_ws.ros_version = SharedString::from(ros_version);
            new_ws.nodes = ros_nodes;

            list.push(new_ws);
        }

        // save list in self
        self.workspaces = list;
    }

    fn get_nodes_in_ws(&self, path: &PathBuf) -> i32 {
        // count number of folders in the ws_path
        let mut count = 0;
        
        // check if path exists to avoid error
        let src_path = path.join("src");
        if !src_path.exists() {
            return count;
        }

        // read files in src
        for file in fs::read_dir(src_path).unwrap() {
            let entry = file.unwrap();
            if entry.metadata().unwrap().is_dir() {
                count += 1;
            }
        }

        return count;
    }

    fn get_ros_version(&self, path: &PathBuf) -> String {
        // given the path to the workspace, figure out of its a ros1 or ros2 workspace, and if we are melodic or foxy? Any files that would tell us? 
        //  Like maybe read the first node in src, and see if we can figure it out from package.xml or something?
    
        return "".to_string();
    }

    pub fn get_ws_item_from_path(&self, path : SharedString) -> Option<&ui::WorkspaceItem> {

        let mut iter = self.workspaces.iter();
        let ws = iter.find(|&x| x.path == path);
        return ws;
    }
}


use slint::{SharedString};
use std::{rc::Rc, fs, path::{PathBuf}, fs::File, io};
use crate::ui;
use glob::glob;
use std::io::{prelude::*, BufReader};

use crate::config::Cfg;

pub struct WorkspaceState {
    pub workspaces: Vec<ui::WorkspaceItem>,
    pub main_window: slint::Weak<ui::MainWindow>,
    pub ws_root_path: String,
    pub settings: Cfg,
}

impl WorkspaceState { 
    // consider making this return the updated model struct of the contents? And then have the main-rs ui thread update the ui so we don't pass ui ref around
    pub fn workspace_changed(&mut self, ws_path: String) {
        // if our workspace is changed, we reload everything
        self.ws_root_path = ws_path.clone();

        // write to config file
        self.settings.ws_root_path = ws_path.clone();
        self.settings.save();

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

    fn is_src_exist(&self, mut path: PathBuf) -> io::Result<bool>{
        path.push("src");
        let metadata = fs::metadata(path)?;
        Ok(metadata.is_dir())
    }

    fn is_src_empty(&self, mut path: PathBuf) -> io::Result<bool>{
        path.push("src");
        let is_empty = path.read_dir()?.next().is_none();
        Ok(is_empty)
    }


    pub fn load_workspaces(&mut self) {
        //let mut workspaces: VecModel<WorkspaceItem> = self.main_window.unwrap().get_workspace_list().iter().collect();
        // new empty vector
        let mut list = Vec::<ui::WorkspaceItem>::new();
    
        // load workspaces from root path if string is not empty
        // TODO better error handling... should look into try catch, as maybe someone sets a path that doesn't exist
        if self.ws_root_path == "" {return;}

        for file in fs::read_dir(self.ws_root_path.as_str()).unwrap() {

            let entry = file.unwrap();
            // if not dir, we skip, we are only looking at dirs here...
            if entry.metadata().unwrap().is_file() {
                continue;
            }

            // if dir is empty, src does not exist, or src is empty, we skip
            if !self.is_src_exist(entry.path()).unwrap() || self.is_src_empty(entry.path()).unwrap() {
                println!("ERR: dir is empty, src doesn't exist or src is empty...");
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

        // update gui
        self.update_gui();

        // DEBUG
        for entry in &self.workspaces {
            println!("Path: {:?}, Version: {:?}, Nodes: {:?}", entry.path, entry.ros_version, entry.nodes);
        }
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
            // we assume any directory or symlinks in "src" is for a node
            if entry.metadata().unwrap().is_dir() || entry.metadata().unwrap().is_symlink() {
                count += 1;
            }
        }

        return count;
    }

    fn get_ros_version(&self, path: &PathBuf) -> String {
        // given the path to the workspace, figure out of its a ros1 or ros2 workspace
        // let mut path_package = path.join("/**/package.xml");
        let mut path_package = String::from(path.to_string_lossy());
        path_package = path_package + "/src/**/package.xml";

        // find the first package.xml in src/*/
        let mut packageFile = PathBuf::new();

        for entry in glob(&path_package).expect("Failed to find ros version") {
            match entry {
                Ok(path) => {packageFile = path; break;},
                Err(e) => println!("{:?}", e),
            }
        }
        
        // println!("{:?}, package:{:?}", packageFile, path_package);

        // Exit if file doesn't exist, err.
        if packageFile.as_os_str().is_empty() {
            return "ERR".to_string();
        }

        // open file on the first path, as any package.xml should tell us ros version
        let file = File::open(packageFile).unwrap();
        let reader = BufReader::new(file);

        let mut ver = "";
        for line in reader.lines(){
            let line = line.unwrap();

            match line {
                s if s.contains("catkin") => {ver = "ROS1"; break;},
                s if s.contains("ament_cmake") => {ver = "ROS2"; break;},
                _ => ver = "ERR",
            }
        }
        return ver.to_string();
    }

    pub fn get_ws_item_from_path(&self, path : SharedString) -> Option<&ui::WorkspaceItem> {

        let mut iter = self.workspaces.iter();
        let ws = iter.find(|&x| x.path == path);
        return ws;
    }
}
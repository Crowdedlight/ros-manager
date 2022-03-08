use std::string;



// consider making this return the updated model struct of the contents? And then have the main-rs ui thread update the ui so we don't pass ui ref around
pub fn workspace_changed(ws_path: String) {
    // if our workspace is changed, we reload everything
    println!("new workspace: {}", ws_path);
}

// fn update_model(ws_path: &str) -> 

fn get_nodes_in_ws(ws_path: &str) -> u16 {
    // count number of folders in the ws_path

    return 5;
}

fn ros_version_select(path: String) -> String {
    // given the path to the workspace, figure out of its a ros1 or ros2 workspace, and if we are melodic or foxy? Any files that would tell us? 
    //  Like maybe read the first node in src, and see if we can figure it out from package.xml or something?

    return "".to_string();
}
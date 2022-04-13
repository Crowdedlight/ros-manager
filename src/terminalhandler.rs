
use crate::ui;
use crate::workspacehandler;
use std::process::Command;
use std::env;

pub fn launch_sourced(item : &ui::WorkspaceItem) {
    // find default shell and ros-version source path
    let shell = get_default_shell();
    let rosver = get_ros_version_source_path(item.ros_version.to_string(), &shell);

    // depending on ros-version, we either find path for devel/setup.zsh or install/setup.zsh
    // TODO find the path needed, then we add it to the command line, before cd
    

    let command = format!("{} -c 'source {}; cd {}; exec $SHELL -i'", shell, rosver, item.path.as_str());
    println!("{:?}", command);

    Command::new("x-terminal-emulator")
    .arg("-e")
    .arg(command)
    // .arg("&")
    // .arg("disown")
    .spawn()
    .expect("failed to execute process");
}

pub fn launch_build(item : &ui::WorkspaceItem) {
    // find default shell and ros-version source path
    let shell = get_default_shell();
    let rosver = get_ros_version_source_path(item.ros_version.to_string(), &shell);

    // run command to build workspace before $shell -i
    // we don't source workspace install/setup here, as we are building
    let build_cmd;
    match item.ros_version.as_str() {
        "ROS1" => build_cmd = "catkin build",
        "ROS2" => build_cmd = "colcon build",
        _ => build_cmd = "ERR",
    }

    let command = format!("{} -c 'source {}; cd {}; {}; exec $SHELL -i'", shell, rosver, item.path.as_str(), build_cmd);
    println!("{:?}", command);

    Command::new("x-terminal-emulator")
    .arg("-e")
    .arg(command)
    // .arg("&")
    // .arg("disown")
    .spawn()
    .expect("failed to execute process");
}

pub fn get_ros_version_source_path(version: String, shell: &String) -> String {
    let rosver;
    match version.as_str() {
        "ROS1" => rosver = format!("/opt/ros/noetic/setup.{}", shell),
        "ROS2" => rosver = format!("/opt/ros/foxy/setup.{}", shell),
        _ => rosver = "ERR".to_string(),
    }
    return rosver;
}

pub fn get_default_shell() -> String {
        // find default shell
        let shell;
        match env::var("SHELL") {
            Ok(val) => shell = val,
            Err(_e) => shell = "Shell Not Found".to_string(),
        }
    
        // remove /bin/, to just have shell name. Should be bash or zsh
        let (_, shell) = shell.split_at(5);
        return shell.to_string();
}
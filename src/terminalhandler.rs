
use crate::ui;
use crate::workspacehandler;
use std::process::Command;

pub fn launch_sourced(item : &ui::WorkspaceItem) {
    
    println!("{}", item.name);

    Command::new("tilix")
    .arg("-x")
    .arg("zsh -c 'source /opt/ros/foxy/setup.zsh; exec $SHELL -i'")
    // .arg("&")
    // .arg("disown")
    .spawn()
    .expect("failed to execute process");

    // the following works, however its important to check if .zshrc sets the PATH explicit. I had that hence why it didn't work initially
    // tilix -x 'zsh -c "source /opt/ros/foxy/setup.zsh; exec $SHELL -i"'

    // Command::new("sh").args(&["-c", "cd", item.path.as_str(), "&&", "source", "/opt/ros/foxy/setup.zsh", "& disown"]);

    // Command::new("source /opt/ros/foxy/setup.zsh & disown")
    //     .current_dir(item.path.as_str())
    //     .spawn()
    //     .expect("failed to spawn terminal");
}
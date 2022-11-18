# Ros Manager

A gui tool to see and choose ros workspace. Shows in a list the current ros workspaces and gives you buttons to quickly start terminals that is cd'ed, sourced and ready to build and run applications in the workspace. 

Ideas:  
* Include button for simulation and launching gazebo-ros   
* Auto populate workspaces based on top folder. All workspaces are in each of their folders inside the root folder  
* Settings to set root-folder  
* Include node/package names inside the workspaces in overview  
* auto detech ros1 or ros2? (potentially by install folder or if we got another hidden specific file)  
* include showing launch files? or maybe choose launch files too?

## Build

1. Install dependencies
    ```
    sudo apt install libxcb-shape0-dev libxcb-xfixes0-dev
    ```
2. Build with cargo
    ```
    cargo build
    ```
3. Run the application binary
     ```
     cargo run
     ```

## Using the application
**First set the workspace root folder with ``Set WS`` button on the left hand side.**

The application expects a folder structure of: 
```
ros_workspaces
 -> ros1_ws
 -> ros2_ws
 -> ros2_project1_ws
 -> another_ros_ws
   -> src
      -> package1
      -> package2
      ...
 .
 .
 .
```
So essentially each ros workspace you have, should be inside the same root-folder. So you point the application
to the root folder, and it will find all the workspaces and their packages inside each.

## Known issues
* Currently crashes if you have an workspace in the path with an empty ``src/`` folder.
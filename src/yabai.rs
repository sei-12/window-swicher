use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct YabaiWindowFrame {
    #[serde(rename = "x")]
    pub x_px: f32,
    #[serde(rename = "y")]
    pub y_px: f32,
    pub w: f32,
    pub h: f32
}

#[derive(Serialize, Deserialize)]
struct YabaiWindow {
    pub id: u32,
    pub frame: YabaiWindowFrame,
    
    #[serde(rename = "has-focus")]
    has_focus: bool,
    
    #[serde(rename="is-visible")]
    is_visible: bool
}

#[derive(Serialize, Deserialize)]
struct YabaiWorkspace {
    pub id: u32,
    
    #[serde(rename = "has-focus")]
    has_focus: bool,
}


pub fn get_window_ids() -> Vec<u32> {
    let output = std::process::Command::new("yabai")
        .arg("-m")
        .arg("query")
        .arg("--windows")
        .output()
        .expect("failed to execute yabai");
    
    let output = String::from_utf8_lossy(&output.stdout);
    
    let windows: Vec<YabaiWindow> = serde_json::from_str(&output).expect("failed to parse yabai output");
    

    let ids: Vec<u32> = windows.into_iter()
        .filter(|window| window.is_visible)
        .map(|window| window.id).collect();
    
    ids
}

pub fn focus_window(id: u32){
    std::process::Command::new("yabai")
        .arg("-m")
        .arg("window")
        .arg("--focus")
        .arg(format!("{}",id))
        .output()
        .expect("failed to execute yabai");
}


pub fn get_current_workspace_id() -> u32 {
    let output = std::process::Command::new("yabai")
        .arg("-m")
        .arg("query")
        .arg("--spaces")
        .output()
        .expect("failed to execute yabai");
    
    let output = String::from_utf8_lossy(&output.stdout);
    
    let windows: Vec<YabaiWorkspace> = serde_json::from_str(&output).expect("failed to parse yabai output");

    let focused_space = windows.into_iter()
        .find(|space|space.has_focus);

    match focused_space {
        Some(s) => s.id,
        None => panic!("fauld to excute yabai")
    }
}
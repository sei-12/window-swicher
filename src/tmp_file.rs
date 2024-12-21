use std::{fs::File, io::{Read, Write}};


pub fn read_tmp_file_or_empty_str(path: &str) -> String {

    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return String::new(),
    };

    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(_) => return String::new(),    
    };
    
    contents
}


pub fn write_tmp_file(path: &str, contents: &str) {
    let mut file = File::create(path).expect("create failed");
    file.write_all(contents.as_bytes()).expect("write failed");
}



#[derive(serde::Serialize,serde::Deserialize)]
pub struct TmpFileData {
    pub current_focus_index: usize,
    pub timestamp_ms: i64,
    pub window_ids: Vec<u32>,
}

impl TryFrom<String> for TmpFileData {
    type Error = serde_json::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        serde_json::from_str(&value)
    }
}

impl Into<String> for TmpFileData {
    fn into(self) -> String {
        serde_json::to_string(&self).expect("json serialize failed")
    }
}
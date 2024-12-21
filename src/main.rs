use std::u32;

use chrono::Utc;
use clap::Parser;
use yabai::{get_current_workspace_id, get_window_ids};

mod yabai;
mod tmp_file;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // pathは適当
    #[arg(short, long, default_value="/tmp/__window_switcher_tmp.C9F1634F.txt", help = "tmp file path")]
    tmp_file_path: String,
    
    #[arg(long, default_value="1000", help = "timeout ms")]
    timeout_ms: u32,
    
    #[arg(long,default_value="next",help = "next or prev")]
    goto: String
}

fn check_timeout(timeout_ms: u32, prev: i64, now: i64) -> bool {
    let diff = now - prev;
    diff > timeout_ms.into()
}

fn calc_next_index(should_reset: bool, window_ids_length: usize, prev_focus_index: Option<usize>, go_next: bool) -> Option<usize> {
    let mut i = if should_reset {
        0
    }else{
        match prev_focus_index { 
            Some(i) => i,
            None => 0
        }
    };
    
    if window_ids_length == 0 {
        return None;
    };    
    
    if go_next {
        i += 1;
        if i >= window_ids_length {
            i = 0
        };
    }else{
        i += 1; // usize

        i -= 1;
        if i < 1 {
            i = window_ids_length - 1
        }else{
            i -= 1; // usize
        }
    };
    
    Some(i)
}

fn main() {
    let call_timestamp_ms = Utc::now().timestamp_millis();

    let args = Args::parse();
    let go_next = if args.goto == "next" { true }else{ false };

    let tmp_file_txt = tmp_file::read_tmp_file_or_empty_str(&args.tmp_file_path);
    let tmp_file_data = tmp_file::TmpFileData::try_from(tmp_file_txt);

    let prev_workspace_id = match &tmp_file_data {
        Err(_) => None,
        Ok(tfd) => Some(tfd.workspace_id)
    };
    let prev_focus_index = match &tmp_file_data {
        Err(_) => None,
        Ok(tfd) => Some(tfd.current_focus_index)
    };
    let prev_timestamp = match &tmp_file_data {
        Err(_) => None,
        Ok(tfd) => Some(tfd.timestamp_ms)
    };
    let prev_window_ids = match tmp_file_data {
        Err(_) => None,
        Ok(tfd) => Some(tfd.window_ids)
    };
    
    let timeout = match prev_timestamp {
        Some(t) => check_timeout(args.timeout_ms, t, call_timestamp_ms),
        None => true
    };

    let current_workspace_id = get_current_workspace_id();
    let should_reset = timeout || prev_workspace_id.is_none() || ( current_workspace_id != prev_workspace_id.unwrap_or(u32::MAX));
    
    let window_ids = if should_reset {
        get_window_ids()
    }else{
        match prev_window_ids {
            Some(ids) => ids,
            None => get_window_ids()
        }
    };
    
    let next_focus_index = calc_next_index(
        timeout, 
        window_ids.len(), 
        prev_focus_index,
        go_next
    );

    let next_focus_index = match next_focus_index {
        None => std::process::exit(0),
        Some(i) => i
    };
    
    yabai::focus_window(window_ids[next_focus_index]);    
    
    let new_tmp_file_data = tmp_file::TmpFileData {
        current_focus_index: next_focus_index,
        timestamp_ms: call_timestamp_ms,
        window_ids,
        workspace_id: current_workspace_id
    };

    let txt : String= new_tmp_file_data.into();

    tmp_file::write_tmp_file(&args.tmp_file_path, &txt);

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_next_index(){
        let i = calc_next_index(true, 10, Some(1), true);
        assert_eq!(Some(1), i);

        let i = calc_next_index(false, 10, Some(1), true);
        assert_eq!(Some(2), i);

        let i = calc_next_index(false, 11, Some(1), true);
        assert_eq!(Some(2), i);

        let i = calc_next_index(false, 11, Some(11), true);
        assert_eq!(Some(0), i);

        let i = calc_next_index(true, 10, Some(1), false);
        assert_eq!(Some(9), i);

        let i = calc_next_index(false, 10, Some(1), false);
        assert_eq!(Some(0), i);

        let i = calc_next_index(false, 11, Some(1), false);
        assert_eq!(Some(0), i);

        let i = calc_next_index(false, 11, Some(11), false);
        assert_eq!(Some(10), i);
    }
}
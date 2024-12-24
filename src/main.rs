#![windows_subsystem = "windows"]
mod lib_crypt;
mod window;

use std::{env, fs};
use winsafe::{prelude::*, co, AnyResult, HWND};
use window::MyWindow;
use crate::window::display_error;


fn main() {
    if let Err(e) = run_app() {
        display_error(format!("Application error: {}", e));
    }
}

fn run_app() -> AnyResult<i32> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        display_error("Invalid argument count. Must be 2!".to_string());
        return Ok(1);
    }
    let op = if &args[1] == "enc" {true} else if &args[1] == "dec" {false} else {
        display_error(format!("Invalid operation: {}", &args[1]));
        return Ok(1);
    };
    let file_path = &args[2];
    if !fs::exists(file_path).unwrap() {
        display_error(format!("File not found: {}", file_path));
        return Ok(1);
    }
    MyWindow::new(file_path.to_string(), op)
        .run()
        .map_err(|err| err.into())
}
#![windows_subsystem = "windows"]

mod lib_crypt;
extern crate native_windows_gui as nwg;

use std::{env, fs};
use std::path::Path;
use std::rc::Rc;
use native_windows_gui::WindowFlags;
use nwg::CheckBoxState;
use crate::lib_crypt::{encrypt_buffer, decrypt_buffer};

pub fn display_error(error_string: String) {
    nwg::error_message("Error", error_string.as_str());
}

fn encrypt_file_xx(file_name: String, password: String, replace: bool) {
    let mut target_file = file_name.clone() + if replace {""} else {".waxe"};
    let file_buffer_r = fs::read(&file_name);
    if file_buffer_r.is_err() {
        display_error("Operation failed because the file can not be read!".to_string());
        return;
    }
    let file_buffer = file_buffer_r.unwrap();
    let encrypted_r = encrypt_buffer(password, file_buffer);
    if encrypted_r.is_err() {
        display_error("Encryption failed: ".to_string() + encrypted_r.unwrap_err().to_string().as_str());
        return;
    }
    let encrypted_buffer = encrypted_r.unwrap();
    if replace {
        let rm_file_r = fs::remove_file(&target_file);
        if rm_file_r.is_err() {
            display_error(format!("Failed to remove file {}, falling back!", target_file));
            target_file = target_file + ".waxe"
        }
    }
    if fs::exists(&target_file).expect("Why tho") {
        display_error("Target file (".to_owned() + &*target_file + " already exists!");
    }
    let write_r = fs::write(target_file, encrypted_buffer);
    if write_r.is_err() {
        display_error("Could not write encrypted file".to_string());
    }
}

fn decrypt_file_xx(file_name: String, password: String, replace: bool) {
    let mut target_file = if replace { file_name.clone() } else { Path::new(&file_name).file_stem().unwrap().to_os_string().into_string().unwrap() + ".waxd" };
    let file_buffer_r = fs::read(&file_name);
    if file_buffer_r.is_err() {
        display_error("Operation failed because the file can not be read!".to_string());
        return;
    }
    let file_buffer = file_buffer_r.unwrap();
    let decrypted_r = decrypt_buffer(password, file_buffer);
    if decrypted_r.is_err() {
        display_error("Decryption failed: ".to_string() + decrypted_r.unwrap_err().to_string().as_str());
        return;
    }
    let decrypted_buffer = decrypted_r.unwrap();
    if replace {
        let rm_file_r = fs::remove_file(&target_file);
        if rm_file_r.is_err() {
            display_error(format!("Failed to remove file {}, falling back! => {}", target_file, rm_file_r.unwrap_err().to_string()));
            target_file = target_file + ".waxd"
        }
    }
    if fs::exists(&target_file).expect("Why tho") {
        display_error("Target file (".to_owned() + &*target_file + ") already exists!");
    }
    let write_r = fs::write(target_file, decrypted_buffer);
    if write_r.is_err() {
        display_error("Could not write decrypted file".to_string());
    }
}

fn wnd_main(file_path: String, op: bool) {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let mut window = Default::default();
    let mut password_entry = Default::default();
    let mut magic_button = Default::default();
    let mut replace_checkbox = Default::default();

    nwg::Window::builder()
        .flags(WindowFlags::SYS_MENU | WindowFlags::VISIBLE)
        .size((600, 300))
        .position((300, 300))
        .title("Wax Encryption and Decryption")
        .build(&mut window)
        .unwrap();

    nwg::TextInput::builder()
        .position((50, 50))
        .size((500, 40))
        .password(Some('X'))
        .text("")
        .placeholder_text(Some("Very secure password"))
        .parent(&window)
        .build(&mut password_entry)
        .unwrap();

    nwg::CheckBox::builder()
        .text("Replace original file")
        .position((200, 100))
        .size((200, 40))
        .parent(&window)
        .build(&mut replace_checkbox)
        .unwrap();

    nwg::Button::builder()
        .size((300, 40))
        .position((150, 175))
        .text(if op {"Encrypt"} else {"Decrypt"})
        .parent(&window)
        .build(&mut magic_button)
        .unwrap();

    let window = Rc::new(window);
    let events_window = window.clone();

    let handler = nwg::full_bind_event_handler(&(&window).handle, move |evt, _evt_data, handle| {
        use nwg::Event as E;

        match evt {
            E::OnWindowClose =>
                if &handle == &events_window as &nwg::Window {
                    nwg::stop_thread_dispatch();
                },
            E::OnButtonClick =>
                if &handle == &magic_button {
                    if op {
                        encrypt_file_xx(file_path.clone(), password_entry.text(), replace_checkbox.check_state() == CheckBoxState::Checked);
                    } else {
                        decrypt_file_xx(file_path.clone(), password_entry.text(), replace_checkbox.check_state() == CheckBoxState::Checked);
                    }
                    
                    let _ = &events_window.close();
                },
            _ => {}
        }
    });

    nwg::dispatch_thread_events();
    nwg::unbind_event_handler(&handler);
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if (&args).len() != 3 {
        display_error("Invalid argument count. Must be 2!".to_string());
        return;
    }
    let op = if (&args)[1] == "enc" {true} else if (&args)[1] == "dec" {false} else {
        display_error(format!("Invalid operation: {}", (&args)[1]));
        return;
    };
    let file_path = (&args[2]).to_owned();
    if !fs::exists(file_path.clone()).unwrap() {
        display_error(format!("File not found: {}", file_path));
        return;
    }
    
    wnd_main(file_path, op);
}

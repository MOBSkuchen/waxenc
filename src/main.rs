#![windows_subsystem = "windows"]

mod lib_crypt;
extern crate native_windows_gui as nwg;

use std::{env, fs, io};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use native_windows_gui::WindowFlags;
use nwg::{CheckBoxState, FileDialogAction};
use nwg::FileDialogAction::Save;
use crate::lib_crypt::{encrypt_file_xx, decrypt_file_xx, hash_file};

pub fn display_error(error_string: String) {
    nwg::error_message("Error", error_string.as_str());
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
                    let password = password_entry.text();
                    if op {
                        encrypt_file_xx(file_path.clone(), password, replace_checkbox.check_state() == CheckBoxState::Checked);
                    } else {
                        decrypt_file_xx(file_path.clone(), password, replace_checkbox.check_state() == CheckBoxState::Checked);
                    }
                    
                    let _ = &events_window.close();
                },
            _ => {}
        }
    });

    nwg::dispatch_thread_events();
    nwg::unbind_event_handler(&handler);
}

pub fn absolute_path(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    let path = path.as_ref();

    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()?.join(path)
    };

    Ok(absolute_path)
}

fn hash_crafter(file_path: String) {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    
    let x = absolute_path(&file_path).unwrap();
    let default_path = x.parent().unwrap().to_str().unwrap();

    let mut file_save_dialog = Default::default();
    nwg::FileDialog::builder()
        .multiselect(false)
        .filters("Hash(*.hash;*.sha)|Any(*.*)")
        .default_folder(default_path)
        .action(Save)
        .title("Save hash")
        .build(&mut file_save_dialog).expect("FileDialog failed to build");

    let result = file_save_dialog.run::<nwg::ControlHandle>(None);
    if result {
        hash_file(file_path, file_save_dialog.get_selected_item().unwrap());
    } else {
        display_error("Failed - cancelled".to_string())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if (&args).len() != 3 {
        display_error("Invalid argument count. Must be 2!".to_string());
        return;
    }
    let file_path = (&args[2]).to_owned();
    if !fs::exists(file_path.clone()).unwrap() {
        display_error(format!("File not found: {}", file_path));
        return;
    }

    if &args[1] == "enc" {wnd_main(file_path, true)}
    else if &args[1] == "dec" {wnd_main(file_path, false)}
    else if &args[1] == "hash" {hash_crafter(file_path)}
    else { display_error(format!("Invalid operation: {}", (&args)[1])); }
}

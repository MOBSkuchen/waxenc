use std::fs;
use std::path::Path;
use winsafe::{prelude::*, gui, AnyResult, co, HWND, co::MB};
use crate::lib_crypt::{decrypt_buffer, encrypt_buffer};

#[derive(Clone)]
pub struct MyWindow {
    wnd:       gui::WindowMain,
    submit_button: gui::Button,
    password_entry: gui::Edit,
    replace_checkbox: gui::CheckBox,
    op_enc: bool,
    file: String,
}

pub fn display_error(error_text: String) {
    let hwnd = HWND::GetDesktopWindow();
    hwnd.MessageBox(error_text.as_str(), "Error", MB::OKCANCEL | MB::ICONERROR).expect("Failed!");
}

fn encrypt_file_xx(file_name: String, password: String, replace: bool) {
    let mut target_file = file_name.clone() + if replace {""} else {".waxe"};
    let file_buffer_r = fs::read(&file_name);
    if file_buffer_r.is_err() {
        display_error("Operation failed because the file can not be read!".to_string());
        return;
    }
    let file_buffer = file_buffer_r.unwrap();
    let encrypted_r = encrypt_buffer(file_buffer, password);
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
    let decrypted_r = decrypt_buffer(file_buffer, password);
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

impl MyWindow {
    pub fn new(file: String, op_enc: bool) -> Self {
        let btn_txt = if op_enc { "Encrypt file".to_owned() } else { "Decrypt file".to_owned() };
        let wnd = gui::WindowMain::new(
                                        gui::WindowMainOpts {
                                            title: "Wax Encryption and Decryption".to_owned(),
                                            class_icon: gui::Icon::None,
                                            size: (500, 250),
                                            ..Default::default()
                                        },
        );

        let _ = gui::Label::new(
            &wnd,
            gui::LabelOpts {
                text: "Enter password below".to_string(),
                position: (175, 40),
                size: (150, 40),
                ..Default::default()
            }
        );

        let password_entry = gui::Edit::new(
            &wnd,
            gui::EditOpts {
                text: "".to_string(),
                position: (100, 90),
                width: 300,
                height: 40,
                edit_style: co::ES::PASSWORD,
                ..Default::default()
            }
        );

        let submit_button = gui::Button::new(
            &wnd,
            gui::ButtonOpts {
                text: btn_txt,
                position: (175, 200),
                width: 150,
                height: 35,
                ..Default::default()
            },
        );

        let replace_checkbox = gui::CheckBox::new(
            &wnd,
            gui::CheckBoxOpts {
                text: "Replace original file".to_string(),
                position: (175, 150),
                ..Default::default()
            }
        );

        let new_self = Self { wnd, submit_button, password_entry, replace_checkbox, op_enc, file };
        new_self.events();
        new_self
    }

    pub fn run(&self) -> AnyResult<i32> {
        self.wnd.run_main(None)
    }

    fn events(&self) {
        let self2 = self.clone();
        self.submit_button.on().bn_clicked(move || { // button click event
            if self2.op_enc {
                encrypt_file_xx(self2.file.clone(), self2.password_entry.text(), self2.replace_checkbox.is_checked());
                self2.wnd.close()
            } else {
                decrypt_file_xx(self2.file.clone(), self2.password_entry.text(), self2.replace_checkbox.is_checked());
                self2.wnd.close()
            }
            Ok(())
        });
    }
}
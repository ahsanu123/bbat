use crate::factory_method_dialog::{gui::Dialog, html_gui::HtmlDialog, windows_gui::WindowsDialog};

pub fn initialize() -> &'static dyn Dialog {
    if cfg!(windows) {
        println!("-- its windows system, create new dialog for windows");
        &WindowsDialog
    } else {
        println!("-- no os detected, create new Html gui");
        &HtmlDialog
    }
}

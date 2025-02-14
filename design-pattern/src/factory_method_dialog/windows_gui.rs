use crate::factory_method_dialog::gui::{Button, Dialog};

pub struct WindowsButton;

impl Button for WindowsButton {
    fn render(&self) {
        println!("Drawing Windows Button");
        self.on_click();
    }

    fn on_click(&self) {
        println!("windows button clicked");
    }
}

pub struct WindowsDialog;

impl Dialog for WindowsDialog {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WindowsButton)
    }
}

use crate::factory_method_dialog::gui::{Button, Dialog};

pub struct HtmlButton;

impl Button for HtmlButton {
    fn render(&self) {
        println!("<button>This Html Button</button>");
    }

    fn on_click(&self) {
        println!("Clicked!!");
    }
}

pub struct HtmlDialog;

impl Dialog for HtmlDialog {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(HtmlButton)
    }
}

use design_pattern::factory_method_dialog::*;

fn main() {
    let dialog = initialization::initialize();
    dialog.render();
    dialog.refresh();
}

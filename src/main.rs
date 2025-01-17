use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame};
use std::env;

use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    dbg!(args);

    let asparagus = Asparagus {};
    // println!("asparagus: {asparagus:?}");
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();

    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}

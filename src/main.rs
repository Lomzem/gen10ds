mod app;
mod dipswitch;
mod format_calculator;
mod ui;

use std::io;

fn main() -> io::Result<()> {
    // Setup terminal
    let terminal = ratatui::init();
    
    // Run application
    let mut app = app::App::new();
    let result = app.run(terminal);
    
    // Restore terminal
    ratatui::restore();
    
    result
}

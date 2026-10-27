//! ターミナルのエディタの起動.

#[cfg(not(tarpaulin_include))]
fn main() {
    use rseditor_tui::app::App;
    use rseditor_tui::ncurses_screen::NcursesScreen;

    let files: Vec<String> = std::env::args().skip(1).collect();
    let screen = NcursesScreen::start();
    let result = App::new(Box::new(screen), &files);
    let outcome = result.map(|mut app| app.run());
    NcursesScreen::stop();
    if let Err(error) = outcome {
        eprintln!("rseditor-tui: {error}");
        std::process::exit(1);
    }
}

#[cfg(tarpaulin_include)]
fn main() {}

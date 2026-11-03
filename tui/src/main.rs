//! ターミナルのエディタの起動.

#[cfg(not(tarpaulin_include))]
fn main() {
    use rseditor_tui::app::App;
    use rseditor_tui::ncurses_screen::NcursesScreen;

    let files: Vec<String> = std::env::args().skip(1).collect();
    if files
        .iter()
        .any(|arg| arg == "--batch" || arg == "-batch" || arg == "--script")
    {
        let io = rseditor_tui::batch::Io {
            out: Box::new(std::io::stdout()),
            err: Box::new(std::io::stderr()),
            input: Box::new(std::io::BufReader::new(std::io::stdin())),
        };
        std::process::exit(rseditor_tui::batch::run(&files, io));
    }
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

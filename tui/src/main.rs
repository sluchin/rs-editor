//! ターミナルのエディタの起動.

#[cfg(not(tarpaulin_include))]
fn main() {
    use rseditor_tui::app::App;
    use rseditor_tui::ncurses_screen::NcursesScreen;

    let args: Vec<String> = std::env::args().skip(1).collect();
    if args
        .iter()
        .any(|arg| arg == "--batch" || arg == "-batch" || arg == "--script")
    {
        let io = rseditor_tui::batch::Io {
            out: Box::new(std::io::stdout()),
            err: Box::new(std::io::stderr()),
            input: Box::new(std::io::BufReader::new(std::io::stdin())),
        };
        std::process::exit(rseditor_tui::batch::run(&args, io));
    }
    let options = match rseditor_core::startup::parse(&args) {
        Ok(options) => options,
        Err(error) => {
            eprintln!("rseditor-tui: {error}");
            std::process::exit(1);
        }
    };
    if let Some(dir) = &options.chdir {
        if let Err(error) = std::env::set_current_dir(dir) {
            eprintln!("rseditor-tui: {dir}: {error}");
            std::process::exit(1);
        }
    }
    let screen = NcursesScreen::start();
    let init_file = rseditor_core::startup::choose_init_file(
        &options,
        rseditor_core::startup::init_file_path(),
    );
    let result = App::new(Box::new(screen), &[]).and_then(|mut app| {
        app.apply_startup(&options);
        app.load_init(init_file.as_deref());
        app.open_files(&options.files)?;
        app.run();
        Ok(())
    });
    let outcome = result;
    NcursesScreen::stop();
    if let Err(error) = outcome {
        eprintln!("rseditor-tui: {error}");
        std::process::exit(1);
    }
}

#[cfg(tarpaulin_include)]
fn main() {}

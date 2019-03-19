use vgraph::Config;

fn main() {
    let matches = vgraph::app::build_cli().get_matches();
    let config = Config::from_matches(&matches).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });

    if let Err(e) = vgraph::run(matches) {
        match config.quiet {
            1 => std::process::exit(1),
            2 => {}
            _ => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }
}

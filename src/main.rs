use vgraph::Config;

fn main() {
    let matches = vgraph::app::build_cli().get_matches();
    if let Err(e) = Config::from_matches(matches).and_then(vgraph::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

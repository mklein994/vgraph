fn main() {
    if let Err(e) = vgraph::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

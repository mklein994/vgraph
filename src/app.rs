use clap::{App, Arg};

pub fn build_cli() -> App<'static, 'static> {
    App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("quiet")
                .long("quiet")
                .short("quiet")
                .multiple(true)
                .help(
                    "Controls error output. \
                     Pass twice to ensure a successful error code (0).",
                ),
        )
}

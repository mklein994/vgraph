mod error;
mod graph;

use self::error::Error;

use getopts::Options;
use std::env;

pub type Result<T> = std::result::Result<T, Error>;

const USAGE: &str = concat!(
    "Usage: ",
    env!("CARGO_PKG_NAME"),
    " [options] [--] [NUMBER...]"
);

#[must_use]
pub fn get_opts() -> Options {
    let mut opts = Options::new();
    opts.parsing_style(getopts::ParsingStyle::StopAtFirstFree)
        .optflag("h", "help", "print this help menu")
        .optflag("n", "no-newline", "don't print a trailing newline")
        .optflag("w", "no-wait", "don't wait for stdin")
        .optflagopt(
            "f",
            "fixed",
            "The range (inclusive) that the values are expected to fit in. Defaults to '0.0,1.0'",
            "<MINIMUM,MAXIMUM>",
        )
        .optflag("V", "version", "show version information and exit");
    opts
}

pub fn run() -> Result<()> {
    let opts = get_opts();

    let matches = opts.parse(env::args().skip(1))?;

    if matches.opt_present("help") {
        println!("{}", opts.usage(USAGE));
        println!(
            r#"Examples:
    $ seq 10 | {name}
     ▁▂▃▄▄▅▆▇█

    $ {name} 1 2 3
     ▄█

    $ seq 0.0 0.1 1.0 | {name}
     ▁▂▂▃▄▅▆▆▇█
"#,
            name = env!("CARGO_PKG_NAME")
        );
        return Ok(());
    }

    if matches.opt_present("version") {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    if matches.opt_present("fixed") {
        match matches.opt_default("fixed", "0.0,1.0") {
            Some(minmax) => {
                let (min, max) = {
                    let (min, max) = minmax.split_once(',').ok_or(Error::InvalidMinMax)?;
                    (min.parse::<f64>()?, max.parse::<f64>()?)
                };
                graph::fixed_at(min, max)?;
            }
            None => graph::fixed()?,
        };
    } else {
        graph::scaled(&matches)?;
    }

    if !matches.opt_present("no-newline") {
        println!();
    }

    Ok(())
}

fn scale(min: f64, max: f64, n: f64) -> f64 {
    (n - min) / (max - min)
}

fn graph(n: f64) -> Result<char> {
    if !(0.0..=1.0).contains(&n) {
        return Err(Error::OutOfBounds);
    }

    let amount = (n * 8_f64).round() as u32;

    if amount == 0 {
        Ok(' ')
    } else {
        std::char::from_u32(0x2580_u32 + amount).ok_or(Error::CharParse)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ` ▁▂▃▄▅▆▇█`
    const BARS: [char; 9] = [
        ' ', '\u{2581}', '\u{2582}', '\u{2583}', '\u{2584}', '\u{2585}', '\u{2586}', '\u{2587}',
        '\u{2588}',
    ];

    #[test]
    fn test_graph() {
        assert_eq!(BARS[0], graph(0.0).unwrap());
        assert_eq!(BARS[1], graph(0.125).unwrap());
        assert_eq!(BARS[2], graph(0.25).unwrap());
        assert_eq!(BARS[3], graph(0.375).unwrap());
        assert_eq!(BARS[4], graph(0.5).unwrap());
        assert_eq!(BARS[5], graph(0.625).unwrap());
        assert_eq!(BARS[6], graph(0.75).unwrap());
        assert_eq!(BARS[7], graph(0.875).unwrap());
        assert_eq!(BARS[8], graph(1.0).unwrap());
    }

    #[test]
    fn amount_very_close_to_zero() {
        assert_eq!(BARS[0], graph(0.014705882352941176).unwrap());
    }

    #[test]
    fn amount_very_close_to_one() {
        assert_eq!(BARS[8], graph(0.985294117647058824).unwrap());
    }

    #[test]
    #[should_panic]
    fn amount_below_allowed_range() {
        graph(-1.0).unwrap();
    }

    #[test]
    #[should_panic]
    fn amount_above_allowed_range() {
        graph(2.0).unwrap();
    }
}

mod error;

use self::error::Error;
use getopts::Options;
use std::env;
use std::io::{self, Read};

pub type Result<T> = std::result::Result<T, Error>;

const PROGRAM: &str = concat!("Usage: ", env!("CARGO_PKG_NAME"), " [options] [NUMBER...]");

pub fn run() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("n", "no-newline", "don't print a trailing newline");

    let matches = opts.parse(&args[1..])?;

    if matches.opt_present("help") {
        print!("{}", opts.usage(PROGRAM));
        return Ok(());
    }

    let mut stdin = io::stdin();
    let mut buf = String::new();

    stdin.read_to_string(&mut buf)?;

    // Using the free parameters provided on the command line first, convert that and each
    // line from stdin into a list of numbers.
    let numbers: Vec<f64> = buf
        .lines()
        .chain(matches.free.iter().map(String::as_str))
        .filter_map(|x| x.parse().ok())
        .collect();

    // Find the highest and lowest values in the list
    let (min, max) = numbers
        .iter()
        .fold((std::f64::MAX, std::f64::MIN), |(min, max), &x| {
            (x.min(min), x.max(max))
        });

    for line in numbers.iter().map(|n| scale(min, max, *n)) {
        print!("{}", graph(line)?);
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
        assert_eq!(BARS[0], graph("0").unwrap());
        assert_eq!(BARS[1], graph("0.125").unwrap());
        assert_eq!(BARS[2], graph("0.25").unwrap());
        assert_eq!(BARS[3], graph("0.375").unwrap());
        assert_eq!(BARS[4], graph("0.5").unwrap());
        assert_eq!(BARS[5], graph("0.625").unwrap());
        assert_eq!(BARS[6], graph("0.75").unwrap());
        assert_eq!(BARS[7], graph("0.875").unwrap());
        assert_eq!(BARS[8], graph("1").unwrap());
    }

    #[test]
    fn amount_very_close_to_zero() {
        assert_eq!(BARS[0], graph("0.014705882352941176").unwrap());
    }

    #[test]
    fn amount_very_close_to_one() {
        assert_eq!(BARS[8], graph("0.985294117647058824").unwrap());
    }

    #[test]
    #[should_panic]
    fn amount_below_allowed_range() {
        graph("-1").unwrap();
    }

    #[test]
    #[should_panic]
    fn amount_above_allowed_range() {
        graph("2").unwrap();
    }
}

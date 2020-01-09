mod error;

use self::error::Error;
use std::env;

pub type Result<T> = std::result::Result<T, Error>;

const USAGE: &str = concat!("Usage: seq 10 | ", env!("CARGO_PKG_NAME"));

pub fn run() -> Result<()> {
    let mut args = env::args();

    // skip program name
    args.next();

    match args.next() {
        Some(arg) => match arg.as_str() {
            "-h" | "-help" | "--help" | "--usage" => println!("{}", USAGE),
            _ => print!("{}", graph(&arg)?),
        },
        _ => println!("{}", USAGE),
    }

    Ok(())
}

fn graph(arg: &str) -> Result<char> {
    let amount = (arg.parse::<f64>()? * 8_f64).round() as u32;

    if amount > 8 {
        return Err(Error::OutOfBounds);
    }

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

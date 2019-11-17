#[macro_use]
extern crate clap;

pub mod app;
mod error;

use self::error::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub struct Config {
    pub quiet: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self { quiet: 0 }
    }
}

impl Config {
    pub fn from_matches(m: &clap::ArgMatches) -> Result<Self> {
        let quiet = match m.occurrences_of("quiet") {
            q @ 0..=2 => q,
            _ => 2,
        };

        Ok(Config { quiet })
    }
}

pub fn run(m: clap::ArgMatches) -> Result<()> {
    if let Some(values) = m.values_of("number") {
        for n in values {
            print!("{}", graph(n)?);
        }
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

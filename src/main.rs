use std::env;
use std::process;
use std::error;
use std::fmt;
use std::num;

#[derive(Debug)]
enum Error {
    CharParse,
    OutOfBounds,
    ParseFloat(num::ParseFloatError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::CharParse => write!(f, "Could not parse amount as a char"),
            Error::OutOfBounds => write!(f, "Value out of bounds"),
            Error::ParseFloat(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::CharParse => "Could not parse amount as a char",
            Error::OutOfBounds => "Must be a floating point number between 0 and 1",
            Error::ParseFloat(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            Error::CharParse => None,
            Error::OutOfBounds => None,
            Error::ParseFloat(ref err) => Some(err),
        }
    }
}

impl From<num::ParseFloatError> for Error {
    fn from(err: num::ParseFloatError) -> Self {
        Error::ParseFloat(err)
    }
}

type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph() {
        assert_eq!(' ', graph("0").unwrap());
        assert_eq!('\u{2581}', graph("0.125").unwrap());
        assert_eq!('\u{2582}', graph("0.25").unwrap());
        assert_eq!('\u{2583}', graph("0.375").unwrap());
        assert_eq!('\u{2584}', graph("0.5").unwrap());
        assert_eq!('\u{2585}', graph("0.625").unwrap());
        assert_eq!('\u{2586}', graph("0.75").unwrap());
        assert_eq!('\u{2587}', graph("0.875").unwrap());
        assert_eq!('\u{2588}', graph("1").unwrap());
    }

    #[test]
    fn amount_very_close_to_zero() {
        assert_eq!(' ', graph("0.014705882352941176").unwrap());
    }

    #[test]
    fn amount_very_close_to_one() {
        assert_eq!('\u{2588}', graph("0.985294117647058824").unwrap());
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

fn graph(amount: &str) -> Result<char> {
    let amount = (amount.parse::<f64>()? * 8f64).round() as u32;

    if amount > 8 {
        return Err(Error::OutOfBounds);
    };

    if amount == 0 {
        Ok(' ')
    } else {
        std::char::from_u32(0x2580u32 + amount).ok_or(Error::CharParse)
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let mut args = env::args();

    // skip program name
    args.next();

    if let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "-help" | "--help" | "--usage" => {
                println!("Usage: vgraph [OPTION]... NUMBER");
            }
            _ => print!("{}", graph(&arg)?),
        }
    };

    Ok(())
}

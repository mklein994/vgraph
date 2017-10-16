use std::env;
use std::process;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph() {
        assert_eq!(' ', graph(0.0 / 8.0));
        assert_eq!('\u{2581}', graph(1.0 / 8.0));
        assert_eq!('\u{2582}', graph(2.0 / 8.0));
        assert_eq!('\u{2583}', graph(3.0 / 8.0));
        assert_eq!('\u{2584}', graph(4.0 / 8.0));
        assert_eq!('\u{2585}', graph(5.0 / 8.0));
        assert_eq!('\u{2586}', graph(6.0 / 8.0));
        assert_eq!('\u{2587}', graph(7.0 / 8.0));
        assert_eq!('\u{2588}', graph(8.0 / 8.0));
    }
}

fn graph(amount: f64) -> char {
    if amount > 1.0 || amount < 0.0 {
        eprintln!("Must be a number between 0 and 1");
        process::exit(1);
    };

    if amount == 0.0 {
        ' '
    } else {
        std::char::from_u32(0x2580u32 + (amount * 8f64).round() as u32).unwrap()
    }
}

fn main() {
    if let Some(arg) = env::args().nth(1) {
        if let Ok(n) = arg.parse::<f64>() {
            print!("{}", graph(n));
        } else {
            eprintln!("Could not parse as a floating point number");
            process::exit(1);
        }
    } else {
        eprintln!("Pass a decimal number between 0 and 1");
        process::exit(1);
    }
}

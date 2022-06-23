use crate::Error;
use atty::Stream;
use std::io;
// use std::io::{self, BufRead, Read};

pub fn scaled(matches: &getopts::Matches) -> Result<(), Error> {
    use std::io::Read;

    // Gather stdin if available, otherwise just an empty string. If run in a pipe, stdin
    // isn't available for the user to enter something with the keyboard for example.
    // Wait for input if no numbers were given though.
    let buf = if !matches.opt_present("no-wait")
        && (atty::isnt(Stream::Stdin) || matches.free.is_empty())
    {
        let mut stdin = io::stdin();
        let mut buf = String::new();

        stdin.read_to_string(&mut buf)?;
        buf
    } else {
        String::new()
    };

    // Using the free parameters provided on the command line first, convert that and each
    // line from stdin into a list of numbers.
    let numbers: Vec<f64> = matches
        .free
        .iter()
        .map(String::as_str)
        .chain(buf.lines())
        .filter_map(|x| x.parse().ok())
        .collect();

    // Find the highest and lowest values in the list
    let (min, max) = if numbers.len() == 1 {
        if numbers[0] > 1_f64 || numbers[0] < 0_f64 {
            return Err(Error::OutOfBounds);
        }
        (0_f64, 1_f64)
    } else {
        numbers
            .iter()
            .fold((std::f64::MAX, std::f64::MIN), |(min, max), &x| {
                (x.min(min), x.max(max))
            })
    };

    for line in numbers.into_iter().map(|n| crate::scale(min, max, n)) {
        print!("{}", crate::graph(line)?);
    }

    Ok(())
}

/// Requires input to be exactly within this interval: [0.0, 1.0].
pub fn fixed() -> Result<(), Error> {
    let stdin = std::io::stdin();
    // let mut stdout = std::io::stdout();
    let mut line = String::new();

    while stdin.read_line(&mut line)? > 0 {
        let number: f64 = line.trim().parse()?;
        let graph = crate::graph(number)?;
        print!("{graph}");
        line = String::new();
    }
    Ok(())
}

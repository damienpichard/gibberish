use clap::Parser;
use rand::{Rng, distr::Alphanumeric};
use lipsum::lipsum;

mod ascii;
mod extended_ascii;
use crate::ascii::Ascii;
use crate::extended_ascii::ExtendedAscii;

/// Generate gibberish
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Arguments {
    /// Length of the string to generate
    #[arg(default_value_t = 512)]
    length: usize,

    /// Use alphanumeric only characters
    #[arg(short, long, default_value_t = false)]
    alphanumeric: bool,

    /// Use extended ASCII characters
    #[arg(short, long, default_value_t = false)]
    extended: bool,

    /// Generate paragraphs with Lorem Ipsum
    #[arg(short, long, default_value_t = false)]
    lipsum: bool
}


fn main() {
    let args = Arguments::parse();
    let s: String ;

    if args.lipsum {
        s = lipsum(args.length);
    } else if args.alphanumeric {
        s = rand::rng()
                .sample_iter(&Alphanumeric)
                .take(args.length)
                .map(char::from)
                .collect();
    } else if args.extended {
        s = rand::rng()
                .sample_iter(&ExtendedAscii)
                .take(args.length)
                .map(char::from)
                .collect();
    } else {
        s = rand::rng()
                .sample_iter(&Ascii)
                .take(args.length)
                .map(char::from)
                .collect();
     }

    println!("{s}");
}

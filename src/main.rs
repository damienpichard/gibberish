/**
 *
 *  Copyright © Damien Pichard 2025
 *  Author: Damien Pichard <damienpichard@tuta.com>
 *
 *  Redistribution and use in source and binary forms, with or without
 *  modification, are permitted provided that the following conditions
 *  are met:
 *  1. Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 *  2. Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 *  3. Neither the name of the copyright holder nor the names of its
 *     contributors may be used to endorse or promote products derived
 *     from this software without specific prior written permission.
 *  THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS “AS IS”
 *  AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 *  IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 *  ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
 *  LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 *  CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 *  SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 *  INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 *  CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 *  ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF
 *  THE POSSIBILITY OF SUCH DAMAGE.
 *
 *  Code:
 *
**/


use clap::Parser;
use lipsum::lipsum;
use rand::{Rng, distr::Alphanumeric};

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

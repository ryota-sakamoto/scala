extern crate scala;

use std::num::ParseIntError;
use scala::util::Either;

fn main() {
    println!("{:?}", f("100").flat_map(Box::new(|n| {
        match "nk".parse::<u64>() {
            Ok(n) => Either::Right(n),
            Err(e) => Either::Left(e),
        }
    })));
}

fn f(s: &str) -> Either<ParseIntError, u64> {
    match s.parse() {
        Ok(n) => Either::Right(n),
        Err(e) => Either::Left(e),
    }
}

#[macro_use]
extern crate scala;

use std::num::ParseIntError;
use scala::util::Either;

fn main() {
    let result = f("1100")
        .flat_map(f1)
        .flat_map(f2);
        //.flat_map(e1);
    println!("{:?}", result);
}

fn f(s: &str) -> Either<ParseIntError, u64> {
    s.parse().into()
}

fn f1(n: u64) -> Either<ParseIntError, u8> {
    format!("{}", n).parse().into()
}

fn f2(n: u8) -> Either<ParseIntError, u64> {
    format!("{}", n).parse().into()
}

fn e1(n: u64) -> Either<String, String> {
    if n == 1000 {
        Either::Right("n is 1000".to_string())
    } else {
        Either::Left("n is not 1000".to_string())
    }
}
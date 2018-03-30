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
    Either::parse_result(s.parse())
}

fn f1(n: u64) -> Either<ParseIntError, u8> {
    Either::parse_result(format!("{}", n).parse())
}

fn f2(n: u8) -> Either<ParseIntError, u64> {
    Either::parse_result(format!("{}", n).parse())
}

fn e1(n: u64) -> Either<String, String> {
    if n == 1000 {
        Either::Right("n is 1000".to_string())
    } else {
        Either::Left("n is not 1000".to_string())
    }
}
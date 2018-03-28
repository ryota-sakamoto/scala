#[macro_use]
extern crate scala;

use std::num::ParseIntError;
use scala::util::Either;

fn main() {
    let result = e_for!{
        _ <- f("1100")
        _ <- f1()
        _ <- f2()
    };
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
#[macro_use]
extern crate scala;

use std::num::ParseIntError;
use scala::util::Either;

fn main() {
    let result = e_for!{
        a <- f("1100")
        _ <- f1()
        _ <- f2()
    };
    println!("{:?}", result);
}

fn f(s: &str) -> Either<ParseIntError, u64> {
    match s.parse() {
        Ok(n) => Either::Right(n),
        Err(e) => Either::Left(e),
    }
}

fn f1(n: u64) -> Either<ParseIntError, u8> {
    match format!("{}", n).parse() {
        Ok(n) => Either::Right(n),
        Err(e) => Either::Left(e),
    }
}

fn f2(n: u8) -> Either<ParseIntError, u64> {
    match format!("{}", n).parse() {
        Ok(n) => Either::Right(n),
        Err(e) => Either::Left(e),
    }
}